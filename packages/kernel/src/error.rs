use thiserror::Error;

use workspace_database::DatabaseError;
use workspace_domain::{
    AiAssistantError, AiEvaluationError, AiMemoryError, AiModelError, AiOrchestrationError,
    AiPersonalizationError, AiPlanningError, AiRequestError, AssistantSurfaceError,
    AutomationContractError, AutomationTriggerError, CognitiveModelError,
    ContextualUnderstandingError, CrossWorkspaceIntelligenceError, DecisionEngineError,
    DecisionQueueError, DecisionSupportError, DomainError, EvidenceCompletenessError,
    EvidenceConsistencyError, EvidenceCoverageError, EvidenceDependencyError,
    EvidenceFreshnessError, EvidenceNavigationError, EvidenceReliabilityError, EvidenceTraceError,
    HistoricalReconstructionError, InsightCoordinationError, IntelligenceHubError,
    KnowledgeIntegrationError, KnowledgeSynthesisError, PolicyGovernanceError, ResourceKind,
    SemanticQueryError, TaskGraphError, TemporalIntelligenceError, WorkspaceActivityError,
    WorkspaceAdaptationError, WorkspaceAttentionError, WorkspaceCognitiveAgentCastError,
    WorkspaceCognitiveAutonomyError, WorkspaceCognitiveGraphError,
    WorkspaceCognitiveOrchestrationError, WorkspaceCompositionError, WorkspaceContinuityError,
    WorkspaceEnvironmentError, WorkspaceEvolutionError, WorkspaceExperienceError,
    WorkspaceExplanationError, WorkspaceIntelligenceError, WorkspaceIntentError,
    WorkspaceInteractionError, WorkspaceLearningAdaptationError, WorkspaceMilestoneError,
    WorkspaceNavigationError, WorkspaceOperatingStateError, WorkspacePatternError,
    WorkspacePlanningError, WorkspaceProfileError, WorkspacePurposeError, WorkspaceReadinessError,
    WorkspaceReasoningMemoryError, WorkspaceRecommendationEngineError, WorkspaceSessionError,
    WorkspaceStateEnvelopeError, WorkspaceTransitionError, WorkspaceWorkContextError,
    WorkspaceWorkingStyleError,
};

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Database error")]
    Database(#[from] DatabaseError),

    /// Durable audit evidence could not be persisted at a governed boundary.
    #[error("Audit persistence failed at {stage}")]
    AuditPersistence {
        stage: &'static str,
        #[source]
        source: DatabaseError,
    },

    #[error("Domain error: {0}")]
    Domain(DomainError),

    #[error("Configuration error: {0}")]
    Config(String),

    /// Infrastructure or runtime failure that is not user-correctable configuration.
    /// Prefer this over [`KernelError::Config`] for lock poison and similar faults.
    #[error("Internal error: {message}")]
    Internal { message: String },

    /// Architectural integrity failure (release-safe invariant / boundary violation).
    /// Distinct from operational projection validation and subsystem validation.
    #[error("Integrity violation: {message}")]
    IntegrityViolation { message: String },

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

    #[error(
        "Action intent capability mismatch for {intent_id}: expected {expected}, got {actual}"
    )]
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

    #[error("Observation capture already in progress")]
    ObservationCaptureInProgress,

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

    #[error("Execution request is already in progress: {execution_request_id}")]
    ExecutionInProgress { execution_request_id: String },

    #[error("Execution reconciliation required: {execution_request_id}")]
    ExecutionReconciliationRequired { execution_request_id: String },

    #[error("Execution atomicity could not be confirmed: {message}")]
    ExecutionAtomicity { message: String },

    #[error("Execution lifecycle persistence failed at {stage}")]
    ExecutionLifecyclePersistence {
        stage: &'static str,
        #[source]
        source: DatabaseError,
    },

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

    #[error("AI evaluation validation failed: {message}")]
    AiEvaluationValidation { message: String },

    #[error("AI orchestration validation failed: {message}")]
    AiOrchestrationValidation { message: String },

    #[error("AI assistant validation failed: {message}")]
    AiAssistantValidation { message: String },

    #[error("AI memory validation failed: {message}")]
    AiMemoryValidation { message: String },

    #[error("AI model provider validation failed: {message}")]
    AiModelValidation { message: String },

    #[error("AI personalization validation failed: {message}")]
    AiPersonalizationValidation { message: String },

    #[error("Workspace intent validation failed: {message}")]
    WorkspaceIntentValidation { message: String },

    #[error("Cognitive model validation failed: {message}")]
    CognitiveModelValidation { message: String },

    #[error("Workspace planning validation failed: {message}")]
    WorkspacePlanningValidation { message: String },

    #[error("Workspace reasoning memory validation failed: {message}")]
    WorkspaceReasoningMemoryValidation { message: String },

    #[error("Workspace cognitive graph validation failed: {message}")]
    WorkspaceCognitiveGraphValidation { message: String },

    #[error("Workspace cognitive orchestration validation failed: {message}")]
    WorkspaceCognitiveOrchestrationValidation { message: String },

    #[error("Workspace learning adaptation validation failed: {message}")]
    WorkspaceLearningAdaptationValidation { message: String },

    #[error("Workspace cognitive agent cast validation failed: {message}")]
    WorkspaceCognitiveAgentCastValidation { message: String },

    #[error("Workspace cognitive autonomy validation failed: {message}")]
    WorkspaceCognitiveAutonomyValidation { message: String },

    #[error("Workspace state envelope validation failed: {message}")]
    WorkspaceStateEnvelopeValidation { message: String },

    #[error("Policy governance validation failed: {message}")]
    PolicyGovernanceValidation { message: String },

    #[error("Historical reconstruction validation failed: {message}")]
    HistoricalReconstructionValidation { message: String },

    #[error("Temporal intelligence validation failed: {message}")]
    TemporalIntelligenceValidation { message: String },

    #[error("Workspace explanation validation failed: {message}")]
    WorkspaceExplanationValidation { message: String },

    #[error("Contextual understanding validation failed: {message}")]
    ContextualUnderstandingValidation { message: String },

    #[error("Knowledge synthesis validation failed: {message}")]
    KnowledgeSynthesisValidation { message: String },

    #[error("Knowledge integration validation failed: {message}")]
    KnowledgeIntegrationValidation { message: String },

    #[error("Insight coordination validation failed: {message}")]
    InsightCoordinationValidation { message: String },

    #[error("Cross-workspace intelligence validation failed: {message}")]
    CrossWorkspaceIntelligenceValidation { message: String },

    #[error("Decision support validation failed: {message}")]
    DecisionSupportValidation { message: String },

    #[error("Intelligence hub validation failed: {message}")]
    IntelligenceHubValidation { message: String },

    #[error("Semantic query validation failed: {message}")]
    SemanticQueryValidation { message: String },

    #[error("Evidence navigation validation failed: {message}")]
    EvidenceNavigationValidation { message: String },

    #[error("Evidence trace validation failed: {message}")]
    EvidenceTraceValidation { message: String },

    #[error("Evidence coverage validation failed: {message}")]
    EvidenceCoverageValidation { message: String },

    #[error("Evidence consistency validation failed: {message}")]
    EvidenceConsistencyValidation { message: String },

    #[error("Evidence dependency validation failed: {message}")]
    EvidenceDependencyValidation { message: String },

    #[error("Evidence freshness validation failed: {message}")]
    EvidenceFreshnessValidation { message: String },

    #[error("Evidence completeness validation failed: {message}")]
    EvidenceCompletenessValidation { message: String },

    #[error("Evidence reliability validation failed: {message}")]
    EvidenceReliabilityValidation { message: String },

    #[error("Assistant surface validation failed: {message}")]
    AssistantSurfaceValidation { message: String },

    #[error("Workspace intelligence validation failed: {message}")]
    WorkspaceIntelligenceValidation { message: String },

    #[error("Automation contract validation failed: {message}")]
    AutomationContractValidation { message: String },

    #[error("Automation trigger validation failed: {message}")]
    AutomationTriggerValidation { message: String },

    #[error("Decision queue validation failed: {message}")]
    DecisionQueueValidation { message: String },

    #[error("Workspace activity validation failed: {message}")]
    WorkspaceActivityValidation { message: String },

    #[error("Workspace continuity validation failed: {message}")]
    WorkspaceContinuityValidation { message: String },

    #[error("Workspace attention validation failed: {message}")]
    WorkspaceAttentionValidation { message: String },

    #[error("Decision engine validation failed: {message}")]
    DecisionEngineValidation { message: String },

    #[error("Decision candidate not found")]
    DecisionEngineNotFound,

    #[error("Decision engine cannot execute or authorize")]
    DecisionEngineCannotExecute,

    #[error("Task graph validation failed: {message}")]
    TaskGraphValidation { message: String },

    #[error("Workspace environment validation failed: {message}")]
    WorkspaceEnvironmentValidation { message: String },

    #[error("Workspace composition validation failed: {message}")]
    WorkspaceCompositionValidation { message: String },

    #[error("Workspace purpose validation failed: {message}")]
    WorkspacePurposeValidation { message: String },

    #[error("Workspace evolution validation failed: {message}")]
    WorkspaceEvolutionValidation { message: String },

    #[error("Workspace recommendation engine validation failed: {message}")]
    WorkspaceRecommendationEngineValidation { message: String },

    #[error("Recommendation candidate not found")]
    RecommendationNotFound,

    #[error("Recommendation engine cannot execute or authorize")]
    RecommendationCannotExecute,

    #[error("Workspace operating state validation failed: {message}")]
    WorkspaceOperatingStateValidation { message: String },

    #[error("Workspace pattern validation failed: {message}")]
    WorkspacePatternValidation { message: String },

    #[error("Workspace adaptation validation failed: {message}")]
    WorkspaceAdaptationValidation { message: String },

    #[error("Workspace readiness validation failed: {message}")]
    WorkspaceReadinessValidation { message: String },

    #[error("Workspace session validation failed: {message}")]
    WorkspaceSessionValidation { message: String },

    #[error("Workspace experience validation failed: {message}")]
    WorkspaceExperienceValidation { message: String },

    #[error("Workspace work context validation failed: {message}")]
    WorkspaceWorkContextValidation { message: String },

    #[error("Workspace navigation validation failed: {message}")]
    WorkspaceNavigationValidation { message: String },

    #[error("Workspace milestone validation failed: {message}")]
    WorkspaceMilestoneValidation { message: String },

    #[error("Workspace working style validation failed: {message}")]
    WorkspaceWorkingStyleValidation { message: String },

    #[error("Workspace transition validation failed: {message}")]
    WorkspaceTransitionValidation { message: String },

    #[error("Workspace interaction validation failed: {message}")]
    WorkspaceInteractionValidation { message: String },

    #[error("Workspace profile validation failed: {message}")]
    WorkspaceProfileValidation { message: String },

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
                KernelError::InvalidParentReference { expected_kind }
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

impl From<AiEvaluationError> for KernelError {
    fn from(error: AiEvaluationError) -> Self {
        KernelError::AiEvaluationValidation {
            message: error.to_string(),
        }
    }
}

impl From<AiOrchestrationError> for KernelError {
    fn from(error: AiOrchestrationError) -> Self {
        match error {
            AiOrchestrationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiOrchestrationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AiAssistantError> for KernelError {
    fn from(error: AiAssistantError) -> Self {
        match error {
            AiAssistantError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiAssistantValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AiMemoryError> for KernelError {
    fn from(error: AiMemoryError) -> Self {
        match error {
            AiMemoryError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiMemoryValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AiModelError> for KernelError {
    fn from(error: AiModelError) -> Self {
        match error {
            AiModelError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiModelValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AiPersonalizationError> for KernelError {
    fn from(error: AiPersonalizationError) -> Self {
        match error {
            AiPersonalizationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AiPersonalizationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceIntentError> for KernelError {
    fn from(error: WorkspaceIntentError) -> Self {
        match error {
            WorkspaceIntentError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceIntentValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<CognitiveModelError> for KernelError {
    fn from(error: CognitiveModelError) -> Self {
        match error {
            CognitiveModelError::Domain(domain) => KernelError::from(domain),
            other => KernelError::CognitiveModelValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspacePlanningError> for KernelError {
    fn from(error: WorkspacePlanningError) -> Self {
        match error {
            WorkspacePlanningError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspacePlanningValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceReasoningMemoryError> for KernelError {
    fn from(error: WorkspaceReasoningMemoryError) -> Self {
        match error {
            WorkspaceReasoningMemoryError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceReasoningMemoryValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceCognitiveGraphError> for KernelError {
    fn from(error: WorkspaceCognitiveGraphError) -> Self {
        match error {
            WorkspaceCognitiveGraphError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceCognitiveGraphValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceCognitiveOrchestrationError> for KernelError {
    fn from(error: WorkspaceCognitiveOrchestrationError) -> Self {
        match error {
            WorkspaceCognitiveOrchestrationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceCognitiveOrchestrationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceLearningAdaptationError> for KernelError {
    fn from(error: WorkspaceLearningAdaptationError) -> Self {
        match error {
            WorkspaceLearningAdaptationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceLearningAdaptationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceCognitiveAgentCastError> for KernelError {
    fn from(error: WorkspaceCognitiveAgentCastError) -> Self {
        match error {
            WorkspaceCognitiveAgentCastError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceCognitiveAgentCastValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceCognitiveAutonomyError> for KernelError {
    fn from(error: WorkspaceCognitiveAutonomyError) -> Self {
        match error {
            WorkspaceCognitiveAutonomyError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceCognitiveAutonomyValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceStateEnvelopeError> for KernelError {
    fn from(error: WorkspaceStateEnvelopeError) -> Self {
        match error {
            WorkspaceStateEnvelopeError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceStateEnvelopeValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<PolicyGovernanceError> for KernelError {
    fn from(error: PolicyGovernanceError) -> Self {
        match error {
            PolicyGovernanceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::PolicyGovernanceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<HistoricalReconstructionError> for KernelError {
    fn from(error: HistoricalReconstructionError) -> Self {
        match error {
            HistoricalReconstructionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::HistoricalReconstructionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<TemporalIntelligenceError> for KernelError {
    fn from(error: TemporalIntelligenceError) -> Self {
        match error {
            TemporalIntelligenceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::TemporalIntelligenceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceExplanationError> for KernelError {
    fn from(error: WorkspaceExplanationError) -> Self {
        match error {
            WorkspaceExplanationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceExplanationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<ContextualUnderstandingError> for KernelError {
    fn from(error: ContextualUnderstandingError) -> Self {
        match error {
            ContextualUnderstandingError::Domain(domain) => KernelError::from(domain),
            other => KernelError::ContextualUnderstandingValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<KnowledgeSynthesisError> for KernelError {
    fn from(error: KnowledgeSynthesisError) -> Self {
        match error {
            KnowledgeSynthesisError::Domain(domain) => KernelError::from(domain),
            other => KernelError::KnowledgeSynthesisValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<KnowledgeIntegrationError> for KernelError {
    fn from(error: KnowledgeIntegrationError) -> Self {
        match error {
            KnowledgeIntegrationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::KnowledgeIntegrationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<InsightCoordinationError> for KernelError {
    fn from(error: InsightCoordinationError) -> Self {
        match error {
            InsightCoordinationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::InsightCoordinationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<CrossWorkspaceIntelligenceError> for KernelError {
    fn from(error: CrossWorkspaceIntelligenceError) -> Self {
        match error {
            CrossWorkspaceIntelligenceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::CrossWorkspaceIntelligenceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<DecisionSupportError> for KernelError {
    fn from(error: DecisionSupportError) -> Self {
        match error {
            DecisionSupportError::Domain(domain) => KernelError::from(domain),
            other => KernelError::DecisionSupportValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<IntelligenceHubError> for KernelError {
    fn from(error: IntelligenceHubError) -> Self {
        match error {
            IntelligenceHubError::Domain(domain) => KernelError::from(domain),
            other => KernelError::IntelligenceHubValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<SemanticQueryError> for KernelError {
    fn from(error: SemanticQueryError) -> Self {
        match error {
            SemanticQueryError::Domain(domain) => KernelError::from(domain),
            other => KernelError::SemanticQueryValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceNavigationError> for KernelError {
    fn from(error: EvidenceNavigationError) -> Self {
        match error {
            EvidenceNavigationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceNavigationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceTraceError> for KernelError {
    fn from(error: EvidenceTraceError) -> Self {
        match error {
            EvidenceTraceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceTraceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceCoverageError> for KernelError {
    fn from(error: EvidenceCoverageError) -> Self {
        match error {
            EvidenceCoverageError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceCoverageValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceConsistencyError> for KernelError {
    fn from(error: EvidenceConsistencyError) -> Self {
        match error {
            EvidenceConsistencyError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceConsistencyValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceDependencyError> for KernelError {
    fn from(error: EvidenceDependencyError) -> Self {
        match error {
            EvidenceDependencyError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceDependencyValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceFreshnessError> for KernelError {
    fn from(error: EvidenceFreshnessError) -> Self {
        match error {
            EvidenceFreshnessError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceFreshnessValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceCompletenessError> for KernelError {
    fn from(error: EvidenceCompletenessError) -> Self {
        match error {
            EvidenceCompletenessError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceCompletenessValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<EvidenceReliabilityError> for KernelError {
    fn from(error: EvidenceReliabilityError) -> Self {
        match error {
            EvidenceReliabilityError::Domain(domain) => KernelError::from(domain),
            other => KernelError::EvidenceReliabilityValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AssistantSurfaceError> for KernelError {
    fn from(error: AssistantSurfaceError) -> Self {
        match error {
            AssistantSurfaceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AssistantSurfaceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceIntelligenceError> for KernelError {
    fn from(error: WorkspaceIntelligenceError) -> Self {
        match error {
            WorkspaceIntelligenceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceIntelligenceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AutomationContractError> for KernelError {
    fn from(error: AutomationContractError) -> Self {
        match error {
            AutomationContractError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AutomationContractValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<AutomationTriggerError> for KernelError {
    fn from(error: AutomationTriggerError) -> Self {
        match error {
            AutomationTriggerError::Domain(domain) => KernelError::from(domain),
            other => KernelError::AutomationTriggerValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<DecisionQueueError> for KernelError {
    fn from(error: DecisionQueueError) -> Self {
        match error {
            DecisionQueueError::Domain(domain) => KernelError::from(domain),
            other => KernelError::DecisionQueueValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceActivityError> for KernelError {
    fn from(error: WorkspaceActivityError) -> Self {
        match error {
            WorkspaceActivityError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceActivityValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceContinuityError> for KernelError {
    fn from(error: WorkspaceContinuityError) -> Self {
        match error {
            WorkspaceContinuityError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceContinuityValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceAttentionError> for KernelError {
    fn from(error: WorkspaceAttentionError) -> Self {
        match error {
            WorkspaceAttentionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceAttentionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<DecisionEngineError> for KernelError {
    fn from(error: DecisionEngineError) -> Self {
        match error {
            DecisionEngineError::Domain(domain) => KernelError::from(domain),
            DecisionEngineError::NotFound => KernelError::DecisionEngineNotFound,
            DecisionEngineError::CannotExecute => KernelError::DecisionEngineCannotExecute,
            other => KernelError::DecisionEngineValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<TaskGraphError> for KernelError {
    fn from(error: TaskGraphError) -> Self {
        match error {
            TaskGraphError::Domain(domain) => KernelError::from(domain),
            other => KernelError::TaskGraphValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceEnvironmentError> for KernelError {
    fn from(error: WorkspaceEnvironmentError) -> Self {
        match error {
            WorkspaceEnvironmentError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceEnvironmentValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceCompositionError> for KernelError {
    fn from(error: WorkspaceCompositionError) -> Self {
        match error {
            WorkspaceCompositionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceCompositionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspacePurposeError> for KernelError {
    fn from(error: WorkspacePurposeError) -> Self {
        match error {
            WorkspacePurposeError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspacePurposeValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceEvolutionError> for KernelError {
    fn from(error: WorkspaceEvolutionError) -> Self {
        match error {
            WorkspaceEvolutionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceEvolutionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceRecommendationEngineError> for KernelError {
    fn from(error: WorkspaceRecommendationEngineError) -> Self {
        match error {
            WorkspaceRecommendationEngineError::Domain(domain) => KernelError::from(domain),
            WorkspaceRecommendationEngineError::NotFound => KernelError::RecommendationNotFound,
            WorkspaceRecommendationEngineError::CannotExecute
            | WorkspaceRecommendationEngineError::CannotBecomeHandoff
            | WorkspaceRecommendationEngineError::ConfirmationCannotCreateAuthority
            | WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority => {
                KernelError::RecommendationCannotExecute
            }
            other => KernelError::WorkspaceRecommendationEngineValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceOperatingStateError> for KernelError {
    fn from(error: WorkspaceOperatingStateError) -> Self {
        match error {
            WorkspaceOperatingStateError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceOperatingStateValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspacePatternError> for KernelError {
    fn from(error: WorkspacePatternError) -> Self {
        match error {
            WorkspacePatternError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspacePatternValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceAdaptationError> for KernelError {
    fn from(error: WorkspaceAdaptationError) -> Self {
        match error {
            WorkspaceAdaptationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceAdaptationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceReadinessError> for KernelError {
    fn from(error: WorkspaceReadinessError) -> Self {
        match error {
            WorkspaceReadinessError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceReadinessValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceSessionError> for KernelError {
    fn from(error: WorkspaceSessionError) -> Self {
        match error {
            WorkspaceSessionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceSessionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceExperienceError> for KernelError {
    fn from(error: WorkspaceExperienceError) -> Self {
        match error {
            WorkspaceExperienceError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceExperienceValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceWorkContextError> for KernelError {
    fn from(error: WorkspaceWorkContextError) -> Self {
        match error {
            WorkspaceWorkContextError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceWorkContextValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceNavigationError> for KernelError {
    fn from(error: WorkspaceNavigationError) -> Self {
        match error {
            WorkspaceNavigationError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceNavigationValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceMilestoneError> for KernelError {
    fn from(error: WorkspaceMilestoneError) -> Self {
        match error {
            WorkspaceMilestoneError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceMilestoneValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceWorkingStyleError> for KernelError {
    fn from(error: WorkspaceWorkingStyleError) -> Self {
        match error {
            WorkspaceWorkingStyleError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceWorkingStyleValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceTransitionError> for KernelError {
    fn from(error: WorkspaceTransitionError) -> Self {
        match error {
            WorkspaceTransitionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceTransitionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceInteractionError> for KernelError {
    fn from(error: WorkspaceInteractionError) -> Self {
        match error {
            WorkspaceInteractionError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceInteractionValidation {
                message: other.to_string(),
            },
        }
    }
}

impl From<WorkspaceProfileError> for KernelError {
    fn from(error: WorkspaceProfileError) -> Self {
        match error {
            WorkspaceProfileError::Domain(domain) => KernelError::from(domain),
            other => KernelError::WorkspaceProfileValidation {
                message: other.to_string(),
            },
        }
    }
}

impl KernelError {
    /// Infrastructure failure helper for poisoned synchronization primitives.
    pub fn lock_poisoned(resource: &str) -> Self {
        KernelError::Internal {
            message: format!("{resource} lock poisoned"),
        }
    }

    /// Architectural integrity failure helper (release-safe invariant boundaries).
    pub fn integrity_violation(message: impl Into<String>) -> Self {
        KernelError::IntegrityViolation {
            message: message.into(),
        }
    }

    /// Map repository lifecycle / immutable-artifact rejections to owning subsystem
    /// validation errors instead of opaque `database_error`.
    pub fn from_recommendation_persistence(error: DatabaseError) -> Self {
        match error {
            DatabaseError::InvalidTransition(message)
            | DatabaseError::ImmutableArtifact(message) => {
                KernelError::WorkspaceRecommendationEngineValidation { message }
            }
            other => KernelError::Database(other),
        }
    }

    pub fn from_decision_engine_persistence(error: DatabaseError) -> Self {
        match error {
            DatabaseError::InvalidTransition(message)
            | DatabaseError::ImmutableArtifact(message) => {
                KernelError::DecisionEngineValidation { message }
            }
            other => KernelError::Database(other),
        }
    }

    pub fn from_decision_queue_persistence(error: DatabaseError) -> Self {
        match error {
            DatabaseError::InvalidTransition(message)
            | DatabaseError::ImmutableArtifact(message) => {
                KernelError::DecisionQueueValidation { message }
            }
            other => KernelError::Database(other),
        }
    }

    pub fn from_task_graph_persistence(error: DatabaseError) -> Self {
        match error {
            DatabaseError::InvalidTransition(message)
            | DatabaseError::ImmutableArtifact(message) => {
                KernelError::TaskGraphValidation { message }
            }
            other => KernelError::Database(other),
        }
    }

    pub fn to_public(&self) -> PublicError {
        match self {
            KernelError::Database(_) => PublicError {
                code: "database_error".into(),
                message: "A database operation failed.".into(),
            },
            KernelError::AuditPersistence { .. } => PublicError {
                code: "audit_persistence_error".into(),
                message: "Required audit evidence could not be persisted.".into(),
            },
            KernelError::Config(_) => PublicError {
                code: "config_error".into(),
                message: "Configuration could not be processed.".into(),
            },
            KernelError::Internal { .. } => PublicError {
                code: "internal_error".into(),
                message: "Workspace core is temporarily unavailable.".into(),
            },
            KernelError::IntegrityViolation { message } => PublicError {
                code: "integrity_violation".into(),
                message: message.clone(),
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
            KernelError::ObservationCaptureInProgress => PublicError {
                code: "observation_capture_in_progress".into(),
                message: "An observation capture is already in progress.".into(),
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
            KernelError::ExecutionInProgress {
                execution_request_id,
            } => PublicError {
                code: "execution_in_progress".into(),
                message: format!("Execution request '{execution_request_id}' is already in progress."),
            },
            KernelError::ExecutionReconciliationRequired {
                execution_request_id,
            } => PublicError {
                code: "execution_reconciliation_required".into(),
                message: format!(
                    "Execution request '{execution_request_id}' requires reconciliation."
                ),
            },
            KernelError::ExecutionAtomicity { .. } => PublicError {
                code: "execution_atomicity_error".into(),
                message: "Execution atomicity could not be confirmed.".into(),
            },
            KernelError::ExecutionLifecyclePersistence { .. } => PublicError {
                code: "execution_lifecycle_persistence_error".into(),
                message: "Execution lifecycle state could not be persisted.".into(),
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
            KernelError::AiEvaluationValidation { message } => PublicError {
                code: "ai_evaluation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiOrchestrationValidation { message } => PublicError {
                code: "ai_orchestration_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiAssistantValidation { message } => PublicError {
                code: "ai_assistant_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiMemoryValidation { message } => PublicError {
                code: "ai_memory_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiModelValidation { message } => PublicError {
                code: "ai_model_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AiPersonalizationValidation { message } => PublicError {
                code: "ai_personalization_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceIntentValidation { message } => PublicError {
                code: "workspace_intent_validation_error".into(),
                message: message.clone(),
            },
            KernelError::CognitiveModelValidation { message } => PublicError {
                code: "cognitive_model_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspacePlanningValidation { message } => PublicError {
                code: "workspace_planning_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceReasoningMemoryValidation { message } => PublicError {
                code: "workspace_reasoning_memory_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceCognitiveGraphValidation { message } => PublicError {
                code: "workspace_cognitive_graph_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceCognitiveOrchestrationValidation { message } => PublicError {
                code: "workspace_cognitive_orchestration_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceLearningAdaptationValidation { message } => PublicError {
                code: "workspace_learning_adaptation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceCognitiveAgentCastValidation { message } => PublicError {
                code: "workspace_cognitive_agent_cast_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceCognitiveAutonomyValidation { message } => PublicError {
                code: "workspace_cognitive_autonomy_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceStateEnvelopeValidation { message } => PublicError {
                code: "workspace_state_envelope_validation_error".into(),
                message: message.clone(),
            },
            KernelError::PolicyGovernanceValidation { message } => PublicError {
                code: "policy_governance_validation_error".into(),
                message: message.clone(),
            },
            KernelError::HistoricalReconstructionValidation { message } => PublicError {
                code: "historical_reconstruction_validation_error".into(),
                message: message.clone(),
            },
            KernelError::TemporalIntelligenceValidation { message } => PublicError {
                code: "temporal_intelligence_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceExplanationValidation { message } => PublicError {
                code: "workspace_explanation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::ContextualUnderstandingValidation { message } => PublicError {
                code: "contextual_understanding_validation_error".into(),
                message: message.clone(),
            },
            KernelError::KnowledgeSynthesisValidation { message } => PublicError {
                code: "knowledge_synthesis_validation_error".into(),
                message: message.clone(),
            },
            KernelError::KnowledgeIntegrationValidation { message } => PublicError {
                code: "knowledge_integration_validation_error".into(),
                message: message.clone(),
            },
            KernelError::InsightCoordinationValidation { message } => PublicError {
                code: "insight_coordination_validation_error".into(),
                message: message.clone(),
            },
            KernelError::CrossWorkspaceIntelligenceValidation { message } => PublicError {
                code: "cross_workspace_intelligence_validation_error".into(),
                message: message.clone(),
            },
            KernelError::DecisionSupportValidation { message } => PublicError {
                code: "decision_support_validation_error".into(),
                message: message.clone(),
            },
            KernelError::IntelligenceHubValidation { message } => PublicError {
                code: "intelligence_hub_validation_error".into(),
                message: message.clone(),
            },
            KernelError::SemanticQueryValidation { message } => PublicError {
                code: "semantic_query_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceNavigationValidation { message } => PublicError {
                code: "evidence_navigation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceTraceValidation { message } => PublicError {
                code: "evidence_trace_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceCoverageValidation { message } => PublicError {
                code: "evidence_coverage_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceConsistencyValidation { message } => PublicError {
                code: "evidence_consistency_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceDependencyValidation { message } => PublicError {
                code: "evidence_dependency_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceFreshnessValidation { message } => PublicError {
                code: "evidence_freshness_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceCompletenessValidation { message } => PublicError {
                code: "evidence_completeness_validation_error".into(),
                message: message.clone(),
            },
            KernelError::EvidenceReliabilityValidation { message } => PublicError {
                code: "evidence_reliability_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AssistantSurfaceValidation { message } => PublicError {
                code: "assistant_surface_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceIntelligenceValidation { message } => PublicError {
                code: "workspace_intelligence_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AutomationContractValidation { message } => PublicError {
                code: "automation_contract_validation_error".into(),
                message: message.clone(),
            },
            KernelError::AutomationTriggerValidation { message } => PublicError {
                code: "automation_trigger_validation_error".into(),
                message: message.clone(),
            },
            KernelError::DecisionQueueValidation { message } => PublicError {
                code: "decision_queue_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceActivityValidation { message } => PublicError {
                code: "workspace_activity_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceContinuityValidation { message } => PublicError {
                code: "workspace_continuity_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceAttentionValidation { message } => PublicError {
                code: "workspace_attention_validation_error".into(),
                message: message.clone(),
            },
            KernelError::DecisionEngineValidation { message } => PublicError {
                code: "decision_engine_validation_error".into(),
                message: message.clone(),
            },
            KernelError::DecisionEngineNotFound => PublicError {
                code: "decision_engine_not_found".into(),
                message: "The requested decision candidate was not found.".into(),
            },
            KernelError::DecisionEngineCannotExecute => PublicError {
                code: "decision_engine_cannot_execute".into(),
                message: "The decision engine cannot execute or authorize actions.".into(),
            },
            KernelError::TaskGraphValidation { message } => PublicError {
                code: "task_graph_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceEnvironmentValidation { message } => PublicError {
                code: "workspace_environment_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceCompositionValidation { message } => PublicError {
                code: "workspace_composition_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspacePurposeValidation { message } => PublicError {
                code: "workspace_purpose_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceEvolutionValidation { message } => PublicError {
                code: "workspace_evolution_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceRecommendationEngineValidation { message } => PublicError {
                code: "workspace_recommendation_engine_validation_error".into(),
                message: message.clone(),
            },
            KernelError::RecommendationNotFound => PublicError {
                code: "recommendation_not_found".into(),
                message: "The requested recommendation candidate was not found.".into(),
            },
            KernelError::RecommendationCannotExecute => PublicError {
                code: "recommendation_cannot_execute".into(),
                message: "The recommendation engine cannot execute or authorize actions.".into(),
            },
            KernelError::WorkspaceOperatingStateValidation { message } => PublicError {
                code: "workspace_operating_state_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspacePatternValidation { message } => PublicError {
                code: "workspace_pattern_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceAdaptationValidation { message } => PublicError {
                code: "workspace_adaptation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceReadinessValidation { message } => PublicError {
                code: "workspace_readiness_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceSessionValidation { message } => PublicError {
                code: "workspace_session_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceExperienceValidation { message } => PublicError {
                code: "workspace_experience_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceWorkContextValidation { message } => PublicError {
                code: "workspace_work_context_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceNavigationValidation { message } => PublicError {
                code: "workspace_navigation_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceMilestoneValidation { message } => PublicError {
                code: "workspace_milestone_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceWorkingStyleValidation { message } => PublicError {
                code: "workspace_working_style_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceTransitionValidation { message } => PublicError {
                code: "workspace_transition_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceInteractionValidation { message } => PublicError {
                code: "workspace_interaction_validation_error".into(),
                message: message.clone(),
            },
            KernelError::WorkspaceProfileValidation { message } => PublicError {
                code: "workspace_profile_validation_error".into(),
                message: message.clone(),
            },
            KernelError::InitializationFailed => PublicError {
                code: "initialization_failed".into(),
                message: "Workspace failed to initialize.".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_error_codes_preserve_failure_boundaries() {
        let cases = [
            (
                KernelError::ExecutionAtomicity {
                    message: "rollback uncertain".into(),
                },
                "execution_atomicity_error",
            ),
            (
                KernelError::ExecutionLifecyclePersistence {
                    stage: "claim",
                    source: DatabaseError::Migration("execution unavailable".into()),
                },
                "execution_lifecycle_persistence_error",
            ),
            (
                KernelError::ExecutionInProgress {
                    execution_request_id: "execution:s-1".into(),
                },
                "execution_in_progress",
            ),
            (
                KernelError::ExecutionReconciliationRequired {
                    execution_request_id: "execution:s-1".into(),
                },
                "execution_reconciliation_required",
            ),
            (
                KernelError::AuditPersistence {
                    stage: "permission.decision",
                    source: DatabaseError::Migration("audit unavailable".into()),
                },
                "audit_persistence_error",
            ),
            (
                KernelError::Internal {
                    message: "database lock poisoned".into(),
                },
                "internal_error",
            ),
            (
                KernelError::IntegrityViolation {
                    message: "recommendation boundary failed".into(),
                },
                "integrity_violation",
            ),
            (
                KernelError::PermissionDenied("denied".into()),
                "permission_denied",
            ),
            (
                KernelError::DecisionEngineNotFound,
                "decision_engine_not_found",
            ),
            (
                KernelError::DecisionEngineCannotExecute,
                "decision_engine_cannot_execute",
            ),
            (
                KernelError::RecommendationNotFound,
                "recommendation_not_found",
            ),
            (
                KernelError::RecommendationCannotExecute,
                "recommendation_cannot_execute",
            ),
            (
                KernelError::ProjectionValidation {
                    message: "projection failed".into(),
                },
                "projection_validation_error",
            ),
        ];

        for (error, expected_code) in cases {
            assert_eq!(error.to_public().code, expected_code);
        }
    }

    #[test]
    fn infrastructure_errors_do_not_expose_internal_context() {
        let public = KernelError::Internal {
            message: "sensitive lock detail".into(),
        }
        .to_public();
        assert_eq!(public.code, "internal_error");
        assert!(!public.message.contains("sensitive"));
    }

    #[test]
    fn persistence_boundary_rejections_map_to_subsystem_validation() {
        let recommendation = KernelError::from_recommendation_persistence(
            DatabaseError::ImmutableArtifact("outcome locked".into()),
        );
        assert_eq!(
            recommendation.to_public().code,
            "workspace_recommendation_engine_validation_error"
        );

        let decision = KernelError::from_decision_engine_persistence(
            DatabaseError::InvalidTransition("cannot reopen".into()),
        );
        assert_eq!(
            decision.to_public().code,
            "decision_engine_validation_error"
        );

        let queue = KernelError::from_decision_queue_persistence(DatabaseError::InvalidTransition(
            "dismissed".into(),
        ));
        assert_eq!(queue.to_public().code, "decision_queue_validation_error");

        let tasks = KernelError::from_task_graph_persistence(DatabaseError::ImmutableArtifact(
            "completed evidence".into(),
        ));
        assert_eq!(tasks.to_public().code, "task_graph_validation_error");
    }
}
