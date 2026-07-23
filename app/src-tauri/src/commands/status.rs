use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;
use workspace_kernel::WorkspaceKernel;

use super::error::CommandError;
use super::response::IpcResponse;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct WorkspaceStatusResponse {
    pub status: String,
    pub version: String,
    pub initialized: bool,
}

pub fn workspace_status_from_kernel(kernel: &WorkspaceKernel) -> WorkspaceStatusResponse {
    let state = kernel.state();
    WorkspaceStatusResponse {
        status: state.lifecycle.as_str().to_string(),
        version: state.version.clone(),
        initialized: state.lifecycle.is_initialized(),
    }
}

/// Returns authoritative runtime status from the Platform Kernel.
#[tauri::command]
pub fn get_workspace_status(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceStatusResponse> {
    match kernel.lock() {
        Ok(kernel) => IpcResponse::success(workspace_status_from_kernel(&kernel)),
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::{LifecycleState, WorkspaceKernel};

    #[test]
    fn maps_kernel_state_to_response() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let response = workspace_status_from_kernel(&kernel);

        assert_eq!(response.status, LifecycleState::Ready.as_str());
        assert!(response.initialized);
        assert!(!response.version.is_empty());
    }
}
