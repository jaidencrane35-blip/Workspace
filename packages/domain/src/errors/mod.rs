use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Workspace name cannot be empty")]
    EmptyName,

    #[error("Workspace name exceeds maximum length of {max}")]
    NameTooLong { max: usize },

    #[error("Invalid workspace identifier")]
    InvalidId,
}

pub type Result<T> = std::result::Result<T, DomainError>;
