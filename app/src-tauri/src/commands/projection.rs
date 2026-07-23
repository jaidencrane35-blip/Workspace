use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceSnapshot;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn get_workspace_snapshot(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceSnapshot> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_snapshot(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(snapshot) => IpcResponse::success(snapshot),
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
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_snapshot_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let workspace = workspace_kernel::CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "IPC Snapshot".into(),
        )
        .unwrap();

        let snapshot = workspace_kernel::CommandHandler::get_workspace_snapshot(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
        )
        .unwrap();

        assert_eq!(snapshot.workspace_name, "IPC Snapshot");
    }
}
