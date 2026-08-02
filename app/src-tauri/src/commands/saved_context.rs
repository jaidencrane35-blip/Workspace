use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{SavedContext, SavedContextCaptureScope};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use super::error::CommandError;
use super::response::IpcResponse;
use crate::actor::{ipc_actor_context, ipc_intent_context};

/// Returns what saving a context would capture, so the interface can show the
/// user the same scope the kernel will honour. Observes nothing.
#[tauri::command]
pub fn get_saved_context_capture_scope(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<SavedContextCaptureScope> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_saved_context_capture_scope(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(scope) => IpcResponse::success(scope),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Captures the desktop once and stores it as a named bounded context.
///
/// `approved_scope` is the capture scope the user confirmed. The kernel refuses
/// the save if it is not the scope this build would actually capture.
#[tauri::command]
pub fn save_workspace_context(
    workspace_id: String,
    name: String,
    approved_scope: String,
    handoff_note: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<SavedContext> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::save_workspace_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            approved_scope,
            handoff_note,
        ) {
            Ok(saved) => IpcResponse::success(saved),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
