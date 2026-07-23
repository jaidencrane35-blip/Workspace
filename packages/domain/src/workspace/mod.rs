use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, Result};

/// A user workspace container (domain entity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

/// A zone within a workspace (position metadata placeholder only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub position_metadata: Option<String>,
}

/// Reference to an application associated with a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationReference {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub identifier: Option<String>,
}

/// Reference to a widget associated with a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetReference {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub widget_type: Option<String>,
}

impl Workspace {
    pub fn validate_name(name: &str) -> Result<()> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::EmptyName);
        }
        if trimmed.len() > 120 {
            return Err(DomainError::NameTooLong { max: 120 });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            id: "ws-1".into(),
            name: "Default".into(),
            created_at: "2026-07-23T00:00:00Z".into(),
            updated_at: "2026-07-23T00:00:00Z".into(),
        };
        assert_eq!(workspace.name, "Default");
    }
}
