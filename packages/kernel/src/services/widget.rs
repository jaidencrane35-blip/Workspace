//! Widget resource service — persistence and graph registration.

use workspace_database::{Database, WidgetRepository};
use workspace_domain::{
    Addressable, ResourceId, ResourceKind, ResourceRef, WidgetId, WidgetReference, WorkspaceId,
};

use super::GraphService;
use crate::error::{KernelError, Result};

pub struct WidgetService;

impl WidgetService {
    pub fn create(
        db: &Database,
        workspace_id: WorkspaceId,
        name: String,
        widget_type: Option<String>,
    ) -> Result<WidgetReference> {
        WidgetReference::validate_name(&name).map_err(KernelError::from)?;

        let widget = WidgetReference {
            id: WidgetId::generate(),
            workspace_id: workspace_id.clone(),
            name: name.trim().to_string(),
            widget_type,
        };

        WidgetRepository::new(db).create(&widget)?;

        let parent = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(workspace_id.as_str()).expect("workspace id is non-empty"),
        );
        GraphService::register_child(db, &parent, &widget.resource_ref())?;

        Ok(widget)
    }

    pub fn load(db: &Database, id: &WidgetId) -> Result<WidgetReference> {
        WidgetRepository::new(db)
            .get_by_id(id)?
            .ok_or(KernelError::WidgetNotFound)
    }

    pub fn update(
        db: &Database,
        id: &WidgetId,
        name: String,
        widget_type: Option<String>,
    ) -> Result<WidgetReference> {
        WidgetReference::validate_name(&name).map_err(KernelError::from)?;
        let updated = WidgetRepository::new(db).update(
            id,
            name.trim(),
            widget_type.as_deref(),
        )?;
        if !updated {
            return Err(KernelError::WidgetNotFound);
        }
        Self::load(db, id)
    }

    pub fn delete(db: &Database, id: &WidgetId) -> Result<()> {
        let widget = Self::load(db, id)?;
        let deleted = WidgetRepository::new(db).delete(id)?;
        if !deleted {
            return Err(KernelError::WidgetNotFound);
        }
        GraphService::unregister_node(db, &widget.resource_ref())?;
        Ok(())
    }

    pub fn validate(name: &str) -> Result<()> {
        WidgetReference::validate_name(name).map_err(KernelError::from)
    }

    pub fn exists(db: &Database, id: &WidgetId) -> Result<bool> {
        WidgetRepository::new(db).exists(id).map_err(Into::into)
    }

    pub fn lookup(db: &Database, id: &WidgetId) -> Result<Option<WidgetReference>> {
        WidgetRepository::new(db).get_by_id(id).map_err(Into::into)
    }

    pub fn list_by_workspace(
        db: &Database,
        workspace_id: &WorkspaceId,
    ) -> Result<Vec<WidgetReference>> {
        WidgetRepository::new(db)
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
    fn creates_widget_and_registers_in_graph() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Home".into()).unwrap();
        let widget = WidgetService::create(
            &db,
            workspace.id.clone(),
            "Clock".into(),
            Some("clock".into()),
        )
        .unwrap();

        assert!(GraphService::exists(&db, &widget.resource_ref()).unwrap());
    }
}
