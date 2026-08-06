use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{CommandHandler, WindowOperationResult, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Executes a Window Provider operation via Capability Runtime.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn execute_window_operation(
    operation: String,
    query: Option<String>,
    path: Option<String>,
    hwnd: Option<String>,
    pid: Option<u32>,
    x: Option<i32>,
    y: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    monitor_index: Option<i32>,
    snap: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WindowOperationResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::execute_window_operation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            operation,
            query,
            path,
            hwnd,
            pid,
            x,
            y,
            width,
            height,
            monitor_index,
            snap,
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
