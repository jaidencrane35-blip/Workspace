//! DecisionScore (DecisionCandidateScore) — scoring result only
//! (not ranking / selection / planner / Gateway / RE mutation).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate,
    DecisionCandidateEvaluationOriginContract, DecisionCandidateEvaluationOriginInput,
    DecisionCandidateEvaluationResolution, DecisionCandidateEvaluationResolutionInput,
    DecisionCandidateLifecycleIntegration, DecisionCandidateScore, DecisionCandidateScoreInput,
    DecisionContext, DecisionEngineCandidateCreation, DecisionEngineCandidateCreationInput,
    DecisionEngineCandidateCreationRequest, DecisionEngineIntakeAssessment,
    DecisionEngineIntakeAssessmentInput, DecisionEngineIntakeCandidate,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEligibility,
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

fn accepted_score_input(
    candidate: DecisionCandidate,
    intake_withdrawn_or_invalidated: bool,
) -> DecisionCandidateScoreInput {
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let eval_input = DecisionCandidateEvaluationOriginInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        existing_evaluated: None,
    };
    let (evaluation_origin, _) =
        DecisionCandidateEvaluationOriginContract::try_evaluate(&eval_input, "t-eval")
            .expect("evaluate origin");
    let resolution_input = DecisionCandidateEvaluationResolutionInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        evaluation_origin,
        intake_withdrawn_or_invalidated,
        existing_resolution: None,
    };
    let (resolution, _) = DecisionCandidateEvaluationResolution::try_resolve(
        &resolution_input,
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t-resolve",
    )
    .expect("accept for scoring");
    DecisionCandidateScoreInput {
        candidate,
        resolution,
        lifecycle_integration,
        intake_withdrawn_or_invalidated,
        existing_score: None,
    }
}

fn rejected_score_input(candidate: DecisionCandidate) -> DecisionCandidateScoreInput {
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let eval_input = DecisionCandidateEvaluationOriginInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        existing_evaluated: None,
    };
    let (evaluation_origin, _) =
        DecisionCandidateEvaluationOriginContract::try_evaluate(&eval_input, "t-eval")
            .expect("evaluate origin");
    let resolution_input = DecisionCandidateEvaluationResolutionInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        evaluation_origin,
        intake_withdrawn_or_invalidated: false,
        existing_resolution: None,
    };
    let (resolution, _) = DecisionCandidateEvaluationResolution::try_resolve(
        &resolution_input,
        DecisionCandidateEvaluationResolution::RESOLVE_REJECT,
        "decline",
        "t-resolve",
    )
    .expect("reject for scoring");
    DecisionCandidateScoreInput {
        candidate,
        resolution,
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
        existing_score: None,
    }
}

/// CASE 1 — Accepted candidate creates DecisionScore.
#[test]
fn case1_accepted_creates_decision_score() {
    let input = accepted_score_input(intake_created_candidate("ds-1"), false);
    let before = input.candidate.clone();
    let (score, unchanged) =
        DecisionCandidateScore::try_create(&input, "t-score").expect("score");
    assert!(score.score_id.starts_with(DecisionCandidateScore::ID_PREFIX));
    assert_eq!(score.origin, DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE);
    assert!(!score.scoring_factors.is_empty());
    assert_eq!(unchanged.score, before.score);
    assert_eq!(unchanged.outcome, before.outcome);
    assert!(score.assert_score_only().is_ok());
}

/// CASE 2 — Rejected candidate cannot create score.
#[test]
fn case2_rejected_cannot_create_score() {
    let input = rejected_score_input(intake_created_candidate("ds-2"));
    assert!(input.resolution.is_rejected_for_scoring());
    assert!(DecisionCandidateScore::try_create(&input, "t-score").is_err());
}

/// CASE 3 — Blocked candidate cannot create score.
#[test]
fn case3_blocked_cannot_create_score() {
    let mut broken = intake_created_candidate("ds-3");
    broken.package_seal_digest = None;
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&broken);
    let evaluation_origin = DecisionCandidateEvaluationOriginContract::derive(
        &DecisionCandidateEvaluationOriginInput {
            candidate: broken.clone(),
            lifecycle_integration: lifecycle_integration.clone(),
            existing_evaluated: None,
        },
    );
    let resolution = DecisionCandidateEvaluationResolution::derive(
        &DecisionCandidateEvaluationResolutionInput {
            candidate: broken.clone(),
            lifecycle_integration: lifecycle_integration.clone(),
            evaluation_origin,
            intake_withdrawn_or_invalidated: false,
            existing_resolution: None,
        },
    );
    assert!(resolution.is_blocked());
    let input = DecisionCandidateScoreInput {
        candidate: broken,
        resolution,
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
        existing_score: None,
    };
    assert!(DecisionCandidateScore::try_create(&input, "t-score").is_err());
}

/// CASE 4 — Invalid provenance blocks scoring.
#[test]
fn case4_invalid_provenance_blocks_scoring() {
    let mut candidate = intake_created_candidate("ds-4");
    let input = accepted_score_input(candidate.clone(), false);
    // Break provenance after acceptance — scoring must re-check.
    candidate.package_seal_digest = None;
    let blocked = DecisionCandidateScoreInput {
        candidate,
        resolution: input.resolution,
        lifecycle_integration: input.lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
        existing_score: None,
    };
    assert!(DecisionCandidateScore::try_create(&blocked, "t-score").is_err());
}

/// CASE 5 — Withdrawn candidate blocks scoring.
#[test]
fn case5_withdrawn_blocks_scoring() {
    let candidate = intake_created_candidate("ds-5");
    let mut input = accepted_score_input(candidate, false);
    input.intake_withdrawn_or_invalidated = true;
    assert!(DecisionCandidateScore::try_create(&input, "t-score").is_err());
}

/// CASE 6 — Score does not rank candidates.
#[test]
fn case6_score_does_not_rank() {
    let a = intake_created_candidate("ds-6a");
    let b = intake_created_candidate("ds-6b");
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
    let before = DecisionEngineState::from_candidates("ws-1", context.clone(), vec![a.clone(), b.clone()]);
    let order_before: Vec<_> = before.candidates.iter().map(|c| c.id.as_str().to_string()).collect();
    let input_a = accepted_score_input(a.clone(), false);
    let input_b = accepted_score_input(b.clone(), false);
    let (score_a, _) = DecisionCandidateScore::try_create(&input_a, "t-a").unwrap();
    let (score_b, _) = DecisionCandidateScore::try_create(&input_b, "t-b").unwrap();
    assert!(!score_a.ranking_applied && !score_a.may_rank());
    assert!(!score_b.ranking_applied && !score_b.may_rank());
    assert!(score_a.attempt_rank().is_err());
    let after = DecisionEngineState::from_candidates("ws-1", context, vec![a, b])
        .with_candidate_scores(vec![score_a, score_b]);
    let order_after: Vec<_> = after.candidates.iter().map(|c| c.id.as_str().to_string()).collect();
    assert_eq!(order_before, order_after);
}

/// CASE 7 — Score does not select candidates.
#[test]
fn case7_score_does_not_select() {
    let input = accepted_score_input(intake_created_candidate("ds-7"), false);
    let (score, unchanged) = DecisionCandidateScore::try_create(&input, "t-score").unwrap();
    assert!(!score.selects_candidate && !score.may_select());
    assert!(score.attempt_select().is_err());
    assert_eq!(unchanged.outcome, DecisionOutcome::Open);
}

/// CASE 8 — Score does not call planner.
#[test]
fn case8_score_does_not_call_planner() {
    let input = accepted_score_input(intake_created_candidate("ds-8"), false);
    let (score, _) = DecisionCandidateScore::try_create(&input, "t-score").unwrap();
    assert!(!score.planner_invoked && !score.may_invoke_planner());
    assert!(score.handoff_command.is_none());
    assert!(score.attempt_invoke_planner().is_err());
}

/// CASE 9 — Score does not call Gateway.
#[test]
fn case9_score_does_not_call_gateway() {
    let input = accepted_score_input(intake_created_candidate("ds-9"), false);
    let (score, _) = DecisionCandidateScore::try_create(&input, "t-score").unwrap();
    assert!(!score.may_invoke_gateway());
    assert!(score.attempt_invoke_gateway().is_err());
    assert_blocked(
        "execute",
        DecisionCandidateScore::attempt_execute().map_err(KernelError::from),
    );
    let _ = CommandHandler::decision_engine_attempt_execute();
}

/// CASE 10 — RE state remains unchanged.
#[test]
fn case10_re_state_unchanged() {
    let input = accepted_score_input(intake_created_candidate("ds-10"), false);
    let (score, _) = DecisionCandidateScore::try_create(&input, "t-score").unwrap();
    assert!(!score.mutates_recommendation_engine);
    assert!(!score.ownership_transferred);
}

/// CASE 11 — Native and recommendation-derived scoring remain distinguishable.
#[test]
fn case11_origins_distinguishable() {
    let native_input = accepted_score_input(native_candidate(), false);
    let intake_input = accepted_score_input(intake_created_candidate("ds-11"), false);
    let (native_score, _) = DecisionCandidateScore::try_create(&native_input, "t-n").unwrap();
    let (intake_score, _) = DecisionCandidateScore::try_create(&intake_input, "t-i").unwrap();
    assert_eq!(native_score.origin, DecisionCandidate::ORIGIN_NATIVE);
    assert_eq!(
        intake_score.origin,
        DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
    );
    assert_ne!(native_score.origin, intake_score.origin);
    assert!(native_score
        .scoring_factors
        .iter()
        .any(|f| f.contains("native")));
    assert!(intake_score
        .scoring_factors
        .iter()
        .any(|f| f.contains("recommendation_intake")));
}

/// CASE 12 — Existing DE synthesis remains unchanged.
#[test]
fn case12_synthesis_unchanged() {
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
    let input = accepted_score_input(native.clone(), false);
    let (score, unchanged) = DecisionCandidateScore::try_create(&input, "t-score").unwrap();
    let with = base.with_candidate_scores(vec![score]);
    assert_eq!(with.candidates[0].score.total, score_before);
    assert_eq!(unchanged.score.total, score_before);
    assert_eq!(with.candidates.len(), 1);
    assert_eq!(with.candidate_scores.len(), 1);
}
