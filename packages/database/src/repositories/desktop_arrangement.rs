//! Desktop arrangement persistence (DAF-1c/1d).
//!
//! Why: durable named window-membership records + optional restore geometry.
//! Owner: database repository; domain owns validation.
//! Does not: move windows, apply arrangements, or call WindowController.

use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    DesktopArrangement, DesktopArrangementEntry, DesktopArrangementId, DesktopArrangementStatus,
    WorkspaceId,
};

/// Repository for desktop arrangements (separate from canvas `LayoutRepository`).
pub struct DesktopArrangementRepository<'a> {
    db: &'a Database,
}

impl<'a> DesktopArrangementRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_arrangement(&self, arrangement: &DesktopArrangement) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO desktop_arrangements (
                id, workspace_id, name, description, status, created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)
             ON CONFLICT(id) DO UPDATE SET
                workspace_id = excluded.workspace_id,
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at,
                deleted = 0",
            (
                arrangement.id.as_str(),
                arrangement.workspace_id.as_str(),
                &arrangement.name,
                &arrangement.description,
                arrangement.status.as_str(),
                &arrangement.created_at,
                &arrangement.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn replace_entries(
        &self,
        arrangement_id: &DesktopArrangementId,
        entries: &[DesktopArrangementEntry],
        created_at: &str,
    ) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM desktop_arrangement_entries WHERE arrangement_id = ?1",
            [arrangement_id.as_str()],
        )?;
        for entry in entries {
            self.db.connection().execute(
                "INSERT INTO desktop_arrangement_entries (
                    id, arrangement_id, stable_window_id, hwnd, process_id, process_name,
                    title_fingerprint, label, sort_order, created_at, x, y, width, height
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                (
                    &entry.id,
                    arrangement_id.as_str(),
                    &entry.stable_window_id,
                    &entry.hwnd,
                    entry.process_id,
                    &entry.process_name,
                    &entry.title_fingerprint,
                    &entry.label,
                    entry.sort_order,
                    created_at,
                    entry.x,
                    entry.y,
                    entry.width,
                    entry.height,
                ),
            )?;
        }
        Ok(())
    }

    pub fn get(&self, id: &DesktopArrangementId) -> Result<Option<DesktopArrangement>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, created_at, updated_at
             FROM desktop_arrangements
             WHERE id = ?1 AND deleted = 0",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            let mut arrangement = map_arrangement_row(row)?;
            arrangement.entries = self.list_entries(id)?;
            return Ok(Some(arrangement));
        }
        Ok(None)
    }

    pub fn list_by_workspace(
        &self,
        workspace_id: &WorkspaceId,
        limit: usize,
    ) -> Result<Vec<DesktopArrangement>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, created_at, updated_at
             FROM desktop_arrangements
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY name ASC, updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id.as_str(), limit), map_arrangement_row)?;
        let mut arrangements = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(crate::error::DatabaseError::from)?;
        for arrangement in &mut arrangements {
            arrangement.entries = self.list_entries(&arrangement.id)?;
        }
        Ok(arrangements)
    }

    pub fn soft_delete(&self, id: &DesktopArrangementId, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE desktop_arrangements
             SET deleted = 1, updated_at = ?2
             WHERE id = ?1 AND deleted = 0",
            (id.as_str(), updated_at),
        )?;
        if changed > 0 {
            self.db.connection().execute(
                "DELETE FROM desktop_arrangement_entries WHERE arrangement_id = ?1",
                [id.as_str()],
            )?;
        }
        Ok(changed > 0)
    }

    fn list_entries(
        &self,
        arrangement_id: &DesktopArrangementId,
    ) -> Result<Vec<DesktopArrangementEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, arrangement_id, stable_window_id, hwnd, process_id, process_name,
                    title_fingerprint, label, sort_order, x, y, width, height
             FROM desktop_arrangement_entries
             WHERE arrangement_id = ?1
             ORDER BY sort_order ASC, id ASC",
        )?;
        let rows = stmt.query_map([arrangement_id.as_str()], map_entry_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_arrangement_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DesktopArrangement> {
    let status = DesktopArrangementStatus::parse(row.get::<_, String>(4)?.as_str()).map_err(
        |error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)),
    )?;
    Ok(DesktopArrangement {
        id: DesktopArrangementId::new(row.get::<_, String>(0)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        name: row.get(2)?,
        description: row.get(3)?,
        status,
        entries: Vec::new(),
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
        authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_entry_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DesktopArrangementEntry> {
    Ok(DesktopArrangementEntry {
        id: row.get(0)?,
        arrangement_id: row.get(1)?,
        stable_window_id: row.get(2)?,
        hwnd: row.get(3)?,
        process_id: row.get(4)?,
        process_name: row.get(5)?,
        title_fingerprint: row.get(6)?,
        label: row.get(7)?,
        sort_order: row.get(8)?,
        x: row.get(9)?,
        y: row.get(10)?,
        width: row.get(11)?,
        height: row.get(12)?,
        authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use crate::repositories::WorkspaceRepository;
    use tempfile::tempdir;
    use workspace_domain::{
        arrangement_now_rfc3339, entries_from_inputs, validate_desktop_arrangement,
        DesktopArrangementEntryInput, DesktopArrangementStatus, Workspace,
    };

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    fn seed_workspace(db: &Database) -> WorkspaceId {
        let workspace = Workspace {
            id: WorkspaceId::new("ws-arr-1").unwrap(),
            name: "Arrangement WS".into(),
            created_at: arrangement_now_rfc3339(),
            updated_at: arrangement_now_rfc3339(),
        };
        WorkspaceRepository::new(db)
            .create(&workspace)
            .expect("insert workspace");
        workspace.id
    }

    #[test]
    fn create_persist_and_retrieve_arrangement() {
        let (_dir, db) = initialized_db();
        let workspace_id = seed_workspace(&db);
        let repo = DesktopArrangementRepository::new(&db);
        let now = arrangement_now_rfc3339();
        let id = DesktopArrangementId::new("arr-persist-1").unwrap();
        let entries = entries_from_inputs(
            &id,
            &[DesktopArrangementEntryInput {
                stable_window_id: Some("stable-1".into()),
                hwnd: Some("0xAA".into()),
                process_id: Some(42),
                process_name: Some("code.exe".into()),
                title_fingerprint: Some("code".into()),
                label: "Editor".into(),
                sort_order: 0,
                x: Some(10),
                y: Some(20),
                width: Some(800),
                height: Some(600),
            }],
            |index| format!("entry-{index}"),
        )
        .unwrap();

        let arrangement = DesktopArrangement {
            id: id.clone(),
            workspace_id: workspace_id.clone(),
            name: "Focus".into(),
            description: "Editor only".into(),
            status: DesktopArrangementStatus::Active,
            entries: entries.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
        };
        validate_desktop_arrangement(&arrangement).unwrap();
        repo.upsert_arrangement(&arrangement).unwrap();
        repo.replace_entries(&id, &entries, &now).unwrap();

        let loaded = repo.get(&id).unwrap().expect("loaded");
        assert_eq!(loaded.name, "Focus");
        assert_eq!(loaded.status, DesktopArrangementStatus::Active);
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(
            loaded.entries[0].stable_window_id.as_deref(),
            Some("stable-1")
        );
        assert_eq!(loaded.entries[0].hwnd.as_deref(), Some("0xAA"));
        assert_eq!(loaded.entries[0].process_id, Some(42));
        assert_eq!(loaded.entries[0].x, Some(10));
        assert_eq!(loaded.entries[0].width, Some(800));

        let listed = repo.list_by_workspace(&workspace_id, 10).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, id);
    }

    #[test]
    fn soft_delete_hides_arrangement_and_clears_entries() {
        let (_dir, db) = initialized_db();
        let workspace_id = seed_workspace(&db);
        let repo = DesktopArrangementRepository::new(&db);
        let now = arrangement_now_rfc3339();
        let id = DesktopArrangementId::new("arr-delete-1").unwrap();
        let entries = entries_from_inputs(
            &id,
            &[DesktopArrangementEntryInput {
                stable_window_id: None,
                hwnd: Some("0xBB".into()),
                process_id: None,
                process_name: None,
                title_fingerprint: None,
                label: "Temp".into(),
                sort_order: 0,
                x: None,
                y: None,
                width: None,
                height: None,
            }],
            |_| "entry-x".into(),
        )
        .unwrap();
        let arrangement = DesktopArrangement {
            id: id.clone(),
            workspace_id,
            name: "Temp".into(),
            description: String::new(),
            status: DesktopArrangementStatus::Draft,
            entries: entries.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
        };
        repo.upsert_arrangement(&arrangement).unwrap();
        repo.replace_entries(&id, &entries, &now).unwrap();
        assert!(repo.soft_delete(&id, &now).unwrap());
        assert!(repo.get(&id).unwrap().is_none());
    }
}
