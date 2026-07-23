use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::services::WorkspaceService;
use workspace_domain::{Workspace, WorkspaceId};

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

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Workspace> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        WorkspaceService::get(ctx.database, &self.id)
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
    use crate::security::AllowAllPermissionGate;

    #[test]
    fn retrieves_created_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            state: &init.state,
            database: init.database.database(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        };

        let created = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Find Me".into()))
            .unwrap();

        let loaded = CommandPipeline::new(CommandContext {
            state: &init.state,
            database: init.database.database(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        })
        .execute_query(GetWorkspace::new(created.id.clone()))
        .unwrap();

        assert_eq!(loaded, created);
    }
}
