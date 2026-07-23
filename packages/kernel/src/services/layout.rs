//! Workspace layout service — spatial presentation state only (Sprint 13).
//!
//! References `ResourceRef` on layout nodes; never mutates or traverses the graph.

use chrono::Utc;
use workspace_database::{Database, LayoutRepository};
use workspace_domain::{
    Layout, LayoutId, LayoutNode, LayoutSnapshot, LayoutError, Viewport, WorkspaceId,
};

use crate::error::{KernelError, Result};

pub struct LayoutService;

impl LayoutService {
    pub fn create(db: &Database, workspace_id: WorkspaceId) -> Result<Layout> {
        if LayoutRepository::new(db).exists_for_workspace(&workspace_id)? {
            return Err(map_layout_error(LayoutError::DuplicateLayout));
        }

        let now = Utc::now().to_rfc3339();
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id,
            viewport: Viewport::default(),
            nodes: Vec::new(),
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
        };

        layout.validate().map_err(map_layout_error)?;
        LayoutRepository::new(db).create(&layout)?;
        Ok(layout)
    }

    pub fn load(db: &Database, id: &LayoutId) -> Result<Layout> {
        LayoutRepository::new(db)
            .get_by_id(id)?
            .ok_or_else(|| map_layout_error(LayoutError::NotFound))
    }

    pub fn load_by_workspace(db: &Database, workspace_id: &WorkspaceId) -> Result<Layout> {
        LayoutRepository::new(db)
            .get_by_workspace_id(workspace_id)?
            .ok_or_else(|| map_layout_error(LayoutError::NotFound))
    }

    pub fn update(
        db: &Database,
        id: &LayoutId,
        viewport: Viewport,
        nodes: Vec<LayoutNode>,
        metadata: Option<workspace_domain::LayoutMetadata>,
    ) -> Result<Layout> {
        let mut layout = Self::load(db, id)?;
        layout.viewport = viewport;
        layout.nodes = nodes;
        layout.metadata = metadata;
        layout.updated_at = Utc::now().to_rfc3339();
        layout.validate().map_err(map_layout_error)?;

        let updated = LayoutRepository::new(db).update(&layout)?;
        if !updated {
            return Err(map_layout_error(LayoutError::NotFound));
        }
        Self::load(db, id)
    }

    pub fn delete(db: &Database, id: &LayoutId) -> Result<()> {
        let deleted = LayoutRepository::new(db).delete(id)?;
        if !deleted {
            return Err(map_layout_error(LayoutError::NotFound));
        }
        Ok(())
    }

    pub fn reset(db: &Database, id: &LayoutId) -> Result<Layout> {
        let mut layout = Self::load(db, id)?;
        layout.viewport = Viewport::default();
        layout.nodes.clear();
        layout.metadata = None;
        layout.updated_at = Utc::now().to_rfc3339();
        layout.validate().map_err(map_layout_error)?;

        LayoutRepository::new(db)
            .update(&layout)
            .map_err(KernelError::Database)?;
        Ok(layout)
    }

    pub fn validate(layout: &Layout) -> Result<()> {
        layout.validate().map_err(map_layout_error)
    }

    pub fn exists_for_workspace(db: &Database, workspace_id: &WorkspaceId) -> Result<bool> {
        LayoutRepository::new(db)
            .exists_for_workspace(workspace_id)
            .map_err(Into::into)
    }

    pub fn lookup_by_workspace(
        db: &Database,
        workspace_id: &WorkspaceId,
    ) -> Result<Option<Layout>> {
        LayoutRepository::new(db)
            .get_by_workspace_id(workspace_id)
            .map_err(Into::into)
    }

    pub fn snapshot(db: &Database, id: &LayoutId) -> Result<LayoutSnapshot> {
        let layout = Self::load(db, id)?;
        Ok(LayoutSnapshot::from_layout(
            &layout,
            Utc::now().to_rfc3339(),
        ))
    }
}

fn map_layout_error(error: LayoutError) -> KernelError {
    match error {
        LayoutError::NotFound => KernelError::LayoutNotFound,
        LayoutError::DuplicateLayout => KernelError::DuplicateLayout,
        LayoutError::DuplicateNode { resource_ref } => KernelError::LayoutValidation {
            message: format!("duplicate layout node for {resource_ref}"),
        },
        LayoutError::MissingResourceRef => KernelError::LayoutValidation {
            message: "missing resource reference in layout node".into(),
        },
        LayoutError::ResourceRefNotFound { resource_ref } => KernelError::LayoutValidation {
            message: format!("resource reference not found: {resource_ref}"),
        },
        LayoutError::NegativeSize { width, height } => KernelError::LayoutValidation {
            message: format!("negative size: width={width}, height={height}"),
        },
        LayoutError::InvalidCoordinates { x, y } => KernelError::LayoutValidation {
            message: format!("invalid coordinates: x={x}, y={y}"),
        },
        LayoutError::InvalidViewport { reason } => KernelError::LayoutValidation { message: reason },
        LayoutError::DanglingLayout => KernelError::LayoutValidation {
            message: "dangling layout node references unknown layout".into(),
        },
        LayoutError::MetadataTooLong { max } => KernelError::LayoutValidation {
            message: format!("layout metadata exceeds maximum length of {max}"),
        },
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
    fn creates_and_resets_layout() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Layout WS".into()).unwrap();
        let layout = LayoutService::create(&db, workspace.id.clone()).unwrap();
        assert!(layout.nodes.is_empty());

        LayoutService::reset(&db, &layout.id).unwrap();
        let loaded = LayoutService::load(&db, &layout.id).unwrap();
        assert_eq!(loaded.viewport, Viewport::default());
    }

    #[test]
    fn rejects_duplicate_layout_for_workspace() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Dup Layout".into()).unwrap();
        LayoutService::create(&db, workspace.id.clone()).unwrap();
        let error = LayoutService::create(&db, workspace.id).unwrap_err();
        assert!(matches!(error, KernelError::DuplicateLayout));
    }
}
