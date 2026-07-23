use thiserror::Error;

use workspace_database::DatabaseError;
use workspace_domain::{DomainError, ResourceKind};

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Database error")]
    Database(#[from] DatabaseError),

    #[error("Domain error: {0}")]
    Domain(DomainError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Workspace kernel is not ready")]
    NotReady,

    #[error("Invalid settings value: {0}")]
    InvalidSettings(String),

    #[error("Workspace not found")]
    WorkspaceNotFound,

    #[error("Zone not found")]
    ZoneNotFound,

    #[error("Application not found")]
    ApplicationNotFound,

    #[error("Widget not found")]
    WidgetNotFound,

    #[error("Resource not found: {kind}")]
    ResourceNotFound { kind: ResourceKind },

    #[error("Duplicate resource: {kind}")]
    DuplicateResource { kind: ResourceKind },

    #[error("Invalid parent reference: expected {expected_kind}")]
    InvalidParentReference { expected_kind: ResourceKind },

    #[error("Service '{0}' failed to start")]
    ServiceStartup(&'static str),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Workspace kernel initialization failed")]
    InitializationFailed,
}

pub type Result<T> = std::result::Result<T, KernelError>;

/// Safe, user-facing error representation for IPC responses.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PublicError {
    pub code: String,
    pub message: String,
}

impl From<DomainError> for KernelError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::WorkspaceNotFound => KernelError::WorkspaceNotFound,
            DomainError::ZoneNotFound => KernelError::ZoneNotFound,
            DomainError::ApplicationNotFound => KernelError::ApplicationNotFound,
            DomainError::WidgetNotFound => KernelError::WidgetNotFound,
            DomainError::DuplicateResource { kind } => KernelError::DuplicateResource { kind },
            DomainError::InvalidParentReference { expected_kind } => {
                KernelError::InvalidParentReference {
                    expected_kind,
                }
            }
            DomainError::ResourceNotFound { kind } => KernelError::ResourceNotFound { kind },
            other => KernelError::Domain(other),
        }
    }
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
            KernelError::Domain(_) => PublicError {
                code: "domain_error".into(),
                message: "The request contained invalid workspace data.".into(),
            },
            KernelError::WorkspaceNotFound => PublicError {
                code: "workspace_not_found".into(),
                message: "The requested workspace was not found.".into(),
            },
            KernelError::ZoneNotFound => PublicError {
                code: "zone_not_found".into(),
                message: "The requested zone was not found.".into(),
            },
            KernelError::ApplicationNotFound => PublicError {
                code: "application_not_found".into(),
                message: "The requested application was not found.".into(),
            },
            KernelError::WidgetNotFound => PublicError {
                code: "widget_not_found".into(),
                message: "The requested widget was not found.".into(),
            },
            KernelError::ResourceNotFound { kind } => PublicError {
                code: "resource_not_found".into(),
                message: format!("The requested {kind} resource was not found."),
            },
            KernelError::DuplicateResource { kind } => PublicError {
                code: "duplicate_resource".into(),
                message: format!("A {kind} resource with that identifier already exists."),
            },
            KernelError::InvalidParentReference { expected_kind } => PublicError {
                code: "invalid_parent_reference".into(),
                message: format!("Invalid parent reference; expected {expected_kind}."),
            },
            KernelError::ServiceStartup(service) => PublicError {
                code: "service_startup_failed".into(),
                message: format!("Service '{service}' failed to start."),
            },
            KernelError::PermissionDenied(_) => PublicError {
                code: "permission_denied".into(),
                message: "This action was not permitted.".into(),
            },
            KernelError::InitializationFailed => PublicError {
                code: "initialization_failed".into(),
                message: "Workspace failed to initialize.".into(),
            },
        }
    }
}
