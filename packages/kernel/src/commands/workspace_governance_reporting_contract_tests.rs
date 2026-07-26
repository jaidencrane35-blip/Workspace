//! Sprint 163 — Governance reporting contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceArchiveContract, GovernanceComplianceContract, GovernanceDecisionEvidence,
    GovernanceDecisionPackage, GovernancePolicy, GovernanceReportContract, GovernanceReportKind,
    GovernanceRisk, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    PublicationReadiness, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState,
};

fn sample_item() -> RecommendationItem {
    RecommendationItem {
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
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[test]
fn case1_report_kinds_are_projections() {
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
    let mut proposal = outcome.to_adaptation_proposal("a", "b", "c");
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
        vec![],
        "Evidenced",
        &provenance,
    )
    .unwrap();
    let policy = GovernancePolicy::from_risk(&risk);
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t-dec",
        vec![],
        &provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
    readiness.mark_risk_reviewed().unwrap();
    let mut archive = GovernanceArchiveContract::new("archive:report");
    archive.archive_proposal(&proposal, "a1").unwrap();
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        None,
        None,
        None,
        Some(&archive),
        Some(&readiness),
        None,
        None,
        &[decision.clone()],
    );
    let compliance = GovernanceComplianceContract::verify(
        &package,
        Some(&evidence),
        None,
        &[decision.clone()],
        1,
        None,
        None,
        None,
        "t",
    );

    let summary = GovernanceReportContract::governance_summary(&proposal, Some(&package), "t");
    let compliance_report =
        GovernanceReportContract::compliance_summary(&proposal, &compliance, "t");
    let history =
        GovernanceReportContract::review_history(&proposal, &[decision.id.clone()], "t");
    let readiness_report =
        GovernanceReportContract::publication_readiness_report(&proposal, &readiness, "t");
    let risk_report = GovernanceReportContract::unresolved_risk_report(
        &proposal,
        &risk,
        vec!["awaiting second reviewer".into()],
        "t",
    );
    let archival = GovernanceReportContract::archival_report(&proposal, &archive, "t");

    assert_eq!(summary.kind, GovernanceReportKind::GovernanceSummary);
    assert_eq!(compliance_report.kind, GovernanceReportKind::ComplianceSummary);
    assert_eq!(history.kind, GovernanceReportKind::ReviewHistory);
    assert_eq!(
        readiness_report.kind,
        GovernanceReportKind::PublicationReadiness
    );
    assert_eq!(risk_report.kind, GovernanceReportKind::UnresolvedRisk);
    assert_eq!(archival.kind, GovernanceReportKind::Archival);
    assert!(!summary.sections.is_empty());
}

#[test]
fn case2_reports_cannot_execute() {
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
    let report = GovernanceReportContract::governance_summary(&proposal, None, "t");
    assert!(!report.may_execute());
    assert!(GovernanceReportContract::attempt_execute().is_err());
}
