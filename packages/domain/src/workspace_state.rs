//! Workspace State — canonical runtime projection (Sprint 118).
//!
//! Observation = facts, Delta = change, WorkspaceState = current interpreted state.
//! Read-only projection only — no AI, automation, capture, or Win32.
//!
//! Distinct from the kernel lifecycle `WorkspaceState` (version/lifecycle).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::workspace_observation::{
    observation_now_rfc3339, ObservedWindow, WorkspaceObservationSnapshot,
};
use crate::workspace_observation_delta::{ObservationWindowRef, WorkspaceObservationDelta};

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

/// Active application summary derived from observed windows (not duplicated window rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActiveApplication {
    pub process_id: i32,
    pub process_name: Option<String>,
    pub window_count: i32,
}

/// Canonical runtime state projection from latest observation + delta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub metadata: WorkspaceStateMetadata,
    pub focused_window: Option<ObservationWindowRef>,
    pub active_applications: Vec<WorkspaceActiveApplication>,
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
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Build projected state from optional latest observation and latest delta.
    pub fn from_observation_and_delta(
        observation: Option<&WorkspaceObservationSnapshot>,
        delta: &WorkspaceObservationDelta,
    ) -> Self {
        let created_at = observation_now_rfc3339();
        let Some(snapshot) = observation else {
            let mut empty = Self::empty();
            empty.metadata.created_at = created_at;
            empty.metadata.has_changes = delta.has_changes;
            empty.metadata.latest_delta_reference = delta_reference(delta);
            return empty;
        };

        let focused_window = snapshot
            .windows
            .iter()
            .find(|window| window.focused)
            .map(ObservationWindowRef::from_window);

        let active_applications = active_applications_from_windows(&snapshot.windows);
        let state_id = format!(
            "workspace-state:{}:{}",
            snapshot.pass.id,
            Uuid::new_v4()
        );

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
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
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

fn active_applications_from_windows(windows: &[ObservedWindow]) -> Vec<WorkspaceActiveApplication> {
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
        assert!(!state.metadata.has_changes);
        assert_eq!(state.authority_effect, "none");
    }

    #[test]
    fn builds_from_observation_with_focus_and_apps() {
        let snapshot = snapshot_with_focus();
        let delta = WorkspaceObservationDelta::empty_with_current(&snapshot);
        let state = WorkspaceState::from_observation_and_delta(Some(&snapshot), &delta);
        assert_eq!(state.metadata.observation_pass_id.as_deref(), Some("pass-1"));
        assert_eq!(state.metadata.window_count, 2);
        assert_eq!(state.metadata.monitor_count, 1);
        assert_eq!(
            state.focused_window.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-a")
        );
        assert_eq!(state.active_applications.len(), 1);
        assert_eq!(state.active_applications[0].process_id, 10);
        assert_eq!(state.active_applications[0].window_count, 2);
        assert_eq!(state.metadata.latest_delta_reference.as_deref(), Some("->pass-1"));
        assert!(!state.metadata.has_changes);
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
