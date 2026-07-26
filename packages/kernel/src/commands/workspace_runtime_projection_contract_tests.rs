//! Sprints 182–186 — live Workspace Runtime projection wiring contract tests.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::WorkspaceKernel;
use workspace_domain::{ActorContext, IntentContext, TaskPriority, GOVERNANCE_AUTHORITY_EFFECT_NONE};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Runtime Projection WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Runtime Platform".into(),
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
        "Wire Runtime Projection".into(),
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
    workspace_id
}

#[test]
fn case1_live_projection_assembles_from_foundations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let view = CommandHandler::generate_workspace_runtime_overview(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();

    assert_eq!(view.workspace_id, ws);
    assert!(view.runtime_context_id.contains(&ws));
    assert!(!view.overview.id.is_empty());
    assert!(!view.diagnostic_snapshot_id.is_empty());
    assert_eq!(view.operator_context.workspace_id, ws);
    assert_eq!(view.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(view.overview.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(view.health.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
}

#[test]
fn case2_governance_labels_remain_non_authoritative_and_publication_blocked() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let view = CommandHandler::generate_workspace_runtime_overview(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();

    assert!(view.publication_blocked);
    assert!(view.operator_context.publication_blocked);
    assert!(view.health.publication_blocked);
    assert!(view.overview.publication_blocked);
    assert!(view.overview.governance_summary.contains("publication_blocked=true"));
    assert!(
        view.operator_context.review_status_label == "none"
            || view.operator_context.review_status_label == "review_pending"
    );
}

#[test]
fn case3_runtime_projection_cannot_execute_or_rescore() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let view = CommandHandler::generate_workspace_runtime_overview(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();

    assert!(!view.may_execute());
    assert!(!view.may_change_scoring());
    assert!(CommandHandler::workspace_runtime_attempt_execute().is_err());
    assert!(view.coherence_ok);
    assert!(view.architecture_review_passed);
    assert!(!view.consistency_has_errors);
}
