use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceWorkingStyleComparison, WorkspaceWorkingStyleState, WorkspaceWorkingStyleValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_working_style(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceWorkingStyleState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_working_style(
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
pub fn compare_workspace_working_styles(
    left: WorkspaceWorkingStyleState,
    right: WorkspaceWorkingStyleState,
) -> IpcResponse<WorkspaceWorkingStyleComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_working_styles(&left, &right))
}

#[tauri::command]
pub fn validate_workspace_working_style(
    state: WorkspaceWorkingStyleState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceWorkingStyleValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_working_style(
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
