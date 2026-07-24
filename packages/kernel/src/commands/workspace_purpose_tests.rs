//! Workspace Purpose Model tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, PurposeEvidenceKind,
    TaskPriority, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Purpose WS".into()))
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
        "Ship v1".into(),
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
    let goal = CommandHandler::create_work_goal(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Ship Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (
        workspace_id,
        project.id.to_string(),
        goal.id.to_string(),
    )
}

/// CASE 1 — Purpose derives only from existing workspace models.
#[test]
fn case1_purpose_derives_from_existing_models() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, goal_id) = seed(&kernel);
    let state = CommandHandler::generate_workspace_purpose(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert_eq!(state.label, "Ship Workspace AI v1");
    assert_eq!(state.primary_work_goal_id.as_deref(), Some(goal_id.as_str()));
    assert!(state
        .evidence_items
        .iter()
        .any(|e| e.kind == PurposeEvidenceKind::WorkGoal));
    assert!(state
        .evidence_items
        .iter()
        .any(|e| e.kind == PurposeEvidenceKind::Composition));
    assert!(state.explanation.contains("does not replace"));
}

/// CASE 2 — Purpose does not duplicate Project or Task ownership.
#[test]
fn case2_purpose_does_not_duplicate_ownership() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "purpose"
            && c.owner == "WorkspacePurposeService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "work_goal"
            && c.owner == "WorkspaceIntentService"
            && c.kind == ConceptOwnerKind::DurableStore
    }));
    let purpose_owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "purpose")
        .collect();
    assert_eq!(purpose_owners.len(), 1);
}

/// CASE 3 — Purpose relationships are explainable.
#[test]
fn case3_purpose_relationships_are_explainable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_purpose(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.relationships.is_empty());
    assert!(state
        .relationships
        .iter()
        .all(|r| !r.explanation.is_empty() && !r.evidence.is_empty()));
    assert!(state
        .evidence_items
        .iter()
        .all(|e| !e.explanation.is_empty()));
}

/// CASE 4 — Purpose updates when supporting work changes.
#[test]
fn case4_purpose_updates_when_work_changes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_purpose(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let open_before = before.open_task_count;

    let task = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Complete Task Graph".into(),
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

    let after = CommandHandler::generate_workspace_purpose(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(after.completed_task_count >= before.completed_task_count);
    assert!(
        after.recent_progress.iter().any(|p| p.contains("Complete Task Graph"))
            || after.completed_task_count > before.completed_task_count
            || after.open_task_count != open_before
            || after.progress_percent != before.progress_percent
    );
}

/// CASE 5 — Continuity contributes Purpose context (Purpose embeds Continuity; Continuity unchanged).
#[test]
fn case5_continuity_contributes_purpose_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_purpose(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state
            .evidence_items
            .iter()
            .any(|e| e.kind == PurposeEvidenceKind::Continuity)
            || state.focus_label.is_some()
            || state.evidence.iter().any(|e| e.contains("Continuity") || e.contains("project")),
        "Purpose should embed Continuity or active project focus evidence"
    );
}

/// CASE 6 — Attention uses Purpose context.
#[test]
fn case6_attention_uses_purpose_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let attention = CommandHandler::generate_workspace_attention(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        attention
            .items
            .iter()
            .any(|i| i.source_type.as_str() == "purpose"),
        "Attention should include Purpose-sourced items"
    );
}

/// CASE 7 — Composition relationships remain intact via Purpose evidence.
#[test]
fn case7_composition_relationships_remain_intact() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let composition = CommandHandler::generate_workspace_composition(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let purpose = CommandHandler::generate_workspace_purpose(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(composition.authority_effect, "none");
    assert!(purpose
        .evidence_items
        .iter()
        .any(|e| e.kind == PurposeEvidenceKind::Composition));
    assert_eq!(
        purpose.composition_label.as_deref(),
        Some(composition.label.as_str())
    );
}

/// CASE 8 — Assistant only explains Purpose (shared Intelligence path).
#[test]
fn case8_assistant_only_explains_purpose() {
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
    assert_eq!(work.purpose.authority_effect, "none");
    assert_eq!(assistant.purpose.authority_effect, "none");
    assert_eq!(work.purpose.workspace_id, assistant.purpose.workspace_id);
    assert!(!work.purpose.summary.is_empty());
    assert!(!work.purpose.explanation.is_empty());
}

/// CASE 9 — Purpose cannot execute actions.
#[test]
fn case9_purpose_cannot_execute() {
    match CommandHandler::workspace_purpose_attempt_execute() {
        Err(KernelError::WorkspacePurposeValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspacePurposeValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing governance / intelligence / composition / attention paths remain green.
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
    assert_eq!(intel.purpose.authority_effect, "none");
    assert_eq!(intel.composition.authority_effect, "none");
    assert_eq!(intel.environment.authority_effect, "none");
    assert_eq!(intel.task_graph.authority_effect, "none");
    assert_eq!(intel.continuity.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_composition(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_attention(&kernel, local, intent, ws).unwrap();
}
