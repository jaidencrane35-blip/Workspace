//! Sprint 267 — Recommendation Decision Engine Acceptance
//! (accept ≠ ownership transfer / DE object / adapter / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionEngineAcceptance, RecommendationDecisionHandoffRequest,
    RecommendationDecisionIntakeAdapterPreparation, RecommendationDecisionIntakeCompatibility,
    RecommendationDecisionIntakeInspection, RecommendationDecisionIntakePackageSeal,
    RecommendationDecisionIntakeProceedDenial, RecommendationDecisionIntakeRequest,
    RecommendationDecisionReadiness, RecommendationEvidence, RecommendationExplanationView,
    RecommendationItem, RecommendationKind, RecommendationOutcomeView,
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

fn awaiting_bundle() -> (
    RecommendationDecisionHandoffRequest,
    RecommendationDecisionEngineAcceptance,
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
    .expect("prep");
    let request = RecommendationDecisionHandoffRequest::try_request(
        &prep,
        &confirmation,
        &seal,
        &compatibility,
        "t-request",
    )
    .expect("request");
    let acceptance =
        RecommendationDecisionEngineAcceptance::derive_from_handoff_request(&request)
            .expect("awaiting acceptance");
    (request, acceptance)
}

/// CASE 1 — Acceptance awaiting ≠ ownership transfer / DE object.
#[test]
fn case1_acceptance_is_not_ownership_transfer_or_de_object() {
    let (request, mut acceptance) = awaiting_bundle();
    assert!(request.is_active_request());
    assert!(acceptance.is_awaiting());
    assert!(!acceptance.ownership_transferred);
    assert!(acceptance.decision_engine_object_id.is_none());
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
    acceptance.accept("t-accept").unwrap();
    assert!(acceptance.is_accepted_for_future());
    assert!(!acceptance.ownership_transferred);
    assert_eq!(
        acceptance.declared_future_owner.as_deref(),
        Some(RecommendationDecisionEngineAcceptance::OWNER_DECISION)
    );
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
    assert!(acceptance.attempt_transfer_ownership().is_err());
    assert!(acceptance.attempt_create_decision_engine_object().is_err());
    assert!(acceptance.assert_acceptance_is_not_ownership_transfer().is_ok());
}

/// CASE 2 — Acceptance ≠ adapter invocation / performed handoff.
#[test]
fn case2_acceptance_is_not_adapter_or_handoff() {
    let (_request, mut acceptance) = awaiting_bundle();
    acceptance.accept("t-accept").unwrap();
    assert!(!acceptance.adapter_invoked);
    assert!(!acceptance.handoff_performed);
    assert!(!acceptance.may_invoke_adapter());
    assert!(acceptance.attempt_invoke_adapter().is_err());
    assert!(acceptance.attempt_perform_handoff().is_err());
}

/// CASE 3 — Acceptance ≠ execution; Gateway untouched.
#[test]
fn case3_acceptance_is_not_execution_gateway_untouched() {
    let (_request, mut acceptance) = awaiting_bundle();
    acceptance.accept("t-accept").unwrap();
    assert!(!acceptance.may_invoke_gateway());
    assert!(!acceptance.may_create_intent());
    assert!(RecommendationDecisionEngineAcceptance::attempt_execute().is_err());
    assert_eq!(
        acceptance.permission_effect,
        RecommendationDecisionEngineAcceptance::PERMISSION_EFFECT_NONE
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

/// CASE 4 — Provenance / seal identity remain intact.
#[test]
fn case4_provenance_remains_immutable() {
    let (request, mut acceptance) = awaiting_bundle();
    let digest = request.sealed_intake_package_digest.clone();
    let version = request.contract_version.clone();
    acceptance.accept("t-accept").unwrap();
    assert!(!acceptance.may_mutate_provenance());
    assert_eq!(acceptance.sealed_intake_package_digest, digest);
    assert_eq!(acceptance.contract_version, version);
    assert_eq!(acceptance.confirmation_intent, request.confirmation_intent);
}

/// CASE 5 — Revoke / inactive request blocks progression.
#[test]
fn case5_revoke_and_inactive_request_block_progression() {
    let (mut request, mut acceptance) = awaiting_bundle();
    acceptance.accept("t-accept").unwrap();
    assert!(acceptance.is_accepted_for_future());
    acceptance.revoke("t-revoke").unwrap();
    assert!(!acceptance.is_accepted_for_future());
    assert!(!acceptance.ownership_transferred);
    assert!(acceptance.attempt_transfer_ownership().is_err());

    let (_request2, awaiting) = awaiting_bundle();
    request.revoke("t-revoke-request").unwrap();
    let rebound = awaiting.rebind_to_handoff_request(&request);
    assert!(!rebound.is_awaiting());
    assert!(
        RecommendationDecisionEngineAcceptance::derive_from_handoff_request(&request).is_none()
    );
}

/// CASE 6 — Decline keeps RE ownership without DE side effects.
#[test]
fn case6_decline_retains_recommendation_ownership() {
    let (_request, mut acceptance) = awaiting_bundle();
    acceptance.decline("t-decline").unwrap();
    assert_eq!(
        acceptance.acceptance_state,
        RecommendationDecisionEngineAcceptance::STATE_DECLINED
    );
    assert_eq!(
        acceptance.ownership_state,
        RecommendationDecisionEngineAcceptance::OWNERSHIP_DECLINED
    );
    assert!(acceptance.declared_future_owner.is_none());
    assert!(!acceptance.ownership_transferred);
    assert!(acceptance.decision_engine_object_id.is_none());
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
}
