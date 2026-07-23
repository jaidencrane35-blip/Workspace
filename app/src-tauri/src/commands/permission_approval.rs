use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    Actor, ActorContext, ApplicationLaunchResult, ApprovalDecisionResult, IntentContext,
    PermissionApprovalRequest,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

fn diagnostic_ai_actor() -> ActorContext {
    ActorContext::new(Actor::ai_assistant("diagnostic-ai").expect("diagnostic-ai id is valid"))
}

#[tauri::command]
pub fn get_permission_approvals(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<PermissionApprovalRequest>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_permission_approvals(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn decide_approval(
    request_id: String,
    decision: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApprovalDecisionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::decide_approval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            request_id,
            decision,
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

/// Diagnostic helper: attempts launch as a non-human AI actor to exercise ApprovalRequired.
#[tauri::command]
pub fn request_ai_application_launch(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationLaunchResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::launch_application(
            &kernel,
            diagnostic_ai_actor(),
            IntentContext::user_request(),
            id,
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
