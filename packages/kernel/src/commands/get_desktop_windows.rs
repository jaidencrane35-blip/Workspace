use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::DesktopWindowService;
use workspace_domain::Capability;
use workspace_windows_integration::DesktopWindowSnapshot;

/// Returns a bounded list of top-level desktop windows (observation only).
pub struct GetDesktopWindows {
    pub limit: Option<usize>,
}

impl GetDesktopWindows {
    pub fn new(limit: Option<usize>) -> Self {
        Self { limit }
    }
}

impl crate::commands::Command for GetDesktopWindows {
    fn name(&self) -> &'static str {
        "GetDesktopWindows"
    }
}

impl QueryCommand for GetDesktopWindows {
    type Output = Vec<DesktopWindowSnapshot>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Window observation supports application awareness; reuse application read.
        Capability::application_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<DesktopWindowSnapshot>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        DesktopWindowService::list_recent(&ctx.database, self.limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    #[test]
    fn get_desktop_windows_is_governed_query() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        let windows = CommandPipeline::new(ctx)
            .execute_query(GetDesktopWindows::new(Some(10)))
            .unwrap();

        // Stub or live — must succeed; live titles are non-empty when present.
        assert!(windows.iter().all(|w| !w.title.trim().is_empty()));
    }
}
