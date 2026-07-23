use thiserror::Error;

use workspace_database::DatabaseError;
use workspace_domain::{AiPlanningError, AiRequestError, DomainError, ResourceKind};

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

    #[error("Layout not found")]
    LayoutNotFound,

    #[error("Duplicate layout for workspace")]
    DuplicateLayout,

    #[error("Layout validation failed: {message}")]
    LayoutValidation { message: String },

    #[error("Projection validation failed: {message}")]
    ProjectionValidation { message: String },

    #[error("Action intent not found: {intent_id}")]
    ActionIntentNotFound { intent_id: String },

    #[error("Action intent validation failed: {message}")]
    ActionIntentValidation { message: String },

    #[error("Action intent capability mismatch for {intent_id}: expected {expected}, got {actual}")]
    ActionIntentCapabilityMismatch {
        intent_id: String,
        expected: String,
        actual: String,
    },

    #[error("Action intent {intent_id} maps to {expected_command}, not {actual_command}")]
    ActionIntentCommandMismatch {
        intent_id: String,
        expected_command: String,
        actual_command: String,
    },

    #[error("Capability discovery validation failed: {message}")]
    CapabilityDiscoveryValidation { message: String },

    #[error("Observation validation failed: {message}")]
    ObservationValidation { message: String },

    #[error("Analytics validation failed: {message}")]
    AnalyticsValidation { message: String },

    #[error("Context validation failed: {message}")]
    ContextValidation { message: String },

    #[error("Suggestion validation failed: {message}")]
    SuggestionValidation { message: String },

    #[error("Suggestion lifecycle validation failed: {message}")]
    SuggestionLifecycleValidation { message: String },

    #[error("Suggestion intent validation failed: {message}")]
    SuggestionIntentValidation { message: String },

    #[error("Intent execution validation failed: {message}")]
    IntentExecutionValidation { message: String },

    #[error("Execution outcome validation failed: {message}")]
    ExecutionOutcomeValidation { message: String },

    #[error("Execution context validation failed: {message}")]
    ExecutionContextValidation { message: String },

    #[error("Execution guard validation failed: {message}")]
    ExecutionGuardValidation { message: String },

    #[error("Duplicate execution blocked for {execution_request_id}")]
    DuplicateExecution { execution_request_id: String },

    #[error("Execution cancellation validation failed: {message}")]
    ExecutionCancellationValidation { message: String },

    #[error("Unknown execution request: {execution_request_id}")]
    UnknownExecutionRequest { execution_request_id: String },

    #[error("Cannot cancel completed execution: {execution_request_id}")]
    CannotCancelCompletedExecution { execution_request_id: String },

    #[error("Execution reconciliation validation failed: {message}")]
    ExecutionReconciliationValidation { message: String },

    #[error("Windows integration error: {message}")]
    WindowsIntegration { message: String },

    #[error("Invalid launch target: {message}")]
    InvalidLaunchTarget { message: String },

    #[error("Service '{0}' failed to start")]
    ServiceStartup(&'static str),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Approval required: {reason}")]
    ApprovalRequired {
        reason: String,
        approval_request_id: String,
    },

    #[error("Permission approval request not found")]
    PermissionApprovalNotFound,

    #[error("Permission approval validation failed: {message}")]
    PermissionApprovalValidation { message: String },

    #[error("AI request validation failed: {message}")]
    AiRequestValidation { message: String },

    #[error("AI planning validation failed: {message}")]
    AiPlanningValidation { message: String },

    #[error("Action catalog validation failed: {message}")]
    ActionCatalogValidation { message: String },

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

impl From<AiRequestError> for KernelError {
    fn from(error: AiRequestError) -> Self {
        match error {
            AiRequestError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiRequestValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AiPlanningError> for KernelError {
    fn from(error: AiPlanningError) -> Self {
        match error {
            AiPlanningError::Request(request) => KernelError::from(request),
            AiPlanningError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiPlanningValidation {
                message: other.to_string(),
            },
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
            KernelError::LayoutNotFound => PublicError {
                code: "layout_not_found".into(),
                message: "The requested layout was not found.".into(),
            },
            KernelError::DuplicateLayout => PublicError {
                code: "duplicate_layout".into(),
                message: "A layout already exists for this workspace.".into(),
            },
            KernelError::LayoutValidation { message } => PublicError {
                code: "layout_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ProjectionValidation { message } => PublicError {
                code: "projection_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ActionIntentNotFound { intent_id } => PublicError {
                code: "action_intent_not_found".into(),
                message: format!("The action intent '{intent_id}' is not supported."),
            },
            KernelError::ActionIntentValidation { message } => PublicError {
                code: "action_intent_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ActionIntentCapabilityMismatch {
                intent_id,
                expected,
                actual,
            } => PublicError {
                code: "action_intent_capability_mismatch".into(),
                message: format!(
                    "Action intent '{intent_id}' requires capability '{expected}', but command declared '{actual}'."
                ),
            },
            KernelError::ActionIntentCommandMismatch {
                intent_id,
                expected_command,
                actual_command,
            } => PublicError {
                code: "action_intent_command_mismatch".into(),
                message: format!(
                    "Action intent '{intent_id}' maps to '{expected_command}', not '{actual_command}'."
                ),
            },
            KernelError::CapabilityDiscoveryValidation { message } => PublicError {
                code: "capability_discovery_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ObservationValidation { message } => PublicError {
                code: "observation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AnalyticsValidation { message } => PublicError {
                code: "analytics_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ContextValidation { message } => PublicError {
                code: "context_validation_error".into(),
                message: message.clone(),
            },
            KernelError::SuggestionValidation { message } => PublicError {
                code: "suggestion_validation_error".into(),
                message: message.clone(),
            },
            KernelError::SuggestionLifecycleValidation { message } => PublicError {
                code: "suggestion_lifecycle_validation_error".into(),
                message: message.clone(),
            },
            KernelError::SuggestionIntentValidation { message } => PublicError {
                code: "suggestion_intent_validation_error".into(),
                message: message.clone(),
            },
            KernelError::IntentExecutionValidation { message } => PublicError {
                code: "intent_execution_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ExecutionOutcomeValidation { message } => PublicError {
                code: "execution_outcome_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ExecutionContextValidation { message } => PublicError {
                code: "execution_context_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ExecutionGuardValidation { message } => PublicError {
                code: "execution_guard_validation_error".into(),
                message: message.clone(),
            },
            KernelError::DuplicateExecution {
                execution_request_id,
            } => PublicError {
                code: "duplicate_execution".into(),
                message: format!(
                    "Execution request '{execution_request_id}' has already completed."
                ),
            },
            KernelError::ExecutionCancellationValidation { message } => PublicError {
                code: "execution_cancellation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::UnknownExecutionRequest {
                execution_request_id,
            } => PublicError {
                code: "unknown_execution_request".into(),
                message: format!("Unknown execution request '{execution_request_id}'."),
            },
            KernelError::CannotCancelCompletedExecution {
                execution_request_id,
            } => PublicError {
                code: "cannot_cancel_completed_execution".into(),
                message: format!(
                    "Execution request '{execution_request_id}' has already completed and cannot be cancelled."
                ),
            },
            KernelError::ExecutionReconciliationValidation { message } => PublicError {
                code: "execution_reconciliation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WindowsIntegration { message } => PublicError {
                code: "windows_integration_error".into(),
                message: message.clone(),
            },
            KernelError::InvalidLaunchTarget { message } => PublicError {
                code: "invalid_launch_target".into(),
                message: message.clone(),
            },
            KernelError::ServiceStartup(service) => PublicError {
                code: "service_startup_failed".into(),
                message: format!("Service '{service}' failed to start."),
            },
            KernelError::PermissionDenied(reason) => PublicError {
                code: "permission_denied".into(),
                message: if reason.trim().is_empty() {
                    "This action was not permitted.".into()
                } else {
                    format!("This action was not permitted: {reason}")
                },
            },
            KernelError::ApprovalRequired {
                reason,
                approval_request_id,
            } => PublicError {
                code: "approval_required".into(),
                message: format!(
                    "This action requires approval. {reason} (request_id={approval_request_id})"
                ),
            },
            KernelError::PermissionApprovalNotFound => PublicError {
                code: "permission_approval_not_found".into(),
                message: "Permission approval request was not found.".into(),
            },
            KernelError::PermissionApprovalValidation { message } => PublicError {
                code: "permission_approval_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiRequestValidation { message } => PublicError {
                code: "ai_request_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiPlanningValidation { message } => PublicError {
                code: "ai_planning_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ActionCatalogValidation { message } => PublicError {
                code: "action_catalog_validation_error".into(),
                message: message.clone(),
            },
            KernelError::InitializationFailed => PublicError {
                code: "initialization_failed".into(),
                message: "Workspace failed to initialize.".into(),
            },
        }
    }
}
