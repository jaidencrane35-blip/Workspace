//! Workspace domain operations — coordinates repositories and events.

use chrono::Utc;
use uuid::Uuid;
use workspace_database::{Database, WorkspaceRepository};
use workspace_domain::Workspace;

use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceEntityCreated, WorkspaceEntityUpdated};
use crate::events::EventBus;

/// Kernel-owned workspace domain service.
pub struct WorkspaceService;

impl WorkspaceService {
    pub fn create(
        db: &Database,
        event_bus: &EventBus,
        name: String,
    ) -> Result<Workspace> {
        Workspace::validate_name(&name)?;

        let now = Utc::now().to_rfc3339();
        let workspace = Workspace {
            id: Uuid::new_v4().to_string(),
            name: name.trim().to_string(),
            created_at: now.clone(),
            updated_at: now,
        };

        WorkspaceRepository::new(db).create(&workspace)?;

        event_bus.publish(DomainEvent::WorkspaceCreated(WorkspaceEntityCreated {
            workspace_id: workspace.id.clone(),
            name: workspace.name.clone(),
        }));

        Ok(workspace)
    }

    pub fn get(db: &Database, id: &str) -> Result<Workspace> {
        if id.trim().is_empty() {
            return Err(KernelError::Domain(workspace_domain::DomainError::InvalidId));
        }

        WorkspaceRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::WorkspaceNotFound)
    }

    pub fn list(db: &Database) -> Result<Vec<Workspace>> {
        WorkspaceRepository::new(db).list().map_err(Into::into)
    }

    pub fn update_name(
        db: &Database,
        event_bus: &EventBus,
        id: &str,
        name: String,
    ) -> Result<Workspace> {
        Workspace::validate_name(&name)?;
        let updated_at = Utc::now().to_rfc3339();
        let trimmed = name.trim().to_string();

        let updated = WorkspaceRepository::new(db).update_name(id, &trimmed, &updated_at)?;
        if !updated {
            return Err(KernelError::WorkspaceNotFound);
        }

        let workspace = Self::get(db, id)?;

        event_bus.publish(DomainEvent::WorkspaceUpdated(WorkspaceEntityUpdated {
            workspace_id: workspace.id.clone(),
            name: workspace.name.clone(),
        }));

        Ok(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventBus;
    use workspace_database::DatabaseService;
    use tempfile::tempdir;

    fn test_db() -> workspace_database::Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn creates_workspace_via_service() {
        let db = test_db();
        let bus = EventBus::new();
        let workspace = WorkspaceService::create(&db, &bus, "Primary".into()).unwrap();
        assert_eq!(workspace.name, "Primary");
    }
}
