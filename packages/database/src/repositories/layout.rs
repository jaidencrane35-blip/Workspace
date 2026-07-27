use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    Layout, LayoutBounds, LayoutId, LayoutMetadata, LayoutNode, Position2D, ResourceKind,
    ResourceRef, Size2D, Viewport, WorkspaceId,
};

/// Persistence for workspace spatial layouts (presentation only).
pub struct LayoutRepository<'a> {
    db: &'a Database,
}

impl<'a> LayoutRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(&self, layout: &Layout) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO layouts (id, workspace_id, viewport_origin_x, viewport_origin_y, viewport_width, viewport_height, viewport_zoom, metadata, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                layout.id.as_str(),
                layout.workspace_id.as_str(),
                layout.viewport.origin.x,
                layout.viewport.origin.y,
                layout.viewport.size.width,
                layout.viewport.size.height,
                layout.viewport.zoom,
                layout.metadata.as_ref().map(|m| m.as_str()),
                &layout.created_at,
                &layout.updated_at,
            ),
        )?;
        self.replace_nodes(&layout.id, &layout.nodes)?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &LayoutId) -> Result<Option<Layout>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, viewport_origin_x, viewport_origin_y, viewport_width, viewport_height, viewport_zoom, metadata, created_at, updated_at
             FROM layouts WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            let layout = map_layout_row(row)?;
            let nodes = self.list_nodes(&layout.id)?;
            return Ok(Some(Layout { nodes, ..layout }));
        }
        Ok(None)
    }

    pub fn get_by_workspace_id(&self, workspace_id: &WorkspaceId) -> Result<Option<Layout>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, viewport_origin_x, viewport_origin_y, viewport_width, viewport_height, viewport_zoom, metadata, created_at, updated_at
             FROM layouts WHERE workspace_id = ?1",
        )?;
        let mut rows = stmt.query([workspace_id.as_str()])?;
        if let Some(row) = rows.next()? {
            let layout = map_layout_row(row)?;
            let nodes = self.list_nodes(&layout.id)?;
            return Ok(Some(Layout { nodes, ..layout }));
        }
        Ok(None)
    }

    pub fn exists_for_workspace(&self, workspace_id: &WorkspaceId) -> Result<bool> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM layouts WHERE workspace_id = ?1",
            [workspace_id.as_str()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn update(&self, layout: &Layout) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE layouts SET viewport_origin_x = ?1, viewport_origin_y = ?2, viewport_width = ?3, viewport_height = ?4, viewport_zoom = ?5, metadata = ?6, updated_at = ?7
             WHERE id = ?8",
            (
                layout.viewport.origin.x,
                layout.viewport.origin.y,
                layout.viewport.size.width,
                layout.viewport.size.height,
                layout.viewport.zoom,
                layout.metadata.as_ref().map(|m| m.as_str()),
                &layout.updated_at,
                layout.id.as_str(),
            ),
        )?;
        if changed == 0 {
            return Ok(false);
        }
        self.replace_nodes(&layout.id, &layout.nodes)?;
        Ok(true)
    }

    pub fn delete(&self, id: &LayoutId) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM layouts WHERE id = ?1",
            [id.as_str()],
        )?;
        Ok(changed > 0)
    }

    fn replace_nodes(&self, layout_id: &LayoutId, nodes: &[LayoutNode]) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM layout_nodes WHERE layout_id = ?1",
            [layout_id.as_str()],
        )?;
        for node in nodes {
            self.db.connection().execute(
                "INSERT INTO layout_nodes (layout_id, resource_kind, resource_id, position_x, position_y, size_width, size_height, z_index, collapsed, hidden, locked, metadata)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                (
                    layout_id.as_str(),
                    kind_to_str(node.resource_ref.kind),
                    node.resource_ref.id.as_str(),
                    node.bounds.position.x,
                    node.bounds.position.y,
                    node.bounds.size.width,
                    node.bounds.size.height,
                    node.z_index,
                    i32::from(node.collapsed),
                    i32::from(node.hidden),
                    i32::from(node.locked),
                    node.metadata.as_ref().map(|m| m.as_str()),
                ),
            )?;
        }
        Ok(())
    }

    fn list_nodes(&self, layout_id: &LayoutId) -> Result<Vec<LayoutNode>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT resource_kind, resource_id, position_x, position_y, size_width, size_height, z_index, collapsed, hidden, locked, metadata
             FROM layout_nodes WHERE layout_id = ?1 ORDER BY z_index, resource_kind, resource_id",
        )?;
        let rows = stmt.query_map([layout_id.as_str()], map_layout_node_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn kind_to_str(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Workspace => "workspace",
        ResourceKind::Zone => "zone",
        ResourceKind::Application => "application",
        ResourceKind::Widget => "widget",
    }
}

fn parse_kind(value: &str) -> rusqlite::Result<ResourceKind> {
    match value {
        "workspace" => Ok(ResourceKind::Workspace),
        "zone" => Ok(ResourceKind::Zone),
        "application" => Ok(ResourceKind::Application),
        "widget" => Ok(ResourceKind::Widget),
        _ => Err(rusqlite::Error::InvalidColumnType(
            0,
            "resource_kind".into(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn map_layout_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Layout> {
    Ok(Layout {
        id: LayoutId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "workspace_id".into(), rusqlite::types::Type::Text)
        })?,
        viewport: Viewport {
            origin: Position2D::new(row.get(2)?, row.get(3)?),
            size: Size2D::new(row.get(4)?, row.get(5)?),
            zoom: row.get(6)?,
        },
        nodes: Vec::new(),
        metadata: row
            .get::<_, Option<String>>(7)?
            .map(|value| LayoutMetadata::new(value).expect("stored metadata is valid")),
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
    })
}

fn map_layout_node_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<LayoutNode> {
    use workspace_domain::ResourceId;

    let kind = parse_kind(&row.get::<_, String>(0)?)?;
    let id = ResourceId::new(row.get::<_, String>(1)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(1, "resource_id".into(), rusqlite::types::Type::Text)
    })?;

    Ok(LayoutNode {
        resource_ref: ResourceRef::new(kind, id),
        bounds: LayoutBounds {
            position: Position2D::new(row.get(2)?, row.get(3)?),
            size: Size2D::new(row.get(4)?, row.get(5)?),
        },
        z_index: row.get(6)?,
        collapsed: row.get::<_, i32>(7)? != 0,
        hidden: row.get::<_, i32>(8)? != 0,
        locked: row.get::<_, i32>(9)? != 0,
        metadata: row
            .get::<_, Option<String>>(10)?
            .map(|value| LayoutMetadata::new(value).expect("stored metadata is valid")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use crate::repositories::WorkspaceRepository;
    use tempfile::tempdir;
    use workspace_domain::Workspace;

    fn initialized_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    fn seed_workspace(db: &Database, id: &str) {
        WorkspaceRepository::new(db)
            .create(&Workspace {
                id: WorkspaceId::new(id).unwrap(),
                name: "Layout Test".into(),
                created_at: "2026-07-23T10:00:00Z".into(),
                updated_at: "2026-07-23T10:00:00Z".into(),
            })
            .unwrap();
    }

    #[test]
    fn creates_and_loads_layout_with_nodes() {
        let (_dir, db) = initialized_db();
        seed_workspace(&db, "ws-layout");
        let workspace_id = WorkspaceId::new("ws-layout").unwrap();
        let now = "2026-07-23T10:00:00Z".to_string();
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id: workspace_id.clone(),
            viewport: Viewport::default(),
            nodes: vec![LayoutNode {
                resource_ref: ResourceRef::new(
                    ResourceKind::Zone,
                    workspace_domain::ResourceId::new("zone-1").unwrap(),
                ),
                bounds: LayoutBounds {
                    position: Position2D::new(10.0, 20.0),
                    size: Size2D::new(200.0, 150.0),
                },
                z_index: 1,
                collapsed: false,
                hidden: false,
                locked: true,
                metadata: None,
            }],
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let repo = LayoutRepository::new(&db);
        repo.create(&layout).unwrap();
        let loaded = repo.get_by_workspace_id(&workspace_id).unwrap().unwrap();
        assert_eq!(loaded.nodes.len(), 1);
        assert!(loaded.nodes[0].locked);
    }

    #[test]
    fn rejects_duplicate_workspace_layout() {
        let (_dir, db) = initialized_db();
        seed_workspace(&db, "ws-dup");
        let workspace_id = WorkspaceId::new("ws-dup").unwrap();
        let now = "2026-07-23T10:00:00Z".to_string();
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id: workspace_id.clone(),
            viewport: Viewport::default(),
            nodes: vec![],
            metadata: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        let repo = LayoutRepository::new(&db);
        repo.create(&layout).unwrap();
        let duplicate = Layout {
            id: LayoutId::generate(),
            workspace_id: workspace_id.clone(),
            viewport: Viewport::default(),
            nodes: vec![],
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
        };
        assert!(repo.create(&duplicate).is_err());
    }

    #[test]
    fn updates_layout_nodes() {
        let (_dir, db) = initialized_db();
        seed_workspace(&db, "ws-update");
        let workspace_id = WorkspaceId::new("ws-update").unwrap();
        let now = "2026-07-23T10:00:00Z".to_string();
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id: workspace_id.clone(),
            viewport: Viewport::default(),
            nodes: vec![],
            metadata: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        let repo = LayoutRepository::new(&db);
        repo.create(&layout).unwrap();

        let updated_layout = Layout {
            nodes: vec![LayoutNode {
                resource_ref: ResourceRef::new(
                    ResourceKind::Widget,
                    workspace_domain::ResourceId::new("widget-1").unwrap(),
                ),
                bounds: LayoutBounds {
                    position: Position2D::new(1.0, 2.0),
                    size: Size2D::new(80.0, 60.0),
                },
                z_index: 2,
                collapsed: true,
                hidden: false,
                locked: false,
                metadata: None,
            }],
            updated_at: "2026-07-23T11:00:00Z".into(),
            ..layout
        };
        assert!(repo.update(&updated_layout).unwrap());
        let loaded = repo.get_by_id(&updated_layout.id).unwrap().unwrap();
        assert_eq!(loaded.nodes.len(), 1);
        assert!(loaded.nodes[0].collapsed);
    }
}
