use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{RecommendationReviewActionResult, WorkspaceRecommendationEngineState};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_workspace_recommendation_engine(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceRecommendationEngineState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_recommendation_engine(
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
pub fn present_recommendation(
    workspace_id: String,
    recommendation_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::present_recommendation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
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
pub fn accept_recommendation(
    workspace_id: String,
    recommendation_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::accept_recommendation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
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
pub fn reject_recommendation(
    workspace_id: String,
    recommendation_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reject_recommendation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
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
pub fn confirm_recommendation_decision(
    workspace_id: String,
    recommendation_id: String,
    confirmation_intent: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::confirm_recommendation_decision(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
            confirmation_intent,
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
pub fn decline_recommendation_decision(
    workspace_id: String,
    recommendation_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::decline_recommendation_decision(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
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
pub fn revoke_recommendation_adapter_preparation(
    workspace_id: String,
    recommendation_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<RecommendationReviewActionResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::revoke_recommendation_adapter_preparation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            recommendation_id,
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
