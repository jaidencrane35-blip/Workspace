use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceObservationSnapshot;
use workspace_kernel::{CommandHandler, WorkspaceKernel};
use workspace_kernel::services::WorkspaceObservationCaptureResult;

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn capture_workspace_observation(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceObservationCaptureResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::capture_workspace_observation(
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

#[tauri::command]
pub fn get_latest_workspace_observation(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Option<WorkspaceObservationSnapshot>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_latest_workspace_observation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
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

#[tauri::command]
pub fn get_workspace_observation_by_id(
    pass_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Option<WorkspaceObservationSnapshot>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_observation_by_id(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            pass_id,
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
