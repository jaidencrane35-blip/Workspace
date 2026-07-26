//! Sprint 232 — Recommendation Decision Intake Request (typed package, no DE objects).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionIntakeRequest, RecommendationDecisionReadiness, RecommendationEvidence,
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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn confirmed_intake() -> RecommendationDecisionIntakeRequest {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let mut confirmation =
        RecommendationDecisionConfirmation::derive_from_boundary(
            &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
                &context, &readiness,
            ),
        );
    confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION,
            "t-confirm",
        )
        .unwrap();
    RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &confirmation)
        .expect("confirmed ready item must assemble intake")
}

/// CASE 1 — Intake has no execution authority.
#[test]
fn case1_intake_has_no_execution_authority() {
    let intake = confirmed_intake();
    assert_eq!(intake.authority_effect, "none");
    assert!(RecommendationDecisionIntakeRequest::attempt_execute().is_err());
    assert!(!intake.may_invoke_gateway());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 2 — Intake does not create Decision Engine objects.
#[test]
fn case2_intake_does_not_create_de_objects() {
    let intake = confirmed_intake();
    assert!(intake.decision_engine_object_id.is_none());
    assert!(!intake.may_create_decision_engine_object());
    assert!(intake.attempt_create_decision_engine_object().is_err());
    assert_eq!(
        intake.intake_state,
        RecommendationDecisionIntakeRequest::STATE_REQUESTED
    );
    assert!(!intake.handoff_performed);
    assert!(intake.attempt_handoff().is_err());
    assert!(intake.assert_non_authoritative().is_ok());
}

/// CASE 3 — No Gateway interaction from intake.
#[test]
fn case3_no_gateway_interaction() {
    let intake = confirmed_intake();
    assert!(!intake.may_invoke_gateway());
    assert!(intake.attempt_create_intent().is_err());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 4 — Provenance/history refs remain intact (copied, not mutated).
#[test]
fn case4_provenance_history_remains_intact() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let before_evidence = item.evidence.clone();
    let before_keys = item
        .explanation
        .as_ref()
        .unwrap()
        .explanation_keys
        .clone();
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let mut confirmation =
        RecommendationDecisionConfirmation::derive_from_boundary(
            &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
                &context, &readiness,
            ),
        );
    confirmation
        .confirm(
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION,
            "t-confirm",
        )
        .unwrap();
    let intake =
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &confirmation)
            .unwrap();
    assert_eq!(item.evidence, before_evidence);
    assert_eq!(
        item.explanation.as_ref().unwrap().explanation_keys,
        before_keys
    );
    assert!(!intake.evidence_refs.is_empty());
    assert!(!intake.may_mutate_provenance());
}

/// CASE 5 — Confirmation semantics preserved: accept/required emit no intake.
#[test]
fn case5_confirmation_semantics_preserved() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let confirmation = RecommendationDecisionConfirmation::derive_from_boundary(
        &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
            &context, &readiness,
        ),
    );
    assert_eq!(
        confirmation.confirmation_state,
        RecommendationDecisionConfirmation::STATE_REQUIRED
    );
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &confirmation)
            .is_none()
    );
}

/// CASE 6 — Decline emits no intake; incomplete context blocks intake.
#[test]
fn case6_decline_and_incomplete_block_intake() {
    let mut item = accepted_ready_item();
    item.related_task_id = None;
    item.related_attention_id = None;
    item.related_decision_id = None;
    item.related_purpose_label = None;
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let mut confirmation =
        RecommendationDecisionConfirmation::derive_from_boundary(
            &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
                &context, &readiness,
            ),
        );
    // Incomplete context → not required; decline path for required uses a ready item.
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &confirmation)
            .is_none()
    );

    let mut ready = accepted_ready_item();
    ready.explanation = Some(RecommendationExplanationView::from_item(&ready));
    let ctx = RecommendationDecisionContext::assemble("ws-1", &ready, &[]);
    let rd = RecommendationDecisionReadiness::assess_from_context(&ctx);
    confirmation = RecommendationDecisionConfirmation::derive_from_boundary(
        &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(&ctx, &rd),
    );
    confirmation.decline("t-decline").unwrap();
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&ctx, &rd, &confirmation).is_none()
    );
}
