use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{AiPlan, ModelProviderDescriptor, ModelResponse};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

const DIAGNOSTIC_AI_ACTOR_ID: &str = "diagnostic-ai";

#[tauri::command]
pub fn list_model_providers(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<ModelProviderDescriptor>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_model_providers(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(providers) => IpcResponse::success(providers),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_model_provider_metadata(
    provider_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ModelProviderDescriptor> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_model_provider_metadata(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            provider_id,
        ) {
            Ok(metadata) => IpcResponse::success(metadata),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn test_model_provider_request(
    task: Option<String>,
    application_ids: Option<Vec<String>>,
    preferred_provider_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ModelResponse> {
    let task = task.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::test_model_provider_request(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            task,
            application_ids.unwrap_or_default(),
            preferred_provider_id,
        ) {
            Ok(response) => IpcResponse::success(response),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: provider → AI proposals only (no execution / no gateway submission).
#[tauri::command]
pub fn diagnose_model_proposal_generation(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlan> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::diagnose_model_proposal_generation(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
