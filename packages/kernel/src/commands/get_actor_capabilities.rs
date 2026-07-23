use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::CapabilityResolver;
use workspace_domain::{Capability, CapabilityDiscovery};

/// Returns a derived view of the requesting actor's capabilities and available actions.
pub struct GetActorCapabilities;

impl crate::commands::Command for GetActorCapabilities {
    fn name(&self) -> &'static str {
        "GetActorCapabilities"
    }
}

impl QueryCommand for GetActorCapabilities {
    type Output = CapabilityDiscovery;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Authorization metadata is sensitive; governed for all actors (DEC-017).
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<CapabilityDiscovery> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        CapabilityResolver::discover(
            &ctx.actor_context,
            &ctx.intent_context,
            &ctx.capability_set,
            ctx.permission_policy,
            ctx.permission_gate,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::{read_is_governed, AlwaysAllowPolicy, GovernanceClass};
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, ActorType, CapabilitySet, IntentContext};

    fn ready_ctx<'a>(
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
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
    fn pipeline_returns_capability_discovery() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        let discovery = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetActorCapabilities)
            .unwrap();

        assert_eq!(discovery.actor_type, ActorType::LocalUser);
        assert!(!discovery.generated_at.is_empty());
        assert!(discovery.validate().is_ok());
    }

    #[test]
    fn capability_discovery_read_is_governed() {
        let command = GetActorCapabilities;
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
