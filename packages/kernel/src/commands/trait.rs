use workspace_domain::ActorContext;

use crate::commands::context::CommandContext;
use crate::error::Result;
use crate::security::{PermissionRequest, PermissionSubject};

/// Base metadata for all kernel commands.
pub trait Command {
    fn name(&self) -> &'static str;
}

/// State-changing command executed through the command pipeline.
pub trait MutationCommand: Command {
    type Output;

    fn permission_subject(&self) -> PermissionSubject;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Read-only command executed through the command pipeline.
pub trait QueryCommand: Command {
    type Output;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Builds a permission request from command metadata and execution actor.
pub fn permission_request(
    actor_context: &ActorContext,
    command: &'static str,
    subject: PermissionSubject,
) -> PermissionRequest {
    PermissionRequest {
        actor: actor_context.actor.clone(),
        command,
        subject,
    }
}
