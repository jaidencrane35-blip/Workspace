//! Sprint 152 — Governance compatibility & dependency contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, BehaviourVersion, GovernanceCompatibilityContract,
    GovernanceDecisionEvidence, GovernanceDependencyKind, GovernancePolicy, GovernanceRecord,
    GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceRisk, GovernanceWorkspace,
    PublicationEnvironment, PublicationReadiness, PublicationRolloutStage,
    RecommendationConfidence, RecommendationEvidence, RecommendationGovernanceRecord,
    RecommendationItem, RecommendationKind, RecommendationLifecycleState,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("activate")
                    || message.contains("publish"),
                "{label}: expected block-style error, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed"),
    }
}

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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn publication_environment() -> (PublicationEnvironment, workspace_domain::RecommendationProvenance)
{
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
        &[decision],
        Some(&readiness),
    );
    let env = PublicationEnvironment::from_governance_workspace(
        &workspace,
        "workspace:local",
        vec!["schema_compatible".into()],
        BehaviourVersion::BASELINE_ID,
        PublicationRolloutStage::StagedCanary,
    );
    (env, provenance)
}

#[test]
fn case1_dependencies_reuse_migration_and_schema_patterns() {
    let (env, provenance) = publication_environment();
    let contract =
        GovernanceCompatibilityContract::from_publication_environment(&env, &provenance);
    assert!(contract
        .dependencies
        .iter()
        .any(|d| d.kind == GovernanceDependencyKind::MigrationOrdering));
    assert!(contract
        .dependencies
        .iter()
        .any(|d| d.kind == GovernanceDependencyKind::SchemaCompatibility));
    assert!(contract
        .dependencies
        .iter()
        .any(|d| d.kind == GovernanceDependencyKind::PackageCompatibility));
    assert_eq!(
        GovernanceDependencyKind::MigrationOrdering.pattern_source(),
        "MigrationRunner ordered versions"
    );
    contract.verify_declared().unwrap();
}

#[test]
fn case2_unsatisfied_prerequisite_blocks_declared_verify() {
    let (env, provenance) = publication_environment();
    let mut contract =
        GovernanceCompatibilityContract::from_publication_environment(&env, &provenance);
    contract.declare_prerequisite("publication_safety:prior", "safety must be release-approved");
    assert!(contract.verify_declared().is_err());
    let prereq = contract
        .dependencies
        .iter()
        .find(|d| d.kind == GovernanceDependencyKind::PrerequisiteContract)
        .unwrap()
        .id
        .clone();
    contract.mark_dependency_satisfied(&prereq).unwrap();
    contract.verify_declared().unwrap();
}

#[test]
fn case3_compatibility_cannot_publish_or_bypass_gateway() {
    let (env, provenance) = publication_environment();
    let contract =
        GovernanceCompatibilityContract::from_publication_environment(&env, &provenance);
    assert!(!contract.may_publish_runtime());
    assert!(!contract.may_execute());
    assert!(!contract.may_bypass_permission_gateway());
    assert!(GovernanceCompatibilityContract::attempt_publish_runtime().is_err());
    assert!(GovernanceCompatibilityContract::attempt_execute().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
}
