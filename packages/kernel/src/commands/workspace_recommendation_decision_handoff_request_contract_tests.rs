//! Sprint 262 — Recommendation Decision Handoff Request
//! (request ≠ preparation / DE object / execution / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
    RecommendationDecisionIntakePackageSeal, RecommendationDecisionIntakeProceedDenial,
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
        decision_intake_adapter_preparation: None,
        decision_handoff_request: None,
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn requested_bundle() -> (
    RecommendationDecisionIntakeRequest,
    RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionHandoffRequest,
) {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let mut confirmation = RecommendationDecisionConfirmation::derive_from_boundary(
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
            .expect("intake");
    let inspection = RecommendationDecisionIntakeInspection::verify(
        &intake,
        &context,
        &confirmation,
        &readiness,
    );
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &inspection);
    let denial =
        RecommendationDecisionIntakeProceedDenial::derive_from_compatibility(&compatibility);
    let seal = RecommendationDecisionIntakePackageSeal::derive_from_proceed_denial(
        &intake,
        &compatibility,
        &denial,
        "t-seal",
    );
    let prep = RecommendationDecisionIntakeAdapterPreparation::try_prepare(
        &intake,
        &confirmation,
        &seal,
        "t-prep",
    )
    .expect("matching seal + confirmation must prepare");
    let request = RecommendationDecisionHandoffRequest::try_request(
        &prep,
        &confirmation,
        &seal,
        &compatibility,
        "t-request",
    )
    .expect("active preparation must allow handoff request");
    (intake, prep, request)
}

/// CASE 1 — Preparation ≠ handoff request / performed handoff.
#[test]
fn case1_preparation_is_not_handoff_request() {
    let (_intake, prep, request) = requested_bundle();
    assert!(prep.is_active_preparation());
    assert!(!prep.handoff_performed);
    assert!(prep.attempt_handoff().is_err());
    assert!(request.handoff_requested);
    assert!(!request.handoff_performed);
    assert!(request.is_active_request());
    assert_ne!(
        prep.preparation_state,
        RecommendationDecisionHandoffRequest::STATE_REQUESTED
    );
    assert!(request.assert_request_is_not_performed_handoff().is_ok());
    assert!(request.attempt_perform_handoff().is_err());
}

/// CASE 2 — Request ≠ Decision Engine object.
#[test]
fn case2_request_is_not_decision_engine_object() {
    let (_intake, _prep, request) = requested_bundle();
    assert!(request.decision_engine_object_id.is_none());
    assert!(!request.may_create_decision_engine_object());
    assert!(request.attempt_create_decision_engine_object().is_err());
    assert!(!request.may_create_intent());
    assert!(request.attempt_create_intent().is_err());
    assert_eq!(
        request.current_owner,
        RecommendationDecisionHandoffRequest::OWNER_RECOMMENDATION
    );
}

/// CASE 3 — Request ≠ execution; Gateway untouched.
#[test]
fn case3_request_is_not_execution_gateway_untouched() {
    let (_intake, _prep, request) = requested_bundle();
    assert!(!request.may_invoke_gateway());
    assert!(!request.may_invoke_adapter());
    assert!(!request.adapter_invoked);
    assert!(RecommendationDecisionHandoffRequest::attempt_execute().is_err());
    assert!(request.attempt_invoke_adapter().is_err());
    assert_eq!(
        request.permission_effect,
        RecommendationDecisionHandoffRequest::PERMISSION_EFFECT_NONE
    );
    assert_eq!(
        request.authority_effect,
        RecommendationDecisionHandoffRequest::AUTHORITY_EFFECT_NONE
    );
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 4 — Provenance remains immutable across request.
#[test]
fn case4_provenance_remains_immutable() {
    let (intake, _prep, request) = requested_bundle();
    let before = intake.evidence_refs.clone();
    assert!(!request.may_mutate_provenance());
    assert_eq!(
        request.sealed_intake_package_digest,
        RecommendationDecisionIntakePackageSeal::package_digest(&intake)
    );
    assert_eq!(intake.evidence_refs, before);
    assert_eq!(
        request.contract_version,
        RecommendationDecisionIntakeCompatibility::CONTRACT_VERSION
    );
}

/// CASE 5 — Revoke (prep or request) blocks progression.
#[test]
fn case5_revoke_blocks_progression() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let mut confirmation = RecommendationDecisionConfirmation::derive_from_boundary(
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
    let (intake, mut prep, mut request) = requested_bundle();
    let compatibility = RecommendationDecisionIntakeCompatibility::derive_from_inspection(
        &intake,
        &RecommendationDecisionIntakeInspection::verify(
            &intake,
            &context,
            &confirmation,
            &readiness,
        ),
    );
    let denial =
        RecommendationDecisionIntakeProceedDenial::derive_from_compatibility(&compatibility);
    let seal = RecommendationDecisionIntakePackageSeal::derive_from_proceed_denial(
        &intake,
        &compatibility,
        &denial,
        "t-seal",
    );

    request.revoke("t-revoke-request").unwrap();
    assert!(!request.is_active_request());
    assert!(!request.handoff_requested);
    assert!(!request.handoff_performed);
    assert!(request.attempt_perform_handoff().is_err());

    prep.revoke("t-revoke-prep").unwrap();
    let rebound = request.rebind_to_preparation(&prep);
    assert!(!rebound.is_active_request());
    assert!(!rebound.handoff_requested);
    assert!(
        RecommendationDecisionHandoffRequest::try_request(
            &prep,
            &confirmation,
            &seal,
            &compatibility,
            "t",
        )
        .is_none()
    );
}

/// CASE 6 — Without preparation, no handoff request.
#[test]
fn case6_previous_semantics_unchanged() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let required = RecommendationDecisionConfirmation::derive_from_boundary(
        &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
            &context, &readiness,
        ),
    );
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &required)
            .is_none()
    );
}
