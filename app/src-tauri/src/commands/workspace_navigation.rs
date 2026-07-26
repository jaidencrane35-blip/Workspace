use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceNavigationComparison, WorkspaceNavigationState, WorkspaceNavigationValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_navigation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceNavigationState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_navigation(
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

#[tauri::command]
pub fn compare_workspace_navigation(
    left: WorkspaceNavigationState,
    right: WorkspaceNavigationState,
) -> IpcResponse<WorkspaceNavigationComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_navigation(&left, &right))
}

#[tauri::command]
pub fn validate_workspace_navigation(
    state: WorkspaceNavigationState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceNavigationValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_navigation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            &state,
        ) {
            Ok(report) => IpcResponse::success(report),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
