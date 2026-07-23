use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceContextService;
use workspace_domain::{Capability, WorkspaceContext, WorkspaceId};

const DEFAULT_LIMIT: usize = 200;
const MAX_LIMIT: usize = 500;

/// Returns a deterministic, read-only composition of the derived workspace read
/// layers (state, activity, metrics, capabilities).
pub struct GetWorkspaceContext {
    pub workspace_id: WorkspaceId,
    pub limit: usize,
}

impl GetWorkspaceContext {
    pub fn new(workspace_id: WorkspaceId, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetWorkspaceContext {
    fn name(&self) -> &'static str {
        "GetWorkspaceContext"
    }
}

impl QueryCommand for GetWorkspaceContext {
    type Output = WorkspaceContext;

    fn permission_subject(&self) -> PermissionSubject {
        // Context aggregates sensitive, system-level derived reads (activity,
        // metrics, capabilities), not a single graph resource.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Context includes activity history derived from the audit trail; reuse
        // the audit read capability — the most sensitive input — rather than
        // inventing a new permission model.
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Combines activity history, workspace state, and capability
        // information; governed and audited regardless of actor (DEC-017).
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceContext> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let limit = self.limit.clamp(1, MAX_LIMIT);
        WorkspaceContextService::build(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            &ctx.capability_set,
            ctx.permission_policy,
            ctx.permission_gate,
            &self.workspace_id,
            limit,
        )
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
    fn pipeline_returns_workspace_context() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Context WS".into()))
            .unwrap();

        let context = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetWorkspaceContext::new(workspace.id.clone(), Some(200)))
            .unwrap();

        assert_eq!(context.snapshot.workspace_name, "Context WS");
        assert_eq!(context.workspace, context.snapshot.workspace);
        assert!(context.metrics.observation_count > 0);
        assert!(context.validate().is_ok());
    }

    #[test]
    fn context_read_is_governed() {
        let command =
            GetWorkspaceContext::new(WorkspaceId::new("ws-gov").unwrap(), None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }

    #[test]
    fn rejects_missing_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        let error = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetWorkspaceContext::new(
                WorkspaceId::new("missing-ws").unwrap(),
                None,
            ))
            .unwrap_err();

        assert!(matches!(error, KernelError::WorkspaceNotFound));
    }
}
