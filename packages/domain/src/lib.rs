//! Shared Workspace domain models — no database or UI logic.

pub mod action_catalog;
pub mod actor;
pub mod ai_evaluation;
pub mod ai_orchestration;
pub mod ai_planning;
pub mod ai_request;
pub mod analytics;
pub mod audit;
pub mod capability;
pub mod context;
pub mod discovery;
pub mod entities;
pub mod errors;
pub mod execution_cancellation;
pub mod execution_context;
pub mod execution_guard;
pub mod execution_outcome;
pub mod execution_reconciliation;
pub mod graph;
pub mod ids;
pub mod intent;
pub mod intent_execution;
pub mod layout;
pub mod observation;
pub mod permission_approval;
pub mod projection;
pub mod resource;
pub mod suggestion;
pub mod suggestion_intent;
pub mod suggestion_lifecycle;
pub mod workspace;

pub use action_catalog::{
    ActionCatalog, ActionCatalogEntry, ActionCatalogError, AiActionAwareness,
};
pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use ai_evaluation::{
    classify_authority_outcome, evaluate_plan, evaluate_submission_result, AiEvaluationError,
    AiEvaluationSummary, AiPlanEvaluationReport, AiProposalEvaluation, AiProposalOutcomeClass,
    AiProposalQualityIssue, AiProposalRelevance, AiProposalValidity,
};
pub use ai_orchestration::{
    AiOrchestratedPlan, AiOrchestratedPlanState, AiOrchestrationError, AiPlanStep, AiPlanStepState,
};
pub use ai_planning::{
    AiActionProposal, AiApplicationAwareness, AiGoal, AiPlan, AiPlanSubmissionResult,
    AiPlanningContext, AiPlanningError, AiProposalAuthorityOutcome, AiProposalSubmission,
    AiWorkspaceAwareness,
};
pub use ai_request::{AiActionRequest, AiRequestError};
pub use analytics::{AnalyticsError, CategoryActivity, WorkspaceMetrics};
pub use audit::AuditEvent;
pub use capability::{Capability, CapabilityId, CapabilityScope, CapabilitySet};
pub use permission_approval::{
    ApprovalDecisionKind, ApprovalDecisionResult, CapabilityGrant, CapabilityGrantStatus,
    GrantKind, PermissionApprovalError, PermissionApprovalRequest, PermissionApprovalStatus,
};
pub use context::{ContextError, WorkspaceContext};
pub use discovery::{
    AvailableIntentSummary, CapabilityDiscovery, CapabilityDiscoveryError,
};
pub use intent::{
    ActionIntentCategory, ActionIntentDefinition, ActionIntentError, ActionIntentId,
    ActionIntentMetadata, ActionIntentRegistry, ActionIntentRequest, Intent, IntentContext,
    IntentMetadata, IntentType, TargetRequirement, AI_SUGGESTION_INTENT_ID,
    SYSTEM_SHUTDOWN_INTENT_ID, SYSTEM_STARTUP_INTENT_ID, USER_REQUEST_INTENT_ID,
};
pub use observation::{
    classify_event, neutral_summary, Observation, ObservationCategory, ObservationError,
    ObservationImportance,
};
pub use resource::{Addressable, ResourceId, ResourceKind, ResourceRef};
pub use suggestion::{
    derive_suggestions, find_pending_suggestion, Suggestion, SuggestionConfidence, SuggestionError,
    SuggestionStatus, SuggestionType,
};
pub use suggestion_lifecycle::{
    classify_suggestion_lifecycle_event, extract_suggestion_id, parse_canonical_resource_ref,
    SuggestionLifecycleError, SuggestionLifecycleRecord, SuggestionLifecycleState,
};
pub use suggestion_intent::{
    map_suggestion_type_to_intent, SuggestionIntentError, SuggestionIntentRequest,
};
pub use intent_execution::{
    IntentExecutionError, IntentExecutionRequest, IntentExecutionStatus,
};
pub use execution_cancellation::{
    sanitize_reason, CancellationRequest, CancellationStatus, ExecutionCancellationError,
};
pub use execution_context::{ExecutionContextError, ExecutionContextSummary};
pub use execution_guard::{
    evaluate_execution_guard, execution_request_id_for_suggestion, ExecutionGuardError,
    ExecutionGuardResult,
};
pub use execution_outcome::{
    classify_execution_outcome_event, outcome_from_audit_event, ExecutionOutcome,
    ExecutionOutcomeError, ExecutionOutcomeStatus,
};
pub use execution_reconciliation::{
    reconcile_execution_state, reconcile_execution_states, ExecutionReconciliation,
    ExecutionReconciliationError, ExecutionState,
};
pub use entities::{
    ApplicationLaunchResult, ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{validate_resource_name, DomainError, Result};
pub use graph::{GraphEdge, GraphRelationship};
pub use ids::{
    ActorId, AiActionProposalId, AiGoalId, AiOrchestratedPlanId, AiPlanStepId, ApplicationId,
    AuditEventId, CapabilityGrantId, IntentId, LayoutId, PermissionApprovalRequestId, WidgetId,
    WorkspaceId, ZoneId,
};
pub use layout::{
    Layout, LayoutBounds, LayoutError, LayoutMetadata, LayoutNode, LayoutSnapshot, Position2D,
    Size2D, Viewport,
};
pub use projection::{
    ApplicationSummary, LayoutPlacementSummary, ProjectionError, ProjectionRelationship,
    WidgetSummary, WorkspaceSnapshot, ZoneSummary,
};
