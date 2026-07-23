//! Read-only workspace state projection (Sprint 14).
//!
//! Derived, non-authoritative snapshots for future consumers. Not persisted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::graph::GraphRelationship;
use crate::ids::LayoutId;
use crate::resource::{ResourceKind, ResourceRef};

/// Projection-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProjectionError {
    #[error("Duplicate resource reference in snapshot: {0}")]
    DuplicateResourceRef(String),

    #[error("Relationship references unknown resource: {0}")]
    UnknownRelationshipTarget(String),

    #[error("Layout placement references unknown resource: {0}")]
    UnknownLayoutResource(String),

    #[error("Workspace resource reference mismatch")]
    WorkspaceRefMismatch,
}

pub type Result<T> = std::result::Result<T, ProjectionError>;

/// Lightweight zone representation in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneSummary {
    pub resource_ref: ResourceRef,
    pub name: String,
}

/// Lightweight application representation in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationSummary {
    pub resource_ref: ResourceRef,
    pub name: String,
    pub identifier: Option<String>,
}

/// Lightweight widget representation in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetSummary {
    pub resource_ref: ResourceRef,
    pub name: String,
    pub widget_type: Option<String>,
}

/// Graph relationship included in a workspace snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionRelationship {
    pub source: ResourceRef,
    pub relationship: GraphRelationship,
    pub target: ResourceRef,
}

/// Spatial placement reference from layout (not full layout state).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutPlacementSummary {
    pub resource_ref: ResourceRef,
    pub z_index: i32,
    pub collapsed: bool,
    pub hidden: bool,
    pub locked: bool,
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
    pub height: f64,
}

/// Derived read model of current workspace state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSnapshot {
    pub workspace: ResourceRef,
    pub workspace_name: String,
    pub zones: Vec<ZoneSummary>,
    pub applications: Vec<ApplicationSummary>,
    pub widgets: Vec<WidgetSummary>,
    pub layout_id: Option<LayoutId>,
    pub layout_placements: Vec<LayoutPlacementSummary>,
    pub relationships: Vec<ProjectionRelationship>,
    pub generated_at: String,
}

impl WorkspaceSnapshot {
    pub fn validate(&self) -> Result<()> {
        let mut seen = BTreeSet::new();
        if !seen.insert(self.workspace.canonical()) {
            return Err(ProjectionError::DuplicateResourceRef(
                self.workspace.canonical(),
            ));
        }

        for zone in &self.zones {
            if !seen.insert(zone.resource_ref.canonical()) {
                return Err(ProjectionError::DuplicateResourceRef(
                    zone.resource_ref.canonical(),
                ));
            }
        }
        for app in &self.applications {
            if !seen.insert(app.resource_ref.canonical()) {
                return Err(ProjectionError::DuplicateResourceRef(
                    app.resource_ref.canonical(),
                ));
            }
        }
        for widget in &self.widgets {
            if !seen.insert(widget.resource_ref.canonical()) {
                return Err(ProjectionError::DuplicateResourceRef(
                    widget.resource_ref.canonical(),
                ));
            }
        }

        for relationship in &self.relationships {
            if relationship.source != self.workspace {
                return Err(ProjectionError::WorkspaceRefMismatch);
            }
            if !seen.contains(relationship.target.canonical().as_str()) {
                return Err(ProjectionError::UnknownRelationshipTarget(
                    relationship.target.canonical(),
                ));
            }
        }

        for placement in &self.layout_placements {
            if !seen.contains(placement.resource_ref.canonical().as_str()) {
                return Err(ProjectionError::UnknownLayoutResource(
                    placement.resource_ref.canonical(),
                ));
            }
        }

        Ok(())
    }

    pub fn resource_refs(&self) -> Vec<&ResourceRef> {
        let mut refs = vec![&self.workspace];
        refs.extend(self.zones.iter().map(|z| &z.resource_ref));
        refs.extend(self.applications.iter().map(|a| &a.resource_ref));
        refs.extend(self.widgets.iter().map(|w| &w.resource_ref));
        refs
    }

    pub fn contains_kind(&self, kind: ResourceKind) -> bool {
        match kind {
            ResourceKind::Workspace => true,
            ResourceKind::Zone => !self.zones.is_empty(),
            ResourceKind::Application => !self.applications.is_empty(),
            ResourceKind::Widget => !self.widgets.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ResourceId;

    fn workspace_ref(id: &str) -> ResourceRef {
        ResourceRef::new(ResourceKind::Workspace, ResourceId::new(id).unwrap())
    }

    fn zone_ref(id: &str) -> ResourceRef {
        ResourceRef::new(ResourceKind::Zone, ResourceId::new(id).unwrap())
    }

    #[test]
    fn validates_consistent_snapshot() {
        let workspace = workspace_ref("ws-1");
        let zone = zone_ref("zone-1");
        let snapshot = WorkspaceSnapshot {
            workspace: workspace.clone(),
            workspace_name: "Main".into(),
            zones: vec![ZoneSummary {
                resource_ref: zone.clone(),
                name: "Primary".into(),
            }],
            applications: vec![],
            widgets: vec![],
            layout_id: None,
            layout_placements: vec![],
            relationships: vec![ProjectionRelationship {
                source: workspace.clone(),
                relationship: GraphRelationship::Contains,
                target: zone,
            }],
            generated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn rejects_unknown_relationship_target() {
        let workspace = workspace_ref("ws-1");
        let snapshot = WorkspaceSnapshot {
            workspace: workspace.clone(),
            workspace_name: "Main".into(),
            zones: vec![],
            applications: vec![],
            widgets: vec![],
            layout_id: None,
            layout_placements: vec![],
            relationships: vec![ProjectionRelationship {
                source: workspace,
                relationship: GraphRelationship::Contains,
                target: zone_ref("missing"),
            }],
            generated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert!(matches!(
            snapshot.validate(),
            Err(ProjectionError::UnknownRelationshipTarget(_))
        ));
    }

    #[test]
    fn rejects_duplicate_resource_refs() {
        let zone = zone_ref("zone-dup");
        let snapshot = WorkspaceSnapshot {
            workspace: workspace_ref("ws-1"),
            workspace_name: "Main".into(),
            zones: vec![
                ZoneSummary {
                    resource_ref: zone.clone(),
                    name: "A".into(),
                },
                ZoneSummary {
                    resource_ref: zone,
                    name: "B".into(),
                },
            ],
            applications: vec![],
            widgets: vec![],
            layout_id: None,
            layout_placements: vec![],
            relationships: vec![],
            generated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert!(matches!(
            snapshot.validate(),
            Err(ProjectionError::DuplicateResourceRef(_))
        ));
    }
}
