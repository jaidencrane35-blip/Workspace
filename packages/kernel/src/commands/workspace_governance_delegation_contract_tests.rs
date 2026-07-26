//! Sprint 161 — Governance delegation contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceDelegationContract, GovernanceDelegationStatus, RecommendationConfidence,
    RecommendationEvidence, RecommendationGovernanceRecord, RecommendationItem,
    RecommendationKind, RecommendationLifecycleState,
};

fn proposal() -> workspace_domain::OutcomeAdaptationProposal {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(
        &RecommendationItem {
            id: "recommendation:continue_work:focus-1".into(),
            kind: RecommendationKind::ContinueWork,
            title: "Continue".into(),
            reason: "Resumable".into(),
            evidence: vec![RecommendationEvidence {
                id: "ev".into(),
                source_model: "continuity".into(),
                source_ref: "c:1".into(),
                summary: "s".into(),
            }],
            impact: "i".into(),
            confidence: RecommendationConfidence::Medium,
            related_attention_id: None,
            attention_reasons: vec![AttentionReason::new(
                AttentionSourceType::Continuity,
                AttentionSignal::ResumableWork,
                30,
                "k",
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
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
        },
        "t0",
    );
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", None).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let mut p = outcome.to_adaptation_proposal("a", "b", "c");
    p.require_review().unwrap();
    p
}

#[test]
fn case1_temporary_delegation_and_revoke() {
    let proposal = proposal();
    let mut d = GovernanceDelegationContract::new(&proposal);
    let id = d
        .delegate(
            AdaptationReviewerIdentity::local_user("r1"),
            AdaptationReviewerIdentity::local_user("r2"),
            None,
            true,
            Some("t+2d".into()),
            "t0",
        )
        .unwrap();
    assert_eq!(d.active_delegatees().len(), 1);
    d.revoke(&id, "r1", "t1").unwrap();
    assert_eq!(
        d.delegations[0].status,
        GovernanceDelegationStatus::Revoked
    );
    assert!(d.audit_events.iter().any(|e| e.action == "revoked"));
    assert_eq!(
        d.delegations[0].scope,
        GovernanceDelegationContract::SCOPE_REVIEW_ONLY
    );
}

#[test]
fn case2_chain_limit_and_no_execution_transfer() {
    let proposal = proposal();
    let mut d = GovernanceDelegationContract::new(&proposal);
    d.max_chain_depth = 1;
    let id = d
        .delegate(
            AdaptationReviewerIdentity::local_user("a"),
            AdaptationReviewerIdentity::local_user("b"),
            None,
            false,
            None,
            "t0",
        )
        .unwrap();
    assert!(d
        .delegate(
            AdaptationReviewerIdentity::local_user("b"),
            AdaptationReviewerIdentity::local_user("c"),
            Some(id),
            false,
            None,
            "t1",
        )
        .is_err());
    assert!(!d.may_transfer_execution_authority());
    assert!(!d.may_execute());
    assert!(GovernanceDelegationContract::attempt_transfer_execution().is_err());
    assert!(GovernanceDelegationContract::attempt_execute().is_err());
}
