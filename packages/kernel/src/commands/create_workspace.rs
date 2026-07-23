use crate::commands::context::CommandContext;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceEntityCreated};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::WorkspaceService;
use workspace_domain::Workspace;

/// Creates a new workspace domain entity.
pub struct CreateWorkspace {
    pub name: String,
}

impl crate::commands::Command for CreateWorkspace {
    fn name(&self) -> &'static str {
        "CreateWorkspace"
    }
}

impl MutationCommand for CreateWorkspace {
    type Output = Workspace;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Workspace
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Workspace> {
        Self::ensure_ready(ctx.state)?;
        let workspace = ctx.with_database(|db| WorkspaceService::create(db, self.name))?;
        ctx.event_bus.publish(DomainEvent::WorkspaceCreated(WorkspaceEntityCreated {
            workspace_id: workspace.id.to_string(),
            name: workspace.name.clone(),
            actor: Some(ctx.actor_context.clone()),
        }));
        Ok(workspace)
    }
}

impl CreateWorkspace {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn ensure_ready(state: &crate::state::WorkspaceState) -> Result<()> {
        if state.lifecycle == LifecycleState::Ready {
            Ok(())
        } else {
            Err(KernelError::NotReady)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::context::CommandContext;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::security::AllowAllPermissionGate;
    use std::sync::{Arc, Mutex};
    use workspace_domain::ActorContext;

    #[test]
    fn emits_workspace_created_event() {
        let bus = EventBus::new();
        let event_name = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&event_name);

        bus.subscribe(move |event| {
            *captured.lock().unwrap() = event.name().to_string();
        });

        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::local_user(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        };

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Dev".into()))
            .unwrap();

        assert_eq!(workspace.name, "Dev");
        assert_eq!(*event_name.lock().unwrap(), "workspace.entity.created");
    }
}
