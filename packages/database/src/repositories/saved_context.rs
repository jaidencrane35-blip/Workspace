use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    SavedContext, SavedContextId, SavedContextMonitor, SavedContextWindow, WorkspaceId,
};

/// Persistence for user-authored bounded workspace contexts (PP-M1-01).
pub struct SavedContextRepository<'a> {
    db: &'a Database,
}

impl<'a> SavedContextRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Writes the context and everything it contains as one unit.
    ///
    /// A context that lost some of its windows would misrepresent what the user
    /// agreed to save, so a partial write is not an acceptable outcome.
    pub fn create(&self, context: &SavedContext) -> Result<()> {
        self.db.transaction(|tx| {
            tx.connection().execute(
                "INSERT INTO saved_contexts (
                     id, workspace_id, name, created_at, approved_scope,
                     observation_pass_id, captured_at, window_count, monitor_count
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                (
                    context.id.as_str(),
                    context.workspace_id.as_str(),
                    &context.name,
                    &context.created_at,
                    &context.approved_scope,
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
                tx.connection().execute(
                    "INSERT INTO saved_context_windows (
                         id, saved_context_id, title, process_id, x, y, width, height,
                         monitor_index, minimized, focused, z_order
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    (
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
                    ),
                )?;
            }

            Ok(())
        })
    }

    pub fn get_by_id(&self, id: &SavedContextId) -> Result<Option<SavedContext>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, created_at, approved_scope,
                    observation_pass_id, captured_at
             FROM saved_contexts WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id.as_str()])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let mut context = map_saved_context_row(row)?;
        context.monitors = self.monitors_for(id)?;
        context.windows = self.windows_for(id)?;
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

    fn windows_for(&self, id: &SavedContextId) -> Result<Vec<SavedContextWindow>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, title, process_id, x, y, width, height, monitor_index,
                    minimized, focused, z_order
             FROM saved_context_windows WHERE saved_context_id = ?1
             ORDER BY z_order IS NULL, z_order, title",
        )?;
        let rows = stmt.query_map([id.as_str()], |row| {
            Ok(SavedContextWindow {
                id: row.get(0)?,
                title: row.get(1)?,
                process_id: row.get(2)?,
                x: row.get(3)?,
                y: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
                monitor_index: row.get(7)?,
                minimized: row.get(8)?,
                focused: row.get(9)?,
                z_order: row.get(10)?,
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
        observation_pass_id: row.get(5)?,
        captured_at: row.get(6)?,
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

    /// Child row ids are unique per context in production (generated ids), so the
    /// fixture derives them from `id` rather than sharing them.
    fn sample_context_named(id: &str, workspace_id: &WorkspaceId) -> SavedContext {
        SavedContext {
            id: SavedContextId::new(id).unwrap(),
            workspace_id: workspace_id.clone(),
            name: "Tuesday review".into(),
            created_at: "2026-08-01T10:05:00Z".into(),
            approved_scope: SAVED_CONTEXT_SCOPE_ID.into(),
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
        // No workspace row exists, so the foreign key refuses the header and the
        // window rows must not survive on their own.
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
}
