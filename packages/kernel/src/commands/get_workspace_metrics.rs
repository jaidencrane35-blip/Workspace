use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAnalyticsService;
use workspace_domain::{Capability, WorkspaceMetrics};

const DEFAULT_LIMIT: usize = 200;
const MAX_LIMIT: usize = 500;

/// Returns deterministic, derived analytics over recent workspace observations.
pub struct GetWorkspaceMetrics {
    pub limit: usize,
}

impl GetWorkspaceMetrics {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetWorkspaceMetrics {
    fn name(&self) -> &'static str {
        "GetWorkspaceMetrics"
    }
}

impl QueryCommand for GetWorkspaceMetrics {
    type Output = WorkspaceMetrics;

    fn permission_subject(&self) -> PermissionSubject {
        // Metrics derive from observations/audit: a sensitive, system-level
        // resource, not a graph resource.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Metrics reveal the same activity history as observations/audit; reuse
        // the audit read capability rather than inventing a new one.
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Activity analytics is sensitive; governed and audited regardless of
        // actor (DEC-017), consistent with GetObservations / GetAuditHistory.
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceMetrics> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        WorkspaceAnalyticsService::compute(&ctx.database, limit)
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
    fn pipeline_returns_metrics() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Analyzed".into()))
            .unwrap();

        let metrics = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetWorkspaceMetrics::new(Some(200)))
            .unwrap();

        assert!(metrics.observation_count > 0);
        assert!(metrics.resource_creation_count >= 1);
        assert!(metrics.validate().is_ok());
    }

    #[test]
    fn metrics_read_is_governed() {
        let command = GetWorkspaceMetrics::new(None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
