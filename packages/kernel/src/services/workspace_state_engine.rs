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

use workspace_database::{Database, DesktopArrangementRepository, ObservationPassRepository};
use workspace_domain::{
    ActorContext, IntentContext, WorkspaceObservationDelta, WorkspaceState,
};

use crate::error::Result;
use crate::services::ObservationDeltaService;

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
    pub(crate) fn get_current(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceState> {
        let delta = ObservationDeltaService::get_latest(db, actor, intent)?;
        let (observation, membership) = {
            let guard = db.lock().expect("database lock poisoned");
            let observation = ObservationPassRepository::new(&guard).load_latest_snapshot()?;
            let membership =
                DesktopArrangementRepository::new(&guard).list_active_membership_facts(2_000)?;
            (observation, membership)
        };
        Ok(Self::build(observation.as_ref(), &delta).with_arrangement_membership(&membership))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{
        ObservedMonitor, ObservedWindow, WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };

    use crate::WorkspaceKernel;

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
        assert!(state.latest_delta.current_pass_id.is_some());
        assert!(!state.latest_delta.has_changes);
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

    #[test]
    fn state_includes_arrangement_membership_groups() {
        use workspace_database::{DesktopArrangementRepository, WorkspaceRepository};
        use workspace_domain::{
            arrangement_now_rfc3339, entries_from_inputs, DesktopArrangement,
            DesktopArrangementEntryInput, DesktopArrangementId, DesktopArrangementStatus,
            Workspace, WorkspaceId,
        };

        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let mut snap = snapshot("pass-arr", "2026-07-26T10:02:00Z", true);
        snap.windows.push(ObservedWindow {
            id: "pass-arr-win-2".into(),
            pass_id: "pass-arr".into(),
            hwnd: "0x2".into(),
            stable_window_id: Some("stable-b".into()),
            title: "Beta".into(),
            process_id: 99,
            process_name: Some("beta.exe".into()),
            x: 20,
            y: 20,
            width: 100,
            height: 100,
            monitor_id: Some("pass-arr-mon".into()),
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(1),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snap.pass.window_count = 2;
        persist(&kernel, &snap);

        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        let workspace_id = WorkspaceId::new("ws-state-arr").unwrap();
        WorkspaceRepository::new(&guard)
            .create(&Workspace {
                id: workspace_id.clone(),
                name: "State Arr".into(),
                created_at: arrangement_now_rfc3339(),
                updated_at: arrangement_now_rfc3339(),
            })
            .unwrap();
        let arrangement_id = DesktopArrangementId::new("arr-state-1").unwrap();
        let now = arrangement_now_rfc3339();
        let entries = entries_from_inputs(
            &arrangement_id,
            &[
                DesktopArrangementEntryInput {
                    stable_window_id: Some("stable-a".into()),
                    hwnd: Some("0x1".into()),
                    process_id: Some(42),
                    process_name: Some("alpha.exe".into()),
                    title_fingerprint: None,
                    label: "Alpha".into(),
                    sort_order: 0,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                },
                DesktopArrangementEntryInput {
                    stable_window_id: Some("stable-b".into()),
                    hwnd: Some("0x2".into()),
                    process_id: Some(99),
                    process_name: Some("beta.exe".into()),
                    title_fingerprint: None,
                    label: "Beta".into(),
                    sort_order: 1,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                },
            ],
            |index| format!("entry-{index}"),
        )
        .unwrap();
        let arrangement = DesktopArrangement {
            id: arrangement_id.clone(),
            workspace_id,
            name: "Pair".into(),
            description: String::new(),
            status: DesktopArrangementStatus::Active,
            entries: entries.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
        };
        let repo = DesktopArrangementRepository::new(&guard);
        repo.upsert_arrangement(&arrangement).unwrap();
        repo.replace_entries(&arrangement_id, &entries, &now).unwrap();
        drop(guard);

        let state = WorkspaceStateEngine::get_current(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        let group = state
            .window_groups
            .iter()
            .find(|group| group.criterion == "arrangement_membership")
            .expect("arrangement membership group");
        assert_eq!(group.label, "Pair");
        assert!(group.member_ids.contains(&"stable-a".into()));
        assert!(group.member_ids.contains(&"stable-b".into()));
    }
}
