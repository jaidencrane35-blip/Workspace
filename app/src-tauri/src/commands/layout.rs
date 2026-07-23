use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{Layout, LayoutMetadata, LayoutNode, LayoutSnapshot, Viewport};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn create_layout(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Layout> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_layout(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(layout) => IpcResponse::success(layout),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn update_layout(
    layout_id: String,
    viewport: Viewport,
    nodes: Vec<LayoutNode>,
    metadata: Option<LayoutMetadata>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Layout> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_layout(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            layout_id,
            viewport,
            nodes,
            metadata,
        ) {
            Ok(layout) => IpcResponse::success(layout),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn delete_layout(
    layout_id: String,
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<()> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_layout(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            layout_id,
            workspace_id,
        ) {
            Ok(()) => IpcResponse::success(()),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn reset_layout(
    layout_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Layout> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::reset_layout(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            layout_id,
        ) {
            Ok(layout) => IpcResponse::success(layout),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_layout(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Layout> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_layout(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
        ) {
            Ok(layout) => IpcResponse::success(layout),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_layout_snapshot(
    layout_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<LayoutSnapshot> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_layout_snapshot(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            layout_id,
        ) {
            Ok(snapshot) => IpcResponse::success(snapshot),
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
    fn ipc_layout_commands_use_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();

        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Layout IPC".into(),
        )
        .unwrap();

        let layout = CommandHandler::create_layout(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
        )
        .unwrap();

        let loaded = CommandHandler::get_layout(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
        )
        .unwrap();

        assert_eq!(loaded.id, layout.id);
    }
}
