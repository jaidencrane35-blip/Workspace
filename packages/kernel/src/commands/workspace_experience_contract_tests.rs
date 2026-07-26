//! Experience explanation catalog contract tests (Sprint 131).

use crate::services::explanation_catalog::{
    catalog_version, exact_key_count, fallback_title_for_signal, lookup_explanation_key,
};
use crate::services::explanation_resolver::{resolve_attention_reason, resolve_attention_reasons};
use workspace_domain::{AttentionReason, AttentionSignal, AttentionSourceType};

fn reason(key: &str, signal: AttentionSignal, weight: i32) -> AttentionReason {
    AttentionReason::new(AttentionSourceType::TaskGraph, signal, weight, key)
}

/// CASE 11 — Canonical catalog resolves the same contract as the Experience resolver.
#[test]
fn case11_catalog_and_resolver_agree_on_known_keys() {
    assert_eq!(catalog_version(), 1);
    assert!(exact_key_count() >= 16);

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
    assert!(display.description.contains("future.contract.unknown"));
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
