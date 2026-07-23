use crate::commands::context::CommandContext;
use crate::commands::r#trait::{Command, MutationCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
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

    fn permission_request(&self) -> PermissionRequest {
        PermissionRequest {
            command: self.name(),
            subject: PermissionSubject::Workspace,
        }
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Workspace> {
        Self::ensure_ready(ctx.state)?;
        WorkspaceService::create(ctx.database, ctx.event_bus, self.name)
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
            state: &init.state,
            database: init.database.database(),
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
