use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::Suggestion;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns deterministic, read-only proposals derived from the workspace
/// context. Suggestions are proposals only — never actions.
#[tauri::command]
pub fn get_suggestions(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<Suggestion>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_suggestions(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(suggestions) => IpcResponse::success(suggestions),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Records explicit user approval of a proposal (decision only).
#[tauri::command]
pub fn accept_suggestion(
    workspace_id: String,
    suggestion_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Suggestion> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::accept_suggestion(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            suggestion_id,
        ) {
            Ok(suggestion) => IpcResponse::success(suggestion),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Records explicit user dismissal of a proposal (decision only).
#[tauri::command]
pub fn reject_suggestion(
    workspace_id: String,
    suggestion_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Suggestion> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reject_suggestion(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            suggestion_id,
        ) {
            Ok(suggestion) => IpcResponse::success(suggestion),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_suggestions_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "IPC Suggest".into(),
        )
        .unwrap();

        // Should return a well-formed (possibly empty) proposal list.
        let suggestions = CommandHandler::get_suggestions(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();

        assert!(suggestions.iter().all(|s| s.validate().is_ok()));
    }
}
