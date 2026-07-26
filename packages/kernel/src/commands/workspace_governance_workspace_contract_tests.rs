//! Sprint 147 — Governance workspace & publication environment contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, BehaviourVersion, ChangeEvaluation, ControlledChangeSurface,
    GovernanceDecisionEvidence, GovernancePolicy, GovernanceRecord, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceRisk, GovernanceWorkspace, OutcomeAdaptationProposal,
    PublicationEnvironment, PublicationReadiness, PublicationRolloutStage, PublishRequest,
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

fn approved_bundle() -> (
    OutcomeAdaptationProposal,
    GovernanceRisk,
    GovernanceDecisionEvidence,
    GovernanceReviewDecision,
    PublicationReadiness,
    GovernanceRecord,
    ChangeEvaluation,
    workspace_domain::BehaviourVersion,
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
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let evaluation =
        ChangeEvaluation::from_behaviour_version(&version, vec![], vec![], None);
    let governance = GovernanceRecord::from_adaptation_chain(
        &proposal,
        Some(&surface),
        Some(&version),
        Some(&evaluation),
        None,
    )
    .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
    .unwrap()
    .with_evidence_and_readiness(&evidence, &readiness, &[decision.clone()])
    .unwrap();
    (
        proposal,
        risk,
        evidence,
        decision,
        readiness,
        governance,
        evaluation,
        version,
    )
}

/// CASE 1 — UI cannot approve execution.
#[test]
fn case1_ui_cannot_approve_execution() {
    let (proposal, risk, evidence, decision, readiness, governance, _, _) = approved_bundle();
    let workspace = GovernanceWorkspace::from_governance_bundle(
        &proposal,
        &governance,
        Some(&evidence),
        Some(&risk),
        &[decision],
        Some(&readiness),
    );
    assert!(!workspace.may_approve_execution());
    assert!(!workspace.may_grant_execution_authority());
    assert!(GovernanceWorkspace::attempt_approve_execution().is_err());
    assert!(GovernanceWorkspace::attempt_execute().is_err());
    assert_eq!(workspace.authority_effect, "none");
}

/// CASE 2 — Publication environment cannot activate runtime.
#[test]
fn case2_publication_environment_cannot_activate_runtime() {
    let (proposal, risk, evidence, decision, readiness, governance, evaluation, version) =
        approved_bundle();
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
    assert!(!env.may_activate_runtime());
    assert!(PublicationEnvironment::attempt_activate().is_err());
    assert!(PublicationEnvironment::attempt_execute().is_err());

    let publish = PublishRequest::from_evaluated_version(
        &version,
        &evaluation,
        &governance,
        AdaptationReviewerIdentity::local_user("local_user"),
    )
    .unwrap();
    env.bind_publish_request(&publish).unwrap();
    assert!(publish.attempt_activate_published_version().is_err());
    let _ = decision;
}

/// CASE 3 — Governance views preserve provenance.
#[test]
fn case3_governance_views_preserve_provenance() {
    let (proposal, risk, evidence, decision, readiness, governance, _, _) = approved_bundle();
    let provenance = proposal.provenance.clone();
    let workspace = GovernanceWorkspace::from_governance_bundle(
        &proposal,
        &governance,
        Some(&evidence),
        Some(&risk),
        &[decision],
        Some(&readiness),
    );
    assert!(workspace.preserves_provenance(&provenance));
    assert_eq!(workspace.provenance, provenance);
    let env = PublicationEnvironment::from_governance_workspace(
        &workspace,
        "workspace:local",
        vec![],
        BehaviourVersion::BASELINE_ID,
        PublicationRolloutStage::None,
    );
    assert!(env.preserves_provenance(&provenance));
    assert_eq!(
        workspace.proposal_view.proposal_reference,
        proposal.id
    );
    assert_eq!(
        workspace.evidence_view.evidence_reference.as_deref(),
        Some(evidence.id.as_str())
    );
    assert!(!workspace.decision_history.is_empty());
}

/// CASE 4 — Permission Gateway remains separate.
#[test]
fn case4_permission_gateway_remains_separate() {
    let (proposal, risk, evidence, decision, readiness, governance, _, _) = approved_bundle();
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
        vec![],
        BehaviourVersion::BASELINE_ID,
        PublicationRolloutStage::Full,
    );
    assert!(!workspace.may_bypass_permission_gateway());
    assert!(!env.may_bypass_permission_gateway());
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
