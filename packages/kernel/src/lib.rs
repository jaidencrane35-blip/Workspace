//! Workspace Platform Kernel — core runtime boundary.
//!
//! Owns application state, configuration, and service registration.
//! Business logic must not live in the React frontend (Sprint 02).

pub mod config;
pub mod error;
pub mod services;
pub mod state;

pub use config::{ConfigManager, SettingsUpdate, WorkspaceSettings};
pub use error::{KernelError, PublicError, Result};
pub use services::ServiceRegistry;
pub use state::{InitializationState, RuntimeStatus, WorkspaceState};

use std::path::Path;

use workspace_database::{Database, DatabaseService};

/// Kernel crate version aligned with application semver.
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Central runtime authority for Workspace.
pub struct WorkspaceKernel {
    state: WorkspaceState,
    database: Database,
    services: ServiceRegistry,
}

impl WorkspaceKernel {
    /// Initializes database, applies migrations, seeds defaults, and marks runtime ready.
    pub fn initialize(db_path: impl AsRef<Path>) -> Result<Self> {
        let mut state = WorkspaceState::new(KERNEL_VERSION);
        state.initialization = InitializationState::Initializing;

        let db_service = DatabaseService::initialize(db_path)?;
        let database = db_service.into_database();

        ConfigManager::ensure_defaults(&database)?;

        state.initialization = InitializationState::Ready;
        state.runtime_status = RuntimeStatus::Running;

        Ok(Self {
            state,
            database,
            services: ServiceRegistry::new(),
        })
    }

    /// Initializes with an existing in-memory database (tests).
    pub fn initialize_in_memory() -> Result<Self> {
        let mut state = WorkspaceState::new(KERNEL_VERSION);
        state.initialization = InitializationState::Initializing;

        let database = Database::open_in_memory()?;
        let runner = workspace_database::MigrationRunner::load_from_dir(
            workspace_database::bundled_migrations_dir(),
        )?;
        runner.apply_all(&database)?;
        ConfigManager::ensure_defaults(&database)?;

        state.initialization = InitializationState::Ready;
        state.runtime_status = RuntimeStatus::Running;

        Ok(Self {
            state,
            database,
            services: ServiceRegistry::new(),
        })
    }

    pub fn state(&self) -> &WorkspaceState {
        &self.state
    }

    pub fn services(&self) -> &ServiceRegistry {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut ServiceRegistry {
        &mut self.services
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn get_settings(&self) -> Result<WorkspaceSettings> {
        self.ensure_ready()?;
        ConfigManager::load(&self.database)
    }

    pub fn update_settings(&self, update: SettingsUpdate) -> Result<WorkspaceSettings> {
        self.ensure_ready()?;
        ConfigManager::update(&self.database, update)
    }

    fn ensure_ready(&self) -> Result<()> {
        if self.state.is_ready() {
            Ok(())
        } else {
            Err(KernelError::NotReady)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_initializes_in_memory() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        assert!(kernel.state().is_ready());
        assert_eq!(kernel.state().version, KERNEL_VERSION);
    }

    #[test]
    fn kernel_exposes_default_settings() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let settings = kernel.get_settings().unwrap();
        assert_eq!(settings.theme, "system");
        assert!(settings.first_run);
    }
}
