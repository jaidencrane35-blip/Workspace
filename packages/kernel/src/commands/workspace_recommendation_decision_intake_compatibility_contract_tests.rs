//! Sprint 242 — Recommendation Decision Intake Compatibility
//! (version pin / field floor — compatible ≠ handoff / transfer / DE ownership).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn confirmed_bundle() -> (
    RecommendationDecisionContext,
    RecommendationDecisionReadiness,
    RecommendationDecisionConfirmation,
    RecommendationDecisionIntakeRequest,
    RecommendationDecisionIntakeInspection,
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
    (context, readiness, confirmation, intake, inspection)
}

/// CASE 1 — Valid inspectable intake is compatible but never transfer/handoff.
#[test]
fn case1_compatible_is_not_handoff_or_transfer() {
    let (_ctx, _rd, _conf, intake, inspection) = confirmed_bundle();
    assert!(inspection.safe_to_inspect);
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &inspection);
    assert!(compatibility.compatible);
    assert!(compatibility.inspection_valid);
    assert!(compatibility.version_current);
    assert!(compatibility.field_floor_satisfied);
    assert_eq!(
        compatibility.contract_version,
        RecommendationDecisionIntakeCompatibility::CONTRACT_VERSION
    );
    assert_eq!(
        compatibility.schema_version,
        RecommendationDecisionIntakeCompatibility::SCHEMA_VERSION
    );
    assert_eq!(
        compatibility.producer,
        RecommendationDecisionIntakeCompatibility::PRODUCER
    );
    assert_eq!(
        compatibility.declared_consumer,
        RecommendationDecisionIntakeCompatibility::DECLARED_CONSUMER
    );
    assert!(!compatibility.transfer_authorized);
    assert!(!compatibility.may_migrate);
    assert!(!compatibility.handoff_performed);
    assert!(compatibility.decision_engine_object_id.is_none());
    assert_eq!(compatibility.authority_effect, "none");
    assert!(compatibility.assert_non_transfer().is_ok());
    assert!(compatibility.attempt_handoff().is_err());
    assert!(compatibility.attempt_transfer().is_err());
    assert!(compatibility.attempt_migrate().is_err());
}

/// CASE 2 — No execution authority / Gateway path.
#[test]
fn case2_no_execution_or_gateway() {
    let (_ctx, _rd, _conf, intake, inspection) = confirmed_bundle();
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &inspection);
    assert!(!compatibility.may_invoke_gateway());
    assert!(!compatibility.may_create_decision_engine_object());
    assert!(!compatibility.may_create_intent());
    assert!(RecommendationDecisionIntakeCompatibility::attempt_execute().is_err());
    assert!(compatibility.attempt_create_decision_engine_object().is_err());
    assert!(compatibility.attempt_create_intent().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 3 — Schema/contract mismatch → incompatible; no DE ownership.
#[test]
fn case3_version_mismatch_blocks_compatible() {
    let (_ctx, _rd, _conf, intake, inspection) = confirmed_bundle();
    let compatibility = RecommendationDecisionIntakeCompatibility::evaluate(
        &intake,
        &inspection,
        "recommendation_decision_intake:v99",
        99,
    );
    assert!(!compatibility.compatible);
    assert!(!compatibility.version_current);
    assert!(compatibility.decision_engine_object_id.is_none());
    assert!(!compatibility.transfer_authorized);
    assert!(compatibility.attempt_create_decision_engine_object().is_err());
}

/// CASE 4 — Failed / stale inspection cannot progress to compatible.
#[test]
fn case4_invalid_stale_intake_cannot_progress() {
    let (mut context, readiness, confirmation, intake, _) = confirmed_bundle();
    context.continuity_fingerprint = "stale-fingerprint".into();
    let stale = RecommendationDecisionIntakeInspection::verify(
        &intake,
        &context,
        &confirmation,
        &readiness,
    );
    assert!(!stale.safe_to_inspect);
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &stale);
    assert!(!compatibility.compatible);
    assert!(!compatibility.inspection_valid);
    assert!(!compatibility.transfer_authorized);
    assert!(compatibility.attempt_handoff().is_err());
}

/// CASE 5 — Non-authoritative / DE ownership violation → incompatible.
#[test]
fn case5_no_de_ownership_transfer() {
    let (_ctx, _rd, _conf, mut intake, inspection) = confirmed_bundle();
    intake.handoff_performed = true;
    intake.decision_engine_object_id = Some("de:fake".into());
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &inspection);
    assert!(!compatibility.compatible);
    assert!(compatibility.decision_engine_object_id.is_none());
    assert!(!compatibility.transfer_authorized);
    assert!(compatibility.attempt_create_decision_engine_object().is_err());
    assert!(compatibility.assert_non_transfer().is_ok());
}

/// CASE 6 — Provenance/version identity remains intact; may_mutate_provenance false.
#[test]
fn case6_provenance_and_identity_intact() {
    let (_ctx, _rd, _conf, intake, inspection) = confirmed_bundle();
    let before_refs = intake.evidence_refs.clone();
    let before_fingerprint = intake.continuity_fingerprint.clone();
    let compatibility =
        RecommendationDecisionIntakeCompatibility::derive_from_inspection(&intake, &inspection);
    assert!(compatibility.compatible);
    assert_eq!(intake.evidence_refs, before_refs);
    assert_eq!(intake.continuity_fingerprint, before_fingerprint);
    assert!(!intake.may_mutate_provenance());
    assert!(!compatibility.may_mutate_provenance());
    assert_eq!(
        compatibility.required_field_floor,
        RecommendationDecisionIntakeCompatibility::required_field_floor()
    );
}
