use thiserror::Error;

use workspace_database::DatabaseError;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Database error")]
    Database(#[from] DatabaseError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Workspace kernel is not ready")]
    NotReady,

    #[error("Invalid settings value: {0}")]
    InvalidSettings(String),
}

pub type Result<T> = std::result::Result<T, KernelError>;

/// Safe, user-facing error representation for IPC responses.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PublicError {
    pub code: String,
    pub message: String,
}

impl KernelError {
    pub fn to_public(&self) -> PublicError {
        match self {
            KernelError::Database(_) => PublicError {
                code: "database_error".into(),
                message: "A database operation failed.".into(),
            },
            KernelError::Config(_) => PublicError {
                code: "config_error".into(),
                message: "Configuration could not be processed.".into(),
            },
            KernelError::NotReady => PublicError {
                code: "not_ready".into(),
                message: "Workspace is still initializing.".into(),
            },
            KernelError::InvalidSettings(_) => PublicError {
                code: "invalid_settings".into(),
                message: "One or more settings values were invalid.".into(),
            },
        }
    }
}
