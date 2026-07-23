use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceService;
use workspace_domain::{Capability, ResourceKind, Workspace, WorkspaceId};

/// Retrieves a workspace domain entity by id.
pub struct GetWorkspace {
    pub id: WorkspaceId,
}

impl crate::commands::Command for GetWorkspace {
    fn name(&self) -> &'static str {
        "GetWorkspace"
    }
}

impl QueryCommand for GetWorkspace {
    type Output = Workspace;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Workspace> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ctx.with_database(|db| WorkspaceService::get(db, &self.id))
    }
}

impl GetWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::context::CommandContext;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    fn test_context<'a>(
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
    fn retrieves_created_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        let created = CommandPipeline::new(test_context(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Find Me".into()))
            .unwrap();

        let loaded = CommandPipeline::new(test_context(&init, &bus))
            .execute_query(GetWorkspace::new(created.id.clone()))
            .unwrap();

        assert_eq!(loaded, created);
    }
}
