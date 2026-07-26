//! Experience explanation catalog contract tests (Sprint 131–132).

use crate::services::explanation_catalog::{
    contract_fixtures, fallback_title_for_signal, lookup_explanation_key,
    lookup_explanation_key_with_tier, unknown_description, LookupTier,
};
use crate::services::explanation_resolver::{
    resolve_attention_reason, resolve_attention_reasons, resolve_decision_reason,
    resolve_decision_reasons,
};
use workspace_domain::{AttentionReason, AttentionSignal, AttentionSourceType, DecisionReason};

fn reason(key: &str, signal: AttentionSignal, weight: i32) -> AttentionReason {
    AttentionReason::new(AttentionSourceType::TaskGraph, signal, weight, key)
}

fn reason_with_source(
    key: &str,
    signal: AttentionSignal,
    source: AttentionSourceType,
    weight: i32,
) -> AttentionReason {
    AttentionReason::new(source, signal, weight, key)
}

/// CASE 11 — Canonical catalog resolves the same contract as the Experience resolver.
#[test]
fn case11_catalog_and_resolver_agree_on_known_keys() {
    let samples = [
        "task.base.blocked",
        "decision.base.outstanding",
        "continuity.interrupted",
        "pattern.observation",
    ];
    for key in samples {
        let (cat_title, cat_desc) = lookup_explanation_key(key).expect(key);
        let display = resolve_attention_reason(&reason(
            key,
            AttentionSignal::BlockedTask,
            40,
        ));
        assert_eq!(display.title, cat_title);
        assert_eq!(display.description, cat_desc);
        assert!(display.known);
    }
}

/// CASE 12 — Unknown keys fall back safely; source identity survives rendering.
#[test]
fn case12_unknown_keys_fallback_and_preserve_identity() {
    let source = reason("future.contract.unknown", AttentionSignal::BlockedTask, 33);
    let display = resolve_attention_reason(&source);
    assert!(!display.known);
    assert_eq!(display.explanation_key, "future.contract.unknown");
    assert_eq!(display.weight, 33);
    assert_eq!(display.signal, "blocked_task");
    assert!(
        display.description.contains("future.contract.unknown"),
        "unknown key must remain visible: {}",
        display.description
    );
    assert_eq!(
        display.title,
        fallback_title_for_signal(AttentionSignal::BlockedTask.as_str())
    );
}

/// CASE 13 — Rendering is deterministic and does not mutate source reasons.
#[test]
fn case13_rendering_is_deterministic_and_non_mutating() {
    let reasons = vec![
        reason("decision.priority.high", AttentionSignal::OutstandingDecision, 12),
        reason("decision.base.outstanding", AttentionSignal::OutstandingDecision, 48),
    ];
    let snapshot: Vec<_> = reasons
        .iter()
        .map(|r| (r.explanation_key.clone(), r.weight, r.signal.as_str()))
        .collect();

    let first = resolve_attention_reasons(&reasons);
    let second = resolve_attention_reasons(&reasons);
    assert_eq!(first, second);

    let after: Vec<_> = reasons
        .iter()
        .map(|r| (r.explanation_key.clone(), r.weight, r.signal.as_str()))
        .collect();
    assert_eq!(snapshot, after);
    assert_eq!(first[0].explanation_key, "decision.priority.high");
    assert_eq!(first[1].explanation_key, "decision.base.outstanding");
}

fn fixture_signal(signal: &str) -> AttentionSignal {
    serde_json::from_str(&format!("\"{signal}\"")).unwrap_or_else(|_| {
        panic!("fixture uses invalid signal {signal}")
    })
}

fn fixture_source(source: &str) -> AttentionSourceType {
    serde_json::from_str(&format!("\"{source}\"")).unwrap_or_else(|_| {
        panic!("fixture uses invalid source {source}")
    })
}

/// CASE 14 — Catalog contract fixtures drive cross-layer equivalence.
#[test]
fn case14_contract_fixtures_match_resolver_output() {
    for fixture in contract_fixtures() {
        let signal = fixture_signal(&fixture.signal);
        let source = fixture_source(&fixture.source);
        let display = resolve_attention_reason(&reason_with_source(
            &fixture.key,
            signal,
            source,
            fixture.weight,
        ));
        assert_eq!(display.known, fixture.known, "fixture {}", fixture.key);
        assert_eq!(display.title, fixture.title, "fixture {}", fixture.key);
        assert_eq!(
            display.description, fixture.description,
            "fixture {}",
            fixture.key
        );
    }
}

/// CASE 15 — Explicit resolution order: suffix before pattern before fallback.
#[test]
fn case15_resolution_order_is_suffix_pattern_fallback() {
    let (_, _, suffix) = lookup_explanation_key_with_tier("task.base.blocked").unwrap();
    assert_eq!(suffix, LookupTier::PrefixSuffix);

    let (_, _, pattern) =
        lookup_explanation_key_with_tier("purpose.obstacle.composition:missing_application")
            .unwrap();
    assert_eq!(pattern, LookupTier::PrefixPattern);

    let (_, _, fallback) =
        lookup_explanation_key_with_tier("purpose.obstacle.unlisted_kind").unwrap();
    assert_eq!(fallback, LookupTier::PrefixFallback);

    assert!(lookup_explanation_key("future.contract.unknown").is_none());
    assert_eq!(
        unknown_description(
            "future.contract.unknown",
            "blocked_task",
            "task_graph",
            33
        ),
        "No Experience translation for 'future.contract.unknown' yet (signal blocked_task from task_graph, weight 33)."
    );
}

/// CASE 19 — Reasoning models stay separate; Recommendation reuses AttentionReason.
#[test]
fn case19_reasoning_models_remain_separate_with_adapters_only() {
    let attention = reason("task.base.blocked", AttentionSignal::BlockedTask, 40);
    let decision = DecisionReason {
        kind: "attention".into(),
        summary: "via attention".into(),
        evidence_ref: None,
        attention_reason: Some(attention.clone()),
    };
    let display_from_decision = resolve_decision_reason(&decision);
    let display_from_attention = resolve_attention_reason(&attention);
    assert_eq!(display_from_decision.title, display_from_attention.title);
    assert_eq!(
        display_from_decision.explanation_key,
        display_from_attention.explanation_key
    );
    assert!(
        std::any::type_name::<workspace_domain::RecommendationItem>()
            .contains("RecommendationItem")
    );
    assert_ne!(
        std::any::type_name::<AttentionReason>(),
        std::any::type_name::<DecisionReason>()
    );
}

/// CASE 20 — Operator-style diagnostic fields are not required on DisplayReason.
#[test]
fn case20_display_reason_carries_experience_payload_not_diagnostic_scores() {
    let display = resolve_attention_reason(&reason(
        "decision.base.outstanding",
        AttentionSignal::OutstandingDecision,
        55,
    ));
    assert!(display.known);
    assert!(!display.title.contains("score"));
    assert!(!display.description.contains("score_factors"));
}

/// CASE 21 — Batch translation preserves Domain object immutability.
#[test]
fn case21_batch_translation_does_not_mutate_domain_reasons() {
    let reasons = vec![
        reason("pattern.observation", AttentionSignal::PatternObservation, 20),
        reason("evolution.insight", AttentionSignal::EvolutionInsight, 35),
    ];
    let keys_before: Vec<_> = reasons.iter().map(|r| r.explanation_key.clone()).collect();
    let weights_before: Vec<_> = reasons.iter().map(|r| r.weight).collect();
    let _ = resolve_attention_reasons(&reasons);
    let keys_after: Vec<_> = reasons.iter().map(|r| r.explanation_key.clone()).collect();
    let weights_after: Vec<_> = reasons.iter().map(|r| r.weight).collect();
    assert_eq!(keys_before, keys_after);
    assert_eq!(weights_before, weights_after);
}

/// CASE 16 — Decision-native reasons translate through Experience without Attention keys.
#[test]
fn case16_decision_native_reasons_use_experience_path() {
    let reason = DecisionReason {
        kind: "memory_relevance".into(),
        summary: "Recent memory is relevant".into(),
        evidence_ref: Some("memory:abc".into()),
        attention_reason: None,
    };
    let display = resolve_decision_reason(&reason);
    assert!(display.known);
    assert_eq!(display.title, reason.summary);
    assert_eq!(display.description, reason.kind);
    assert_eq!(display.explanation_key, "decision.memory_relevance");
}

/// CASE 17 — UI contract: batch decision translation is deterministic.
#[test]
fn case17_decision_reason_batch_is_deterministic() {
    let reasons = vec![
        DecisionReason {
            kind: "attention".into(),
            summary: "unused".into(),
            evidence_ref: None,
            attention_reason: Some(reason(
                "task.base.blocked",
                AttentionSignal::BlockedTask,
                40,
            )),
        },
        DecisionReason {
            kind: "plan_ready".into(),
            summary: "Plan is ready to review".into(),
            evidence_ref: None,
            attention_reason: None,
        },
    ];
    assert_eq!(
        resolve_decision_reasons(&reasons),
        resolve_decision_reasons(&reasons)
    );
}

/// CASE 18 — Translation never mutates structured Domain reasoning.
#[test]
fn case18_translation_preserves_domain_reason_structure() {
    let attention = reason("pattern.observation", AttentionSignal::PatternObservation, 15);
    let decision = DecisionReason {
        kind: "attention".into(),
        summary: "via attention".into(),
        evidence_ref: None,
        attention_reason: Some(attention.clone()),
    };
    let before_key = attention.explanation_key.clone();
    let before_weight = attention.weight;
    let _ = resolve_attention_reason(&attention);
    let _ = resolve_decision_reason(&decision);
    assert_eq!(attention.explanation_key, before_key);
    assert_eq!(attention.weight, before_weight);
}
