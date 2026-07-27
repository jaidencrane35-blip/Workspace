//! Decision Engine Intake Eligibility — observational only
//! (eligible ≠ DecisionCandidate / goal / intent / planner / Gateway / score / rank).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionEngineIntakeAssessment,
    DecisionEngineIntakeAssessmentInput, DecisionEngineIntakeEligibility,
    DecisionEngineIntakeReceipt, RecommendationConfidence, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionEngineAcceptance,
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

fn assess_eligible(receipt: DecisionEngineIntakeReceipt) -> DecisionEngineIntakeAssessment {
    DecisionEngineIntakeAssessment::assess_batch(&[DecisionEngineIntakeAssessmentInput {
        receipt,
        acceptance_active: true,
        lifecycle_superseded: false,
        receipt_current: true,
    }])
    .remove(0)
}

fn eligibility_for(
    receipt: &DecisionEngineIntakeReceipt,
    assessment: &DecisionEngineIntakeAssessment,
) -> DecisionEngineIntakeEligibility {
    DecisionEngineIntakeEligibility::derive(receipt, assessment)
}

/// CASE 1 — Eligible receipt does not create DecisionCandidate.
#[test]
fn case1_eligible_does_not_create_candidate() {
    let (receipt, acceptance, _seal) = accepted_chain("task-1");
    let acceptance_before = acceptance.clone();
    let assessment = assess_eligible(receipt.clone());
    let eligibility = eligibility_for(&receipt, &assessment);
    assert!(eligibility.is_eligible);
    assert_eq!(
        eligibility.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_ELIGIBLE
    );
    assert!(!eligibility.creates_decision_candidate);
    assert!(eligibility.decision_engine_object_id.is_none());
    assert!(eligibility.attempt_create_decision_candidate().is_err());
    assert_eq!(acceptance, acceptance_before);
}

/// CASE 2 — Blocked assessment never becomes eligible.
#[test]
fn case2_blocked_assessment_never_eligible() {
    let (mut receipt, _, _) = accepted_chain("task-2");
    receipt.seal_aligned = false;
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
    let assessment = assess_eligible(receipt.clone());
    assert_eq!(
        assessment.assessment_state,
        DecisionEngineIntakeAssessment::STATE_BLOCKED
    );
    let eligibility = eligibility_for(&receipt, &assessment);
    assert!(!eligibility.is_eligible);
    assert_eq!(
        eligibility.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_BLOCKED
    );
}

/// CASE 3 — Duplicate remains ineligible.
#[test]
fn case3_duplicate_remains_ineligible() {
    let (mut a, _, _) = accepted_chain("task-a");
    let (mut b, _, _) = accepted_chain("task-b");
    let digest = a.sealed_intake_package_digest.clone();
    b.sealed_intake_package_digest = digest;
    let assessments = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: a.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: true,
        },
        DecisionEngineIntakeAssessmentInput {
            receipt: b.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: true,
        },
    ]);
    let eligibilities =
        DecisionEngineIntakeEligibility::derive_batch(&[a, b], &assessments);
    let duplicates: Vec<_> = eligibilities.iter().filter(|e| e.is_duplicate).collect();
    let eligible: Vec<_> = eligibilities.iter().filter(|e| e.is_eligible).collect();
    assert_eq!(duplicates.len(), 1);
    assert_eq!(eligible.len(), 1);
    assert_eq!(
        duplicates[0].eligibility_state,
        DecisionEngineIntakeEligibility::STATE_DUPLICATE
    );
    assert!(!duplicates[0].creates_decision_candidate);
}

/// CASE 4 — Superseded remains ineligible.
#[test]
fn case4_superseded_remains_ineligible() {
    let (receipt, _, _) = accepted_chain("task-3");
    let assessment = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active: true,
            lifecycle_superseded: true,
            receipt_current: true,
        },
    ])
    .remove(0);
    let eligibility = eligibility_for(&receipt, &assessment);
    assert!(!eligibility.is_eligible);
    assert_eq!(
        eligibility.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_SUPERSEDED
    );
}

/// CASE 5 — Seal mismatch remains ineligible.
#[test]
fn case5_seal_mismatch_remains_ineligible() {
    let (mut receipt, _, _) = accepted_chain("task-4");
    receipt.seal_aligned = false;
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
    let assessment = assess_eligible(receipt.clone());
    let eligibility = eligibility_for(&receipt, &assessment);
    assert!(!eligibility.is_eligible);
    assert!(!eligibility.seal_aligned);
    assert_ne!(
        eligibility.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_ELIGIBLE
    );
}

/// CASE 6–9 — Eligibility never creates Goal/Intent or invokes planner/Gateway.
#[test]
fn case6_to_9_eligibility_never_creates_or_invokes() {
    let (receipt, _, _) = accepted_chain("task-5");
    let assessment = assess_eligible(receipt.clone());
    let e = eligibility_for(&receipt, &assessment);
    assert!(e.is_eligible);
    assert!(!e.creates_goal);
    assert!(!e.creates_intent);
    assert!(!e.may_create_goal());
    assert!(!e.may_create_intent());
    assert!(!e.may_invoke_planner());
    assert!(!e.may_invoke_gateway());
    assert!(e.attempt_create_goal().is_err());
    assert!(e.attempt_create_intent().is_err());
    assert!(e.attempt_invoke_planner().is_err());
    assert!(DecisionEngineIntakeEligibility::attempt_execute().is_err());
    assert!(e.assert_observational_only().is_ok());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 10 — Recommendation Engine overlays remain unchanged.
#[test]
fn case10_recommendation_overlays_unchanged() {
    let (receipt, acceptance, seal) = accepted_chain("task-6");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    let assessment = assess_eligible(receipt.clone());
    let eligibility = eligibility_for(&receipt, &assessment);
    assert!(eligibility.is_eligible);
    assert!(!eligibility.ownership_transferred);
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
}

/// CASE 11 — Eligibility remains fully recomputable (projection, no persistence).
#[test]
fn case11_eligibility_recomputable() {
    let (receipt, _, _) = accepted_chain("task-7");
    let stale_assessment = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: false,
        },
    ])
    .remove(0);
    let stale = eligibility_for(&receipt, &stale_assessment);
    assert!(!stale.is_eligible);
    assert_eq!(
        stale.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_STALE
    );

    let fresh_assessment = assess_eligible(receipt.clone());
    let recomputed = eligibility_for(&receipt, &fresh_assessment);
    assert!(recomputed.is_eligible);
    assert_eq!(
        recomputed.eligibility_state,
        DecisionEngineIntakeEligibility::STATE_ELIGIBLE
    );
    assert!(!recomputed.ownership_transferred);
    assert!(recomputed.decision_engine_object_id.is_none());
    assert!(recomputed.assert_observational_only().is_ok());
}
