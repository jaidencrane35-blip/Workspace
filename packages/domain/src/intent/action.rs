//! Workspace action intents — explicit "what to do" metadata (Sprint 15).
//!
//! Distinct from motivation [`super::Intent`] ("why" an action runs).

use std::collections::BTreeSet;
use std::sync::LazyLock;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::capability::Capability;
use crate::ids::ActionIntentId;
use crate::resource::{ResourceKind, ResourceRef};

pub type Result<T> = std::result::Result<T, ActionIntentError>;

/// Classification of workspace actions for future consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionIntentCategory {
    Workspace,
    Zone,
    Application,
    Widget,
    Layout,
    Settings,
    Audit,
    System,
}

/// Whether an action intent requires a target [`ResourceRef`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetRequirement {
    None,
    Optional,
    Required,
}

/// Optional metadata attached to an action intent request.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActionIntentMetadata {
    pub label: Option<String>,
    pub notes: Option<String>,
}

/// Static definition of a supported workspace action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionIntentDefinition {
    pub id: ActionIntentId,
    pub name: String,
    pub description: String,
    pub category: ActionIntentCategory,
    pub capability_required: Capability,
    pub command_name: &'static str,
    pub target_requirement: TargetRequirement,
    pub allowed_target_kinds: &'static [ResourceKind],
}

/// Runtime action intent — metadata describing what an actor wants to do.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionIntentRequest {
    pub intent_id: ActionIntentId,
    pub target_resource: Option<ResourceRef>,
    pub metadata: ActionIntentMetadata,
    pub created_at: String,
}

/// Action intent validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ActionIntentError {
    #[error("Action intent not found: {0}")]
    NotFound(String),

    #[error("Duplicate action intent id in registry: {0}")]
    DuplicateId(String),

    #[error("Action intent requires a target resource")]
    MissingTarget,

    #[error("Action intent does not accept a target resource")]
    UnexpectedTarget,

    #[error("Target resource kind {kind} is not allowed for this intent")]
    InvalidTargetKind { kind: ResourceKind },

    #[error("Required capability mismatch: expected {expected}, got {actual}")]
    CapabilityMismatch { expected: String, actual: String },

    #[error("Invalid action intent metadata: {message}")]
    InvalidMetadata { message: String },
}

impl ActionIntentRequest {
    pub fn new(intent_id: ActionIntentId) -> Self {
        Self {
            intent_id,
            target_resource: None,
            metadata: ActionIntentMetadata::default(),
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn with_target(mut self, target: ResourceRef) -> Self {
        self.target_resource = Some(target);
        self
    }

    pub fn with_metadata(mut self, metadata: ActionIntentMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn validate(&self, definition: &ActionIntentDefinition) -> Result<()> {
        if self.intent_id != definition.id {
            return Err(ActionIntentError::NotFound(self.intent_id.to_string()));
        }

        if let Some(label) = &self.metadata.label {
            if label.len() > 256 {
                return Err(ActionIntentError::InvalidMetadata {
                    message: "label exceeds maximum length of 256".into(),
                });
            }
        }

        if let Some(notes) = &self.metadata.notes {
            if notes.len() > 4096 {
                return Err(ActionIntentError::InvalidMetadata {
                    message: "notes exceed maximum length of 4096".into(),
                });
            }
        }

        match definition.target_requirement {
            TargetRequirement::None => {
                if self.target_resource.is_some() {
                    return Err(ActionIntentError::UnexpectedTarget);
                }
            }
            TargetRequirement::Required => {
                let target = self
                    .target_resource
                    .as_ref()
                    .ok_or(ActionIntentError::MissingTarget)?;
                Self::validate_target_kind(target, definition)?;
            }
            TargetRequirement::Optional => {
                if let Some(target) = &self.target_resource {
                    Self::validate_target_kind(target, definition)?;
                }
            }
        }

        Ok(())
    }

    fn validate_target_kind(
        target: &ResourceRef,
        definition: &ActionIntentDefinition,
    ) -> Result<()> {
        if definition.allowed_target_kinds.is_empty() {
            return Ok(());
        }

        if definition
            .allowed_target_kinds
            .iter()
            .any(|kind| *kind == target.kind)
        {
            Ok(())
        } else {
            Err(ActionIntentError::InvalidTargetKind {
                kind: target.kind,
            })
        }
    }
}

impl ActionIntentDefinition {
    pub fn matches_capability(&self, capability: &Capability) -> bool {
        self.capability_required.id == capability.id
    }
}

/// Static registry of supported workspace action intents.
pub struct ActionIntentRegistry;

impl ActionIntentRegistry {
    pub fn all() -> Vec<ActionIntentDefinition> {
        DEFINITIONS.clone()
    }

    pub fn lookup(id: &ActionIntentId) -> Option<ActionIntentDefinition> {
        DEFINITIONS
            .iter()
            .find(|definition| &definition.id == id)
            .cloned()
    }

    pub fn validate_registry() -> Result<()> {
        let mut seen = BTreeSet::new();
        for definition in DEFINITIONS.iter() {
            if !seen.insert(definition.id.as_str()) {
                return Err(ActionIntentError::DuplicateId(
                    definition.id.to_string(),
                ));
            }
        }
        Ok(())
    }
}

fn build_definitions() -> Vec<ActionIntentDefinition> {
    vec![
        definition(
            "create-workspace",
            "Create Workspace",
            "Create a new workspace container.",
            ActionIntentCategory::Workspace,
            Capability::workspace_write(),
            "CreateWorkspace",
            TargetRequirement::None,
            &[],
        ),
        definition(
            "get-workspace",
            "Get Workspace",
            "Load a workspace by identifier.",
            ActionIntentCategory::Workspace,
            Capability::workspace_read(),
            "GetWorkspace",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "get-workspace-snapshot",
            "Get Workspace Snapshot",
            "Build a derived read model of workspace state.",
            ActionIntentCategory::Workspace,
            Capability::workspace_read(),
            "GetWorkspaceSnapshot",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "create-zone",
            "Create Zone",
            "Create a zone within a workspace.",
            ActionIntentCategory::Zone,
            Capability::zone_write(),
            "CreateZone",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "delete-zone",
            "Delete Zone",
            "Remove a zone from a workspace.",
            ActionIntentCategory::Zone,
            Capability::zone_write(),
            "DeleteZone",
            TargetRequirement::Required,
            &[ResourceKind::Zone],
        ),
        definition(
            "get-zone",
            "Get Zone",
            "Load a zone by identifier.",
            ActionIntentCategory::Zone,
            Capability::zone_read(),
            "GetZone",
            TargetRequirement::Required,
            &[ResourceKind::Zone],
        ),
        definition(
            "create-application",
            "Create Application",
            "Register an application reference in a workspace.",
            ActionIntentCategory::Application,
            Capability::application_write(),
            "CreateApplication",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "delete-application",
            "Delete Application",
            "Remove an application reference.",
            ActionIntentCategory::Application,
            Capability::application_write(),
            "DeleteApplication",
            TargetRequirement::Required,
            &[ResourceKind::Application],
        ),
        definition(
            "get-application",
            "Get Application",
            "Load an application reference.",
            ActionIntentCategory::Application,
            Capability::application_read(),
            "GetApplication",
            TargetRequirement::Required,
            &[ResourceKind::Application],
        ),
        definition(
            "create-widget",
            "Create Widget",
            "Register a widget reference in a workspace.",
            ActionIntentCategory::Widget,
            Capability::widget_write(),
            "CreateWidget",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "delete-widget",
            "Delete Widget",
            "Remove a widget reference.",
            ActionIntentCategory::Widget,
            Capability::widget_write(),
            "DeleteWidget",
            TargetRequirement::Required,
            &[ResourceKind::Widget],
        ),
        definition(
            "get-widget",
            "Get Widget",
            "Load a widget reference.",
            ActionIntentCategory::Widget,
            Capability::widget_read(),
            "GetWidget",
            TargetRequirement::Required,
            &[ResourceKind::Widget],
        ),
        definition(
            "create-layout",
            "Create Layout",
            "Create spatial layout state for a workspace.",
            ActionIntentCategory::Layout,
            Capability::layout_write(),
            "CreateLayout",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "update-layout",
            "Update Layout",
            "Update layout viewport and node placements.",
            ActionIntentCategory::Layout,
            Capability::layout_write(),
            "UpdateLayout",
            TargetRequirement::Required,
            &[ResourceKind::Workspace],
        ),
        definition(
            "delete-layout",
            "Delete Layout",
            "Remove layout state for a workspace.",
            ActionIntentCategory::Layout,
            Capability::layout_write(),
            "DeleteLayout",
            TargetRequirement::Required,
            &[ResourceKind::Workspace],
        ),
        definition(
            "reset-layout",
            "Reset Layout",
            "Reset layout viewport and nodes to defaults.",
            ActionIntentCategory::Layout,
            Capability::layout_write(),
            "ResetLayout",
            TargetRequirement::Required,
            &[ResourceKind::Workspace],
        ),
        definition(
            "get-layout",
            "Get Layout",
            "Load layout state for a workspace.",
            ActionIntentCategory::Layout,
            Capability::layout_read(),
            "GetLayout",
            TargetRequirement::Optional,
            &[ResourceKind::Workspace],
        ),
        definition(
            "get-layout-snapshot",
            "Get Layout Snapshot",
            "Capture a point-in-time layout snapshot.",
            ActionIntentCategory::Layout,
            Capability::layout_read(),
            "GetLayoutSnapshot",
            TargetRequirement::Required,
            &[ResourceKind::Workspace],
        ),
        definition(
            "update-settings",
            "Update Settings",
            "Update workspace configuration settings.",
            ActionIntentCategory::Settings,
            Capability::settings_write(),
            "UpdateSettings",
            TargetRequirement::None,
            &[],
        ),
        definition(
            "get-audit-history",
            "Get Audit History",
            "Query recent audit records.",
            ActionIntentCategory::Audit,
            Capability::audit_read(),
            "GetAuditHistory",
            TargetRequirement::None,
            &[],
        ),
    ]
}

fn definition(
    id: &str,
    name: &str,
    description: &str,
    category: ActionIntentCategory,
    capability_required: Capability,
    command_name: &'static str,
    target_requirement: TargetRequirement,
    allowed_target_kinds: &'static [ResourceKind],
) -> ActionIntentDefinition {
    ActionIntentDefinition {
        id: ActionIntentId::new(id).expect("action intent id is valid"),
        name: name.into(),
        description: description.into(),
        category,
        capability_required,
        command_name,
        target_requirement,
        allowed_target_kinds,
    }
}

static DEFINITIONS: LazyLock<Vec<ActionIntentDefinition>> = LazyLock::new(build_definitions);

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
    fn registry_has_unique_ids() {
        ActionIntentRegistry::validate_registry().unwrap();
    }

    #[test]
    fn lookup_returns_create_workspace_definition() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        assert_eq!(definition.command_name, "CreateWorkspace");
        assert!(definition.matches_capability(&Capability::workspace_write()));
    }

    #[test]
    fn validates_request_without_target() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let request = ActionIntentRequest::new(id);
        assert!(request.validate(&definition).is_ok());
    }

    #[test]
    fn rejects_missing_required_target() {
        let id = ActionIntentId::new("delete-zone").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let request = ActionIntentRequest::new(id);
        assert_eq!(request.validate(&definition), Err(ActionIntentError::MissingTarget));
    }

    #[test]
    fn rejects_unexpected_target() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let request = ActionIntentRequest::new(id).with_target(workspace_ref("ws-1"));
        assert_eq!(
            request.validate(&definition),
            Err(ActionIntentError::UnexpectedTarget)
        );
    }

    #[test]
    fn rejects_invalid_target_kind() {
        let id = ActionIntentId::new("delete-zone").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let request = ActionIntentRequest::new(id).with_target(workspace_ref("ws-1"));
        assert!(matches!(
            request.validate(&definition),
            Err(ActionIntentError::InvalidTargetKind {
                kind: ResourceKind::Workspace
            })
        ));
    }

    #[test]
    fn accepts_valid_zone_target() {
        let id = ActionIntentId::new("delete-zone").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let request = ActionIntentRequest::new(id).with_target(zone_ref("zone-1"));
        assert!(request.validate(&definition).is_ok());
    }

    #[test]
    fn capability_mismatch_detected() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        assert!(!definition.matches_capability(&Capability::zone_write()));
    }
}
