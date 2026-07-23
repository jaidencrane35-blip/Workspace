use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::Workspace;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::ipc_actor_context;
use super::error::CommandError;
use super::response::IpcResponse;

/// Creates a workspace through the kernel command layer.
#[tauri::command]
pub fn create_workspace(
    name: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Workspace> {
    match kernel.lock() {
        Ok(kernel) => {
            match CommandHandler::create_workspace(&kernel, ipc_actor_context(), name) {
                Ok(workspace) => IpcResponse::success(workspace),
                Err(error) => IpcResponse::failure(CommandError::from(error)),
            }
        }
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Retrieves a workspace by id through the kernel command layer.
#[tauri::command]
pub fn get_workspace(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Workspace> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace(&kernel, ipc_actor_context(), id) {
            Ok(workspace) => IpcResponse::success(workspace),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{ActorType, LOCAL_USER_ACTOR_ID};
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_workspace_mutation_uses_command_layer_with_local_user() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        assert_eq!(actor.actor.actor_type, ActorType::LocalUser);
        assert_eq!(actor.actor.id.as_str(), LOCAL_USER_ACTOR_ID);

        let created =
            CommandHandler::create_workspace(&kernel, actor.clone(), "IPC Workspace".into())
                .unwrap();
        let loaded = CommandHandler::get_workspace(
            &kernel,
            actor,
            created.id.to_string(),
        )
        .unwrap();
        assert_eq!(loaded.name, "IPC Workspace");
    }
}
