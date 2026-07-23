use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, Result};

/// Classification of an addressable resource (DEC-016).
///
/// Extensible: new resource kinds are added here and gain a uniform address
/// without changing unrelated commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    Workspace,
    Zone,
    Application,
    Widget,
}

impl ResourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceKind::Workspace => "workspace",
            ResourceKind::Zone => "zone",
            ResourceKind::Application => "application",
            ResourceKind::Widget => "widget",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Generic, globally-unique resource identifier (DEC-016).
///
/// Typed entity IDs (`WorkspaceId`, `ZoneId`, …) remain for entity-internal
/// type safety; `ResourceId` is the cross-cutting identifier used inside a
/// `ResourceRef`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceId(String);

impl ResourceId {
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

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceId {
    type Err = DomainError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Canonical resource address = kind + id (DEC-016).
///
/// Serializes over IPC as `{ "kind": ..., "id": ... }`; the `kind:id` string
/// form (via `Display`) is the canonical representation for audit and logs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceRef {
    pub kind: ResourceKind,
    pub id: ResourceId,
}

impl ResourceRef {
    pub fn new(kind: ResourceKind, id: ResourceId) -> Self {
        Self { kind, id }
    }

    /// Canonical `kind:id` string used for audit records and logs.
    pub fn canonical(&self) -> String {
        format!("{}:{}", self.kind.as_str(), self.id.as_str())
    }
}

impl fmt::Display for ResourceRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canonical())
    }
}

/// Bridge from a typed domain entity to its canonical `ResourceRef` (DEC-016).
pub trait Addressable {
    fn resource_kind(&self) -> ResourceKind;
    fn resource_ref(&self) -> ResourceRef;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_ref_has_canonical_string_form() {
        let reference = ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new("abc-123").unwrap(),
        );
        assert_eq!(reference.canonical(), "workspace:abc-123");
        assert_eq!(reference.to_string(), "workspace:abc-123");
    }

    #[test]
    fn resource_ref_serializes_as_kind_and_id_object() {
        let reference = ResourceRef::new(
            ResourceKind::Zone,
            ResourceId::new("zone-1").unwrap(),
        );
        let json = serde_json::to_string(&reference).unwrap();
        assert!(json.contains("\"kind\":\"zone\""));
        assert!(json.contains("\"id\":\"zone-1\""));

        let restored: ResourceRef = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, reference);
    }

    #[test]
    fn rejects_empty_resource_id() {
        assert_eq!(ResourceId::new("  ").unwrap_err(), DomainError::InvalidId);
    }
}
