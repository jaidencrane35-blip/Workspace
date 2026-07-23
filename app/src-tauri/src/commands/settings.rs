use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{CommandHandler, SettingsUpdate, WorkspaceKernel, WorkspaceSettings};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Reads persisted application settings via the kernel command layer.
#[tauri::command]
pub fn get_settings(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceSettings> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_settings(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(settings) => IpcResponse::success(settings),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Updates persisted application settings via the kernel command layer.
#[tauri::command]
pub fn update_settings(
    update: SettingsUpdate,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceSettings> {
    match kernel.lock() {
        Ok(kernel) => {
            match CommandHandler::update_settings(
                &kernel,
                ipc_actor_context(),
                ipc_intent_context(),
                update,
            ) {
                Ok(settings) => IpcResponse::success(settings),
                Err(error) => IpcResponse::failure(CommandError::from(error)),
            }
        }
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_settings_path_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let initial = CommandHandler::get_settings(&kernel, actor.clone(), intent.clone()).unwrap();
        assert_eq!(initial.theme, "system");

        let updated = CommandHandler::update_settings(
            &kernel,
            actor,
            intent,
            SettingsUpdate {
                theme: Some("light".into()),
                first_run: Some(false),
            },
        )
        .unwrap();

        assert_eq!(updated.theme, "light");
        assert!(!updated.first_run);
    }
}
