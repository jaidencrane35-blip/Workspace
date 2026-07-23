use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, Result};

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
    Settings,
    Audit,
    System,
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
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capability(mut self, capability: &Capability) -> Self {
        self.capabilities.insert(capability.id.clone());
        self
    }

    pub fn contains(&self, capability: &Capability) -> bool {
        self.capabilities.contains(&capability.id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CapabilityId> {
        self.capabilities.iter()
    }

    /// Standard capability identifiers for the local user placeholder (not enforced).
    pub fn local_user_standard() -> Self {
        Self::new()
            .with_capability(&Capability::workspace_read())
            .with_capability(&Capability::workspace_write())
            .with_capability(&Capability::settings_read())
            .with_capability(&Capability::settings_write())
            .with_capability(&Capability::audit_read())
            .with_capability(&Capability::audit_write())
    }

    /// Capabilities attributed to system lifecycle operations.
    pub fn system_standard() -> Self {
        Self::new()
            .with_capability(&Capability::system_startup())
            .with_capability(&Capability::system_shutdown())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn rejects_empty_capability_id() {
        assert_eq!(CapabilityId::new("").unwrap_err(), DomainError::InvalidId);
    }
}
