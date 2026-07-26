//! Sprint 138 — Recommendation identity and lifecycle contract tests.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use workspace_domain::{
    ActionProposal, AttentionReason, AttentionSignal, AttentionSourceType, RecommendationConfidence,
    RecommendationEvidence, RecommendationFamily, RecommendationGovernanceRecord,
    RecommendationIdentity, RecommendationItem, RecommendationKind, RecommendationLifecycleState,
    RecommendationProvenance,
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
        id: "recommendation:complete_task:task-1".into(),
        kind: RecommendationKind::CompleteTask,
        title: "Complete open task".into(),
        reason: "Task Graph lists ready work".into(),
        evidence: vec![RecommendationEvidence {
            id: "ev-task".into(),
            source_model: "task_graph".into(),
            source_ref: "task:1".into(),
            summary: "Open high-priority task".into(),
        }],
        impact: "Advances progress".into(),
        confidence: RecommendationConfidence::Medium,
        related_attention_id: Some("attention:task:1".into()),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::TaskGraph,
            AttentionSignal::InProgressTask,
            25,
            "task.base.in_progress",
        )],
        related_task_id: Some("task:1".into()),
        related_purpose_label: None,
        related_decision_id: Some("engine_decision:attention:task:1".into()),
        authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

/// CASE 1 — Identity preserves native namespace and cross-references.
#[test]
fn case1_recommendation_identity_preserves_native_ids() {
    let item = sample_item();
    let identity = RecommendationIdentity::from_recommendation_item(&item)
        .with_decision_ref(item.related_decision_id.clone().unwrap());
    assert_eq!(identity.native_id, item.id);
    assert!(identity
        .native_id
        .starts_with(RecommendationFamily::RecommendationEngine.native_id_prefix()));
    assert_eq!(identity.family, RecommendationFamily::RecommendationEngine);
    assert_eq!(identity.source_domain, "task_graph");
    assert_eq!(
        identity.decision_ref.as_deref(),
        Some("engine_decision:attention:task:1")
    );
    // Distinct namespaces remain distinct.
    assert!(!identity.native_id.starts_with("engine_decision:"));
    assert!(!identity.native_id.starts_with("rec-attention-"));
    assert!(!identity.native_id.starts_with("action_proposal:"));
}

/// CASE 2 — Lifecycle transitions enforce the contract.
#[test]
fn case2_lifecycle_transitions() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(
            RecommendationLifecycleState::Presented,
            "t2",
            Some("local_user".into()),
        )
        .unwrap();
    assert_eq!(
        record.lifecycle.state,
        RecommendationLifecycleState::Presented
    );
    assert_eq!(record.lifecycle.presented_at.as_deref(), Some("t2"));

    let invalid = record.transition(RecommendationLifecycleState::Created, "t3", None);
    assert!(invalid.is_err());
}

/// CASE 3 — Invalid Accepted → Available is rejected.
#[test]
fn case3_invalid_transition_rejection() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    let err = record
        .transition(RecommendationLifecycleState::Available, "t4", None)
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("accepted"));
    assert!(msg.contains("available"));
}

/// CASE 4 — Provenance immutability across lifecycle.
#[test]
fn case4_provenance_immutability() {
    let item = sample_item();
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
    let before = record.provenance.clone();
    let reasons_before = record.provenance.reasoning_origins.clone();
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Rejected, "t3", None)
        .unwrap();
    assert_eq!(record.provenance, before);
    assert_eq!(record.provenance.reasoning_origins, reasons_before);
}

/// CASE 5 — Acceptance does not execute or grant authority.
#[test]
fn case5_acceptance_does_not_execute() {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&sample_item(), "t0");
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", Some("local_user".into())).unwrap();
    assert_eq!(record.authority_effect, "none");
    assert_eq!(
        record.lifecycle.state,
        RecommendationLifecycleState::Accepted
    );
    assert!(ActionProposal::attempt_execute().is_err());
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
}

/// CASE 6 — Expiry does not alter reasoning provenance.
#[test]
fn case6_expiry_does_not_alter_reasoning() {
    let item = sample_item();
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
    let keys = record.provenance.explanation_keys.clone();
    let origins = record.provenance.reasoning_origins.clone();
    record
        .transition(RecommendationLifecycleState::Expired, "t1", None)
        .unwrap();
    assert_eq!(record.provenance.explanation_keys, keys);
    assert_eq!(record.provenance.reasoning_origins, origins);
    assert_eq!(
        record.lifecycle.state,
        RecommendationLifecycleState::Expired
    );
}

/// CASE 7 — ActionProposal links identity without collapsing namespaces.
#[test]
fn case7_action_proposal_identity_link() {
    let item = sample_item();
    let proposal = ActionProposal::from_recommendation_item("ws-1", &item);
    assert!(proposal.id.starts_with("action_proposal:"));
    assert_eq!(proposal.recommendation_ref, item.id);
    assert_eq!(proposal.identity.native_id, item.id);
    assert_eq!(
        proposal.identity.action_proposal_ref.as_deref(),
        Some(proposal.id.as_str())
    );
    assert_eq!(proposal.authority_effect, "none");
    let _ = RecommendationProvenance::from_recommendation_item(&item);
}
