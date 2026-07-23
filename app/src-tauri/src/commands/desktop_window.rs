use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{CommandHandler, WorkspaceKernel};
use workspace_windows_integration::DesktopWindowSnapshot;

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns a bounded list of top-level desktop windows (observation only).
#[tauri::command]
pub fn get_desktop_windows(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<DesktopWindowSnapshot>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_desktop_windows(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(windows) => IpcResponse::success(windows),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
