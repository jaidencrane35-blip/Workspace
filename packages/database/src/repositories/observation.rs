use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ObservationCaptureErrorClass, ObservationCaptureFailure, ObservationWindowIdentity,
    ObservedMonitor, ObservedWindow, WindowIdentityConfidence, WorkspaceObservationError,
    WorkspaceObservationPass, WorkspaceObservationPassMetadata, WorkspaceObservationSnapshot,
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

    /// Lightweight latest-pass metadata — no window/monitor/identity row loads.
    pub fn get_latest_metadata(&self) -> Result<Option<WorkspaceObservationPassMetadata>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, captured_at, source, window_count, monitor_count
             FROM observation_passes
             ORDER BY captured_at DESC, id DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query([])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let id: String = row.get(0)?;
        let captured_at: String = row.get(1)?;
        let source: String = row.get(2)?;
        let window_count: i32 = row.get(3)?;
        let monitor_count: i32 = row.get(4)?;
        let identity_count = count_pass_identities_on(self.db.connection(), &id)?;
        Ok(Some(WorkspaceObservationPassMetadata {
            id,
            captured_at,
            source,
            window_count,
            monitor_count,
            identity_count,
        }))
    }

    pub fn record_capture_failure(&self, failure: &ObservationCaptureFailure) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO observation_capture_failures (id, failed_at, error_class, message, source)
             VALUES (1, ?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
                failed_at = excluded.failed_at,
                error_class = excluded.error_class,
                message = excluded.message,
                source = excluded.source",
            (
                &failure.failed_at,
                failure.error_class.as_str(),
                &failure.message,
                failure.source.as_deref(),
            ),
        )?;
        Ok(())
    }

    pub fn clear_capture_failure(&self) -> Result<()> {
        self.db
            .connection()
            .execute("DELETE FROM observation_capture_failures WHERE id = 1", [])?;
        Ok(())
    }

    pub fn get_capture_failure(&self) -> Result<Option<ObservationCaptureFailure>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT failed_at, error_class, message, source
             FROM observation_capture_failures
             WHERE id = 1",
        )?;
        let mut rows = stmt.query([])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let error_class = ObservationCaptureErrorClass::parse(row.get::<_, String>(1)?.as_str())
            .map_err(map_domain_error)?;
        Ok(Some(ObservationCaptureFailure {
            failed_at: row.get(0)?,
            error_class,
            message: row.get(2)?,
            source: row.get(3)?,
            authority_effect: ObservationCaptureFailure::AUTHORITY_EFFECT_NONE.into(),
        }))
    }

    /// Retention foundation — keep the newest `keep` passes; delete older passes (cascade).
    pub fn purge_older_than_keep(&self, keep: usize) -> Result<usize> {
        purge_older_than_keep_on(self.db.connection(), keep)
    }

    /// Inserts a full snapshot transactionally after domain validation.
    pub fn insert_snapshot(&self, snapshot: &WorkspaceObservationSnapshot) -> Result<()> {
        snapshot.validate().map_err(map_domain_error)?;
        self.db.transaction(|tx| insert_snapshot_on(tx.connection(), snapshot))
    }

    /// Atomically loads scoped identities, builds a snapshot, inserts it, and applies retention.
    pub fn persist_reconciled_snapshot<E, F>(
        &self,
        process_ids: &[i32],
        build: F,
        pass_retention: usize,
    ) -> Result<WorkspaceObservationSnapshot>
    where
        F: FnOnce(&[ObservationWindowIdentity]) -> std::result::Result<WorkspaceObservationSnapshot, E>,
        E: std::fmt::Display,
    {
        self.db.transaction(|tx| {
            let conn = tx.connection();
            let existing = list_identities_by_process_ids_on(conn, process_ids)?;
            let snapshot = build(&existing).map_err(|error| map_domain_display(error))?;
            snapshot.validate().map_err(map_domain_error)?;
            insert_snapshot_on(conn, &snapshot)?;
            purge_older_than_keep_on(conn, pass_retention)?;
            purge_unreferenced_identities_on(conn)?;
            Ok(snapshot)
        })
    }

    /// Loads a full snapshot by pass id.
    ///
    /// Identities are omitted: the shared identity registry is live and must not be
    /// presented as historical pass state. Window `stable_window_id` links remain.
    pub fn load_snapshot(&self, pass_id: &str) -> Result<Option<WorkspaceObservationSnapshot>> {
        let Some(pass) = self.get(pass_id)? else {
            return Ok(None);
        };
        let windows = ObservationWindowRepository::new(self.db).list_by_pass(pass_id)?;
        let monitors = ObservationMonitorRepository::new(self.db).list_by_pass(pass_id)?;

        let snapshot = WorkspaceObservationSnapshot {
            pass,
            windows,
            monitors,
            identities: Vec::new(),
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

    /// Scoped lookup for capture reconciliation — only identities for observed process ids.
    pub fn list_by_process_ids(&self, process_ids: &[i32]) -> Result<Vec<ObservationWindowIdentity>> {
        list_identities_by_process_ids_on(self.db.connection(), process_ids)
    }

    /// Removes identities not referenced by any retained observation window.
    pub fn purge_unreferenced(&self) -> Result<usize> {
        purge_unreferenced_identities_on(self.db.connection())
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

fn map_domain_display(error: impl std::fmt::Display) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(error.to_string())
}

fn insert_snapshot_on(
    conn: &rusqlite::Connection,
    snapshot: &WorkspaceObservationSnapshot,
) -> Result<()> {
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
}

fn purge_older_than_keep_on(conn: &rusqlite::Connection, keep: usize) -> Result<usize> {
    let keep = keep.max(1) as i64;
    let deleted = conn.execute(
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

fn list_identities_by_process_ids_on(
    conn: &rusqlite::Connection,
    process_ids: &[i32],
) -> Result<Vec<ObservationWindowIdentity>> {
    if process_ids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = process_ids
        .iter()
        .enumerate()
        .map(|(index, _)| format!("?{}", index + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT id, process_id, title_fingerprint, first_seen_at, last_seen_at, last_hwnd, confidence
         FROM observation_window_identities
         WHERE process_id IN ({placeholders})
         ORDER BY last_seen_at DESC, id ASC"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(rusqlite::params_from_iter(process_ids.iter().copied()), map_identity_row)?;
    rows.collect::<std::result::Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn purge_unreferenced_identities_on(conn: &rusqlite::Connection) -> Result<usize> {
    let deleted = conn.execute(
        "DELETE FROM observation_window_identities
         WHERE id NOT IN (
            SELECT DISTINCT stable_window_id FROM observation_windows
            WHERE stable_window_id IS NOT NULL
         )",
        [],
    )?;
    Ok(deleted)
}

fn count_pass_identities_on(conn: &rusqlite::Connection, pass_id: &str) -> Result<i32> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT stable_window_id)
         FROM observation_windows
         WHERE pass_id = ?1 AND stable_window_id IS NOT NULL",
        [pass_id],
        |row| row.get(0),
    )?;
    Ok(count as i32)
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
        // Historical loads omit live identity registry state.
        assert!(loaded.identities.is_empty());
        assert_eq!(
            loaded.windows[0].stable_window_id.as_deref(),
            Some("pass-a-identity")
        );
        let identities = ObservationWindowIdentityRepository::new(&db)
            .list_by_ids(&[format!("pass-a-identity")])
            .unwrap();
        assert_eq!(identities.len(), 1);
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

    #[test]
    fn historical_snapshot_does_not_embed_mutated_identity_state() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        let first = sample_snapshot("pass-hist");
        repo.insert_snapshot(&first).unwrap();

        let mut second = sample_snapshot("pass-later");
        second.pass.captured_at = "2026-07-26T12:00:00Z".into();
        second.identities[0].id = "pass-hist-identity".into();
        second.identities[0].last_seen_at = "2026-07-26T12:00:00Z".into();
        second.identities[0].last_hwnd = "0xCHANGED".into();
        second.identities[0].confidence = WindowIdentityConfidence::Low;
        second.windows[0].stable_window_id = Some("pass-hist-identity".into());
        second.windows[0].id = "pass-later-win".into();
        second.monitors[0].id = "pass-later-mon".into();
        second.windows[0].monitor_id = Some("pass-later-mon".into());
        second.monitors[0].pass_id = "pass-later".into();
        second.windows[0].pass_id = "pass-later".into();
        repo.insert_snapshot(&second).unwrap();

        let historical = repo.load_snapshot("pass-hist").unwrap().expect("historical");
        assert!(historical.identities.is_empty());
        assert_eq!(
            historical.windows[0].stable_window_id.as_deref(),
            Some("pass-hist-identity")
        );
        // Live registry was mutated by later capture; historical payload must not claim it.
        let live = ObservationWindowIdentityRepository::new(&db)
            .list_by_ids(&[format!("pass-hist-identity")])
            .unwrap();
        assert_eq!(live[0].last_hwnd, "0xCHANGED");
        assert_eq!(live[0].confidence, WindowIdentityConfidence::Low);
    }

    #[test]
    fn scoped_process_id_lookup_excludes_unrelated_identities() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.insert_snapshot(&sample_snapshot("pass-scope")).unwrap();
        let mut unrelated = sample_snapshot("pass-other");
        unrelated.identities[0].id = "other-identity".into();
        unrelated.identities[0].process_id = 999;
        unrelated.windows[0].stable_window_id = Some("other-identity".into());
        unrelated.windows[0].process_id = 999;
        unrelated.windows[0].id = "other-win".into();
        unrelated.monitors[0].id = "other-mon".into();
        unrelated.windows[0].monitor_id = Some("other-mon".into());
        repo.insert_snapshot(&unrelated).unwrap();

        let scoped = ObservationWindowIdentityRepository::new(&db)
            .list_by_process_ids(&[100])
            .unwrap();
        assert!(scoped.iter().all(|identity| identity.process_id == 100));
        assert!(!scoped.iter().any(|identity| identity.id == "other-identity"));
    }

    #[test]
    fn unreferenced_identities_are_purged_after_pass_retention() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        for index in 0..3 {
            let mut snapshot = sample_snapshot(&format!("pass-ret-{index}"));
            snapshot.pass.captured_at = format!("2026-07-26T10:00:{index:02}Z");
            snapshot.identities[0].id = format!("identity-{index}");
            snapshot.windows[0].stable_window_id = Some(format!("identity-{index}"));
            repo.insert_snapshot(&snapshot).unwrap();
        }
        repo.purge_older_than_keep(1).unwrap();
        let deleted = ObservationWindowIdentityRepository::new(&db)
            .purge_unreferenced()
            .unwrap();
        assert!(deleted >= 2);
        let remaining = ObservationWindowIdentityRepository::new(&db)
            .list_all()
            .unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "identity-2");
    }

    #[test]
    fn atomic_persist_rolls_back_on_failure() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        let error = repo
            .persist_reconciled_snapshot(&[100], |_| -> std::result::Result<_, WorkspaceObservationError> {
                Err(WorkspaceObservationError::Invalid("forced failure".into()))
            }, 50)
            .unwrap_err();
        assert!(error.to_string().contains("forced failure"));
        let count: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM observation_passes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn atomic_persist_commits_snapshot_and_scoped_identities() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        let snapshot = repo
            .persist_reconciled_snapshot(
                &[100],
                |_| {
                    Ok::<_, WorkspaceObservationError>(sample_snapshot("pass-atomic"))
                },
                50,
            )
            .unwrap();
        assert_eq!(snapshot.pass.id, "pass-atomic");
        let loaded = repo.load_snapshot("pass-atomic").unwrap().expect("loaded");
        assert_eq!(loaded.windows.len(), 1);
        let identities = ObservationWindowIdentityRepository::new(&db)
            .list_by_process_ids(&[100])
            .unwrap();
        assert_eq!(identities.len(), 1);
    }

    #[test]
    fn latest_metadata_returns_counts_without_child_collections() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.insert_snapshot(&sample_snapshot("pass-meta")).unwrap();
        let metadata = repo.get_latest_metadata().unwrap().expect("metadata");
        assert_eq!(metadata.id, "pass-meta");
        assert_eq!(metadata.window_count, 1);
        assert_eq!(metadata.monitor_count, 1);
        assert_eq!(metadata.identity_count, 1);
        assert_eq!(metadata.source, "test_inject");
    }

    #[test]
    fn capture_failure_round_trips_and_clears() {
        let db = test_db();
        let repo = ObservationPassRepository::new(&db);
        repo.record_capture_failure(&ObservationCaptureFailure {
            failed_at: "2026-07-26T12:00:00Z".into(),
            error_class: ObservationCaptureErrorClass::Validation,
            message: "forced".into(),
            source: Some("test".into()),
            authority_effect: ObservationCaptureFailure::AUTHORITY_EFFECT_NONE.into(),
        })
        .unwrap();
        let loaded = repo.get_capture_failure().unwrap().expect("failure");
        assert_eq!(loaded.error_class, ObservationCaptureErrorClass::Validation);
        assert_eq!(loaded.message, "forced");
        repo.clear_capture_failure().unwrap();
        assert!(repo.get_capture_failure().unwrap().is_none());
    }
}
