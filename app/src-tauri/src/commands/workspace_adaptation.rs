use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{AdaptationActionResult, WorkspaceAdaptationState};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_adaptation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAdaptationState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_adaptation(
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
pub fn review_adaptation_proposal(
    workspace_id: String,
    proposal_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AdaptationActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::review_adaptation_proposal(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            proposal_id,
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
pub fn accept_adaptation_proposal(
    workspace_id: String,
    proposal_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AdaptationActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::accept_adaptation_proposal(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            proposal_id,
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
pub fn reject_adaptation_proposal(
    workspace_id: String,
    proposal_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AdaptationActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reject_adaptation_proposal(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            proposal_id,
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
