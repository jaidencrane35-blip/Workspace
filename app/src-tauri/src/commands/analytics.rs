use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::WorkspaceMetrics;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns deterministic, derived analytics over recent workspace observations.
#[tauri::command]
pub fn get_workspace_metrics(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceMetrics> {
    match kernel.lock() {
        Ok(kernel) => {
            match CommandHandler::get_workspace_metrics(
                &kernel,
                ipc_actor_context(),
                ipc_intent_context(),
                limit,
            ) {
                Ok(metrics) => IpcResponse::success(metrics),
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
    fn ipc_metrics_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();
        CommandHandler::create_workspace(&kernel, actor.clone(), intent.clone(), "Analyzed".into())
            .unwrap();

        let metrics =
            CommandHandler::get_workspace_metrics(&kernel, actor, intent, Some(200)).unwrap();

        assert!(metrics.observation_count > 0);
        assert!(metrics.validate().is_ok());
    }
}
