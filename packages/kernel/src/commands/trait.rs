use crate::commands::context::CommandContext;
use crate::error::Result;
use crate::security::PermissionRequest;

/// Base metadata for all kernel commands.
pub trait Command {
    fn name(&self) -> &'static str;
}

/// State-changing command executed through the command pipeline.
pub trait MutationCommand: Command {
    type Output;

    fn permission_request(&self) -> PermissionRequest;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}

/// Read-only command executed through the command pipeline.
pub trait QueryCommand: Command {
    type Output;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output>;
}
