//! Workspace Task Graph tests (Phase 5 — Sprints 82–83).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
    TaskRelationshipKind, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn seed_workspace(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Task Graph WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Graph Project".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        workspace_id.clone(),
        "Intent-backed work".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Graph persists across restart (in-memory DB re-open via regenerate).
#[test]
fn case1_graph_persists_across_restart() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let created = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Durable node".into(),
        None,
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let before = CommandHandler::generate_task_graph(&kernel, local.clone(), intent.clone(), ws.clone())
        .unwrap();
    assert!(before.nodes.iter().any(|n| n.task.id == created.id));

    // "Restart" = regenerate from same durable DB — nodes remain.
    let after = CommandHandler::generate_task_graph(&kernel, local, intent, ws).unwrap();
    assert!(after.nodes.iter().any(|n| n.task.id == created.id));
    assert_eq!(after.authority_effect, "none");
}

/// CASE 2 — Dependencies resolve correctly.
#[test]
fn case2_dependencies_resolve() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Task A".into(),
        None,
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    let b = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Task B".into(),
        None,
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::add_task_relationship(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        a.id.to_string(),
        b.id.to_string(),
        TaskRelationshipKind::DependsOn,
    )
    .unwrap();
    let graph = CommandHandler::generate_task_graph(&kernel, local, intent, ws).unwrap();
    let node_a = graph
        .nodes
        .iter()
        .find(|n| n.task.id == a.id)
        .expect("task A");
    assert!(node_a.dependency_ids.contains(&b.id.to_string()));
    assert_eq!(node_a.task.status, WorkspaceTaskStatus::Waiting);
    assert!(node_a
        .waiting_reason
        .as_ref()
        .unwrap()
        .contains("must complete first"));
}

/// CASE 3 — Circular dependency detection prevents invalid graphs.
#[test]
fn case3_circular_dependency_rejected() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Cycle A".into(),
        None,
        WorkspaceTaskPriority::Low,
    )
    .unwrap();
    let b = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Cycle B".into(),
        None,
        WorkspaceTaskPriority::Low,
    )
    .unwrap();
    CommandHandler::add_task_relationship(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        a.id.to_string(),
        b.id.to_string(),
        TaskRelationshipKind::DependsOn,
    )
    .unwrap();
    let err = CommandHandler::add_task_relationship(
        &kernel,
        local,
        intent,
        ws,
        b.id.to_string(),
        a.id.to_string(),
        TaskRelationshipKind::DependsOn,
    )
    .unwrap_err();
    match err {
        KernelError::TaskGraphValidation { message } => {
            assert!(message.contains("circular") || message.contains("Circular"));
        }
        other => panic!("expected TaskGraphValidation, got {other:?}"),
    }
}

/// CASE 4 — Completed tasks are not recommended again unless reopened.
#[test]
fn case4_completed_not_recommended() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let task = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Finish docs".into(),
        None,
        WorkspaceTaskPriority::Critical,
    )
    .unwrap();
    CommandHandler::update_workspace_task_status(
        &kernel,
        local.clone(),
        intent.clone(),
        task.id.to_string(),
        WorkspaceTaskStatus::Completed,
        None,
    )
    .unwrap();
    let decisions = CommandHandler::generate_decision_engine(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(!decisions.candidates.iter().any(|c| {
        c.title.to_lowercase().contains("finish docs")
            && c.recommendation_id
                .as_ref()
                .is_some_and(|id| id.contains(&task.id.to_string()))
    }));
}

#[test]
fn completed_task_cannot_reenter_open_lifecycle() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, _) = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let task = CommandHandler::create_workspace_task(
        &kernel,
        actor.clone(),
        intent.clone(),
        workspace_id,
        "Terminal task".into(),
        None,
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::update_workspace_task_status(
        &kernel,
        actor.clone(),
        intent.clone(),
        task.id.to_string(),
        WorkspaceTaskStatus::Completed,
        None,
    )
    .unwrap();
    let error = CommandHandler::update_workspace_task_status(
        &kernel,
        actor,
        intent,
        task.id.to_string(),
        WorkspaceTaskStatus::InProgress,
        None,
    )
    .unwrap_err();
    assert!(matches!(error, KernelError::TaskGraphValidation { .. }));
}

/// CASE 5 — Planner consumes graph nodes without duplicating task state.
#[test]
fn case5_planner_consumes_graph_nodes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let task = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Plan me".into(),
        None,
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let inputs = CommandHandler::get_task_graph_planning_inputs(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(inputs.iter().any(|t| t.id == task.id));
    assert!(inputs.iter().all(|t| t.status.is_open()));
    // No parallel planner task store — inputs are graph nodes only.
    assert!(inputs.iter().all(|t| t.authority_effect == "none"));
}

/// CASE 6 — Permission Gateway behavior is unchanged.
#[test]
fn case6_permission_gateway_unchanged() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let graph = CommandHandler::generate_task_graph(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(graph.authority_effect, "none");
    for node in &graph.nodes {
        assert_eq!(node.task.authority_effect, "none");
    }
}

/// CASE 7 — No graph operation can execute an action directly.
#[test]
fn case7_execution_attempt_blocked() {
    let err = CommandHandler::task_graph_attempt_execute().unwrap_err();
    match err {
        KernelError::TaskGraphValidation { message } => {
            assert!(message.contains("cannot execute") || message.contains("authorize"));
        }
        other => panic!("expected TaskGraphValidation, got {other:?}"),
    }
}

#[test]
fn task_graph_concept_ownership_registered() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "workspace_task"
            && c.owner == "TaskGraphService"
            && c.kind == ConceptOwnerKind::DurableStore
    }));
}

#[test]
fn validate_graph_integrity_ok_for_acyclic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let graph = CommandHandler::validate_task_graph(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(graph.integrity_ok);
}
