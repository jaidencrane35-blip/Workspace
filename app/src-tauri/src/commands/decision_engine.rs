use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{DecisionEngineActionResult, DecisionEngineState};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_decision_engine(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionEngineState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_decision_engine(
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
pub fn select_decision_candidate(
    workspace_id: String,
    candidate_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionEngineActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::select_decision_candidate(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            candidate_id,
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
pub fn dismiss_decision_candidate(
    workspace_id: String,
    candidate_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionEngineActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::dismiss_decision_candidate(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            candidate_id,
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
pub fn postpone_decision_candidate(
    workspace_id: String,
    candidate_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionEngineActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::postpone_decision_candidate(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            candidate_id,
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
