use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    AutomationContract, AutomationContractIntentRequest, AutomationTriggerKind,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

fn parse_trigger_kind(value: &str) -> Result<AutomationTriggerKind, CommandError> {
    AutomationTriggerKind::parse(value).map_err(|e| {
        CommandError::new("automation_contract_validation_error", e.to_string())
    })
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn create_automation_contract(
    workspace_id: String,
    project_id: String,
    task_id: Option<String>,
    name: String,
    description: Option<String>,
    trigger_kind: String,
    trigger_definition: Option<String>,
    intent_statement: String,
    required_capabilities: Vec<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    let kind = match parse_trigger_kind(&trigger_kind) {
        Ok(kind) => kind,
        Err(error) => return IpcResponse::failure(error),
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            project_id,
            task_id,
            name,
            description,
            kind,
            trigger_definition,
            intent_statement,
            required_capabilities,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_automation_contracts(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<AutomationContract>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_automation_contracts(
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
pub fn get_automation_contract(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn request_automation_contract_approval(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::request_automation_contract_approval(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn approve_automation_contract(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::approve_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn pause_automation_contract(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::pause_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn resume_automation_contract(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::resume_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
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
pub fn update_automation_contract(
    contract_id: String,
    name: Option<String>,
    description: Option<Option<String>>,
    intent_statement: Option<String>,
    trigger_kind: Option<String>,
    trigger_definition: Option<String>,
    required_capabilities: Option<Vec<String>>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    let kind = match trigger_kind {
        Some(value) => match parse_trigger_kind(&value) {
            Ok(kind) => Some(kind),
            Err(error) => return IpcResponse::failure(error),
        },
        None => None,
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
            name,
            description,
            intent_statement,
            kind,
            trigger_definition,
            required_capabilities,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn revoke_automation_contract(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContract> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::revoke_automation_contract(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
        ) {
            Ok(contract) => IpcResponse::success(contract),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn prepare_automation_contract_intent(
    contract_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AutomationContractIntentRequest> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::prepare_automation_contract_intent(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            contract_id,
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
