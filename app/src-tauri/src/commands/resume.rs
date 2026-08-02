use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{ActionOperationResult, ActionPlan, SavedContext};
use workspace_kernel::{CommandHandler, ResumePlanPreview, WorkspaceKernel};

use super::error::CommandError;
use super::response::IpcResponse;
use crate::actor::{ipc_actor_context, ipc_intent_context};

#[tauri::command]
pub fn list_saved_contexts(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<SavedContext>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_saved_contexts(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(list) => IpcResponse::success(list),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_saved_context(
    saved_context_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<SavedContext> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_saved_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            saved_context_id,
        ) {
            Ok(context) => IpcResponse::success(context),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn delete_saved_context(
    saved_context_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<()> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_saved_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            saved_context_id,
        ) {
            Ok(()) => IpcResponse::success(()),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn resolve_resume_plan(
    saved_context_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ResumePlanPreview> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::resolve_resume_plan(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            saved_context_id,
        ) {
            Ok(preview) => IpcResponse::success(preview),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn execute_resume_plan(
    plan: ActionPlan,
    approved_plan_digest: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ActionOperationResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::execute_resume_plan(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            plan,
            approved_plan_digest,
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
