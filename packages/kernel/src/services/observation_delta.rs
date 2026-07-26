//! Observation delta service (Sprint 116).
//!
//! Pure read path: load latest + previous snapshots, compare, return facts.
//! Never captures, writes, or decides.

use std::sync::{Arc, Mutex};

use workspace_database::{Database, ObservationPassRepository};
use workspace_domain::{
    compare_observation_snapshots, ActorContext, IntentContext, WorkspaceObservationDelta,
};

use crate::error::Result;

/// Read-only observation change facts between consecutive snapshots.
pub(crate) struct ObservationDeltaService;

impl ObservationDeltaService {
    /// Compare two in-memory snapshots (pure; no I/O).
    pub(crate) fn compare(
        previous: &workspace_domain::WorkspaceObservationSnapshot,
        current: &workspace_domain::WorkspaceObservationSnapshot,
    ) -> WorkspaceObservationDelta {
        compare_observation_snapshots(previous, current)
    }

    /// Latest vs immediately previous persisted snapshot.
    ///
    /// Fewer than two snapshots → empty / no-change delta (not an error).
    pub(crate) fn get_latest(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        _intent: &IntentContext,
    ) -> Result<WorkspaceObservationDelta> {
        let guard = db.lock().expect("database lock poisoned");
        let repo = ObservationPassRepository::new(&guard);
        let latest = repo.load_latest_snapshot()?;
        let previous = repo.load_previous_snapshot()?;
        Ok(match (previous, latest) {
            (Some(previous), Some(current)) => compare_observation_snapshots(&previous, &current),
            (None, Some(current)) => WorkspaceObservationDelta::empty_with_current(&current),
            _ => WorkspaceObservationDelta::empty(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{
        ActorContext, IntentContext, ObservedMonitor, ObservedWindow, WorkspaceObservationPass,
        WorkspaceObservationSnapshot,
    };

    use crate::WorkspaceKernel;

    fn snapshot(
        pass_id: &str,
        captured_at: &str,
        windows: Vec<ObservedWindow>,
        monitors: Vec<ObservedMonitor>,
    ) -> WorkspaceObservationSnapshot {
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test_inject".into(),
                foreground_hwnd: windows.iter().find(|w| w.focused).map(|w| w.hwnd.clone()),
                window_count: windows.len() as i32,
                monitor_count: monitors.len() as i32,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            windows,
            monitors,
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn primary_monitor(pass_id: &str) -> ObservedMonitor {
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
        }
    }

    fn secondary_monitor(pass_id: &str) -> ObservedMonitor {
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
        }
    }

    fn window(
        pass_id: &str,
        id: &str,
        hwnd: &str,
        stable: &str,
        title: &str,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        monitor_id: &str,
        minimized: bool,
        focused: bool,
    ) -> ObservedWindow {
        ObservedWindow {
            id: id.into(),
            pass_id: pass_id.into(),
            hwnd: hwnd.into(),
            stable_window_id: Some(stable.into()),
            title: title.into(),
            process_id: 42,
            process_name: None,
            x,
            y,
            width: w,
            height: h,
            monitor_id: Some(monitor_id.into()),
            visible: !minimized,
            minimized,
            focused,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn persist(kernel: &WorkspaceKernel, snap: &WorkspaceObservationSnapshot) {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        ObservationPassRepository::new(&guard)
            .insert_snapshot(snap)
            .unwrap();
    }

    #[test]
    fn first_snapshot_returns_empty_delta() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let pass = "pass-1";
        persist(
            &kernel,
            &snapshot(
                pass,
                "2026-07-26T10:00:00Z",
                vec![window(
                    pass,
                    "w1",
                    "0x1",
                    "stable-a",
                    "Alpha",
                    10,
                    10,
                    100,
                    100,
                    &format!("{pass}-mon-0"),
                    false,
                    true,
                )],
                vec![primary_monitor(pass)],
            ),
        );

        let delta = ObservationDeltaService::get_latest(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert!(!delta.has_changes);
        assert_eq!(delta.current_pass_id.as_deref(), Some("pass-1"));
        assert!(delta.previous_pass_id.is_none());
        assert!(delta.opened_windows.is_empty());
    }

    #[test]
    fn no_snapshots_returns_empty_delta() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let delta = ObservationDeltaService::get_latest(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert_eq!(delta, WorkspaceObservationDelta::empty());
    }

    #[test]
    fn compare_delegates_to_domain() {
        let pass = "pass-1";
        let snap = snapshot(
            pass,
            "2026-07-26T10:00:00Z",
            vec![window(
                pass,
                "w1",
                "0x1",
                "stable-a",
                "Alpha",
                10,
                10,
                100,
                100,
                &format!("{pass}-mon-0"),
                false,
                true,
            )],
            vec![primary_monitor(pass)],
        );
        let delta = ObservationDeltaService::compare(&snap, &snap);
        assert!(!delta.has_changes);
    }

    #[test]
    fn detects_opened_closed_focus_geometry_minimize_monitor() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let p1 = "pass-1";
        persist(
            &kernel,
            &snapshot(
                p1,
                "2026-07-26T10:00:00Z",
                vec![
                    window(
                        p1,
                        "w-a",
                        "0x1",
                        "stable-a",
                        "Alpha",
                        10,
                        10,
                        800,
                        600,
                        &format!("{p1}-mon-0"),
                        false,
                        true,
                    ),
                    window(
                        p1,
                        "w-c",
                        "0x3",
                        "stable-c",
                        "Closing",
                        1,
                        1,
                        50,
                        50,
                        &format!("{p1}-mon-0"),
                        false,
                        false,
                    ),
                ],
                vec![primary_monitor(p1), secondary_monitor(p1)],
            ),
        );

        let p2 = "pass-2";
        persist(
            &kernel,
            &snapshot(
                p2,
                "2026-07-26T10:01:00Z",
                vec![
                    window(
                        p2,
                        "w-a2",
                        "0x1",
                        "stable-a",
                        "Alpha",
                        40,
                        50,
                        900,
                        700,
                        &format!("{p2}-mon-1"),
                        true,
                        false,
                    ),
                    window(
                        p2,
                        "w-b",
                        "0x2",
                        "stable-b",
                        "Beta",
                        100,
                        100,
                        400,
                        300,
                        &format!("{p2}-mon-0"),
                        false,
                        true,
                    ),
                ],
                vec![primary_monitor(p2), secondary_monitor(p2)],
            ),
        );

        let delta = ObservationDeltaService::get_latest(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();

        assert!(delta.has_changes);
        assert_eq!(delta.previous_pass_id.as_deref(), Some("pass-1"));
        assert_eq!(delta.current_pass_id.as_deref(), Some("pass-2"));
        assert_eq!(delta.opened_windows.len(), 1);
        assert_eq!(
            delta.opened_windows[0].stable_window_id.as_deref(),
            Some("stable-b")
        );
        assert_eq!(delta.closed_windows.len(), 1);
        assert_eq!(
            delta.closed_windows[0].stable_window_id.as_deref(),
            Some("stable-c")
        );
        assert_eq!(delta.moved_windows.len(), 1);
        assert_eq!(delta.resized_windows.len(), 1);
        assert_eq!(delta.minimized_changes.len(), 1);
        assert!(delta.minimized_changes[0].is_minimized);
        assert_eq!(delta.monitor_changes.len(), 1);
        assert_eq!(delta.monitor_changes[0].to_monitor_index, Some(1));
        let focus = delta.focused_window_changed.expect("focus");
        assert_eq!(
            focus.previous.unwrap().stable_window_id.as_deref(),
            Some("stable-a")
        );
        assert_eq!(
            focus.current.unwrap().stable_window_id.as_deref(),
            Some("stable-b")
        );
    }

    #[test]
    fn unchanged_consecutive_snapshots() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        for (pass, at) in [("pass-1", "2026-07-26T10:00:00Z"), ("pass-2", "2026-07-26T10:01:00Z")]
        {
            persist(
                &kernel,
                &snapshot(
                    pass,
                    at,
                    vec![window(
                        pass,
                        &format!("{pass}-w"),
                        "0x1",
                        "stable-a",
                        "Alpha",
                        10,
                        10,
                        100,
                        100,
                        &format!("{pass}-mon-0"),
                        false,
                        true,
                    )],
                    vec![primary_monitor(pass)],
                ),
            );
        }

        let delta = ObservationDeltaService::get_latest(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert!(!delta.has_changes);
        assert!(delta.opened_windows.is_empty());
        assert!(delta.focused_window_changed.is_none());
    }
}
