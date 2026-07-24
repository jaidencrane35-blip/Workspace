use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    Project, Task, TaskPriority, TaskStatus, WorkflowContext, WorkspaceIntelligenceComparison,
    WorkspaceIntelligenceState,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn create_project(
    workspace_id: String,
    name: String,
    description: Option<String>,
    metadata: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Project> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_project(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            description,
            metadata,
        ) {
            Ok(project) => IpcResponse::success(project),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_projects(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<Project>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_projects(
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
pub fn get_project(
    project_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Project> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_project(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            project_id,
        ) {
            Ok(project) => IpcResponse::success(project),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn create_task(
    project_id: String,
    workspace_id: String,
    title: String,
    priority: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Task> {
    let priority = match priority.as_deref().unwrap_or("medium") {
        "low" => TaskPriority::Low,
        "high" => TaskPriority::High,
        _ => TaskPriority::Medium,
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_task(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            project_id,
            workspace_id,
            title,
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
pub fn list_tasks(
    workspace_id: String,
    project_id: Option<String>,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<Task>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_tasks(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            project_id,
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
pub fn get_task(
    task_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Task> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_task(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            task_id,
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
pub fn update_task_status(
    task_id: String,
    status: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Task> {
    let status = match TaskStatus::parse(&status) {
        Ok(status) => status,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "workspace_intent_validation_error",
                &error.to_string(),
            ));
        }
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_task(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            task_id,
            None,
            Some(status),
            None,
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
pub fn set_active_work(
    workspace_id: String,
    project_id: Option<String>,
    task_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkflowContext> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::set_active_work(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            project_id,
            task_id,
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
pub fn get_workflow_context(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkflowContext> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workflow_context(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
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
pub fn generate_workspace_intelligence(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceIntelligenceState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_intelligence(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
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

#[tauri::command]
pub fn compare_workspace_intelligence_states(
    left_workspace_id: String,
    right_workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceIntelligenceComparison> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::compare_workspace_intelligence_states(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            left_workspace_id,
            right_workspace_id,
        ) {
            Ok(comparison) => IpcResponse::success(comparison),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
