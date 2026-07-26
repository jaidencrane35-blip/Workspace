use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Transition Engine reads (movement explanation only).
/// Reuses existing `work_context.read` — no new Gateway paths.
pub struct GateTransitionRead;

impl crate::commands::Command for GateTransitionRead {
    fn name(&self) -> &'static str {
        "GateTransitionRead"
    }
}

impl QueryCommand for GateTransitionRead {
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
