//! Sprint 148 — Governance lifecycle integrity & timeline end-to-end contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, GovernanceDecisionEvidence, GovernanceLifecycleStage, GovernancePolicy,
    GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceRisk,
    GovernanceTimeline, GovernanceWorkspace, OutcomeAdaptationProposal, PublicationEnvironment,
    PublicationReadiness, PublicationRolloutStage, RecommendationConfidence,
    RecommendationEvidence, RecommendationGovernanceRecord, RecommendationItem,
    RecommendationKind, RecommendationLifecycleState, RecommendationOutcome,
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
                decision_intake_inspection: None,
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

struct LifecycleBundle {
    proposal: OutcomeAdaptationProposal,
    risk: GovernanceRisk,
    evidence: GovernanceDecisionEvidence,
    decision: GovernanceReviewDecision,
    readiness: PublicationReadiness,
    governance: GovernanceRecord,
    workspace: GovernanceWorkspace,
    timeline: GovernanceTimeline,
}

fn approved_lifecycle() -> LifecycleBundle {
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
    let mut evidence = GovernanceDecisionEvidence::assemble(
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
    evidence.record_dissent("reviewer_b", "Wording nit", "t-dissent");
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
        "life",
    )
    .unwrap();
    LifecycleBundle {
        proposal,
        risk,
        evidence,
        decision,
        readiness,
        governance,
        workspace,
        timeline,
    }
}

/// CASE 1 — Provenance survives full lifecycle.
#[test]
fn case1_provenance_survives_full_lifecycle() {
    let bundle = approved_lifecycle();
    let provenance = bundle.proposal.provenance.clone();
    assert!(bundle.timeline.provenance_survives(&provenance));
    assert_eq!(bundle.timeline.provenance, provenance);
    assert_eq!(bundle.governance.provenance, provenance);
    assert!(bundle.workspace.preserves_provenance(&provenance));
    assert!(bundle.timeline.stages().contains(&GovernanceLifecycleStage::ChangeProposal));
    assert!(bundle
        .timeline
        .stages()
        .contains(&GovernanceLifecycleStage::PublicationReadinessReached));
    bundle.timeline.validate_lifecycle_integrity().unwrap();
    let _ = (
        bundle.risk,
        bundle.evidence,
        bundle.decision,
        bundle.readiness,
    );
}

/// CASE 2 — Rejected changes remain historically visible.
#[test]
fn case2_rejected_changes_remain_historically_visible() {
    let outcome = accepted_outcome();
    let provenance = outcome.provenance.clone();
    let mut proposal = outcome.to_adaptation_proposal(
        "experience_presentation",
        "change",
        "effect",
    );
    proposal.require_review().unwrap();
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    let evidence = GovernanceDecisionEvidence::assemble(
        &risk,
        vec!["ref:1".into()],
        vec![],
        vec![],
        "Collected then rejected",
        &provenance,
    )
    .unwrap();
    let policy = GovernancePolicy::from_risk(&risk);
    let reject = GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Reject,
        "Does not fit",
        "t-rej",
        vec![],
        &provenance,
    )
    .unwrap()
    .with_evidence(&evidence);
    let timeline =
        GovernanceTimeline::from_rejected_lifecycle(&proposal, &risk, &evidence, &reject, "rej")
            .unwrap();
    assert!(timeline.rejected_visible);
    assert!(timeline
        .stages()
        .contains(&GovernanceLifecycleStage::RejectedVisible));
    assert!(timeline.provenance_survives(&provenance));
    assert!(timeline
        .events
        .iter()
        .any(|e| e.decision_reference.as_deref() == Some(reject.id.as_str())));
}

/// CASE 3 — Dissent remains immutable.
#[test]
fn case3_dissent_remains_immutable() {
    let bundle = approved_lifecycle();
    assert_eq!(bundle.timeline.dissent_immutable_refs.len(), 1);
    assert_eq!(
        bundle.timeline.dissent_immutable_refs,
        bundle
            .evidence
            .dissent_records
            .iter()
            .map(|d| d.id.clone())
            .collect::<Vec<_>>()
    );
    assert!(!bundle.timeline.may_rewrite_events());
    let mut timeline = bundle.timeline.clone();
    assert!(timeline.attempt_rewrite_event().is_err());
    assert!(!bundle.evidence.may_erase_approval_history());
}

/// CASE 4 — Governance cannot execute commands.
#[test]
fn case4_governance_cannot_execute_commands() {
    let bundle = approved_lifecycle();
    assert!(!bundle.timeline.may_execute());
    assert!(!bundle.timeline.may_grant_execution_authority());
    assert!(GovernanceTimeline::attempt_execute().is_err());
    assert!(GovernanceWorkspace::attempt_execute().is_err());
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

/// CASE 5 — Publication readiness cannot activate runtime.
#[test]
fn case5_publication_readiness_cannot_activate_runtime() {
    let bundle = approved_lifecycle();
    assert!(!bundle.timeline.may_activate_runtime());
    assert!(GovernanceTimeline::attempt_activate_runtime().is_err());
    let mut readiness = bundle.readiness.clone();
    assert!(readiness.attempt_activate_published().is_err());
    let env = PublicationEnvironment::from_governance_workspace(
        &bundle.workspace,
        "workspace:local",
        vec![],
        "behaviour:v0_baseline",
        PublicationRolloutStage::StagedCanary,
    );
    assert!(PublicationEnvironment::attempt_activate().is_err());
    assert!(!env.may_bypass_permission_gateway());
}
