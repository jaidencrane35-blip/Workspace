//! Sprint 227 — Recommendation Decision Confirmation contract (state only).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionBoundary, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionReadiness, RecommendationEvidence,
    RecommendationExplanationView, RecommendationItem, RecommendationKind,
    RecommendationOutcomeView,
};

fn assert_blocked(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected rejection, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed"),
    }
}

fn accepted_ready_item() -> RecommendationItem {
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
        decision_engine_acceptance: None,
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn boundary_and_confirmation(
    item: &RecommendationItem,
) -> (
    RecommendationDecisionBoundary,
    RecommendationDecisionConfirmation,
) {
    let context = RecommendationDecisionContext::assemble("ws-1", item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let boundary =
        RecommendationDecisionBoundary::from_context_and_readiness(&context, &readiness);
    let confirmation = RecommendationDecisionConfirmation::derive_from_boundary(&boundary);
    (boundary, confirmation)
}

/// CASE 1 — Accept ≠ confirmation (accept leaves agreement_only / not confirmed).
#[test]
fn case1_accept_is_not_confirmation() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let (boundary, confirmation) = boundary_and_confirmation(&item);
    assert!(boundary.accepted_as_recommendation_decision);
    assert_eq!(
        boundary.user_intent_kind,
        RecommendationDecisionBoundary::INTENT_AGREEMENT
    );
    assert_ne!(
        confirmation.confirmation_state,
        RecommendationDecisionConfirmation::STATE_CONFIRMED
    );
    assert_eq!(
        confirmation.confirmation_intent,
        RecommendationDecisionConfirmation::INTENT_AGREEMENT_ONLY
    );
    assert_eq!(
        confirmation.confirmation_state,
        RecommendationDecisionConfirmation::STATE_REQUIRED
    );
}

/// CASE 2 — Confirmation ≠ intent creation.
#[test]
fn case2_confirmation_is_not_intent_creation() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let (_, mut confirmation) = boundary_and_confirmation(&item);
    confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION,
            "t-confirm",
        )
        .unwrap();
    assert_eq!(
        confirmation.confirmation_state,
        RecommendationDecisionConfirmation::STATE_CONFIRMED
    );
    assert!(!confirmation.creates_intent);
    assert!(!confirmation.creates_decision_engine_object);
    assert!(confirmation.attempt_create_intent().is_err());
    assert!(confirmation.attempt_create_decision_engine_object().is_err());
    assert!(confirmation.assert_non_authoritative().is_ok());
}

/// CASE 3 — Confirmation ≠ execution.
#[test]
fn case3_confirmation_is_not_execution() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let (_, mut confirmation) = boundary_and_confirmation(&item);
    confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_REQUEST_ACTION_REVIEW,
            "t-confirm",
        )
        .unwrap();
    assert!(!confirmation.grants_execution_authority);
    assert!(!confirmation.handoff_performed);
    assert!(RecommendationDecisionConfirmation::attempt_execute().is_err());
    assert!(confirmation.attempt_handoff().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 4 — Gateway remains isolated.
#[test]
fn case4_gateway_remains_isolated() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let (_, confirmation) = boundary_and_confirmation(&item);
    assert_eq!(
        confirmation.execution_owner,
        RecommendationDecisionConfirmation::OWNER_GATEWAY
    );
    assert!(!confirmation.may_invoke_gateway());
    assert!(!confirmation.may_grant_execution_authority());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 5 — Provenance remains immutable across confirmation.
#[test]
fn case5_provenance_remains_immutable() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before_evidence = item.evidence.clone();
    let before_reasons = item.attention_reasons.clone();
    let (_, mut confirmation) = boundary_and_confirmation(&item);
    confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION,
            "t-confirm",
        )
        .unwrap();
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(item.attention_reasons, before_reasons);
    assert!(!confirmation.may_mutate_provenance());
}

/// CASE 6 — Invalid transitions blocked; decline stays non-authoritative.
#[test]
fn case6_invalid_transitions_and_decline() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let (_, mut confirmation) = boundary_and_confirmation(&item);
    assert!(confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_AGREEMENT_ONLY,
            "t-bad",
        )
        .is_err());
    confirmation.decline("t-decline").unwrap();
    assert_eq!(
        confirmation.confirmation_state,
        RecommendationDecisionConfirmation::STATE_DECLINED
    );
    assert!(confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION,
            "t-again",
        )
        .is_err());
    assert!(!confirmation.creates_intent);
    assert!(!confirmation.handoff_performed);
}
