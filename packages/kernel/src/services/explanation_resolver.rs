//! Experience explanation resolver (Sprint 130).
//!
//! Translates stable `AttentionReason.explanation_key` values into user-facing
//! `DisplayReason` wording. Owns presentation only — never scores, ranks, or
//! re-infers. Domain keeps `signal` / `source` / `weight` / `explanation_key`;
//! this module owns human wording.

use workspace_domain::{AttentionReason, DisplayImportance, DisplayReason};

/// Resolve one Attention reason into display wording.
///
/// Known keys get a curated title + description. Unknown keys fall back safely
/// without being hidden — the unresolved key stays visible in the description.
pub(crate) fn resolve_attention_reason(reason: &AttentionReason) -> DisplayReason {
    let importance = DisplayImportance::from_weight(reason.weight);
    let signal = reason.signal.as_str().to_string();
    let source = reason.source.as_str().to_string();
    let explanation_key = reason.explanation_key.clone();
    let weight = reason.weight;

    if let Some((title, description)) = lookup_key(&explanation_key) {
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

    // Safe fallback — never silent. Preserve identity; use signal for a readable title.
    DisplayReason {
        title: fallback_title(&signal),
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

fn fallback_title(signal: &str) -> String {
    match signal {
        "outstanding_decision" => "Outstanding decision needs attention".into(),
        "blocked_action" => "Blocked action needs attention".into(),
        "blocked_task" => "Blocked task needs attention".into(),
        "waiting_task" => "Waiting task needs attention".into(),
        "in_progress_task" => "In-progress task needs attention".into(),
        "interrupted_work" => "Interrupted work needs attention".into(),
        "resumable_work" => "Resumable work needs attention".into(),
        "current_focus" => "Current focus needs attention".into(),
        "dormant_work" => "Dormant work needs attention".into(),
        "commitment_pending" => "Pending commitment needs attention".into(),
        "environment_disconnect" => "Environment disconnect needs attention".into(),
        "missing_application" => "Missing application needs attention".into(),
        "composition_gap" => "Composition gap needs attention".into(),
        "high_priority_intent" => "High-priority intent needs attention".into(),
        "purpose_obstacle" => "Purpose obstacle needs attention".into(),
        "purpose_outcome" => "Purpose outcome needs attention".into(),
        "evolution_insight" => "Evolution insight needs attention".into(),
        "activity_progress" => "Recent activity needs attention".into(),
        "recommendation_candidate" => "Recommendation needs attention".into(),
        "pattern_observation" => "Pattern observation needs attention".into(),
        other => format!("Attention signal '{other}' needs attention"),
    }
}

fn lookup_key(key: &str) -> Option<(&'static str, &'static str)> {
    // Exact matches first.
    if let Some(pair) = exact_catalog(key) {
        return Some(pair);
    }
    // Prefixed / parameterized keys emitted by Attention.
    if let Some(rest) = key.strip_prefix("task.base.") {
        return Some(match rest {
            "blocked" => (
                "Blocked task needs attention",
                "because work is currently waiting on completion",
            ),
            "waiting" => (
                "Waiting task needs attention",
                "because this task is waiting on a dependency",
            ),
            "in_progress" => (
                "In-progress task needs attention",
                "because active work is underway and still open",
            ),
            "ready" => (
                "Ready task needs attention",
                "because open work is ready to continue",
            ),
            _ => (
                "Open task needs attention",
                "because Task Graph still lists this work as open",
            ),
        });
    }
    if let Some(rest) = key.strip_prefix("task.priority.") {
        return Some(match rest {
            "critical" => (
                "Critical priority raises focus",
                "because this task is marked critical",
            ),
            "high" => (
                "High priority raises focus",
                "because this task is marked high priority",
            ),
            "normal" => (
                "Normal priority contributes to focus",
                "because this task carries a normal priority band",
            ),
            "low" => (
                "Low priority still contributes",
                "because open low-priority work remains on the graph",
            ),
            _ => (
                "Task priority contributes to focus",
                "because Task Graph priority influenced this ranking",
            ),
        });
    }
    if let Some(rest) = key.strip_prefix("purpose.obstacle.") {
        return Some(match rest {
            "blocked_task" => (
                "Purpose blocked by a task",
                "because a blocked Task Graph node sits on the path to Purpose",
            ),
            "blocked_work" => (
                "Purpose blocked by open work",
                "because blocked Continuity work is stalling Purpose progress",
            ),
            "interrupted_work" => (
                "Purpose interrupted",
                "because interrupted work is pulling focus away from Purpose",
            ),
            "outstanding_decisions" => (
                "Purpose waiting on decisions",
                "because outstanding decisions gate Purpose progress",
            ),
            other if other.starts_with("composition:") => (
                "Purpose blocked by composition",
                "because a Composition gap is obstructing Purpose progress",
            ),
            _ => (
                "Purpose obstacle needs attention",
                "because Purpose reports an obstacle on the current path",
            ),
        });
    }
    if let Some(rest) = key.strip_prefix("composition.gap.") {
        return Some(match rest {
            "disconnected_work" => (
                "Composition is disconnected from work",
                "because the working environment does not match active work",
            ),
            "missing_application" => (
                "Composition is missing an application",
                "because a required application is not present in the composition",
            ),
            _ => (
                "Composition gap needs attention",
                "because Composition reports a membership gap",
            ),
        });
    }
    if let Some(rest) = key.strip_prefix("environment.gap.") {
        return Some(match rest {
            "disconnected_work" => (
                "Desktop is disconnected from work",
                "because open windows do not align with active work",
            ),
            "missing_application" => (
                "Required application is missing",
                "because Environment expects an application that is not present",
            ),
            _ => (
                "Environment gap needs attention",
                "because Environment reports a desktop–work gap",
            ),
        });
    }
    None
}

fn exact_catalog(key: &str) -> Option<(&'static str, &'static str)> {
    Some(match key {
        "decision.base.blocker" => (
            "Blocked decision needs attention",
            "because a blocked Decision Queue item is stopping progress",
        ),
        "decision.base.outstanding" => (
            "Outstanding decision needs attention",
            "because a pending Decision Queue item still needs a human choice",
        ),
        "decision.priority.critical" => (
            "Critical decision raises focus",
            "because this Decision Queue item is marked critical",
        ),
        "decision.priority.high" => (
            "High-priority decision raises focus",
            "because this Decision Queue item is marked high priority",
        ),
        "decision.priority.normal" => (
            "Decision contributes to focus",
            "because this Decision Queue item carries normal priority",
        ),
        "decision.deferred" => (
            "Deferred decision still matters",
            "because a deferred Decision Queue item remains unresolved",
        ),
        "continuity.interrupted" => (
            "Interrupted work needs attention",
            "because Continuity shows work that was left unfinished",
        ),
        "continuity.resumable" => (
            "Resumable work is ready",
            "because Continuity can pick up where you left off",
        ),
        "continuity.commitment" => (
            "Pending commitment needs attention",
            "because an automation commitment is still awaiting resolution",
        ),
        "continuity.dormant" => (
            "Dormant work needs attention",
            "because Continuity shows work that has gone quiet",
        ),
        "continuity.focus" => (
            "Current focus needs attention",
            "because Continuity identifies this as the active focus",
        ),
        "activity.progress" => (
            "Recent progress is worth noticing",
            "because Activity Graph recorded related work recently",
        ),
        "purpose.outcome" => (
            "Purpose progress is visible",
            "because Purpose reports meaningful progress toward the goal",
        ),
        "evolution.insight" => (
            "Workspace evolution needs attention",
            "because Evolution surfaced how work recently changed",
        ),
        "recommendation.candidate" => (
            "A next-step suggestion is available",
            "because the Recommendation Engine proposed a candidate action",
        ),
        "pattern.observation" => (
            "A work pattern was observed",
            "because Pattern Model noticed a recurring workspace signal",
        ),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
        // Order preserved — Experience must not re-rank.
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
        // Input order kept even when later weight is higher — no re-sort.
        assert!(displayed[1].weight > displayed[0].weight);
        assert_eq!(displayed[0].explanation_key, "decision.priority.high");
    }
}
