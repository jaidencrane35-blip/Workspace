//! Sprint 141 — Adaptation review & change authority contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, ControlledChangeSurface, OutcomeAdaptationProposal,
    OutcomeAdaptationReviewStatus, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState, RecommendationOutcome,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected CannotExecute-style error, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed at attempt_execute"),
    }
}

fn sample_item() -> RecommendationItem {
    RecommendationItem {
        id: "recommendation:continue_work:focus-1".into(),
        kind: RecommendationKind::ContinueWork,
        title: "Continue focused work".into(),
        reason: "Continuity shows resumable focus".into(),
        evidence: vec![RecommendationEvidence {
            id: "ev-c".into(),
            source_model: "continuity".into(),
            source_ref: "continuity:focus:1".into(),
            summary: "Resumable work".into(),
        }],
        impact: "Restores momentum".into(),
        confidence: RecommendationConfidence::Medium,
        related_attention_id: Some("attention:focus:1".into()),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::Continuity,
            AttentionSignal::ResumableWork,
            30,
            "continuity.resumable",
        )],
        related_task_id: None,
        related_purpose_label: None,
        related_decision_id: None,
        lifecycle_state: None,
        lifecycle_presented_at: None,
        lifecycle_resolved_at: None,
        lifecycle_resolution_type: None,
        explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn accepted_outcome() -> RecommendationOutcome {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record.provenance = record.provenance.with_experience_trace_match_keys(vec![
        "exact:continuity.resumable".into(),
    ]);
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    record.record_outcome("t4").unwrap()
}

fn pending_proposal() -> OutcomeAdaptationProposal {
    let outcome = accepted_outcome();
    let mut proposal = outcome
        .to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        )
        .with_expiry("t-expire");
    proposal.require_review().unwrap();
    proposal
}

/// CASE 1 — Proposals cannot self-approve.
#[test]
fn case1_proposals_cannot_self_approve() {
    let mut proposal = pending_proposal();
    assert!(!proposal.may_self_approve());
    assert!(proposal.attempt_self_approve().is_err());
    assert!(proposal
        .approve(
            AdaptationReviewerIdentity {
                actor_id: OutcomeAdaptationProposal::PROPOSER_ACTOR_ID.into(),
                actor_type: "local_user".into(),
            },
            "t"
        )
        .is_err());
    assert!(proposal
        .approve(
            AdaptationReviewerIdentity {
                actor_id: proposal.id.clone(),
                actor_type: "local_user".into(),
            },
            "t"
        )
        .is_err());
    // Valid human approve still does not execute.
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    assert!(proposal.review_status.is_approved());
    assert!(!proposal.may_execute_from_approval());
}

/// CASE 2 — Approvals cannot execute actions.
#[test]
fn case2_approvals_cannot_execute_actions() {
    let mut proposal = pending_proposal();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    assert!(proposal.controlled_change_surface_ready());
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    assert_eq!(surface.authority_effect, "none");
    assert!(!surface.may_mutate_runtime_cognition());
    assert!(!surface.may_bypass_permission_gateway());
    assert!(ControlledChangeSurface::attempt_apply().is_err());
    assert!(ControlledChangeSurface::attempt_mutate_runtime_cognition().is_err());
    assert!(OutcomeAdaptationProposal::attempt_execute().is_err());
    assert!(proposal.attempt_mark_applied().is_err());
    assert_eq!(proposal.authority_effect, "none");
}

/// CASE 3 — Rejected proposals cannot apply.
#[test]
fn case3_rejected_proposals_cannot_apply() {
    let mut proposal = pending_proposal();
    proposal
        .reject(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-reject",
            "does not fit workflow",
        )
        .unwrap();
    assert_eq!(
        proposal.review_status,
        OutcomeAdaptationReviewStatus::Rejected
    );
    assert!(proposal.attempt_apply_after_rejection().is_err());
    assert!(ControlledChangeSurface::from_approved_proposal(&proposal).is_err());
    assert!(proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-late"
        )
        .is_err());
}

/// CASE 4 — Provenance remains immutable through review.
#[test]
fn case4_provenance_remains_immutable() {
    let outcome = accepted_outcome();
    let provenance = outcome.provenance.clone();
    let mut proposal = outcome.to_adaptation_proposal("area", "change", "effect");
    proposal.require_review().unwrap();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    assert_eq!(proposal.provenance, provenance);
    assert_eq!(
        proposal.experience_trace_match_keys,
        vec!["exact:continuity.resumable"]
    );
    assert!(!proposal.may_mutate_cognition());
    assert!(!proposal.may_silently_change_scoring());
}

/// CASE 5 — Permission Gateway remains separate.
#[test]
fn case5_permission_gateway_remains_separate() {
    let mut proposal = pending_proposal();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    assert!(!proposal.may_bypass_permission_gateway());
    assert!(!proposal.may_execute_from_approval());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    // Audit events are review records, not capability grants.
    assert!(proposal
        .audit_events
        .iter()
        .any(|e| e.action == "approved"));
    assert!(!proposal
        .audit_events
        .iter()
        .any(|e| e.action.contains("grant")));
}
