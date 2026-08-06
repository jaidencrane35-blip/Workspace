use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, Result};
use crate::actor::ActorType;

/// Strongly typed capability identifier (e.g. `workspace.read`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityId(String);

/// Scope a capability applies to (structural hint only — not enforced in Sprint 09).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityScope {
    Global,
    Workspace,
    Zone,
    Application,
    Widget,
    Layout,
    Settings,
    Audit,
    System,
    Memory,
    Personalization,
    WorkContext,
}

/// A named capability required or granted for an operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capability {
    pub id: CapabilityId,
    pub scope: CapabilityScope,
}

/// Collection of capabilities (identifiers only — not evaluated in Sprint 09).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CapabilitySet {
    capabilities: BTreeSet<CapabilityId>,
}

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CapabilityId {
    type Err = DomainError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Capability {
    pub fn new(id: impl Into<String>, scope: CapabilityScope) -> Result<Self> {
        Ok(Self {
            id: CapabilityId::new(id)?,
            scope,
        })
    }

    pub fn workspace_read() -> Self {
        Self {
            id: CapabilityId::new("workspace.read").expect("workspace.read is valid"),
            scope: CapabilityScope::Workspace,
        }
    }

    pub fn workspace_write() -> Self {
        Self {
            id: CapabilityId::new("workspace.write").expect("workspace.write is valid"),
            scope: CapabilityScope::Workspace,
        }
    }

    pub fn settings_read() -> Self {
        Self {
            id: CapabilityId::new("settings.read").expect("settings.read is valid"),
            scope: CapabilityScope::Settings,
        }
    }

    pub fn settings_write() -> Self {
        Self {
            id: CapabilityId::new("settings.write").expect("settings.write is valid"),
            scope: CapabilityScope::Settings,
        }
    }

    pub fn audit_read() -> Self {
        Self {
            id: CapabilityId::new("audit.read").expect("audit.read is valid"),
            scope: CapabilityScope::Audit,
        }
    }

    pub fn audit_write() -> Self {
        Self {
            id: CapabilityId::new("audit.write").expect("audit.write is valid"),
            scope: CapabilityScope::Audit,
        }
    }

    pub fn system_startup() -> Self {
        Self {
            id: CapabilityId::new("system.startup").expect("system.startup is valid"),
            scope: CapabilityScope::System,
        }
    }

    pub fn system_shutdown() -> Self {
        Self {
            id: CapabilityId::new("system.shutdown").expect("system.shutdown is valid"),
            scope: CapabilityScope::System,
        }
    }

    pub fn zone_read() -> Self {
        Self {
            id: CapabilityId::new("zone.read").expect("zone.read is valid"),
            scope: CapabilityScope::Zone,
        }
    }

    pub fn zone_write() -> Self {
        Self {
            id: CapabilityId::new("zone.write").expect("zone.write is valid"),
            scope: CapabilityScope::Zone,
        }
    }

    pub fn application_read() -> Self {
        Self {
            id: CapabilityId::new("application.read").expect("application.read is valid"),
            scope: CapabilityScope::Application,
        }
    }

    pub fn application_write() -> Self {
        Self {
            id: CapabilityId::new("application.write").expect("application.write is valid"),
            scope: CapabilityScope::Application,
        }
    }

    pub fn application_launch() -> Self {
        Self {
            id: CapabilityId::new("application.launch").expect("application.launch is valid"),
            scope: CapabilityScope::Application,
        }
    }

    pub fn widget_read() -> Self {
        Self {
            id: CapabilityId::new("widget.read").expect("widget.read is valid"),
            scope: CapabilityScope::Widget,
        }
    }

    pub fn widget_write() -> Self {
        Self {
            id: CapabilityId::new("widget.write").expect("widget.write is valid"),
            scope: CapabilityScope::Widget,
        }
    }

    pub fn layout_read() -> Self {
        Self {
            id: CapabilityId::new("layout.read").expect("layout.read is valid"),
            scope: CapabilityScope::Layout,
        }
    }

    pub fn layout_write() -> Self {
        Self {
            id: CapabilityId::new("layout.write").expect("layout.write is valid"),
            scope: CapabilityScope::Layout,
        }
    }

    pub fn memory_read() -> Self {
        Self {
            id: CapabilityId::new("memory.read").expect("memory.read is valid"),
            scope: CapabilityScope::Memory,
        }
    }

    pub fn memory_write() -> Self {
        Self {
            id: CapabilityId::new("memory.write").expect("memory.write is valid"),
            scope: CapabilityScope::Memory,
        }
    }

    pub fn personalization_read() -> Self {
        Self {
            id: CapabilityId::new("personalization.read")
                .expect("personalization.read is valid"),
            scope: CapabilityScope::Personalization,
        }
    }

    pub fn personalization_write() -> Self {
        Self {
            id: CapabilityId::new("personalization.write")
                .expect("personalization.write is valid"),
            scope: CapabilityScope::Personalization,
        }
    }

    pub fn work_context_read() -> Self {
        Self {
            id: CapabilityId::new("work_context.read").expect("work_context.read is valid"),
            scope: CapabilityScope::WorkContext,
        }
    }

    pub fn work_context_write() -> Self {
        Self {
            id: CapabilityId::new("work_context.write").expect("work_context.write is valid"),
            scope: CapabilityScope::WorkContext,
        }
    }

    pub fn desktop_read() -> Self {
        Self {
            id: CapabilityId::new("desktop.read").expect("desktop.read is valid"),
            scope: CapabilityScope::System,
        }
    }

    pub fn action_plan_resolve() -> Self {
        Self {
            id: CapabilityId::new("action.plan.resolve").expect("action.plan.resolve is valid"),
            scope: CapabilityScope::System,
        }
    }

    pub fn action_window_place() -> Self {
        Self {
            id: CapabilityId::new("action.window.place").expect("action.window.place is valid"),
            scope: CapabilityScope::System,
        }
    }

    pub fn action_window_focus() -> Self {
        Self {
            id: CapabilityId::new("action.window.focus").expect("action.window.focus is valid"),
            scope: CapabilityScope::System,
        }
    }

    /// Read system clipboard text (sensitive — audit length/format only).
    pub fn clipboard_read() -> Self {
        Self {
            id: CapabilityId::new("clipboard.read").expect("clipboard.read is valid"),
            scope: CapabilityScope::System,
        }
    }

    /// Write system clipboard text.
    pub fn clipboard_write() -> Self {
        Self {
            id: CapabilityId::new("clipboard.write").expect("clipboard.write is valid"),
            scope: CapabilityScope::System,
        }
    }

    /// Focus an application window (Application Provider).
    pub fn application_focus() -> Self {
        Self {
            id: CapabilityId::new("application.focus").expect("application.focus is valid"),
            scope: CapabilityScope::Application,
        }
    }

    /// Close / quit an application window (Application Provider — graceful WM_CLOSE).
    pub fn application_close() -> Self {
        Self {
            id: CapabilityId::new("application.close").expect("application.close is valid"),
            scope: CapabilityScope::Application,
        }
    }

    /// Minimize an application window.
    pub fn application_minimize() -> Self {
        Self {
            id: CapabilityId::new("application.minimize").expect("application.minimize is valid"),
            scope: CapabilityScope::Application,
        }
    }

    /// Restore a minimized application window.
    pub fn application_restore() -> Self {
        Self {
            id: CapabilityId::new("application.restore").expect("application.restore is valid"),
            scope: CapabilityScope::Application,
        }
    }
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capability(mut self, capability: &Capability) -> Self {
        self.capabilities.insert(capability.id.clone());
        self
    }

    pub fn with_capability_id(mut self, id: CapabilityId) -> Self {
        self.capabilities.insert(id);
        self
    }

    pub fn contains(&self, capability: &Capability) -> bool {
        self.capabilities.contains(&capability.id)
    }

    pub fn contains_id(&self, id: &CapabilityId) -> bool {
        self.capabilities.contains(id)
    }

    pub fn merge(mut self, other: &CapabilitySet) -> Self {
        for id in other.iter() {
            self.capabilities.insert(id.clone());
        }
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &CapabilityId> {
        self.capabilities.iter()
    }

    /// Standard capability identifiers for the local user placeholder (not enforced).
    pub fn local_user_standard() -> Self {
        Self::new()
            .with_capability(&Capability::workspace_read())
            .with_capability(&Capability::workspace_write())
            .with_capability(&Capability::zone_read())
            .with_capability(&Capability::zone_write())
            .with_capability(&Capability::application_read())
            .with_capability(&Capability::application_write())
            .with_capability(&Capability::application_launch())
            .with_capability(&Capability::widget_read())
            .with_capability(&Capability::widget_write())
            .with_capability(&Capability::layout_read())
            .with_capability(&Capability::layout_write())
            .with_capability(&Capability::settings_read())
            .with_capability(&Capability::settings_write())
            .with_capability(&Capability::audit_read())
            .with_capability(&Capability::audit_write())
            .with_capability(&Capability::memory_read())
            .with_capability(&Capability::memory_write())
            .with_capability(&Capability::personalization_read())
            .with_capability(&Capability::personalization_write())
            .with_capability(&Capability::work_context_read())
            .with_capability(&Capability::work_context_write())
            .with_capability(&Capability::desktop_read())
            .with_capability(&Capability::action_plan_resolve())
            .with_capability(&Capability::action_window_place())
            .with_capability(&Capability::action_window_focus())
            .with_capability(&Capability::clipboard_read())
            .with_capability(&Capability::clipboard_write())
            .with_capability(&Capability::application_focus())
            .with_capability(&Capability::application_close())
            .with_capability(&Capability::application_minimize())
            .with_capability(&Capability::application_restore())
    }

    /// Capabilities attributed to system lifecycle operations.
    pub fn system_standard() -> Self {
        Self::new()
            .with_capability(&Capability::system_startup())
            .with_capability(&Capability::system_shutdown())
    }

    /// Nominal capability identifiers attributed to an actor type.
    ///
    /// Non-human actors (AI, automation, plugin, remote) start with an empty
    /// set — Permission Gateway default-denies until capabilities are granted.
    pub fn for_actor_type(actor_type: ActorType) -> Self {
        match actor_type {
            ActorType::System => Self::system_standard(),
            ActorType::LocalUser => Self::local_user_standard(),
            ActorType::AIAssistant
            | ActorType::Automation
            | ActorType::Plugin
            | ActorType::RemoteSession => Self::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;

    #[test]
    fn capability_serializes_as_object() {
        let capability = Capability::workspace_write();
        let json = serde_json::to_string(&capability).unwrap();
        assert!(json.contains("workspace.write"));
        let restored: Capability = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, capability);
    }

    #[test]
    fn capability_set_tracks_membership() {
        let set = CapabilitySet::local_user_standard();
        assert!(set.contains(&Capability::audit_read()));
        assert!(!set.contains(&Capability::system_shutdown()));
    }

    #[test]
    fn layout_capabilities_in_local_user_set() {
        let set = CapabilitySet::local_user_standard();
        assert!(set.contains(&Capability::layout_read()));
        assert!(set.contains(&Capability::layout_write()));
    }

    #[test]
    fn layout_write_capability_has_layout_scope() {
        assert_eq!(Capability::layout_write().scope, CapabilityScope::Layout);
    }

    #[test]
    fn for_actor_type_matches_system_and_local_user_sets() {
        assert!(CapabilitySet::for_actor_type(ActorType::System)
            .contains(&Capability::system_shutdown()));
        assert!(CapabilitySet::for_actor_type(ActorType::LocalUser)
            .contains(&Capability::workspace_read()));
    }

    #[test]
    fn non_human_actors_have_empty_nominal_capabilities() {
        assert!(!CapabilitySet::for_actor_type(ActorType::AIAssistant)
            .contains(&Capability::workspace_read()));
        assert!(!CapabilitySet::for_actor_type(ActorType::Automation)
            .contains(&Capability::workspace_write()));
        assert!(!CapabilitySet::for_actor_type(ActorType::Plugin)
            .contains(&Capability::settings_write()));
        assert!(!CapabilitySet::for_actor_type(ActorType::RemoteSession)
            .contains(&Capability::audit_read()));
    }
}
