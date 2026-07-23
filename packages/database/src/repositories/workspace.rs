use crate::connection::Database;
use crate::error::Result;
use workspace_domain::Workspace;

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
                &workspace.id,
                &workspace.name,
                &workspace.created_at,
                &workspace.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Workspace>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, name, created_at, updated_at FROM workspaces WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            }));
        }
        Ok(None)
    }

    pub fn list(&self) -> Result<Vec<Workspace>> {
        let mut stmt = self
            .db
            .connection()
            .prepare("SELECT id, name, created_at, updated_at FROM workspaces ORDER BY created_at")?;

        let rows = stmt.query_map([], |row| {
            Ok(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn update_name(&self, id: &str, name: &str, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE workspaces SET name = ?1, updated_at = ?2 WHERE id = ?3",
            (name, updated_at, id),
        )?;
        Ok(changed > 0)
    }
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

    #[test]
    fn creates_and_retrieves_workspace() {
        let (_dir, db) = initialized_db();
        let repo = WorkspaceRepository::new(&db);
        let workspace = Workspace {
            id: "ws-test".into(),
            name: "Test".into(),
            created_at: "2026-07-23T10:00:00Z".into(),
            updated_at: "2026-07-23T10:00:00Z".into(),
        };

        repo.create(&workspace).unwrap();
        let loaded = repo.get_by_id("ws-test").unwrap().unwrap();
        assert_eq!(loaded, workspace);
    }

    #[test]
    fn lists_workspaces() {
        let (_dir, db) = initialized_db();
        let repo = WorkspaceRepository::new(&db);

        repo.create(&Workspace {
            id: "ws-a".into(),
            name: "A".into(),
            created_at: "2026-07-23T10:00:00Z".into(),
            updated_at: "2026-07-23T10:00:00Z".into(),
        })
        .unwrap();

        assert_eq!(repo.list().unwrap().len(), 1);
    }
}
