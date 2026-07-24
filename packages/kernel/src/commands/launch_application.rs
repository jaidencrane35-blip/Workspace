use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_ready;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::ApplicationLaunchService;
use workspace_domain::{
    ApplicationId, ApplicationLaunchResult, Capability, ResourceId, ResourceKind, ResourceRef,
};

/// Launches a registered desktop application through the Permission Gateway.
///
/// OS spawn occurs only after gateway Allow. Tests inject the stub launcher via
/// [`ApplicationLaunchService::launch_simulated`] when needed; production
/// command uses the platform launcher.
pub struct LaunchApplication {
    pub application_id: ApplicationId,
    /// When true, uses stub launcher (no OS spawn) — for integration tests.
    pub(crate) simulate: bool,
}

impl LaunchApplication {
    pub fn new(application_id: ApplicationId) -> Self {
        Self {
            application_id,
            simulate: false,
        }
    }

    pub fn simulated(application_id: ApplicationId) -> Self {
        Self {
            application_id,
            simulate: true,
        }
    }
}

impl crate::commands::Command for LaunchApplication {
    fn name(&self) -> &'static str {
        "LaunchApplication"
    }
}

impl MutationCommand for LaunchApplication {
    type Output = ApplicationLaunchResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Application)
    }

    fn permission_target_id(&self) -> Option<String> {
        Some(self.application_id.to_string())
    }

    fn required_capability(&self) -> Capability {
        Capability::application_launch()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Application,
            ResourceId::new(output.application_id.as_str()).expect("application id is non-empty"),
        ))
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "application_name": output.name,
                "executable_path": output.executable_path,
                "process_id": output.process_id,
                "simulated": output.simulated,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ApplicationLaunchResult> {
        ensure_ready(ctx)?;
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ctx.with_database(|db| {
            if self.simulate {
                ApplicationLaunchService::launch_simulated(db, &self.application_id)
            } else {
                ApplicationLaunchService::launch(db, &self.application_id)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::application::CreateApplication;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
    use crate::security::{
        AllowAllPermissionGate, StandardPermissionGate,
    };
    use crate::services::AuditService;
    use workspace_domain::{Actor, ActorContext, ActorType, CapabilitySet, IntentContext};

    fn ready_ctx<'a>(
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        bus: &'a EventBus,
        gate: &'a dyn crate::security::PermissionGate,
        policy: &'a dyn crate::policy::PermissionPolicy,
        capabilities: CapabilitySet,
        actor: ActorContext,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: actor,
            intent_context: IntentContext::user_request(),
            capability_set: capabilities,
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: gate,
            permission_policy: policy,
        }
    }

    fn seed_launchable_app(
        init: &crate::commands::initialize::InitializeWorkspaceResult,
        bus: &EventBus,
    ) -> ApplicationId {
        let workspace = CommandPipeline::new(ready_ctx(
            init,
            bus,
            &AllowAllPermissionGate,
            &AlwaysAllowPolicy,
            CapabilitySet::local_user_standard(),
            ActorContext::local_user(),
        ))
        .execute_mutation(CreateWorkspace::new("Launch WS".into()))
        .unwrap();

        let app = CommandPipeline::new(ready_ctx(
            init,
            bus,
            &AllowAllPermissionGate,
            &AlwaysAllowPolicy,
            CapabilitySet::local_user_standard(),
            ActorContext::local_user(),
        ))
        .execute_mutation(CreateApplication::new(
            workspace.id,
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap();

        app.id
    }

    #[test]
    fn successful_launch_goes_through_gateway() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let app_id = seed_launchable_app(&init, &bus);

        let result = CommandPipeline::new(ready_ctx(
            &init,
            &bus,
            &StandardPermissionGate,
            &CapabilityBoundPolicy,
            CapabilitySet::local_user_standard(),
            ActorContext::local_user(),
        ))
        .execute_mutation(LaunchApplication::simulated(app_id.clone()))
        .unwrap();

        assert_eq!(result.name, "Notepad");
        assert!(result.simulated);

        let records = AuditService::list_recent(&init.database.shared(), 30).unwrap();
        assert!(records.iter().any(|r| {
            r.event_type == "permission.allowed"
                && r.command_name.as_deref() == Some("LaunchApplication")
        }));
        assert!(records.iter().any(|r| {
            r.event_type == "command.executed"
                && r.command_name.as_deref() == Some("LaunchApplication")
                && r.success
        }));
    }

    #[test]
    fn denied_launch_does_not_execute() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let app_id = seed_launchable_app(&init, &bus);

        let error = CommandPipeline::new(ready_ctx(
            &init,
            &bus,
            &AllowAllPermissionGate,
            &CapabilityBoundPolicy,
            CapabilitySet::new(),
            ActorContext::local_user(),
        ))
        .execute_mutation(LaunchApplication::simulated(app_id))
        .unwrap_err();

        assert!(matches!(error, KernelError::PermissionDenied(_)));

        let records = AuditService::list_recent(&init.database.shared(), 30).unwrap();
        assert!(records.iter().any(|r| {
            r.event_type == "permission.denied"
                && r.command_name.as_deref() == Some("LaunchApplication")
        }));
        assert!(!records.iter().any(|r| {
            r.event_type == "command.executed"
                && r.command_name.as_deref() == Some("LaunchApplication")
        }));
    }

    #[test]
    fn approval_required_blocks_ai_launch() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let app_id = seed_launchable_app(&init, &bus);

        let error = CommandPipeline::new(ready_ctx(
            &init,
            &bus,
            &StandardPermissionGate,
            &AlwaysAllowPolicy,
            CapabilitySet::local_user_standard(),
            ActorContext::new(Actor::ai_assistant("ai-1").unwrap()),
        ))
        .execute_mutation(LaunchApplication::simulated(app_id))
        .unwrap_err();

        assert!(matches!(
            error,
            KernelError::ApprovalRequired { .. }
        ));
        assert_eq!(error.to_public().code, "approval_required");

        let records = AuditService::list_recent(&init.database.shared(), 30).unwrap();
        assert!(records.iter().any(|r| {
            r.event_type == "permission.approval_required"
                && r.command_name.as_deref() == Some("LaunchApplication")
                && r.actor_type == ActorType::AIAssistant
        }));
        assert!(!records.iter().any(|r| {
            r.event_type == "command.executed"
                && r.command_name.as_deref() == Some("LaunchApplication")
        }));
    }
}
