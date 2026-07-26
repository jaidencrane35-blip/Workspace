//! Experience translation resolver (Sprint 130–133).
//!
//! Unified boundary: structured `AttentionReason` / `DecisionReason` → `DisplayReason`.
//! Wording comes from the canonical catalog only.

use workspace_domain::{
    AttentionReason, DecisionReason, DisplayImportance, DisplayReason,
};

use super::explanation_catalog::{
    fallback_title_for_signal, lookup_explanation_key, unknown_description,
};

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
        description: unknown_description(&explanation_key, &signal, &source, weight),
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

/// Decision Engine reasons: Attention translation when present; else Decision summary path.
pub(crate) fn resolve_decision_reason(reason: &DecisionReason) -> DisplayReason {
    if let Some(attention) = &reason.attention_reason {
        return resolve_attention_reason(attention);
    }
    DisplayReason {
        title: reason.summary.clone(),
        description: reason.kind.clone(),
        importance: DisplayImportance::Medium,
        explanation_key: format!("decision.{}", reason.kind),
        signal: reason.kind.clone(),
        source: "decision_engine".into(),
        weight: 0,
        known: true,
    }
}

pub(crate) fn resolve_decision_reasons(reasons: &[DecisionReason]) -> Vec<DisplayReason> {
    reasons.iter().map(resolve_decision_reason).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::explanation_catalog::catalog_version;
    use workspace_domain::{AttentionSignal, AttentionSourceType, DecisionReason};

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
        assert_eq!(catalog_version(), 2);
    }

    #[test]
    fn decision_native_reason_uses_experience_summary_path() {
        let reason = DecisionReason {
            kind: "goal_alignment".into(),
            summary: "Active goal still needs progress".into(),
            evidence_ref: None,
            attention_reason: None,
        };
        let display = resolve_decision_reason(&reason);
        assert!(display.known);
        assert_eq!(display.title, "Active goal still needs progress");
        assert_eq!(display.description, "goal_alignment");
        assert_eq!(display.explanation_key, "decision.goal_alignment");
    }

    #[test]
    fn decision_attention_backed_reason_delegates_to_catalog() {
        let reason = DecisionReason {
            kind: "attention".into(),
            summary: "ignored when attention present".into(),
            evidence_ref: None,
            attention_reason: Some(reason(
                "decision.base.outstanding",
                AttentionSignal::OutstandingDecision,
                48,
            )),
        };
        let display = resolve_decision_reason(&reason);
        assert_eq!(display.title, "Outstanding decision needs attention");
        assert_eq!(display.explanation_key, "decision.base.outstanding");
    }
}
