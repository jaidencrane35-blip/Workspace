use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{AiMemoryAwareness, AiPlan, MemoryEntry, MemoryType};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

const DIAGNOSTIC_AI_ACTOR_ID: &str = "diagnostic-ai";

#[tauri::command]
pub fn create_memory_entry(
    memory_type: String,
    key: String,
    summary: String,
    source: Option<String>,
    workspace_id: Option<String>,
    attributes: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<MemoryEntry> {
    let memory_type = match MemoryType::parse(&memory_type) {
        Ok(value) => value,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "ai_memory_validation_error",
                error.to_string(),
            ));
        }
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_memory_entry(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            memory_type,
            key,
            summary,
            source.unwrap_or_else(|| "operator_console".into()),
            workspace_id,
            attributes,
        ) {
            Ok(entry) => IpcResponse::success(entry),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_memory_entries(
    workspace_id: Option<String>,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<MemoryEntry>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_memory_entries(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(entries) => IpcResponse::success(entries),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_memory_context(
    workspace_id: Option<String>,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiMemoryAwareness> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_memory_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(context) => IpcResponse::success(context),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn delete_memory_entry(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<MemoryEntry> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_memory_entry(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
        ) {
            Ok(entry) => IpcResponse::success(entry),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn clear_memory_entries(
    memory_type: Option<String>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<usize> {
    let memory_type = match memory_type {
        None => None,
        Some(value) => match MemoryType::parse(&value) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                return IpcResponse::failure(CommandError::new(
                    "ai_memory_validation_error",
                    error.to_string(),
                ));
            }
        },
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::clear_memory_entries(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            memory_type,
            workspace_id,
        ) {
            Ok(count) => IpcResponse::success(count),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

/// Diagnostic: generate a memory-aware plan without submission/execution.
#[tauri::command]
pub fn diagnose_ai_plan_preview(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlan> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::diagnose_ai_plan_preview(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
        ) {
            Ok(plan) => IpcResponse::success(plan),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
