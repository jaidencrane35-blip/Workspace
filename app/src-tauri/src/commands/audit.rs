use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::AuditEvent;
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::ipc_actor_context;
use super::error::CommandError;
use super::response::IpcResponse;

/// Returns recent audit history through the kernel query layer.
#[tauri::command]
pub fn get_audit_history(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<AuditEvent>> {
    match kernel.lock() {
        Ok(kernel) => {
            match CommandHandler::get_audit_history(&kernel, ipc_actor_context(), limit) {
                Ok(history) => IpcResponse::success(history),
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
    use workspace_domain::ActorType;
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_audit_query_returns_envelope_with_local_user_attribution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        CommandHandler::create_workspace(&kernel, actor.clone(), "Audited".into()).unwrap();

        let response = match CommandHandler::get_audit_history(&kernel, actor, Some(10)) {
            Ok(history) => IpcResponse::success(history),
            Err(error) => IpcResponse::failure(super::CommandError::from(error)),
        };

        assert!(response.success);
        assert!(response.data.as_ref().unwrap().iter().any(|record| {
            record.command_name.as_deref() == Some("CreateWorkspace")
                && record.actor_type == ActorType::LocalUser
        }));
    }
}
