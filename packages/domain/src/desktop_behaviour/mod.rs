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
use crate::desktop_grouping::DesktopWindowGroup;

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

/// Focus A→B transitions aggregated across the sample window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopFocusFollow {
    pub from: ObservationWindowRef,
    pub to: ObservationWindowRef,
    pub transition_count: i32,
    pub last_at: String,
    /// Relationship confidence from transition evidence: `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
}

/// Windows observed open together across samples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopCoPresence {
    pub left: ObservationWindowRef,
    pub right: ObservationWindowRef,
    pub sample_count: i32,
    pub last_seen_at: String,
    /// Relationship confidence from co-presence evidence: `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
}

/// Observation session inferred from coverage continuity (not OS login sessions).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopObservationSession {
    pub id: String,
    pub started_at: String,
    pub ended_at: String,
    pub start_pass_id: String,
    pub end_pass_id: String,
    pub sample_count: i32,
    pub focus_transition_count: i32,
    pub dominant_focus: Option<ObservationWindowRef>,
    /// `active` | `completed` | `returning`
    pub kind: String,
    /// Session inference confidence from sample + transition evidence.
    pub confidence: String,
}

/// How often a window became focused across the sample window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowRevisit {
    pub window: ObservationWindowRef,
    pub focus_count: i32,
    pub last_focused_at: String,
    /// Behaviour confidence from revisit evidence.
    pub confidence: String,
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

/// Open/close lifecycle evidence for a window across retained samples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowLifecycle {
    pub window: ObservationWindowRef,
    pub opened_count: i32,
    pub closed_count: i32,
    pub last_opened_at: Option<String>,
    pub last_closed_at: Option<String>,
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
    pub focus_follows: Vec<DesktopFocusFollow>,
    pub co_presence: Vec<DesktopCoPresence>,
    pub sessions: Vec<DesktopObservationSession>,
    pub window_lifecycles: Vec<DesktopWindowLifecycle>,
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
            focus_follows: Vec::new(),
            co_presence: Vec::new(),
            sessions: Vec::new(),
            window_lifecycles: Vec::new(),
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
    let mut follow_counts: BTreeMap<(String, String), (ObservationWindowRef, ObservationWindowRef, i32, String)> =
        BTreeMap::new();
    let mut co_presence_counts: BTreeMap<(String, String), (ObservationWindowRef, ObservationWindowRef, i32, String)> =
        BTreeMap::new();
    let mut recent_focus_spans = Vec::new();

    // Seed co-presence from the first sample.
    if let Some(first) = snapshots.first() {
        record_co_presence(first, &mut co_presence_counts);
    }

    let mut span_window: Option<ObservationWindowRef> = focused_ref(snapshots.first());
    let mut span_started_at = snapshots
        .first()
        .map(|snapshot| snapshot.pass.captured_at.clone())
        .unwrap_or_default();
    let mut span_sample_count = 1_i32;
    let mut open_counts: BTreeMap<String, (ObservationWindowRef, i32, String)> = BTreeMap::new();
    let mut close_counts: BTreeMap<String, (ObservationWindowRef, i32, String)> = BTreeMap::new();

    for window in snapshots.windows(2) {
        let previous = &window[0];
        let current = &window[1];
        record_co_presence(current, &mut co_presence_counts);

        let mut crossed_gap = false;
        if let Some(gap_seconds) =
            sample_gap_seconds(&previous.pass.captured_at, &current.pass.captured_at)
        {
            if gap_seconds >= DESKTOP_BEHAVIOUR_GAP_SECONDS {
                crossed_gap = true;
                coverage_gaps.push(DesktopCoverageGap {
                    after_pass_id: previous.pass.id.clone(),
                    before_pass_id: current.pass.id.clone(),
                    previous_captured_at: previous.pass.captured_at.clone(),
                    next_captured_at: current.pass.captured_at.clone(),
                    gap_seconds,
                });
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
                span_window = focused_ref(Some(current));
                span_started_at = current.pass.captured_at.clone();
                span_sample_count = 1;
            }
        }

        let delta = compare_observation_snapshots(previous, current);
        if !crossed_gap {
            for opened in &delta.opened_windows {
                let key = window_key(opened);
                let entry = open_counts.entry(key).or_insert_with(|| {
                    (opened.clone(), 0, current.pass.captured_at.clone())
                });
                entry.0 = opened.clone();
                entry.1 += 1;
                entry.2 = current.pass.captured_at.clone();
            }
            for closed in &delta.closed_windows {
                let key = window_key(closed);
                let entry = close_counts.entry(key).or_insert_with(|| {
                    (closed.clone(), 0, current.pass.captured_at.clone())
                });
                entry.0 = closed.clone();
                entry.1 += 1;
                entry.2 = current.pass.captured_at.clone();
            }
        }

        if let Some(change) = delta.focused_window_changed {
            // Focus transitions across coverage gaps are unobserved; keep session
            // boundaries via gaps/spans only.
            if !crossed_gap {
                focus_transitions.push(DesktopFocusTransition {
                    at: current.pass.captured_at.clone(),
                    from_pass_id: previous.pass.id.clone(),
                    to_pass_id: current.pass.id.clone(),
                    previous: change.previous.clone(),
                    current: change.current.clone(),
                });
                if let (Some(from), Some(to)) = (change.previous.clone(), change.current.clone()) {
                    let key = (window_key(&from), window_key(&to));
                    let entry = follow_counts.entry(key).or_insert_with(|| {
                        (from.clone(), to.clone(), 0, current.pass.captured_at.clone())
                    });
                    entry.0 = from;
                    entry.1 = to;
                    entry.2 += 1;
                    entry.3 = current.pass.captured_at.clone();
                }
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
            }
        } else if !crossed_gap {
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
            confidence: DesktopWindowGroup::confidence_for_evidence(focus_count).into(),
        })
        .collect();
    window_revisits.sort_by(|a, b| {
        b.focus_count
            .cmp(&a.focus_count)
            .then_with(|| a.window.hwnd.cmp(&b.window.hwnd))
    });

    let mut focus_follows: Vec<DesktopFocusFollow> = follow_counts
        .into_values()
        .map(|(from, to, transition_count, last_at)| DesktopFocusFollow {
            from,
            to,
            transition_count,
            last_at,
            confidence: DesktopWindowGroup::confidence_for_evidence(transition_count).into(),
        })
        .collect();
    focus_follows.sort_by(|a, b| {
        b.transition_count
            .cmp(&a.transition_count)
            .then_with(|| a.from.hwnd.cmp(&b.from.hwnd))
            .then_with(|| a.to.hwnd.cmp(&b.to.hwnd))
    });
    if focus_follows.len() > 24 {
        focus_follows.truncate(24);
    }

    let mut co_presence: Vec<DesktopCoPresence> = co_presence_counts
        .into_values()
        .filter(|(_, _, count, _)| *count >= 2)
        .map(|(left, right, sample_count, last_seen_at)| DesktopCoPresence {
            left,
            right,
            sample_count,
            last_seen_at,
            confidence: DesktopWindowGroup::confidence_for_evidence(sample_count).into(),
        })
        .collect();
    co_presence.sort_by(|a, b| {
        b.sample_count
            .cmp(&a.sample_count)
            .then_with(|| a.left.hwnd.cmp(&b.left.hwnd))
            .then_with(|| a.right.hwnd.cmp(&b.right.hwnd))
    });
    if co_presence.len() > 32 {
        co_presence.truncate(32);
    }

    let mut lifecycle_keys = BTreeMap::<String, DesktopWindowLifecycle>::new();
    for (key, (window, count, at)) in open_counts {
        let entry = lifecycle_keys.entry(key).or_insert_with(|| DesktopWindowLifecycle {
            window: window.clone(),
            opened_count: 0,
            closed_count: 0,
            last_opened_at: None,
            last_closed_at: None,
        });
        entry.window = window;
        entry.opened_count = count;
        entry.last_opened_at = Some(at);
    }
    for (key, (window, count, at)) in close_counts {
        let entry = lifecycle_keys.entry(key).or_insert_with(|| DesktopWindowLifecycle {
            window: window.clone(),
            opened_count: 0,
            closed_count: 0,
            last_opened_at: None,
            last_closed_at: None,
        });
        entry.window = window;
        entry.closed_count = count;
        entry.last_closed_at = Some(at);
    }
    let mut window_lifecycles: Vec<DesktopWindowLifecycle> = lifecycle_keys.into_values().collect();
    window_lifecycles.sort_by(|a, b| {
        (b.opened_count + b.closed_count)
            .cmp(&(a.opened_count + a.closed_count))
            .then_with(|| a.window.hwnd.cmp(&b.window.hwnd))
    });
    if window_lifecycles.len() > 32 {
        window_lifecycles.truncate(32);
    }

    // Keep a bounded recent span history (completed spans only).
    if recent_focus_spans.len() > 24 {
        let skip = recent_focus_spans.len() - 24;
        recent_focus_spans = recent_focus_spans.split_off(skip);
    }

    DesktopBehaviourTimeline {
        sample_count: snapshots.len() as i32,
        coverage_started_at,
        coverage_ended_at,
        focus_transitions: focus_transitions.clone(),
        window_revisits,
        recent_focus_spans,
        focus_follows,
        co_presence,
        sessions: project_observation_sessions(snapshots, &focus_transitions, &coverage_gaps),
        window_lifecycles,
        current_focus,
        current_focus_started_at,
        current_focus_sample_span_seconds,
        coverage_gaps,
        authority_effect: DesktopBehaviourTimeline::AUTHORITY_EFFECT_NONE.into(),
    }
}

/// Raise grouping confidence from co-presence evidence without replacing structural groups.
pub fn strengthen_groups_from_behaviour(
    groups: &mut [DesktopWindowGroup],
    behaviour: &DesktopBehaviourTimeline,
) {
    for group in groups.iter_mut() {
        if group.member_ids.len() < 2 {
            continue;
        }
        let mut best = group.evidence_count;
        for presence in &behaviour.co_presence {
            let left = window_key(&presence.left);
            let right = window_key(&presence.right);
            if group.member_ids.iter().any(|id| id == &left)
                && group.member_ids.iter().any(|id| id == &right)
            {
                best = best.max(presence.sample_count);
            }
        }
        group.evidence_count = best;
        group.confidence = DesktopWindowGroup::confidence_for_evidence(best).into();
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

fn observed_ref(window: &crate::workspace_observation::ObservedWindow) -> ObservationWindowRef {
    ObservationWindowRef::from_window(window)
}

fn record_co_presence(
    snapshot: &WorkspaceObservationSnapshot,
    counts: &mut BTreeMap<
        (String, String),
        (ObservationWindowRef, ObservationWindowRef, i32, String),
    >,
) {
    let visible: Vec<_> = snapshot
        .windows
        .iter()
        .filter(|window| window.visible)
        .collect();
    for i in 0..visible.len() {
        for j in (i + 1)..visible.len() {
            let left = observed_ref(visible[i]);
            let right = observed_ref(visible[j]);
            let mut a = window_key(&left);
            let mut b = window_key(&right);
            let (left_ref, right_ref) = if a <= b {
                (left, right)
            } else {
                std::mem::swap(&mut a, &mut b);
                (right, left)
            };
            let entry = counts.entry((a, b)).or_insert_with(|| {
                (
                    left_ref.clone(),
                    right_ref.clone(),
                    0,
                    snapshot.pass.captured_at.clone(),
                )
            });
            entry.0 = left_ref;
            entry.1 = right_ref;
            entry.2 += 1;
            entry.3 = snapshot.pass.captured_at.clone();
        }
    }
}

fn sample_gap_seconds(previous: &str, next: &str) -> Option<i64> {
    let previous = DateTime::parse_from_rfc3339(previous).ok()?.with_timezone(&Utc);
    let next = DateTime::parse_from_rfc3339(next).ok()?.with_timezone(&Utc);
    Some((next - previous).num_seconds())
}

fn project_observation_sessions(
    snapshots: &[WorkspaceObservationSnapshot],
    focus_transitions: &[DesktopFocusTransition],
    coverage_gaps: &[DesktopCoverageGap],
) -> Vec<DesktopObservationSession> {
    if snapshots.is_empty() {
        return Vec::new();
    }
    let gap_before_pass: std::collections::BTreeSet<&str> = coverage_gaps
        .iter()
        .map(|gap| gap.before_pass_id.as_str())
        .collect();

    let mut sessions = Vec::new();
    let mut start_index = 0usize;
    for index in 1..snapshots.len() {
        if gap_before_pass.contains(snapshots[index].pass.id.as_str()) {
            sessions.push(build_session(
                &snapshots[start_index..=index - 1],
                focus_transitions,
                !sessions.is_empty(),
                false,
            ));
            start_index = index;
        }
    }
    sessions.push(build_session(
        &snapshots[start_index..],
        focus_transitions,
        !sessions.is_empty(),
        true,
    ));
    sessions
}

fn build_session(
    snapshots: &[WorkspaceObservationSnapshot],
    focus_transitions: &[DesktopFocusTransition],
    after_gap: bool,
    is_current: bool,
) -> DesktopObservationSession {
    let first = &snapshots[0];
    let last = &snapshots[snapshots.len() - 1];
    let start_at = first.pass.captured_at.as_str();
    let end_at = last.pass.captured_at.as_str();
    let transition_count = focus_transitions
        .iter()
        .filter(|transition| {
            transition.at.as_str() >= start_at && transition.at.as_str() <= end_at
        })
        .count() as i32;

    let mut focus_tallies: BTreeMap<String, (ObservationWindowRef, i32)> = BTreeMap::new();
    for snapshot in snapshots {
        if let Some(focused) = snapshot
            .windows
            .iter()
            .find(|window| window.focused)
            .map(ObservationWindowRef::from_window)
        {
            let key = window_key(&focused);
            let entry = focus_tallies
                .entry(key)
                .or_insert_with(|| (focused.clone(), 0));
            entry.0 = focused;
            entry.1 += 1;
        }
    }
    let dominant_focus = focus_tallies
        .into_values()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.hwnd.cmp(&b.0.hwnd)))
        .map(|(window, _)| window);

    let kind = if is_current && after_gap {
        "returning"
    } else if is_current {
        "active"
    } else {
        "completed"
    };

    let evidence = snapshots.len() as i32 + transition_count;
    DesktopObservationSession {
        id: format!("session:{}:{}", first.pass.id, last.pass.id),
        started_at: first.pass.captured_at.clone(),
        ended_at: last.pass.captured_at.clone(),
        start_pass_id: first.pass.id.clone(),
        end_pass_id: last.pass.id.clone(),
        sample_count: snapshots.len() as i32,
        focus_transition_count: transition_count,
        dominant_focus,
        kind: kind.into(),
        confidence: DesktopWindowGroup::confidence_for_evidence(evidence).into(),
    }
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
        assert!(behaviour
            .focus_follows
            .iter()
            .any(|follow| follow.from.hwnd == "0x1"
                && follow.to.hwnd == "0x2"
                && follow.transition_count == 1));
        assert!(behaviour
            .co_presence
            .iter()
            .any(|pair| pair.sample_count >= 2));
        assert!(behaviour.focus_follows.iter().all(|follow| {
            !follow.confidence.is_empty()
                && follow.confidence
                    == DesktopWindowGroup::confidence_for_evidence(follow.transition_count)
        }));
        assert!(behaviour.co_presence.iter().all(|pair| {
            !pair.confidence.is_empty()
                && pair.confidence
                    == DesktopWindowGroup::confidence_for_evidence(pair.sample_count)
        }));
        assert!(behaviour
            .sessions
            .iter()
            .all(|session| !session.confidence.is_empty()));
        assert!(behaviour
            .window_revisits
            .iter()
            .all(|revisit| !revisit.confidence.is_empty()));
    }

    #[test]
    fn strengthens_group_confidence_from_co_presence() {
        let snapshots = vec![
            snap("p1", "2026-07-30T10:00:00Z", "0x1", "stable-b"),
            snap("p2", "2026-07-30T10:01:00Z", "0x2", "stable-b"),
            snap("p3", "2026-07-30T10:02:00Z", "0x1", "stable-b"),
            snap("p4", "2026-07-30T10:03:00Z", "0x1", "stable-b"),
        ];
        let behaviour = project_desktop_behaviour(&snapshots);
        let mut groups = vec![DesktopWindowGroup {
            id: "group:process_id:x".into(),
            criterion: "process_id".into(),
            fact_key: "x".into(),
            label: "pair".into(),
            member_ids: vec!["stable-a".into(), "stable-b".into()],
            evidence_count: 1,
            confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
            authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
        }];
        strengthen_groups_from_behaviour(&mut groups, &behaviour);
        assert!(groups[0].evidence_count >= 2);
        assert_ne!(
            groups[0].confidence,
            DesktopWindowGroup::CONFIDENCE_STRUCTURAL
        );
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
        assert_eq!(behaviour.sessions.len(), 2);
        assert_eq!(behaviour.sessions[0].kind, "completed");
        assert_eq!(behaviour.sessions[1].kind, "returning");
        assert!(
            behaviour
                .current_focus_sample_span_seconds
                .map(|seconds| seconds < 3600)
                .unwrap_or(true),
            "current focus span must not bridge the coverage gap"
        );
    }
}
