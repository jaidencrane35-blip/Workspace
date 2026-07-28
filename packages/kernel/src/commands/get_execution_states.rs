//! Governed read of reconciled execution states list (Sprint 30).

use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ExecutionReconciliationService;
use workspace_domain::{Capability, ExecutionLifecycleProjection};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Returns a dual-channel execution lifecycle projection (actionable + history).
pub struct GetExecutionStates {
    pub limit: usize,
}

impl GetExecutionStates {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetExecutionStates {
    fn name(&self) -> &'static str {
        "GetExecutionStates"
    }
}

impl QueryCommand for GetExecutionStates {
    type Output = ExecutionLifecycleProjection;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ExecutionLifecycleProjection> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        ExecutionReconciliationService::projection(&ctx.database, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::execute_intent_request::ExecuteIntentRequest;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::CommandContext;
    use crate::events::EventBus;
    use crate::policy::{read_is_governed, AlwaysAllowPolicy, GovernanceClass};
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{
        ActorContext, ActorType, CapabilitySet, ExecutionState, IntentContext,
    };

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
    fn pipeline_returns_execution_states() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("States WS".into()))
            .unwrap();
        let _ = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
            ExecuteIntentRequest::new(workspace.id.clone(), "missing-suggestion".into()),
        );

        let projection = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetExecutionStates::new(Some(50)))
            .unwrap();

        assert!(projection.history.iter().any(|entry| {
            entry.execution_request_id == "execution:missing-suggestion"
                && entry.state == ExecutionState::Failed
                && entry.is_non_actionable()
        }));
        assert!(projection.history.iter().all(|entry| entry.is_non_actionable()));
        assert_eq!(projection.authority_effect, "none");
    }

    #[test]
    fn execution_states_read_is_governed() {
        let command = GetExecutionStates::new(None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert_eq!(command.required_capability(), Capability::audit_read());
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
