use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_kernel::{CapabilityIntent, CommandHandler, OperatorTurnResult, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Single Conversation → Kernel Operator execution entry (P12 Finalization).
#[tauri::command]
pub fn execute_capability_intent(
    intent: CapabilityIntent,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<OperatorTurnResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::execute_capability_intent(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            intent,
        ) {
            Ok(result) => IpcResponse::success(result),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
