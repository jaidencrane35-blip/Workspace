use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Interaction Model reads (opportunities only).
/// Reuses existing `work_context.read` — no new Gateway paths.
pub struct GateWorkspaceInteractionRead;

impl crate::commands::Command for GateWorkspaceInteractionRead {
    fn name(&self) -> &'static str {
        "GateWorkspaceInteractionRead"
    }
}

impl QueryCommand for GateWorkspaceInteractionRead {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<()> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(())
    }
}

/// Capability gate for Interaction select (handoff audit only — never executes).
/// Reuses existing `work_context.write` — no new Gateway paths.
pub struct GateWorkspaceInteractionWrite;

impl crate::commands::Command for GateWorkspaceInteractionWrite {
    fn name(&self) -> &'static str {
        "GateWorkspaceInteractionWrite"
    }
}

impl MutationCommand for GateWorkspaceInteractionWrite {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(())
    }
}
