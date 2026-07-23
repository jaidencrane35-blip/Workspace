use thiserror::Error;

use crate::resource::ResourceKind;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Name exceeds maximum length of {max}")]
    NameTooLong { max: usize },

    #[error("Invalid identifier")]
    InvalidId,

    #[error("Resource not found: {kind}")]
    ResourceNotFound { kind: ResourceKind },

    #[error("Duplicate resource: {kind}")]
    DuplicateResource { kind: ResourceKind },

    #[error("Invalid parent reference: expected {expected_kind}")]
    InvalidParentReference { expected_kind: ResourceKind },

    #[error("Workspace not found")]
    WorkspaceNotFound,

    #[error("Zone not found")]
    ZoneNotFound,

    #[error("Application not found")]
    ApplicationNotFound,

    #[error("Widget not found")]
    WidgetNotFound,
}

pub type Result<T> = std::result::Result<T, DomainError>;

const MAX_NAME_LEN: usize = 120;

/// Shared name validation for resource entities.
pub fn validate_resource_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(DomainError::EmptyName);
    }
    if trimmed.len() > MAX_NAME_LEN {
        return Err(DomainError::NameTooLong {
            max: MAX_NAME_LEN,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_resource_names() {
        assert!(validate_resource_name("Valid").is_ok());
        assert_eq!(
            validate_resource_name("  ").unwrap_err(),
            DomainError::EmptyName
        );
    }
}
