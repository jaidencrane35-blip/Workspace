//! DecisionCandidate Evaluation Resolution — scoring-path admission only
//! (not DecisionScore / ranking / planner / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate,
    DecisionCandidateEvaluationOriginContract, DecisionCandidateEvaluationOriginInput,
    DecisionCandidateEvaluationResolution, DecisionCandidateEvaluationResolutionInput,
    DecisionCandidateLifecycleIntegration, DecisionContext, DecisionEngineCandidateCreation,
    DecisionEngineCandidateCreationInput, DecisionEngineCandidateCreationRequest,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeDisposition,
    DecisionEngineIntakeEligibility, DecisionEngineIntakeEvaluation,
    DecisionEngineIntakePromotionBoundary, DecisionEngineIntakePromotionBoundaryInput,
    DecisionEngineIntakeReceipt, DecisionEngineState, DecisionOutcome, DecisionScore,
    RecommendationConfidence, RecommendationDecisionConfirmation, RecommendationDecisionContext,
    RecommendationDecisionEngineAcceptance, RecommendationDecisionHandoffRequest,
    RecommendationDecisionIntakeAdapterPreparation, RecommendationDecisionIntakeCompatibility,
    RecommendationDecisionIntakeInspection, RecommendationDecisionIntakePackageSeal,
    RecommendationDecisionIntakeProceedDenial, RecommendationDecisionIntakeRequest,
    RecommendationDecisionReadiness, RecommendationEvidence, RecommendationExplanationView,
    RecommendationItem, RecommendationKind, RecommendationOutcomeView, WorkspaceId,
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

fn native_candidate() -> DecisionCandidate {
    DecisionCandidate {
        id: DecisionCandidate::synthetic_id("goal:demo"),
        workspace_id: WorkspaceId::new("ws-1").unwrap(),
        title: "Demo".into(),
        goal_statement: "Demo goal".into(),
        originating_goal: None,
        attention_item_id: None,
        recommendation_id: None,
        intake_candidate_id: None,
        creation_request_id: None,
        package_seal_digest: None,
        origin: DecisionCandidate::ORIGIN_NATIVE.into(),
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

fn intake_created_candidate(id_suffix: &str) -> DecisionCandidate {
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
    let boundary =
        DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
            candidate: candidate.clone(),
            evaluation: Some(evaluation),
            disposition: Some(disposition),
            acceptance_active: true,
            seal_aligned: true,
            previously_promoted: false,
        });
    let creation_request = DecisionEngineCandidateCreationRequest::derive(&boundary);
    let input = DecisionEngineCandidateCreationInput {
        creation_request,
        package_seal_digest: candidate.package_seal_digest.clone(),
        intake_candidate: candidate,
        promotion_boundary: boundary,
        existing_creation: None,
    };
    DecisionEngineCandidateCreation::try_create(&input, "t-created")
        .expect("create")
        .1
}

fn evaluated_resolution_input(
    candidate: DecisionCandidate,
    intake_withdrawn_or_invalidated: bool,
) -> DecisionCandidateEvaluationResolutionInput {
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let eval_input = DecisionCandidateEvaluationOriginInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        existing_evaluated: None,
    };
    let (evaluation_origin, _) =
        DecisionCandidateEvaluationOriginContract::try_evaluate(&eval_input, "t-eval")
            .expect("evaluate origin");
    DecisionCandidateEvaluationResolutionInput {
        candidate,
        lifecycle_integration,
        evaluation_origin,
        intake_withdrawn_or_invalidated,
        existing_resolution: None,
    }
}

/// CASE 1 — Valid evaluated candidate can resolve.
#[test]
fn case1_valid_evaluated_can_resolve() {
    let input = evaluated_resolution_input(intake_created_candidate("er-1"), false);
    let projected = DecisionCandidateEvaluationResolution::derive(&input);
    assert!(projected.is_awaiting());
    let (resolved, unchanged) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t-resolve",
    )
    .expect("accept");
    assert!(resolved.is_accepted_for_scoring());
    assert_eq!(unchanged.score, input.candidate.score);
}

/// CASE 2 — Invalid provenance blocks.
#[test]
fn case2_invalid_provenance_blocks() {
    let mut broken = intake_created_candidate("er-2");
    broken.package_seal_digest = None;
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&broken);
    let evaluation_origin = DecisionCandidateEvaluationOriginContract::derive(
        &DecisionCandidateEvaluationOriginInput {
            candidate: broken.clone(),
            lifecycle_integration: lifecycle_integration.clone(),
            existing_evaluated: None,
        },
    );
    let input = DecisionCandidateEvaluationResolutionInput {
        candidate: broken,
        lifecycle_integration,
        evaluation_origin,
        intake_withdrawn_or_invalidated: false,
        existing_resolution: None,
    };
    let projected = DecisionCandidateEvaluationResolution::derive(&input);
    assert!(projected.is_blocked());
    assert!(DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .is_err());
}

/// CASE 3 — Withdrawn lifecycle blocks.
#[test]
fn case3_withdrawn_lifecycle_blocks() {
    let input = evaluated_resolution_input(intake_created_candidate("er-3"), true);
    let projected = DecisionCandidateEvaluationResolution::derive(&input);
    assert!(projected.is_blocked());
    assert!(!projected.candidate_active);
    assert!(DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .is_err());
}

/// CASE 4 — Accepted resolution does not create score.
#[test]
fn case4_accepted_does_not_create_score() {
    let input = evaluated_resolution_input(intake_created_candidate("er-4"), false);
    let score_before = input.candidate.score.clone();
    let (resolved, after) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    assert!(!resolved.creates_decision_score);
    assert!(!resolved.scoring_applied);
    assert!(!resolved.may_create_decision_score());
    assert!(resolved.attempt_create_decision_score().is_err());
    assert_eq!(after.score, score_before);
    assert_eq!(after.score, DecisionCandidate::unscored());
}

/// CASE 5 — Accepted resolution does not rank.
#[test]
fn case5_accepted_does_not_rank() {
    let input = evaluated_resolution_input(intake_created_candidate("er-5"), false);
    let (resolved, _) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    assert!(!resolved.ranking_applied);
    assert!(!resolved.may_rank());
    assert!(resolved.attempt_rank().is_err());
}

/// CASE 6 — No planner/Gateway/goal/intent path exists.
#[test]
fn case6_no_planner_gateway_goal_intent() {
    let input = evaluated_resolution_input(intake_created_candidate("er-6"), false);
    let (resolved, after) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    assert!(!resolved.planner_invoked);
    assert!(!resolved.may_invoke_planner());
    assert!(!resolved.may_invoke_gateway());
    assert!(!resolved.may_create_goal());
    assert!(!resolved.may_create_intent());
    assert!(resolved.attempt_invoke_planner().is_err());
    assert!(resolved.attempt_create_goal().is_err());
    assert!(resolved.attempt_create_intent().is_err());
    assert!(DecisionCandidateEvaluationResolution::attempt_execute().is_err());
    assert!(after.handoff_command.is_empty());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 7 — RE overlays unchanged.
#[test]
fn case7_recommendation_overlays_unchanged() {
    let candidate = intake_created_candidate("er-7");
    let before_rec = candidate.recommendation_id.clone();
    let before_seal = candidate.package_seal_digest.clone();
    let input = evaluated_resolution_input(candidate, false);
    let (resolved, after) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_REJECT,
        "decline",
        "t",
    )
    .unwrap();
    assert!(resolved.is_rejected_for_scoring());
    assert!(!resolved.mutates_recommendation_engine);
    assert_eq!(after.recommendation_id, before_rec);
    assert_eq!(after.package_seal_digest, before_seal);
}

/// CASE 8 — Native DecisionCandidate lifecycle remains unchanged.
#[test]
fn case8_native_lifecycle_unchanged() {
    let native = native_candidate();
    let outcome_before = native.outcome;
    let input = evaluated_resolution_input(native, false);
    let (_, after) = DecisionCandidateEvaluationResolution::try_resolve(
        &input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    assert_eq!(after.outcome, outcome_before);
    assert_eq!(after.origin, DecisionCandidate::ORIGIN_NATIVE);
    let integration = DecisionCandidateLifecycleIntegration::derive(&after);
    assert!(integration.is_integrated());
}

/// CASE 9 — Recommendation-derived and native candidates remain distinguishable.
#[test]
fn case9_origins_distinguishable() {
    let native = DecisionCandidateEvaluationResolution::derive(&evaluated_resolution_input(
        native_candidate(),
        false,
    ));
    let intake = DecisionCandidateEvaluationResolution::derive(&evaluated_resolution_input(
        intake_created_candidate("er-9"),
        false,
    ));
    assert_eq!(native.origin, DecisionCandidate::ORIGIN_NATIVE);
    assert_eq!(
        intake.origin,
        DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
    );
    assert_ne!(native.origin, intake.origin);
}

/// CASE 10 — Existing synthesis behaviour unchanged.
#[test]
fn case10_synthesis_unchanged() {
    let native = native_candidate();
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
    let base = DecisionEngineState::from_candidates("ws-1", context, vec![native.clone()]);
    let score_before = base.candidates[0].score.total;
    let intake = intake_created_candidate("er-10");
    let native_input = evaluated_resolution_input(native.clone(), false);
    let intake_input = evaluated_resolution_input(intake.clone(), false);
    let (native_res, _) = DecisionCandidateEvaluationResolution::try_resolve(
        &native_input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    let (intake_res, _) = DecisionCandidateEvaluationResolution::try_resolve(
        &intake_input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t",
    )
    .unwrap();
    let with = DecisionEngineState::from_candidates(
        "ws-1",
        base.context.clone(),
        vec![native, intake],
    )
    .with_evaluation_resolutions(vec![native_res, intake_res]);
    let native_in_state = with
        .candidates
        .iter()
        .find(|c| c.origin == DecisionCandidate::ORIGIN_NATIVE)
        .unwrap();
    assert_eq!(native_in_state.score.total, score_before);
    assert_eq!(with.evaluation_resolutions.len(), 2);
}
