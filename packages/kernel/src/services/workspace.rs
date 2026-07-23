//! Workspace domain operations — persistence and graph node registration.

use chrono::Utc;
use workspace_database::{Database, WorkspaceRepository};
use workspace_domain::{Addressable, Workspace, WorkspaceId};

use super::GraphService;
use crate::error::{KernelError, Result};

/// Kernel-owned workspace resource service.
pub struct WorkspaceService;

impl WorkspaceService {
    pub(crate) fn create(db: &Database, name: String) -> Result<Workspace> {
        Workspace::validate_name(&name).map_err(KernelError::from)?;

        let now = Utc::now().to_rfc3339();
        let workspace = Workspace {
            id: WorkspaceId::generate(),
            name: name.trim().to_string(),
            created_at: now.clone(),
            updated_at: now,
        };

        WorkspaceRepository::new(db).create(&workspace)?;
        GraphService::register_node(db, &workspace.resource_ref())?;

        Ok(workspace)
    }

    pub fn load(db: &Database, id: &WorkspaceId) -> Result<Workspace> {
        WorkspaceRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::WorkspaceNotFound)
    }

    pub(crate) fn update(db: &Database, id: &WorkspaceId, name: String) -> Result<Workspace> {
        Workspace::validate_name(&name).map_err(KernelError::from)?;
        let updated_at = Utc::now().to_rfc3339();
        let trimmed = name.trim().to_string();

        let updated = WorkspaceRepository::new(db).update_name(id, &trimmed, &updated_at)?;
        if !updated {
            return Err(KernelError::WorkspaceNotFound);
        }

        Self::load(db, id)
    }

    pub(crate) fn delete(db: &Database, id: &WorkspaceId) -> Result<()> {
        let workspace = Self::load(db, id)?;
        let deleted = WorkspaceRepository::new(db).delete(id)?;
        if !deleted {
            return Err(KernelError::WorkspaceNotFound);
        }
        GraphService::unregister_node(db, &workspace.resource_ref())?;
        Ok(())
    }

    pub fn validate(name: &str) -> Result<()> {
        Workspace::validate_name(name).map_err(KernelError::from)
    }

    pub fn exists(db: &Database, id: &WorkspaceId) -> Result<bool> {
        WorkspaceRepository::new(db).exists(id).map_err(Into::into)
    }

    pub fn lookup(db: &Database, id: &WorkspaceId) -> Result<Option<Workspace>> {
        WorkspaceRepository::new(db).get_by_id(id).map_err(Into::into)
    }

    pub fn list(db: &Database) -> Result<Vec<Workspace>> {
        WorkspaceRepository::new(db).list().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::DatabaseService;
    use tempfile::tempdir;

    fn test_db() -> Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn creates_workspace_and_registers_graph_node() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Primary".into()).unwrap();
        assert_eq!(workspace.name, "Primary");
        assert!(GraphService::exists(&db, &workspace.resource_ref()).unwrap());
    }

    #[test]
    fn deletes_workspace_and_unregisters_graph_node() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Temporary".into()).unwrap();
        WorkspaceService::delete(&db, &workspace.id).unwrap();
        assert!(!GraphService::exists(&db, &workspace.resource_ref()).unwrap());
    }
}
