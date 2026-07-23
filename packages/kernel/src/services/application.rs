//! Application resource service — persistence and graph registration.

use workspace_database::{ApplicationRepository, Database};
use workspace_domain::{
    Addressable, ApplicationId, ApplicationReference, ResourceId, ResourceKind, ResourceRef,
    WorkspaceId,
};

use super::GraphService;
use crate::error::{KernelError, Result};

pub struct ApplicationService;

impl ApplicationService {
    pub fn create(
        db: &Database,
        workspace_id: WorkspaceId,
        name: String,
        identifier: Option<String>,
    ) -> Result<ApplicationReference> {
        ApplicationReference::validate_name(&name).map_err(KernelError::from)?;

        let application = ApplicationReference {
            id: ApplicationId::generate(),
            workspace_id: workspace_id.clone(),
            name: name.trim().to_string(),
            identifier,
        };

        ApplicationRepository::new(db).create(&application)?;

        let parent = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(workspace_id.as_str()).expect("workspace id is non-empty"),
        );
        GraphService::register_child(db, &parent, &application.resource_ref())?;

        Ok(application)
    }

    pub fn load(db: &Database, id: &ApplicationId) -> Result<ApplicationReference> {
        ApplicationRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::ApplicationNotFound)
    }

    pub fn update(
        db: &Database,
        id: &ApplicationId,
        name: String,
        identifier: Option<String>,
    ) -> Result<ApplicationReference> {
        ApplicationReference::validate_name(&name).map_err(KernelError::from)?;
        let updated = ApplicationRepository::new(db).update(
            id,
            name.trim(),
            identifier.as_deref(),
        )?;
        if !updated {
            return Err(KernelError::ApplicationNotFound);
        }
        Self::load(db, id)
    }

    pub fn delete(db: &Database, id: &ApplicationId) -> Result<()> {
        let application = Self::load(db, id)?;
        let deleted = ApplicationRepository::new(db).delete(id)?;
        if !deleted {
            return Err(KernelError::ApplicationNotFound);
        }
        GraphService::unregister_node(db, &application.resource_ref())?;
        Ok(())
    }

    pub fn validate(name: &str) -> Result<()> {
        ApplicationReference::validate_name(name).map_err(KernelError::from)
    }

    pub fn exists(db: &Database, id: &ApplicationId) -> Result<bool> {
        ApplicationRepository::new(db).exists(id).map_err(Into::into)
    }

    pub fn lookup(db: &Database, id: &ApplicationId) -> Result<Option<ApplicationReference>> {
        ApplicationRepository::new(db)
            .get_by_id(id)
            .map_err(Into::into)
    }

    pub fn list_by_workspace(
        db: &Database,
        workspace_id: &WorkspaceId,
    ) -> Result<Vec<ApplicationReference>> {
        ApplicationRepository::new(db)
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
    fn creates_application_and_registers_in_graph() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Dev".into()).unwrap();
        let app = ApplicationService::create(
            &db,
            workspace.id.clone(),
            "Terminal".into(),
            Some("com.example.terminal".into()),
        )
        .unwrap();

        assert!(GraphService::exists(&db, &app.resource_ref()).unwrap());
    }
}
