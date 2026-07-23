//! Workspace domain operations — coordinates repositories and events.

use chrono::Utc;
use workspace_database::{Database, WorkspaceRepository};
use workspace_domain::{Workspace, WorkspaceId};

use crate::error::{KernelError, Result};

/// Kernel-owned workspace domain service.
pub struct WorkspaceService;

impl WorkspaceService {
    pub fn create(db: &Database, name: String) -> Result<Workspace> {
        Workspace::validate_name(&name)?;

        let now = Utc::now().to_rfc3339();
        let workspace = Workspace {
            id: WorkspaceId::generate(),
            name: name.trim().to_string(),
            created_at: now.clone(),
            updated_at: now,
        };

        WorkspaceRepository::new(db).create(&workspace)?;

        Ok(workspace)
    }

    pub fn get(db: &Database, id: &WorkspaceId) -> Result<Workspace> {
        WorkspaceRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::WorkspaceNotFound)
    }

    pub fn list(db: &Database) -> Result<Vec<Workspace>> {
        WorkspaceRepository::new(db).list().map_err(Into::into)
    }

    pub fn update_name(db: &Database, id: &WorkspaceId, name: String) -> Result<Workspace> {
        Workspace::validate_name(&name)?;
        let updated_at = Utc::now().to_rfc3339();
        let trimmed = name.trim().to_string();

        db.transaction(|tx| {
            let updated = WorkspaceRepository::update_name_in_transaction(
                tx,
                id,
                &trimmed,
                &updated_at,
            )?;
            if !updated {
                return Err(workspace_database::DatabaseError::NotFound(
                    "workspace".into(),
                ));
            }

            WorkspaceRepository::get_by_id_in_transaction(tx, id)?
                .ok_or_else(|| workspace_database::DatabaseError::NotFound("workspace".into()))
        })
        .map_err(|error| match error {
            workspace_database::DatabaseError::NotFound(_) => KernelError::WorkspaceNotFound,
            other => KernelError::Database(other),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let workspace = WorkspaceService::create(&db, "Primary".into()).unwrap();
        assert_eq!(workspace.name, "Primary");
    }
}
