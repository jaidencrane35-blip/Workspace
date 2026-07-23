//! Shared Workspace domain models — no database or UI logic.

pub mod actor;
pub mod audit;
pub mod entities;
pub mod errors;
pub mod ids;
pub mod workspace;

pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use audit::AuditEvent;
pub use entities::{
    ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{DomainError, Result};
pub use ids::{ActorId, ApplicationId, AuditEventId, WidgetId, WorkspaceId, ZoneId};
