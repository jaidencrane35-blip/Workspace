//! Sprint 156 — Governance conflict resolution contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceConflictKind, GovernanceConflictResolutionContract, GovernanceConflictState,
    GovernanceConsensusRule, GovernancePolicy, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceRisk, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState,
};

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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn conflicting_pair() -> (
    workspace_domain::OutcomeAdaptationProposal,
    GovernanceReviewDecision,
    GovernanceReviewDecision,
) {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let provenance = outcome.provenance.clone();
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
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let policy = GovernancePolicy::from_risk(&risk);
    let approve = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("r1"),
        GovernanceReviewDecisionKind::Approve,
        "Approve with testing",
        "t1",
        vec!["requires testing".into()],
        &provenance,
    )
    .unwrap();
    let reject = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("r2"),
        GovernanceReviewDecisionKind::Reject,
        "Reject change",
        "t2",
        vec![],
        &provenance,
    )
    .unwrap();
    (proposal, approve, reject)
}

#[test]
fn case1_conflicting_decisions_and_conditions_detected() {
    let (proposal, approve, reject) = conflicting_pair();
    let approve_b = {
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        let policy = GovernancePolicy::from_risk(&risk);
        GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("r3"),
            GovernanceReviewDecisionKind::Approve,
            "Approve with docs",
            "t3",
            vec!["requires documentation".into()],
            &proposal.provenance,
        )
        .unwrap()
    };
    let conflict = GovernanceConflictResolutionContract::from_decisions(
        &proposal,
        &[approve, reject, approve_b],
        GovernanceConsensusRule::Majority,
    );
    assert!(conflict
        .conflicts
        .iter()
        .any(|c| c.kind == GovernanceConflictKind::ConflictingDecisions));
    assert!(conflict
        .conflicts
        .iter()
        .any(|c| c.kind == GovernanceConflictKind::ConflictingConditions));
    assert_eq!(conflict.state, GovernanceConflictState::Detected);
}

#[test]
fn case2_dissent_arbitration_unresolved() {
    let (proposal, approve, reject) = conflicting_pair();
    let mut conflict = GovernanceConflictResolutionContract::from_decisions(
        &proposal,
        &[approve, reject],
        GovernanceConsensusRule::QuorumWithArbiter,
    );
    conflict.record_dissent("dissent:1", "Prefer alternate wording");
    conflict.require_arbitration("arbiter:local").unwrap();
    assert_eq!(conflict.state, GovernanceConflictState::UnderArbitration);
    conflict.mark_unresolved().unwrap();
    assert_eq!(conflict.state, GovernanceConflictState::Unresolved);
    assert!(conflict.is_blocking_publication());
}

#[test]
fn case3_conflict_cannot_publish_execute_or_rewrite_history() {
    let (proposal, approve, reject) = conflicting_pair();
    let conflict = GovernanceConflictResolutionContract::from_decisions(
        &proposal,
        &[approve, reject],
        GovernanceConsensusRule::Unanimous,
    );
    assert!(!conflict.may_publish());
    assert!(!conflict.may_execute());
    assert!(!conflict.may_modify_history());
    assert!(conflict.history_immutable);
    assert!(GovernanceConflictResolutionContract::attempt_publish().is_err());
    assert!(GovernanceConflictResolutionContract::attempt_execute().is_err());
    assert!(conflict.attempt_modify_history().is_err());
}
