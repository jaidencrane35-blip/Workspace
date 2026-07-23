use workspace_domain::Actor;

use crate::error::{KernelError, Result};

/// What kind of resource a command intends to mutate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionSubject {
    System,
    Settings,
    Workspace,
}

/// Authorization context for a mutation command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionRequest {
    pub actor: Actor,
    pub command: &'static str,
    pub subject: PermissionSubject,
}

/// Outcome of a permission check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    Allowed,
    Denied { reason: String },
}

/// Boundary for user approval, automation policy, and future AI suggestions.
pub trait PermissionGate: Send + Sync {
    fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision>;

    fn require(&self, request: &PermissionRequest) -> Result<()> {
        match self.authorize(request)? {
            PermissionDecision::Allowed => Ok(()),
            PermissionDecision::Denied { reason } => Err(KernelError::PermissionDenied(reason)),
        }
    }
}
