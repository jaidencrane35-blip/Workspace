use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{ApplicationOperationResult, CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Executes an Application Provider operation via Capability Runtime.
#[tauri::command]
pub fn execute_application_operation(
    operation: String,
    query: Option<String>,
    path: Option<String>,
    hwnd: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationOperationResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::execute_application_operation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            operation,
            query,
            path,
            hwnd,
        ) {
            Ok(result) => IpcResponse::success(result),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
