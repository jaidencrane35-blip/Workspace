//! Sprint 143 — Governance ledger & change publication contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, ChangeEvaluation, ControlledChangeSurface, GovernanceRecord,
    OutcomeAdaptationProposal, PublishRequest, PublishedVersionRecord, RecommendationConfidence,
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
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
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

fn approved_chain() -> (
    OutcomeAdaptationProposal,
    ControlledChangeSurface,
    workspace_domain::BehaviourVersion,
    ChangeEvaluation,
    workspace_domain::BehaviourVersion,
    GovernanceRecord,
) {
    let outcome = accepted_outcome();
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
    let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
    let version = surface.to_behaviour_version_draft().unwrap();
    let rollback = version.prepare_rollback_draft("safer prior").unwrap();
    let evaluation = ChangeEvaluation::from_behaviour_version(
        &version,
        vec!["users resume faster".into()],
        vec!["continuity click-through improves".into()],
        Some("rollback if false positives rise".into()),
    );
    let governance = GovernanceRecord::from_adaptation_chain(
        &proposal,
        Some(&surface),
        Some(&version),
        Some(&evaluation),
        Some(&rollback),
    );
    (proposal, surface, version, evaluation, rollback, governance)
}

/// CASE 1 — Every change retains provenance.
#[test]
fn case1_every_change_retains_provenance() {
    let (proposal, surface, version, evaluation, rollback, governance) = approved_chain();
    let provenance = proposal.provenance.clone();
    assert!(governance.retains_provenance(&provenance));
    assert_eq!(surface.provenance, provenance);
    assert_eq!(version.provenance, provenance);
    assert_eq!(evaluation.provenance_snapshot, provenance);
    assert_eq!(rollback.provenance, provenance);
    let publish = PublishRequest::from_evaluated_version(
        &version,
        &evaluation,
        &governance,
        AdaptationReviewerIdentity::local_user("local_user"),
    )
    .unwrap();
    assert_eq!(publish.provenance, provenance);
}

/// CASE 2 — Publication requires approval.
#[test]
fn case2_publication_requires_approval() {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal("area", "change", "effect");
    proposal.require_review().unwrap();
    let governance = GovernanceRecord::from_adaptation_chain(
        &proposal, None, None, None, None,
    );
    assert!(governance.approval_reference.is_none());
    assert!(PublishRequest::attempt_record_published_without_approval().is_err());

    let (proposal, _surface, version, evaluation, _rollback, governance) = approved_chain();
    assert!(governance.approval_reference.is_some());
    let mut publish = PublishRequest::from_evaluated_version(
        &version,
        &evaluation,
        &governance,
        AdaptationReviewerIdentity::local_user("local_user"),
    )
    .unwrap();
    publish.submit_for_governance_review().unwrap();
    publish
        .approve_for_publish(&AdaptationReviewerIdentity::local_user("local_user"))
        .unwrap();
    assert!(publish.attempt_activate_published_version().is_err());
    assert!(PublishedVersionRecord::attempt_from_publish_request(&publish).is_err());
    assert_eq!(proposal.authority_effect, "none");
}

/// CASE 3 — Rollback preserves history.
#[test]
fn case3_rollback_preserves_history() {
    let (proposal, _surface, version, evaluation, rollback, governance) = approved_chain();
    let provenance = proposal.provenance.clone();
    assert_eq!(rollback.provenance, provenance);
    assert_eq!(
        governance.rollback_reference.as_deref(),
        Some(rollback.id.as_str())
    );
    assert_eq!(governance.version_reference.as_deref(), Some(version.id.as_str()));
    assert_eq!(
        governance.evaluation_reference.as_deref(),
        Some(evaluation.id.as_str())
    );
    // Original version provenance unchanged after rollback draft.
    assert_eq!(version.provenance, provenance);
    assert!(governance.timestamps.published_at.is_none());
}

/// CASE 4 — Governance cannot grant execution authority.
#[test]
fn case4_governance_cannot_grant_execution_authority() {
    let (_proposal, surface, version, evaluation, _rollback, governance) = approved_chain();
    let publish = PublishRequest::from_evaluated_version(
        &version,
        &evaluation,
        &governance,
        AdaptationReviewerIdentity::local_user("local_user"),
    )
    .unwrap();
    assert!(!governance.may_grant_execution_authority());
    assert!(!publish.may_grant_execution_authority());
    assert!(!governance.may_bypass_permission_gateway());
    assert!(!publish.may_bypass_permission_gateway());
    assert!(GovernanceRecord::attempt_grant_execution_authority().is_err());
    assert!(PublishRequest::attempt_grant_execution_authority().is_err());
    assert!(GovernanceRecord::attempt_execute().is_err());
    assert!(PublishRequest::attempt_execute().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert!(!surface.may_bypass_permission_gateway());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_eq!(governance.authority_effect, "none");
    assert_eq!(publish.authority_effect, "none");
}
