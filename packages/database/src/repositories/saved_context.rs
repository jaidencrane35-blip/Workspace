use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    SavedContext, SavedContextId, SavedContextMonitor, SavedContextRestoreIdentity,
    SavedContextWindow, WorkspaceId, RESTORE_IDENTITY_LEGACY_REASON,
};

/// Persistence for user-authored bounded workspace contexts (PP-M1-01 / PP-M1-02).
pub struct SavedContextRepository<'a> {
    db: &'a Database,
}

impl<'a> SavedContextRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Writes the context and everything it contains as one unit.
    pub fn create(&self, context: &SavedContext) -> Result<()> {
        self.db.transaction(|tx| {
            tx.connection().execute(
                "INSERT INTO saved_contexts (
                     id, workspace_id, name, created_at, approved_scope, handoff_note,
                     observation_pass_id, captured_at, window_count, monitor_count
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                (
                    context.id.as_str(),
                    context.workspace_id.as_str(),
                    &context.name,
                    &context.created_at,
                    &context.approved_scope,
                    &context.handoff_note,
                    &context.observation_pass_id,
                    &context.captured_at,
                    context.windows.len() as i64,
                    context.monitors.len() as i64,
                ),
            )?;

            for monitor in &context.monitors {
                tx.connection().execute(
                    "INSERT INTO saved_context_monitors (
                         id, saved_context_id, monitor_index, name, x, y, width, height, is_primary
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    (
                        &monitor.id,
                        context.id.as_str(),
                        monitor.monitor_index,
                        &monitor.name,
                        monitor.x,
                        monitor.y,
                        monitor.width,
                        monitor.height,
                        monitor.is_primary,
                    ),
                )?;
            }

            for window in &context.windows {
                let (
                    identity_schema_version,
                    desktop_session_id,
                    captured_hwnd,
                    title_fingerprint,
                    restore_identity_unavailable_reason,
                ) = match &window.restore_identity {
                    Some(identity) => (
                        Some(identity.identity_schema_version.as_str()),
                        Some(identity.desktop_session_id.as_str()),
                        Some(identity.captured_hwnd.as_str()),
                        Some(identity.title_fingerprint.as_str()),
                        None::<&str>,
                    ),
                    None => (
                        None,
                        None,
                        None,
                        None,
                        window
                            .restore_identity_unavailable_reason
                            .as_deref()
                            .or(Some(RESTORE_IDENTITY_LEGACY_REASON)),
                    ),
                };

                tx.connection().execute(
                    "INSERT INTO saved_context_windows (
                         id, saved_context_id, title, process_id, x, y, width, height,
                         monitor_index, minimized, focused, z_order,
                         identity_schema_version, desktop_session_id, captured_hwnd,
                         title_fingerprint, restore_identity_unavailable_reason
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
                    rusqlite::params![
                        &window.id,
                        context.id.as_str(),
                        &window.title,
                        window.process_id,
                        window.x,
                        window.y,
                        window.width,
                        window.height,
                        window.monitor_index,
                        window.minimized,
                        window.focused,
                        window.z_order,
                        identity_schema_version,
                        desktop_session_id,
                        captured_hwnd,
                        title_fingerprint,
                        restore_identity_unavailable_reason,
                    ],
                )?;
            }

            Ok(())
        })
    }

    pub fn get_by_id(&self, id: &SavedContextId) -> Result<Option<SavedContext>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, created_at, approved_scope, handoff_note,
                    observation_pass_id, captured_at
             FROM saved_contexts WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id.as_str()])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let mut context = map_saved_context_row(row)?;
        context.monitors = self.monitors_for(id)?;
        context.windows = self.windows_for(id, &context.captured_at)?;
        Ok(Some(context))
    }

    pub fn list_for_workspace(&self, workspace_id: &WorkspaceId) -> Result<Vec<SavedContext>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id FROM saved_contexts WHERE workspace_id = ?1 ORDER BY created_at DESC",
        )?;
        let ids = stmt
            .query_map([workspace_id.as_str()], |row| row.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let mut contexts = Vec::with_capacity(ids.len());
        for id in ids {
            let Ok(typed) = SavedContextId::new(id) else {
                continue;
            };
            if let Some(context) = self.get_by_id(&typed)? {
                contexts.push(context);
            }
        }
        Ok(contexts)
    }

    pub fn delete_by_id(&self, id: &SavedContextId) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM saved_contexts WHERE id = ?1",
            [id.as_str()],
        )?;
        Ok(changed > 0)
    }

    fn monitors_for(&self, id: &SavedContextId) -> Result<Vec<SavedContextMonitor>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, monitor_index, name, x, y, width, height, is_primary
             FROM saved_context_monitors WHERE saved_context_id = ?1 ORDER BY monitor_index",
        )?;
        let rows = stmt.query_map([id.as_str()], |row| {
            Ok(SavedContextMonitor {
                id: row.get(0)?,
                monitor_index: row.get(1)?,
                name: row.get(2)?,
                x: row.get(3)?,
                y: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
                is_primary: row.get(7)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn windows_for(
        &self,
        id: &SavedContextId,
        captured_at: &str,
    ) -> Result<Vec<SavedContextWindow>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, title, process_id, x, y, width, height, monitor_index,
                    minimized, focused, z_order,
                    identity_schema_version, desktop_session_id, captured_hwnd,
                    title_fingerprint, restore_identity_unavailable_reason
             FROM saved_context_windows WHERE saved_context_id = ?1
             ORDER BY z_order IS NULL, z_order, title",
        )?;
        let captured_at = captured_at.to_string();
        let rows = stmt.query_map([id.as_str()], move |row| {
            let process_id: i32 = row.get(2)?;
            let identity_schema_version: Option<String> = row.get(11)?;
            let desktop_session_id: Option<String> = row.get(12)?;
            let captured_hwnd: Option<String> = row.get(13)?;
            let title_fingerprint: Option<String> = row.get(14)?;
            let unavailable: Option<String> = row.get(15)?;

            let (restore_identity, restore_identity_unavailable_reason) = match (
                identity_schema_version,
                desktop_session_id,
                captured_hwnd,
                title_fingerprint,
            ) {
                (Some(version), Some(session), Some(hwnd), Some(fingerprint))
                    if !version.trim().is_empty()
                        && !session.trim().is_empty()
                        && !hwnd.trim().is_empty()
                        && !fingerprint.trim().is_empty() =>
                {
                    (
                        Some(SavedContextRestoreIdentity {
                            identity_schema_version: version,
                            desktop_session_id: session,
                            captured_hwnd: hwnd,
                            captured_process_id: process_id,
                            title_fingerprint: fingerprint,
                            captured_at: captured_at.clone(),
                        }),
                        None,
                    )
                }
                _ => (
                    None,
                    Some(
                        unavailable
                            .filter(|value| !value.trim().is_empty())
                            .unwrap_or_else(|| RESTORE_IDENTITY_LEGACY_REASON.into()),
                    ),
                ),
            };

            Ok(SavedContextWindow {
                id: row.get(0)?,
                title: row.get(1)?,
                process_id,
                x: row.get(3)?,
                y: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
                monitor_index: row.get(7)?,
                minimized: row.get(8)?,
                focused: row.get(9)?,
                z_order: row.get(10)?,
                restore_identity,
                restore_identity_unavailable_reason,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_saved_context_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SavedContext> {
    let id = SavedContextId::new(row.get::<_, String>(0)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
    })?;
    let workspace_id = WorkspaceId::new(row.get::<_, String>(1)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(1, "workspace_id".into(), rusqlite::types::Type::Text)
    })?;

    Ok(SavedContext {
        id,
        workspace_id,
        name: row.get(2)?,
        created_at: row.get(3)?,
        approved_scope: row.get(4)?,
        handoff_note: row.get(5)?,
        observation_pass_id: row.get(6)?,
        captured_at: row.get(7)?,
        windows: Vec::new(),
        monitors: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use crate::repositories::WorkspaceRepository;
    use tempfile::tempdir;
    use workspace_domain::{Workspace, SAVED_CONTEXT_SCOPE_ID};

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    fn seeded_workspace(db: &Database) -> Workspace {
        let workspace = Workspace {
            id: WorkspaceId::new("ws-saved-context").unwrap(),
            name: "Test".into(),
            created_at: "2026-08-01T10:00:00Z".into(),
            updated_at: "2026-08-01T10:00:00Z".into(),
        };
        WorkspaceRepository::new(db).create(&workspace).unwrap();
        workspace
    }

    fn sample_context_named(id: &str, workspace_id: &WorkspaceId) -> SavedContext {
        SavedContext {
            id: SavedContextId::new(id).unwrap(),
            workspace_id: workspace_id.clone(),
            name: "Tuesday review".into(),
            created_at: "2026-08-01T10:05:00Z".into(),
            approved_scope: SAVED_CONTEXT_SCOPE_ID.into(),
            handoff_note: "Finish the client proposal outline".into(),
            observation_pass_id: "pass-1".into(),
            captured_at: "2026-08-01T10:05:00Z".into(),
            windows: vec![
                SavedContextWindow {
                    id: format!("{id}-w-1"),
                    title: "Focused window".into(),
                    process_id: 42,
                    x: 10,
                    y: 20,
                    width: 800,
                    height: 600,
                    monitor_index: Some(0),
                    minimized: false,
                    focused: true,
                    z_order: Some(0),
                    restore_identity: Some(SavedContextRestoreIdentity::new(
                        "stub-desktop-session-1",
                        "0xAA",
                        42,
                        "Focused window",
                        "2026-08-01T10:05:00Z",
                    )),
                    restore_identity_unavailable_reason: None,
                },
                SavedContextWindow {
                    id: format!("{id}-w-2"),
                    title: "Minimised window".into(),
                    process_id: 43,
                    x: -32000,
                    y: -32000,
                    width: 160,
                    height: 28,
                    monitor_index: None,
                    minimized: true,
                    focused: false,
                    z_order: Some(1),
                    restore_identity: None,
                    restore_identity_unavailable_reason: Some(
                        "incomplete restore identity at capture".into(),
                    ),
                },
            ],
            monitors: vec![SavedContextMonitor {
                id: format!("{id}-m-1"),
                monitor_index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                is_primary: true,
            }],
        }
    }

    fn sample_context(workspace_id: &WorkspaceId) -> SavedContext {
        sample_context_named("sc-1", workspace_id)
    }

    #[test]
    fn stores_and_returns_the_whole_context() {
        let (_dir, db) = initialized_db();
        let workspace = seeded_workspace(&db);
        let repo = SavedContextRepository::new(&db);
        let context = sample_context(&workspace.id);

        repo.create(&context).unwrap();
        let loaded = repo.get_by_id(&context.id).unwrap().unwrap();

        assert_eq!(loaded, context);
        assert_eq!(loaded.approved_scope, SAVED_CONTEXT_SCOPE_ID);
        assert!(loaded.windows[0].restore_identity.is_some());
    }

    #[test]
    fn counts_recorded_on_the_header_match_the_stored_rows() {
        let (_dir, db) = initialized_db();
        let workspace = seeded_workspace(&db);
        let context = sample_context(&workspace.id);
        SavedContextRepository::new(&db).create(&context).unwrap();

        let (windows, monitors): (i64, i64) = db
            .connection()
            .query_row(
                "SELECT window_count, monitor_count FROM saved_contexts WHERE id = ?1",
                [context.id.as_str()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(windows, 2);
        assert_eq!(monitors, 1);
    }

    #[test]
    fn a_rejected_write_leaves_nothing_behind() {
        let (_dir, db) = initialized_db();
        let repo = SavedContextRepository::new(&db);
        let orphan = sample_context(&WorkspaceId::new("ws-missing").unwrap());

        assert!(repo.create(&orphan).is_err());

        let windows: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM saved_context_windows", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(windows, 0);
    }

    #[test]
    fn lists_contexts_for_one_workspace_newest_first() {
        let (_dir, db) = initialized_db();
        let workspace = seeded_workspace(&db);
        let repo = SavedContextRepository::new(&db);

        let mut older = sample_context_named("sc-older", &workspace.id);
        older.created_at = "2026-08-01T09:00:00Z".into();
        let mut newer = sample_context_named("sc-newer", &workspace.id);
        newer.created_at = "2026-08-01T11:00:00Z".into();

        repo.create(&older).unwrap();
        repo.create(&newer).unwrap();

        let listed = repo.list_for_workspace(&workspace.id).unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].id, newer.id);
    }

    #[test]
    fn delete_removes_restore_identity_rows() {
        let (_dir, db) = initialized_db();
        let workspace = seeded_workspace(&db);
        let repo = SavedContextRepository::new(&db);
        let context = sample_context(&workspace.id);
        repo.create(&context).unwrap();
        assert!(repo.delete_by_id(&context.id).unwrap());
        let windows: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM saved_context_windows", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(windows, 0);
    }
}
