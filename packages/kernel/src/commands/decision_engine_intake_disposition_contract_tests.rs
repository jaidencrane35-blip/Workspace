//! Decision Engine Intake Disposition — lifecycle decision only
//! (dispose ≠ DecisionCandidate / goal / intent / planner / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate, DecisionContext,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeDisposition, DecisionEngineIntakeEligibility,
    DecisionEngineIntakeEvaluation, DecisionEngineIntakeReceipt, DecisionEngineState,
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

fn active_candidate(id_suffix: &str) -> (
    DecisionEngineIntakeCandidate,
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
            .expect("active");
    (candidate, acceptance, seal)
}

fn evaluated_active(id_suffix: &str) -> (
    DecisionEngineIntakeCandidate,
    DecisionEngineIntakeEvaluation,
    RecommendationDecisionEngineAcceptance,
    RecommendationDecisionIntakePackageSeal,
) {
    let (candidate, acceptance, seal) = active_candidate(id_suffix);
    let evaluation = DecisionEngineIntakeEvaluation::try_evaluate(
        &candidate,
        DecisionEngineIntakeEvaluation::STATE_EVALUATED,
        "examined",
        "t-eval",
    )
    .expect("evaluation");
    (candidate, evaluation, acceptance, seal)
}

/// CASE 1 — Evaluated active intake can receive disposition.
#[test]
fn case1_evaluated_active_can_receive_disposition() {
    let (candidate, evaluation, _, _) = evaluated_active("disp-1");
    let disposition = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_RETAINED,
        "keep for future processing",
        "t-disp",
    )
    .expect("dispose");
    assert_eq!(
        disposition.disposition_state,
        DecisionEngineIntakeDisposition::STATE_RETAINED
    );
    assert_eq!(disposition.evaluation_id, evaluation.evaluation_id);
    assert_eq!(
        disposition.authority_effect,
        DecisionEngineIntakeDisposition::AUTHORITY_EFFECT_NONE
    );
    assert!(disposition.assert_disposition_only().is_ok());
}

/// CASE 2 — Unevaluated intake cannot receive disposition.
#[test]
fn case2_unevaluated_cannot_dispose() {
    let (candidate, _, _) = active_candidate("disp-2");
    // Fabricate mismatched/unevaluated evaluation identity.
    let bogus = DecisionEngineIntakeEvaluation {
        evaluation_id: "engine_decision_intake_eval:missing".into(),
        workspace_id: "ws-other".into(),
        intake_candidate_id: "engine_decision_intake:missing".into(),
        evaluated_at: "t".into(),
        evaluation_state: DecisionEngineIntakeEvaluation::STATE_EVALUATED.into(),
        evaluation_reason: "bogus".into(),
        creates_decision_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: String::new(),
        authority_effect: DecisionEngineIntakeEvaluation::AUTHORITY_EFFECT_NONE.into(),
    };
    assert!(DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &bogus,
        DecisionEngineIntakeDisposition::STATE_RETAINED,
        "should fail",
        "t-disp",
    )
    .is_err());
}

/// CASE 3 — Withdrawn intake cannot receive disposition.
#[test]
fn case3_withdrawn_cannot_dispose() {
    let (mut candidate, evaluation, _, _) = evaluated_active("disp-3");
    candidate.withdraw("t-w").unwrap();
    assert!(DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DISMISSED,
        "should fail",
        "t-disp",
    )
    .is_err());
}

/// CASE 4 — Invalidated intake cannot receive disposition.
#[test]
fn case4_invalidated_cannot_dispose() {
    let (mut candidate, evaluation, _, _) = evaluated_active("disp-4");
    candidate
        .lifecycle
        .invalidate(
            workspace_domain::DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH,
            "t-inv",
        )
        .unwrap();
    assert!(DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DEFERRED,
        "should fail",
        "t-disp",
    )
    .is_err());
}

/// CASE 5 — Retained does not create DecisionCandidate.
#[test]
fn case5_retained_does_not_create_decision_candidate() {
    let (candidate, evaluation, _, _) = evaluated_active("disp-5");
    let d = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_RETAINED,
        "retain",
        "t-disp",
    )
    .unwrap();
    assert!(!d.creates_decision_candidate);
    assert!(!d.may_create_decision_candidate());
    assert!(d.attempt_create_decision_candidate().is_err());
    assert!(d.handoff_command.is_none());
}

/// CASE 6 — Dismissed does not alter RE.
#[test]
fn case6_dismissed_does_not_alter_re() {
    let (candidate, evaluation, acceptance, seal) = evaluated_active("disp-6");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    let _ = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DISMISSED,
        "decline further consideration",
        "t-disp",
    )
    .unwrap();
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
    assert!(!acceptance.ownership_transferred);
}

/// CASE 7–9 — Deferred creates no planning work; no goal/intent; no planner/Gateway.
#[test]
fn case7_to_9_deferred_no_planning_goal_intent_gateway() {
    let (candidate, evaluation, _, _) = evaluated_active("disp-7");
    let d = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_DEFERRED,
        "postpone consideration",
        "t-disp",
    )
    .unwrap();
    assert!(!d.creates_goal);
    assert!(!d.creates_intent);
    assert!(!d.planner_invoked);
    assert!(!d.may_create_goal());
    assert!(!d.may_create_intent());
    assert!(!d.may_invoke_planner());
    assert!(!d.may_invoke_gateway());
    assert!(d.handoff_command.is_none());
    assert!(d.attempt_create_goal().is_err());
    assert!(d.attempt_create_intent().is_err());
    assert!(d.attempt_invoke_planner().is_err());
    assert!(DecisionEngineIntakeDisposition::attempt_execute().is_err());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 10 — Existing Decision Engine synthesis unchanged.
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
    let (candidate, evaluation, _, _) = evaluated_active("disp-8");
    let disposition = DecisionEngineIntakeDisposition::try_dispose(
        &candidate,
        &evaluation,
        DecisionEngineIntakeDisposition::STATE_RETAINED,
        "retain",
        "t-disp",
    )
    .unwrap();
    let with = base.with_intake_dispositions(vec![disposition]);
    assert_eq!(
        with.candidates
            .iter()
            .map(|c| c.score.total)
            .collect::<Vec<_>>(),
        scores_before
    );
    assert_eq!(with.candidates.len(), 1);
    assert_eq!(with.intake_dispositions.len(), 1);
}
