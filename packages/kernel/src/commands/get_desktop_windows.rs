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
        Capability::desktop_read()
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
    use crate::commands::initialize::{InitializeWorkspace, InitializeWorkspaceResult};
    use crate::commands::GateObservationRead;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::{AllowAllPermissionGate, StandardPermissionGate};
    use crate::services::WorkspaceObservationService;
    use workspace_domain::{Actor, ActorContext, CapabilitySet, IntentContext};
    use workspace_windows_integration::StubDesktopCapturer;

    fn local_ctx<'a>(
        init: &'a InitializeWorkspaceResult,
        bus: &'a EventBus,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        }
    }

    #[test]
    fn get_desktop_windows_is_governed_query() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = local_ctx(&init, &bus);

        let windows = CommandPipeline::new(ctx)
            .execute_query(GetDesktopWindows::new(Some(10)))
            .unwrap();

        assert!(windows.is_empty());
    }

    #[test]
    fn get_desktop_windows_requires_desktop_read() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::new(Actor::ai_assistant("ai-desktop").unwrap()),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::for_actor_type(workspace_domain::ActorType::AIAssistant),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &StandardPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        let result = CommandPipeline::new(ctx).execute_query(GetDesktopWindows::new(Some(10)));
        match result {
            Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
            other => panic!("expected permission failure, got {other:?}"),
        }
    }

    #[test]
    fn get_desktop_windows_reads_persisted_snapshot() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        CommandPipeline::new(local_ctx(&init, &bus))
            .execute_query(GateObservationRead)
            .unwrap();
        WorkspaceObservationService::capture_with(
            &init.database.shared(),
            &local,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let windows = CommandPipeline::new(local_ctx(&init, &bus))
            .execute_query(GetDesktopWindows::new(Some(10)))
            .unwrap();
        assert_eq!(windows.len(), 4);
        assert!(windows.iter().any(|window| window.focused));
    }
}
