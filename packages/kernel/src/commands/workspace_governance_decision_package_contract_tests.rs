//! Sprint 157 — Governance decision package contract tests.

use workspace_domain::{
    AdaptationReviewerIdentity, AttentionReason, AttentionSignal, AttentionSourceType,
    BehaviourVersion, GovernanceArchiveContract, GovernanceCompatibilityContract,
    GovernanceConditionContract, GovernanceDecisionEvidence, GovernanceDecisionPackage,
    GovernanceIntegrityVerification, GovernancePolicy, GovernanceRecord, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceReviewMode, GovernanceReviewWorkflowContract,
    GovernanceRisk, GovernanceTimeline, GovernanceWorkspace, PublicationEnvironment,
    PublicationReadiness, PublicationRolloutStage, RecommendationConfidence,
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
fn case1_package_aggregates_governance_references() {
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
        "Approve with conditions",
        "t-dec",
        vec!["requires testing".into()],
        &provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let conditions = GovernanceConditionContract::from_decision(&decision).unwrap();
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
    let mut archive = GovernanceArchiveContract::new("archive:pkg");
    archive.archive_proposal(&proposal, "a1").unwrap();
    archive.archive_review(&decision, "a2").unwrap();
    let mut workflow =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Optional);
    workflow
        .assign_reviewer(AdaptationReviewerIdentity::local_user("local_user"), 1, true)
        .unwrap();

    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        Some(&evidence),
        Some(&risk),
        Some(&conditions),
        Some(&compatibility),
        Some(&integrity),
        Some(&archive),
        Some(&readiness),
        Some(&workflow),
        None,
        &[decision],
    );
    assert_eq!(package.evidence_reference.as_deref(), Some(evidence.id.as_str()));
    assert!(package.condition_contract_reference.is_some());
    assert!(!package.obligation_ids.is_empty());
    assert!(package.compatibility_contract_reference.is_some());
    assert!(package.integrity_verification_reference.is_some());
    assert!(!package.archive_references.is_empty());
    assert!(package.publication_readiness_reference.is_some());
    assert!(package.workflow_reference.is_some());
    assert!(!package.sealed);
}

#[test]
fn case2_sealed_package_is_immutable_and_cannot_activate() {
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
    let mut package = GovernanceDecisionPackage::assemble(
        &proposal,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &[],
    );
    package.seal().unwrap();
    assert!(package.sealed);
    assert!(package.attempt_mutate_after_seal().is_err());
    assert!(package.seal().is_err());
    assert!(!package.may_activate());
    assert!(!package.may_execute());
    assert!(GovernanceDecisionPackage::attempt_activate().is_err());
    assert!(GovernanceDecisionPackage::attempt_execute().is_err());
}
