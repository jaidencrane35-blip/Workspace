use workspace_domain::{ActorContext, Capability, IntentContext, ResourceRef};

use crate::commands::context::CommandContext;
use crate::error::Result;
use crate::policy::{GovernanceClass, PolicyContext, PolicyDecision, PolicyResult};
use crate::security::{PermissionRequest, PermissionSubject};

/// Base metadata for all kernel commands.
pub trait Command {
    fn name(&self) -> &'static str;
}

/// State-changing command executed through the command pipeline.
pub trait MutationCommand: Command {
    type Output;

    fn permission_subject(&self) -> PermissionSubject;

    fn required_capability(&self) -> Capability;

    /// Optional resource address recorded in the audit trail after execution.
    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        let _ = (self, output);
        None
    }

    /// Optional JSON metadata recorded in the audit trail after execution.
    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let _ = (self, output);
        None
    }

    /// Optional JSON metadata recorded when the command fails before producing output.
    /// Used for execution-outcome classification (Sprint 25) without a parallel event system.
    fn audit_failure_metadata(&self) -> Option<String> {
        None
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Read-only command executed through the command pipeline.
///
/// Query commands declare their read capability and governance class (DEC-017)
/// so read governance can be applied without changing command shapes later.
pub trait QueryCommand: Command {
    type Output;

    fn permission_subject(&self) -> PermissionSubject;

    fn required_capability(&self) -> Capability;

    fn governance_class(&self) -> GovernanceClass;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Builds a permission request from command metadata and execution context.
pub fn permission_request(
    actor_context: &ActorContext,
    intent_context: &IntentContext,
    command: &'static str,
    subject: PermissionSubject,
    capability: Capability,
) -> PermissionRequest {
    PermissionRequest {
        actor: actor_context.actor.clone(),
        intent: intent_context.intent.clone(),
        capability,
        command,
        subject,
    }
}

/// Builds policy evaluation input from a permission request.
pub fn policy_context(request: &PermissionRequest) -> PolicyContext {
    PolicyContext::new(
        request.actor.id.to_string(),
        request.intent.clone(),
        request.capability.clone(),
        request.command,
        request.subject.clone(),
    )
}

/// Applies a policy result — denies when the policy rejects the operation.
pub fn require_policy(result: PolicyResult) -> Result<()> {
    match result.decision {
        PolicyDecision::Allow => Ok(()),
        PolicyDecision::Deny { reason } => {
            Err(crate::error::KernelError::PermissionDenied(reason))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{Capability, IntentContext, IntentType, ResourceKind};

    #[test]
    fn permission_request_includes_intent_and_capability() {
        let request = permission_request(
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            "CreateWorkspace",
            PermissionSubject::Resource(ResourceKind::Workspace),
            Capability::workspace_write(),
        );

        assert_eq!(request.intent.intent_type, IntentType::UserRequest);
        assert_eq!(request.capability.id.as_str(), "workspace.write");
        assert_eq!(request.command, "CreateWorkspace");
    }

    #[test]
    fn policy_context_derives_from_permission_request() {
        let request = permission_request(
            &ActorContext::local_user(),
            &IntentContext::system_shutdown(),
            "ShutdownWorkspace",
            PermissionSubject::System,
            Capability::system_shutdown(),
        );

        let context = policy_context(&request);
        assert_eq!(context.intent.intent_type, IntentType::SystemShutdown);
        assert_eq!(context.capability.id.as_str(), "system.shutdown");
    }
}
