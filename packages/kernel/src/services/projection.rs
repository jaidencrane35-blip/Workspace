//! Derived workspace state projection — read-only, non-authoritative (Sprint 14).

use chrono::Utc;
use workspace_database::Database;
use workspace_domain::{
    Addressable, ApplicationSummary, LayoutPlacementSummary, ProjectionError,
    ProjectionRelationship, WidgetSummary, WorkspaceId, WorkspaceSnapshot, ZoneSummary,
};

use super::{
    ApplicationService, GraphService, LayoutService, WidgetService, WorkspaceService, ZoneService,
};
use crate::error::{KernelError, Result};

pub struct WorkspaceProjectionService;

impl WorkspaceProjectionService {
    /// Gathers authoritative domain state and constructs a validated snapshot.
    pub fn build_snapshot(db: &Database, workspace_id: &WorkspaceId) -> Result<WorkspaceSnapshot> {
        let workspace = WorkspaceService::load(db, workspace_id)?;
        let workspace_ref = workspace.resource_ref();

        let zones = ZoneService::list_by_workspace(db, workspace_id)?;
        let applications = ApplicationService::list_by_workspace(db, workspace_id)?;
        let widgets = WidgetService::list_by_workspace(db, workspace_id)?;

        let layout = LayoutService::lookup_by_workspace(db, workspace_id)?;
        let (layout_id, layout_placements) = match layout {
            Some(layout) => {
                let placements = layout
                    .nodes
                    .iter()
                    .map(|node| LayoutPlacementSummary {
                        resource_ref: node.resource_ref.clone(),
                        z_index: node.z_index,
                        collapsed: node.collapsed,
                        hidden: node.hidden,
                        locked: node.locked,
                        position_x: node.bounds.position.x,
                        position_y: node.bounds.position.y,
                        width: node.bounds.size.width,
                        height: node.bounds.size.height,
                    })
                    .collect();
                (Some(layout.id), placements)
            }
            None => (None, Vec::new()),
        };

        let relationships = GraphService::list_edges_from(db, &workspace_ref)?
            .into_iter()
            .map(|edge| ProjectionRelationship {
                source: edge.source,
                relationship: edge.relationship,
                target: edge.target,
            })
            .collect();

        let snapshot = WorkspaceSnapshot {
            workspace: workspace_ref,
            workspace_name: workspace.name,
            zones: zones
                .iter()
                .map(|zone| ZoneSummary {
                    resource_ref: zone.resource_ref(),
                    name: zone.name.clone(),
                })
                .collect(),
            applications: applications
                .iter()
                .map(|application| ApplicationSummary {
                    resource_ref: application.resource_ref(),
                    name: application.name.clone(),
                    identifier: application.identifier.clone(),
                })
                .collect(),
            widgets: widgets
                .iter()
                .map(|widget| WidgetSummary {
                    resource_ref: widget.resource_ref(),
                    name: widget.name.clone(),
                    widget_type: widget.widget_type.clone(),
                })
                .collect(),
            layout_id,
            layout_placements,
            relationships,
            generated_at: Utc::now().to_rfc3339(),
        };

        snapshot.validate().map_err(map_projection_error)?;
        Ok(snapshot)
    }
}

fn map_projection_error(error: ProjectionError) -> KernelError {
    KernelError::ProjectionValidation {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{
        ApplicationService, LayoutService, WidgetService, WorkspaceService, ZoneService,
    };
    use workspace_database::DatabaseService;
    use workspace_domain::{
        LayoutBounds, LayoutNode, Position2D, ResourceKind, ResourceRef, Size2D, Viewport,
    };
    use tempfile::tempdir;

    fn test_db() -> workspace_database::Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn builds_snapshot_from_domain_services() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Projection".into()).unwrap();
        let zone = ZoneService::create(&db, workspace.id.clone(), "Primary".into(), None).unwrap();
        ApplicationService::create(
            &db,
            workspace.id.clone(),
            "Terminal".into(),
            Some("com.example.terminal".into()),
        )
        .unwrap();
        WidgetService::create(
            &db,
            workspace.id.clone(),
            "Clock".into(),
            Some("clock".into()),
        )
        .unwrap();

        let layout = LayoutService::create(&db, workspace.id.clone()).unwrap();
        LayoutService::update(
            &db,
            &layout.id,
            Viewport::default(),
            vec![LayoutNode {
                resource_ref: zone.resource_ref(),
                bounds: LayoutBounds {
                    position: Position2D::new(10.0, 20.0),
                    size: Size2D::new(200.0, 100.0),
                },
                z_index: 1,
                collapsed: false,
                hidden: false,
                locked: false,
                metadata: None,
            }],
            None,
        )
        .unwrap();

        let snapshot =
            WorkspaceProjectionService::build_snapshot(&db, &workspace.id).unwrap();

        assert_eq!(snapshot.workspace_name, "Projection");
        assert_eq!(snapshot.zones.len(), 1);
        assert_eq!(snapshot.applications.len(), 1);
        assert_eq!(snapshot.widgets.len(), 1);
        assert_eq!(snapshot.relationships.len(), 3);
        assert_eq!(snapshot.layout_placements.len(), 1);
        assert!(snapshot.layout_id.is_some());
        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn handles_missing_layout_gracefully() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "No Layout".into()).unwrap();
        ZoneService::create(&db, workspace.id.clone(), "Only Zone".into(), None).unwrap();

        let snapshot =
            WorkspaceProjectionService::build_snapshot(&db, &workspace.id).unwrap();

        assert!(snapshot.layout_id.is_none());
        assert!(snapshot.layout_placements.is_empty());
        assert_eq!(snapshot.zones.len(), 1);
    }

    #[test]
    fn rejects_inconsistent_graph_edge() {
        use workspace_database::GraphRepository;
        use workspace_domain::GraphRelationship;

        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Inconsistent".into()).unwrap();
        let orphan = ResourceRef::new(
            ResourceKind::Zone,
            workspace_domain::ResourceId::new("orphan-zone").unwrap(),
        );
        let repo = GraphRepository::new(&db);
        repo.register_node(&orphan).unwrap();
        repo.add_edge(
            &workspace.resource_ref(),
            GraphRelationship::Contains,
            &orphan,
        )
        .unwrap();

        let error = WorkspaceProjectionService::build_snapshot(&db, &workspace.id).unwrap_err();
        assert!(matches!(error, KernelError::ProjectionValidation { .. }));
    }
}
