use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{ClipboardReadResult, ClipboardWriteResult, CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Reads clipboard text via Capability Runtime → ClipboardProvider.
#[tauri::command]
pub fn read_clipboard(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ClipboardReadResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::read_clipboard(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
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

/// Writes clipboard text via Capability Runtime → ClipboardProvider.
#[tauri::command]
pub fn write_clipboard(
    text: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ClipboardWriteResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::write_clipboard(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            text,
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
