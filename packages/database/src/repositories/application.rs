use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{ApplicationId, ApplicationReference, WorkspaceId};

/// Persistence for application reference entities.
pub struct ApplicationRepository<'a> {
    db: &'a Database,
}

impl<'a> ApplicationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, application: &ApplicationReference) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO applications (id, workspace_id, name, identifier, executable_path) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                application.id.as_str(),
                application.workspace_id.as_str(),
                &application.name,
                &application.identifier,
                &application.executable_path,
            ),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &ApplicationId) -> Result<Option<ApplicationReference>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, identifier, executable_path FROM applications WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_application_row(row)?));
        }
        Ok(None)
    }

    pub fn exists(&self, id: &ApplicationId) -> Result<bool> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM applications WHERE id = ?1",
            [id.as_str()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn update(
        &self,
        id: &ApplicationId,
        name: &str,
        identifier: Option<&str>,
        executable_path: Option<&str>,
    ) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE applications SET name = ?1, identifier = ?2, executable_path = ?3, updated_at = datetime('now') WHERE id = ?4",
            (name, identifier, executable_path, id.as_str()),
        )?;
        Ok(changed > 0)
    }

    pub fn delete(&self, id: &ApplicationId) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM applications WHERE id = ?1",
            [id.as_str()],
        )?;
        Ok(changed > 0)
    }

    pub fn list_by_workspace(&self, workspace_id: &WorkspaceId) -> Result<Vec<ApplicationReference>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, identifier, executable_path FROM applications WHERE workspace_id = ?1 ORDER BY name",
        )?;
        let rows = stmt.query_map([workspace_id.as_str()], map_application_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_application_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ApplicationReference> {
    Ok(ApplicationReference {
        id: ApplicationId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "workspace_id".into(), rusqlite::types::Type::Text)
        })?,
        name: row.get(2)?,
        identifier: row.get(3)?,
        executable_path: row.get(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use crate::repositories::WorkspaceRepository;
    use tempfile::tempdir;
    use workspace_domain::Workspace;

    fn initialized_db() -> Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn creates_and_loads_application() {
        let db = initialized_db();
        let workspace_id = WorkspaceId::new("ws-app").unwrap();
        WorkspaceRepository::new(&db)
            .create(&Workspace {
                id: workspace_id.clone(),
                name: "Main".into(),
                created_at: "2026-07-23T10:00:00Z".into(),
                updated_at: "2026-07-23T10:00:00Z".into(),
            })
            .unwrap();

        let app = ApplicationReference {
            id: ApplicationId::new("app-1").unwrap(),
            workspace_id: workspace_id.clone(),
            name: "Terminal".into(),
            identifier: Some("com.example.terminal".into()),
            executable_path: Some("C:\\Windows\\System32\\notepad.exe".into()),
        };

        let repo = ApplicationRepository::new(&db);
        repo.create(&app).unwrap();
        let loaded = repo.get_by_id(&app.id).unwrap().unwrap();
        assert_eq!(loaded, app);
    }
}
