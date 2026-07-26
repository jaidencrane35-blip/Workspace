//! Sprint 217 — Recommendation Decision Context contract (assessment/input only).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionContext, RecommendationDecisionReadiness, RecommendationEvidence,
    RecommendationExplanationView, RecommendationItem, RecommendationKind,
    RecommendationOutcomeView,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot become")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected CannotExecute/CannotBecomeHandoff-style error, got {err}"
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
                decision_confirmation: None,
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

/// CASE 1 — Context assembly is read-only / non-executive.
#[test]
fn case1_context_assembly_is_read_only() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before = item.evidence.clone();
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    assert_eq!(
        context.authority_effect,
        RecommendationDecisionContext::AUTHORITY_EFFECT_NONE
    );
    assert!(!context.handoff_performed);
    assert!(context.decision_engine_object_id.is_none());
    assert!(!context.may_create_intent());
    assert!(!context.may_become_decision_engine_object());
    assert!(!context.may_invoke_gateway());
    assert!(!context.may_mutate_provenance());
    assert!(RecommendationDecisionContext::attempt_execute().is_err());
    assert!(context.attempt_handoff().is_err());
    assert_eq!(item.evidence, before);
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 2 — Incomplete context blocks readiness.
#[test]
fn case2_incomplete_context_blocks_readiness() {
    let mut item = base_item();
    item.related_attention_id = None;
    item.related_task_id = None;
    item.related_decision_id = None;
    item.related_purpose_label = None;
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    assert!(!context.complete);
    assert!(context
        .missing
        .iter()
        .any(|m| m == RecommendationDecisionReadiness::PREREQ_DECISION_CONTEXT));
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    assert_eq!(
        readiness.readiness_state,
        RecommendationDecisionReadiness::STATE_BLOCKED
    );
    assert!(!readiness.ready_for_future_handoff);
    assert!(readiness.attempt_handoff().is_err());
}

/// CASE 3 — Accepted recommendation does not become an intent / DE object.
#[test]
fn case3_accepted_recommendation_does_not_become_intent() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    assert!(context.complete);
    assert_eq!(context.user_decision.as_deref(), Some("accepted"));
    assert!(!context.may_create_intent());
    assert!(!context.may_become_decision_engine_object());
    assert!(context.decision_engine_object_id.is_none());
    assert!(!context.handoff_performed);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    assert!(readiness.ready_for_future_handoff);
    assert!(readiness.attempt_handoff().is_err());
    assert_cannot_execute(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 4 — Gateway remains isolated from context assembly.
#[test]
fn case4_gateway_remains_isolated() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    assert!(!context.may_invoke_gateway());
    assert_eq!(context.authority_effect, "none");
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 5 — Provenance remains immutable across context/readiness assessment.
#[test]
fn case5_provenance_remains_immutable() {
    let mut item = base_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before_evidence = item.evidence.clone();
    let before_reasons = item.attention_reasons.clone();
    let before_keys = item
        .explanation
        .as_ref()
        .map(|e| e.explanation_keys.clone())
        .unwrap();
    let context = RecommendationDecisionContext::assemble(
        "ws-1",
        &item,
        &["recommendation_outcome:prior".into()],
    );
    let _readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(item.attention_reasons, before_reasons);
    assert_eq!(
        item.explanation.as_ref().unwrap().explanation_keys,
        before_keys
    );
    assert!(!context.may_mutate_provenance());
    assert!(context.outcome_history_refs.contains(&"recommendation_outcome:prior".into()));
}
