//! DecisionCandidateRanking — comparative ordering only
//! (not selection / planner / Gateway / goals / intents / RE mutation).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate,
    DecisionCandidateEvaluationOriginContract, DecisionCandidateEvaluationOriginInput,
    DecisionCandidateEvaluationResolution, DecisionCandidateEvaluationResolutionInput,
    DecisionCandidateLifecycleIntegration, DecisionCandidateRanking,
    DecisionCandidateRankingMemberInput, DecisionCandidateScore, DecisionCandidateScoreInput,
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

fn native_candidate_with_total(suffix: &str, total: u32) -> DecisionCandidate {
    let source_key = format!("goal:{suffix}");
    DecisionCandidate {
        id: DecisionCandidate::synthetic_id(&source_key),
        workspace_id: WorkspaceId::new("ws-1").unwrap(),
        title: format!("Demo {suffix}"),
        goal_statement: format!("Demo goal {suffix}"),
        originating_goal: None,
        attention_item_id: None,
        recommendation_id: None,
        intake_candidate_id: None,
        creation_request_id: None,
        package_seal_digest: None,
        origin: DecisionCandidate::ORIGIN_NATIVE.into(),
        score: DecisionScore {
            total,
            attention_contribution: total,
            memory_contribution: 0,
            personalization_contribution: 0,
            goal_contribution: 0,
            factors: vec![],
        },
        explanation: workspace_domain::DecisionExplanation {
            headline: format!("Demo {suffix}"),
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

fn scored_member(candidate: DecisionCandidate) -> DecisionCandidateRankingMemberInput {
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
        DecisionCandidateEvaluationResolution::RESOLVE_ACCEPT,
        "admit",
        "t-resolve",
    )
    .expect("accept");
    let score_input = DecisionCandidateScoreInput {
        candidate: candidate.clone(),
        resolution,
        lifecycle_integration: lifecycle_integration.clone(),
        intake_withdrawn_or_invalidated: false,
        existing_score: None,
    };
    let (score, _) = DecisionCandidateScore::try_create(&score_input, "t-score").expect("score");
    DecisionCandidateRankingMemberInput {
        candidate,
        score: Some(score),
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
    }
}

/// CASE 1 — Valid scores can be ranked.
#[test]
fn case1_valid_scores_can_be_ranked() {
    let high = scored_member(native_candidate_with_total("rk-hi", 80));
    let low = scored_member(native_candidate_with_total("rk-lo", 20));
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[high, low]);
    assert!(ranking.ranking_id.starts_with(DecisionCandidateRanking::ID_PREFIX));
    assert_eq!(ranking.entries.len(), 2);
    assert_eq!(ranking.entries[0].rank, 1);
    assert_eq!(ranking.entries[0].score_total, 80);
    assert_eq!(ranking.entries[1].rank, 2);
    assert_eq!(ranking.entries[1].score_total, 20);
    assert!(!ranking.ranking_factors.is_empty());
    assert!(ranking.assert_ranking_only().is_ok());
}

/// CASE 2 — Missing scores cannot rank.
#[test]
fn case2_missing_scores_cannot_rank() {
    let candidate = native_candidate_with_total("rk-miss", 10);
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let input = DecisionCandidateRankingMemberInput {
        candidate,
        score: None,
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
    };
    assert!(DecisionCandidateRanking::try_rank_member(&input).is_err());
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[input]);
    assert!(ranking.entries.is_empty());
}

/// CASE 3 — Invalid provenance blocks ranking.
#[test]
fn case3_invalid_provenance_blocks_ranking() {
    let mut member = scored_member(intake_created_candidate("rk-3"));
    member.candidate.package_seal_digest = None;
    assert!(DecisionCandidateRanking::try_rank_member(&member).is_err());
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[member]);
    assert!(ranking.entries.is_empty());
}

/// CASE 4 — Withdrawn candidates excluded.
#[test]
fn case4_withdrawn_excluded() {
    let mut member = scored_member(intake_created_candidate("rk-4"));
    member.intake_withdrawn_or_invalidated = true;
    assert!(DecisionCandidateRanking::try_rank_member(&member).is_err());
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[member]);
    assert!(ranking.entries.is_empty());
}

/// CASE 5 — Ranking does not select.
#[test]
fn case5_ranking_does_not_select() {
    let member = scored_member(native_candidate_with_total("rk-5", 10));
    let before_outcome = member.candidate.outcome;
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[member.clone()]);
    assert!(!ranking.selects_candidate && !ranking.may_select());
    assert!(ranking.selected_candidate_id.is_none());
    assert!(ranking.attempt_select().is_err());
    assert_eq!(member.candidate.outcome, before_outcome);
    assert_eq!(member.candidate.outcome, DecisionOutcome::Open);
}

/// CASE 6 — Ranking does not planner handoff.
#[test]
fn case6_ranking_does_not_planner_handoff() {
    let ranking = DecisionCandidateRanking::derive(
        "ws-1",
        "t-rank",
        &[scored_member(native_candidate_with_total("rk-6", 10))],
    );
    assert!(!ranking.planner_invoked && !ranking.may_invoke_planner());
    assert!(ranking.handoff_command.is_none());
    assert!(ranking.attempt_invoke_planner().is_err());
}

/// CASE 7 — Ranking does not create goals/intents.
#[test]
fn case7_ranking_does_not_create_goals_intents() {
    let ranking = DecisionCandidateRanking::derive(
        "ws-1",
        "t-rank",
        &[scored_member(native_candidate_with_total("rk-7", 10))],
    );
    assert!(!ranking.creates_goal && !ranking.may_create_goal());
    assert!(!ranking.creates_intent && !ranking.may_create_intent());
    assert!(ranking.attempt_create_goal().is_err());
    assert!(ranking.attempt_create_intent().is_err());
}

/// CASE 8 — Ranking does not call Gateway.
#[test]
fn case8_ranking_does_not_call_gateway() {
    let ranking = DecisionCandidateRanking::derive(
        "ws-1",
        "t-rank",
        &[scored_member(native_candidate_with_total("rk-8", 10))],
    );
    assert!(!ranking.may_invoke_gateway());
    assert!(ranking.attempt_invoke_gateway().is_err());
    assert_blocked(
        "execute",
        DecisionCandidateRanking::attempt_execute().map_err(KernelError::from),
    );
    let _ = CommandHandler::decision_engine_attempt_execute();
}

/// CASE 9 — RE state unchanged.
#[test]
fn case9_re_state_unchanged() {
    let ranking = DecisionCandidateRanking::derive(
        "ws-1",
        "t-rank",
        &[scored_member(intake_created_candidate("rk-9"))],
    );
    assert!(!ranking.mutates_recommendation_engine);
    assert!(!ranking.ownership_transferred);
}

/// CASE 10 — Native and recommendation-derived candidates remain distinguishable.
#[test]
fn case10_origins_distinguishable() {
    let native = scored_member(native_candidate_with_total("rk-10n", 50));
    let intake = scored_member(intake_created_candidate("rk-10i"));
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[native, intake]);
    assert_eq!(ranking.entries.len(), 2);
    let origins: Vec<_> = ranking.entries.iter().map(|e| e.origin.as_str()).collect();
    assert!(origins.contains(&DecisionCandidate::ORIGIN_NATIVE));
    assert!(origins.contains(&DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE));
}

/// CASE 11 — Existing synthesis unchanged.
#[test]
fn case11_synthesis_unchanged() {
    let native = native_candidate_with_total("rk-11", 10);
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
    let member = scored_member(native);
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[member]);
    let with = base.with_candidate_ranking(ranking);
    assert_eq!(with.candidates[0].score.total, score_before);
    assert_eq!(with.candidates[0].outcome, DecisionOutcome::Open);
    assert_eq!(with.candidates.len(), 1);
    assert!(with.candidate_ranking.is_some());
}
