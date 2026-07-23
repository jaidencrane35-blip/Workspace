use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::Observation;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns a derived, read-only stream of recent workspace observations.
#[tauri::command]
pub fn get_observations(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<Observation>> {
    match kernel.lock() {
        Ok(kernel) => {
            match CommandHandler::get_observations(
                &kernel,
                ipc_actor_context(),
                ipc_intent_context(),
                limit,
            ) {
                Ok(observations) => IpcResponse::success(observations),
                Err(error) => IpcResponse::failure(CommandError::from(error)),
            }
        }
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
    fn ipc_observations_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();
        CommandHandler::create_workspace(&kernel, actor.clone(), intent.clone(), "Observed".into())
            .unwrap();

        let observations =
            CommandHandler::get_observations(&kernel, actor, intent, Some(10)).unwrap();

        assert!(observations
            .iter()
            .any(|obs| obs.source_event_type == "workspace.entity.created"));
    }
}
