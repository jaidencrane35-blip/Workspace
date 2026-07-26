use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceTransitionComparison, WorkspaceTransitionState, WorkspaceTransitionValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_transitions(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceTransitionState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_transitions(
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
pub fn compare_workspace_transitions(
    left: WorkspaceTransitionState,
    right: WorkspaceTransitionState,
) -> IpcResponse<WorkspaceTransitionComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_transitions(&left, &right))
}

#[tauri::command]
pub fn validate_workspace_transitions(
    state: WorkspaceTransitionState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceTransitionValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_transitions(
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
