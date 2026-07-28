use std::path::{Path, PathBuf};

use workspace_database::DatabaseService;

use crate::commands::Command;
use crate::error::{KernelError, Result};
use crate::events::EventBus;
use crate::lifecycle::LifecycleState;
use crate::services::{ConfigurationService, DatabaseServiceHandle, ServiceRegistry, ServiceStatus};
use crate::state::WorkspaceState;
use crate::{KERNEL_VERSION, SERVICE_CONFIGURATION, SERVICE_DATABASE, SERVICE_WORKSPACE};

/// Initializes database services and transitions workspace to ready.
pub struct InitializeWorkspace {
    pub db_path: PathBuf,
    pub migrations_dir: Option<PathBuf>,
}

impl Command for InitializeWorkspace {
    fn name(&self) -> &'static str {
        "InitializeWorkspace"
    }
}

pub struct InitializeWorkspaceResult {
    pub state: WorkspaceState,
    pub database: DatabaseServiceHandle,
    pub services: ServiceRegistry,
}

impl InitializeWorkspace {
    pub fn at_path(db_path: impl AsRef<Path>) -> Self {
        Self {
            db_path: db_path.as_ref().to_path_buf(),
            migrations_dir: None,
        }
    }

    pub fn in_memory() -> Self {
        Self {
            db_path: PathBuf::from(":memory:"),
            migrations_dir: None,
        }
    }

    pub fn execute(self, event_bus: &EventBus) -> Result<InitializeWorkspaceResult> {
        log::info!("COMMAND: InitializeWorkspace");
        let _ = event_bus;

        // Domain events WorkspaceStarted / WorkspaceReady are published by
        // CommandHandler *after* apply_runtime registers AuditEventSubscriber,
        // so startup evidence is durable. Do not publish here.

        let mut state = WorkspaceState::new(KERNEL_VERSION);
        let mut services = ServiceRegistry::new();
        state.transition(LifecycleState::Initializing)?;

        let database = if self.db_path.to_string_lossy() == ":memory:" {
            Self::initialize_in_memory_database(&mut services)?
        } else if let Some(migrations_dir) = self.migrations_dir {
            let service = DatabaseService::initialize_with_migrations(
                &self.db_path,
                migrations_dir,
            )
            .map_err(|error| {
                log::error!("InitializeWorkspace database failure: {error}");
                services.register(SERVICE_DATABASE, ServiceStatus::Failed);
                match state.transition(LifecycleState::Error) {
                    Ok(()) => KernelError::from(error),
                    Err(transition_error) => transition_error,
                }
            })?;
            services.register(SERVICE_DATABASE, ServiceStatus::Healthy);
            service.into_database()
        } else {
            let service = DatabaseService::initialize(&self.db_path).map_err(|error| {
                log::error!("InitializeWorkspace database failure: {error}");
                services.register(SERVICE_DATABASE, ServiceStatus::Failed);
                match state.transition(LifecycleState::Error) {
                    Ok(()) => KernelError::from(error),
                    Err(transition_error) => transition_error,
                }
            })?;
            services.register(SERVICE_DATABASE, ServiceStatus::Healthy);
            service.into_database()
        };

        ConfigurationService::initialize(&database).map_err(|error| {
            log::error!("InitializeWorkspace configuration failure: {error}");
            services.register(SERVICE_CONFIGURATION, ServiceStatus::Failed);
            match state.transition(LifecycleState::Error) {
                Ok(()) => KernelError::ServiceStartup(SERVICE_CONFIGURATION),
                Err(transition_error) => transition_error,
            }
        })?;
        services.register(SERVICE_CONFIGURATION, ServiceStatus::Healthy);
        services.register(SERVICE_WORKSPACE, ServiceStatus::Healthy);

        state.transition(LifecycleState::Ready)?;

        Ok(InitializeWorkspaceResult {
            state,
            database: DatabaseServiceHandle::new(database),
            services,
        })
    }

    fn initialize_in_memory_database(services: &mut ServiceRegistry) -> Result<workspace_database::Database> {
        let database = workspace_database::Database::open_in_memory().map_err(|error| {
            services.register(SERVICE_DATABASE, ServiceStatus::Failed);
            error
        })?;
        let runner = workspace_database::MigrationRunner::load_from_dir(
            workspace_database::bundled_migrations_dir(),
        )?;
        runner.apply_all(&database).map_err(|error| {
            services.register(SERVICE_DATABASE, ServiceStatus::Failed);
            error
        })?;
        services.register(SERVICE_DATABASE, ServiceStatus::Healthy);
        Ok(database)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventBus;

    #[test]
    fn initializes_in_memory_workspace() {
        let bus = EventBus::new();
        let result = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        assert!(result.state.is_ready());
        assert!(result.services.all_healthy());
    }
}
