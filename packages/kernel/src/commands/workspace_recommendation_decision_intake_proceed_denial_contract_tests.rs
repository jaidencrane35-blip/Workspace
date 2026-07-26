//! Sprint 247 — Recommendation Decision Intake Proceed Denial
//! (compatible ≠ proceed / consume / adapter / DE ownership / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
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

fn confirmed_bundle() -> (
    RecommendationDecisionIntakeRequest,
    RecommendationDecisionIntakeCompatibility,
    RecommendationDecisionIntakeProceedDenial,
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
            .expect("confirmed ready item must assemble intake");
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
    (intake, compatibility, denial)
}

/// CASE 1 — Compatible package remains identity_pin_only; proceed always denied.
#[test]
fn case1_compatible_is_not_permission_to_proceed() {
    let (_intake, compatibility, denial) = confirmed_bundle();
    assert!(compatibility.compatible);
    assert_eq!(
        denial.eligibility_state,
        RecommendationDecisionIntakeProceedDenial::STATE_IDENTITY_PIN_ONLY
    );
    assert!(denial.compatibility_compatible);
    assert!(!denial.proceed_authorized);
    assert!(!denial.consume_authorized);
    assert!(!denial.adapter_invokable);
    assert!(!denial.may_proceed());
    assert!(!denial.may_consume());
    assert!(!denial.may_invoke_adapter());
    assert_eq!(
        denial.current_owner,
        RecommendationDecisionIntakeProceedDenial::OWNER_RECOMMENDATION
    );
    assert_eq!(
        denial.declared_consumer_role,
        RecommendationDecisionIntakeProceedDenial::CONSUMER_ROLE_PIN_ONLY
    );
    assert_eq!(denial.permission_effect, "none");
    assert_eq!(denial.authority_effect, "none");
    assert!(denial
        .denial_reasons
        .iter()
        .any(|r| r == "compatibility_is_not_permission"));
    assert!(denial.assert_compatible_is_not_permission().is_ok());
    assert!(denial.attempt_authorize_proceed().is_err());
    assert!(denial.attempt_consume().is_err());
    assert!(denial.attempt_invoke_adapter().is_err());
    assert!(denial.attempt_handoff().is_err());
}

/// CASE 2 — No execution authority / Gateway path.
#[test]
fn case2_no_execution_or_gateway() {
    let (_intake, _compat, denial) = confirmed_bundle();
    assert!(!denial.may_invoke_gateway());
    assert!(!denial.may_create_decision_engine_object());
    assert!(!denial.may_create_intent());
    assert!(RecommendationDecisionIntakeProceedDenial::attempt_execute().is_err());
    assert!(denial.attempt_create_decision_engine_object().is_err());
    assert!(denial.attempt_create_intent().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 3 — Identity mismatch blocks progression; still no proceed.
#[test]
fn case3_identity_mismatch_blocks_progression() {
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
            .unwrap();
    let inspection = RecommendationDecisionIntakeInspection::verify(
        &intake,
        &context,
        &confirmation,
        &readiness,
    );
    let mismatched = RecommendationDecisionIntakeCompatibility::evaluate(
        &intake,
        &inspection,
        "recommendation_decision_intake:v99",
        99,
    );
    assert!(!mismatched.compatible);
    let denial =
        RecommendationDecisionIntakeProceedDenial::derive_from_compatibility(&mismatched);
    assert_eq!(
        denial.eligibility_state,
        RecommendationDecisionIntakeProceedDenial::STATE_INCOMPATIBLE_BLOCKED
    );
    assert!(!denial.proceed_authorized);
    assert!(!denial.adapter_invokable);
    assert!(denial
        .denial_reasons
        .iter()
        .any(|r| r.contains("mismatch") || r.contains("blocks_progression")));
    assert!(denial.attempt_authorize_proceed().is_err());
    assert!(denial.decision_engine_object_id.is_none());
}

/// CASE 4 — No DE ownership transfer even when compatible.
#[test]
fn case4_no_de_ownership_transfer() {
    let (_intake, compatibility, denial) = confirmed_bundle();
    assert!(compatibility.compatible);
    assert!(denial.decision_engine_object_id.is_none());
    assert!(!denial.handoff_performed);
    assert_eq!(
        denial.current_owner,
        RecommendationDecisionIntakeProceedDenial::OWNER_RECOMMENDATION
    );
    assert_ne!(
        denial.declared_consumer_role,
        RecommendationDecisionIntakeProceedDenial::OWNER_RECOMMENDATION
    );
    assert!(denial.attempt_create_decision_engine_object().is_err());
    assert!(denial.attempt_handoff().is_err());
}

/// CASE 5 — Provenance remains intact across denial derivation.
#[test]
fn case5_provenance_remains_intact() {
    let (intake, _compat, denial) = confirmed_bundle();
    let before_refs = intake.evidence_refs.clone();
    let before_fingerprint = intake.continuity_fingerprint.clone();
    assert!(!denial.may_mutate_provenance());
    assert!(!intake.may_mutate_provenance());
    assert_eq!(intake.evidence_refs, before_refs);
    assert_eq!(intake.continuity_fingerprint, before_fingerprint);
    assert_eq!(denial.recommendation_id, intake.recommendation_id);
}

/// CASE 6 — Lifecycle / confirmation semantics unchanged: accept still emits no intake path.
#[test]
fn case6_previous_lifecycle_semantics_unchanged() {
    let mut item = accepted_ready_item();
    item.explanation = Some(RecommendationExplanationView::from_item(&item));
    let context = RecommendationDecisionContext::assemble("ws-1", &item, &[]);
    let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
    let required = RecommendationDecisionConfirmation::derive_from_boundary(
        &workspace_domain::RecommendationDecisionBoundary::from_context_and_readiness(
            &context, &readiness,
        ),
    );
    assert_eq!(
        required.confirmation_state,
        RecommendationDecisionConfirmation::STATE_REQUIRED
    );
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &required)
            .is_none(),
        "accept/required must not assemble intake or proceed denial"
    );
}
