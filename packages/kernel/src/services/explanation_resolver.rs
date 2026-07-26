//! Experience explanation resolver (Sprint 130–131).
//!
//! Translates stable `AttentionReason.explanation_key` values into user-facing
//! `DisplayReason` wording via the canonical catalog. Owns presentation only.

use workspace_domain::{AttentionReason, DisplayImportance, DisplayReason};

use super::explanation_catalog::{fallback_title_for_signal, lookup_explanation_key};

/// Resolve one Attention reason into display wording.
pub(crate) fn resolve_attention_reason(reason: &AttentionReason) -> DisplayReason {
    let importance = DisplayImportance::from_weight(reason.weight);
    let signal = reason.signal.as_str().to_string();
    let source = reason.source.as_str().to_string();
    let explanation_key = reason.explanation_key.clone();
    let weight = reason.weight;

    if let Some((title, description)) = lookup_explanation_key(&explanation_key) {
        return DisplayReason {
            title: title.to_string(),
            description: description.to_string(),
            importance,
            explanation_key,
            signal,
            source,
            weight,
            known: true,
        };
    }

    DisplayReason {
        title: fallback_title_for_signal(&signal),
        description: format!(
            "No Experience translation for '{explanation_key}' yet \
             (signal {signal} from {source}, weight {weight})."
        ),
        importance,
        explanation_key,
        signal,
        source,
        weight,
        known: false,
    }
}

/// Resolve many reasons in input order — no reordering, no filtering.
pub(crate) fn resolve_attention_reasons(reasons: &[AttentionReason]) -> Vec<DisplayReason> {
    reasons.iter().map(resolve_attention_reason).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::explanation_catalog::catalog_version;
    use workspace_domain::{AttentionSignal, AttentionSourceType};

    fn reason(key: &str, signal: AttentionSignal, weight: i32) -> AttentionReason {
        AttentionReason::new(AttentionSourceType::TaskGraph, signal, weight, key)
    }

    #[test]
    fn known_key_renders_expected_display_contract() {
        let display = resolve_attention_reason(&reason(
            "task.base.blocked",
            AttentionSignal::BlockedTask,
            40,
        ));
        assert!(display.known);
        assert_eq!(display.title, "Blocked task needs attention");
        assert_eq!(
            display.description,
            "because work is currently waiting on completion"
        );
        assert_eq!(display.importance, DisplayImportance::Medium);
        assert_eq!(display.explanation_key, "task.base.blocked");
        assert_eq!(display.signal, "blocked_task");
        assert_eq!(display.source, "task_graph");
        assert_eq!(display.weight, 40);
    }

    #[test]
    fn unknown_key_falls_back_safely_without_hiding() {
        let display = resolve_attention_reason(&reason(
            "future.signal.unknown",
            AttentionSignal::BlockedTask,
            40,
        ));
        assert!(!display.known);
        assert_eq!(display.title, "Blocked task needs attention");
        assert!(
            display.description.contains("future.signal.unknown"),
            "unknown key must remain visible: {}",
            display.description
        );
        assert_eq!(display.explanation_key, "future.signal.unknown");
        assert_eq!(display.weight, 40);
    }

    #[test]
    fn same_input_produces_deterministic_display_output() {
        let input = reason(
            "decision.base.outstanding",
            AttentionSignal::OutstandingDecision,
            55,
        );
        let a = resolve_attention_reason(&input);
        let b = resolve_attention_reason(&input);
        assert_eq!(a, b);
        assert_eq!(a.importance, DisplayImportance::High);
    }

    #[test]
    fn ui_projection_preserves_source_reason_identity() {
        let reasons = vec![
            reason(
                "continuity.interrupted",
                AttentionSignal::InterruptedWork,
                60,
            ),
            reason("activity.progress", AttentionSignal::ActivityProgress, 10),
        ];
        let displayed = resolve_attention_reasons(&reasons);
        assert_eq!(displayed.len(), reasons.len());
        for (source, display) in reasons.iter().zip(displayed.iter()) {
            assert_eq!(display.explanation_key, source.explanation_key);
            assert_eq!(display.signal, source.signal.as_str());
            assert_eq!(display.source, source.source.as_str());
            assert_eq!(display.weight, source.weight);
        }
        assert_eq!(displayed[0].explanation_key, "continuity.interrupted");
        assert_eq!(displayed[1].explanation_key, "activity.progress");
    }

    #[test]
    fn rendering_does_not_change_weights_or_order() {
        let reasons = vec![
            reason("decision.priority.high", AttentionSignal::OutstandingDecision, 12),
            reason("decision.base.outstanding", AttentionSignal::OutstandingDecision, 48),
        ];
        let before: Vec<_> = reasons
            .iter()
            .map(|r| (r.explanation_key.clone(), r.weight))
            .collect();
        let displayed = resolve_attention_reasons(&reasons);
        let after: Vec<_> = displayed
            .iter()
            .map(|d| (d.explanation_key.clone(), d.weight))
            .collect();
        assert_eq!(before, after);
        assert!(displayed[1].weight > displayed[0].weight);
        assert_eq!(displayed[0].explanation_key, "decision.priority.high");
    }

    #[test]
    fn catalog_version_is_loaded() {
        assert_eq!(catalog_version(), 1);
    }
}
