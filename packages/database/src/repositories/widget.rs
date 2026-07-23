use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{WidgetId, WidgetReference, WorkspaceId};

/// Persistence for widget reference entities.
pub struct WidgetRepository<'a> {
    db: &'a Database,
}

impl<'a> WidgetRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, widget: &WidgetReference) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO widgets (id, workspace_id, name, widget_type) VALUES (?1, ?2, ?3, ?4)",
            (
                widget.id.as_str(),
                widget.workspace_id.as_str(),
                &widget.name,
                &widget.widget_type,
            ),
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &WidgetId) -> Result<Option<WidgetReference>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, widget_type FROM widgets WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_widget_row(row)?));
        }
        Ok(None)
    }

    pub fn exists(&self, id: &WidgetId) -> Result<bool> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM widgets WHERE id = ?1",
            [id.as_str()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn update(&self, id: &WidgetId, name: &str, widget_type: Option<&str>) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE widgets SET name = ?1, widget_type = ?2, updated_at = datetime('now') WHERE id = ?3",
            (name, widget_type, id.as_str()),
        )?;
        Ok(changed > 0)
    }

    pub fn delete(&self, id: &WidgetId) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM widgets WHERE id = ?1",
            [id.as_str()],
        )?;
        Ok(changed > 0)
    }

    pub fn list_by_workspace(&self, workspace_id: &WorkspaceId) -> Result<Vec<WidgetReference>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, widget_type FROM widgets WHERE workspace_id = ?1 ORDER BY name",
        )?;
        let rows = stmt.query_map([workspace_id.as_str()], map_widget_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_widget_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WidgetReference> {
    Ok(WidgetReference {
        id: WidgetId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "workspace_id".into(), rusqlite::types::Type::Text)
        })?,
        name: row.get(2)?,
        widget_type: row.get(3)?,
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
    fn creates_and_loads_widget() {
        let db = initialized_db();
        let workspace_id = WorkspaceId::new("ws-widget").unwrap();
        WorkspaceRepository::new(&db)
            .create(&Workspace {
                id: workspace_id.clone(),
                name: "Main".into(),
                created_at: "2026-07-23T10:00:00Z".into(),
                updated_at: "2026-07-23T10:00:00Z".into(),
            })
            .unwrap();

        let widget = WidgetReference {
            id: WidgetId::new("widget-1").unwrap(),
            workspace_id: workspace_id.clone(),
            name: "Clock".into(),
            widget_type: Some("clock".into()),
        };

        let repo = WidgetRepository::new(&db);
        repo.create(&widget).unwrap();
        let loaded = repo.get_by_id(&widget.id).unwrap().unwrap();
        assert_eq!(loaded, widget);
    }
}
