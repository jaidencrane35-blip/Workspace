//! Decision Engine Intake Promotion Boundary — readiness only
//! (promotion_allowed ≠ DecisionCandidate / planner / Gateway / scoring).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate, DecisionContext,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeDisposition, DecisionEngineIntakeEligibility,
    DecisionEngineIntakeEvaluation, DecisionEngineIntakePromotionBoundary,
    DecisionEngineIntakePromotionBoundaryInput, DecisionEngineIntakeReceipt, DecisionEngineState,
    DecisionOutcome, DecisionScore, RecommendationConfidence, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionEngineAcceptance,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
    RecommendationDecisionIntakePackageSeal, RecommendationDecisionIntakeProceedDenial,
    RecommendationDecisionIntakeRequest, RecommendationDecisionReadiness, RecommendationEvidence,
    RecommendationExplanationView, RecommendationItem, RecommendationKind,
    RecommendationOutcomeView, WorkspaceId,
};

fn assert_blocked(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize")
                    || message.contains("transition"),
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

fn retained_evaluated(id_suffix: &str) -> (
    DecisionEngineIntakeCandidate,
    DecisionEngineIntakeEvaluation,
    DecisionEngineIntakeDisposition,
    RecommendationDecisionEngineAcceptance,
    RecommendationDecisionIntakePackageSeal,
) {
    let (receipt, acceptance, seal) = accepted_chain(id_suffix);
    let assessment = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: true,
        },
    ])
    .remove(0);
    let eligibility = DecisionEngineIntakeEligibility::derive(&receipt, &assessment);
    let candidate =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t-create")
            .expect("candidate");
    let evaluation = DecisionEngineIntakeEvaluation::try_evaluate(
        &candidate,
        DecisionEngineIntakeEvaluation::STATE_EVALUATED,
        "examined",
        "t-eval",
    )
    .expect("evaluation");
    let disposition = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_RETAINED,
        "retain",
        "t-disp",
    )
    .expect("disposition");
    (candidate, evaluation, disposition, acceptance, seal)
}

fn boundary_for(
    candidate: DecisionEngineIntakeCandidate,
    evaluation: Option<DecisionEngineIntakeEvaluation>,
    disposition: Option<DecisionEngineIntakeDisposition>,
    acceptance_active: bool,
    seal_aligned: bool,
) -> DecisionEngineIntakePromotionBoundary {
    DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
        candidate,
        evaluation,
        disposition,
        acceptance_active,
        seal_aligned,
        previously_promoted: false,
    })
}

/// CASE 1 — Active retained evaluated intake becomes promotion_allowed.
#[test]
fn case1_active_retained_evaluated_promotion_allowed() {
    let (candidate, evaluation, disposition, _, _) = retained_evaluated("promo-1");
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_ALLOWED
    );
    assert!(boundary.is_promotion_allowed());
    assert!(!boundary.creates_decision_candidate);
}

/// CASE 2 — Non-retained disposition blocks promotion.
#[test]
fn case2_non_retained_blocks_promotion() {
    let (candidate, evaluation, _, _, _) = retained_evaluated("promo-2");
    let dismissed = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DISMISSED,
        "decline",
        "t-disp",
    )
    .unwrap();
    let boundary = boundary_for(candidate, Some(evaluation), Some(dismissed), true, true);
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED
    );
}

/// CASE 3 — Withdrawn intake blocks promotion.
#[test]
fn case3_withdrawn_blocks_promotion() {
    let (mut candidate, evaluation, disposition, _, _) = retained_evaluated("promo-3");
    candidate.withdraw("t-w").unwrap();
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED
    );
}

/// CASE 4 — Invalidated intake blocks promotion.
#[test]
fn case4_invalidated_blocks_promotion() {
    let (mut candidate, evaluation, disposition, _, _) = retained_evaluated("promo-4");
    candidate
        .lifecycle
        .invalidate(
            workspace_domain::DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH,
            "t-inv",
        )
        .unwrap();
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED
    );
}

/// CASE 5 — Seal mismatch blocks promotion.
#[test]
fn case5_seal_mismatch_blocks_promotion() {
    let (candidate, evaluation, disposition, _, _) = retained_evaluated("promo-5");
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        false,
    );
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED
    );
    assert!(!boundary.seal_aligned);
}

/// CASE 6 — Acceptance revoke blocks promotion.
#[test]
fn case6_acceptance_revoke_blocks_promotion() {
    let (candidate, evaluation, disposition, mut acceptance, seal) =
        retained_evaluated("promo-6");
    acceptance.revoke("t-revoke").unwrap();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        false,
        true,
    );
    assert_eq!(
        boundary.boundary_state,
        DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED
    );
    assert!(!boundary.acceptance_active);
}

/// CASE 7–8 — Promotion boundary does not create DecisionCandidate or call planner/Gateway.
#[test]
fn case7_to_8_no_candidate_planner_gateway() {
    let (candidate, evaluation, disposition, _, _) = retained_evaluated("promo-7");
    let b = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    assert!(b.is_promotion_allowed());
    assert!(!b.may_create_decision_candidate());
    assert!(!b.may_create_goal());
    assert!(!b.may_create_intent());
    assert!(!b.may_invoke_planner());
    assert!(!b.may_invoke_gateway());
    assert!(b.attempt_create_decision_candidate().is_err());
    assert!(b.attempt_invoke_planner().is_err());
    assert!(DecisionEngineIntakePromotionBoundary::attempt_execute().is_err());
    assert!(b.assert_boundary_only().is_ok());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 9 — RE overlays remain unchanged.
#[test]
fn case9_recommendation_overlays_unchanged() {
    let (candidate, evaluation, disposition, acceptance, seal) = retained_evaluated("promo-8");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    let _ = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
}

/// CASE 10 — Existing Decision Engine synthesis remains unchanged.
#[test]
fn case10_synthesis_unchanged() {
    let context = DecisionContext {
        workspace_id: "ws-1".into(),
        active_project_id: None,
        active_task_id: None,
        attention_item_count: 0,
        memory_highlight_count: 0,
        preference_highlight_count: 0,
        pending_approval_count: 0,
        pending_plan_count: 0,
        task_graph_open_count: 0,
        task_graph_blocked_count: 0,
    };
    let decision = DecisionCandidate {
        id: DecisionCandidate::synthetic_id("goal:demo"),
        workspace_id: WorkspaceId::new("ws-1").unwrap(),
        title: "Demo".into(),
        goal_statement: "Demo goal".into(),
        originating_goal: None,
        attention_item_id: None,
        recommendation_id: None,
        score: DecisionScore {
            total: 10,
            attention_contribution: 10,
            memory_contribution: 0,
            personalization_contribution: 0,
            goal_contribution: 0,
            factors: vec![],
        },
        explanation: workspace_domain::DecisionExplanation {
            headline: "Demo".into(),
            reasons: vec![],
            confidence: "low".into(),
        },
        related_goal_ids: vec![],
        pending_approval_ids: vec![],
        outcome: DecisionOutcome::Open,
        created_at: "t0".into(),
        handoff_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
        authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
    };
    let base = DecisionEngineState::from_candidates("ws-1", context, vec![decision]);
    let scores_before: Vec<_> = base.candidates.iter().map(|c| c.score.total).collect();
    let (candidate, evaluation, disposition, _, _) = retained_evaluated("promo-9");
    let boundary = boundary_for(
        candidate,
        Some(evaluation),
        Some(disposition),
        true,
        true,
    );
    let with = base.with_intake_promotion_boundaries(vec![boundary]);
    assert_eq!(
        with.candidates
            .iter()
            .map(|c| c.score.total)
            .collect::<Vec<_>>(),
        scores_before
    );
    assert_eq!(with.candidates.len(), 1);
    assert_eq!(with.intake_promotion_boundaries.len(), 1);
    assert!(with.intake_promotion_boundaries[0].is_promotion_allowed());
}
