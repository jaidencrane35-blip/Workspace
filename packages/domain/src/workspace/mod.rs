use serde::{Deserialize, Serialize};

use crate::errors::{validate_resource_name, Result};
use crate::ids::{ApplicationId, WidgetId, WorkspaceId, ZoneId};
use crate::resource::{Addressable, ResourceId, ResourceKind, ResourceRef};

/// A user workspace container (domain entity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

/// A zone within a workspace (position metadata placeholder only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub id: ZoneId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub position_metadata: Option<String>,
}

/// Reference to an application associated with a workspace.
///
/// `identifier` is opaque metadata (bundle id, product key).
/// `executable_path` is the OS launch target used by governed launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationReference {
    pub id: ApplicationId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub identifier: Option<String>,
    pub executable_path: Option<String>,
}

/// Outcome of a governed application launch (authority + OS boundary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationLaunchResult {
    pub application_id: ApplicationId,
    pub name: String,
    pub executable_path: String,
    pub process_id: Option<u32>,
    /// True when the platform stub simulated launch (non-Windows / tests).
    pub simulated: bool,
}

/// Reference to a widget associated with a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetReference {
    pub id: WidgetId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub widget_type: Option<String>,
}

impl Workspace {
    pub fn validate_name(name: &str) -> Result<()> {
        validate_resource_name(name)
    }
}

impl Zone {
    pub fn validate_name(name: &str) -> Result<()> {
        validate_resource_name(name)
    }
}

impl ApplicationReference {
    pub fn validate_name(name: &str) -> Result<()> {
        validate_resource_name(name)
    }

    /// Returns a non-empty executable path suitable for launch, if configured.
    pub fn launch_executable(&self) -> Option<&str> {
        self.executable_path
            .as_deref()
            .map(str::trim)
            .filter(|path| !path.is_empty())
    }
}

impl WidgetReference {
    pub fn validate_name(name: &str) -> Result<()> {
        validate_resource_name(name)
    }
}

/// Maps each typed entity to its canonical `ResourceRef` (DEC-016).
/// Entity IDs are non-empty by construction, so `ResourceId` creation is infallible here.
impl Addressable for Workspace {
    fn resource_kind(&self) -> ResourceKind {
        ResourceKind::Workspace
    }

    fn resource_ref(&self) -> ResourceRef {
        ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(self.id.as_str()).expect("workspace id is non-empty"),
        )
    }
}

impl Addressable for Zone {
    fn resource_kind(&self) -> ResourceKind {
        ResourceKind::Zone
    }

    fn resource_ref(&self) -> ResourceRef {
        ResourceRef::new(
            ResourceKind::Zone,
            ResourceId::new(self.id.as_str()).expect("zone id is non-empty"),
        )
    }
}

impl Addressable for ApplicationReference {
    fn resource_kind(&self) -> ResourceKind {
        ResourceKind::Application
    }

    fn resource_ref(&self) -> ResourceRef {
        ResourceRef::new(
            ResourceKind::Application,
            ResourceId::new(self.id.as_str()).expect("application id is non-empty"),
        )
    }
}

impl Addressable for WidgetReference {
    fn resource_kind(&self) -> ResourceKind {
        ResourceKind::Widget
    }

    fn resource_ref(&self) -> ResourceRef {
        ResourceRef::new(
            ResourceKind::Widget,
            ResourceId::new(self.id.as_str()).expect("widget id is non-empty"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::DomainError;

    #[test]
    fn validates_workspace_name() {
        assert!(Workspace::validate_name("My Workspace").is_ok());
        assert_eq!(
            Workspace::validate_name("  ").unwrap_err(),
            DomainError::EmptyName
        );
    }

    #[test]
    fn entity_structs_are_constructible() {
        let workspace = Workspace {
            id: WorkspaceId::new("ws-1").unwrap(),
            name: "Default".into(),
            created_at: "2026-07-23T00:00:00Z".into(),
            updated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert_eq!(workspace.name, "Default");
    }

    #[test]
    fn entities_map_to_canonical_resource_refs() {
        let workspace = Workspace {
            id: WorkspaceId::new("ws-1").unwrap(),
            name: "Default".into(),
            created_at: "2026-07-23T00:00:00Z".into(),
            updated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert_eq!(workspace.resource_kind(), ResourceKind::Workspace);
        assert_eq!(workspace.resource_ref().canonical(), "workspace:ws-1");

        let zone = Zone {
            id: ZoneId::new("zone-1").unwrap(),
            workspace_id: WorkspaceId::new("ws-1").unwrap(),
            name: "Primary".into(),
            position_metadata: None,
        };
        assert_eq!(zone.resource_ref().canonical(), "zone:zone-1");
    }
}
