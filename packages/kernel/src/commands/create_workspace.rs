use workspace_database::Database;

use crate::commands::Command;
use crate::error::Result;
use crate::events::EventBus;
use crate::services::WorkspaceService;
use crate::state::WorkspaceState;
use crate::lifecycle::LifecycleState;
use workspace_domain::Workspace;

/// Creates a new workspace domain entity.
pub struct CreateWorkspace {
    pub name: String,
}

impl Command for CreateWorkspace {
    fn name(&self) -> &'static str {
        "CreateWorkspace"
    }
}

impl CreateWorkspace {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn execute(
        self,
        state: &WorkspaceState,
        database: &Database,
        event_bus: &EventBus,
    ) -> Result<Workspace> {
        log::info!("COMMAND: CreateWorkspace");

        if state.lifecycle != LifecycleState::Ready {
            return Err(crate::error::KernelError::NotReady);
        }

        WorkspaceService::create(database, event_bus, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::events::EventBus;
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
        let workspace = CreateWorkspace::new("Dev".into())
            .execute(&init.state, init.database.database(), &bus)
            .unwrap();

        assert_eq!(workspace.name, "Dev");
        assert_eq!(*event_name.lock().unwrap(), "workspace.entity.created");
    }
}
