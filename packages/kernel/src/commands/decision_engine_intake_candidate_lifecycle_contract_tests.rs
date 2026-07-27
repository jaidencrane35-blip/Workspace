//! Decision Engine Intake Candidate Lifecycle — DE-owned only
//! (lifecycle ≠ DecisionCandidate / planner / execution / RE mutation).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate, DecisionContext,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeCandidateLifecycle,
    DecisionEngineIntakeEligibility, DecisionEngineIntakeReceipt, DecisionEngineState,
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

fn create_active(id_suffix: &str) -> (
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
            .expect("active intake candidate");
    (candidate, acceptance, seal)
}

/// Creation starts active.
#[test]
fn creation_starts_active() {
    let (candidate, _, _) = create_active("life-1");
    assert!(candidate.lifecycle.is_active());
    assert_eq!(
        candidate.lifecycle.lifecycle_state,
        DecisionEngineIntakeCandidateLifecycle::STATE_ACTIVE
    );
    assert!(candidate.lifecycle.reason.is_none());
}

/// Withdrawal only affects DE-owned state; RE overlays unchanged.
#[test]
fn withdrawal_only_affects_de_state() {
    let (mut candidate, acceptance, seal) = create_active("life-2");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    candidate.withdraw("t-withdraw").unwrap();
    assert!(candidate.lifecycle.is_withdrawn());
    assert_eq!(
        candidate.lifecycle.reason.as_deref(),
        Some(DecisionEngineIntakeCandidateLifecycle::REASON_DE_WITHDRAWAL)
    );
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
    assert!(!acceptance.ownership_transferred);
}

/// Seal mismatch invalidates.
#[test]
fn seal_mismatch_invalidates() {
    let (mut candidate, _, _) = create_active("life-3");
    let (mut receipt, _, _) = accepted_chain("life-3");
    receipt.seal_aligned = false;
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
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
    assert!(!eligibility.seal_aligned);
    candidate.apply_source_reevaluation(Some(&eligibility), Some("accepted"), "t-seal");
    assert!(candidate.lifecycle.is_invalidated());
    assert_eq!(
        candidate.lifecycle.reason.as_deref(),
        Some(DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH)
    );
}

/// Revoked acceptance invalidates.
#[test]
fn revoked_acceptance_invalidates() {
    let (mut candidate, mut acceptance, seal) = create_active("life-4");
    acceptance.revoke("t-revoke").unwrap();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
    candidate.apply_source_reevaluation(
        None,
        Some(RecommendationDecisionEngineAcceptance::STATE_REVOKED),
        "t-rev",
    );
    assert!(candidate.lifecycle.is_invalidated());
    assert_eq!(
        candidate.lifecycle.reason.as_deref(),
        Some(DecisionEngineIntakeCandidateLifecycle::REASON_ACCEPTANCE_REVOKED)
    );
}

/// Superseded recommendation invalidates.
#[test]
fn superseded_recommendation_invalidates() {
    let (mut candidate, _, _) = create_active("life-5");
    let (receipt, _, _) = accepted_chain("life-5");
    let assessment = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active: true,
            lifecycle_superseded: true,
            receipt_current: true,
        },
    ])
    .remove(0);
    let eligibility = DecisionEngineIntakeEligibility::derive(&receipt, &assessment);
    assert!(eligibility.is_superseded);
    candidate.apply_source_reevaluation(Some(&eligibility), Some("accepted"), "t-sup");
    assert!(candidate.lifecycle.is_invalidated());
    assert_eq!(
        candidate.lifecycle.reason.as_deref(),
        Some(DecisionEngineIntakeCandidateLifecycle::REASON_SUPERSEDED)
    );
}

/// Invalidated cannot reactivate.
#[test]
fn invalidated_cannot_reactivate() {
    let (mut candidate, _, _) = create_active("life-6");
    candidate
        .lifecycle
        .invalidate(
            DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH,
            "t-inv",
        )
        .unwrap();
    assert!(candidate.lifecycle.attempt_reactivate().is_err());
    assert!(!candidate.lifecycle.allows_transition(
        DecisionEngineIntakeCandidateLifecycle::STATE_ACTIVE
    ));
    // Eligible source again must not revive invalidated.
    let (receipt, _, _) = accepted_chain("life-6");
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
    candidate.apply_source_reevaluation(Some(&eligibility), Some("accepted"), "t-again");
    assert!(candidate.lifecycle.is_invalidated());
}

/// No planner / Gateway / intent / goal path.
#[test]
fn no_planner_gateway_intent_goal_path() {
    let (candidate, _, _) = create_active("life-7");
    let life = &candidate.lifecycle;
    assert!(!life.may_create_decision_candidate());
    assert!(!life.may_create_goal());
    assert!(!life.may_create_intent());
    assert!(!life.may_invoke_planner());
    assert!(!life.may_invoke_gateway());
    assert!(DecisionEngineIntakeCandidateLifecycle::attempt_execute().is_err());
    assert!(!candidate.may_create_goal());
    assert!(!candidate.may_create_intent());
    assert!(!candidate.may_invoke_planner());
    assert!(!candidate.may_invoke_gateway());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// Existing Decision Engine synthesis remains unchanged.
#[test]
fn synthesis_unchanged_by_lifecycle() {
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
    let (mut intake, _, _) = create_active("life-8");
    intake.withdraw("t-w").unwrap();
    let with = base.with_intake_candidates(vec![intake]);
    assert_eq!(
        with.candidates
            .iter()
            .map(|c| c.score.total)
            .collect::<Vec<_>>(),
        scores_before
    );
    assert_eq!(with.candidates.len(), 1);
    assert!(with.intake_candidates[0].lifecycle.is_withdrawn());
}
