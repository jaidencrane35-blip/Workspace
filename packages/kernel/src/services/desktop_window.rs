//! Legacy desktop-window IPC adapter over WorkspaceState (Sprint 121).
//!
//! Production desktop truth is WorkspaceState. This service maps projected
//! state windows into the historical `DesktopWindowSnapshot` DTO for
//! `get_desktop_windows` and similar title-awareness callers.
//!
//! Prefer `get_workspace_state` / `WorkspaceStateEngine` for new consumers.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{ActorContext, IntentContext, WorkspaceStateWindow};
use workspace_windows_integration::DesktopWindowSnapshot;

use crate::error::Result;
use crate::services::WorkspaceStateEngine;

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Compatibility adapter: WorkspaceState → DesktopWindowSnapshot DTOs.
pub struct DesktopWindowService;

impl DesktopWindowService {
    /// Returns windows from the latest WorkspaceState projection, or empty.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let state = WorkspaceStateEngine::get_current(
            db,
            &ActorContext::system(),
            &IntentContext::user_request(),
        )?;
        Ok(state
            .windows
            .into_iter()
            .take(limit)
            .map(map_state_window)
            .collect())
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

fn map_state_window(window: WorkspaceStateWindow) -> DesktopWindowSnapshot {
    DesktopWindowSnapshot {
        hwnd: window.hwnd,
        title: window.title,
        process_id: window.process_id as u32,
        visible: window.visible,
        focused: window.focused,
        minimized: window.minimized,
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
        monitor_index: window.monitor_index,
        monitor_name: window.monitor_name,
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
