use crate::connection::Database;
use crate::error::Result;
use crate::transaction::Transaction;
use workspace_domain::{Workspace, WorkspaceId};

/// Persistence for workspace domain entities.
pub struct WorkspaceRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, workspace: &Workspace) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (
                workspace.id.as_str(),
                &workspace.name,
                &workspace.created_at,
                &workspace.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn create_in_transaction(tx: &Transaction<'_>, workspace: &Workspace) -> Result<()> {
        tx.connection().execute(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (
                workspace.id.as_str(),
                &workspace.name,
                &workspace.created_at,
                &workspace.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &WorkspaceId) -> Result<Option<Workspace>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, name, created_at, updated_at FROM workspaces WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_workspace_row(row)?));
        }
        Ok(None)
    }

    pub fn get_by_id_in_transaction(
        tx: &Transaction<'_>,
        id: &WorkspaceId,
    ) -> Result<Option<Workspace>> {
        let mut stmt = tx.connection().prepare(
            "SELECT id, name, created_at, updated_at FROM workspaces WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_workspace_row(row)?));
        }
        Ok(None)
    }

    pub fn list(&self) -> Result<Vec<Workspace>> {
        let mut stmt = self
            .db
            .connection()
            .prepare("SELECT id, name, created_at, updated_at FROM workspaces ORDER BY created_at")?;

        let rows = stmt.query_map([], map_workspace_row)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn update_name(&self, id: &WorkspaceId, name: &str, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE workspaces SET name = ?1, updated_at = ?2 WHERE id = ?3",
            (name, updated_at, id.as_str()),
        )?;
        Ok(changed > 0)
    }

    pub fn update_name_in_transaction(
        tx: &Transaction<'_>,
        id: &WorkspaceId,
        name: &str,
        updated_at: &str,
    ) -> Result<bool> {
        let changed = tx.connection().execute(
            "UPDATE workspaces SET name = ?1, updated_at = ?2 WHERE id = ?3",
            (name, updated_at, id.as_str()),
        )?;
        Ok(changed > 0)
    }
}

fn map_workspace_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Workspace> {
    let id = WorkspaceId::new(row.get::<_, String>(0)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
    })?;

    Ok(Workspace {
        id,
        name: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use tempfile::tempdir;

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    fn sample_workspace(id: &str) -> Workspace {
        Workspace {
            id: WorkspaceId::new(id).unwrap(),
            name: "Test".into(),
            created_at: "2026-07-23T10:00:00Z".into(),
            updated_at: "2026-07-23T10:00:00Z".into(),
        }
    }

    #[test]
    fn creates_and_retrieves_workspace() {
        let (_dir, db) = initialized_db();
        let repo = WorkspaceRepository::new(&db);
        let workspace = sample_workspace("ws-test");

        repo.create(&workspace).unwrap();
        let loaded = repo.get_by_id(&workspace.id).unwrap().unwrap();
        assert_eq!(loaded, workspace);
    }

    #[test]
    fn lists_workspaces() {
        let (_dir, db) = initialized_db();
        let repo = WorkspaceRepository::new(&db);

        repo.create(&sample_workspace("ws-a")).unwrap();

        assert_eq!(repo.list().unwrap().len(), 1);
    }
}
