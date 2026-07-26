//! Sprint 146 — Governance decision evidence & publication readiness contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, GovernanceDecisionEvidence, GovernancePolicy, GovernanceRecord,
    GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceRisk,
    OutcomeAdaptationProposal, PublicationReadiness, PublicationReadinessState,
    RecommendationConfidence, RecommendationEvidence, RecommendationGovernanceRecord,
    RecommendationItem, RecommendationKind, RecommendationLifecycleState, RecommendationOutcome,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected CannotExecute-style error, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed at attempt_execute"),
    }
}

fn sample_item() -> RecommendationItem {
    RecommendationItem {
        id: "recommendation:continue_work:focus-1".into(),
        kind: RecommendationKind::ContinueWork,
        title: "Continue focused work".into(),
        reason: "Continuity shows resumable focus".into(),
        evidence: vec![RecommendationEvidence {
            id: "ev-c".into(),
            source_model: "continuity".into(),
            source_ref: "continuity:focus:1".into(),
            summary: "Resumable work".into(),
        }],
        impact: "Restores momentum".into(),
        confidence: RecommendationConfidence::Medium,
        related_attention_id: Some("attention:focus:1".into()),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::Continuity,
            AttentionSignal::ResumableWork,
            30,
            "continuity.resumable",
        )],
        related_task_id: None,
        related_purpose_label: None,
        related_decision_id: None,
        lifecycle_state: None,
        lifecycle_presented_at: None,
        lifecycle_resolved_at: None,
        lifecycle_resolution_type: None,
        explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn accepted_outcome() -> RecommendationOutcome {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record.provenance = record.provenance.with_experience_trace_match_keys(vec![
        "exact:continuity.resumable".into(),
    ]);
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    record.record_outcome("t4").unwrap()
}

fn approved_proposal() -> OutcomeAdaptationProposal {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal(
        "experience_presentation",
        "Keep DisplayReason primary",
        "Consistent rationale",
    );
    proposal.require_review().unwrap();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    proposal
}

/// CASE 1 — Evidence cannot grant execution authority.
#[test]
fn case1_evidence_cannot_grant_execution_authority() {
    let proposal = approved_proposal();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec!["outcome:sample".into()],
        vec![],
        vec![],
        "Supported by outcome",
        &proposal.provenance,
    )
    .unwrap();
    assert!(!evidence.may_grant_execution_authority());
    assert!(!evidence.may_bypass_permission_gateway());
    assert!(GovernanceDecisionEvidence::attempt_grant_execution_authority().is_err());
    assert!(GovernanceDecisionEvidence::attempt_execute().is_err());
}

/// CASE 2 — Approval requires required evidence.
#[test]
fn case2_approval_requires_required_evidence() {
    let proposal = approved_proposal();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    assert!(GovernanceDecisionEvidence::assemble(
        &risk,
        vec![],
        vec![],
        vec![],
        "no refs",
        &proposal.provenance,
    )
    .is_err());

    let evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec![
            format!("proposal:{}", proposal.id),
            "exact:continuity.resumable".into(),
        ],
        vec![],
        vec!["no activation".into()],
        "Presentation-only change is evidenced",
        &proposal.provenance,
    )
    .unwrap();
    evidence.validate_for_approval().unwrap();

    let policy = GovernancePolicy::from_risk(&risk);
    let bare = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t",
        vec![],
        &proposal.provenance,
    )
    .unwrap();
    let with_ev = bare.clone().with_evidence(&evidence);
    let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
    readiness.mark_risk_reviewed().unwrap();
    readiness.mark_approved(&evidence).unwrap();

    let governance = GovernanceRecord::from_adaptation_chain(&proposal, None, None, None, None)
        .with_policy_and_decisions(&policy, &proposal, &[with_ev.clone()])
        .unwrap();
    assert!(governance
        .clone()
        .with_evidence_and_readiness(&evidence, &readiness, &[bare])
        .is_err());
    governance
        .with_evidence_and_readiness(&evidence, &readiness, &[with_ev])
        .unwrap();
}

/// CASE 3 — Dissent does not erase approval history.
#[test]
fn case3_dissent_does_not_erase_approval_history() {
    let proposal = approved_proposal();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let mut evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec!["outcome:x".into()],
        vec![],
        vec![],
        "OK",
        &proposal.provenance,
    )
    .unwrap();
    let policy = GovernancePolicy::from_risk(&risk);
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t",
        vec![],
        &proposal.provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
    readiness.mark_risk_reviewed().unwrap();
    readiness.mark_approved(&evidence).unwrap();
    readiness.mark_ready_for_publication(&evidence).unwrap();
    let approval_states_before = readiness.history.clone();
    evidence.record_dissent("reviewer_b", "Wording nit", "t2");
    assert_eq!(evidence.dissent_count(), 1);
    assert!(!evidence.may_erase_approval_history());
    assert!(readiness.history_preserves_approvals());
    assert_eq!(readiness.history, approval_states_before);
    assert!(readiness
        .history
        .contains(&PublicationReadinessState::Approved));

    let governance = GovernanceRecord::from_adaptation_chain(&proposal, None, None, None, None)
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap();
    let prior_approval = governance.approval_reference.clone();
    let updated = governance
        .with_evidence_and_readiness(&evidence, &readiness, &[decision])
        .unwrap();
    assert_eq!(updated.approval_reference, prior_approval);
    assert_eq!(updated.dissent_references.len(), 1);
}

/// CASE 4 — Publication readiness cannot activate runtime changes.
#[test]
fn case4_publication_readiness_cannot_activate_runtime() {
    let proposal = approved_proposal();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec!["ref:1".into()],
        vec![],
        vec![],
        "Ready package",
        &proposal.provenance,
    )
    .unwrap();
    let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
    readiness.mark_risk_reviewed().unwrap();
    readiness.mark_approved(&evidence).unwrap();
    readiness.mark_ready_for_publication(&evidence).unwrap();
    assert!(!readiness.may_activate_runtime());
    assert!(readiness.attempt_activate_published().is_err());
    assert!(PublicationReadiness::attempt_execute().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_eq!(
        readiness.state,
        PublicationReadinessState::ReadyForPublication
    );
}
