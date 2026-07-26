//! Sprint 144 — Governance policy & human review contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, ChangeEvaluation, ControlledChangeSurface, GovernancePolicy,
    GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    OutcomeAdaptationProposal, PublishRequest, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState, RecommendationOutcome,
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

/// CASE 1 — Policy cannot execute actions.
#[test]
fn case1_policy_cannot_execute_actions() {
    let policy = GovernancePolicy::for_outcome_adaptation();
    assert!(!policy.may_execute());
    assert!(!policy.may_grant_execution_authority());
    assert!(GovernancePolicy::attempt_execute().is_err());
    assert!(GovernancePolicy::attempt_grant_execution_authority().is_err());
    assert!(GovernanceReviewDecision::attempt_execute().is_err());
    assert_eq!(policy.authority_effect, "none");
}

/// CASE 2 — Reviewers cannot bypass provenance.
#[test]
fn case2_reviewers_cannot_bypass_provenance() {
    let proposal = approved_proposal();
    let provenance = proposal.provenance.clone();
    let policy = GovernancePolicy::for_outcome_adaptation();
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Accept presentation change",
        "t-dec",
        vec![],
        &provenance,
    )
    .unwrap();
    assert!(decision.retains_provenance(&provenance));
    assert!(!decision.may_bypass_provenance());
    assert!(!decision.may_rewrite_provenance());
    assert!(GovernanceReviewDecision::attempt_bypass_provenance().is_err());

    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let evaluation =
        ChangeEvaluation::from_behaviour_version(&version, vec![], vec![], None);
    let mut foreign = provenance.clone();
    foreign.recommendation_id = "tampered".into();
    let mut bad = decision.clone();
    bad.provenance_snapshot = foreign;
    let err = GovernanceRecord::from_adaptation_chain(
        &proposal,
        Some(&surface),
        Some(&version),
        Some(&evaluation),
        None,
    )
    .with_policy_and_decisions(&policy, &proposal, &[bad]);
    assert!(err.is_err());
}

/// CASE 3 — Approval requirements are enforced.
#[test]
fn case3_approval_requirements_are_enforced() {
    let proposal = approved_proposal();
    let provenance = proposal.provenance.clone();
    let policy = GovernancePolicy::for_outcome_adaptation();
    assert!(policy
        .enforce_approval_requirements(&proposal, &[])
        .is_err());

    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Looks good",
        "t-dec",
        vec!["no runtime activation".into()],
        &provenance,
    )
    .unwrap();
    policy
        .enforce_approval_requirements(&proposal, &[decision.clone()])
        .unwrap();

    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let evaluation =
        ChangeEvaluation::from_behaviour_version(&version, vec![], vec![], None);
    let governance = GovernanceRecord::from_adaptation_chain(
        &proposal,
        Some(&surface),
        Some(&version),
        Some(&evaluation),
        None,
    )
    .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
    .unwrap();
    assert!(governance.policy_reference.is_some());
    assert_eq!(governance.review_decision_references, vec![decision.id.clone()]);

    let publish = PublishRequest::from_evaluated_version_under_policy(
        &version,
        &evaluation,
        &governance,
        &policy,
        &proposal,
        &[decision],
        AdaptationReviewerIdentity::local_user("local_user"),
    )
    .unwrap();
    assert_eq!(publish.governance_record_id, governance.id);

    // Self-approval under policy forbidden.
    let self_decision = GovernanceReviewDecision {
        id: "bad".into(),
        policy_id: policy.id.clone(),
        reviewer: AdaptationReviewerIdentity {
            actor_id: OutcomeAdaptationProposal::PROPOSER_ACTOR_ID.into(),
            actor_type: "local_user".into(),
        },
        decision: GovernanceReviewDecisionKind::Approve,
        rationale: "I propose and approve".into(),
        timestamp: "t".into(),
        conditions: vec![],
        evidence_reference: None,
        provenance_snapshot: provenance,
        authority_effect: "none".into(),
    };
    assert!(policy
        .enforce_approval_requirements(&proposal, &[self_decision])
        .is_err());
}

/// CASE 4 — Permission Gateway remains separate.
#[test]
fn case4_permission_gateway_remains_separate() {
    let proposal = approved_proposal();
    let policy = GovernancePolicy::for_outcome_adaptation();
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve under policy",
        "t-dec",
        vec![],
        &proposal.provenance,
    )
    .unwrap();
    assert!(!policy.may_bypass_permission_gateway());
    assert!(!decision.may_bypass_permission_gateway());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}
