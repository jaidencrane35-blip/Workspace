//! Sprint 164 — Governance export contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    BehaviourVersion, GovernanceArchiveContract, GovernanceCompatibilityContract,
    GovernanceDecisionEvidence, GovernanceDecisionPackage, GovernanceExportPackage,
    GovernancePolicy, GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    GovernanceRisk, GovernanceWorkspace, PublicationEnvironment, PublicationReadiness,
    PublicationRolloutStage, RecommendationConfidence, RecommendationEvidence,
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
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[test]
fn case1_export_bundles_provenance_evidence_archive_compatibility() {
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
    let mut archive = GovernanceArchiveContract::new("archive:export");
    archive.archive_proposal(&proposal, "a1").unwrap();
    archive.archive_review(&decision, "a2").unwrap();
    let mut package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        None,
        Some(&compatibility),
        None,
        Some(&archive),
        Some(&readiness),
        None,
        None,
        &[decision],
    );
    package.seal().unwrap();

    let export = GovernanceExportPackage::from_artifacts(
        &proposal,
        Some(&package),
        Some(&evidence),
        Some(&archive),
        Some(&compatibility),
        "t-export",
    );
    assert!(export.read_only);
    assert_eq!(export.provenance_bundle, provenance);
    assert!(export.evidence_references.contains(&evidence.id));
    assert_eq!(export.archive_references.len(), 2);
    assert!(!export.compatibility_metadata.is_empty());
    assert_eq!(
        export.version_metadata,
        GovernanceExportPackage::VERSION_METADATA
    );
    assert!(export
        .integrity_hash_placeholder
        .starts_with("integrity_placeholder:"));
    assert_eq!(
        export.decision_package_reference.as_deref(),
        Some(package.id.as_str())
    );
}

#[test]
fn case2_export_forbids_import_sync_publish() {
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
    let export =
        GovernanceExportPackage::from_artifacts(&proposal, None, None, None, None, "t");
    assert!(!export.may_import());
    assert!(!export.may_synchronise());
    assert!(!export.may_publish());
    assert!(!export.may_execute());
    assert!(export.attempt_import().is_err());
    assert!(export.attempt_synchronise().is_err());
    assert!(export.attempt_publish().is_err());
    assert!(GovernanceExportPackage::attempt_execute().is_err());
}
