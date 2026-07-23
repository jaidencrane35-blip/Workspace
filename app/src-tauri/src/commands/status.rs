use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;
use workspace_kernel::WorkspaceKernel;

use super::error::CommandError;

#[derive(Debug, Serialize)]
pub struct WorkspaceStatusResponse {
    pub status: String,
    pub version: String,
    pub initialization: String,
}

pub fn workspace_status_from_kernel(kernel: &WorkspaceKernel) -> WorkspaceStatusResponse {
    let state = kernel.state();
    WorkspaceStatusResponse {
        status: state.runtime_status.as_str().to_string(),
        version: state.version.clone(),
        initialization: state.initialization.as_str().to_string(),
    }
}

/// Returns authoritative runtime status from the Platform Kernel.
#[tauri::command]
pub fn get_workspace_status(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> Result<WorkspaceStatusResponse, CommandError> {
    let kernel = kernel
        .lock()
        .map_err(|_| CommandError::new("internal_error", "Workspace core is temporarily unavailable."))?;

    Ok(workspace_status_from_kernel(&kernel))
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::{InitializationState, RuntimeStatus};

    #[test]
    fn maps_kernel_state_to_response() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let response = workspace_status_from_kernel(&kernel);

        assert_eq!(response.status, RuntimeStatus::Running.as_str());
        assert_eq!(response.initialization, InitializationState::Ready.as_str());
        assert!(!response.version.is_empty());
    }
}
