//! Desktop window observation — kernel orchestration over the Windows
//! Integration Layer (Sprint 35–36). Kernel never calls Win32 directly.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_windows_integration::{
    platform_window_enumerator, DesktopWindowSnapshot, WindowEnumerator,
};

use crate::error::{KernelError, Result};
use crate::services::WorkspaceObservationService;

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Lists top-level desktop windows from the latest persisted observation snapshot,
/// falling back to live enumeration when no snapshot exists yet.
pub struct DesktopWindowService;

impl DesktopWindowService {
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        if let Some(snapshot) = WorkspaceObservationService::get_latest_snapshot(db)? {
            let mut windows: Vec<DesktopWindowSnapshot> = snapshot
                .windows
                .iter()
                .filter(|window| window.visible && !window.minimized)
                .map(|window| DesktopWindowSnapshot {
                    hwnd: window.hwnd.clone(),
                    title: window.title.clone(),
                    process_id: window.process_id as u32,
                    visible: window.visible,
                })
                .collect();
            windows.truncate(limit);
            return Ok(windows);
        }

        Self::list_with(db, &*platform_window_enumerator(), Some(limit))
    }

    pub fn list_with(
        _db: &Arc<Mutex<Database>>,
        enumerator: &dyn WindowEnumerator,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let mut windows = enumerator.enumerate_windows().map_err(|error| {
            KernelError::WindowsIntegration {
                message: error.to_string(),
            }
        })?;
        windows.truncate(limit);
        Ok(windows)
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
    fn stub_enumerator_returns_empty() {
        let db = in_memory_db();
        let windows = DesktopWindowService::list_with(&db, &StubWindowEnumerator, Some(10)).unwrap();
        assert!(windows.is_empty());
    }
}
