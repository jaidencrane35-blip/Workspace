use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceState;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns the canonical WorkspaceState projection (observation + delta).
#[tauri::command]
pub fn get_workspace_state(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_state(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(state) => IpcResponse::success(state),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
