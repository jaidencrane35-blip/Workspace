//! Sprint 151 — Governance condition & obligation contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AttentionReason, AttentionSignal,
    AttentionSourceType, GovernanceConditionContract, GovernanceObligationKind,
    GovernanceObligationStatus, GovernancePolicy, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceRisk, RecommendationConfidence,
    RecommendationEvidence, RecommendationGovernanceRecord, RecommendationItem,
    RecommendationKind, RecommendationLifecycleState,
};

fn assert_cannot_execute(label: &str, result: Result<(), KernelError>) {
    match result {
        Err(err) => {
            let message = err.to_string().to_lowercase();
            assert!(
                message.contains("cannot")
                    || message.contains("grant")
                    || message.contains("authorize")
                    || message.contains("obligation")
                    || message.contains("condition"),
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
    }
}

fn approve_with_conditions() -> GovernanceReviewDecision {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
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
    let policy = GovernancePolicy::from_risk(&risk);
    GovernanceReviewDecision::new(
        &policy,
        AdaptationReviewerIdentity::local_user("local_user"),
        GovernanceReviewDecisionKind::Approve,
        "Approve with obligations",
        "t-dec",
        vec![
            "requires documentation".into(),
            "requires testing".into(),
            "requires compatibility verification".into(),
            "requires rollback plan".into(),
            "requires second reviewer".into(),
            "expires if unmet".into(),
        ],
        &outcome.provenance,
    )
    .unwrap()
}

#[test]
fn case1_conditions_attach_without_execution() {
    let decision = approve_with_conditions();
    let contract = GovernanceConditionContract::from_decision(&decision).unwrap();
    assert!(contract.obligations.len() >= 6);
    assert!(contract
        .obligations
        .iter()
        .any(|o| o.kind == GovernanceObligationKind::RequiresDocumentation));
    assert!(contract
        .obligations
        .iter()
        .any(|o| o.kind == GovernanceObligationKind::RequiresSecondReviewer));
    assert!(!contract.may_execute());
    assert!(GovernanceConditionContract::attempt_execute().is_err());
}

#[test]
fn case2_unmet_expiry_does_not_grant_authority() {
    let decision = approve_with_conditions();
    let mut contract = GovernanceConditionContract::from_decision(&decision).unwrap();
    let expirables: Vec<_> = contract
        .obligations
        .iter()
        .filter(|o| o.expires_if_unmet)
        .map(|o| o.id.clone())
        .collect();
    assert!(!expirables.is_empty());
    for id in expirables {
        contract.mark_unmet(&id).unwrap();
    }
    assert!(contract.unmet_or_expired());
    assert!(contract
        .obligations
        .iter()
        .any(|o| o.status == GovernanceObligationStatus::Expired));
    assert!(!contract.may_grant_authority());
    assert!(GovernanceConditionContract::attempt_grant_authority().is_err());
}

#[test]
fn case3_conditions_cannot_mutate_cognition() {
    let decision = approve_with_conditions();
    let contract = GovernanceConditionContract::from_decision(&decision).unwrap();
    assert!(!contract.may_mutate_cognition());
    assert!(GovernanceConditionContract::attempt_mutate_cognition().is_err());
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
}
