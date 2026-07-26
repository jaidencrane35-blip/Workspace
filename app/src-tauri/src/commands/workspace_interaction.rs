use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    InteractionSelectResult, WorkspaceInteractionComparison, WorkspaceInteractionState,
    WorkspaceInteractionValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_interactions(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceInteractionState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_interactions(
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
pub fn compare_workspace_interactions(
    left: WorkspaceInteractionState,
    right: WorkspaceInteractionState,
) -> IpcResponse<WorkspaceInteractionComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_interactions(&left, &right))
}

#[tauri::command]
pub fn validate_workspace_interactions(
    state: WorkspaceInteractionState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceInteractionValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_interactions(
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

#[tauri::command]
pub fn select_workspace_interaction(
    workspace_id: String,
    interaction_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<InteractionSelectResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::select_workspace_interaction(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            interaction_id,
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
