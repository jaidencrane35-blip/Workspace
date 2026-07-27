//! Decision Engine Intake Candidate — DE-owned acknowledgement only
//! (IntakeCandidate ≠ DecisionCandidate / goal / intent / planner / Gateway).

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, DecisionCandidate, DecisionContext,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeEligibility, DecisionEngineIntakeReceipt,
    DecisionEngineState, DecisionOutcome, DecisionScore, RecommendationConfidence,
    RecommendationDecisionConfirmation, RecommendationDecisionContext,
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
                    || message.contains("authorize"),
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

fn pipeline(
    receipt: DecisionEngineIntakeReceipt,
    acceptance_active: bool,
    lifecycle_superseded: bool,
    receipt_current: bool,
) -> (
    DecisionEngineIntakeReceipt,
    DecisionEngineIntakeAssessment,
    DecisionEngineIntakeEligibility,
) {
    let assessment = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: receipt.clone(),
            acceptance_active,
            lifecycle_superseded,
            receipt_current,
        },
    ])
    .remove(0);
    let eligibility = DecisionEngineIntakeEligibility::derive(&receipt, &assessment);
    (receipt, assessment, eligibility)
}

/// CASE 1 — Eligible intake creates DecisionEngineIntakeCandidate.
#[test]
fn case1_eligible_creates_intake_candidate() {
    let (receipt, acceptance, _) = accepted_chain("task-1");
    let acceptance_before = acceptance.clone();
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let candidate =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t-create")
            .expect("intake candidate");
    assert_eq!(
        candidate.state,
        DecisionEngineIntakeCandidate::STATE_READY
    );
    assert!(candidate
        .intake_candidate_id
        .starts_with(DecisionEngineIntakeCandidate::ID_PREFIX));
    assert!(!candidate.is_decision_candidate);
    assert_eq!(acceptance, acceptance_before);
}

/// CASE 2 — Invalid receipt creates no intake candidate.
#[test]
fn case2_invalid_receipt_creates_none() {
    let (mut receipt, _, _) = accepted_chain("task-2");
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
    receipt.seal_aligned = false;
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    assert!(
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t").is_none()
    );
}

/// CASE 3 — Seal mismatch blocks creation.
#[test]
fn case3_seal_mismatch_blocks_creation() {
    let (mut receipt, _, _) = accepted_chain("task-3");
    receipt.seal_aligned = false;
    receipt.receipt_state = DecisionEngineIntakeReceipt::STATE_SEAL_MISMATCH.into();
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    assert!(!eligibility.is_eligible);
    assert!(
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t").is_none()
    );
}

/// CASE 4 — Superseded recommendation blocks creation.
#[test]
fn case4_superseded_blocks_creation() {
    let (receipt, _, _) = accepted_chain("task-4");
    let (receipt, assessment, eligibility) = pipeline(receipt, true, true, true);
    assert!(assessment.is_superseded);
    assert!(
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t").is_none()
    );
}

/// CASE 5 — Revoked acceptance blocks creation / withdraws existing.
#[test]
fn case5_revoked_acceptance_blocks_and_withdraws() {
    let (receipt, mut acceptance, seal) = accepted_chain("task-5");
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let mut candidate =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t-create")
            .expect("created while accepted");
    acceptance.revoke("t-revoke").unwrap();
    assert!(DecisionEngineIntakeReceipt::try_observe(&acceptance, &seal).is_none());
    let (receipt2, assessment2, eligibility2) = pipeline(receipt.clone(), false, false, false);
    assert!(
        DecisionEngineIntakeCandidate::try_create(&receipt2, &assessment2, &eligibility2, "t")
            .is_none()
    );
    candidate.reevaluate(
        None,
        Some(RecommendationDecisionEngineAcceptance::STATE_REVOKED),
    );
    assert_eq!(
        candidate.state,
        DecisionEngineIntakeCandidate::STATE_WITHDRAWN
    );
    assert!(!candidate.ownership_transferred);
}

/// CASE 6 — Duplicate intake is blocked.
#[test]
fn case6_duplicate_intake_blocked() {
    let (mut a, _, _) = accepted_chain("task-a");
    let (mut b, _, _) = accepted_chain("task-b");
    let digest = a.sealed_intake_package_digest.clone();
    b.sealed_intake_package_digest = digest;
    let assessments = DecisionEngineIntakeAssessment::assess_batch(&[
        DecisionEngineIntakeAssessmentInput {
            receipt: a.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: true,
        },
        DecisionEngineIntakeAssessmentInput {
            receipt: b.clone(),
            acceptance_active: true,
            lifecycle_superseded: false,
            receipt_current: true,
        },
    ]);
    let eligibilities =
        DecisionEngineIntakeEligibility::derive_batch(&[a.clone(), b.clone()], &assessments);
    let created = DecisionEngineIntakeCandidate::create_batch(
        &[a, b],
        &assessments,
        &eligibilities,
        "t-create",
    );
    assert_eq!(created.len(), 1);
    assert_eq!(created[0].state, DecisionEngineIntakeCandidate::STATE_READY);
}

/// CASE 7 — IntakeCandidate is not DecisionCandidate.
#[test]
fn case7_intake_candidate_is_not_decision_candidate() {
    let (receipt, _, _) = accepted_chain("task-7");
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let candidate =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t")
            .unwrap();
    assert!(!candidate.is_decision_candidate);
    assert!(!candidate.creates_decision_candidate);
    assert!(!candidate.intake_candidate_id.starts_with("engine_decision:")
        || candidate
            .intake_candidate_id
            .starts_with(DecisionEngineIntakeCandidate::ID_PREFIX));
    assert_ne!(
        candidate.intake_candidate_id,
        DecisionCandidate::synthetic_id(&candidate.recommendation_reference).as_str()
    );
    assert!(candidate.assert_intake_only().is_ok());
}

/// CASE 8–10 — No planner handoff, goals/intents, or Gateway.
#[test]
fn case8_to_10_no_planner_goals_gateway() {
    let (receipt, _, _) = accepted_chain("task-8");
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let c =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t").unwrap();
    assert!(!c.may_create_planner_handoff());
    assert!(!c.may_create_goal());
    assert!(!c.may_create_intent());
    assert!(!c.may_invoke_planner());
    assert!(!c.may_invoke_gateway());
    assert!(c.handoff_command.is_none());
    assert!(c.attempt_create_planner_handoff().is_err());
    assert!(c.attempt_create_goal().is_err());
    assert!(c.attempt_create_intent().is_err());
    assert!(c.attempt_invoke_planner().is_err());
    assert!(DecisionEngineIntakeCandidate::attempt_execute().is_err());
    assert_blocked(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 11 — Recommendation Engine records remain unchanged.
#[test]
fn case11_recommendation_overlays_unchanged() {
    let (receipt, acceptance, seal) = accepted_chain("task-9");
    let acceptance_before = acceptance.clone();
    let seal_before = seal.clone();
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let _ = DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t")
        .unwrap();
    assert_eq!(acceptance, acceptance_before);
    assert_eq!(seal, seal_before);
    assert_eq!(
        acceptance.current_owner,
        RecommendationDecisionEngineAcceptance::OWNER_RECOMMENDATION
    );
    assert!(!acceptance.ownership_transferred);
}

/// CASE 12 — Existing Decision Engine synthesis behaviour remains unchanged.
#[test]
fn case12_synthesis_unchanged_by_intake_candidates() {
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
    let base = DecisionEngineState::from_candidates("ws-1", context, vec![decision.clone()]);
    let candidates_before = base.candidates.clone();
    let top_before = base.top_candidates.clone();
    let scores_before: Vec<_> = base.candidates.iter().map(|c| c.score.total).collect();

    let (receipt, _, _) = accepted_chain("task-10");
    let (receipt, assessment, eligibility) = pipeline(receipt, true, false, true);
    let intake =
        DecisionEngineIntakeCandidate::try_create(&receipt, &assessment, &eligibility, "t")
            .unwrap();
    let with_intake = base.with_intake_candidates(vec![intake]);

    assert_eq!(with_intake.candidates, candidates_before);
    assert_eq!(with_intake.top_candidates, top_before);
    assert_eq!(
        with_intake
            .candidates
            .iter()
            .map(|c| c.score.total)
            .collect::<Vec<_>>(),
        scores_before
    );
    assert_eq!(with_intake.intake_candidates.len(), 1);
    assert!(!with_intake.intake_candidates[0].is_decision_candidate);
}
