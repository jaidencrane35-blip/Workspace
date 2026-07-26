//! Sprint 154 — Governance archive & historical preservation contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceArchiveContract, GovernanceArchiveKind, GovernanceDecisionEvidence, GovernancePolicy,
    GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceRisk,
    GovernanceTimeline, GovernanceWorkspace, PublicationReadiness, RecommendationConfidence,
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
                decision_intake_compatibility: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[test]
fn case1_archive_preserves_proposal_review_publication_timeline() {
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
    readiness.mark_approved(&evidence).unwrap();
    readiness.mark_ready_for_publication(&evidence).unwrap();
    let governance = GovernanceRecord::from_adaptation_chain(&proposal, None, None, None, None)
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[decision.clone()])
        .unwrap();
    let workspace = GovernanceWorkspace::from_governance_bundle(
        &proposal,
        &governance,
        Some(&evidence),
        Some(&risk),
        &[decision.clone()],
        Some(&readiness),
    );
    let timeline = GovernanceTimeline::from_full_lifecycle(
        &proposal,
        &risk,
        &evidence,
        &[decision.clone()],
        &governance,
        &workspace,
        &readiness,
        "t",
    )
    .unwrap();

    let mut archive = GovernanceArchiveContract::new("archive:gov");
    archive.archive_proposal(&proposal, "a1").unwrap();
    archive.archive_review(&decision, "a2").unwrap();
    archive
        .archive_publication_readiness(&readiness, "a3")
        .unwrap();
    archive.archive_timeline(&timeline, "a4").unwrap();
    assert_eq!(archive.snapshots.len(), 4);
    assert!(archive
        .snapshots
        .iter()
        .any(|s| s.kind == GovernanceArchiveKind::ProposalSnapshot));
    assert!(archive
        .snapshots
        .iter()
        .any(|s| s.kind == GovernanceArchiveKind::ReviewRecord));
    assert!(archive
        .snapshots
        .iter()
        .any(|s| s.kind == GovernanceArchiveKind::PublicationRecord));
    assert!(archive
        .snapshots
        .iter()
        .any(|s| s.kind == GovernanceArchiveKind::TimelineSnapshot));
    assert!(archive.preserves_all_history());
}

#[test]
fn case2_superseded_proposals_remain_visible() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", None).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let provenance = outcome.provenance.clone();
    let mut archive = GovernanceArchiveContract::new("archive:supersede");
    archive
        .mark_superseded("proposal:old", "proposal:new", &provenance, "t-s1")
        .unwrap();
    archive
        .mark_superseded("proposal:old", "proposal:newer", &provenance, "t-s2")
        .unwrap();
    assert_eq!(archive.snapshots.len(), 2);
    assert!(archive
        .snapshots
        .iter()
        .all(|s| s.kind == GovernanceArchiveKind::SupersededProposal));
    assert!(archive.snapshots.iter().all(|s| s.superseded_by.is_some()));
    assert!(archive.append_only);
}

#[test]
fn case3_archive_cannot_delete_or_activate() {
    let mut archive = GovernanceArchiveContract::new("archive:guard");
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", None).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    archive
        .mark_superseded("p1", "p2", &outcome.provenance, "t")
        .unwrap();
    let before = archive.snapshots.len();
    assert!(!archive.may_delete());
    assert!(!archive.may_activate_runtime());
    assert!(!archive.may_execute());
    assert!(archive.attempt_delete().is_err());
    assert!(GovernanceArchiveContract::attempt_activate_runtime().is_err());
    assert!(GovernanceArchiveContract::attempt_execute().is_err());
    assert_eq!(archive.snapshots.len(), before);
}
