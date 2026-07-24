use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspacePatternState;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_pattern(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspacePatternState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_pattern(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
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
