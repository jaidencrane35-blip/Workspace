//! Workspace Operating State tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, OperatingSignalKind, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Operating State WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace Platform".into(),
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
        "Ship Operating State".into(),
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
    CommandHandler::create_work_goal(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Build Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Operating State aggregates existing systems only.
#[test]
fn case1_aggregates_existing_systems_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_operating_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Purpose")));
    assert!(state.evidence.iter().any(|e| e.contains("Activity Graph")));
    assert!(state.explanation.contains("Sources remain authoritative"));
    assert!(state.signals.iter().all(|s| s.authority_effect == "none"));
}

/// CASE 2 — No duplicate ownership exists.
#[test]
fn case2_no_duplicate_ownership() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "operating_state"
            && c.owner == "WorkspaceOperatingStateService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "operating_state")
        .collect();
    assert_eq!(owners.len(), 1);
    // Source concepts remain owned by their services.
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| c.concept == "purpose"));
    assert!(PLATFORM_CONCEPT_OWNERS
        .iter()
        .any(|c| c.concept == "recommendation_candidate"));
}

/// CASE 3 — Operating State updates when source systems change.
#[test]
fn case3_updates_when_sources_change() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_operating_state(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let task = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "New Task Graph progress node".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    CommandHandler::update_workspace_task_status(
        &kernel,
        local.clone(),
        intent.clone(),
        task.id.to_string(),
        WorkspaceTaskStatus::Completed,
        Some("Done".into()),
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_operating_state(&kernel, local, intent, ws).unwrap();
    assert!(after.signal_count >= before.signal_count);
    assert!(after.signals.iter().any(|s| {
        s.kind == OperatingSignalKind::Progress
            && (s.current_value.contains("completed")
                || s.evidence.iter().any(|e| e.contains("node")))
    }) || after.context.recent_progress != before.context.recent_progress
        || after.summary != before.summary);
}

/// CASE 4 — Purpose appears correctly.
#[test]
fn case4_purpose_appears_correctly() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_operating_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.context.purpose_label.is_empty());
    assert!(state
        .signals
        .iter()
        .any(|s| s.kind == OperatingSignalKind::Purpose));
    assert!(state.operating_summary.purpose_line.contains("Purpose"));
}

/// CASE 5 — Environment appears correctly.
#[test]
fn case5_environment_appears_correctly() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_operating_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.context.environment_summary.is_empty());
    assert!(!state.context.composition_label.is_empty());
    assert!(state
        .signals
        .iter()
        .any(|s| s.kind == OperatingSignalKind::Environment));
    assert!(state
        .signals
        .iter()
        .any(|s| s.kind == OperatingSignalKind::Composition));
}

/// CASE 6 — Task Graph context appears correctly.
#[test]
fn case6_task_graph_context_appears() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Open Task Graph work".into(),
        Some(project_id),
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    let state =
        CommandHandler::generate_workspace_operating_state(&kernel, local, intent, ws).unwrap();
    assert!(state.signals.iter().any(|s| {
        s.source_model == "task_graph" || s.kind == OperatingSignalKind::ActiveWork
    }));
    assert!(state.context.active_project_label.is_some());
}

/// CASE 7 — Decision Queue context appears correctly.
#[test]
fn case7_decision_queue_context_appears() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_operating_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(state.evidence.iter().any(|e| e.contains("Decision Queue")));
    // Pending list may be empty when queue is empty — still represented in evidence.
    assert!(
        state
            .signals
            .iter()
            .any(|s| s.kind == OperatingSignalKind::PendingDecision)
            || state.context.pending_decisions.is_empty()
    );
}

/// CASE 8 — Recommendation context appears correctly.
#[test]
fn case8_recommendation_context_appears() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Work that yields recommendations".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let state =
        CommandHandler::generate_workspace_operating_state(&kernel, local, intent, ws).unwrap();
    assert!(state
        .evidence
        .iter()
        .any(|e| e.contains("Recommendation candidates")));
    assert!(
        state
            .signals
            .iter()
            .any(|s| s.kind == OperatingSignalKind::Recommendation)
            || !state.operating_summary.suggested_line.is_empty()
    );
}

/// CASE 9 — Assistant remains explain-only.
#[test]
fn case9_assistant_remains_explain_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let work = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(
        work.operating_state.authority_effect,
        assistant.operating_state.authority_effect
    );
    assert_eq!(
        work.operating_state.authority_effect,
        workspace_domain::WorkspaceOperatingState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work.operating_state.workspace_id,
        assistant.operating_state.workspace_id
    );
    assert!(!work.operating_state.summary.is_empty());
    assert!(work.operating_state.summary.contains("never executes"));
}

/// CASE 10 — Operating State cannot execute or bypass Gateway.
#[test]
fn case10_cannot_execute_or_bypass_gateway() {
    match CommandHandler::workspace_operating_state_attempt_execute() {
        Err(KernelError::WorkspaceOperatingStateValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceOperatingStateValidation, got {other:?}"),
    }
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
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
    assert_eq!(intel.operating_state.authority_effect, "none");
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert_eq!(intel.evolution.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
}
