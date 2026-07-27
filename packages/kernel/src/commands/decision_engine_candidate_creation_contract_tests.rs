//! Decision Engine Candidate Creation — may create DecisionCandidate
//! without scoring / planner / Gateway / goals / intents.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate, DecisionContext,
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

fn eligible_input(
    id_suffix: &str,
    acceptance_active: bool,
    seal_aligned: bool,
) -> DecisionEngineCandidateCreationInput {
    let (candidate, evaluation, disposition, _, _) = retained_evaluated(id_suffix);
    let package_seal_digest = if seal_aligned {
        candidate.package_seal_digest.clone()
    } else {
        "mismatch-digest".into()
    };
    let boundary =
        DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
            candidate: candidate.clone(),
            evaluation: Some(evaluation),
            disposition: Some(disposition),
            acceptance_active,
            seal_aligned,
            previously_promoted: false,
        });
    let request = DecisionEngineCandidateCreationRequest::derive(&boundary);
    DecisionEngineCandidateCreationInput {
        creation_request: request,
        intake_candidate: candidate,
        promotion_boundary: boundary,
        package_seal_digest,
        existing_creation: None,
    }
}

/// CASE 1 — Valid creation request creates DecisionCandidate.
#[test]
fn case1_valid_request_creates_decision_candidate() {
    let input = eligible_input("cc-1", true, true);
    assert!(input.creation_request.is_requested());
    assert!(input.promotion_boundary.is_promotion_allowed());
    let projected = DecisionEngineCandidateCreation::derive(&input);
    assert!(projected.is_eligible_for_creation());

    let (creation, candidate) =
        DecisionEngineCandidateCreation::try_create(&input, "t-created").expect("create");
    assert!(creation.is_created());
    assert!(creation.creates_decision_candidate);
    assert!(candidate.id.as_str().starts_with("engine_decision:intake:"));
    assert!(!candidate.id.as_str().starts_with("engine_decision_intake:"));
    assert_eq!(candidate.score.total, 0);
    assert!(candidate.handoff_command.is_empty());
}

/// CASE 2 — Blocked request cannot create.
#[test]
fn case2_blocked_request_cannot_create() {
    let (candidate, evaluation, _, _, _) = retained_evaluated("cc-2");
    let dismissed = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DISMISSED,
        "decline",
        "t-disp",
    )
    .unwrap();
    let boundary =
        DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
            candidate: candidate.clone(),
            evaluation: Some(evaluation),
            disposition: Some(dismissed),
            acceptance_active: true,
            seal_aligned: true,
            previously_promoted: false,
        });
    let request = DecisionEngineCandidateCreationRequest::derive(&boundary);
    let input = DecisionEngineCandidateCreationInput {
        creation_request: request,
        package_seal_digest: candidate.package_seal_digest.clone(),
        intake_candidate: candidate,
        promotion_boundary: boundary,
        existing_creation: None,
    };
    assert!(DecisionEngineCandidateCreation::derive(&input).is_blocked());
    assert!(DecisionEngineCandidateCreation::try_create(&input, "t").is_err());
}

/// CASE 3 — Invalidated intake cannot create.
#[test]
fn case3_invalidated_intake_cannot_create() {
    let mut input = eligible_input("cc-3", true, true);
    input
        .intake_candidate
        .lifecycle
        .invalidate(
            workspace_domain::DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH,
            "t-inv",
        )
        .unwrap();
    // Re-derive boundary/request against invalidated intake.
    let boundary =
        DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
            candidate: input.intake_candidate.clone(),
            evaluation: None,
            disposition: None,
            acceptance_active: true,
            seal_aligned: true,
            previously_promoted: false,
        });
    input.promotion_boundary = boundary.clone();
    input.creation_request = DecisionEngineCandidateCreationRequest::derive(&boundary);
    assert!(DecisionEngineCandidateCreation::derive(&input).is_blocked());
    assert!(DecisionEngineCandidateCreation::try_create(&input, "t").is_err());
}

/// CASE 4 — Seal mismatch blocks creation.
#[test]
fn case4_seal_mismatch_blocks_creation() {
    let input = eligible_input("cc-4", true, false);
    assert!(DecisionEngineCandidateCreation::derive(&input).is_blocked());
    assert!(DecisionEngineCandidateCreation::try_create(&input, "t").is_err());
}

/// CASE 5 — Acceptance revoke blocks creation.
#[test]
fn case5_acceptance_revoke_blocks_creation() {
    let input = eligible_input("cc-5", false, true);
    assert!(DecisionEngineCandidateCreation::derive(&input).is_blocked());
    assert!(DecisionEngineCandidateCreation::try_create(&input, "t").is_err());
}

/// CASE 6 — Created candidate has provenance back to intake.
#[test]
fn case6_created_candidate_has_provenance() {
    let input = eligible_input("cc-6", true, true);
    let (creation, candidate) =
        DecisionEngineCandidateCreation::try_create(&input, "t-created").unwrap();
    assert_eq!(
        candidate.intake_candidate_id.as_deref(),
        Some(creation.intake_candidate_id.as_str())
    );
    assert_eq!(
        candidate.creation_request_id.as_deref(),
        Some(creation.creation_request_id.as_str())
    );
    assert_eq!(
        candidate.package_seal_digest.as_deref(),
        Some(creation.package_seal_digest.as_str())
    );
    assert_eq!(
        candidate.recommendation_id.as_deref(),
        Some(creation.recommendation_reference.as_str())
    );
}

/// CASE 7 — Creation does not add scoring.
#[test]
fn case7_creation_does_not_add_scoring() {
    let input = eligible_input("cc-7", true, true);
    let (creation, candidate) =
        DecisionEngineCandidateCreation::try_create(&input, "t-created").unwrap();
    assert!(!creation.creates_decision_score);
    assert!(!creation.may_create_decision_score());
    assert!(creation.attempt_create_decision_score().is_err());
    assert_eq!(candidate.score, DecisionCandidate::unscored());
    assert!(candidate.score.factors.is_empty());
}

/// CASE 8 — Creation does not invoke planner/Gateway.
#[test]
fn case8_creation_does_not_invoke_planner_gateway() {
    let input = eligible_input("cc-8", true, true);
    let (creation, candidate) =
        DecisionEngineCandidateCreation::try_create(&input, "t-created").unwrap();
    assert!(!creation.planner_invoked);
    assert!(!creation.may_invoke_planner());
    assert!(!creation.may_invoke_gateway());
    assert!(creation.attempt_invoke_planner().is_err());
    assert!(DecisionEngineCandidateCreation::attempt_execute().is_err());
    assert!(candidate.handoff_command.is_empty());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 9 — RE overlays remain unchanged.
#[test]
fn case9_recommendation_overlays_unchanged() {
    let (candidate, evaluation, disposition, acceptance, seal) = retained_evaluated("cc-9");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    let boundary =
        DecisionEngineIntakePromotionBoundary::derive(&DecisionEngineIntakePromotionBoundaryInput {
            candidate: candidate.clone(),
            evaluation: Some(evaluation),
            disposition: Some(disposition),
            acceptance_active: true,
            seal_aligned: true,
            previously_promoted: false,
        });
    let request = DecisionEngineCandidateCreationRequest::derive(&boundary);
    let input = DecisionEngineCandidateCreationInput {
        creation_request: request,
        package_seal_digest: candidate.package_seal_digest.clone(),
        intake_candidate: candidate,
        promotion_boundary: boundary,
        existing_creation: None,
    };
    let _ = DecisionEngineCandidateCreation::try_create(&input, "t-created").unwrap();
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
}

/// CASE 10 — Existing DE synthesis remains unchanged.
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
        intake_candidate_id: None,
        creation_request_id: None,
        package_seal_digest: None,
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
    let input = eligible_input("cc-10", true, true);
    let (creation, candidate) =
        DecisionEngineCandidateCreation::try_create(&input, "t-created").unwrap();
    let mut with_candidates = base.candidates.clone();
    with_candidates.push(candidate);
    let with = DecisionEngineState::from_candidates(
        "ws-1",
        base.context.clone(),
        with_candidates,
    )
    .with_candidate_creations(vec![creation]);
    assert_eq!(with.candidates[0].score.total, scores_before[0]);
    assert_eq!(with.candidates.len(), 2);
    assert_eq!(with.candidates[1].score.total, 0);
    assert!(with.candidate_creations[0].is_created());
}
