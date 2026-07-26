//! Experience translation resolver (Sprint 130–135).
//!
//! Unified boundary: structured `AttentionReason` / `DecisionReason` → `DisplayReason`.
//! Optional traces answer: "What cognition produced this displayed explanation?"
//! Wording comes from the canonical catalog only. Traces are developer diagnostics —
//! never mutate Domain and never required for normal UI rendering.

use workspace_domain::{
    AttentionReason, DecisionReason, DisplayImportance, DisplayReason, ExperienceResolverPath,
    ExperienceResolverPathKind, ExperienceTranslationTrace,
};

use super::explanation_catalog::{
    fallback_title_for_signal, lookup_catalog_match, unknown_description, LookupTier,
};

fn path_kind_from_tier(tier: LookupTier) -> ExperienceResolverPathKind {
    match tier {
        LookupTier::Exact => ExperienceResolverPathKind::Exact,
        LookupTier::PrefixSuffix => ExperienceResolverPathKind::PrefixSuffix,
        LookupTier::PrefixPattern => ExperienceResolverPathKind::PrefixPattern,
        LookupTier::PrefixFallback => ExperienceResolverPathKind::PrefixFallback,
    }
}

/// Resolve one Attention reason into display wording.
pub(crate) fn resolve_attention_reason(reason: &AttentionReason) -> DisplayReason {
    resolve_attention_reason_traced(reason, None).display
}

/// Resolve one Attention reason and return a developer translation trace.
pub(crate) fn resolve_attention_reason_traced(
    reason: &AttentionReason,
    rendering_surface: Option<&str>,
) -> ExperienceTranslationTrace {
    let importance = DisplayImportance::from_weight(reason.weight);
    let signal = reason.signal.as_str().to_string();
    let source = reason.source.as_str().to_string();
    let explanation_key = reason.explanation_key.clone();
    let weight = reason.weight;

    let (display, resolver_path) = if let Some(matched) = lookup_catalog_match(&explanation_key) {
        (
            DisplayReason {
                title: matched.title.to_string(),
                description: matched.description.to_string(),
                importance,
                explanation_key: explanation_key.clone(),
                signal: signal.clone(),
                source: source.clone(),
                weight,
                known: true,
            },
            ExperienceResolverPath::new(path_kind_from_tier(matched.tier), matched.match_key),
        )
    } else {
        (
            DisplayReason {
                title: fallback_title_for_signal(&signal),
                description: unknown_description(&explanation_key, &signal, &source, weight),
                importance,
                explanation_key: explanation_key.clone(),
                signal: signal.clone(),
                source: source.clone(),
                weight,
                known: false,
            },
            ExperienceResolverPath::new(
                ExperienceResolverPathKind::Unknown,
                format!("unknown:{explanation_key}"),
            ),
        )
    };

    ExperienceTranslationTrace {
        source_reasoning_type: "attention_reason".into(),
        source_identifier: explanation_key.clone(),
        explanation_key,
        resolver_path,
        display,
        rendering_surface: rendering_surface.map(str::to_string),
    }
}

/// Resolve many reasons in input order — no reordering, no filtering.
pub(crate) fn resolve_attention_reasons(reasons: &[AttentionReason]) -> Vec<DisplayReason> {
    reasons.iter().map(resolve_attention_reason).collect()
}

pub(crate) fn resolve_attention_reasons_traced(
    reasons: &[AttentionReason],
    rendering_surface: Option<&str>,
) -> Vec<ExperienceTranslationTrace> {
    reasons
        .iter()
        .map(|r| resolve_attention_reason_traced(r, rendering_surface))
        .collect()
}

/// Decision Engine reasons: Attention translation when present; else Decision summary path.
pub(crate) fn resolve_decision_reason(reason: &DecisionReason) -> DisplayReason {
    resolve_decision_reason_traced(reason, None).display
}

pub(crate) fn resolve_decision_reason_traced(
    reason: &DecisionReason,
    rendering_surface: Option<&str>,
) -> ExperienceTranslationTrace {
    if let Some(attention) = &reason.attention_reason {
        let mut trace = resolve_attention_reason_traced(attention, rendering_surface);
        trace.source_reasoning_type = "decision_reason".into();
        trace.source_identifier = format!(
            "decision.{}→{}",
            reason.kind, attention.explanation_key
        );
        return trace;
    }

    let explanation_key = format!("decision.{}", reason.kind);
    let display = DisplayReason {
        title: reason.summary.clone(),
        description: reason.kind.clone(),
        importance: DisplayImportance::Medium,
        explanation_key: explanation_key.clone(),
        signal: reason.kind.clone(),
        source: "decision_engine".into(),
        weight: 0,
        known: true,
    };

    ExperienceTranslationTrace {
        source_reasoning_type: "decision_reason".into(),
        source_identifier: reason.kind.clone(),
        explanation_key: explanation_key.clone(),
        resolver_path: ExperienceResolverPath::new(
            ExperienceResolverPathKind::DecisionNative,
            format!("decision_native:{explanation_key}"),
        ),
        display,
        rendering_surface: rendering_surface.map(str::to_string),
    }
}

pub(crate) fn resolve_decision_reasons(reasons: &[DecisionReason]) -> Vec<DisplayReason> {
    reasons.iter().map(resolve_decision_reason).collect()
}

pub(crate) fn resolve_decision_reasons_traced(
    reasons: &[DecisionReason],
    rendering_surface: Option<&str>,
) -> Vec<ExperienceTranslationTrace> {
    reasons
        .iter()
        .map(|r| resolve_decision_reason_traced(r, rendering_surface))
        .collect()
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

    #[test]
    fn traced_exact_path_records_match_key() {
        let trace = resolve_attention_reason_traced(
            &reason(
                "decision.base.outstanding",
                AttentionSignal::OutstandingDecision,
                55,
            ),
            Some("test"),
        );
        assert_eq!(
            trace.resolver_path.kind,
            ExperienceResolverPathKind::Exact
        );
        assert_eq!(
            trace.resolver_path.match_key,
            "exact:decision.base.outstanding"
        );
        assert_eq!(trace.rendering_surface.as_deref(), Some("test"));
        assert_eq!(trace.display, resolve_attention_reason(&reason(
            "decision.base.outstanding",
            AttentionSignal::OutstandingDecision,
            55,
        )));
    }

    #[test]
    fn traced_prefix_pattern_records_pattern_key() {
        let trace = resolve_attention_reason_traced(
            &reason(
                "purpose.obstacle.composition:missing_application",
                AttentionSignal::PurposeObstacle,
                36,
            ),
            None,
        );
        assert_eq!(
            trace.resolver_path.kind,
            ExperienceResolverPathKind::PrefixPattern
        );
        assert_eq!(
            trace.resolver_path.match_key,
            "prefix_pattern:purpose.obstacle.composition:*"
        );
        assert!(trace.display.known);
    }

    #[test]
    fn traced_unknown_path_is_safe() {
        let input = reason("future.trace.unknown", AttentionSignal::BlockedTask, 11);
        let before = input.explanation_key.clone();
        let trace = resolve_attention_reason_traced(&input, Some("operator"));
        assert_eq!(input.explanation_key, before);
        assert_eq!(
            trace.resolver_path.kind,
            ExperienceResolverPathKind::Unknown
        );
        assert_eq!(
            trace.resolver_path.match_key,
            "unknown:future.trace.unknown"
        );
        assert!(!trace.display.known);
        assert!(trace.display.description.contains("future.trace.unknown"));
    }
}
