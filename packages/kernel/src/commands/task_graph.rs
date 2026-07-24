use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Task Graph reads.
pub struct GateTaskGraphRead;

impl crate::commands::Command for GateTaskGraphRead {
    fn name(&self) -> &'static str {
        "GateTaskGraphRead"
    }
}

impl QueryCommand for GateTaskGraphRead {
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

/// Capability gate for Task Graph writes (nodes/relationships — never executes).
pub struct GateTaskGraphWrite;

impl crate::commands::Command for GateTaskGraphWrite {
    fn name(&self) -> &'static str {
        "GateTaskGraphWrite"
    }
}

impl MutationCommand for GateTaskGraphWrite {
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
