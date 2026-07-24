use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{DecisionActionResult, DecisionItem, DecisionQueue};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_decision_queue(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionQueue> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_decision_queue(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(queue) => IpcResponse::success(queue),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn mark_decision_item_viewed(
    workspace_id: String,
    decision_item_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionItem> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::mark_decision_item_viewed(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            decision_item_id,
        ) {
            Ok(item) => IpcResponse::success(item),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn defer_decision_item(
    workspace_id: String,
    decision_item_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionItem> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::defer_decision_item(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            decision_item_id,
        ) {
            Ok(item) => IpcResponse::success(item),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn dismiss_decision_item(
    workspace_id: String,
    decision_item_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionItem> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::dismiss_decision_item(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            decision_item_id,
        ) {
            Ok(item) => IpcResponse::success(item),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn accept_decision_item(
    workspace_id: String,
    decision_item_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::accept_decision_item(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            decision_item_id,
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
pub fn reject_decision_item(
    workspace_id: String,
    decision_item_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DecisionActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reject_decision_item(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            decision_item_id,
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
