use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{ApplicationReference, WidgetReference, Zone};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn create_zone(
    workspace_id: String,
    name: String,
    position_metadata: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Zone> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_zone(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            position_metadata,
        ) {
            Ok(zone) => IpcResponse::success(zone),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn delete_zone(id: String, kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>) -> IpcResponse<()> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_zone(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
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
pub fn get_zone(id: String, kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>) -> IpcResponse<Zone> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_zone(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
        ) {
            Ok(zone) => IpcResponse::success(zone),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn create_application(
    workspace_id: String,
    name: String,
    identifier: Option<String>,
    executable_path: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationReference> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_application(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            identifier,
            executable_path,
        ) {
            Ok(application) => IpcResponse::success(application),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn launch_application(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<workspace_domain::ApplicationLaunchResult> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::launch_application(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
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
pub fn delete_application(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<()> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_application(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
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
pub fn get_application(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<ApplicationReference> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_application(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
        ) {
            Ok(application) => IpcResponse::success(application),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn create_widget(
    workspace_id: String,
    name: String,
    widget_type: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WidgetReference> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_widget(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            widget_type,
        ) {
            Ok(widget) => IpcResponse::success(widget),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn delete_widget(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<()> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_widget(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
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
pub fn get_widget(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WidgetReference> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_widget(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
        ) {
            Ok(widget) => IpcResponse::success(widget),
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
    use workspace_domain::{ActorType, IntentType, LOCAL_USER_ACTOR_ID};
    use workspace_kernel::WorkspaceKernel;

    #[test]
    fn ipc_resource_commands_use_command_layer() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ipc_actor_context();
        let intent = ipc_intent_context();
        assert_eq!(actor.actor.actor_type, ActorType::LocalUser);
        assert_eq!(actor.actor.id.as_str(), LOCAL_USER_ACTOR_ID);
        assert_eq!(intent.intent.intent_type, IntentType::UserRequest);

        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Resource IPC".into(),
        )
        .unwrap();

        let zone = CommandHandler::create_zone(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            "Zone A".into(),
            None,
        )
        .unwrap();
        let loaded = CommandHandler::get_zone(&kernel, actor, intent, zone.id.to_string()).unwrap();
        assert_eq!(loaded.name, "Zone A");
    }
}
