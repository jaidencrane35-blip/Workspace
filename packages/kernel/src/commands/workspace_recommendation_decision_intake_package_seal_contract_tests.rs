//! Sprint 252 — Recommendation Decision Intake Package Seal
//! (frozen digest — seal ≠ proceed / adapter / handoff / DE ownership).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
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
        decision_engine_acceptance: None,
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn sealed_bundle() -> (
    RecommendationDecisionIntakeRequest,
    RecommendationDecisionIntakePackageSeal,
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
    let seal = RecommendationDecisionIntakePackageSeal::derive_from_proceed_denial(
        &intake,
        &compatibility,
        &denial,
        "t-seal",
    );
    (intake, seal)
}

/// CASE 1 — Compatible denial seals package; proceed/adapter remain denied.
#[test]
fn case1_seal_does_not_authorize_proceed_or_adapter() {
    let (intake, seal) = sealed_bundle();
    assert!(seal.sealed);
    assert_eq!(
        seal.seal_state,
        RecommendationDecisionIntakePackageSeal::STATE_SEALED
    );
    assert!(seal.package_matches_seal);
    assert!(seal.assert_matches_intake(&intake).is_ok());
    assert!(!seal.proceed_authorized);
    assert!(!seal.consume_authorized);
    assert!(!seal.adapter_invokable);
    assert!(!seal.may_proceed());
    assert!(!seal.may_invoke_adapter());
    assert!(seal.attempt_authorize_proceed().is_err());
    assert!(seal.attempt_invoke_adapter().is_err());
    assert!(seal.attempt_mutate_after_seal().is_err());
    assert!(seal.assert_seal_is_not_handoff().is_ok());
    assert_eq!(
        seal.current_owner,
        RecommendationDecisionIntakePackageSeal::OWNER_RECOMMENDATION
    );
}

/// CASE 2 — No execution authority / Gateway path.
#[test]
fn case2_no_execution_or_gateway() {
    let (_intake, seal) = sealed_bundle();
    assert!(!seal.may_invoke_gateway());
    assert!(!seal.may_create_decision_engine_object());
    assert!(!seal.may_create_intent());
    assert!(RecommendationDecisionIntakePackageSeal::attempt_execute().is_err());
    assert!(seal.attempt_create_decision_engine_object().is_err());
    assert!(seal.attempt_handoff().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 3 — Drifted / stale intake fails seal match; cannot progress.
#[test]
fn case3_stale_intake_cannot_progress() {
    let (mut intake, seal) = sealed_bundle();
    intake.title = "Drifted title".into();
    let verified = seal.reverify_against(&intake);
    assert!(!verified.package_matches_seal);
    assert_eq!(
        verified.seal_state,
        RecommendationDecisionIntakePackageSeal::STATE_SEAL_MISMATCH
    );
    assert!(verified.assert_matches_intake(&intake).is_err());
    assert!(!verified.proceed_authorized);
    assert!(!verified.adapter_invokable);
    assert!(verified.attempt_invoke_adapter().is_err());
    assert!(verified.attempt_authorize_proceed().is_err());
}

/// CASE 4 — No DE ownership transfer from seal.
#[test]
fn case4_no_de_ownership_transfer() {
    let (_intake, seal) = sealed_bundle();
    assert!(seal.decision_engine_object_id.is_none());
    assert!(!seal.handoff_performed);
    assert_eq!(
        seal.current_owner,
        RecommendationDecisionIntakePackageSeal::OWNER_RECOMMENDATION
    );
    assert!(seal.attempt_create_decision_engine_object().is_err());
    assert!(seal.attempt_create_intent().is_err());
    assert!(seal.attempt_handoff().is_err());
}

/// CASE 5 — Provenance remains intact; digest is stable for unchanged intake.
#[test]
fn case5_provenance_and_digest_intact() {
    let (intake, seal) = sealed_bundle();
    let before_refs = intake.evidence_refs.clone();
    let before_fingerprint = intake.continuity_fingerprint.clone();
    let digest = RecommendationDecisionIntakePackageSeal::package_digest(&intake);
    assert_eq!(seal.intake_package_digest, digest);
    assert_eq!(
        seal.continuity_fingerprint_at_seal,
        intake.continuity_fingerprint
    );
    assert!(!seal.may_mutate_provenance());
    assert!(!intake.may_mutate_provenance());
    assert_eq!(intake.evidence_refs, before_refs);
    assert_eq!(intake.continuity_fingerprint, before_fingerprint);
}

/// CASE 6 — Accept / required confirmation still emits no seal.
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
    assert!(
        RecommendationDecisionIntakeRequest::try_assemble(&context, &readiness, &required)
            .is_none(),
        "accept/required must not assemble intake or seal"
    );
}
