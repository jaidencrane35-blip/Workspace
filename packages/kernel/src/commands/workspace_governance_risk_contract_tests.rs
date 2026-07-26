//! Sprint 145 — Governance risk classification & review routing contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AdaptationReviewerIdentity, AdaptationRiskClass, AttentionReason,
    AttentionSignal, AttentionSourceType, GovernanceImpactClass, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceReviewRouting, GovernanceRisk,
    OutcomeAdaptationProposal, RecommendationConfidence, RecommendationEvidence,
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

fn approved_proposal(area: &str) -> OutcomeAdaptationProposal {
    let outcome = accepted_outcome();
    let mut proposal = outcome.to_adaptation_proposal(area, "change", "effect");
    proposal.require_review().unwrap();
    proposal
        .approve(
            AdaptationReviewerIdentity::local_user("local_user"),
            "t-ok",
        )
        .unwrap();
    proposal
}

/// CASE 1 — Risk cannot execute actions.
#[test]
fn case1_risk_cannot_execute_actions() {
    let risk = GovernanceRisk::from_level(
        AdaptationRiskClass::High,
        "behaviour_workflow",
        GovernanceImpactClass::BehaviourVersionDraft,
    );
    assert!(!risk.may_execute());
    assert!(GovernanceRisk::attempt_execute().is_err());
    assert!(GovernanceReviewRouting::attempt_execute().is_err());
    assert_eq!(risk.authority_effect, "none");
}

/// CASE 2 — High-risk changes require stronger review.
#[test]
fn case2_high_risk_requires_stronger_review() {
    let proposal = approved_proposal("behaviour_workflow");
    let routing = GovernanceReviewRouting::route_change(&proposal, None).unwrap();
    assert_eq!(routing.risk.risk_level, AdaptationRiskClass::High);
    assert!(routing.risk.requires_conditions);
    assert_eq!(routing.required_reviewer_count, 2);
    assert_eq!(routing.policy.approval_threshold, 2);

    let a = GovernanceReviewDecision::new(
        &routing.policy,
        AdaptationReviewerIdentity::local_user("reviewer_a"),
        GovernanceReviewDecisionKind::Approve,
        "First",
        "t1",
        vec!["no activation".into()],
        &proposal.provenance,
    )
    .unwrap();
    assert!(routing.enforce_decisions(&proposal, &[a.clone()]).is_err());

    let b = GovernanceReviewDecision::new(
        &routing.policy,
        AdaptationReviewerIdentity::local_user("reviewer_b"),
        GovernanceReviewDecisionKind::Approve,
        "Second",
        "t2",
        vec!["no scoring mutation".into()],
        &proposal.provenance,
    )
    .unwrap();
    routing.enforce_decisions(&proposal, &[a, b]).unwrap();

    // High-risk without conditions fails.
    let bare = GovernanceReviewDecision::new(
        &routing.policy,
        AdaptationReviewerIdentity::local_user("reviewer_a"),
        GovernanceReviewDecisionKind::Approve,
        "Bare",
        "t3",
        vec![],
        &proposal.provenance,
    )
    .unwrap();
    let bare2 = GovernanceReviewDecision::new(
        &routing.policy,
        AdaptationReviewerIdentity::local_user("reviewer_b"),
        GovernanceReviewDecisionKind::Approve,
        "Bare2",
        "t4",
        vec![],
        &proposal.provenance,
    )
    .unwrap();
    assert!(routing.enforce_decisions(&proposal, &[bare, bare2]).is_err());
}

/// CASE 3 — Risk metadata cannot mutate cognition.
#[test]
fn case3_risk_metadata_cannot_mutate_cognition() {
    let proposal = approved_proposal("attention_weight_scoring");
    let risk = GovernanceRisk::classify_from_proposal(&proposal);
    assert_eq!(
        risk.impact_classification,
        GovernanceImpactClass::CognitionScoringMutation
    );
    assert!(!risk.may_mutate_cognition());
    assert!(!risk.may_silently_change_scoring());
    assert!(risk.enforce_not_cognition_mutation().is_err());
    assert!(GovernanceRisk::attempt_mutate_cognition().is_err());
    assert!(GovernanceReviewRouting::route_change(&proposal, None).is_err());

    let presentation = approved_proposal("experience_presentation");
    let routing = GovernanceReviewRouting::route_change(&presentation, None).unwrap();
    assert!(!routing.may_mutate_cognition());
    assert_eq!(routing.provenance, presentation.provenance);
}

/// CASE 4 — Permission Gateway remains separate.
#[test]
fn case4_permission_gateway_remains_separate() {
    let proposal = approved_proposal("behaviour_workflow");
    let routing = GovernanceReviewRouting::route_change(&proposal, None).unwrap();
    assert!(!routing.risk.may_bypass_permission_gateway());
    assert!(!routing.may_bypass_permission_gateway());
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
