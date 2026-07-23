use workspace_database::Database;

use crate::commands::Command;
use crate::error::Result;
use crate::lifecycle::LifecycleState;
use crate::services::WorkspaceService;
use crate::state::WorkspaceState;
use workspace_domain::Workspace;

/// Retrieves a workspace domain entity by id.
pub struct GetWorkspace {
    pub id: String,
}

impl Command for GetWorkspace {
    fn name(&self) -> &'static str {
        "GetWorkspace"
    }
}

impl GetWorkspace {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn execute(self, state: &WorkspaceState, database: &Database) -> Result<Workspace> {
        log::info!("COMMAND: GetWorkspace");

        if state.lifecycle != LifecycleState::Ready {
            return Err(crate::error::KernelError::NotReady);
        }

        WorkspaceService::get(database, &self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::events::EventBus;

    #[test]
    fn retrieves_created_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        let created = CreateWorkspace::new("Find Me".into())
            .execute(&init.state, init.database.database(), &bus)
            .unwrap();

        let loaded = GetWorkspace::new(created.id.clone())
            .execute(&init.state, init.database.database())
            .unwrap();

        assert_eq!(loaded, created);
    }
}
