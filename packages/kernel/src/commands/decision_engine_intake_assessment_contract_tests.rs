//! Decision Engine Intake Assessment — observational only
//! (assess ≠ DecisionCandidate / goal / intent / planner / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionEngineIntakeAssessment,
    DecisionEngineIntakeAssessmentInput, DecisionEngineIntakeReceipt, RecommendationConfidence,
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

fn accepted_ready_item(id_suffix: &str) -> RecommendationItem {
    let id = format!("recommendation:resolve_blocker:{id_suffix}");
    RecommendationItem {
        id: id.clone(),
        kind: RecommendationKind::ResolveBlocker,
        title: "Resolve blocker".into(),
        reason: "Blocked work needs attention".into(),
        evidence: vec![RecommendationEvidence {
            id: format!("ev:{id_suffix}"),
            source_model: "attention".into(),
            source_ref: format!("attention:task:{id_suffix}"),
            summary: "Blocked task".into(),
        }],
        impact: "Unblocks progress".into(),
        confidence: RecommendationConfidence::High,
        related_attention_id: Some(format!("attention:task:{id_suffix}")),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::TaskGraph,
            AttentionSignal::BlockedTask,
            40,
            "task.base.blocked",
        )],
        related_task_id: Some(format!("task:{id_suffix}")),
        related_purpose_label: None,
        related_decision_id: None,
        lifecycle_state: Some("accepted".into()),
        lifecycle_presented_at: Some("t-presented".into()),
        lifecycle_resolved_at: Some("t-resolved".into()),
        lifecycle_resolution_type: Some("accepted".into()),
        explanation: None,
        outcome: Some(RecommendationOutcomeView {
            outcome_id: format!("recommendation_outcome:{id}"),
            recommendation_id: id,
            user_decision: "accepted".into(),
            result_kind: "accepted_follow_through".into(),
            lifecycle_resolution: Some("accepted".into()),
            recorded_at: "t-resolved".into(),
            explanation_keys: vec!["task.base.blocked".into()],
            evidence_refs: vec![format!("attention:task:{id_suffix}")],
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

fn accepted_chain(id_suffix: &str) -> (
    DecisionEngineIntakeReceipt,
    RecommendationDecisionEngineAcceptance,
    RecommendationDecisionIntakePackageSeal,
) {
    let mut item = accepted_ready_item(id_suffix);
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
    let mut acceptance =
        RecommendationDecisionEngineAcceptance::derive_from_handoff_request(&request)
            .expect("awaiting");
    acceptance.accept("t-accept").unwrap();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    (receipt, acceptance, seal)
}

fn eligible_input(receipt: DecisionEngineIntakeReceipt) -> DecisionEngineIntakeAssessmentInput {
    DecisionEngineIntakeAssessmentInput {
        receipt,
        acceptance_active: true,
        lifecycle_superseded: false,
        receipt_current: true,
    }
}

/// CASE 1 — Assessment only for active accepted receipts (awaiting → no receipt → no assess).
#[test]
fn case1_assessment_only_for_accepted_receipts() {
    let (receipt, mut acceptance, seal) = accepted_chain("task-1");
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[eligible_input(receipt)]);
    assert_eq!(assessed.len(), 1);
    assert!(assessed[0].eligible_for_future_candidate);

    acceptance.acceptance_state =
        RecommendationDecisionEngineAcceptance::STATE_AWAITING.into();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
}

/// CASE 2 — Seal mismatch prevents eligible state.
#[test]
fn case2_seal_mismatch_blocks_eligible() {
    let (mut receipt, _acceptance, _seal) = accepted_chain("task-2");
    receipt.seal_aligned = false;
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[eligible_input(receipt)]);
    assert_eq!(assessed.len(), 1);
    assert_eq!(
        assessed[0].assessment_state,
        DecisionEngineIntakeAssessment::STATE_BLOCKED
    );
    assert!(!assessed[0].eligible_for_future_candidate);
    assert!(!assessed[0].seal_valid);
}

/// CASE 3 — Duplicate digest detected without candidate creation.
#[test]
fn case3_duplicate_detected_without_candidate() {
    let (mut a, _, _) = accepted_chain("task-a");
    let (mut b, _, _) = accepted_chain("task-b");
    // Force identical digest for duplicate detection.
    let digest = a.sealed_intake_package_digest.clone();
    b.sealed_intake_package_digest = digest;
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[
        eligible_input(a),
        eligible_input(b),
    ]);
    assert_eq!(assessed.len(), 2);
    let duplicates: Vec<_> = assessed.iter().filter(|x| x.is_duplicate).collect();
    let eligible: Vec<_> = assessed
        .iter()
        .filter(|x| x.eligible_for_future_candidate)
        .collect();
    assert_eq!(duplicates.len(), 1);
    assert_eq!(eligible.len(), 1);
    assert_eq!(
        duplicates[0].assessment_state,
        DecisionEngineIntakeAssessment::STATE_DUPLICATE
    );
    assert!(!duplicates[0].creates_decision_candidate);
    assert!(duplicates[0].attempt_create_decision_candidate().is_err());
}

/// CASE 4 — Superseded receipt never becomes eligible.
#[test]
fn case4_superseded_never_eligible() {
    let (receipt, _, _) = accepted_chain("task-3");
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt,
            acceptance_active: true,
            lifecycle_superseded: true,
            receipt_current: true,
        },
    ]);
    assert_eq!(
        assessed[0].assessment_state,
        DecisionEngineIntakeAssessment::STATE_SUPERSEDED
    );
    assert!(!assessed[0].eligible_for_future_candidate);
    assert!(assessed[0].is_superseded);
}

/// CASE 5–9 — Assessment never creates candidate/goal/intent or invokes planner/Gateway.
#[test]
fn case5_to_9_assessment_never_creates_or_invokes() {
    let (receipt, _, _) = accepted_chain("task-4");
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[eligible_input(receipt)]);
    let a = &assessed[0];
    assert!(a.eligible_for_future_candidate);
    assert!(!a.creates_decision_candidate);
    assert!(!a.creates_goal);
    assert!(!a.creates_intent);
    assert!(!a.may_create_decision_candidate());
    assert!(!a.may_create_goal());
    assert!(!a.may_create_intent());
    assert!(!a.may_invoke_planner());
    assert!(!a.may_invoke_gateway());
    assert!(a.attempt_create_decision_candidate().is_err());
    assert!(a.attempt_create_goal().is_err());
    assert!(a.attempt_create_intent().is_err());
    assert!(a.attempt_invoke_planner().is_err());
    assert!(DecisionEngineIntakeAssessment::attempt_execute().is_err());
    assert!(a.assert_observational_only().is_ok());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 10 — Stale (inactive chain) is not eligible; assessment is recomputable projection.
#[test]
fn case10_stale_not_eligible_and_recomputable() {
    let (receipt, _, _) = accepted_chain("task-5");
    let assessed = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: false,
        },
    ]);
    assert_eq!(
        assessed[0].assessment_state,
        DecisionEngineIntakeAssessment::STATE_STALE
    );
    assert!(!assessed[0].eligible_for_future_candidate);
    // Recompute with current chain → eligible again (projection, no persistence).
    let recomputed =
        DecisionEngineIntakeAssessment::assess_batch(&[eligible_input(receipt)]);
    assert!(recomputed[0].eligible_for_future_candidate);
    assert!(!recomputed[0].ownership_transferred);
    assert!(recomputed[0].decision_engine_object_id.is_none());
}
