//! Execution-intelligence IPC (Sprints 23–30): intent bridge, governed execute,
//! outcomes, reconciliation reads, and cancellation request.

use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    CancellationRequest, ExecutionLifecycleProjection, ExecutionOutcome, ExecutionReconciliation,
    IntentExecutionRequest, SuggestionIntentRequest,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

/// Approval-gated suggestion → intent bridge (Sprint 23). Does not execute.
#[tauri::command]
pub fn create_suggestion_intent_request(
    workspace_id: String,
    suggestion_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<SuggestionIntentRequest> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_suggestion_intent_request(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            suggestion_id,
        ) {
            Ok(request) => IpcResponse::success(request),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Governed intent execution (Sprint 24; Sprint 27 guard inside).
#[tauri::command]
pub fn execute_intent_request(
    workspace_id: String,
    suggestion_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<IntentExecutionRequest> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::execute_intent_request(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            suggestion_id,
        ) {
            Ok(request) => IpcResponse::success(request),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Audit-derived execution outcome list (Sprint 25).
#[tauri::command]
pub fn get_execution_outcomes(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<ExecutionOutcome>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_execution_outcomes(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(outcomes) => IpcResponse::success(outcomes),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Single-id execution state reconciliation (Sprint 29).
#[tauri::command]
pub fn get_execution_state(
    execution_request_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ExecutionReconciliation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_execution_state(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            execution_request_id,
        ) {
            Ok(state) => IpcResponse::success(state),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Dual-channel execution lifecycle projection (actionable + history).
#[tauri::command]
pub fn get_execution_states(
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ExecutionLifecycleProjection> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_execution_states(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            limit,
        ) {
            Ok(projection) => IpcResponse::success(projection),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Request cancellation of an execution (Sprint 28 — request boundary only).
#[tauri::command]
pub fn request_execution_cancellation(
    workspace_id: String,
    execution_request_id: String,
    reason: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<CancellationRequest> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::request_execution_cancellation(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            execution_request_id,
            reason,
        ) {
            Ok(request) => IpcResponse::success(request),
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
    fn ipc_execution_reads_use_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let outcomes =
            CommandHandler::get_execution_outcomes(&kernel, actor.clone(), intent.clone(), Some(10))
                .unwrap();
        assert!(outcomes.iter().all(|o| o.validate().is_ok()));

        let projection =
            CommandHandler::get_execution_states(&kernel, actor, intent, Some(10)).unwrap();
        assert!(projection.history.iter().all(|h| h.is_non_actionable()));
        assert_eq!(projection.authority_effect, "none");
    }
}
