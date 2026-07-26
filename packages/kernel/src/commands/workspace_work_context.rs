use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Work Context Engine reads (semantic projection only).
/// Reuses existing `work_context.read` — no new Gateway paths.
pub struct GateWorkContextEngineRead;

impl crate::commands::Command for GateWorkContextEngineRead {
    fn name(&self) -> &'static str {
        "GateWorkContextEngineRead"
    }
}

impl QueryCommand for GateWorkContextEngineRead {
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
