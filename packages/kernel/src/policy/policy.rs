use super::context::{PolicyContext, PolicyResult};
use crate::error::Result;

/// Contract for evaluating whether an operation is permitted.
pub trait PermissionPolicy: Send + Sync {
    fn evaluate(&self, context: &PolicyContext) -> Result<PolicyResult>;
}
