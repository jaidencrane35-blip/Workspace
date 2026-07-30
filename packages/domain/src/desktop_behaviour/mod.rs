//! Deterministic desktop behaviour projection from retained observation samples.
//!
//! Infers focus transitions, revisits, sample coverage gaps, and current observed
//! focus span from adjacent snapshot comparisons. Sample-based — not OS active time.
//! No AI, no Win32, no workflow invention.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::workspace_observation::WorkspaceObservationSnapshot;
use crate::workspace_observation_delta::{
    compare_observation_snapshots, ObservationWindowRef,
};

/// Maximum observation samples consumed by the behaviour timeline.
pub const DESKTOP_BEHAVIOUR_SAMPLE_LIMIT: usize = 50;

/// Gap between samples larger than this (seconds) is recorded as coverage loss.
pub const DESKTOP_BEHAVIOUR_GAP_SECONDS: i64 = 30 * 60;

/// One observed focus change between adjacent samples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopFocusTransition {
    pub at: String,
    pub from_pass_id: String,
    pub to_pass_id: String,
    pub previous: Option<ObservationWindowRef>,
    pub current: Option<ObservationWindowRef>,
}

/// How often a window became focused across the sample window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowRevisit {
    pub window: ObservationWindowRef,
    pub focus_count: i32,
    pub last_focused_at: String,
}

/// Interrupted observation coverage between retained samples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopCoverageGap {
    pub after_pass_id: String,
    pub before_pass_id: String,
    pub previous_captured_at: String,
    pub next_captured_at: String,
    pub gap_seconds: i64,
}

/// Completed observed focus span (sample-based, not OS active time).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopObservedFocusSpan {
    pub window: ObservationWindowRef,
    pub started_at: String,
    pub ended_at: String,
    pub sample_span_seconds: Option<i64>,
    pub sample_count: i32,
}

/// Bounded behavioural timeline projected onto WorkspaceState.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopBehaviourTimeline {
    pub sample_count: i32,
    pub coverage_started_at: Option<String>,
    pub coverage_ended_at: Option<String>,
    pub focus_transitions: Vec<DesktopFocusTransition>,
    pub window_revisits: Vec<DesktopWindowRevisit>,
    pub recent_focus_spans: Vec<DesktopObservedFocusSpan>,
    pub current_focus: Option<ObservationWindowRef>,
    pub current_focus_started_at: Option<String>,
    pub current_focus_sample_span_seconds: Option<i64>,
    pub coverage_gaps: Vec<DesktopCoverageGap>,
    pub authority_effect: String,
}

impl DesktopBehaviourTimeline {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            sample_count: 0,
            coverage_started_at: None,
            coverage_ended_at: None,
            focus_transitions: Vec::new(),
            window_revisits: Vec::new(),
            recent_focus_spans: Vec::new(),
            current_focus: None,
            current_focus_started_at: None,
            current_focus_sample_span_seconds: None,
            coverage_gaps: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Project behaviour from observation samples ordered oldest → newest.
pub fn project_desktop_behaviour(
    snapshots: &[WorkspaceObservationSnapshot],
) -> DesktopBehaviourTimeline {
    if snapshots.is_empty() {
        return DesktopBehaviourTimeline::empty();
    }

    let coverage_started_at = snapshots
        .first()
        .map(|snapshot| snapshot.pass.captured_at.clone());
    let coverage_ended_at = snapshots
        .last()
        .map(|snapshot| snapshot.pass.captured_at.clone());

    let mut focus_transitions = Vec::new();
    let mut coverage_gaps = Vec::new();
    let mut revisit_counts: BTreeMap<String, (ObservationWindowRef, i32, String)> = BTreeMap::new();
    let mut recent_focus_spans = Vec::new();

    let mut span_window: Option<ObservationWindowRef> = focused_ref(snapshots.first());
    let mut span_started_at = snapshots
        .first()
        .map(|snapshot| snapshot.pass.captured_at.clone())
        .unwrap_or_default();
    let mut span_sample_count = 1_i32;

    for window in snapshots.windows(2) {
        let previous = &window[0];
        let current = &window[1];
        if let Some(gap_seconds) =
            sample_gap_seconds(&previous.pass.captured_at, &current.pass.captured_at)
        {
            if gap_seconds >= DESKTOP_BEHAVIOUR_GAP_SECONDS {
                coverage_gaps.push(DesktopCoverageGap {
                    after_pass_id: previous.pass.id.clone(),
                    before_pass_id: current.pass.id.clone(),
                    previous_captured_at: previous.pass.captured_at.clone(),
                    next_captured_at: current.pass.captured_at.clone(),
                    gap_seconds,
                });
            }
        }

        let delta = compare_observation_snapshots(previous, current);
        if let Some(change) = delta.focused_window_changed {
            focus_transitions.push(DesktopFocusTransition {
                at: current.pass.captured_at.clone(),
                from_pass_id: previous.pass.id.clone(),
                to_pass_id: current.pass.id.clone(),
                previous: change.previous.clone(),
                current: change.current.clone(),
            });
            if let Some(focused) = change.current.clone() {
                let key = window_key(&focused);
                let entry = revisit_counts.entry(key).or_insert_with(|| {
                    (focused.clone(), 0, current.pass.captured_at.clone())
                });
                entry.0 = focused;
                entry.1 += 1;
                entry.2 = current.pass.captured_at.clone();
            }

            if let Some(previous_span) = span_window.take() {
                recent_focus_spans.push(DesktopObservedFocusSpan {
                    window: previous_span,
                    started_at: span_started_at.clone(),
                    ended_at: previous.pass.captured_at.clone(),
                    sample_span_seconds: sample_gap_seconds(
                        &span_started_at,
                        &previous.pass.captured_at,
                    ),
                    sample_count: span_sample_count,
                });
            }
            span_window = change.current;
            span_started_at = current.pass.captured_at.clone();
            span_sample_count = 1;
        } else {
            span_sample_count += 1;
        }
    }

    let current_focus = focused_ref(snapshots.last());
    let current_focus_started_at = if current_focus.is_some() {
        Some(span_started_at.clone())
    } else {
        None
    };
    let current_focus_sample_span_seconds = match (
        current_focus_started_at.as_deref(),
        coverage_ended_at.as_deref(),
    ) {
        (Some(start), Some(end)) => sample_gap_seconds(start, end),
        _ => None,
    };

    let mut window_revisits: Vec<DesktopWindowRevisit> = revisit_counts
        .into_values()
        .map(|(window, focus_count, last_focused_at)| DesktopWindowRevisit {
            window,
            focus_count,
            last_focused_at,
        })
        .collect();
    window_revisits.sort_by(|a, b| {
        b.focus_count
            .cmp(&a.focus_count)
            .then_with(|| a.window.hwnd.cmp(&b.window.hwnd))
    });

    // Keep a bounded recent span history (completed spans only).
    if recent_focus_spans.len() > 24 {
        let skip = recent_focus_spans.len() - 24;
        recent_focus_spans = recent_focus_spans.split_off(skip);
    }

    DesktopBehaviourTimeline {
        sample_count: snapshots.len() as i32,
        coverage_started_at,
        coverage_ended_at,
        focus_transitions,
        window_revisits,
        recent_focus_spans,
        current_focus,
        current_focus_started_at,
        current_focus_sample_span_seconds,
        coverage_gaps,
        authority_effect: DesktopBehaviourTimeline::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn focused_ref(snapshot: Option<&WorkspaceObservationSnapshot>) -> Option<ObservationWindowRef> {
    snapshot.and_then(|snap| {
        snap.windows
            .iter()
            .find(|window| window.focused)
            .map(ObservationWindowRef::from_window)
    })
}

fn window_key(window: &ObservationWindowRef) -> String {
    window
        .stable_window_id
        .as_ref()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| window.hwnd.clone())
}

fn sample_gap_seconds(previous: &str, next: &str) -> Option<i64> {
    let previous = DateTime::parse_from_rfc3339(previous).ok()?.with_timezone(&Utc);
    let next = DateTime::parse_from_rfc3339(next).ok()?.with_timezone(&Utc);
    Some((next - previous).num_seconds())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, WorkspaceObservationPass,
    };

    fn snap(
        pass_id: &str,
        captured_at: &str,
        focused_hwnd: &str,
        focused_stable: &str,
    ) -> WorkspaceObservationSnapshot {
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test".into(),
                foreground_hwnd: Some(focused_hwnd.into()),
                window_count: 2,
                monitor_count: 1,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: format!("{pass_id}-mon"),
                pass_id: pass_id.into(),
                monitor_index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                work_x: 0,
                work_y: 0,
                work_w: 1920,
                work_h: 1040,
                is_primary: true,
                dpi_scale: None,
                authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
            }],
            windows: vec![
                ObservedWindow {
                    id: format!("{pass_id}-a"),
                    pass_id: pass_id.into(),
                    hwnd: "0x1".into(),
                    stable_window_id: Some("stable-a".into()),
                    title: "Alpha".into(),
                    process_id: 10,
                    process_name: Some("alpha.exe".into()),
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                    monitor_id: Some(format!("{pass_id}-mon")),
                    visible: true,
                    minimized: false,
                    focused: focused_hwnd == "0x1",
                    z_order: Some(0),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                },
                ObservedWindow {
                    id: format!("{pass_id}-b"),
                    pass_id: pass_id.into(),
                    hwnd: "0x2".into(),
                    stable_window_id: Some(focused_stable.into()),
                    title: "Beta".into(),
                    process_id: 20,
                    process_name: Some("beta.exe".into()),
                    x: 10,
                    y: 10,
                    width: 100,
                    height: 100,
                    monitor_id: Some(format!("{pass_id}-mon")),
                    visible: true,
                    minimized: false,
                    focused: focused_hwnd == "0x2",
                    z_order: Some(1),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                },
            ],
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn projects_focus_transitions_revisits_and_current_span() {
        let snapshots = vec![
            snap("p1", "2026-07-30T10:00:00Z", "0x1", "stable-b"),
            snap("p2", "2026-07-30T10:01:00Z", "0x2", "stable-b"),
            snap("p3", "2026-07-30T10:02:00Z", "0x1", "stable-b"),
            snap("p4", "2026-07-30T10:03:00Z", "0x1", "stable-b"),
        ];
        let behaviour = project_desktop_behaviour(&snapshots);
        assert_eq!(behaviour.sample_count, 4);
        assert_eq!(behaviour.focus_transitions.len(), 2);
        assert_eq!(
            behaviour.current_focus.as_ref().map(|w| w.hwnd.as_str()),
            Some("0x1")
        );
        assert_eq!(
            behaviour.current_focus_started_at.as_deref(),
            Some("2026-07-30T10:02:00Z")
        );
        assert_eq!(behaviour.current_focus_sample_span_seconds, Some(60));
        assert!(behaviour
            .window_revisits
            .iter()
            .any(|revisit| revisit.window.hwnd == "0x2" && revisit.focus_count == 1));
        assert!(behaviour
            .window_revisits
            .iter()
            .any(|revisit| revisit.window.hwnd == "0x1" && revisit.focus_count == 1));
        assert_eq!(behaviour.recent_focus_spans.len(), 2);
        assert!(behaviour.coverage_gaps.is_empty());
    }

    #[test]
    fn records_coverage_gaps_between_sparse_samples() {
        let snapshots = vec![
            snap("p1", "2026-07-30T10:00:00Z", "0x1", "stable-b"),
            snap("p2", "2026-07-30T11:00:00Z", "0x1", "stable-b"),
        ];
        let behaviour = project_desktop_behaviour(&snapshots);
        assert_eq!(behaviour.coverage_gaps.len(), 1);
        assert_eq!(behaviour.coverage_gaps[0].gap_seconds, 3600);
    }
}
