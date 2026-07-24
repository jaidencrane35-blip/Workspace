use workspace_domain::{Actor, Capability, Intent, ResourceKind};

use crate::error::{KernelError, Result};

/// What a command intends to act on (DEC-016).
///
/// Resource operations are addressed by `ResourceKind`; `System` covers
/// non-resource operations (startup, shutdown, settings-as-system).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionSubject {
    System,
    Resource(ResourceKind),
}

/// Authorization context for a mutation command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionRequest {
    pub actor: Actor,
    pub intent: Intent,
    pub capability: Capability,
    pub command: &'static str,
    pub subject: PermissionSubject,
    /// Optional concrete resource id (e.g. application id) for audit/scoping.
    /// Does not grant authority; used so approvals can be workspace-attributed.
    pub target_resource_id: Option<String>,
}

/// Outcome of a permission check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    Allowed,
    Denied { reason: String },
    /// Future approval UI — not executable until an approval flow grants authority.
    ApprovalRequired { reason: String },
}

/// Boundary for user approval, automation policy, and future AI suggestions.
pub trait PermissionGate: Send + Sync {
    fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision>;

    fn require(&self, request: &PermissionRequest) -> Result<()> {
        match self.authorize(request)? {
            PermissionDecision::Allowed => Ok(()),
            PermissionDecision::Denied { reason } => Err(KernelError::PermissionDenied(reason)),
            PermissionDecision::ApprovalRequired { reason } => Err(KernelError::ApprovalRequired {
                reason,
                approval_request_id: String::new(),
            }),
        }
    }
}
