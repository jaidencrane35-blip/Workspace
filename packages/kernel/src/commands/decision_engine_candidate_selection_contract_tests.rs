//! DecisionCandidateSelection — progression decision after ranking
//! (not execution / planner / Gateway / goals / intents / RE mutation).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate,
    DecisionCandidateEvaluationOriginContract, DecisionCandidateEvaluationOriginInput,
    DecisionCandidateEvaluationResolution, DecisionCandidateEvaluationResolutionInput,
    DecisionCandidateLifecycleIntegration, DecisionCandidateRanking,
    DecisionCandidateRankingMemberInput, DecisionCandidateScore, DecisionCandidateScoreInput,
    DecisionCandidateSelection, DecisionCandidateSelectionInput, DecisionContext,
    DecisionEngineCandidateCreation, DecisionEngineCandidateCreationInput,
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

fn scored_ranked(candidate: DecisionCandidate) -> (DecisionCandidate, DecisionCandidateScore, DecisionCandidateRanking) {
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let eval_input = DecisionCandidateEvaluationOriginInput {
        candidate: candidate.clone(),
        lifecycle_integration: lifecycle_integration.clone(),
        existing_evaluated: None,
    };
    let (evaluation_origin, _) =
        DecisionCandidateEvaluationOriginContract::try_evaluate(&eval_input, "t-eval")
            .expect("evaluate");
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
    let member = DecisionCandidateRankingMemberInput {
        candidate: candidate.clone(),
        score: Some(score.clone()),
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
    };
    let ranking = DecisionCandidateRanking::derive("ws-1", "t-rank", &[member]);
    (candidate, score, ranking)
}

fn awaiting_selection_input(
    candidate: DecisionCandidate,
) -> DecisionCandidateSelectionInput {
    let (candidate, score, ranking) = scored_ranked(candidate);
    let lifecycle_integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    let ranking_entry = ranking
        .entries
        .iter()
        .find(|e| e.decision_candidate_id == candidate.id.as_str())
        .cloned();
    DecisionCandidateSelectionInput {
        candidate,
        score: Some(score),
        ranking: Some(ranking),
        ranking_entry,
        lifecycle_integration,
        intake_withdrawn_or_invalidated: false,
        existing_selection: None,
    }
}

/// CASE 1 — Ranked candidate can be selected.
#[test]
fn case1_ranked_candidate_can_be_selected() {
    let input = awaiting_selection_input(native_candidate_with_total("sel-1", 50));
    let projected = DecisionCandidateSelection::derive(&input);
    assert!(projected.is_awaiting());
    let before = input.candidate.clone();
    let (selected, unchanged) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .expect("select");
    assert!(selected.is_selected());
    assert!(selected.selection_id.starts_with(DecisionCandidateSelection::ID_PREFIX));
    assert_eq!(unchanged.outcome, before.outcome);
    assert_eq!(unchanged.score, before.score);
    assert!(selected.assert_selection_only().is_ok());
}

/// CASE 2 — Missing ranking blocks selection.
#[test]
fn case2_missing_ranking_blocks_selection() {
    let mut input = awaiting_selection_input(native_candidate_with_total("sel-2", 50));
    input.ranking_entry = None;
    assert!(DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .is_err());
}

/// CASE 3 — Missing score blocks selection.
#[test]
fn case3_missing_score_blocks_selection() {
    let mut input = awaiting_selection_input(native_candidate_with_total("sel-3", 50));
    input.score = None;
    assert!(DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .is_err());
}

/// CASE 4 — Invalid provenance blocks selection.
#[test]
fn case4_invalid_provenance_blocks_selection() {
    let mut input = awaiting_selection_input(intake_created_candidate("sel-4"));
    input.candidate.package_seal_digest = None;
    assert!(DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .is_err());
}

/// CASE 5 — Withdrawn candidate cannot select.
#[test]
fn case5_withdrawn_cannot_select() {
    let mut input = awaiting_selection_input(intake_created_candidate("sel-5"));
    input.intake_withdrawn_or_invalidated = true;
    assert!(DecisionCandidateSelection::derive(&input).is_withdrawn());
    assert!(DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .is_err());
}

/// CASE 6 — Selection does not execute.
#[test]
fn case6_selection_does_not_execute() {
    let input = awaiting_selection_input(native_candidate_with_total("sel-6", 50));
    let (selected, _) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    assert!(!selected.may_execute());
    assert_blocked(
        "execute",
        DecisionCandidateSelection::attempt_execute().map_err(KernelError::from),
    );
    let _ = CommandHandler::decision_engine_attempt_execute();
}

/// CASE 7 — Selection does not call planner.
#[test]
fn case7_selection_does_not_call_planner() {
    let input = awaiting_selection_input(native_candidate_with_total("sel-7", 50));
    let (selected, _) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    assert!(!selected.planner_invoked && !selected.may_invoke_planner());
    assert!(selected.handoff_command.is_none());
    assert!(selected.attempt_invoke_planner().is_err());
}

/// CASE 8 — Selection does not create goals/intents.
#[test]
fn case8_selection_does_not_create_goals_intents() {
    let input = awaiting_selection_input(native_candidate_with_total("sel-8", 50));
    let (selected, _) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    assert!(!selected.creates_goal && !selected.may_create_goal());
    assert!(!selected.creates_intent && !selected.may_create_intent());
    assert!(selected.attempt_create_goal().is_err());
    assert!(selected.attempt_create_intent().is_err());
}

/// CASE 9 — Selection does not call Gateway.
#[test]
fn case9_selection_does_not_call_gateway() {
    let input = awaiting_selection_input(native_candidate_with_total("sel-9", 50));
    let (selected, _) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    assert!(!selected.may_invoke_gateway());
    assert!(selected.attempt_invoke_gateway().is_err());
}

/// CASE 10 — RE state unchanged.
#[test]
fn case10_re_state_unchanged() {
    let input = awaiting_selection_input(intake_created_candidate("sel-10"));
    let (selected, _) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    assert!(!selected.mutates_recommendation_engine);
    assert!(!selected.ownership_transferred);
}

/// CASE 11 — Native and recommendation-derived candidates remain distinguishable.
#[test]
fn case11_origins_distinguishable() {
    let native = DecisionCandidateSelection::try_select(
        &awaiting_selection_input(native_candidate_with_total("sel-11n", 50)),
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap()
    .0;
    let intake = DecisionCandidateSelection::try_select(
        &awaiting_selection_input(intake_created_candidate("sel-11i")),
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap()
    .0;
    assert_eq!(native.origin, DecisionCandidate::ORIGIN_NATIVE);
    assert_eq!(intake.origin, DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE);
    assert_ne!(native.origin, intake.origin);
}

/// CASE 12 — Existing synthesis unchanged.
#[test]
fn case12_synthesis_unchanged() {
    let native = native_candidate_with_total("sel-12", 10);
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
    let input = awaiting_selection_input(native);
    let (selected, unchanged) = DecisionCandidateSelection::try_select(
        &input,
        DecisionCandidateSelection::ACTION_SELECT,
        "progress",
        "t-sel",
    )
    .unwrap();
    let with = base.with_candidate_selections(vec![selected]);
    assert_eq!(with.candidates[0].score.total, score_before);
    assert_eq!(with.candidates[0].outcome, DecisionOutcome::Open);
    assert_eq!(unchanged.outcome, DecisionOutcome::Open);
    assert!(!with.candidate_selections[0].mutates_candidate_outcome);
}
