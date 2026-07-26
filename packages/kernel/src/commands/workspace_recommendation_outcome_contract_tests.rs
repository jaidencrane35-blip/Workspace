//! Sprint 139 — Recommendation outcome and feedback contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationEvidence, RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState, RecommendationOutcome, RecommendationResultKind,
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
        id: "recommendation:resolve_blocker:task-9".into(),
        kind: RecommendationKind::ResolveBlocker,
        title: "Resolve blocker".into(),
        reason: "Blocked work needs attention".into(),
        evidence: vec![RecommendationEvidence {
            id: "ev1".into(),
            source_model: "attention".into(),
            source_ref: "attention:task:9".into(),
            summary: "Blocked task".into(),
        }],
        impact: "Unblocks progress".into(),
        confidence: RecommendationConfidence::High,
        related_attention_id: Some("attention:task:9".into()),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::TaskGraph,
            AttentionSignal::BlockedTask,
            40,
            "task.base.blocked",
        )],
        related_task_id: Some("task:9".into()),
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
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn present_then(
    record: &mut RecommendationGovernanceRecord,
    to: RecommendationLifecycleState,
) {
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.transition(to, "t3", Some("local_user".into())).unwrap();
}

/// CASE 1 — Outcomes preserve provenance snapshots.
#[test]
fn case1_outcomes_preserve_provenance() {
    let item = sample_item();
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
    record.provenance = record.provenance.with_experience_trace_match_keys(vec![
        "prefix_suffix:task.base.blocked".into(),
    ]);
    let provenance = record.provenance.clone();
    present_then(&mut record, RecommendationLifecycleState::Accepted);
    let outcome = record.record_outcome("t4").unwrap();
    assert_eq!(outcome.provenance, provenance);
    assert_eq!(outcome.identity.native_id, item.id);
    assert_eq!(
        outcome.experience_trace_match_keys,
        vec!["prefix_suffix:task.base.blocked"]
    );
}

/// CASE 2 — Rejection is not system failure.
#[test]
fn case2_rejection_is_not_failure() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    present_then(&mut record, RecommendationLifecycleState::Rejected);
    let outcome = record.record_outcome("t4").unwrap();
    assert_eq!(
        outcome.result_kind,
        RecommendationResultKind::RejectedByUser
    );
    assert!(!outcome.is_system_failure());
}

/// CASE 3 — Expiration is not system failure.
#[test]
fn case3_expiration_is_not_failure() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Expired, "t1", None)
        .unwrap();
    let outcome = record.record_outcome("t2").unwrap();
    assert_eq!(
        outcome.result_kind,
        RecommendationResultKind::ExpiredWithoutAction
    );
    assert!(!outcome.is_system_failure());
    assert_eq!(outcome.provenance.reasoning_origins.len(), 1);
}

/// CASE 4 — Outcomes cannot execute actions.
#[test]
fn case4_outcomes_cannot_execute() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    present_then(&mut record, RecommendationLifecycleState::Accepted);
    let outcome = record.record_outcome("t4").unwrap();
    assert_eq!(outcome.authority_effect, "none");
    assert!(RecommendationOutcome::attempt_execute().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 5 — Adaptation boundaries remain explicit (outcomes inform only).
#[test]
fn case5_adaptation_boundaries_remain_explicit() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    present_then(&mut record, RecommendationLifecycleState::Accepted);
    let outcome = record.record_outcome("t4").unwrap();
    assert!(outcome.may_inform_future_adaptation());
    assert!(!outcome.may_mutate_historical_reasoning());
    assert!(!outcome.may_silently_change_scoring());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
}
