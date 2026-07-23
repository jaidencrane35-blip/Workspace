use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ObservationService;
use workspace_domain::{Capability, Observation};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Returns a derived, read-only stream of recent workspace observations.
pub struct GetObservations {
    pub limit: usize,
}

impl GetObservations {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetObservations {
    fn name(&self) -> &'static str {
        "GetObservations"
    }
}

impl QueryCommand for GetObservations {
    type Output = Vec<Observation>;

    fn permission_subject(&self) -> PermissionSubject {
        // Observations derive from the audit trail: a sensitive, system-level
        // resource, not a graph resource.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Observations reveal the same activity history as audit; reuse the
        // audit read capability rather than inventing a new one.
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Activity history is sensitive; governed and audited regardless of
        // actor (DEC-017), consistent with GetAuditHistory.
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<Observation>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        ObservationService::list_recent(&ctx.database, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
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
    fn pipeline_returns_observations() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Observed".into()))
            .unwrap();

        let observations = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetObservations::new(Some(50)))
            .unwrap();

        assert!(!observations.is_empty());
        assert!(observations.iter().all(|obs| obs.validate().is_ok()));
    }

    #[test]
    fn observations_read_is_governed() {
        let command = GetObservations::new(None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
