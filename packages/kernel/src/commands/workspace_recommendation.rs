use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Capability gate for Recommendation Engine reads (aggregation only).
pub struct GateRecommendationEngineRead;

impl crate::commands::Command for GateRecommendationEngineRead {
    fn name(&self) -> &'static str {
        "GateRecommendationEngineRead"
    }
}

impl QueryCommand for GateRecommendationEngineRead {
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

/// Capability gate for Recommendation Engine lifecycle writes (overlay only — never executes).
pub struct GateRecommendationEngineWrite;

impl crate::commands::Command for GateRecommendationEngineWrite {
    fn name(&self) -> &'static str {
        "GateRecommendationEngineWrite"
    }
}

impl MutationCommand for GateRecommendationEngineWrite {
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
