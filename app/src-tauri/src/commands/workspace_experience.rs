use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{WorkspaceExperienceComparison, WorkspaceExperienceState};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_experience(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceExperienceState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_experience(
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
pub fn compare_workspace_experiences(
    left: WorkspaceExperienceState,
    right: WorkspaceExperienceState,
) -> IpcResponse<WorkspaceExperienceComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_experiences(&left, &right))
}
