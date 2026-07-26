//! Sprint 153 — Governance integrity verification contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    GovernanceDecisionEvidence, GovernanceIntegrityCheckKind,
    GovernanceIntegrityDiagnosticSeverity, GovernanceIntegrityVerification, GovernancePolicy,
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
                decision_readiness: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn full_bundle() -> (
    workspace_domain::OutcomeAdaptationProposal,
    GovernanceDecisionEvidence,
    Vec<GovernanceReviewDecision>,
    GovernanceRecord,
    GovernanceTimeline,
) {
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
    let mut evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec![format!("outcome:{}", outcome.id)],
        vec![],
        vec!["no activation".into()],
        "Evidenced",
        &provenance,
    )
    .unwrap();
    evidence.record_dissent("reviewer-b", "Prefer alternate wording", "t-dissent");
    let policy = GovernancePolicy::from_risk(&risk);
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve with evidence",
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
    (proposal, evidence, vec![decision], governance, timeline)
}

#[test]
fn case1_valid_bundle_produces_info_diagnostics() {
    let (proposal, evidence, decisions, record, timeline) = full_bundle();
    let report = GovernanceIntegrityVerification::verify_bundle(
        &proposal,
        Some(&evidence),
        &decisions,
        &record,
        Some(&timeline),
        "check-1",
    );
    assert!(!report.has_errors());
    assert!(report.diagnostics.iter().any(|d| {
        d.severity == GovernanceIntegrityDiagnosticSeverity::Info
            && d.kind == GovernanceIntegrityCheckKind::TimelineConsistency
    }));
}

#[test]
fn case2_missing_evidence_and_empty_review_chain_are_errors() {
    let (proposal, _evidence, _decisions, record, _timeline) = full_bundle();
    let report = GovernanceIntegrityVerification::verify_bundle(
        &proposal,
        None,
        &[],
        &record,
        None,
        "check-2",
    );
    assert!(report.has_errors());
    assert!(report.diagnostics.iter().any(|d| {
        d.kind == GovernanceIntegrityCheckKind::MissingEvidence
            && d.severity == GovernanceIntegrityDiagnosticSeverity::Error
    }));
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == GovernanceIntegrityCheckKind::InvalidReviewChain));
}

#[test]
fn case3_verification_never_repairs_or_mutates() {
    let (proposal, evidence, decisions, record, timeline) = full_bundle();
    let report = GovernanceIntegrityVerification::verify_bundle(
        &proposal,
        Some(&evidence),
        &decisions,
        &record,
        Some(&timeline),
        "check-3",
    );
    assert!(!report.may_repair_automatically());
    assert!(!report.may_mutate_governance());
    assert!(!report.may_execute());
    assert!(report.attempt_repair().is_err());
    assert!(report.attempt_mutate_governance().is_err());
    assert!(GovernanceIntegrityVerification::attempt_execute().is_err());
}
