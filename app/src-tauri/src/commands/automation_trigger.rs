use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    AutomationIntentProposal, AutomationIntentProposalStatus, TriggerEvaluationResult,
    TriggerEvent, TriggerEventType,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

fn parse_event_type(value: &str) -> Result<TriggerEventType, CommandError> {
    TriggerEventType::parse(value).map_err(|e| {
        CommandError::new("automation_trigger_validation_error", e.to_string())
    })
}

fn parse_proposal_status(
    value: Option<String>,
) -> Result<Option<AutomationIntentProposalStatus>, CommandError> {
    match value {
        None => Ok(None),
        Some(raw) => AutomationIntentProposalStatus::parse(&raw)
            .map(Some)
            .map_err(|e| CommandError::new("automation_trigger_validation_error", e.to_string())),
    }
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn record_trigger_event(
    workspace_id: String,
    event_type: String,
    source: String,
    context: String,
    project_id: Option<String>,
    task_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TriggerEvent> {
    let event_type = match parse_event_type(&event_type) {
        Ok(value) => value,
        Err(error) => return IpcResponse::failure(error),
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::record_trigger_event(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
        ) {
            Ok(event) => IpcResponse::success(event),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn evaluate_triggers(
    trigger_event_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TriggerEvaluationResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::evaluate_triggers(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            trigger_event_id,
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

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn record_and_evaluate_triggers(
    workspace_id: String,
    event_type: String,
    source: String,
    context: String,
    project_id: Option<String>,
    task_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TriggerEvaluationResult> {
    let event_type = match parse_event_type(&event_type) {
        Ok(value) => value,
        Err(error) => return IpcResponse::failure(error),
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::record_and_evaluate_triggers(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
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

#[tauri::command]
pub fn list_trigger_events(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<TriggerEvent>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_trigger_events(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_automation_intent_proposals(
    workspace_id: String,
    status: Option<String>,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<AutomationIntentProposal>> {
    let status = match parse_proposal_status(status) {
        Ok(value) => value,
        Err(error) => return IpcResponse::failure(error),
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_automation_intent_proposals(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            status,
            limit,
        ) {
            Ok(items) => IpcResponse::success(items),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn accept_automation_intent_proposal(
    proposal_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationIntentProposal> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::accept_automation_intent_proposal(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            proposal_id,
        ) {
            Ok(proposal) => IpcResponse::success(proposal),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn reject_automation_intent_proposal(
    proposal_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationIntentProposal> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reject_automation_intent_proposal(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            proposal_id,
        ) {
            Ok(proposal) => IpcResponse::success(proposal),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
