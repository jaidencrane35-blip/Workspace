//! Sprint 257 — Recommendation Decision Intake Adapter Preparation
//! (prepare ≠ invoke / DE ownership / Gateway / handoff).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn prepared_bundle() -> (
    RecommendationDecisionIntakeRequest,
    RecommendationDecisionIntakePackageSeal,
    RecommendationDecisionIntakeAdapterPreparation,
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
    (intake, seal, prep)
}

/// CASE 1 — Prep is active but never invoke/mapping/ownership transfer.
#[test]
fn case1_prepare_is_not_invoke_or_ownership() {
    let (_intake, _seal, prep) = prepared_bundle();
    assert!(prep.is_active_preparation());
    assert_eq!(
        prep.preparation_state,
        RecommendationDecisionIntakeAdapterPreparation::STATE_PREPARED
    );
    assert!(!prep.adapter_invoked);
    assert!(!prep.mapping_performed);
    assert!(!prep.proceed_authorized);
    assert!(!prep.handoff_performed);
    assert!(prep.decision_engine_object_id.is_none());
    assert_eq!(
        prep.current_owner,
        RecommendationDecisionIntakeAdapterPreparation::OWNER_RECOMMENDATION
    );
    assert!(prep.attempt_invoke_adapter().is_err());
    assert!(prep.attempt_perform_mapping().is_err());
    assert!(prep.attempt_create_decision_engine_object().is_err());
    assert!(prep.assert_preparation_is_not_invocation().is_ok());
}

/// CASE 2 — No execution / Gateway path.
#[test]
fn case2_no_execution_or_gateway() {
    let (_intake, _seal, prep) = prepared_bundle();
    assert!(!prep.may_invoke_gateway());
    assert!(!prep.may_create_intent());
    assert!(RecommendationDecisionIntakeAdapterPreparation::attempt_execute().is_err());
    assert!(prep.attempt_handoff().is_err());
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 3 — Seal mismatch blocks active preparation progression.
#[test]
fn case3_invalid_stale_seal_blocks_progression() {
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
    let (mut intake, seal, prep) = prepared_bundle();
    intake.title = "Drifted".into();
    let mismatched = seal.reverify_against(&intake);
    assert!(!mismatched.package_matches_seal);
    let rebound = prep.rebind_to_seal(&mismatched);
    assert!(!rebound.is_active_preparation());
    assert!(!rebound.seal_aligned);
    assert!(rebound.attempt_invoke_adapter().is_err());
    assert!(
        RecommendationDecisionIntakeAdapterPreparation::try_prepare(
            &intake,
            &confirmation,
            &mismatched,
            "t",
        )
        .is_none()
    );
}

/// CASE 4 — Revoke is reversible and keeps ownership on RE.
#[test]
fn case4_revoke_is_reversible_without_de_side_effects() {
    let (_intake, _seal, mut prep) = prepared_bundle();
    prep.revoke("t-revoke").unwrap();
    assert!(!prep.is_active_preparation());
    assert_eq!(
        prep.preparation_state,
        RecommendationDecisionIntakeAdapterPreparation::STATE_REVOKED
    );
    assert!(prep.revoked_at.as_deref() == Some("t-revoke"));
    assert!(prep.decision_engine_object_id.is_none());
    assert!(!prep.adapter_invoked);
    assert!(prep.attempt_invoke_adapter().is_err());
    assert_eq!(
        prep.current_owner,
        RecommendationDecisionIntakeAdapterPreparation::OWNER_RECOMMENDATION
    );
}

/// CASE 5 — Provenance / mapping notes informational only.
#[test]
fn case5_provenance_intact_mapping_not_performed() {
    let (intake, _seal, prep) = prepared_bundle();
    let before = intake.evidence_refs.clone();
    assert!(!prep.mapping_performed);
    assert!(!prep.suggested_mapping_notes.is_empty());
    assert!(prep
        .suggested_mapping_notes
        .iter()
        .any(|n| n.starts_with("never:")));
    assert!(!prep.may_mutate_provenance());
    assert_eq!(intake.evidence_refs, before);
}

/// CASE 6 — Accept/required confirmation still emits no preparation.
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
