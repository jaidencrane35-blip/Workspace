//! Shared Workspace domain models — no database or UI logic.

pub mod actor;
pub mod analytics;
pub mod audit;
pub mod capability;
pub mod context;
pub mod discovery;
pub mod entities;
pub mod errors;
pub mod execution_outcome;
pub mod graph;
pub mod ids;
pub mod intent;
pub mod intent_execution;
pub mod layout;
pub mod observation;
pub mod projection;
pub mod resource;
pub mod suggestion;
pub mod suggestion_intent;
pub mod suggestion_lifecycle;
pub mod workspace;

pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use analytics::{AnalyticsError, CategoryActivity, WorkspaceMetrics};
pub use audit::AuditEvent;
pub use capability::{Capability, CapabilityId, CapabilityScope, CapabilitySet};
pub use context::{ContextError, WorkspaceContext};
pub use discovery::{
    AvailableIntentSummary, CapabilityDiscovery, CapabilityDiscoveryError,
};
pub use intent::{
    ActionIntentCategory, ActionIntentDefinition, ActionIntentError, ActionIntentId,
    ActionIntentMetadata, ActionIntentRegistry, ActionIntentRequest, Intent, IntentContext,
    IntentMetadata, IntentType, TargetRequirement, SYSTEM_SHUTDOWN_INTENT_ID,
    SYSTEM_STARTUP_INTENT_ID, USER_REQUEST_INTENT_ID,
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
pub use execution_outcome::{
    classify_execution_outcome_event, outcome_from_audit_event, ExecutionOutcome,
    ExecutionOutcomeError, ExecutionOutcomeStatus,
};
pub use entities::{
    ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{validate_resource_name, DomainError, Result};
pub use graph::{GraphEdge, GraphRelationship};
pub use ids::{
    ActorId, ApplicationId, AuditEventId, IntentId, LayoutId, WidgetId, WorkspaceId, ZoneId,
};
pub use layout::{
    Layout, LayoutBounds, LayoutError, LayoutMetadata, LayoutNode, LayoutSnapshot, Position2D,
    Size2D, Viewport,
};
pub use projection::{
    ApplicationSummary, LayoutPlacementSummary, ProjectionError, ProjectionRelationship,
    WidgetSummary, WorkspaceSnapshot, ZoneSummary,
};
