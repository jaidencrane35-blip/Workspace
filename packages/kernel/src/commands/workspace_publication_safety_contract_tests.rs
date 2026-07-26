//! Sprint 149 — Publication safety contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, BehaviourVersion, GovernanceDecisionEvidence, GovernancePolicy,
    GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceRisk,
    GovernanceWorkspace, OutcomeAdaptationProposal, PublicationEnvironment, PublicationReadiness,
    PublicationRolloutStage, PublicationSafetyContract, PublicationSafetyLifecycleState,
    RecommendationConfidence, RecommendationEvidence, RecommendationGovernanceRecord,
    RecommendationItem, RecommendationKind, RecommendationLifecycleState, RecommendationOutcome,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize"),
                "{label}: expected CannotExecute-style error, got {err}"
            );
        }
        Ok(()) => panic!("{label}: must not succeed at attempt_execute"),
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
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn accepted_outcome() -> RecommendationOutcome {
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
    record.record_outcome("t4").unwrap()
}

fn ready_pair() -> (
    OutcomeAdaptationProposal,
    PublicationReadiness,
    PublicationEnvironment,
) {
    let outcome = accepted_outcome();
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
        vec![
            format!("outcome:{}", outcome.id),
            "exact:continuity.resumable".into(),
        ],
        vec![],
        vec!["no activation".into()],
        "Presentation-only change evidenced",
        &provenance,
    )
    .unwrap();
    let policy = GovernancePolicy::from_risk(&risk);
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve with evidence",
        "t-dec",
        vec!["no activation".into()],
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
    (proposal, readiness, env)
}

/// CASE 1 — Failed validation blocks publication.
#[test]
fn case1_failed_validation_blocks_publication() {
    let (_proposal, readiness, env) = ready_pair();
    let mut safety = PublicationSafetyContract::from_ready(&readiness, &env)
        .unwrap()
        .with_failed_gate("compat:schema_compatible");
    assert!(safety.run_validation().is_err());
    assert_eq!(
        safety.lifecycle_state,
        PublicationSafetyLifecycleState::ValidationFailed
    );
    assert!(safety.prepare_migration().is_err());
    assert!(safety.approve_release().is_err());
    assert!(safety.attempt_publish_activate().is_err());
}

/// CASE 2 — Rollback preserves history.
#[test]
fn case2_rollback_preserves_history() {
    let (_proposal, readiness, env) = ready_pair();
    let mut safety = PublicationSafetyContract::from_ready(&readiness, &env).unwrap();
    let prior = safety.lifecycle_history.clone();
    safety.prepare_migration().unwrap();
    safety.prepare_rollback().unwrap();
    assert!(safety.rollback_preserves_history());
    assert!(safety.failure_handling.preserves_history);
    assert!(safety
        .rollback_requirements
        .iter()
        .all(|r| r.history_preserved && r.prepared));
    for state in prior {
        assert!(safety.lifecycle_history.contains(&state));
    }
    assert!(safety
        .lifecycle_history
        .contains(&PublicationSafetyLifecycleState::RollbackPrepared));
}

/// CASE 3 — Publication cannot activate runtime.
#[test]
fn case3_publication_cannot_activate_runtime() {
    let (_proposal, readiness, env) = ready_pair();
    let mut safety = PublicationSafetyContract::from_ready(&readiness, &env).unwrap();
    safety.prepare_migration().unwrap();
    safety.prepare_rollback().unwrap();
    safety.approve_release().unwrap();
    assert!(!safety.may_activate_runtime());
    assert!(safety.attempt_publish_activate().is_err());
    assert!(!safety.may_mutate_cognition());
    assert!(!safety.may_rewrite_provenance());
    assert!(PublicationSafetyContract::attempt_mutate_cognition().is_err());
    assert!(PublicationSafetyContract::attempt_rewrite_provenance().is_err());
    assert_eq!(
        safety.lifecycle_state,
        PublicationSafetyLifecycleState::ReleaseApproved
    );
}

/// CASE 4 — Gateway remains separate.
#[test]
fn case4_gateway_remains_separate() {
    let (_proposal, readiness, env) = ready_pair();
    let safety = PublicationSafetyContract::from_ready(&readiness, &env).unwrap();
    assert!(!safety.may_execute_commands());
    assert!(!safety.may_bypass_permission_gateway());
    assert!(PublicationSafetyContract::attempt_execute().is_err());
    assert!(PublicationSafetyContract::attempt_bypass_gateway().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
}
