use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::SuggestionLifecycleRecord;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns a derived, read-only stream of recent suggestion lifecycle records.
#[tauri::command]
pub fn get_suggestion_lifecycle(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<SuggestionLifecycleRecord>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_suggestion_lifecycle(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(records) => IpcResponse::success(records),
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
    fn ipc_suggestion_lifecycle_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        CommandHandler::create_workspace(&kernel, actor.clone(), intent.clone(), "IPC Lifecycle".into())
            .unwrap();

        let records =
            CommandHandler::get_suggestion_lifecycle(&kernel, actor, intent, Some(10)).unwrap();

        assert!(records.iter().all(|record| record.validate().is_ok()));
    }
}
