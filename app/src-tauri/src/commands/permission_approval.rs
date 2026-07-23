use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    AiPlanSubmissionResult, ApplicationLaunchResult, ApprovalDecisionResult,
    PermissionApprovalRequest,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

const DIAGNOSTIC_AI_ACTOR_ID: &str = "diagnostic-ai";

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

/// Diagnostic AI simulation: AI actor proposes launch through the governed path.
///
/// Expected default outcome: ApprovalRequired (AI has zero capabilities).
/// Not a chatbot — architecture validation only.
#[tauri::command]
pub fn request_ai_application_launch(
    id: String,
    reason: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationLaunchResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::submit_ai_application_launch(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            id,
            reason.or_else(|| {
                Some("Diagnostic AI simulation — propose launch for approval".into())
            }),
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

/// Diagnostic AI planning: goal → proposals → governed submissions (no auto-retry).
///
/// Default: each proposal hits ApprovalRequired. Not a chatbot or autonomous agent.
#[tauri::command]
pub fn diagnose_ai_workspace_plan(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlanSubmissionResult> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::submit_ai_plan(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
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
