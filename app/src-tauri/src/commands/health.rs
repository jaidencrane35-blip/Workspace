use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::WorkspaceHealth;
use workspace_kernel::WorkspaceKernel;

use super::error::CommandError;
use super::response::IpcResponse;

/// Returns runtime health information from the Platform Kernel.
#[tauri::command]
pub fn get_workspace_health(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceHealth> {
    match kernel.lock() {
        Ok(kernel) => IpcResponse::success(kernel.health()),
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
    fn health_command_returns_success_envelope() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let response = match kernel.health() {
            health => IpcResponse::success(health),
        };

        assert!(response.success);
        assert_eq!(response.data.as_ref().unwrap().status, "ready");
    }
}
