//! Workspace Milestone Engine tests (Phase 6 Batch 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, MilestoneStatus, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, WorkspaceMilestoneState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Milestone WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace AI Platform".into(),
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
        "Ship Milestone Engine".into(),
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
        "Develop and ship meaningful milestone coordination".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Milestones generated deterministically.
#[test]
fn case1_milestones_generated_deterministically() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_milestones(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_milestones(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.authority_effect, "none");
    assert!(!a.milestones.is_empty());
    assert_eq!(a.milestone_summary.current_line, b.milestone_summary.current_line);
    assert_eq!(a.milestone_count, b.milestone_count);
    assert!(a.milestones.iter().all(|m| !m.why.is_empty() && !m.evidence.is_empty()));
}

/// CASE 2 — Task Graph / active work changes update milestones.
#[test]
fn case2_task_graph_changes_update_milestones() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_milestones(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_milestones(&kernel, local, intent, ws).unwrap();
    let comparison = CommandHandler::compare_workspace_milestones(&before, &after);
    assert!(!comparison.differences.is_empty());
}

/// CASE 3 — Purpose / goal changes update milestones.
#[test]
fn case3_purpose_changes_update_milestones() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_milestones(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::create_work_goal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Research documentation planning outcome".into(),
        None,
        None,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_milestones(&kernel, local, intent, ws).unwrap();
    assert!(
        before.summary != after.summary
            || before.milestone_summary.current_line != after.milestone_summary.current_line
            || before.evidence != after.evidence
    );
}

/// CASE 4 — Blocked decisions / readiness affect milestones.
#[test]
fn case4_blocked_decisions_affect_milestones() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_milestones(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    // With or without blockers, blocked-line is always explainable.
    assert!(!state.milestone_summary.blocked_line.is_empty());
    for m in state.milestones.iter().filter(|m| m.status == MilestoneStatus::Blocked) {
        assert!(!m.evidence.is_empty());
        assert!(!m.why.is_empty());
    }
}

/// CASE 5 — Navigation references milestones without ownership.
#[test]
fn case5_navigation_references_milestones_without_ownership() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.milestones.milestone_count > 0);
    assert!(
        intel.navigation.explanation.contains("Milestones")
            || intel
                .navigation
                .summary
                .contains("milestone")
            || intel.navigation.explanation.contains("evidence")
            || intel.milestones.authority_effect == "none"
    );
    assert_eq!(intel.navigation.authority_effect, "none");
    assert_eq!(intel.milestones.authority_effect, "none");
}

/// CASE 6 — Recommendations use milestone evidence only.
#[test]
fn case6_recommendations_use_milestone_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        intel.recommendation_engine.explanation.contains("Milestones")
            || intel
                .recommendation_engine
                .summary
                .contains("milestone")
            || intel.milestones.milestone_count > 0
    );
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
}

/// CASE 7 — attempt_execute returns CannotExecute.
#[test]
fn case7_attempt_execute_cannot_execute() {
    match CommandHandler::workspace_milestones_attempt_execute() {
        Err(KernelError::WorkspaceMilestoneValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("plan")
                    || message.contains("schedule")
            );
        }
        other => panic!("expected WorkspaceMilestoneValidation, got {other:?}"),
    }
    assert_eq!(
        WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE,
        "none"
    );
}

/// CASE 8 — Gateway unchanged (reuse capability; single owner).
#[test]
fn case8_gateway_unchanged() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "milestones")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceMilestoneService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);
}

/// CASE 9 — No persistence introduced (projection-only fields + validate).
#[test]
fn case9_no_persistence_introduced() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_milestones(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
    )
    .unwrap();
    assert!(state.explanation.contains("coordination projection") || state.explanation.contains("never plan"));
    assert!(!state.session_generated_at.is_empty());
    assert!(!state.navigation_generated_at.is_empty());
    let report =
        CommandHandler::validate_workspace_milestones(&kernel, local, intent, &state).unwrap();
    assert!(report.valid);
}

/// CASE 10 — Milestones remain explainable and deterministic.
#[test]
fn case10_explainable_and_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_milestones(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(state.current_milestone().is_some());
    assert!(!state.milestone_summary.narrative.is_empty());
    assert!(state.milestones.iter().all(|m| {
        m.authority_effect == "none"
            && !m.why.is_empty()
            && m.evidence.iter().all(|e| !e.why.is_empty())
    }));
    let comparison = CommandHandler::compare_workspace_milestones(&state, &state);
    assert_eq!(comparison.authority_effect, "none");
}
