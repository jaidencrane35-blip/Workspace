//! Zone resource service — persistence and graph registration.

use workspace_database::{Database, ZoneRepository};
use workspace_domain::{
    Addressable, ResourceId, ResourceKind, ResourceRef, WorkspaceId, Zone, ZoneId,
};

use super::GraphService;
use crate::error::{KernelError, Result};

pub struct ZoneService;

impl ZoneService {
    pub fn create(
        db: &Database,
        workspace_id: WorkspaceId,
        name: String,
        position_metadata: Option<String>,
    ) -> Result<Zone> {
        Zone::validate_name(&name).map_err(KernelError::from)?;

        let zone = Zone {
            id: ZoneId::generate(),
            workspace_id: workspace_id.clone(),
            name: name.trim().to_string(),
            position_metadata,
        };

        ZoneRepository::new(db).create(&zone)?;

        let parent = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(workspace_id.as_str()).expect("workspace id is non-empty"),
        );
        GraphService::register_child(db, &parent, &zone.resource_ref())?;

        Ok(zone)
    }

    pub fn load(db: &Database, id: &ZoneId) -> Result<Zone> {
        ZoneRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::ZoneNotFound)
    }

    pub fn update(
        db: &Database,
        id: &ZoneId,
        name: String,
        position_metadata: Option<String>,
    ) -> Result<Zone> {
        Zone::validate_name(&name).map_err(KernelError::from)?;
        let updated = ZoneRepository::new(db).update(
            id,
            name.trim(),
            position_metadata.as_deref(),
        )?;
        if !updated {
            return Err(KernelError::ZoneNotFound);
        }
        Self::load(db, id)
    }

    pub fn delete(db: &Database, id: &ZoneId) -> Result<()> {
        let zone = Self::load(db, id)?;
        let deleted = ZoneRepository::new(db).delete(id)?;
        if !deleted {
            return Err(KernelError::ZoneNotFound);
        }
        GraphService::unregister_node(db, &zone.resource_ref())?;
        Ok(())
    }

    pub fn validate(name: &str) -> Result<()> {
        Zone::validate_name(name).map_err(KernelError::from)
    }

    pub fn exists(db: &Database, id: &ZoneId) -> Result<bool> {
        ZoneRepository::new(db).exists(id).map_err(Into::into)
    }

    pub fn lookup(db: &Database, id: &ZoneId) -> Result<Option<Zone>> {
        ZoneRepository::new(db).get_by_id(id).map_err(Into::into)
    }

    pub fn list_by_workspace(db: &Database, workspace_id: &WorkspaceId) -> Result<Vec<Zone>> {
        ZoneRepository::new(db)
            .list_by_workspace(workspace_id)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::WorkspaceService;
    use workspace_database::DatabaseService;
    use tempfile::tempdir;

    fn test_db() -> Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn creates_zone_and_registers_in_graph() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Main".into()).unwrap();
        let zone = ZoneService::create(&db, workspace.id.clone(), "Primary".into(), None).unwrap();

        assert_eq!(zone.workspace_id, workspace.id);
        assert!(GraphService::exists(&db, &zone.resource_ref()).unwrap());
    }
}
