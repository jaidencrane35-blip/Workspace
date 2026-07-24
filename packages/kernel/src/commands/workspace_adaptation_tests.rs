//! Workspace Adaptation Proposal tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AdaptationKind, AdaptationProposal, AdaptationStatus, ConceptOwnerKind,
    IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Adaptation WS".into()))
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
        "Ship Adaptation".into(),
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
    workspace_id
}

/// CASE 1 — Adaptations derive only from existing systems.
#[test]
fn case1_adaptations_derive_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Pattern")));
    assert!(state.explanation.contains("proposals only"));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "adaptation_proposal"
            && c.owner == "WorkspaceAdaptationService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

/// CASE 2 — Every adaptation has evidence.
#[test]
fn case2_every_adaptation_has_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.proposals.is_empty());
    assert!(state.proposals.iter().all(|p| {
        !p.reason.is_empty()
            && !p.evidence.is_empty()
            && !p.impact.benefit.is_empty()
            && !p.impact.risk.is_empty()
            && p.authority_effect == "none"
    }));
}

/// CASE 3 — Adaptations cannot execute.
#[test]
fn case3_adaptations_cannot_execute() {
    match CommandHandler::workspace_adaptation_attempt_execute() {
        Err(KernelError::WorkspaceAdaptationValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceAdaptationValidation, got {other:?}"),
    }
}

/// CASE 4 — Human review is required before accept.
#[test]
fn case4_human_review_is_required() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.proposals[0].id.clone();
    let direct = CommandHandler::accept_adaptation_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    );
    assert!(matches!(
        direct,
        Err(KernelError::WorkspaceAdaptationValidation { .. })
    ));
    let reviewed = CommandHandler::review_adaptation_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();
    assert_eq!(
        reviewed.proposal.as_ref().unwrap().status,
        AdaptationStatus::Reviewed
    );
    assert!(reviewed.handoff.is_none());
    let accepted = CommandHandler::accept_adaptation_proposal(
        &kernel, local, intent, ws, id,
    )
    .unwrap();
    assert_eq!(
        accepted.proposal.as_ref().unwrap().status,
        AdaptationStatus::Accepted
    );
}

/// CASE 5 — Accepted adaptations still enter normal governance (Intent handoff).
#[test]
fn case5_accepted_enters_normal_governance() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.proposals[0].id.clone();
    CommandHandler::review_adaptation_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id.clone(),
    )
    .unwrap();
    let accepted = CommandHandler::accept_adaptation_proposal(
        &kernel, local, intent, ws, id,
    )
    .unwrap();
    let handoff = accepted.handoff.expect("accept must hand off to Intent");
    assert_eq!(
        handoff.next_command,
        AdaptationProposal::HANDOFF_SUBMIT_ASSISTANT_GOAL
    );
    assert_eq!(handoff.authority_effect, "none");
    assert!(handoff.note.contains("never executes"));
    assert!(handoff.note.contains("Gateway"));
}

/// CASE 6 — Rejected adaptations do not modify workspace state.
#[test]
fn case6_rejected_does_not_modify_workspace() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_purpose(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = state.proposals[0].id.clone();
    let rejected = CommandHandler::reject_adaptation_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        id,
    )
    .unwrap();
    assert_eq!(
        rejected.proposal.as_ref().unwrap().status,
        AdaptationStatus::Rejected
    );
    assert!(rejected.handoff.is_none());
    let after = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
    assert_eq!(before.label, after.label);
    assert_eq!(before.progress_percent, after.progress_percent);
}

/// CASE 7 — Patterns influence proposals correctly.
#[test]
fn case7_patterns_influence_proposals() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_adaptation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state
            .proposals
            .iter()
            .any(|p| p.related_pattern_id.is_some())
            || state.evidence.iter().any(|e| e.contains("Pattern"))
    );
    assert!(state.proposals.iter().any(|p| matches!(
        p.kind,
        AdaptationKind::ApplicationGrouping
            | AdaptationKind::LayoutImprovement
            | AdaptationKind::WorkflowShortcut
            | AdaptationKind::TaskOrganization
            | AdaptationKind::WorkspaceOrganization
            | AdaptationKind::ContextRestoration
    )));
}

/// CASE 8 — Recommendations remain separate from adaptations.
#[test]
fn case8_recommendations_remain_separate() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let adaptation = CommandHandler::generate_workspace_adaptation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let recommendations = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(
        adaptation.explanation.to_lowercase().contains("recommendation")
            && adaptation.explanation.contains("proposals only")
    );
    assert!(recommendations.explanation.contains("suggestions only"));
    assert_ne!(adaptation.summary, recommendations.summary);
}

/// CASE 9 — Assistant remains explain-only.
#[test]
fn case9_assistant_remains_explain_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
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
        work.adaptation.authority_effect,
        assistant.adaptation.authority_effect
    );
    assert_eq!(
        work.adaptation.authority_effect,
        workspace_domain::WorkspaceAdaptationState::AUTHORITY_EFFECT_NONE
    );
    assert!(!work.adaptation.summary.is_empty());
    assert!(work.adaptation.summary.contains("never execute"));
}

/// CASE 10 — Existing governance surfaces remain green.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
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
    assert_eq!(intel.adaptation.authority_effect, "none");
    assert_eq!(intel.pattern.authority_effect, "none");
    assert_eq!(intel.operating_state.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_pattern(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
}
