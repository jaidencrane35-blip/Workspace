//! Sprint 155 — Governance review workflow contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceReviewMode, GovernanceReviewWorkflowContract, GovernanceReviewWorkflowStage,
    GovernanceReviewerAssignmentStatus, RecommendationConfidence, RecommendationEvidence,
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
        explanation: None,
                outcome: None,
                decision_readiness: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn proposal() -> workspace_domain::OutcomeAdaptationProposal {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let mut proposal = outcome.to_adaptation_proposal(
        "experience_presentation",
        "Keep DisplayReason primary",
        "Consistent rationale",
    );
    proposal.require_review().unwrap();
    proposal
}

#[test]
fn case1_ordered_and_parallel_review_modes() {
    let proposal = proposal();
    let mut ordered =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Ordered);
    let a1 = ordered
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r1"), 1, false)
        .unwrap();
    let a2 = ordered
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r2"), 2, false)
        .unwrap();
    ordered.enqueue_pending().unwrap();
    assert!(ordered.start_review(&a2).is_err());
    ordered.start_review(&a1).unwrap();
    ordered.complete_assignment(&a1).unwrap();
    ordered.start_review(&a2).unwrap();

    let mut parallel =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Parallel);
    let p1 = parallel
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r1"), 1, false)
        .unwrap();
    let p2 = parallel
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r2"), 1, false)
        .unwrap();
    parallel.enqueue_pending().unwrap();
    parallel.start_review(&p1).unwrap();
    parallel.start_review(&p2).unwrap();
    assert_eq!(parallel.stage, GovernanceReviewWorkflowStage::InReview);
}

#[test]
fn case2_reassignment_escalation_and_completion() {
    let proposal = proposal();
    let mut wf =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Parallel);
    let a1 = wf
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r1"), 1, false)
        .unwrap();
    wf.enqueue_pending().unwrap();
    let a2 = wf
        .reassign(&a1, AdaptationReviewerIdentity::local_user("r2"))
        .unwrap();
    assert!(wf
        .assignments
        .iter()
        .any(|a| a.status == GovernanceReviewerAssignmentStatus::Reassigned));
    wf.start_review(&a2).unwrap();
    wf.escalate("needs senior reviewer").unwrap();
    assert_eq!(wf.stage, GovernanceReviewWorkflowStage::Escalated);
    wf.start_review(&a2).unwrap();
    wf.complete_assignment(&a2).unwrap();
    wf.complete_workflow().unwrap();
    assert_eq!(wf.stage, GovernanceReviewWorkflowStage::Completed);
}

#[test]
fn case3_workflow_cannot_execute_or_publish() {
    let proposal = proposal();
    let wf = GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Optional);
    assert!(!wf.may_execute());
    assert!(!wf.may_publish());
    assert!(GovernanceReviewWorkflowContract::attempt_execute().is_err());
    assert!(GovernanceReviewWorkflowContract::attempt_publish().is_err());
}
