//! Workspace Observation Delta — pure comparison of two observation snapshots (Sprint 116).
//!
//! Facts only: no capture, no decisions, no automation, no authority.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::workspace_observation::{
    ObservedMonitor, ObservedWindow, WorkspaceObservationSnapshot,
};

/// Lightweight window identity for delta entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationWindowRef {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub process_id: i32,
}

impl ObservationWindowRef {
    pub fn from_window(window: &ObservedWindow) -> Self {
        Self {
            stable_window_id: window.stable_window_id.clone(),
            hwnd: window.hwnd.clone(),
            title: window.title.clone(),
            process_id: window.process_id,
        }
    }
}

/// Focus transition between two snapshots (at most one focused window each).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationFocusedWindowChange {
    pub previous: Option<ObservationWindowRef>,
    pub current: Option<ObservationWindowRef>,
}

/// Geometry move (x/y) for a matched window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationWindowMove {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
}

/// Geometry resize (width/height) for a matched window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationWindowResize {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub from_width: i32,
    pub from_height: i32,
    pub to_width: i32,
    pub to_height: i32,
}

/// Minimized ↔ restored transition for a matched window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationMinimizedChange {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub was_minimized: bool,
    pub is_minimized: bool,
}

/// Monitor assignment change for a matched window (by monitor_index, not pass-scoped id).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationMonitorAssignmentChange {
    pub stable_window_id: Option<String>,
    pub hwnd: String,
    pub title: String,
    pub from_monitor_index: Option<i32>,
    pub to_monitor_index: Option<i32>,
    pub from_monitor_name: Option<String>,
    pub to_monitor_name: Option<String>,
}

/// Canonical description of what changed between two observation snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceObservationDelta {
    pub previous_pass_id: Option<String>,
    pub current_pass_id: Option<String>,
    pub previous_captured_at: Option<String>,
    pub current_captured_at: Option<String>,
    pub opened_windows: Vec<ObservationWindowRef>,
    pub closed_windows: Vec<ObservationWindowRef>,
    pub focused_window_changed: Option<ObservationFocusedWindowChange>,
    pub moved_windows: Vec<ObservationWindowMove>,
    pub resized_windows: Vec<ObservationWindowResize>,
    pub minimized_changes: Vec<ObservationMinimizedChange>,
    pub monitor_changes: Vec<ObservationMonitorAssignmentChange>,
    pub has_changes: bool,
    pub authority_effect: String,
}

impl WorkspaceObservationDelta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Empty / no-change delta (fewer than two snapshots, or identical).
    pub fn empty() -> Self {
        Self {
            previous_pass_id: None,
            current_pass_id: None,
            previous_captured_at: None,
            current_captured_at: None,
            opened_windows: Vec::new(),
            closed_windows: Vec::new(),
            focused_window_changed: None,
            moved_windows: Vec::new(),
            resized_windows: Vec::new(),
            minimized_changes: Vec::new(),
            monitor_changes: Vec::new(),
            has_changes: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Empty delta annotated with the single available pass (still no changes).
    pub fn empty_with_current(current: &WorkspaceObservationSnapshot) -> Self {
        let mut delta = Self::empty();
        delta.current_pass_id = Some(current.pass.id.clone());
        delta.current_captured_at = Some(current.pass.captured_at.clone());
        delta
    }

    fn finalize(mut self) -> Self {
        self.has_changes = !self.opened_windows.is_empty()
            || !self.closed_windows.is_empty()
            || self.focused_window_changed.is_some()
            || !self.moved_windows.is_empty()
            || !self.resized_windows.is_empty()
            || !self.minimized_changes.is_empty()
            || !self.monitor_changes.is_empty();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self
    }
}

/// Pure comparison of two persisted observation snapshots.
pub fn compare_observation_snapshots(
    previous: &WorkspaceObservationSnapshot,
    current: &WorkspaceObservationSnapshot,
) -> WorkspaceObservationDelta {
    let previous_index = index_windows(&previous.windows);
    let current_index = index_windows(&current.windows);
    let previous_monitors = index_monitors(&previous.monitors);
    let current_monitors = index_monitors(&current.monitors);

    let mut opened_windows = Vec::new();
    let mut closed_windows = Vec::new();
    let mut moved_windows = Vec::new();
    let mut resized_windows = Vec::new();
    let mut minimized_changes = Vec::new();
    let mut monitor_changes = Vec::new();

    for (key, window) in &current_index {
        if !previous_index.contains_key(key) {
            opened_windows.push(ObservationWindowRef::from_window(window));
        }
    }

    for (key, window) in &previous_index {
        if !current_index.contains_key(key) {
            closed_windows.push(ObservationWindowRef::from_window(window));
        }
    }

    for (key, current_window) in &current_index {
        let Some(previous_window) = previous_index.get(key) else {
            continue;
        };

        if previous_window.x != current_window.x || previous_window.y != current_window.y {
            moved_windows.push(ObservationWindowMove {
                stable_window_id: current_window.stable_window_id.clone(),
                hwnd: current_window.hwnd.clone(),
                title: current_window.title.clone(),
                from_x: previous_window.x,
                from_y: previous_window.y,
                to_x: current_window.x,
                to_y: current_window.y,
            });
        }

        if previous_window.width != current_window.width
            || previous_window.height != current_window.height
        {
            resized_windows.push(ObservationWindowResize {
                stable_window_id: current_window.stable_window_id.clone(),
                hwnd: current_window.hwnd.clone(),
                title: current_window.title.clone(),
                from_width: previous_window.width,
                from_height: previous_window.height,
                to_width: current_window.width,
                to_height: current_window.height,
            });
        }

        if previous_window.minimized != current_window.minimized {
            minimized_changes.push(ObservationMinimizedChange {
                stable_window_id: current_window.stable_window_id.clone(),
                hwnd: current_window.hwnd.clone(),
                title: current_window.title.clone(),
                was_minimized: previous_window.minimized,
                is_minimized: current_window.minimized,
            });
        }

        let from_mon = resolve_monitor(previous_window, &previous_monitors);
        let to_mon = resolve_monitor(current_window, &current_monitors);
        if from_mon.index != to_mon.index || from_mon.name != to_mon.name {
            monitor_changes.push(ObservationMonitorAssignmentChange {
                stable_window_id: current_window.stable_window_id.clone(),
                hwnd: current_window.hwnd.clone(),
                title: current_window.title.clone(),
                from_monitor_index: from_mon.index,
                to_monitor_index: to_mon.index,
                from_monitor_name: from_mon.name,
                to_monitor_name: to_mon.name,
            });
        }
    }

    let previous_focus = focused_window(&previous.windows);
    let current_focus = focused_window(&current.windows);
    let focused_window_changed = if window_identity_key_opt(previous_focus)
        != window_identity_key_opt(current_focus)
    {
        Some(ObservationFocusedWindowChange {
            previous: previous_focus.map(ObservationWindowRef::from_window),
            current: current_focus.map(ObservationWindowRef::from_window),
        })
    } else {
        None
    };

    // Stable ordering for deterministic tests / IPC.
    opened_windows.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));
    closed_windows.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));
    moved_windows.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));
    resized_windows.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));
    minimized_changes.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));
    monitor_changes.sort_by(|a, b| a.hwnd.cmp(&b.hwnd));

    WorkspaceObservationDelta {
        previous_pass_id: Some(previous.pass.id.clone()),
        current_pass_id: Some(current.pass.id.clone()),
        previous_captured_at: Some(previous.pass.captured_at.clone()),
        current_captured_at: Some(current.pass.captured_at.clone()),
        opened_windows,
        closed_windows,
        focused_window_changed,
        moved_windows,
        resized_windows,
        minimized_changes,
        monitor_changes,
        has_changes: false,
        authority_effect: WorkspaceObservationDelta::AUTHORITY_EFFECT_NONE.into(),
    }
    .finalize()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WindowIdentityKey {
    Stable(String),
    Hwnd(String),
}

fn window_identity_key(window: &ObservedWindow) -> WindowIdentityKey {
    match window
        .stable_window_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(stable) => WindowIdentityKey::Stable(stable.to_string()),
        None => WindowIdentityKey::Hwnd(window.hwnd.clone()),
    }
}

fn window_identity_key_opt(window: Option<&ObservedWindow>) -> Option<WindowIdentityKey> {
    window.map(window_identity_key)
}

fn index_windows(windows: &[ObservedWindow]) -> HashMap<WindowIdentityKey, &ObservedWindow> {
    let mut map = HashMap::new();
    for window in windows {
        map.insert(window_identity_key(window), window);
    }
    map
}

fn index_monitors(monitors: &[ObservedMonitor]) -> HashMap<&str, &ObservedMonitor> {
    monitors
        .iter()
        .map(|monitor| (monitor.id.as_str(), monitor))
        .collect()
}

struct ResolvedMonitor {
    index: Option<i32>,
    name: Option<String>,
}

fn resolve_monitor(
    window: &ObservedWindow,
    monitors: &HashMap<&str, &ObservedMonitor>,
) -> ResolvedMonitor {
    let Some(monitor_id) = window.monitor_id.as_deref() else {
        return ResolvedMonitor {
            index: None,
            name: None,
        };
    };
    match monitors.get(monitor_id) {
        Some(monitor) => ResolvedMonitor {
            index: Some(monitor.monitor_index),
            name: Some(monitor.name.clone()),
        },
        None => ResolvedMonitor {
            index: None,
            name: None,
        },
    }
}

fn focused_window(windows: &[ObservedWindow]) -> Option<&ObservedWindow> {
    windows.iter().find(|window| window.focused)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };

    fn base_snapshot(pass_id: &str, captured_at: &str) -> WorkspaceObservationSnapshot {
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test".into(),
                foreground_hwnd: Some("0x1".into()),
                window_count: 1,
                monitor_count: 2,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![
                ObservedMonitor {
                    id: format!("{pass_id}-mon-0"),
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
                },
                ObservedMonitor {
                    id: format!("{pass_id}-mon-1"),
                    pass_id: pass_id.into(),
                    monitor_index: 1,
                    name: "Secondary".into(),
                    x: 1920,
                    y: 0,
                    width: 1920,
                    height: 1080,
                    work_x: 1920,
                    work_y: 0,
                    work_w: 1920,
                    work_h: 1040,
                    is_primary: false,
                    dpi_scale: None,
                    authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
                },
            ],
            windows: vec![ObservedWindow {
                id: format!("{pass_id}-win-a"),
                pass_id: pass_id.into(),
                hwnd: "0x1".into(),
                stable_window_id: Some("stable-a".into()),
                title: "Alpha".into(),
                process_id: 10,
                process_name: None,
                x: 100,
                y: 100,
                width: 800,
                height: 600,
                monitor_id: Some(format!("{pass_id}-mon-0")),
                visible: true,
                minimized: false,
                focused: true,
                z_order: Some(0),
                authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
            }],
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn unchanged_snapshots_produce_empty_delta() {
        let previous = base_snapshot("pass-1", "2026-07-26T10:00:00Z");
        let mut current = base_snapshot("pass-2", "2026-07-26T10:01:00Z");
        // Same logical window/monitor assignment via stable id + monitor_index.
        current.windows[0].x = 100;
        current.windows[0].y = 100;
        let delta = compare_observation_snapshots(&previous, &current);
        assert!(!delta.has_changes);
        assert!(delta.opened_windows.is_empty());
        assert!(delta.closed_windows.is_empty());
        assert!(delta.focused_window_changed.is_none());
        assert_eq!(delta.previous_pass_id.as_deref(), Some("pass-1"));
        assert_eq!(delta.current_pass_id.as_deref(), Some("pass-2"));
    }

    #[test]
    fn detects_opened_and_closed_windows() {
        let previous = base_snapshot("pass-1", "2026-07-26T10:00:00Z");
        let mut current = base_snapshot("pass-2", "2026-07-26T10:01:00Z");
        current.windows.clear();
        current.windows.push(ObservedWindow {
            id: "pass-2-win-b".into(),
            pass_id: "pass-2".into(),
            hwnd: "0x2".into(),
            stable_window_id: Some("stable-b".into()),
            title: "Beta".into(),
            process_id: 20,
            process_name: None,
            x: 10,
            y: 10,
            width: 400,
            height: 300,
            monitor_id: Some("pass-2-mon-0".into()),
            visible: true,
            minimized: false,
            focused: true,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        current.pass.window_count = 1;
        current.pass.foreground_hwnd = Some("0x2".into());

        let delta = compare_observation_snapshots(&previous, &current);
        assert!(delta.has_changes);
        assert_eq!(delta.closed_windows.len(), 1);
        assert_eq!(
            delta.closed_windows[0].stable_window_id.as_deref(),
            Some("stable-a")
        );
        assert_eq!(delta.opened_windows.len(), 1);
        assert_eq!(
            delta.opened_windows[0].stable_window_id.as_deref(),
            Some("stable-b")
        );
        assert!(delta.focused_window_changed.is_some());
    }

    #[test]
    fn detects_focus_move_resize_minimize_monitor() {
        let previous = base_snapshot("pass-1", "2026-07-26T10:00:00Z");
        let mut current = base_snapshot("pass-2", "2026-07-26T10:01:00Z");
        current.windows[0].focused = false;
        current.windows.push(ObservedWindow {
            id: "pass-2-win-b".into(),
            pass_id: "pass-2".into(),
            hwnd: "0x2".into(),
            stable_window_id: Some("stable-b".into()),
            title: "Beta".into(),
            process_id: 20,
            process_name: None,
            x: 50,
            y: 60,
            width: 400,
            height: 300,
            monitor_id: Some("pass-2-mon-0".into()),
            visible: true,
            minimized: false,
            focused: true,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        // Mutate stable-a: move, resize, minimize, monitor transition.
        current.windows[0].x = 200;
        current.windows[0].y = 220;
        current.windows[0].width = 900;
        current.windows[0].height = 700;
        current.windows[0].minimized = true;
        current.windows[0].visible = false;
        current.windows[0].monitor_id = Some("pass-2-mon-1".into());
        current.pass.window_count = 2;
        current.pass.foreground_hwnd = Some("0x2".into());

        let delta = compare_observation_snapshots(&previous, &current);
        assert!(delta.has_changes);
        assert_eq!(delta.opened_windows.len(), 1);
        assert_eq!(delta.moved_windows.len(), 1);
        assert_eq!(delta.moved_windows[0].to_x, 200);
        assert_eq!(delta.resized_windows.len(), 1);
        assert_eq!(delta.resized_windows[0].to_width, 900);
        assert_eq!(delta.minimized_changes.len(), 1);
        assert!(delta.minimized_changes[0].is_minimized);
        assert_eq!(delta.monitor_changes.len(), 1);
        assert_eq!(delta.monitor_changes[0].from_monitor_index, Some(0));
        assert_eq!(delta.monitor_changes[0].to_monitor_index, Some(1));
        let focus = delta.focused_window_changed.expect("focus change");
        assert_eq!(
            focus.previous.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-a")
        );
        assert_eq!(
            focus.current.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-b")
        );
    }
}
