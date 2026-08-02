//! Workspace State Engine (Sprint 118).
//!
//! Builds a canonical [`WorkspaceState`] from the latest persisted observation
//! and latest observation delta. Projection only — no persistence, capture,
//! scheduling, Win32, AI, or automation.
//!
//! ```text
//! latest observation + latest delta
//!         ↓
//! WorkspaceStateEngine
//!         ↓
//! WorkspaceState (interpreted runtime projection)
//! ```

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    ActorContext, IntentContext, WorkspaceObservationDelta, WorkspaceState,
};

use crate::error::Result;
use crate::services::WorkspaceRuntimeStateService;

/// Read-only projection engine for WorkspaceState.
pub(crate) struct WorkspaceStateEngine;

impl WorkspaceStateEngine {
    /// Pure construction from already-loaded inputs.
    pub(crate) fn build(
        observation: Option<&workspace_domain::WorkspaceObservationSnapshot>,
        delta: &WorkspaceObservationDelta,
    ) -> WorkspaceState {
        WorkspaceState::from_observation_and_delta(observation, delta)
    }

    /// Load latest observation + delta and project WorkspaceState.
    ///
    /// Routed through [`WorkspaceRuntimeStateService`] so all consumers share
    /// one cached desktop projection keyed by observation pass id.
    pub(crate) fn get_current(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceState> {
        WorkspaceRuntimeStateService::desktop_projection(db, actor, intent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::ObservationPassRepository;
    use workspace_domain::{
        ObservedMonitor, ObservedWindow, WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };

    use crate::WorkspaceKernel;
    use crate::services::WorkspaceRuntimeStateService;

    fn persist(kernel: &WorkspaceKernel, snap: &WorkspaceObservationSnapshot) {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        ObservationPassRepository::new(&guard)
            .insert_snapshot(snap)
            .unwrap();
    }

    fn snapshot(
        pass_id: &str,
        captured_at: &str,
        focused: bool,
    ) -> WorkspaceObservationSnapshot {
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test_inject".into(),
                foreground_hwnd: if focused {
                    Some("0x1".into())
                } else {
                    None
                },
                window_count: 1,
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
            windows: vec![ObservedWindow {
                id: format!("{pass_id}-win"),
                pass_id: pass_id.into(),
                hwnd: "0x1".into(),
                stable_window_id: Some("stable-a".into()),
                title: "Alpha".into(),
                process_id: 42,
                process_name: Some("alpha.exe".into()),
                x: 10,
                y: 10,
                width: 100,
                height: 100,
                monitor_id: Some(format!("{pass_id}-mon")),
                visible: true,
                minimized: false,
                focused,
                z_order: Some(0),
                authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
            }],
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn empty_state_with_no_observation() {
        WorkspaceRuntimeStateService::reset_for_tests();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let state = WorkspaceStateEngine::get_current(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert!(state.metadata.observation_pass_id.is_none());
        assert_eq!(state.metadata.window_count, 0);
        assert!(state.focused_window.is_none());
        assert!(!state.metadata.has_changes);
    }

    #[test]
    fn state_from_one_observation() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        persist(&kernel, &snapshot("pass-1", "2026-07-26T10:00:00Z", true));

        let state = WorkspaceStateEngine::get_current(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert_eq!(state.metadata.observation_pass_id.as_deref(), Some("pass-1"));
        assert_eq!(state.metadata.window_count, 1);
        assert_eq!(state.metadata.monitor_count, 1);
        assert_eq!(
            state.focused_window.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-a")
        );
        assert_eq!(state.active_applications.len(), 1);
        assert_eq!(
            state.active_applications[0].process_name.as_deref(),
            Some("alpha.exe")
        );
        assert!(!state.metadata.has_changes);
        assert_eq!(
            state.metadata.latest_delta_reference.as_deref(),
            Some("->pass-1")
        );
    }

    #[test]
    fn state_after_delta_reports_changes() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        persist(&kernel, &snapshot("pass-1", "2026-07-26T10:00:00Z", true));

        let mut second = snapshot("pass-2", "2026-07-26T10:01:00Z", true);
        second.windows[0].x = 200;
        second.windows[0].y = 200;
        persist(&kernel, &second);

        let state = WorkspaceStateEngine::get_current(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert_eq!(state.metadata.observation_pass_id.as_deref(), Some("pass-2"));
        assert!(state.metadata.has_changes);
        assert_eq!(
            state.metadata.latest_delta_reference.as_deref(),
            Some("pass-1->pass-2")
        );
        assert_eq!(
            state.focused_window.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-a")
        );
    }
}
