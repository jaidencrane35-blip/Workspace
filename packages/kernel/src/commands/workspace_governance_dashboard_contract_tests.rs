//! Sprint 159 — Governance readiness dashboard projection contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceArchiveContract, GovernanceComplianceContract, GovernanceConditionContract,
    GovernanceConflictResolutionContract, GovernanceConsensusRule, GovernanceDashboardComplianceStatus,
    GovernanceDecisionEvidence, GovernanceDecisionPackage, GovernancePolicy,
    GovernanceReadinessDashboardProjection, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    GovernanceReviewMode, GovernanceReviewWorkflowContract, GovernanceRisk,
    GovernanceReviewerAssignmentStatus, PublicationReadiness, RecommendationConfidence,
    RecommendationEvidence, RecommendationGovernanceRecord, RecommendationItem,
    RecommendationKind, RecommendationLifecycleState,
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
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[test]
fn case1_dashboard_projects_review_obligations_conflicts_compliance() {
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
    let evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec![format!("outcome:{}", outcome.id)],
        vec![],
        vec!["requires testing".into()],
        "Evidenced",
        &provenance,
    )
    .unwrap();
    let policy = GovernancePolicy::from_risk(&risk);
    let approve = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("r1"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t1",
        vec!["requires testing".into()],
        &provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let reject = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("r2"),
        GovernanceReviewDecisionKind::Reject,
        "Reject",
        "t2",
        vec![],
        &provenance,
    )
    .unwrap();
    let conditions = GovernanceConditionContract::from_decision(&approve).unwrap();
    let conflict = GovernanceConflictResolutionContract::from_decisions(
        &proposal,
        &[approve.clone(), reject],
        GovernanceConsensusRule::Majority,
    );
    let mut workflow =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Parallel);
    let a1 = workflow
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r1"), 1, false)
        .unwrap();
    workflow
        .assign_reviewer(AdaptationReviewerIdentity::local_user("r2"), 1, false)
        .unwrap();
    workflow.enqueue_pending().unwrap();
    workflow.start_review(&a1).unwrap();
    workflow.complete_assignment(&a1).unwrap();
    let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
    readiness.mark_risk_reviewed().unwrap();
    let mut archive = GovernanceArchiveContract::new("archive:dash");
    archive.archive_proposal(&proposal, "a1").unwrap();
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        Some(&conditions),
        None,
        None,
        Some(&archive),
        Some(&readiness),
        Some(&workflow),
        Some(&conflict),
        &[approve.clone()],
    );
    let compliance = GovernanceComplianceContract::verify(
        &package,
        Some(&evidence),
        Some(&conditions),
        &[approve],
        2,
        None,
        None,
        None,
        "t",
    );
    let dash = GovernanceReadinessDashboardProjection::project(
        &proposal,
        Some(&workflow),
        Some(&conditions),
        Some(&conflict),
        Some(&compliance),
        Some(&readiness),
        Some(&archive),
        Some(&package),
    );
    assert_eq!(dash.review_progress_total, 2);
    assert_eq!(dash.review_progress_completed, 1);
    assert!(!dash.outstanding_obligation_ids.is_empty());
    assert!(!dash.unresolved_conflict_ids.is_empty());
    assert_eq!(
        dash.compliance_status,
        GovernanceDashboardComplianceStatus::Failing
    );
    assert_eq!(dash.archived_snapshot_count, 1);
    assert!(dash.package_reference.is_some());
    assert!(workflow
        .assignments
        .iter()
        .any(|a| a.status == GovernanceReviewerAssignmentStatus::Completed));
}

#[test]
fn case2_dashboard_cannot_execute_or_publish() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", None).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let mut proposal = outcome.to_adaptation_proposal("a", "b", "c");
    proposal.require_review().unwrap();
    let dash = GovernanceReadinessDashboardProjection::project(
        &proposal,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    assert!(!dash.may_execute());
    assert!(!dash.may_publish());
    assert!(!dash.is_publication_ready_projection());
    assert!(GovernanceReadinessDashboardProjection::attempt_execute().is_err());
    assert!(GovernanceReadinessDashboardProjection::attempt_publish().is_err());
}
