//! Spatial layout domain — presentation state separate from graph topology (Sprint 13).

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ids::LayoutId;
use crate::resource::{ResourceId, ResourceKind, ResourceRef};
use crate::ids::WorkspaceId;

/// Layout-specific validation and persistence errors.
#[derive(Debug, Error, PartialEq)]
pub enum LayoutError {
    #[error("Layout not found")]
    NotFound,

    #[error("Duplicate layout for workspace")]
    DuplicateLayout,

    #[error("Duplicate layout node for resource: {resource_ref}")]
    DuplicateNode { resource_ref: String },

    #[error("Missing resource reference in layout node")]
    MissingResourceRef,

    #[error("Resource reference not found: {resource_ref}")]
    ResourceRefNotFound { resource_ref: String },

    #[error("Negative size: width={width}, height={height}")]
    NegativeSize { width: f64, height: f64 },

    #[error("Invalid coordinates: x={x}, y={y}")]
    InvalidCoordinates { x: f64, y: f64 },

    #[error("Invalid viewport: {reason}")]
    InvalidViewport { reason: String },

    #[error("Dangling layout node references unknown layout")]
    DanglingLayout,

    #[error("Layout metadata exceeds maximum length of {max}")]
    MetadataTooLong { max: usize },
}

pub type Result<T> = std::result::Result<T, LayoutError>;

const MAX_METADATA_LEN: usize = 4096;

/// Two-dimensional position in layout space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f64,
    pub y: f64,
}

impl Position2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn validate(&self) -> Result<()> {
        if !self.x.is_finite() || !self.y.is_finite() {
            return Err(LayoutError::InvalidCoordinates {
                x: self.x,
                y: self.y,
            });
        }
        Ok(())
    }
}

/// Two-dimensional size in layout space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size2D {
    pub width: f64,
    pub height: f64,
}

impl Size2D {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn validate(&self) -> Result<()> {
        if !self.width.is_finite()
            || !self.height.is_finite()
            || self.width < 0.0
            || self.height < 0.0
        {
            return Err(LayoutError::NegativeSize {
                width: self.width,
                height: self.height,
            });
        }
        Ok(())
    }
}

/// Position and size of a layout node.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayoutBounds {
    pub position: Position2D,
    pub size: Size2D,
}

impl LayoutBounds {
    pub fn validate(&self) -> Result<()> {
        self.position.validate()?;
        self.size.validate()
    }
}

/// Workspace viewport state (pan/zoom framing — not rendering).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub origin: Position2D,
    pub size: Size2D,
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            origin: Position2D::new(0.0, 0.0),
            size: Size2D::new(1920.0, 1080.0),
            zoom: 1.0,
        }
    }
}

impl Viewport {
    pub fn validate(&self) -> Result<()> {
        self.origin.validate()?;
        self.size.validate()?;
        if !self.zoom.is_finite() || self.zoom <= 0.0 {
            return Err(LayoutError::InvalidViewport {
                reason: "zoom must be a positive finite number".into(),
            });
        }
        Ok(())
    }
}

/// Optional opaque metadata attached to layouts or nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LayoutMetadata(pub String);

impl LayoutMetadata {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.len() > MAX_METADATA_LEN {
            return Err(LayoutError::MetadataTooLong {
                max: MAX_METADATA_LEN,
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Spatial placement of a graph-backed resource (references ResourceRef only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutNode {
    pub resource_ref: ResourceRef,
    pub bounds: LayoutBounds,
    pub z_index: i32,
    pub collapsed: bool,
    pub hidden: bool,
    pub locked: bool,
    pub metadata: Option<LayoutMetadata>,
}

impl LayoutNode {
    pub fn validate(&self) -> Result<()> {
        if self.resource_ref.id.as_str().trim().is_empty() {
            return Err(LayoutError::MissingResourceRef);
        }
        self.bounds.validate()?;
        if let Some(metadata) = &self.metadata {
            LayoutMetadata::new(metadata.as_str())?;
        }
        Ok(())
    }
}

/// Persisted workspace layout aggregate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    pub id: LayoutId,
    pub workspace_id: WorkspaceId,
    pub viewport: Viewport,
    pub nodes: Vec<LayoutNode>,
    pub metadata: Option<LayoutMetadata>,
    pub created_at: String,
    pub updated_at: String,
}

impl Layout {
    pub fn workspace_resource_ref(&self) -> ResourceRef {
        ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(self.workspace_id.as_str()).expect("workspace id is non-empty"),
        )
    }

    pub fn validate(&self) -> Result<()> {
        self.viewport.validate()?;
        if let Some(metadata) = &self.metadata {
            LayoutMetadata::new(metadata.as_str())?;
        }

        let mut seen = std::collections::BTreeSet::new();
        for node in &self.nodes {
            node.validate()?;
            let key = node.resource_ref.canonical();
            if !seen.insert(key.clone()) {
                return Err(LayoutError::DuplicateNode { resource_ref: key });
            }
        }
        Ok(())
    }
}

/// Point-in-time layout state (read-only snapshot).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutSnapshot {
    pub layout_id: LayoutId,
    pub workspace_id: WorkspaceId,
    pub viewport: Viewport,
    pub nodes: Vec<LayoutNode>,
    pub metadata: Option<LayoutMetadata>,
    pub captured_at: String,
}

impl LayoutSnapshot {
    pub fn from_layout(layout: &Layout, captured_at: String) -> Self {
        Self {
            layout_id: layout.id.clone(),
            workspace_id: layout.workspace_id.clone(),
            viewport: layout.viewport,
            nodes: layout.nodes.clone(),
            metadata: layout.metadata.clone(),
            captured_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_bounds_and_viewport() {
        let bounds = LayoutBounds {
            position: Position2D::new(10.0, 20.0),
            size: Size2D::new(100.0, 50.0),
        };
        assert!(bounds.validate().is_ok());

        let invalid = Size2D::new(-1.0, 10.0);
        assert!(matches!(
            invalid.validate(),
            Err(LayoutError::NegativeSize { .. })
        ));

        assert!(Viewport::default().validate().is_ok());
        let bad_zoom = Viewport {
            zoom: -1.0,
            ..Viewport::default()
        };
        assert!(matches!(
            bad_zoom.validate(),
            Err(LayoutError::InvalidViewport { .. })
        ));
    }

    #[test]
    fn rejects_non_finite_coordinates() {
        let position = Position2D::new(f64::NAN, 1.0);
        assert!(matches!(
            position.validate(),
            Err(LayoutError::InvalidCoordinates { .. })
        ));
    }

    #[test]
    fn layout_rejects_duplicate_nodes() {
        let resource = ResourceRef::new(
            ResourceKind::Zone,
            ResourceId::new("zone-1").unwrap(),
        );
        let node = LayoutNode {
            resource_ref: resource.clone(),
            bounds: LayoutBounds {
                position: Position2D::new(0.0, 0.0),
                size: Size2D::new(100.0, 100.0),
            },
            z_index: 0,
            collapsed: false,
            hidden: false,
            locked: false,
            metadata: None,
        };
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id: WorkspaceId::new("ws-1").unwrap(),
            viewport: Viewport::default(),
            nodes: vec![node.clone(), node],
            metadata: None,
            created_at: "2026-07-23T00:00:00Z".into(),
            updated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert!(matches!(
            layout.validate(),
            Err(LayoutError::DuplicateNode { .. })
        ));
    }

    #[test]
    fn rejects_metadata_too_long() {
        let long = "x".repeat(5000);
        assert!(matches!(
            LayoutMetadata::new(long),
            Err(LayoutError::MetadataTooLong { .. })
        ));
    }

    #[test]
    fn snapshot_preserves_layout_state() {
        let layout = Layout {
            id: LayoutId::generate(),
            workspace_id: WorkspaceId::new("ws-1").unwrap(),
            viewport: Viewport::default(),
            nodes: vec![],
            metadata: None,
            created_at: "2026-07-23T00:00:00Z".into(),
            updated_at: "2026-07-23T00:00:00Z".into(),
        };
        let snapshot = LayoutSnapshot::from_layout(&layout, "2026-07-23T01:00:00Z".into());
        assert_eq!(snapshot.workspace_id, layout.workspace_id);
        assert_eq!(snapshot.captured_at, "2026-07-23T01:00:00Z");
    }
}
