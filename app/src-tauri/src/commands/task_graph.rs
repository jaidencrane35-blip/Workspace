use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    TaskGraph, TaskRelationship, TaskRelationshipKind, WorkspaceTask, WorkspaceTaskPriority,
    WorkspaceTaskStatus,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn generate_task_graph(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TaskGraph> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_task_graph(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(graph) => IpcResponse::success(graph),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn create_workspace_task(
    workspace_id: String,
    title: String,
    project_id: Option<String>,
    priority: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceTask> {
    let priority = priority
        .as_deref()
        .and_then(|p| WorkspaceTaskPriority::parse(p).ok())
        .unwrap_or(WorkspaceTaskPriority::Medium);
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_workspace_task(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            title,
            project_id,
            priority,
        ) {
            Ok(task) => IpcResponse::success(task),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn update_workspace_task_status(
    task_id: String,
    status: String,
    explanation: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceTask> {
    let status = match WorkspaceTaskStatus::parse(&status) {
        Ok(s) => s,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "task_graph_validation_error",
                &error.to_string(),
            ))
        }
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_workspace_task_status(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            task_id,
            status,
            explanation,
        ) {
            Ok(task) => IpcResponse::success(task),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn add_task_relationship(
    workspace_id: String,
    from_task_id: String,
    to_task_id: String,
    kind: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TaskRelationship> {
    let kind = match TaskRelationshipKind::parse(&kind) {
        Ok(k) => k,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "task_graph_validation_error",
                &error.to_string(),
            ))
        }
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::add_task_relationship(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            from_task_id,
            to_task_id,
            kind,
        ) {
            Ok(rel) => IpcResponse::success(rel),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn validate_task_graph(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<TaskGraph> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_task_graph(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(graph) => IpcResponse::success(graph),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_task_graph_planning_inputs(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<WorkspaceTask>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_task_graph_planning_inputs(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(tasks) => IpcResponse::success(tasks),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
