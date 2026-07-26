//! Sprint 212 — Recommendation → Decision Engine readiness contract (informational only).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionReadiness, RecommendationEvidence, RecommendationExplanationView,
    RecommendationItem, RecommendationKind, RecommendationOutcomeView,
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

fn base_item() -> RecommendationItem {
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
        lifecycle_state: Some("accepted".into()),
        lifecycle_presented_at: Some("t-presented".into()),
        lifecycle_resolved_at: Some("t-resolved".into()),
        lifecycle_resolution_type: Some("accepted".into()),
        explanation: None,
        outcome: Some(RecommendationOutcomeView {
            outcome_id: "recommendation_outcome:recommendation:resolve_blocker:task-9".into(),
            recommendation_id: "recommendation:resolve_blocker:task-9".into(),
            user_decision: "accepted".into(),
            result_kind: "accepted_follow_through".into(),
            lifecycle_resolution: Some("accepted".into()),
            recorded_at: "t-resolved".into(),
            explanation_keys: vec!["task.base.blocked".into()],
            evidence_refs: vec!["attention:task:9".into()],
            experience_trace_match_keys: vec!["prefix_suffix:task.base.blocked".into()],
            is_system_failure: false,
            authority_effect: RecommendationOutcomeView::AUTHORITY_EFFECT_NONE.into(),
        }),
        decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
        decision_handoff_request: None,
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

/// CASE 1 — Readiness is informational only (no commands, no Gateway).
#[test]
fn case1_readiness_is_informational_only() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let readiness = RecommendationDecisionReadiness::assess(&item);
    assert_eq!(
        readiness.authority_effect,
        RecommendationDecisionReadiness::AUTHORITY_EFFECT_NONE
    );
    assert!(!readiness.may_create_decision_commands());
    assert!(!readiness.may_invoke_gateway());
    assert!(!readiness.may_mutate_provenance());
    assert!(RecommendationDecisionReadiness::attempt_execute().is_err());
    assert!(readiness.attempt_handoff().is_err());
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 2 — Missing decision context blocks readiness even after accept/outcome.
#[test]
fn case2_missing_context_blocks_readiness() {
    let mut item = base_item();
    item.related_attention_id = None;
    item.related_task_id = None;
    item.related_decision_id = None;
    item.related_purpose_label = None;
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let readiness = RecommendationDecisionReadiness::assess(&item);
    assert_eq!(
        readiness.readiness_state,
        RecommendationDecisionReadiness::STATE_BLOCKED
    );
    assert!(!readiness.ready_for_future_handoff);
    assert!(readiness
        .missing
        .iter()
        .any(|m| m == RecommendationDecisionReadiness::PREREQ_DECISION_CONTEXT));
    assert!(!readiness.may_create_decision_commands());
}

/// CASE 3 — Complete prerequisites yield handoff_deferred (still no execution).
#[test]
fn case3_complete_prerequisites_are_handoff_deferred() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let readiness = RecommendationDecisionReadiness::assess(&item);
    assert!(readiness.missing.is_empty(), "missing={:?}", readiness.missing);
    assert_eq!(
        readiness.readiness_state,
        RecommendationDecisionReadiness::STATE_HANDOFF_DEFERRED
    );
    assert!(readiness.ready_for_future_handoff);
    assert!(!readiness.may_create_decision_commands());
    assert!(!readiness.may_invoke_gateway());
    assert!(RecommendationDecisionReadiness::attempt_execute().is_err());
}

/// CASE 4 — Incomplete lifecycle is not ready; provenance remains untouched by assess.
#[test]
fn case4_incomplete_lifecycle_and_immutable_provenance() {
    let mut item = base_item();
    item.lifecycle_state = Some("presented".into());
    item.outcome = None;
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before_evidence = item.evidence.clone();
    let before_reasons = item.attention_reasons.clone();
    let readiness = RecommendationDecisionReadiness::assess(&item);
    assert_eq!(
        readiness.readiness_state,
        RecommendationDecisionReadiness::STATE_INCOMPLETE
    );
    assert!(!readiness.ready_for_future_handoff);
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(item.attention_reasons, before_reasons);
    assert!(!readiness.may_mutate_provenance());
}

/// CASE 5 — Gateway remains isolated from readiness assessment.
#[test]
fn case5_gateway_remains_isolated() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let readiness = RecommendationDecisionReadiness::assess(&item);
    assert_eq!(readiness.authority_effect, "none");
    assert!(!readiness.may_invoke_gateway());
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}
