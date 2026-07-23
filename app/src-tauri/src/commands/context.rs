use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceContext;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns a deterministic, read-only composition of the derived workspace read
/// layers (state, activity, metrics, capabilities).
#[tauri::command]
pub fn get_workspace_context(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceContext> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
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

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_context_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "IPC Context".into(),
        )
        .unwrap();

        let context = CommandHandler::get_workspace_context(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();

        assert_eq!(context.snapshot.workspace_name, "IPC Context");
        assert_eq!(context.workspace, context.snapshot.workspace);
        assert!(context.validate().is_ok());
    }
}
