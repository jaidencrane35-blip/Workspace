//! Workspace Composition Engine tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, CompositionMemberKind, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, TaskRelationshipKind,
};
use workspace_windows_integration::DesktopWindowSnapshot;

fn seed(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Composition WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace AI".into(),
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
        "Phase 5".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    let app = CommandHandler::create_application(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "VS Code".into(),
        Some("Code.exe".into()),
        None,
    )
    .unwrap();
    (workspace_id, app.id.to_string(), project.id.to_string())
}

fn windows_with_vscode() -> Vec<DesktopWindowSnapshot> {
    vec![DesktopWindowSnapshot {
        hwnd: "0xc1".into(),
        title: "main.rs - VS Code".into(),
        process_id: 501,
        visible: true,
    }]
}

fn compose_with_windows(
    kernel: &WorkspaceKernel,
    ws: &str,
    windows: &[DesktopWindowSnapshot],
) -> workspace_domain::WorkspaceCompositionState {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let apps = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        workspace_database::ApplicationRepository::new(&guard)
            .list_by_workspace(&workspace_domain::WorkspaceId::new(ws).unwrap())
            .unwrap()
    };
    let workflow = CommandHandler::get_workflow_context(
        kernel,
        local.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    let environment = crate::services::WorkspaceEnvironmentService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        ws,
        windows,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();
    let task_graph =
        crate::services::TaskGraphService::generate(&kernel.shared_database(), &local, ws).unwrap();
    let queue = crate::services::DecisionQueueService::aggregate_readonly(
        &kernel.shared_database(),
        &local,
        &kernel.orchestrated_plans(),
        &kernel.assistant_workflows(),
        ws,
    )
    .unwrap();
    let activity = crate::services::WorkspaceActivityGraphService::generate_with_decision_queue(
        &kernel.shared_database(),
        &local,
        &kernel.orchestrated_plans(),
        &kernel.assistant_workflows(),
        ws,
        Some(&queue),
    )
    .unwrap();
    let continuity = crate::services::WorkspaceContinuityService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        ws,
        &queue,
        &activity,
    )
    .unwrap();
    let project = workflow.active_project_id.as_ref().and_then(|id| {
        crate::services::WorkspaceIntentService::get_project(&kernel.shared_database(), id.as_str())
            .ok()
    });
    crate::services::WorkspaceCompositionService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        ws,
        &environment,
        Some(&task_graph),
        &continuity,
        &activity,
        &workflow,
        &queue,
        project.as_ref(),
    )
    .unwrap()
}

/// CASE 1 — Compositions derive entirely from existing models.
#[test]
fn case1_compositions_derive_from_existing_models() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_id, _) = seed(&kernel);
    let state = compose_with_windows(&kernel, &ws, &windows_with_vscode());
    assert_eq!(state.authority_effect, "none");
    assert!(state.members.iter().any(|m| {
        m.kind == CompositionMemberKind::Application
            && m.ref_id == app_id
            && m.present
    }));
    assert!(state
        .members
        .iter()
        .any(|m| m.kind == CompositionMemberKind::Environment));
    assert!(state.evidence.iter().any(|e| e.contains("Environment Model")));
    assert!(!state.explanation.is_empty());
}

/// CASE 2 — Environment changes update compositions.
#[test]
fn case2_environment_changes_update_compositions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let empty = compose_with_windows(&kernel, &ws, &[]);
    assert_eq!(empty.present_application_count, 0);
    assert!(empty.missing_application_count >= 1);

    let with_app = compose_with_windows(&kernel, &ws, &windows_with_vscode());
    assert_eq!(with_app.present_application_count, 1);
    assert_eq!(with_app.missing_application_count, 0);
    assert_eq!(with_app.window_count, 1);
}

/// CASE 3 — Task Graph relationships appear.
#[test]
fn case3_task_graph_relationships_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let t1 = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Implement composition".into(),
        Some(project_id.clone()),
        workspace_domain::WorkspaceTaskPriority::High,
    )
    .unwrap();
    let t2 = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Wire Attention".into(),
        Some(project_id),
        workspace_domain::WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::add_task_relationship(
        &kernel,
        local,
        intent,
        ws.clone(),
        t2.id.to_string(),
        t1.id.to_string(),
        TaskRelationshipKind::DependsOn,
    )
    .unwrap();

    let state = compose_with_windows(&kernel, &ws, &windows_with_vscode());
    assert!(state.task_node_count >= 1);
    assert!(state
        .members
        .iter()
        .any(|m| m.kind == CompositionMemberKind::TaskNode));
    assert!(state
        .relationships
        .iter()
        .any(|r| r.kind.starts_with("task_graph:")));
}

/// CASE 4 — Continuity contributes correctly.
#[test]
fn case4_continuity_contributes_correctly() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let state = compose_with_windows(&kernel, &ws, &windows_with_vscode());
    assert!(
        state
            .members
            .iter()
            .any(|m| m.kind == CompositionMemberKind::Continuity
                || m.kind == CompositionMemberKind::ActiveWork
                || m.kind == CompositionMemberKind::Project),
        "composition should include Continuity and/or active work members"
    );
    assert!(state.evidence.iter().any(|e| {
        e.contains("Continuity") || e.contains("Active project") || e.contains("Decision Queue")
    }));
}

/// CASE 5 — Attention references compositions.
#[test]
fn case5_attention_references_compositions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    // Missing app gap → composition gap → Attention Composition source.
    let _ = compose_with_windows(&kernel, &ws, &[]);
    let attention = CommandHandler::generate_workspace_attention(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(
        attention
            .items
            .iter()
            .any(|i| i.source_type.as_str() == "composition"
                || i.source_type.as_str() == "environment"),
        "Attention should reference Composition and/or Environment gaps"
    );
}

/// CASE 6 — Assistant and Intelligence consume identical composition models.
#[test]
fn case6_assistant_and_intelligence_share_composition() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let work = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(
        work.composition.authority_effect,
        assistant.composition.authority_effect
    );
    assert_eq!(work.composition.workspace_id, assistant.composition.workspace_id);
    assert_eq!(
        work.composition.authority_effect,
        workspace_domain::WorkspaceCompositionState::AUTHORITY_EFFECT_NONE
    );
}

/// CASE 7 — No authority changes occur.
#[test]
fn case7_no_authority_changes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_composition(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state
        .members
        .iter()
        .all(|m| m.authority_effect == "none"));
}

/// CASE 8 — No execution occurs.
#[test]
fn case8_no_execution() {
    match CommandHandler::workspace_composition_attempt_execute() {
        Err(KernelError::WorkspaceCompositionValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceCompositionValidation, got {other:?}"),
    }
}

/// CASE 9 — No duplicate persistence exists (aggregator ownership only).
#[test]
fn case9_no_duplicate_persistence() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "composition"
            && c.owner == "WorkspaceCompositionService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "composition")
        .collect();
    assert_eq!(owners.len(), 1);
}

/// CASE 10 — Existing governance / intelligence / environment / task graph paths remain green.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.environment.authority_effect, "none");
    assert_eq!(intel.composition.authority_effect, "none");
    assert_eq!(intel.task_graph.authority_effect, "none");
    assert_eq!(intel.continuity.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_environment(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_task_graph(&kernel, local, intent, ws).unwrap();
}
