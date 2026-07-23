//! Shared Workspace domain models — no database or UI logic.

pub mod entities;
pub mod errors;
pub mod workspace;

pub use entities::{
    ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{DomainError, Result};
