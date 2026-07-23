//! Shared Workspace domain models — no database or UI logic.

pub mod actor;
pub mod audit;
pub mod capability;
pub mod discovery;
pub mod entities;
pub mod errors;
pub mod graph;
pub mod ids;
pub mod intent;
pub mod layout;
pub mod observation;
pub mod projection;
pub mod resource;
pub mod workspace;

pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use audit::AuditEvent;
pub use capability::{Capability, CapabilityId, CapabilityScope, CapabilitySet};
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
