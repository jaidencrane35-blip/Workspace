use std::path::Path;

use crate::commands::initialize::InitializeWorkspace;
use crate::commands::update_settings::UpdateSettings;
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceShutdown};
use crate::lifecycle::LifecycleState;
use crate::services::ConfigurationService;
use crate::WorkspaceKernel;

/// Executes kernel commands and coordinates services + events.
pub struct CommandHandler;

impl CommandHandler {
    pub fn initialize_workspace(
        kernel: &mut WorkspaceKernel,
        db_path: impl AsRef<Path>,
    ) -> Result<()> {
        let result = InitializeWorkspace::at_path(db_path).execute(kernel.event_bus())?;
        kernel.apply_runtime(result.state, result.database, result.services);
        Ok(())
    }

    pub fn initialize_workspace_in_memory(kernel: &mut WorkspaceKernel) -> Result<()> {
        let result = InitializeWorkspace::in_memory().execute(kernel.event_bus())?;
        kernel.apply_runtime(result.state, result.database, result.services);
        Ok(())
    }

    pub fn get_settings(kernel: &WorkspaceKernel) -> Result<WorkspaceSettings> {
        if !kernel.state().is_ready() {
            return Err(KernelError::NotReady);
        }
        ConfigurationService::load(kernel.database())
    }

    pub fn update_settings(
        kernel: &WorkspaceKernel,
        update: SettingsUpdate,
    ) -> Result<WorkspaceSettings> {
        UpdateSettings::new(update).execute(
            kernel.state(),
            kernel.database(),
            kernel.event_bus(),
        )
    }

    pub fn shutdown(kernel: &mut WorkspaceKernel) {
        log::info!("COMMAND: ShutdownWorkspace");
        kernel.transition_lifecycle(LifecycleState::ShuttingDown);
        kernel
            .event_bus()
            .publish(DomainEvent::WorkspaceShutdown(WorkspaceShutdown));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;

    #[test]
    fn command_handler_runs_initialize_and_update() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let settings = CommandHandler::update_settings(
            &kernel,
            SettingsUpdate {
                theme: Some("light".into()),
                first_run: Some(false),
            },
        )
        .unwrap();

        assert_eq!(settings.theme, "light");
        assert!(!settings.first_run);
    }
}
