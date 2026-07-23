use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{ActionCatalog, CapabilityDiscovery};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn get_actor_capabilities(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<CapabilityDiscovery> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_actor_capabilities(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(discovery) => IpcResponse::success(discovery),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: informational action catalog (existence ≠ authorization).
#[tauri::command]
pub fn get_action_catalog(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ActionCatalog> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_action_catalog(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(catalog) => IpcResponse::success(catalog),
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
    fn ipc_capability_discovery_uses_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let discovery = workspace_kernel::CommandHandler::get_actor_capabilities(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        )
        .unwrap();

        assert!(!discovery.capabilities.is_empty());
    }
}
