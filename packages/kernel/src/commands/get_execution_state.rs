//! Governed read of reconciled execution state (Sprint 29).

use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ExecutionReconciliationService;
use workspace_domain::{Capability, ExecutionReconciliation};

/// Returns the current interpreted state for one execution request id.
pub struct GetExecutionState {
    pub execution_request_id: String,
}

impl GetExecutionState {
    pub fn new(execution_request_id: String) -> Self {
        Self {
            execution_request_id,
        }
    }
}

impl crate::commands::Command for GetExecutionState {
    fn name(&self) -> &'static str {
        "GetExecutionState"
    }
}

impl QueryCommand for GetExecutionState {
    type Output = ExecutionReconciliation;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ExecutionReconciliation> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ExecutionReconciliationService::reconcile(&ctx.database, &self.execution_request_id)
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
    fn pipeline_returns_reconciled_state() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("State WS".into()))
            .unwrap();
        let suggestion_id = "missing-suggestion";
        let _ = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
            ExecuteIntentRequest::new(workspace.id.clone(), suggestion_id.into()),
        );
        let execution_request_id = format!("execution:{suggestion_id}");

        let state = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetExecutionState::new(execution_request_id.clone()))
            .unwrap();

        assert_eq!(state.execution_request_id, execution_request_id);
        assert_eq!(state.current_state, ExecutionState::Failed);
        assert!(state.dispatch_allowed);
        assert!(state.cancellation_allowed);
        assert!(state.validate().is_ok());
    }

    #[test]
    fn execution_state_read_is_governed() {
        let command = GetExecutionState::new("execution:s-1".into());
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert_eq!(command.required_capability(), Capability::audit_read());
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
