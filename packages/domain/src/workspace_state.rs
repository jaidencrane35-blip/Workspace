//! Workspace State — canonical runtime projection (Sprint 118/119).
//!
//! Observation = facts, Delta = change, WorkspaceState = current interpreted state.
//! Read-only projection only — no AI, automation, capture, or Win32.
//!
//! Distinct from the kernel lifecycle `WorkspaceState` (version/lifecycle).
//! Canonical input to WorkspaceEnvironmentService (Sprint 119).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::desktop_behaviour::{
    project_desktop_behaviour, strengthen_groups_from_behaviour, DesktopBehaviourTimeline,
};
use crate::desktop_grouping::{
    group_desktop_members, DesktopGroupCriterion, DesktopGroupMemberFact, DesktopWindowGroup,
};
use crate::desktop_decision::{project_desktop_decisions, DesktopDecisionProjection};
use crate::desktop_runtime_memory::{
    project_desktop_runtime_memory, strengthen_groups_from_runtime_memory, DesktopRuntimeMemory,
};
use crate::desktop_semantic::{project_desktop_semantics, DesktopSemanticProjection};
use crate::workspace_observation::{
    observation_now_rfc3339, ObservedMonitor, ObservedWindow, ObservationWindowIdentity,
    WorkspaceObservationSnapshot,
};
use crate::workspace_observation_delta::{ObservationWindowRef, WorkspaceObservationDelta};

/// Maximum windows retained on the projected state (matches former desktop adapter bound).
pub const WORKSPACE_STATE_WINDOW_LIMIT: usize = 50;

/// Metadata for a projected workspace state snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateMetadata {
    pub state_id: String,
    pub created_at: String,
    /// Latest observation pass id when present.
    pub observation_pass_id: Option<String>,
    /// Compact reference to the latest delta (`previous→current` or current-only).
    pub latest_delta_reference: Option<String>,
    pub window_count: i32,
    pub monitor_count: i32,
    pub has_changes: bool,
    pub authority_effect: String,
}

impl WorkspaceStateMetadata {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Active application summary derived from observed windows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActiveApplication {
    pub process_id: i32,
    pub process_name: Option<String>,
    pub window_count: i32,
}

/// Observed monitor row on WorkspaceState — authoritative desktop display geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateMonitor {
    pub monitor_index: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_w: i32,
    pub work_h: i32,
    pub is_primary: bool,
}

impl WorkspaceStateMonitor {
    pub fn from_observed(monitor: &ObservedMonitor) -> Self {
        Self {
            monitor_index: monitor.monitor_index,
            name: monitor.name.clone(),
            x: monitor.x,
            y: monitor.y,
            width: monitor.width,
            height: monitor.height,
            work_x: monitor.work_x,
            work_y: monitor.work_y,
            work_w: monitor.work_w,
            work_h: monitor.work_h,
            is_primary: monitor.is_primary,
        }
    }
}

/// Window row on WorkspaceState — authoritative runtime desktop object.
/// Owns observed geometry (x/y/width/height) for future OS apply consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateWindow {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub process_id: i32,
    pub process_name: Option<String>,
    pub visible: bool,
    pub focused: bool,
    pub minimized: bool,
    /// Observed stacking order when the capturer provides it.
    pub z_order: Option<i32>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: Option<i32>,
    pub monitor_name: Option<String>,
    /// Live identity-registry continuity (not historical snapshot state).
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
    pub identity_confidence: Option<String>,
}

impl WorkspaceStateWindow {
    pub fn from_observed(window: &ObservedWindow, monitors: &[ObservedMonitor]) -> Self {
        let monitor = window
            .monitor_id
            .as_ref()
            .and_then(|monitor_id| monitors.iter().find(|monitor| &monitor.id == monitor_id));
        Self {
            stable_window_id: window.stable_window_id.clone(),
            hwnd: window.hwnd.clone(),
            title: window.title.clone(),
            process_id: window.process_id,
            process_name: window.process_name.clone(),
            visible: window.visible,
            focused: window.focused,
            minimized: window.minimized,
            z_order: window.z_order,
            x: window.x,
            y: window.y,
            width: window.width,
            height: window.height,
            monitor_index: monitor.map(|monitor| monitor.monitor_index),
            monitor_name: monitor.map(|monitor| monitor.name.clone()),
            first_seen_at: None,
            last_seen_at: None,
            identity_confidence: None,
        }
    }

    /// Observed rectangle owned by this runtime object (not Stage %-layout).
    pub fn observed_geometry(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }

    pub fn member_id(&self) -> String {
        self.stable_window_id
            .as_ref()
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| self.hwnd.clone())
    }

    pub fn to_window_ref(&self) -> ObservationWindowRef {
        ObservationWindowRef {
            stable_window_id: self.stable_window_id.clone(),
            hwnd: self.hwnd.clone(),
            title: self.title.clone(),
            process_id: self.process_id,
        }
    }

    pub fn to_group_member_fact(&self) -> DesktopGroupMemberFact {
        DesktopGroupMemberFact {
            member_id: self.member_id(),
            process_id: self.process_id,
            process_name: self.process_name.clone(),
            monitor_index: self.monitor_index,
            arrangement_ids: Vec::new(),
            matched_application_id: None,
            matched_application_name: None,
        }
    }
}

/// Canonical runtime state projection from latest observation + delta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub metadata: WorkspaceStateMetadata,
    pub focused_window: Option<ObservationWindowRef>,
    pub active_applications: Vec<WorkspaceActiveApplication>,
    /// Bounded desktop window rows for Environment and other consumers.
    pub windows: Vec<WorkspaceStateWindow>,
    /// Observed displays — authoritative desktop geometry plane.
    pub monitors: Vec<WorkspaceStateMonitor>,
    /// Fact-driven groups from the generic desktop grouping engine.
    pub window_groups: Vec<DesktopWindowGroup>,
    /// Latest observation delta captured with this projection (atomic with windows).
    pub latest_delta: WorkspaceObservationDelta,
    /// Deterministic behaviour timeline from retained observation samples.
    pub behaviour: DesktopBehaviourTimeline,
    /// Identity-keyed continuity memory (present / absent / returning).
    pub runtime_memory: DesktopRuntimeMemory,
    /// Deterministic semantic projection (roles, relationships, activities, graph).
    pub semantics: DesktopSemanticProjection,
    /// Deterministic decision support (decisions, recommendations, consistency).
    pub decisions: DesktopDecisionProjection,
    pub authority_effect: String,
}

impl WorkspaceState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            metadata: WorkspaceStateMetadata {
                state_id: format!("workspace-state:empty:{}", Uuid::new_v4()),
                created_at: observation_now_rfc3339(),
                observation_pass_id: None,
                latest_delta_reference: None,
                window_count: 0,
                monitor_count: 0,
                has_changes: false,
                authority_effect: WorkspaceStateMetadata::AUTHORITY_EFFECT_NONE.into(),
            },
            focused_window: None,
            active_applications: Vec::new(),
            windows: Vec::new(),
            monitors: Vec::new(),
            window_groups: Vec::new(),
            latest_delta: WorkspaceObservationDelta::empty(),
            behaviour: DesktopBehaviourTimeline::empty(),
            runtime_memory: DesktopRuntimeMemory::empty(),
            semantics: DesktopSemanticProjection::empty(),
            decisions: DesktopDecisionProjection::empty(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Build projected state from optional latest observation and latest delta.
    pub fn from_observation_and_delta(
        observation: Option<&WorkspaceObservationSnapshot>,
        delta: &WorkspaceObservationDelta,
    ) -> Self {
        Self::from_observation_delta_and_history(observation, delta, &[])
    }

    /// Build projected state including a bounded behaviour timeline from history.
    ///
    /// `history` must be oldest → newest. When empty, behaviour uses the current
    /// observation alone (one-sample timeline) when present.
    pub fn from_observation_delta_and_history(
        observation: Option<&WorkspaceObservationSnapshot>,
        delta: &WorkspaceObservationDelta,
        history: &[WorkspaceObservationSnapshot],
    ) -> Self {
        let created_at = observation_now_rfc3339();
        let behaviour = if history.is_empty() {
            match observation {
                Some(snapshot) => project_desktop_behaviour(std::slice::from_ref(snapshot)),
                None => DesktopBehaviourTimeline::empty(),
            }
        } else {
            project_desktop_behaviour(history)
        };

        let Some(snapshot) = observation else {
            let mut empty = Self::empty();
            empty.metadata.created_at = created_at;
            empty.metadata.has_changes = delta.has_changes;
            empty.metadata.latest_delta_reference = delta_reference(delta);
            empty.latest_delta = delta.clone();
            empty.behaviour = behaviour;
            return empty;
        };

        let mut windows: Vec<WorkspaceStateWindow> = snapshot
            .windows
            .iter()
            .map(|window| WorkspaceStateWindow::from_observed(window, &snapshot.monitors))
            .collect();
        windows.truncate(WORKSPACE_STATE_WINDOW_LIMIT);

        let monitors: Vec<WorkspaceStateMonitor> = snapshot
            .monitors
            .iter()
            .map(WorkspaceStateMonitor::from_observed)
            .collect();

        let focused_window = windows
            .iter()
            .find(|window| window.focused)
            .map(WorkspaceStateWindow::to_window_ref);

        let active_applications = active_applications_from_state_windows(&windows);
        let mut window_groups = project_window_groups(&windows, &[]);
        strengthen_groups_from_behaviour(&mut window_groups, &behaviour);
        let state_id = format!("workspace-state:{}:{}", snapshot.pass.id, Uuid::new_v4());

        Self {
            metadata: WorkspaceStateMetadata {
                state_id,
                created_at,
                observation_pass_id: Some(snapshot.pass.id.clone()),
                latest_delta_reference: delta_reference(delta),
                window_count: snapshot.pass.window_count,
                monitor_count: snapshot.pass.monitor_count,
                has_changes: delta.has_changes,
                authority_effect: WorkspaceStateMetadata::AUTHORITY_EFFECT_NONE.into(),
            },
            focused_window,
            active_applications,
            windows,
            monitors,
            window_groups,
            latest_delta: delta.clone(),
            behaviour,
            runtime_memory: DesktopRuntimeMemory::empty(),
            semantics: DesktopSemanticProjection::empty(),
            decisions: DesktopDecisionProjection::empty(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Test / fixture helper: build state from already-projected window rows.
    pub fn from_windows(windows: Vec<WorkspaceStateWindow>) -> Self {
        let focused_window = windows
            .iter()
            .find(|window| window.focused)
            .map(WorkspaceStateWindow::to_window_ref);
        let active_applications = active_applications_from_state_windows(&windows);
        let window_groups = project_window_groups(&windows, &[]);
        let window_count = windows.len() as i32;
        Self {
            metadata: WorkspaceStateMetadata {
                state_id: format!("workspace-state:fixture:{}", Uuid::new_v4()),
                created_at: observation_now_rfc3339(),
                observation_pass_id: None,
                latest_delta_reference: None,
                window_count,
                monitor_count: 0,
                has_changes: false,
                authority_effect: WorkspaceStateMetadata::AUTHORITY_EFFECT_NONE.into(),
            },
            focused_window,
            active_applications,
            windows,
            monitors: Vec::new(),
            window_groups,
            latest_delta: WorkspaceObservationDelta::empty(),
            behaviour: DesktopBehaviourTimeline::empty(),
            runtime_memory: DesktopRuntimeMemory::empty(),
            semantics: DesktopSemanticProjection::empty(),
            decisions: DesktopDecisionProjection::empty(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Re-project `window_groups` including arrangement-membership facts.
    ///
    /// `membership` rows are `(member_id, arrangement_id, arrangement_name)`.
    /// Member ids match [`WorkspaceStateWindow::member_id`] (stable id or hwnd).
    pub fn with_arrangement_membership(
        mut self,
        membership: &[(String, String, String)],
    ) -> Self {
        self.window_groups = project_window_groups(&self.windows, membership);
        strengthen_groups_from_behaviour(&mut self.window_groups, &self.behaviour);
        self
    }

    /// Attach live identity-registry continuity facts onto matching windows.
    ///
    /// Matches on `stable_window_id` == identity `id`. Does not invent history.
    pub fn with_identity_continuity(
        mut self,
        identities: &[ObservationWindowIdentity],
    ) -> Self {
        use std::collections::HashMap;
        let by_id: HashMap<&str, &ObservationWindowIdentity> = identities
            .iter()
            .map(|identity| (identity.id.as_str(), identity))
            .collect();
        for window in &mut self.windows {
            let Some(stable) = window
                .stable_window_id
                .as_ref()
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
            else {
                continue;
            };
            let Some(identity) = by_id.get(stable) else {
                continue;
            };
            window.first_seen_at = Some(identity.first_seen_at.clone());
            window.last_seen_at = Some(identity.last_seen_at.clone());
            window.identity_confidence = Some(identity.confidence.as_str().into());
        }
        self
    }

    /// Project identity-keyed runtime memory (present / absent / returning).
    ///
    /// Uses live identity rows + the same retained history that feeds behaviour.
    pub fn with_runtime_memory(
        mut self,
        identities: &[ObservationWindowIdentity],
        history: &[WorkspaceObservationSnapshot],
    ) -> Self {
        self.runtime_memory =
            project_desktop_runtime_memory(history, identities, &self.behaviour);
        strengthen_groups_from_runtime_memory(&mut self.window_groups, &self.runtime_memory);
        self.semantics = project_desktop_semantics(
            &self.behaviour,
            &self.runtime_memory,
            &self.window_groups,
        );
        self.decisions = project_desktop_decisions(
            &self.behaviour,
            &self.runtime_memory,
            &self.semantics,
        );
        self
    }

    /// Fixture window builder for Environment / Composition tests.
    pub fn fixture_window(
        hwnd: impl Into<String>,
        title: impl Into<String>,
        process_id: i32,
        focused: bool,
    ) -> WorkspaceStateWindow {
        WorkspaceStateWindow {
            stable_window_id: None,
            hwnd: hwnd.into(),
            title: title.into(),
            process_id,
            process_name: None,
            visible: true,
            focused,
            minimized: false,
            z_order: None,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            monitor_index: Some(0),
            monitor_name: Some("Primary".into()),
            first_seen_at: None,
            last_seen_at: None,
            identity_confidence: None,
        }
    }
}

/// Project fact-driven groups. Membership rows: `(member_id, arrangement_id, name)`.
fn project_window_groups(
    windows: &[WorkspaceStateWindow],
    membership: &[(String, String, String)],
) -> Vec<DesktopWindowGroup> {
    use std::collections::{BTreeMap, BTreeSet};

    let mut ids_by_member: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut names_by_arrangement: BTreeMap<String, String> = BTreeMap::new();
    for (member_id, arrangement_id, arrangement_name) in membership {
        let member_key = member_id.trim();
        let arrangement_key = arrangement_id.trim();
        if member_key.is_empty() || arrangement_key.is_empty() {
            continue;
        }
        ids_by_member
            .entry(member_key.to_string())
            .or_default()
            .insert(arrangement_key.to_string());
        names_by_arrangement
            .entry(arrangement_key.to_string())
            .or_insert_with(|| {
                let name = arrangement_name.trim();
                if name.is_empty() {
                    arrangement_key.to_string()
                } else {
                    name.to_string()
                }
            });
    }

    let members: Vec<DesktopGroupMemberFact> = windows
        .iter()
        .map(|window| {
            let mut fact = window.to_group_member_fact();
            let mut arrangement_ids = BTreeSet::new();
            if let Some(ids) = ids_by_member.get(&fact.member_id) {
                arrangement_ids.extend(ids.iter().cloned());
            }
            if fact.member_id != window.hwnd {
                if let Some(ids) = ids_by_member.get(&window.hwnd) {
                    arrangement_ids.extend(ids.iter().cloned());
                }
            }
            fact.arrangement_ids = arrangement_ids.into_iter().collect();
            fact
        })
        .collect();

    let mut groups = group_desktop_members(
        &members,
        &[
            DesktopGroupCriterion::ProcessId,
            DesktopGroupCriterion::MonitorIndex,
            DesktopGroupCriterion::ArrangementMembership,
        ],
    );
    for group in &mut groups {
        if group.criterion == DesktopGroupCriterion::ArrangementMembership.as_str() {
            if let Some(name) = names_by_arrangement.get(&group.fact_key) {
                group.label = name.clone();
            }
        }
    }
    groups
}

fn delta_reference(delta: &WorkspaceObservationDelta) -> Option<String> {
    match (
        delta.previous_pass_id.as_deref(),
        delta.current_pass_id.as_deref(),
    ) {
        (Some(previous), Some(current)) => Some(format!("{previous}->{current}")),
        (None, Some(current)) => Some(format!("->{current}")),
        _ => None,
    }
}

fn active_applications_from_state_windows(
    windows: &[WorkspaceStateWindow],
) -> Vec<WorkspaceActiveApplication> {
    let mut apps: Vec<WorkspaceActiveApplication> = Vec::new();
    for window in windows {
        if let Some(existing) = apps
            .iter_mut()
            .find(|app| app.process_id == window.process_id)
        {
            existing.window_count += 1;
            if existing.process_name.is_none() {
                existing.process_name = window.process_name.clone();
            }
        } else {
            apps.push(WorkspaceActiveApplication {
                process_id: window.process_id,
                process_name: window.process_name.clone(),
                window_count: 1,
            });
        }
    }
    apps.sort_by_key(|app| app.process_id);
    apps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };

    fn snapshot_with_focus() -> WorkspaceObservationSnapshot {
        let pass_id = "pass-1";
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: "2026-07-26T12:00:00Z".into(),
                schema_version: 1,
                source: "test".into(),
                foreground_hwnd: Some("0x1".into()),
                window_count: 2,
                monitor_count: 1,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: "mon-0".into(),
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
                    id: "w1".into(),
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
                    monitor_id: Some("mon-0".into()),
                    visible: true,
                    minimized: false,
                    focused: true,
                    z_order: Some(0),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                },
                ObservedWindow {
                    id: "w2".into(),
                    pass_id: pass_id.into(),
                    hwnd: "0x2".into(),
                    stable_window_id: Some("stable-b".into()),
                    title: "Beta".into(),
                    process_id: 10,
                    process_name: Some("alpha.exe".into()),
                    x: 10,
                    y: 10,
                    width: 50,
                    height: 50,
                    monitor_id: Some("mon-0".into()),
                    visible: true,
                    minimized: false,
                    focused: false,
                    z_order: Some(1),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                },
            ],
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn empty_state_has_no_observation() {
        let state = WorkspaceState::from_observation_and_delta(
            None,
            &WorkspaceObservationDelta::empty(),
        );
        assert!(state.metadata.observation_pass_id.is_none());
        assert_eq!(state.metadata.window_count, 0);
        assert!(state.focused_window.is_none());
        assert!(state.windows.is_empty());
        assert!(!state.metadata.has_changes);
        assert_eq!(state.authority_effect, "none");
    }

    #[test]
    fn builds_from_observation_with_focus_apps_and_windows() {
        let snapshot = snapshot_with_focus();
        let delta = WorkspaceObservationDelta::empty_with_current(&snapshot);
        let state = WorkspaceState::from_observation_and_delta(Some(&snapshot), &delta);
        assert_eq!(state.metadata.observation_pass_id.as_deref(), Some("pass-1"));
        assert_eq!(state.metadata.window_count, 2);
        assert_eq!(state.windows.len(), 2);
        assert_eq!(state.windows[0].monitor_name.as_deref(), Some("Primary"));
        assert_eq!(state.monitors.len(), 1);
        assert_eq!(state.monitors[0].name, "Primary");
        assert!(state.monitors[0].is_primary);
        assert_eq!(
            state.focused_window.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-a")
        );
        assert_eq!(state.active_applications.len(), 1);
        assert_eq!(state.active_applications[0].window_count, 2);
        assert!(state
            .window_groups
            .iter()
            .any(|group| group.criterion == "process_id" && group.member_ids.len() == 2));
        assert!(state
            .window_groups
            .iter()
            .any(|group| group.criterion == "monitor_index"));
        assert_eq!(
            state.latest_delta.current_pass_id.as_deref(),
            Some("pass-1")
        );
        assert!(!state.latest_delta.has_changes);
    }

    #[test]
    fn arrangement_membership_enriches_window_groups() {
        let snapshot = snapshot_with_focus();
        let delta = WorkspaceObservationDelta::empty_with_current(&snapshot);
        let state = WorkspaceState::from_observation_and_delta(Some(&snapshot), &delta)
            .with_arrangement_membership(&[
                ("stable-a".into(), "arr-1".into(), "Focus set".into()),
                ("stable-b".into(), "arr-1".into(), "Focus set".into()),
            ]);
        let arrangement = state
            .window_groups
            .iter()
            .find(|group| group.criterion == "arrangement_membership")
            .expect("arrangement group");
        assert_eq!(arrangement.fact_key, "arr-1");
        assert_eq!(arrangement.label, "Focus set");
        assert_eq!(arrangement.member_ids, vec!["stable-a", "stable-b"]);
    }

    #[test]
    fn identity_continuity_attaches_live_registry_facts() {
        use crate::workspace_observation::WindowIdentityConfidence;
        let snapshot = snapshot_with_focus();
        let delta = WorkspaceObservationDelta::empty_with_current(&snapshot);
        let identities = vec![ObservationWindowIdentity {
            id: "stable-a".into(),
            process_id: 10,
            title_fingerprint: "alpha".into(),
            first_seen_at: "2026-07-26T11:00:00Z".into(),
            last_seen_at: "2026-07-26T12:00:00Z".into(),
            last_hwnd: "0x1".into(),
            confidence: WindowIdentityConfidence::High,
            authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
        }];
        let state = WorkspaceState::from_observation_and_delta(Some(&snapshot), &delta)
            .with_identity_continuity(&identities);
        let alpha = state
            .windows
            .iter()
            .find(|window| window.stable_window_id.as_deref() == Some("stable-a"))
            .expect("alpha");
        assert_eq!(alpha.first_seen_at.as_deref(), Some("2026-07-26T11:00:00Z"));
        assert_eq!(alpha.last_seen_at.as_deref(), Some("2026-07-26T12:00:00Z"));
        assert_eq!(alpha.identity_confidence.as_deref(), Some("high"));
        let beta = state
            .windows
            .iter()
            .find(|window| window.stable_window_id.as_deref() == Some("stable-b"))
            .expect("beta");
        assert!(beta.first_seen_at.is_none());
    }

    #[test]
    fn propagates_delta_has_changes() {
        let snapshot = snapshot_with_focus();
        let mut delta = WorkspaceObservationDelta::empty_with_current(&snapshot);
        delta.previous_pass_id = Some("pass-0".into());
        delta.has_changes = true;
        delta.opened_windows.push(ObservationWindowRef {
            stable_window_id: Some("stable-c".into()),
            hwnd: "0x3".into(),
            title: "Gamma".into(),
            process_id: 99,
        });
        let state = WorkspaceState::from_observation_and_delta(Some(&snapshot), &delta);
        assert!(state.metadata.has_changes);
        assert_eq!(
            state.metadata.latest_delta_reference.as_deref(),
            Some("pass-0->pass-1")
        );
    }
}
