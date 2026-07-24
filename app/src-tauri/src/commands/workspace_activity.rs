use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{WorkspaceActivity, WorkspaceActivityGraph};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_activity_graph(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceActivityGraph> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_activity_graph(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(graph) => IpcResponse::success(graph),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_workspace_activity_timeline(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<WorkspaceActivity>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_activity_timeline(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
