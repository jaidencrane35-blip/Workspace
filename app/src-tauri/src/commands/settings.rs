use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{SettingsUpdate, WorkspaceKernel, WorkspaceSettings};

use super::error::CommandError;

/// Reads persisted application settings via the Platform Kernel.
#[tauri::command]
pub fn get_settings(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> Result<WorkspaceSettings, CommandError> {
    let kernel = kernel
        .lock()
        .map_err(|_| CommandError::new("internal_error", "Workspace core is temporarily unavailable."))?;

    kernel.get_settings().map_err(CommandError::from)
}

/// Updates persisted application settings via the Platform Kernel.
#[tauri::command]
pub fn update_settings(
    update: SettingsUpdate,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> Result<WorkspaceSettings, CommandError> {
    let kernel = kernel
        .lock()
        .map_err(|_| CommandError::new("internal_error", "Workspace core is temporarily unavailable."))?;

    kernel.update_settings(update).map_err(CommandError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn settings_flow_through_kernel() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let initial = kernel.get_settings().unwrap();
        assert_eq!(initial.theme, "system");

        let updated = kernel
            .update_settings(SettingsUpdate {
                theme: Some("light".into()),
                first_run: Some(false),
            })
            .unwrap();

        assert_eq!(updated.theme, "light");
        assert!(!updated.first_run);

        let reloaded = kernel.get_settings().unwrap();
        assert_eq!(reloaded, updated);
    }
}
