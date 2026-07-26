use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceRuntimeOperatorView;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_runtime_overview(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceRuntimeOperatorView> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_runtime_overview(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(view) => IpcResponse::success(view),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
