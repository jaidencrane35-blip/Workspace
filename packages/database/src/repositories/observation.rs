use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ObservationWindowIdentity, ObservedMonitor, ObservedWindow, WindowIdentityConfidence,
    WorkspaceObservationError, WorkspaceObservationPass, WorkspaceObservationSnapshot,
};

/// Persistence for observation pass headers (system-scoped desktop truth).
pub struct ObservationPassRepository<'a> {
    db: &'a Database,
}

impl<'a> ObservationPassRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn insert_pass(&self, pass: &WorkspaceObservationPass) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO observation_passes (
                id, captured_at, schema_version, source, foreground_hwnd,
                window_count, monitor_count, duration_ms, metadata_json
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &pass.id,
                &pass.captured_at,
                pass.schema_version,
                &pass.source,
                pass.foreground_hwnd.as_deref(),
                pass.window_count,
                pass.monitor_count,
                pass.duration_ms,
                &pass.metadata_json,
            ),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<WorkspaceObservationPass>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, captured_at, schema_version, source, foreground_hwnd,
                    window_count, monitor_count, duration_ms, metadata_json
             FROM observation_passes
             WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_pass_row(row)?));
        }
        Ok(None)
    }

    pub fn get_latest(&self) -> Result<Option<WorkspaceObservationPass>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, captured_at, schema_version, source, foreground_hwnd,
                    window_count, monitor_count, duration_ms, metadata_json
             FROM observation_passes
             ORDER BY captured_at DESC, id DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_pass_row(row)?));
        }
        Ok(None)
    }

    /// Retention foundation — keep the newest `keep` passes; delete older passes (cascade).
    pub fn purge_older_than_keep(&self, keep: usize) -> Result<usize> {
        let keep = keep.max(1) as i64;
        let deleted = self.db.connection().execute(
            "DELETE FROM observation_passes
             WHERE id NOT IN (
                SELECT id FROM observation_passes
                ORDER BY captured_at DESC, id DESC
                LIMIT ?1
             )",
            [keep],
        )?;
        Ok(deleted)
    }

    /// Inserts a full snapshot transactionally after domain validation.
    pub fn insert_snapshot(&self, snapshot: &WorkspaceObservationSnapshot) -> Result<()> {
        snapshot
            .validate()
            .map_err(map_domain_error)?;

        self.db.transaction(|tx| {
            let conn = tx.connection();
            conn.execute(
                "INSERT INTO observation_passes (
                    id, captured_at, schema_version, source, foreground_hwnd,
                    window_count, monitor_count, duration_ms, metadata_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (
                    &snapshot.pass.id,
                    &snapshot.pass.captured_at,
                    snapshot.pass.schema_version,
                    &snapshot.pass.source,
                    snapshot.pass.foreground_hwnd.as_deref(),
                    snapshot.pass.window_count,
                    snapshot.pass.monitor_count,
                    snapshot.pass.duration_ms,
                    &snapshot.pass.metadata_json,
                ),
            )?;
            for monitor in &snapshot.monitors {
                conn.execute(
                    "INSERT INTO observation_monitors (
                        id, pass_id, monitor_index, name, x, y, width, height,
                        work_x, work_y, work_w, work_h, is_primary, dpi_scale
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                    (
                        &monitor.id,
                        &monitor.pass_id,
                        monitor.monitor_index,
                        &monitor.name,
                        monitor.x,
                        monitor.y,
                        monitor.width,
                        monitor.height,
                        monitor.work_x,
                        monitor.work_y,
                        monitor.work_w,
                        monitor.work_h,
                        bool_to_int(monitor.is_primary),
                        monitor.dpi_scale,
                    ),
                )?;
            }
            for window in &snapshot.windows {
                conn.execute(
                    "INSERT INTO observation_windows (
                        id, pass_id, hwnd, stable_window_id, title, process_id, process_name,
                        x, y, width, height, monitor_id, visible, minimized, focused, z_order
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
                    (
                        &window.id,
                        &window.pass_id,
                        &window.hwnd,
                        window.stable_window_id.as_deref(),
                        &window.title,
                        window.process_id,
                        window.process_name.as_deref(),
                        window.x,
                        window.y,
                        window.width,
                        window.height,
                        window.monitor_id.as_deref(),
                        bool_to_int(window.visible),
                        bool_to_int(window.minimized),
                        bool_to_int(window.focused),
                        window.z_order,
                    ),
                )?;
            }
            for identity in &snapshot.identities {
                conn.execute(
                    "INSERT INTO observation_window_identities (
                        id, process_id, title_fingerprint, first_seen_at, last_seen_at, last_hwnd, confidence
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                     ON CONFLICT(id) DO UPDATE SET
                        process_id = excluded.process_id,
                        title_fingerprint = excluded.title_fingerprint,
                        last_seen_at = excluded.last_seen_at,
                        last_hwnd = excluded.last_hwnd,
                        confidence = excluded.confidence",
                    (
                        &identity.id,
                        identity.process_id,
                        &identity.title_fingerprint,
                        &identity.first_seen_at,
                        &identity.last_seen_at,
                        &identity.last_hwnd,
                        identity.confidence.as_str(),
                    ),
                )?;
            }
            Ok(())
        })
    }

    /// Loads a full snapshot by pass id.
    pub fn load_snapshot(&self, pass_id: &str) -> Result<Option<WorkspaceObservationSnapshot>> {
        let Some(pass) = self.get(pass_id)? else {
            return Ok(None);
        };
        let windows = ObservationWindowRepository::new(self.db).list_by_pass(pass_id)?;
        let monitors = ObservationMonitorRepository::new(self.db).list_by_pass(pass_id)?;
        let identity_ids: Vec<String> = windows
            .iter()
            .filter_map(|window| window.stable_window_id.clone())
            .collect();
        let identities = if identity_ids.is_empty() {
            Vec::new()
        } else {
            ObservationWindowIdentityRepository::new(self.db).list_by_ids(&identity_ids)?
        };

        let snapshot = WorkspaceObservationSnapshot {
            pass,
            windows,
            monitors,
            identities,
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        };
        snapshot.validate().map_err(map_domain_error)?;
        Ok(Some(snapshot))
    }

    /// Loads the most recent full snapshot.
    pub fn load_latest_snapshot(&self) -> Result<Option<WorkspaceObservationSnapshot>> {
        let Some(pass) = self.get_latest()? else {
            return Ok(None);
        };
        self.load_snapshot(&pass.id)
    }
}

/// Persistence for observed monitors within a pass.
pub struct ObservationMonitorRepository<'a> {
    db: &'a Database,
}

impl<'a> ObservationMonitorRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn insert_all(&self, monitors: &[ObservedMonitor]) -> Result<()> {
        for monitor in monitors {
            self.db.connection().execute(
                "INSERT INTO observation_monitors (
                    id, pass_id, monitor_index, name, x, y, width, height,
                    work_x, work_y, work_w, work_h, is_primary, dpi_scale
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                (
                    &monitor.id,
                    &monitor.pass_id,
                    monitor.monitor_index,
                    &monitor.name,
                    monitor.x,
                    monitor.y,
                    monitor.width,
                    monitor.height,
                    monitor.work_x,
                    monitor.work_y,
                    monitor.work_w,
                    monitor.work_h,
                    bool_to_int(monitor.is_primary),
                    monitor.dpi_scale,
                ),
            )?;
        }
        Ok(())
    }

    pub fn list_by_pass(&self, pass_id: &str) -> Result<Vec<ObservedMonitor>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, pass_id, monitor_index, name, x, y, width, height,
                    work_x, work_y, work_w, work_h, is_primary, dpi_scale
             FROM observation_monitors
             WHERE pass_id = ?1
             ORDER BY monitor_index ASC, id ASC",
        )?;
        let rows = stmt.query_map([pass_id], map_monitor_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

/// Persistence for observed windows within a pass.
pub struct ObservationWindowRepository<'a> {
    db: &'a Database,
}

impl<'a> ObservationWindowRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn insert_all(&self, windows: &[ObservedWindow]) -> Result<()> {
        for window in windows {
            self.db.connection().execute(
                "INSERT INTO observation_windows (
                    id, pass_id, hwnd, stable_window_id, title, process_id, process_name,
                    x, y, width, height, monitor_id, visible, minimized, focused, z_order
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
                (
                    &window.id,
                    &window.pass_id,
                    &window.hwnd,
                    window.stable_window_id.as_deref(),
                    &window.title,
                    window.process_id,
                    window.process_name.as_deref(),
                    window.x,
                    window.y,
                    window.width,
                    window.height,
                    window.monitor_id.as_deref(),
                    bool_to_int(window.visible),
                    bool_to_int(window.minimized),
                    bool_to_int(window.focused),
                    window.z_order,
                ),
            )?;
        }
        Ok(())
    }

    pub fn list_by_pass(&self, pass_id: &str) -> Result<Vec<ObservedWindow>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, pass_id, hwnd, stable_window_id, title, process_id, process_name,
                    x, y, width, height, monitor_id, visible, minimized, focused, z_order
             FROM observation_windows
             WHERE pass_id = ?1
             ORDER BY z_order ASC, id ASC",
        )?;
        let rows = stmt.query_map([pass_id], map_window_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

/// Persistence for cross-pass window identity registry entries.
pub struct ObservationWindowIdentityRepository<'a> {
    db: &'a Database,
}

impl<'a> ObservationWindowIdentityRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_all(&self, identities: &[ObservationWindowIdentity]) -> Result<()> {
        for identity in identities {
            self.db.connection().execute(
                "INSERT INTO observation_window_identities (
                    id, process_id, title_fingerprint, first_seen_at, last_seen_at, last_hwnd, confidence
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                 ON CONFLICT(id) DO UPDATE SET
                    process_id = excluded.process_id,
                    title_fingerprint = excluded.title_fingerprint,
                    last_seen_at = excluded.last_seen_at,
                    last_hwnd = excluded.last_hwnd,
                    confidence = excluded.confidence",
                (
                    &identity.id,
                    identity.process_id,
                    &identity.title_fingerprint,
                    &identity.first_seen_at,
                    &identity.last_seen_at,
                    &identity.last_hwnd,
                    identity.confidence.as_str(),
                ),
            )?;
        }
        Ok(())
    }

    pub fn list_by_ids(&self, ids: &[String]) -> Result<Vec<ObservationWindowIdentity>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let placeholders = ids
            .iter()
            .enumerate()
            .map(|(index, _)| format!("?{}", index + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT id, process_id, title_fingerprint, first_seen_at, last_seen_at, last_hwnd, confidence
             FROM observation_window_identities
             WHERE id IN ({placeholders})
             ORDER BY last_seen_at DESC, id ASC"
        );
        let mut stmt = self.db.connection().prepare(&sql)?;
        let params: Vec<&str> = ids.iter().map(String::as_str).collect();
        let rows = stmt.query_map(rusqlite::params_from_iter(params), map_identity_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_all(&self) -> Result<Vec<ObservationWindowIdentity>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, process_id, title_fingerprint, first_seen_at, last_seen_at, last_hwnd, confidence
             FROM observation_window_identities
             ORDER BY last_seen_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([], map_identity_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn bool_to_int(value: bool) -> i32 {
    i32::from(value)
}

fn int_to_bool(value: i32) -> bool {
    value != 0
}

fn map_domain_error(error: WorkspaceObservationError) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(error.to_string())
}

fn map_pass_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceObservationPass> {
    Ok(WorkspaceObservationPass {
        id: row.get(0)?,
        captured_at: row.get(1)?,
        schema_version: row.get(2)?,
        source: row.get(3)?,
        foreground_hwnd: row.get(4)?,
        window_count: row.get(5)?,
        monitor_count: row.get(6)?,
        duration_ms: row.get(7)?,
        metadata_json: row.get(8)?,
        authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_monitor_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ObservedMonitor> {
    Ok(ObservedMonitor {
        id: row.get(0)?,
        pass_id: row.get(1)?,
        monitor_index: row.get(2)?,
        name: row.get(3)?,
        x: row.get(4)?,
        y: row.get(5)?,
        width: row.get(6)?,
        height: row.get(7)?,
        work_x: row.get(8)?,
        work_y: row.get(9)?,
        work_w: row.get(10)?,
        work_h: row.get(11)?,
        is_primary: int_to_bool(row.get(12)?),
        dpi_scale: row.get(13)?,
        authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_window_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ObservedWindow> {
    Ok(ObservedWindow {
        id: row.get(0)?,
        pass_id: row.get(1)?,
        hwnd: row.get(2)?,
        stable_window_id: row.get(3)?,
        title: row.get(4)?,
        process_id: row.get(5)?,
        process_name: row.get(6)?,
        x: row.get(7)?,
        y: row.get(8)?,
        width: row.get(9)?,
        height: row.get(10)?,
        monitor_id: row.get(11)?,
        visible: int_to_bool(row.get(12)?),
        minimized: int_to_bool(row.get(13)?),
        focused: int_to_bool(row.get(14)?),
        z_order: row.get(15)?,
        authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_identity_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ObservationWindowIdentity> {
    let confidence = WindowIdentityConfidence::parse(row.get::<_, String>(6)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(ObservationWindowIdentity {
        id: row.get(0)?,
        process_id: row.get(1)?,
        title_fingerprint: row.get(2)?,
        first_seen_at: row.get(3)?,
        last_seen_at: row.get(4)?,
        last_hwnd: row.get(5)?,
        confidence,
        authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::bundled_migrations_dir;
    use crate::migration::MigrationRunner;

    fn test_db() -> Database {
        let db = Database::open_in_memory().unwrap();
        let runner = MigrationRunner::load_from_dir(bundled_migrations_dir()).unwrap();
        runner.apply_all(&db).unwrap();
        db
    }

    fn sample_snapshot(pass_id: &str) -> WorkspaceObservationSnapshot {
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: "2026-07-26T10:00:00Z".into(),
                schema_version: 1,
                source: "test_inject".into(),
                foreground_hwnd: Some("0x0000000000000001".into()),
                window_count: 1,
                monitor_count: 1,
                duration_ms: Some(5),
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
                hwnd: "0x0000000000000001".into(),
                stable_window_id: Some(format!("{pass_id}-identity")),
                title: "Repo Window".into(),
                process_id: 100,
                process_name: None,
                x: 10,
                y: 20,
                width: 800,
                height: 600,
                monitor_id: Some(format!("{pass_id}-mon")),
                visible: true,
                minimized: false,
                focused: true,
                z_order: Some(0),
                authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
            }],
            identities: vec![ObservationWindowIdentity {
                id: format!("{pass_id}-identity"),
                process_id: 100,
                title_fingerprint: "repo-window".into(),
                first_seen_at: "2026-07-26T10:00:00Z".into(),
                last_seen_at: "2026-07-26T10:00:00Z".into(),
                last_hwnd: "0x0000000000000001".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn observation_snapshot_persists_and_loads() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        let snapshot = sample_snapshot("pass-a");

        repo.insert_snapshot(&snapshot).unwrap();
        let loaded = repo.load_snapshot("pass-a").unwrap().expect("snapshot");
        assert_eq!(loaded.pass.id, "pass-a");
        assert_eq!(loaded.windows.len(), 1);
        assert_eq!(loaded.monitors.len(), 1);
        assert_eq!(loaded.identities.len(), 1);
    }

    #[test]
    fn child_windows_load_for_pass() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.insert_snapshot(&sample_snapshot("pass-b")).unwrap();

        let windows = ObservationWindowRepository::new(&db)
            .list_by_pass("pass-b")
            .unwrap();
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].title, "Repo Window");
    }

    #[test]
    fn child_monitors_load_for_pass() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.insert_snapshot(&sample_snapshot("pass-c")).unwrap();

        let monitors = ObservationMonitorRepository::new(&db)
            .list_by_pass("pass-c")
            .unwrap();
        assert_eq!(monitors.len(), 1);
        assert!(monitors[0].is_primary);
    }

    #[test]
    fn latest_snapshot_loads() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.insert_snapshot(&sample_snapshot("pass-old")).unwrap();
        let mut newer = sample_snapshot("pass-new");
        newer.pass.captured_at = "2026-07-26T11:00:00Z".into();
        repo.insert_snapshot(&newer).unwrap();

        let latest = repo.load_latest_snapshot().unwrap().expect("latest");
        assert_eq!(latest.pass.id, "pass-new");
    }

    #[test]
    fn retention_purge_removes_old_passes() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        for index in 0..5 {
            let mut snapshot = sample_snapshot(&format!("pass-{index}"));
            snapshot.pass.captured_at = format!("2026-07-26T10:00:{index:02}Z");
            repo.insert_snapshot(&snapshot).unwrap();
        }

        let deleted = repo.purge_older_than_keep(2).unwrap();
        assert_eq!(deleted, 3);

        let remaining: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM observation_passes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(remaining, 2);

        let latest = repo.load_latest_snapshot().unwrap().expect("latest");
        assert_eq!(latest.pass.id, "pass-4");
    }

    #[test]
    fn empty_stub_snapshot_persists() {
        let db = test_db();
        let snapshot =
            workspace_domain::empty_stub_snapshot("stub-pass", "2026-07-26T10:00:00Z");
        ObservationPassRepository::new(&db)
            .insert_snapshot(&snapshot)
            .unwrap();

        let loaded = ObservationPassRepository::new(&db)
            .load_snapshot("stub-pass")
            .unwrap()
            .expect("stub");
        assert_eq!(loaded.pass.source, "stub");
        assert!(loaded.windows.is_empty());
        assert!(loaded.monitors.is_empty());
    }

    #[test]
    fn invalid_snapshot_rejected_before_insert() {
        let db = test_db();
        let mut snapshot = sample_snapshot("pass-invalid");
        snapshot.pass.window_count = 0;
        let error = ObservationPassRepository::new(&db)
            .insert_snapshot(&snapshot)
            .unwrap_err();
        assert!(error.to_string().contains("window_count"));
    }

    #[test]
    fn identity_confidence_round_trips() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        let mut snapshot = sample_snapshot("pass-id");
        snapshot.identities[0].confidence = WindowIdentityConfidence::Ephemeral;
        repo.insert_snapshot(&snapshot).unwrap();

        let identities = ObservationWindowIdentityRepository::new(&db)
            .list_by_ids(&[format!("pass-id-identity")])
            .unwrap();
        assert_eq!(identities[0].confidence, WindowIdentityConfidence::Ephemeral);
    }
}
