//! Shared Workspace domain models — no database or UI logic.

pub mod actor;
pub mod audit;
pub mod capability;
pub mod entities;
pub mod errors;
pub mod ids;
pub mod intent;
pub mod workspace;

pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use audit::AuditEvent;
pub use capability::{Capability, CapabilityId, CapabilityScope, CapabilitySet};
pub use intent::{
    Intent, IntentContext, IntentMetadata, IntentType, SYSTEM_SHUTDOWN_INTENT_ID,
    SYSTEM_STARTUP_INTENT_ID, USER_REQUEST_INTENT_ID,
};
pub use entities::{
    ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{DomainError, Result};
pub use ids::{
    ActorId, ApplicationId, AuditEventId, IntentId, WidgetId, WorkspaceId, ZoneId,
};
