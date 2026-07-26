//! Sprint 137 — Recommendation provenance and ActionProposal architecture guards.
//!
//! Verifies provenance preservation, recommendation non-execution, and Experience
//! translation-only behavior. Does not introduce automation or execution paths.

use crate::commands::handler::CommandHandler;
use crate::error::KernelError;
use crate::services::explanation_resolver::resolve_attention_reason_traced;
use crate::services::WorkspaceRecommendationEngineService;
use workspace_domain::{
    ActionProposal, AttentionReason, AttentionSignal, AttentionSourceType, DecisionReason,
    RecommendationConfidence, RecommendationEvidence, RecommendationFamily, RecommendationItem,
    RecommendationKind, RecommendationProvenance,
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

fn sample_recommendation() -> RecommendationItem {
    RecommendationItem {
        id: "recommendation:attention:blocked".into(),
        kind: RecommendationKind::ResolveBlocker,
        title: "Resolve blocker".into(),
        reason: "Attention flags blocked work".into(),
        evidence: vec![RecommendationEvidence {
            id: "ev1".into(),
            source_model: "attention".into(),
            source_ref: "attention:task:1".into(),
            summary: "Blocked task scored high".into(),
        }],
        impact: "Unblocks progress".into(),
        confidence: RecommendationConfidence::High,
        related_attention_id: Some("attention:task:1".into()),
        attention_reasons: vec![AttentionReason::new(
            AttentionSourceType::TaskGraph,
            AttentionSignal::BlockedTask,
            40,
            "task.base.blocked",
        )],
        related_task_id: Some("task:1".into()),
        related_purpose_label: None,
        related_decision_id: None,
        lifecycle_state: None,
        lifecycle_presented_at: None,
        lifecycle_resolved_at: None,
        lifecycle_resolution_type: None,
        explanation: None,
                outcome: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

/// CASE 1 — Provenance preserves evidence, reasoning, and explanation keys.
#[test]
fn case1_provenance_preservation() {
    let item = sample_recommendation();
    let provenance = RecommendationProvenance::from_recommendation_item(&item);
    assert_eq!(
        provenance.family,
        RecommendationFamily::RecommendationEngine
    );
    assert_eq!(provenance.source_evidence, item.evidence);
    assert_eq!(provenance.reasoning_origins, item.attention_reasons);
    assert_eq!(provenance.explanation_keys, vec!["task.base.blocked"]);
    assert_eq!(
        provenance.related_attention_id.as_deref(),
        Some("attention:task:1")
    );
}

/// CASE 2 — Recommendation payload remains immutable across provenance / proposal build.
#[test]
fn case2_recommendation_immutability_for_payload_fields() {
    let item = sample_recommendation();
    let before = item.clone();
    let proposal = ActionProposal::from_recommendation_item("ws-test", &item)
        .with_experience_trace_match_keys(vec!["prefix_suffix:task.base.blocked".into()]);
    assert_eq!(item, before);
    assert_eq!(proposal.recommendation_ref, before.id);
    assert_eq!(proposal.authority_effect, "none");
    assert_eq!(
        proposal.provenance.experience_trace_match_keys,
        vec!["prefix_suffix:task.base.blocked"]
    );
}

/// CASE 3 — Recommendations / Decision / ActionProposal cannot bypass permission.
#[test]
fn case3_no_recommendation_bypasses_permission() {
    assert_cannot_execute(
        "recommendation_engine",
        CommandHandler::workspace_recommendation_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "decision_engine",
        CommandHandler::decision_engine_attempt_execute(),
    );
    assert_cannot_execute(
        "adaptation",
        CommandHandler::workspace_adaptation_attempt_execute(),
    );
    assert!(
        ActionProposal::attempt_execute().is_err(),
        "ActionProposal must hard-fail attempt_execute"
    );
    // Domain service guard remains in place.
    assert!(WorkspaceRecommendationEngineService::attempt_execute().is_err());
}

/// CASE 4 — Experience remains translation-only; can attach traces to provenance.
#[test]
fn case4_experience_translation_links_to_provenance() {
    let reason = AttentionReason::new(
        AttentionSourceType::TaskGraph,
        AttentionSignal::BlockedTask,
        40,
        "task.base.blocked",
    );
    let before = reason.clone();
    let trace = resolve_attention_reason_traced(&reason, Some("provenance_test"));
    assert_eq!(reason, before);

    let item = sample_recommendation();
    let proposal = ActionProposal::from_recommendation_item("ws-test", &item)
        .with_experience_trace_match_keys(vec![trace.resolver_path.match_key.clone()]);
    assert_eq!(proposal.authority_effect, "none");
    assert_eq!(
        proposal.provenance.experience_trace_match_keys[0],
        trace.resolver_path.match_key
    );
    assert_eq!(trace.display.explanation_key, "task.base.blocked");
    assert_cannot_execute(
        "experience",
        CommandHandler::workspace_experience_attempt_execute(),
    );
}

/// CASE 5 — DecisionReason Attention adapter preserves structured provenance atom.
#[test]
fn case5_decision_reason_attention_adapter_preserves_origin() {
    let attention = AttentionReason::new(
        AttentionSourceType::DecisionQueue,
        AttentionSignal::OutstandingDecision,
        55,
        "decision.base.outstanding",
    );
    let decision = DecisionReason {
        kind: "attention".into(),
        summary: "Outstanding decision needs attention".into(),
        evidence_ref: Some("attention:decision:1".into()),
        attention_reason: Some(attention.clone()),
    };
    assert_eq!(decision.attention_reason.as_ref(), Some(&attention));
    assert_eq!(
        decision
            .attention_reason
            .as_ref()
            .map(|r| r.explanation_key.as_str()),
        Some("decision.base.outstanding")
    );
}
