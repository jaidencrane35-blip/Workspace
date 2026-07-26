//! Decision Engine Intake Receipt — observational only
//! (observe ≠ DecisionCandidate / ownership transfer / planner / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionEngineIntakeReceipt,
    RecommendationConfidence, RecommendationDecisionConfirmation, RecommendationDecisionContext,
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

fn accepted_seal_and_acceptance() -> (
    RecommendationDecisionIntakePackageSeal,
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
    let mut acceptance =
        RecommendationDecisionEngineAcceptance::derive_from_handoff_request(&request)
            .expect("awaiting");
    acceptance.accept("t-accept").unwrap();
    (seal, acceptance)
}

/// CASE 1 — Observed receipt is not a DecisionCandidate / planner handoff.
#[test]
fn case1_receipt_is_not_decision_candidate() {
    let (seal, acceptance) = accepted_seal_and_acceptance();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    assert!(receipt.is_observed());
    assert!(!receipt.creates_decision_candidate);
    assert!(receipt.decision_engine_object_id.is_none());
    assert!(receipt.handoff_command.is_none());
    assert!(receipt.attempt_create_decision_candidate().is_err());
    assert!(receipt.attempt_invoke_planner().is_err());
    assert!(receipt.assert_observational_only().is_ok());
}

/// CASE 2 — Observation does not transfer ownership or create goals/intents.
#[test]
fn case2_receipt_does_not_transfer_or_create_intent() {
    let (seal, acceptance) = accepted_seal_and_acceptance();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    assert!(!receipt.ownership_transferred);
    assert_eq!(
        receipt.current_owner,
        DecisionEngineIntakeReceipt::OWNER_RECOMMENDATION
    );
    assert!(!receipt.creates_goal);
    assert!(!receipt.creates_intent);
    assert!(receipt.attempt_transfer_ownership().is_err());
    assert!(receipt.attempt_create_goal().is_err());
    assert!(receipt.attempt_create_intent().is_err());
}

/// CASE 3 — No adapter / Gateway / execution.
#[test]
fn case3_receipt_is_not_execution_or_gateway() {
    let (seal, acceptance) = accepted_seal_and_acceptance();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    assert!(!receipt.adapter_invoked);
    assert!(!receipt.may_invoke_adapter());
    assert!(!receipt.may_invoke_gateway());
    assert!(receipt.attempt_invoke_adapter().is_err());
    assert!(DecisionEngineIntakeReceipt::attempt_execute().is_err());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
    assert_blocked(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}

/// CASE 4 — Seal mismatch yields non-observed receipt; still no candidate.
#[test]
fn case4_seal_mismatch_blocks_observation() {
    let (mut seal, acceptance) = accepted_seal_and_acceptance();
    seal.package_matches_seal = false;
    seal.seal_state = RecommendationDecisionIntakePackageSeal::STATE_SEAL_MISMATCH.into();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    assert!(!receipt.is_observed());
    assert_eq!(
        receipt.receipt_state,
        DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH
    );
    assert!(!receipt.seal_aligned);
    assert!(receipt.attempt_create_decision_candidate().is_err());
    assert!(!receipt.ownership_transferred);
}

/// CASE 5 — Awaiting / declined acceptance produces no receipt.
#[test]
fn case5_non_accepted_produces_no_receipt() {
    let (seal, mut acceptance) = accepted_seal_and_acceptance();
    acceptance.acceptance_state =
        RecommendationDecisionEngineAcceptance::STATE_AWAITING.into();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
    acceptance.acceptance_state =
        RecommendationDecisionEngineAcceptance::STATE_DECLINED.into();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
}

/// CASE 6 — Provenance identity preserved; RE ownership acknowledged.
#[test]
fn case6_provenance_and_namespace_intact() {
    let (seal, acceptance) = accepted_seal_and_acceptance();
    let digest = acceptance.sealed_intake_package_digest.clone();
    let version = acceptance.contract_version.clone();
    let receipt = DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).expect("receipt");
    assert_eq!(receipt.sealed_intake_package_digest, digest);
    assert_eq!(receipt.contract_version, version);
    assert_eq!(receipt.recommendation_id, acceptance.recommendation_id);
    assert!(!receipt.recommendation_id.starts_with("engine_decision:"));
    assert_eq!(
        receipt.current_owner,
        DecisionEngineIntakeReceipt::OWNER_RECOMMENDATION
    );
}
