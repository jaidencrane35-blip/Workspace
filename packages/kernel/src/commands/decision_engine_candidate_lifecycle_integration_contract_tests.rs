//! DecisionCandidate Lifecycle Integration — origin-aware DE lifecycle
//! without scoring / ranking / planner / Gateway.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate,
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

/// CASE 1 — Native candidate lifecycle unchanged.
#[test]
fn case1_native_lifecycle_unchanged() {
    let native = native_candidate();
    let integration = DecisionCandidateLifecycleIntegration::derive(&native);
    assert!(integration.is_integrated());
    assert_eq!(integration.origin, DecisionCandidate::ORIGIN_NATIVE);
    let (next, updated) = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &native,
        DecisionOutcome::Dismissed,
    )
    .expect("dismiss");
    assert!(next.is_integrated());
    assert_eq!(updated.outcome, DecisionOutcome::Dismissed);
    assert_eq!(updated.origin, DecisionCandidate::ORIGIN_NATIVE);
    assert!(updated.intake_candidate_id.is_none());
    assert_eq!(updated.score.total, native.score.total);
}

/// CASE 2 — Recommendation-derived candidate enters lifecycle correctly.
#[test]
fn case2_recommendation_derived_enters_lifecycle() {
    let candidate = intake_created_candidate("lci-2");
    let integration = DecisionCandidateLifecycleIntegration::derive(&candidate);
    assert!(integration.is_integrated());
    assert_eq!(
        integration.origin,
        DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
    );
    let (_, updated) = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &candidate,
        DecisionOutcome::Postponed,
    )
    .expect("postpone");
    assert_eq!(updated.outcome, DecisionOutcome::Postponed);
    assert!(updated.is_recommendation_intake());
}

/// CASE 3 — Provenance retained after lifecycle changes.
#[test]
fn case3_provenance_retained_after_lifecycle_change() {
    let candidate = intake_created_candidate("lci-3");
    let before_intake = candidate.intake_candidate_id.clone();
    let before_request = candidate.creation_request_id.clone();
    let before_seal = candidate.package_seal_digest.clone();
    let before_rec = candidate.recommendation_id.clone();
    let (_, updated) = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &candidate,
        DecisionOutcome::Selected,
    )
    .expect("select");
    assert_eq!(updated.intake_candidate_id, before_intake);
    assert_eq!(updated.creation_request_id, before_request);
    assert_eq!(updated.package_seal_digest, before_seal);
    assert_eq!(updated.recommendation_id, before_rec);
    assert!(
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&candidate, &updated)
            .is_ok()
    );
}

/// CASE 4 — Provenance cannot be removed.
#[test]
fn case4_provenance_cannot_be_removed() {
    let candidate = intake_created_candidate("lci-4");
    assert!(
        DecisionCandidateLifecycleIntegration::attempt_strip_provenance(&candidate).is_err()
    );
    let mut stripped = candidate.clone();
    stripped.intake_candidate_id = None;
    stripped.creation_request_id = None;
    stripped.package_seal_digest = None;
    assert!(
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&candidate, &stripped)
            .is_err()
    );
}

/// CASE 5 — RE overlays unchanged.
#[test]
fn case5_recommendation_overlays_unchanged() {
    let mut item = accepted_ready_item("lci-5");
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
    let before = confirmation.clone();
    let candidate = intake_created_candidate("lci-5b");
    let _ = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &candidate,
        DecisionOutcome::Dismissed,
    )
    .unwrap();
    // confirmation constructed independently remains recommendation-owned shape
    assert_eq!(
        before.confirmation_state,
        confirmation.confirmation_state
    );
    assert_eq!(candidate.origin, DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE);
}

/// CASE 6 — No scoring added.
#[test]
fn case6_no_scoring_added() {
    let candidate = intake_created_candidate("lci-6");
    let (integration, updated) = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &candidate,
        DecisionOutcome::Dismissed,
    )
    .unwrap();
    assert!(!integration.scoring_applied);
    assert!(!integration.creates_decision_score);
    assert!(!integration.may_create_decision_score());
    assert!(integration.attempt_create_decision_score().is_err());
    assert_eq!(updated.score, DecisionCandidate::unscored());
}

/// CASE 7 — No planner/Gateway path exists.
#[test]
fn case7_no_planner_gateway_path() {
    let candidate = intake_created_candidate("lci-7");
    let (integration, updated) = DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &candidate,
        DecisionOutcome::Selected,
    )
    .unwrap();
    assert!(!integration.planner_invoked);
    assert!(!integration.may_invoke_planner());
    assert!(!integration.may_invoke_gateway());
    assert!(integration.attempt_invoke_planner().is_err());
    assert!(DecisionCandidateLifecycleIntegration::attempt_execute().is_err());
    assert!(updated.handoff_command.is_empty());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 8 — Existing DE synthesis unchanged.
#[test]
fn case8_synthesis_unchanged() {
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
    let intake = intake_created_candidate("lci-8");
    let integrations = DecisionCandidateLifecycleIntegration::derive_batch(&[
        native.clone(),
        intake.clone(),
    ]);
    let with = DecisionEngineState::from_candidates(
        "ws-1",
        base.context.clone(),
        vec![native, intake],
    )
    .with_lifecycle_integrations(integrations);
    let native_in_state = with
        .candidates
        .iter()
        .find(|c| c.origin == DecisionCandidate::ORIGIN_NATIVE)
        .unwrap();
    assert_eq!(native_in_state.score.total, score_before);
    assert_eq!(with.lifecycle_integrations.len(), 2);
}

/// CASE 9 — Candidate origins remain distinguishable.
#[test]
fn case9_origins_distinguishable() {
    let native = native_candidate();
    let intake = intake_created_candidate("lci-9");
    let n = DecisionCandidateLifecycleIntegration::derive(&native);
    let i = DecisionCandidateLifecycleIntegration::derive(&intake);
    assert_eq!(n.origin, DecisionCandidate::ORIGIN_NATIVE);
    assert_eq!(i.origin, DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE);
    assert_ne!(n.origin, i.origin);
    assert!(native.is_native_origin());
    assert!(intake.is_recommendation_intake());
}

/// CASE 10 — Invalid provenance blocks integration.
#[test]
fn case10_invalid_provenance_blocks_integration() {
    let mut broken = intake_created_candidate("lci-10");
    broken.package_seal_digest = None;
    let integration = DecisionCandidateLifecycleIntegration::derive(&broken);
    assert_eq!(
        integration.integration_state,
        DecisionCandidateLifecycleIntegration::STATE_PROVENANCE_INVALID
    );
    assert!(!integration.is_integrated());
    assert!(DecisionCandidateLifecycleIntegration::try_apply_outcome(
        &broken,
        DecisionOutcome::Dismissed,
    )
    .is_err());
}
