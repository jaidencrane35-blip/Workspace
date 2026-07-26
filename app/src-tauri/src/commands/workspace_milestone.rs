use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceMilestoneComparison, WorkspaceMilestoneState, WorkspaceMilestoneValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_milestones(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceMilestoneState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_milestones(
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
pub fn compare_workspace_milestones(
    left: WorkspaceMilestoneState,
    right: WorkspaceMilestoneState,
) -> IpcResponse<WorkspaceMilestoneComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_milestones(&left, &right))
}

#[tauri::command]
pub fn validate_workspace_milestones(
    state: WorkspaceMilestoneState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceMilestoneValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_milestones(
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
