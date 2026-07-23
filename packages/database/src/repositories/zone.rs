use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{WorkspaceId, Zone, ZoneId};

/// Persistence for workspace zone entities.
pub struct ZoneRepository<'a> {
    db: &'a Database,
}

impl<'a> ZoneRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, zone: &Zone) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO zones (id, workspace_id, name, position_metadata) VALUES (?1, ?2, ?3, ?4)",
            (
                zone.id.as_str(),
                zone.workspace_id.as_str(),
                &zone.name,
                &zone.position_metadata,
            ),
        )?;
        Ok(())
    }

    pub fn list_by_workspace(&self, workspace_id: &WorkspaceId) -> Result<Vec<Zone>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, position_metadata FROM zones WHERE workspace_id = ?1 ORDER BY name",
        )?;

        let rows = stmt.query_map([workspace_id.as_str()], |row| {
            Ok(Zone {
                id: ZoneId::new(row.get::<_, String>(0)?).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        0,
                        "id".into(),
                        rusqlite::types::Type::Text,
                    )
                })?,
                workspace_id: WorkspaceId::new(row.get::<_, String>(1)?).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        1,
                        "workspace_id".into(),
                        rusqlite::types::Type::Text,
                    )
                })?,
                name: row.get(2)?,
                position_metadata: row.get(3)?,
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use crate::repositories::WorkspaceRepository;
    use tempfile::tempdir;
    use workspace_domain::Workspace;

    #[test]
    fn creates_zone_for_workspace() {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();

        let workspace_id = WorkspaceId::new("ws-1").unwrap();

        WorkspaceRepository::new(&db)
            .create(&Workspace {
                id: workspace_id.clone(),
                name: "Main".into(),
                created_at: "2026-07-23T10:00:00Z".into(),
                updated_at: "2026-07-23T10:00:00Z".into(),
            })
            .unwrap();

        ZoneRepository::new(&db)
            .create(&Zone {
                id: ZoneId::new("zone-1").unwrap(),
                workspace_id: workspace_id.clone(),
                name: "Primary".into(),
                position_metadata: Some("{}".into()),
            })
            .unwrap();

        assert_eq!(
            ZoneRepository::new(&db)
                .list_by_workspace(&workspace_id)
                .unwrap()
                .len(),
            1
        );
    }
}
