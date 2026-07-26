use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Workspace Profile reads.
/// Reuses existing `work_context.read` — no new Gateway paths.
pub struct GateWorkspaceProfileRead;

impl crate::commands::Command for GateWorkspaceProfileRead {
    fn name(&self) -> &'static str {
        "GateWorkspaceProfileRead"
    }
}

impl QueryCommand for GateWorkspaceProfileRead {
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

/// Capability gate for Workspace Profile mutations (create/update — never executes setups).
/// Reuses existing `work_context.write` — no new Gateway paths.
pub struct GateWorkspaceProfileWrite;

impl crate::commands::Command for GateWorkspaceProfileWrite {
    fn name(&self) -> &'static str {
        "GateWorkspaceProfileWrite"
    }
}

impl MutationCommand for GateWorkspaceProfileWrite {
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
