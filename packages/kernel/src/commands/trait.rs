use workspace_domain::{ActorContext, Capability, IntentContext};

use crate::commands::context::CommandContext;
use crate::error::Result;
use crate::policy::{PolicyContext, PolicyDecision, PolicyResult};
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

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Read-only command executed through the command pipeline.
pub trait QueryCommand: Command {
    type Output;

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
    use workspace_domain::{Capability, IntentContext, IntentType};

    #[test]
    fn permission_request_includes_intent_and_capability() {
        let request = permission_request(
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            "CreateWorkspace",
            PermissionSubject::Workspace,
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
