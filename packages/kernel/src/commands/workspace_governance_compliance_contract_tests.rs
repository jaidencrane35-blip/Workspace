//! Sprint 158 — Governance compliance contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    BehaviourVersion, GovernanceCompatibilityContract, GovernanceComplianceCheckKind,
    GovernanceComplianceContract, GovernanceComplianceSeverity, GovernanceConditionContract,
    GovernanceDecisionEvidence, GovernanceDecisionPackage, GovernanceIntegrityVerification,
    GovernanceObligationStatus, GovernancePolicy, GovernanceRecord, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceRisk, GovernanceTimeline, GovernanceWorkspace,
    PublicationEnvironment, PublicationReadiness, PublicationRolloutStage,
    PublicationSafetyContract, RecommendationConfidence, RecommendationEvidence,
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
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn bundle() -> (
    workspace_domain::OutcomeAdaptationProposal,
    GovernanceDecisionEvidence,
    GovernanceReviewDecision,
    GovernanceConditionContract,
    GovernanceCompatibilityContract,
    GovernanceIntegrityVerification,
    PublicationSafetyContract,
    GovernanceRisk,
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
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t-dec",
        vec!["requires testing".into()],
        &provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let mut conditions = GovernanceConditionContract::from_decision(&decision).unwrap();
    for o in conditions.obligations.clone() {
        let _ = conditions.mark_satisfied(&o.id);
    }
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
    let env = PublicationEnvironment::from_governance_workspace(
        &workspace,
        "workspace:local",
        vec!["schema_compatible".into()],
        BehaviourVersion::BASELINE_ID,
        PublicationRolloutStage::StagedCanary,
    );
    let compatibility =
        GovernanceCompatibilityContract::from_publication_environment(&env, &provenance);
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
    let integrity =
        GovernanceIntegrityVerification::verify_timeline(&timeline, &provenance, "check");
    let mut safety = PublicationSafetyContract::from_ready(&readiness, &env).unwrap();
    safety.prepare_migration().unwrap();
    safety.prepare_rollback().unwrap();
    safety.approve_release().unwrap();
    (
        proposal,
        evidence,
        decision,
        conditions,
        compatibility,
        integrity,
        safety,
        risk,
    )
}

#[test]
fn case1_compliance_reports_passing_diagnostics() {
    let (proposal, evidence, decision, conditions, compatibility, integrity, safety, risk) =
        bundle();
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        Some(&conditions),
        Some(&compatibility),
        Some(&integrity),
        None,
        None,
        None,
        None,
        &[decision.clone()],
    );
    let report = GovernanceComplianceContract::verify(
        &package,
        Some(&evidence),
        Some(&conditions),
        &[decision],
        1,
        Some(&compatibility),
        Some(&integrity),
        Some(&safety),
        "t-check",
    );
    assert!(!report.has_errors());
    assert!(report.diagnostics.iter().any(|d| {
        d.kind == GovernanceComplianceCheckKind::RequiredEvidenceExists
            && d.severity == GovernanceComplianceSeverity::Info
    }));
    assert!(conditions
        .obligations
        .iter()
        .all(|o| o.status == GovernanceObligationStatus::Satisfied));
}

#[test]
fn case2_missing_evidence_is_compliance_error() {
    let (proposal, _evidence, decision, conditions, compatibility, integrity, safety, risk) =
        bundle();
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        None,
        Some(&risk),
        Some(&conditions),
        Some(&compatibility),
        Some(&integrity),
        None,
        None,
        None,
        None,
        &[decision.clone()],
    );
    let report = GovernanceComplianceContract::verify(
        &package,
        None,
        Some(&conditions),
        &[decision],
        1,
        Some(&compatibility),
        Some(&integrity),
        Some(&safety),
        "t-check",
    );
    assert!(report.has_errors());
    assert!(report.diagnostics.iter().any(|d| {
        d.kind == GovernanceComplianceCheckKind::RequiredEvidenceExists
            && d.severity == GovernanceComplianceSeverity::Error
    }));
}

#[test]
fn case3_compliance_never_repairs_or_grants_authority() {
    let (proposal, evidence, decision, conditions, compatibility, integrity, safety, risk) =
        bundle();
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        Some(&conditions),
        Some(&compatibility),
        Some(&integrity),
        None,
        None,
        None,
        None,
        &[decision.clone()],
    );
    let report = GovernanceComplianceContract::verify(
        &package,
        Some(&evidence),
        Some(&conditions),
        &[decision],
        1,
        Some(&compatibility),
        Some(&integrity),
        Some(&safety),
        "t-check",
    );
    assert!(!report.may_repair_automatically());
    assert!(!report.may_grant_authority());
    assert!(report.attempt_repair().is_err());
    assert!(GovernanceComplianceContract::attempt_grant_authority().is_err());
    assert!(GovernanceComplianceContract::attempt_execute().is_err());
}
