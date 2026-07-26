//! Sprint 142 — Controlled change & evaluation contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, BehaviourVersion, ChangeEvaluation, ControlledChangeSurface,
    OutcomeAdaptationProposal, RecommendationConfidence, RecommendationEvidence,
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

fn approved_proposal() -> OutcomeAdaptationProposal {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal(
        "experience_presentation",
        "Keep DisplayReason primary",
        "Consistent rationale",
    );
    proposal.require_review().unwrap();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    proposal
}

/// CASE 1 — Unapproved changes cannot exist.
#[test]
fn case1_unapproved_changes_cannot_exist() {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal("area", "change", "effect");
    assert!(ControlledChangeSurface::from_unapproved_proposal(&proposal).is_err());
    assert!(BehaviourVersion::from_unapproved_proposal(&proposal).is_err());
    proposal.require_review().unwrap();
    assert!(ControlledChangeSurface::from_approved_proposal(&proposal).is_err());
    let approved = approved_proposal();
    let surface = ControlledChangeSurface::from_approved_proposal(&approved).unwrap();
    assert_eq!(surface.originating_proposal_id, approved.id);
    assert!(!surface.reviewer.actor_id.is_empty());
    assert!(!surface.rollback.may_auto_rollback);
}

/// CASE 2 — Rollback preserves provenance.
#[test]
fn case2_rollback_preserves_provenance() {
    let proposal = approved_proposal();
    let provenance = proposal.provenance.clone();
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    assert_eq!(surface.provenance, provenance);
    let version = surface.to_behaviour_version_draft().unwrap();
    assert_eq!(version.provenance, provenance);
    let rollback = version.prepare_rollback_draft("safer prior").unwrap();
    assert_eq!(rollback.provenance, provenance);
    assert_eq!(
        rollback.previous_version_id.as_deref(),
        Some(BehaviourVersion::BASELINE_ID)
    );
    assert!(!rollback.may_mutate_runtime());
}

/// CASE 3 — Evaluation cannot rewrite history.
#[test]
fn case3_evaluation_cannot_rewrite_history() {
    let proposal = approved_proposal();
    let provenance = proposal.provenance.clone();
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let mut evaluation = ChangeEvaluation::from_behaviour_version(
        &version,
        vec!["users resume faster".into()],
        vec!["continuity click-through improves".into()],
        Some("rollback if false positives rise".into()),
    );
    assert!(!evaluation.may_rewrite_history());
    assert!(evaluation.attempt_rewrite_provenance().is_err());
    assert_eq!(evaluation.provenance_snapshot, provenance);
    assert_eq!(version.provenance, provenance);
    assert!(ChangeEvaluation::attempt_mutate_runtime().is_err());
    assert!(BehaviourVersion::attempt_mutate_runtime().is_err());
    assert!(version.attempt_publish().is_err());
}

/// CASE 4 — Adaptation cannot bypass Permission Gateway.
#[test]
fn case4_adaptation_cannot_bypass_permission_gateway() {
    let proposal = approved_proposal();
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let evaluation = ChangeEvaluation::from_behaviour_version(
        &version,
        vec![],
        vec![],
        None,
    );
    assert!(!proposal.may_bypass_permission_gateway());
    assert!(!surface.may_bypass_permission_gateway());
    assert!(!version.may_bypass_permission_gateway());
    assert!(!evaluation.may_bypass_permission_gateway());
    assert!(ControlledChangeSurface::attempt_apply().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_eq!(surface.authority_effect, "none");
    assert_eq!(version.authority_effect, "none");
    assert_eq!(evaluation.authority_effect, "none");
}
