//! Sprints 165–169 — Governance consolidation & boundary contract tests.

use workspace_domain::{
    governance_authority_is_none, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, GOVERNANCE_AUTHORITY_EFFECT_NONE, GovernanceAggregateRoot,
    GovernanceArchiveContract, GovernanceBoundaryOwner, GovernanceCompatibilityContract,
    GovernanceConditionContract, GovernanceConflictResolutionContract, GovernanceConsensusRule,
    GovernanceDecisionPackage, GovernanceDelegationContract, GovernanceExportPackage,
    GovernanceMetricsContract, GovernanceNotificationContract, GovernancePolicy,
    GovernanceReportContract, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    GovernanceReviewMode, GovernanceReviewWorkflowContract, GovernanceRisk,
    RecommendationConfidence, RecommendationEvidence, RecommendationGovernanceRecord,
    RecommendationItem, RecommendationKind, RecommendationLifecycleState,
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
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
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

/// CASE 1 — Aggregate roots remain governance-owned; public APIs still construct.
#[test]
fn case1_aggregates_coherent_and_apis_stable() {
    assert_eq!(GovernanceAggregateRoot::all().len(), 6);
    for root in GovernanceAggregateRoot::all() {
        assert_eq!(root.owner_domain(), GovernanceBoundaryOwner::Governance.as_str());
    }

    let proposal = proposal();
    let workflow =
        GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Optional);
    let conflict = GovernanceConflictResolutionContract::from_decisions(
        &proposal,
        &[],
        GovernanceConsensusRule::Majority,
    );
    let package = GovernanceDecisionPackage::assemble(
        &proposal,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&workflow),
        Some(&conflict),
        &[],
    );
    let notifications = GovernanceNotificationContract::new(&proposal);
    let delegation = GovernanceDelegationContract::new(&proposal);
    let metrics = GovernanceMetricsContract::new(&proposal);
    let report = GovernanceReportContract::governance_summary(&proposal, Some(&package), "t");
    let export =
        GovernanceExportPackage::from_artifacts(&proposal, Some(&package), None, None, None, "t");
    let archive = GovernanceArchiveContract::new("archive:consolidation");

    assert_eq!(workflow.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(conflict.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(package.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(notifications.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(delegation.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(metrics.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(report.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(export.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert_eq!(archive.authority_effect, GOVERNANCE_AUTHORITY_EFFECT_NONE);
    assert!(archive.append_only);
}

/// CASE 2 — Shared authority marker; no execute / publish / Gateway merge.
#[test]
fn case2_shared_authority_and_boundary_guards() {
    assert!(governance_authority_is_none(GOVERNANCE_AUTHORITY_EFFECT_NONE));
    assert!(!governance_authority_is_none("allow"));

    let proposal = proposal();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let policy = GovernancePolicy::from_risk(&risk);
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("r1"),
        GovernanceReviewDecisionKind::Approve,
        "Approve",
        "t",
        vec!["requires testing".into()],
        &proposal.provenance,
    )
    .unwrap();
    let conditions = GovernanceConditionContract::from_decision(&decision).unwrap();
    assert_eq!(
        conditions.authority_effect,
        GovernanceConditionContract::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        GovernanceConditionContract::AUTHORITY_EFFECT_NONE,
        GOVERNANCE_AUTHORITY_EFFECT_NONE
    );

    assert!(!conditions.may_execute());
    assert!(GovernanceConditionContract::attempt_execute().is_err());
    assert!(GovernanceReviewWorkflowContract::attempt_execute().is_err());
    assert!(GovernanceConflictResolutionContract::attempt_publish().is_err());
    assert!(GovernanceDelegationContract::attempt_transfer_execution().is_err());
    assert!(GovernanceExportPackage::from_artifacts(&proposal, None, None, None, None, "t")
        .attempt_import()
        .is_err());
    assert!(GovernanceCompatibilityContract::attempt_publish_runtime().is_err());

    // Boundary owners remain distinct labels.
    assert_ne!(
        GovernanceBoundaryOwner::Governance.as_str(),
        GovernanceBoundaryOwner::PermissionGateway.as_str()
    );
    assert_ne!(
        GovernanceBoundaryOwner::Experience.as_str(),
        GovernanceBoundaryOwner::UiPresentation.as_str()
    );
}
