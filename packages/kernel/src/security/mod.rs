//! Permission and authorization boundaries for state-changing operations.

mod allow_all;
mod gate;

pub use allow_all::AllowAllPermissionGate;
pub use gate::{PermissionDecision, PermissionGate, PermissionRequest, PermissionSubject};
