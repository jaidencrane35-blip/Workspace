//! Deterministic desktop runtime memory from identity registry + observation history.
//!
//! Projects present / absent / returning entity continuity onto WorkspaceState.
//! Evidence-only — no AI, no Win32, no parallel history store.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::desktop_behaviour::DesktopBehaviourTimeline;
use crate::desktop_grouping::DesktopWindowGroup;
use crate::workspace_observation::{
    ObservationWindowIdentity, ObservedWindow, WorkspaceObservationSnapshot,
};

/// Soft bound so memory stays inspectable alongside the behaviour sample window.
pub const DESKTOP_RUNTIME_MEMORY_ENTITY_LIMIT: usize = 100;

/// One identity-keyed continuity memory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopObjectMemory {
    pub stable_window_id: String,
    pub hwnd: String,
    pub title: String,
    pub process_id: i32,
    pub process_name: Option<String>,
    pub first_observed_at: String,
    pub last_observed_at: String,
    pub identity_confidence: String,
    /// `present` | `absent` | `returning`
    pub presence: String,
    pub sample_presence_count: i32,
    pub session_presence_count: i32,
    pub focus_count: i32,
    pub opened_count: i32,
    pub closed_count: i32,
    /// Times the entity reappeared after an absence within retained samples.
    pub recurrence_count: i32,
    /// `ephemeral` | `intermittent` | `stable` | `persistent`
    pub stability: String,
    /// Continuity confidence from observation evidence (independent of identity match quality).
    /// `structural` | `emerging` | `recurring` | `strong`
    pub continuity_confidence: String,
    pub authority_effect: String,
}

impl DesktopObjectMemory {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PRESENCE_PRESENT: &'static str = "present";
    pub const PRESENCE_ABSENT: &'static str = "absent";
    pub const PRESENCE_RETURNING: &'static str = "returning";
    pub const STABILITY_EPHEMERAL: &'static str = "ephemeral";
    pub const STABILITY_INTERMITTENT: &'static str = "intermittent";
    pub const STABILITY_STABLE: &'static str = "stable";
    pub const STABILITY_PERSISTENT: &'static str = "persistent";
}

/// Bounded runtime memory projected onto WorkspaceState.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopRuntimeMemory {
    pub entities: Vec<DesktopObjectMemory>,
    pub present_count: i32,
    pub absent_count: i32,
    pub returning_count: i32,
    pub authority_effect: String,
}

impl DesktopRuntimeMemory {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            entities: Vec::new(),
            present_count: 0,
            absent_count: 0,
            returning_count: 0,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Project identity-keyed continuity memory from retained samples + live registry rows.
///
/// `history` must be oldest → newest.
pub fn project_desktop_runtime_memory(
    history: &[WorkspaceObservationSnapshot],
    identities: &[ObservationWindowIdentity],
    behaviour: &DesktopBehaviourTimeline,
) -> DesktopRuntimeMemory {
    if history.is_empty() && identities.is_empty() {
        return DesktopRuntimeMemory::empty();
    }

    let sample_count = history.len() as i32;
    let identity_by_id: HashMap<&str, &ObservationWindowIdentity> = identities
        .iter()
        .map(|identity| (identity.id.as_str(), identity))
        .collect();

    let mut ordered_ids: BTreeSet<String> = BTreeSet::new();
    let mut last_window_by_id: BTreeMap<String, &ObservedWindow> = BTreeMap::new();

    for snapshot in history {
        for window in &snapshot.windows {
            let Some(id) = stable_id(window.stable_window_id.as_deref()) else {
                continue;
            };
            ordered_ids.insert(id.clone());
            last_window_by_id.insert(id, window);
        }
    }
    for identity in identities {
        ordered_ids.insert(identity.id.clone());
    }

    let mut presence_by_id: BTreeMap<String, Vec<bool>> = BTreeMap::new();
    for id in &ordered_ids {
        let series: Vec<bool> = history
            .iter()
            .map(|snapshot| {
                snapshot.windows.iter().any(|window| {
                    stable_id(window.stable_window_id.as_deref())
                        .as_ref()
                        .is_some_and(|value| value == id)
                })
            })
            .collect();
        presence_by_id.insert(id.clone(), series);
    }

    let focus_by_id = focus_counts(behaviour);
    let lifecycle_by_id = lifecycle_counts(behaviour);
    let session_presence = session_presence_counts(history, behaviour);

    let current_present: BTreeSet<String> = history
        .last()
        .map(|snapshot| {
            snapshot
                .windows
                .iter()
                .filter_map(|window| stable_id(window.stable_window_id.as_deref()))
                .collect()
        })
        .unwrap_or_default();

    let mut entities = Vec::new();
    for id in ordered_ids {
        let series = presence_by_id.get(&id).cloned().unwrap_or_default();
        let sample_presence_count = series.iter().filter(|present| **present).count() as i32;
        if sample_presence_count == 0 && !identity_by_id.contains_key(id.as_str()) {
            continue;
        }

        let recurrence_count = count_recurrences(&series);
        let is_present = current_present.contains(&id);
        let presence = if !is_present {
            DesktopObjectMemory::PRESENCE_ABSENT
        } else if recurrence_count > 0 {
            DesktopObjectMemory::PRESENCE_RETURNING
        } else {
            DesktopObjectMemory::PRESENCE_PRESENT
        };

        let identity = identity_by_id.get(id.as_str()).copied();
        let last_window = last_window_by_id.get(&id).copied();
        let focus_count = focus_by_id.get(&id).copied().unwrap_or(0);
        let (opened_count, closed_count) = lifecycle_by_id.get(&id).copied().unwrap_or((0, 0));

        let first_observed_at = identity
            .map(|row| row.first_seen_at.clone())
            .or_else(|| first_present_at(history, &series))
            .unwrap_or_default();
        let last_observed_at = identity
            .map(|row| row.last_seen_at.clone())
            .or_else(|| last_present_at(history, &series))
            .unwrap_or_default();

        let evidence = sample_presence_count
            .saturating_add(focus_count)
            .saturating_add(recurrence_count);
        let continuity_confidence = DesktopWindowGroup::confidence_for_evidence(evidence);
        let stability = stability_for(sample_presence_count, sample_count, recurrence_count);

        entities.push(DesktopObjectMemory {
            stable_window_id: id.clone(),
            hwnd: identity
                .map(|row| row.last_hwnd.clone())
                .or_else(|| last_window.map(|window| window.hwnd.clone()))
                .unwrap_or_default(),
            title: last_window
                .map(|window| window.title.clone())
                .or_else(|| identity.map(|row| row.title_fingerprint.clone()))
                .unwrap_or_default(),
            process_id: identity
                .map(|row| row.process_id)
                .or_else(|| last_window.map(|window| window.process_id))
                .unwrap_or(0),
            process_name: last_window.and_then(|window| window.process_name.clone()),
            first_observed_at,
            last_observed_at,
            identity_confidence: identity
                .map(|row| row.confidence.as_str().to_string())
                .unwrap_or_else(|| "unknown".into()),
            presence: presence.into(),
            sample_presence_count,
            session_presence_count: session_presence.get(&id).copied().unwrap_or(0),
            focus_count,
            opened_count,
            closed_count,
            recurrence_count,
            stability: stability.into(),
            continuity_confidence: continuity_confidence.into(),
            authority_effect: DesktopObjectMemory::AUTHORITY_EFFECT_NONE.into(),
        });
    }

    entities.sort_by(|left, right| {
        right
            .last_observed_at
            .cmp(&left.last_observed_at)
            .then_with(|| left.stable_window_id.cmp(&right.stable_window_id))
    });
    entities.truncate(DESKTOP_RUNTIME_MEMORY_ENTITY_LIMIT);

    let present_count = entities
        .iter()
        .filter(|entity| entity.presence == DesktopObjectMemory::PRESENCE_PRESENT)
        .count() as i32;
    let absent_count = entities
        .iter()
        .filter(|entity| entity.presence == DesktopObjectMemory::PRESENCE_ABSENT)
        .count() as i32;
    let returning_count = entities
        .iter()
        .filter(|entity| entity.presence == DesktopObjectMemory::PRESENCE_RETURNING)
        .count() as i32;

    DesktopRuntimeMemory {
        entities,
        present_count,
        absent_count,
        returning_count,
        authority_effect: DesktopRuntimeMemory::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn stable_id(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn count_recurrences(series: &[bool]) -> i32 {
    let mut count = 0;
    let mut seen = false;
    let mut absent_after_seen = false;
    for &present in series {
        if present {
            if absent_after_seen {
                count += 1;
                absent_after_seen = false;
            }
            seen = true;
        } else if seen {
            absent_after_seen = true;
        }
    }
    count
}

fn stability_for(
    sample_presence_count: i32,
    sample_count: i32,
    recurrence_count: i32,
) -> &'static str {
    if sample_presence_count <= 1 {
        return DesktopObjectMemory::STABILITY_EPHEMERAL;
    }
    if sample_count <= 0 {
        return DesktopObjectMemory::STABILITY_EPHEMERAL;
    }
    let ratio = (sample_presence_count as f64) / (sample_count as f64);
    if ratio >= 0.7 && recurrence_count == 0 {
        DesktopObjectMemory::STABILITY_PERSISTENT
    } else if ratio >= 0.45 {
        DesktopObjectMemory::STABILITY_STABLE
    } else {
        DesktopObjectMemory::STABILITY_INTERMITTENT
    }
}

fn focus_counts(behaviour: &DesktopBehaviourTimeline) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for revisit in &behaviour.window_revisits {
        if let Some(id) = stable_id(revisit.window.stable_window_id.as_deref()) {
            map.insert(id, revisit.focus_count);
        }
    }
    map
}

fn lifecycle_counts(behaviour: &DesktopBehaviourTimeline) -> HashMap<String, (i32, i32)> {
    let mut map = HashMap::new();
    for lifecycle in &behaviour.window_lifecycles {
        if let Some(id) = stable_id(lifecycle.window.stable_window_id.as_deref()) {
            map.insert(id, (lifecycle.opened_count, lifecycle.closed_count));
        }
    }
    map
}

fn session_presence_counts(
    history: &[WorkspaceObservationSnapshot],
    behaviour: &DesktopBehaviourTimeline,
) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    if behaviour.sessions.is_empty() {
        return map;
    }

    for session in &behaviour.sessions {
        let mut seen_in_session: BTreeSet<String> = BTreeSet::new();
        let mut in_range = false;
        for snapshot in history {
            if snapshot.pass.id == session.start_pass_id {
                in_range = true;
            }
            if in_range {
                for window in &snapshot.windows {
                    if let Some(id) = stable_id(window.stable_window_id.as_deref()) {
                        seen_in_session.insert(id);
                    }
                }
            }
            if snapshot.pass.id == session.end_pass_id {
                break;
            }
        }
        for id in seen_in_session {
            *map.entry(id).or_insert(0) += 1;
        }
    }
    map
}

fn first_present_at(history: &[WorkspaceObservationSnapshot], series: &[bool]) -> Option<String> {
    series
        .iter()
        .zip(history.iter())
        .find(|(present, _)| **present)
        .map(|(_, snapshot)| snapshot.pass.captured_at.clone())
}

fn last_present_at(history: &[WorkspaceObservationSnapshot], series: &[bool]) -> Option<String> {
    series
        .iter()
        .zip(history.iter())
        .rev()
        .find(|(present, _)| **present)
        .map(|(_, snapshot)| snapshot.pass.captured_at.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop_behaviour::project_desktop_behaviour;
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, WindowIdentityConfidence, WorkspaceObservationPass,
        WorkspaceObservationSnapshot,
    };

    fn snap(
        pass_id: &str,
        captured_at: &str,
        windows: Vec<(&str, &str, bool)>,
    ) -> WorkspaceObservationSnapshot {
        let mon_id = format!("{pass_id}-mon");
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test".into(),
                foreground_hwnd: windows
                    .iter()
                    .find(|(_, _, focused)| *focused)
                    .map(|(_, hwnd, _)| (*hwnd).into()),
                window_count: windows.len() as i32,
                monitor_count: 1,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: mon_id.clone(),
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
            windows: windows
                .into_iter()
                .enumerate()
                .map(|(index, (stable, hwnd, focused))| ObservedWindow {
                    id: format!("{pass_id}-{stable}"),
                    pass_id: pass_id.into(),
                    hwnd: hwnd.into(),
                    stable_window_id: Some(stable.into()),
                    title: format!("Window {stable}"),
                    process_id: 100 + index as i32,
                    process_name: Some("app.exe".into()),
                    x: 0,
                    y: 0,
                    width: 800,
                    height: 600,
                    monitor_id: Some(mon_id.clone()),
                    visible: true,
                    minimized: false,
                    focused,
                    z_order: Some(index as i32),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                })
                .collect(),
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn projects_present_absent_and_returning() {
        let history = vec![
            snap("p1", "2026-07-30T10:00:00Z", vec![("stable-a", "0x1", true)]),
            snap("p2", "2026-07-30T10:01:00Z", vec![("stable-b", "0x2", true)]),
            snap(
                "p3",
                "2026-07-30T10:02:00Z",
                vec![("stable-a", "0x1", true), ("stable-b", "0x2", false)],
            ),
        ];
        let identities = vec![
            ObservationWindowIdentity {
                id: "stable-a".into(),
                process_id: 100,
                title_fingerprint: "Window stable-a".into(),
                first_seen_at: "2026-07-30T09:00:00Z".into(),
                last_seen_at: "2026-07-30T10:02:00Z".into(),
                last_hwnd: "0x1".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            },
            ObservationWindowIdentity {
                id: "stable-b".into(),
                process_id: 101,
                title_fingerprint: "Window stable-b".into(),
                first_seen_at: "2026-07-30T10:01:00Z".into(),
                last_seen_at: "2026-07-30T10:02:00Z".into(),
                last_hwnd: "0x2".into(),
                confidence: WindowIdentityConfidence::Medium,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            },
        ];
        let behaviour = project_desktop_behaviour(&history);
        let memory = project_desktop_runtime_memory(&history, &identities, &behaviour);

        let a = memory
            .entities
            .iter()
            .find(|entity| entity.stable_window_id == "stable-a")
            .expect("stable-a");
        assert_eq!(a.presence, DesktopObjectMemory::PRESENCE_RETURNING);
        assert!(a.recurrence_count >= 1);
        assert_eq!(a.first_observed_at, "2026-07-30T09:00:00Z");

        let b = memory
            .entities
            .iter()
            .find(|entity| entity.stable_window_id == "stable-b")
            .expect("stable-b");
        assert_eq!(b.presence, DesktopObjectMemory::PRESENCE_PRESENT);
        assert_eq!(b.sample_presence_count, 2);
    }

    #[test]
    fn marks_absent_when_missing_from_latest_sample() {
        let history = vec![
            snap("p1", "2026-07-30T10:00:00Z", vec![("stable-a", "0x1", true)]),
            snap("p2", "2026-07-30T10:01:00Z", vec![("stable-b", "0x2", true)]),
        ];
        let identities = vec![ObservationWindowIdentity {
            id: "stable-a".into(),
            process_id: 100,
            title_fingerprint: "Window stable-a".into(),
            first_seen_at: "2026-07-30T10:00:00Z".into(),
            last_seen_at: "2026-07-30T10:00:00Z".into(),
            last_hwnd: "0x1".into(),
            confidence: WindowIdentityConfidence::High,
            authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
        }];
        let behaviour = project_desktop_behaviour(&history);
        let memory = project_desktop_runtime_memory(&history, &identities, &behaviour);
        let a = memory
            .entities
            .iter()
            .find(|entity| entity.stable_window_id == "stable-a")
            .expect("stable-a");
        assert_eq!(a.presence, DesktopObjectMemory::PRESENCE_ABSENT);
        assert!(memory.absent_count >= 1);
    }
}
