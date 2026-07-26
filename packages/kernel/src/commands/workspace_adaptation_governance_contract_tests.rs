//! Sprint 140 — Adaptation governance contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AttentionReason, AttentionSignal, AttentionSourceType, OutcomeAdaptationProposal,
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
                decision_readiness: None,
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

/// CASE 1 — Outcomes cannot mutate cognition (scoring / historical reasoning).
#[test]
fn case1_outcomes_cannot_mutate_cognition() {
    let outcome = accepted_outcome();
    let reasons = outcome.provenance.reasoning_origins.clone();
    let _ = outcome.to_adaptation_proposal(
        "continuity",
        "Surface resumable work earlier",
        "Faster resume",
    );
    assert_eq!(outcome.provenance.reasoning_origins, reasons);
    assert!(!outcome.may_mutate_historical_reasoning());
    assert!(!outcome.may_silently_change_scoring());
}

/// CASE 2 — Adaptation requires explicit handling / review.
#[test]
fn case2_adaptation_requires_explicit_handling() {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal(
        "experience_presentation",
        "Keep DisplayReason primary for resumable work",
        "Consistent rationale rendering",
    );
    assert!(proposal.requires_explicit_handling());
    assert!(proposal.review_required);
    assert!(!proposal.may_auto_apply());
    proposal.require_review().unwrap();
    assert_eq!(
        proposal.review_status,
        OutcomeAdaptationReviewStatus::AwaitingReview
    );
    // Cannot skip to ApprovedForHandoff from AwaitingReview.
    assert!(proposal
        .transition_review(OutcomeAdaptationReviewStatus::ApprovedForHandoff)
        .is_err());
    proposal
        .transition_review(OutcomeAdaptationReviewStatus::Reviewed)
        .unwrap();
    proposal.approve_for_handoff().unwrap();
    assert_eq!(proposal.authority_effect, "none");
}

/// CASE 3 — Provenance remains immutable through adaptation proposal creation.
#[test]
fn case3_provenance_remains_immutable() {
    let outcome = accepted_outcome();
    let provenance = outcome.provenance.clone();
    let proposal = OutcomeAdaptationProposal::from_outcome(
        &outcome,
        "recommendation_engine",
        "Weight continuity evidence higher in explanations only",
        "Better continuity-linked suggestions (presentation)",
    );
    assert_eq!(proposal.provenance, provenance);
    assert_eq!(
        proposal.experience_trace_match_keys,
        vec!["exact:continuity.resumable"]
    );
    assert!(!proposal.may_mutate_cognition());
}

/// CASE 4 — Execution boundaries unchanged.
#[test]
fn case4_execution_boundaries_unchanged() {
    let outcome = accepted_outcome();
    let proposal = outcome.to_adaptation_proposal("layout", "n/a", "n/a");
    assert!(OutcomeAdaptationProposal::attempt_apply().is_err());
    assert!(OutcomeAdaptationProposal::attempt_execute().is_err());
    assert!(RecommendationOutcome::attempt_execute().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_eq!(proposal.authority_effect, "none");
    let _ = proposal;
}
