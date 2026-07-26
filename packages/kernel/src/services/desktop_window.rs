//! Desktop window read model — adapter over persisted observation snapshots.
//!
//! Kernel never calls Win32 or live enumerators in production. Observation
//! capture is owned by `WorkspaceObservationService`.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{ObservedMonitor, ObservedWindow, WorkspaceObservationSnapshot};
use workspace_windows_integration::DesktopWindowSnapshot;

use crate::error::Result;
use crate::services::WorkspaceObservationService;

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Adapts persisted observation snapshots into legacy desktop window DTOs.
pub struct DesktopWindowService;

impl DesktopWindowService {
    /// Returns windows from the latest persisted observation snapshot, or empty when none exists.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let Some(snapshot) = WorkspaceObservationService::get_latest_snapshot(db)? else {
            return Ok(Vec::new());
        };
        Ok(Self::windows_from_snapshot(&snapshot, limit))
    }

    /// Maps one observation snapshot into bounded desktop window DTOs.
    pub fn windows_from_snapshot(
        snapshot: &WorkspaceObservationSnapshot,
        limit: Option<usize>,
    ) -> Vec<DesktopWindowSnapshot> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let mut windows: Vec<DesktopWindowSnapshot> = snapshot
            .windows
            .iter()
            .map(|window| map_observed_window(window, &snapshot.monitors))
            .collect();
        windows.truncate(limit);
        windows
    }

    #[cfg(test)]
    pub(crate) fn list_with_stub_enumerator(
        _db: &Arc<Mutex<Database>>,
        enumerator: &dyn workspace_windows_integration::WindowEnumerator,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let mut windows = enumerator.enumerate_windows().map_err(|error| {
            crate::error::KernelError::WindowsIntegration {
                message: error.to_string(),
            }
        })?;
        windows.truncate(limit);
        Ok(windows)
    }
}

fn map_observed_window(
    window: &ObservedWindow,
    monitors: &[ObservedMonitor],
) -> DesktopWindowSnapshot {
    let monitor = window
        .monitor_id
        .as_ref()
        .and_then(|monitor_id| monitors.iter().find(|monitor| monitor.id == *monitor_id));
    DesktopWindowSnapshot {
        hwnd: window.hwnd.clone(),
        title: window.title.clone(),
        process_id: window.process_id as u32,
        visible: window.visible,
        focused: window.focused,
        minimized: window.minimized,
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
        monitor_index: monitor.map(|monitor| monitor.monitor_index),
        monitor_name: monitor.map(|monitor| monitor.name.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::{bundled_migrations_dir, Database, MigrationRunner};
    use workspace_windows_integration::StubWindowEnumerator;

    fn in_memory_db() -> Arc<Mutex<Database>> {
        let db = Database::open_in_memory().unwrap();
        let runner = MigrationRunner::load_from_dir(bundled_migrations_dir()).unwrap();
        runner.apply_all(&db).unwrap();
        Arc::new(Mutex::new(db))
    }

    #[test]
    fn list_recent_returns_empty_without_snapshot() {
        let db = in_memory_db();
        let windows = DesktopWindowService::list_recent(&db, Some(10)).unwrap();
        assert!(windows.is_empty());
    }

    #[test]
    fn stub_enumerator_test_helper_returns_empty() {
        let db = in_memory_db();
        let windows =
            DesktopWindowService::list_with_stub_enumerator(&db, &StubWindowEnumerator, Some(10))
                .unwrap();
        assert!(windows.is_empty());
    }
}
