use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{DesktopArrangement, DesktopArrangementRestoreResult};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn capture_desktop_arrangement(
    workspace_id: String,
    name: String,
    description: Option<String>,
    arrangement_id: Option<String>,
    refresh_observation: Option<bool>,
    member_hwnds: Option<Vec<String>>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DesktopArrangement> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::capture_desktop_arrangement(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            description,
            arrangement_id,
            refresh_observation,
            member_hwnds,
        ) {
            Ok(arrangement) => IpcResponse::success(arrangement),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn restore_desktop_arrangement(
    arrangement_id: String,
    focus_first: Option<bool>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<DesktopArrangementRestoreResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::restore_desktop_arrangement(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            arrangement_id,
            focus_first,
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
pub fn focus_desktop_window(
    hwnd: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<workspace_domain::DesktopWindowFocusResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::focus_desktop_window(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            hwnd,
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
pub fn get_desktop_arrangement(
    arrangement_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Option<DesktopArrangement>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_desktop_arrangement(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            arrangement_id,
        ) {
            Ok(arrangement) => IpcResponse::success(arrangement),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_desktop_arrangements(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<DesktopArrangement>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_desktop_arrangements(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(arrangements) => IpcResponse::success(arrangements),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
