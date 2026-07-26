//! Sprint 222 — Recommendation Decision Boundary contract (non-executing).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionBoundary, RecommendationDecisionContext, RecommendationDecisionReadiness,
    RecommendationEvidence, RecommendationExplanationView, RecommendationItem, RecommendationKind,
    RecommendationOutcomeView,
};

fn assert_blocked(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot become")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected boundary rejection, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed"),
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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn boundary_for(item: &RecommendationItem) -> RecommendationDecisionBoundary {
    let context = RecommendationDecisionContext::assemble("ws-1", item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    RecommendationDecisionBoundary::from_context_and_readiness(&context, &readiness)
}

/// CASE 1 — Acceptance remains non-executing recommendation agreement.
#[test]
fn case1_acceptance_remains_non_executing() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let boundary = boundary_for(&item);
    assert!(boundary.accepted_as_recommendation_decision);
    assert_eq!(
        boundary.user_intent_kind,
        RecommendationDecisionBoundary::INTENT_AGREEMENT
    );
    assert!(!boundary.creates_intent);
    assert!(!boundary.grants_execution_authority);
    assert!(!boundary.handoff_performed);
    assert_eq!(
        boundary.handoff_state,
        RecommendationDecisionBoundary::HANDOFF_NOT_PERFORMED
    );
    assert_eq!(boundary.authority_effect, "none");
    assert!(boundary.assert_rejection_guards().is_ok());
    assert!(RecommendationDecisionBoundary::attempt_execute().is_err());
    assert!(boundary.attempt_authorize_execution().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 2 — Context readiness cannot create Decision Engine objects.
#[test]
fn case2_context_readiness_cannot_create_de_objects() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    assert!(context.complete);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    assert!(readiness.ready_for_future_handoff);
    let boundary =
        RecommendationDecisionBoundary::from_context_and_readiness(&context, &readiness);
    assert_eq!(
        boundary.transition_state,
        RecommendationDecisionBoundary::STATE_AWAITING_DECISION_ENGINE_INTAKE
    );
    assert!(!boundary.creates_decision_engine_object);
    assert!(boundary.attempt_create_decision_engine_object().is_err());
    assert!(boundary.attempt_create_intent().is_err());
    assert!(boundary.attempt_handoff().is_err());
    assert!(context.decision_engine_object_id.is_none());
}

/// CASE 3 — No Gateway path exists from boundary.
#[test]
fn case3_no_gateway_path_exists() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let boundary = boundary_for(&item);
    assert_eq!(
        boundary.execution_owner,
        RecommendationDecisionBoundary::OWNER_GATEWAY
    );
    assert!(!boundary.may_invoke_gateway());
    assert!(!boundary.may_grant_execution_authority());
    assert!(boundary.attempt_authorize_execution().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 4 — Provenance remains immutable across boundary derivation.
#[test]
fn case4_provenance_remains_immutable() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before_evidence = item.evidence.clone();
    let before_reasons = item.attention_reasons.clone();
    let _boundary = boundary_for(&item);
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(item.attention_reasons, before_reasons);
    assert!(!_boundary.may_mutate_provenance());
}

/// CASE 5 — Ownership boundaries remain intact.
#[test]
fn case5_ownership_boundaries_remain_intact() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let boundary = boundary_for(&item);
    assert_eq!(
        boundary.recommendation_owner,
        RecommendationDecisionBoundary::OWNER_RECOMMENDATION
    );
    assert_eq!(
        boundary.decision_owner,
        RecommendationDecisionBoundary::OWNER_DECISION
    );
    assert_eq!(
        boundary.execution_owner,
        RecommendationDecisionBoundary::OWNER_GATEWAY
    );
    assert_eq!(
        boundary.governance_owner,
        RecommendationDecisionBoundary::OWNER_GOVERNANCE
    );
    assert_eq!(
        boundary.experience_owner,
        RecommendationDecisionBoundary::OWNER_EXPERIENCE
    );
    assert_ne!(boundary.recommendation_owner, boundary.decision_owner);
    assert_ne!(boundary.recommendation_owner, boundary.execution_owner);
    assert!(boundary.assert_rejection_guards().is_ok());
}

/// CASE 6 — Incomplete context stays recommendation_only (accepted ≠ intake).
#[test]
fn case6_incomplete_context_stays_recommendation_only() {
    let mut item = base_item();
    item.related_attention_id = None;
    item.related_task_id = None;
    item.related_decision_id = None;
    item.related_purpose_label = None;
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let boundary = boundary_for(&item);
    assert!(boundary.accepted_as_recommendation_decision);
    assert_eq!(
        boundary.transition_state,
        RecommendationDecisionBoundary::STATE_RECOMMENDATION_ONLY
    );
    assert_eq!(
        boundary.handoff_state,
        RecommendationDecisionBoundary::HANDOFF_NOT_PERFORMED
    );
    assert!(!boundary.creates_intent);
}
