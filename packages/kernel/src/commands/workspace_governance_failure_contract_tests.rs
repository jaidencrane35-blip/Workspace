//! Sprint 150 — Governance resilience & failure contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, BehaviourVersion, GovernanceActorRefs, GovernanceDecisionEvidence,
    GovernanceFailureCategory, GovernanceFailureRecoveryRequirement, GovernanceFailureRecoveryState,
    GovernanceFailureState, GovernancePolicy, GovernanceRecord, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceRisk, GovernanceWorkspace, OutcomeAdaptationProposal,
    PublicationEnvironment, PublicationReadiness, PublicationRolloutStage,
    PublicationSafetyContract, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState, RecommendationOutcome,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot execute")
                    || message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize")
                    || message.contains("recovery"),
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
        lifecycle_state: None,
        lifecycle_presented_at: None,
        lifecycle_resolved_at: None,
        lifecycle_resolution_type: None,
        explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
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

fn reviewed_bundle() -> (
    OutcomeAdaptationProposal,
    GovernanceDecisionEvidence,
    GovernancePolicy,
    GovernanceActorRefs,
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
    let actors = GovernanceActorRefs {
        proposer_actor_id: proposal.proposed_by_actor_id.clone(),
        reviewer_actor_id: Some("local_user".into()),
        publisher_actor_id: None,
    };
    (proposal, evidence, policy, actors)
}

/// CASE 1 — Failed reviews preserve history.
#[test]
fn case1_failed_reviews_preserve_history() {
    let (proposal, evidence, policy, actors) = reviewed_bundle();
    let reject = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Reject,
        "Not appropriate for publication",
        "t-rej",
        vec![],
        &proposal.provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let mut failure =
        GovernanceFailureState::from_failed_review(&reject, &evidence, actors).unwrap();
    let provenance_before = failure.provenance.clone();
    failure.record().unwrap();
    failure
        .plan_recovery(GovernanceFailureRecoveryRequirement::default_safe(
            "retain reject on timeline",
        ))
        .unwrap();
    assert!(failure.preserves_provenance(&provenance_before));
    assert!(failure.historically_visible);
    assert!(failure.preserves_evidence());
    assert!(failure
        .recovery_history
        .contains(&GovernanceFailureRecoveryState::Recorded));
    assert_eq!(failure.category, GovernanceFailureCategory::ReviewRejection);
}

/// CASE 2 — Failed validation preserves evidence.
#[test]
fn case2_failed_validation_preserves_evidence() {
    let (proposal, evidence, policy, actors) = reviewed_bundle();
    let decision = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve with evidence",
        "t-dec",
        vec!["no activation".into()],
        &proposal.provenance,
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
        Some(&GovernanceRisk::classify_from_proposal(&proposal)),
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
    let mut safety = PublicationSafetyContract::from_ready(&readiness, &env)
        .unwrap()
        .with_failed_gate("compat:schema_compatible");
    assert!(safety.run_validation().is_err());
    let mut failure = GovernanceFailureState::from_publication_validation_failure(
        &safety,
        evidence.id.clone(),
        actors,
    );
    failure.record().unwrap();
    assert!(failure.preserves_evidence());
    assert!(failure
        .preserved_evidence_references
        .contains(&evidence.id));
    assert!(failure.preserves_provenance(&proposal.provenance));
    assert_eq!(
        failure.category,
        GovernanceFailureCategory::PublicationValidation
    );
}

/// CASE 3 — Abandoned changes remain visible.
#[test]
fn case3_abandoned_changes_remain_visible() {
    let (proposal, _evidence, _policy, actors) = reviewed_bundle();
    let mut failure = GovernanceFailureState::from_review_expiry(
        &proposal,
        vec!["evidence:prior".into()],
        actors,
    );
    failure.record().unwrap();
    failure
        .plan_recovery(GovernanceFailureRecoveryRequirement::default_safe(
            "abandon expired review path",
        ))
        .unwrap();
    failure.abandon().unwrap();
    assert_eq!(
        failure.recovery_state,
        GovernanceFailureRecoveryState::Abandoned
    );
    assert!(failure.abandoned_remains_visible());
    assert!(failure.historically_visible);
    assert!(!failure.may_delete_provenance());
    assert!(failure.attempt_delete_provenance().is_err());
}

/// CASE 4 — Recovery cannot execute; Gateway remains separate.
#[test]
fn case4_recovery_cannot_execute() {
    let (proposal, evidence, policy, actors) = reviewed_bundle();
    let reject = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Reject,
        "Reject",
        "t-rej",
        vec![],
        &proposal.provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let mut failure =
        GovernanceFailureState::from_failed_review(&reject, &evidence, actors).unwrap();
    failure.record().unwrap();
    failure
        .plan_recovery(GovernanceFailureRecoveryRequirement::default_safe(
            "no execution path",
        ))
        .unwrap();
    assert!(!failure.may_execute());
    assert!(!failure.may_grant_authority());
    assert!(!failure.may_activate_runtime());
    assert!(!failure.may_bypass_permission_gateway());
    assert!(failure.attempt_recovery_execute().is_err());
    assert!(GovernanceFailureState::attempt_execute().is_err());
    assert!(GovernanceFailureState::attempt_grant_authority().is_err());
    assert!(GovernanceFailureState::attempt_bypass_gateway().is_err());
    assert!(GovernanceFailureState::attempt_activate_runtime().is_err());
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
