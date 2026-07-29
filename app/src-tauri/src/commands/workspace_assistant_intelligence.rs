//! Programme IV Batches 11–16 — Assistant Intelligence IPC (compose / package / get / explain).
//! Presentation-only surfaces; no approve / execute / decide authority.

use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceAssistantContextExplanation, WorkspaceAssistantContextProjection,
    WorkspaceAssistantExplanationExplanation, WorkspaceAssistantExplanationProjection,
    WorkspaceAssistantInteractionExplanation, WorkspaceAssistantInteractionProjection,
    WorkspaceAssistantPersonalisationExplanation, WorkspaceAssistantPersonalisationProjection,
    WorkspaceAssistantRetrievalExplanation, WorkspaceAssistantRetrievalProjection,
    WorkspaceAssistantSurfaceExplanation, WorkspaceAssistantSurfaceProjection,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

fn unavailable<T>() -> IpcResponse<T> {
    IpcResponse::failure(CommandError::new(
        "internal_error",
        "Workspace core is temporarily unavailable.",
    ))
}

#[tauri::command]
pub fn compose_workspace_assistant_turn(
    workspace_id: String,
    human_ask: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantSurfaceProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::compose_workspace_assistant_turn(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            human_ask,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_surface(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantSurfaceProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_surface(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_surface(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantSurfaceExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_surface(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn package_workspace_assistant_context(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantContextProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::package_workspace_assistant_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_context(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantContextProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_context(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantContextExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn package_workspace_assistant_retrieval(
    workspace_id: String,
    human_ask: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantRetrievalProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::package_workspace_assistant_retrieval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            human_ask,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_retrieval(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantRetrievalProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_retrieval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_retrieval(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantRetrievalExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_retrieval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn package_workspace_assistant_explanation(
    workspace_id: String,
    human_ask: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantExplanationProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::package_workspace_assistant_explanation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            human_ask,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_explanation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantExplanationProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_explanation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_explanation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantExplanationExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_explanation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn package_workspace_assistant_interaction(
    workspace_id: String,
    human_ask: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantInteractionProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::package_workspace_assistant_interaction(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            human_ask,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_interaction(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantInteractionProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_interaction(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_interaction(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantInteractionExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_interaction(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn package_workspace_assistant_personalisation(
    workspace_id: String,
    human_ask: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantPersonalisationProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::package_workspace_assistant_personalisation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            human_ask,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn get_workspace_assistant_personalisation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantPersonalisationProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_assistant_personalisation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}

#[tauri::command]
pub fn explain_assistant_personalisation(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceAssistantPersonalisationExplanation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::explain_assistant_personalisation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(explanation) => IpcResponse::success(explanation),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => unavailable(),
    }
}
