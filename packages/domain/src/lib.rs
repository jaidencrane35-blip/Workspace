//! Shared Workspace domain models — no database or UI logic.

pub mod entities;
pub mod errors;
pub mod ids;
pub mod workspace;

pub use entities::{
    ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{DomainError, Result};
pub use ids::{ApplicationId, WidgetId, WorkspaceId, ZoneId};
