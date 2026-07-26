use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    WorkspaceProfile, WorkspaceProfileComparison, WorkspaceProfileMemberInput,
    WorkspaceProfileState, WorkspaceProfileStateComparison, WorkspaceProfileStatus,
    WorkspaceProfileValidation,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

#[tauri::command]
pub fn create_workspace_profile(
    workspace_id: String,
    name: String,
    description: String,
    members: Vec<WorkspaceProfileMemberInput>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfile> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_workspace_profile(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            name,
            description,
            members,
        ) {
            Ok(profile) => IpcResponse::success(profile),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn update_workspace_profile(
    profile_id: String,
    name: Option<String>,
    description: Option<String>,
    status: Option<WorkspaceProfileStatus>,
    members: Option<Vec<WorkspaceProfileMemberInput>>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfile> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_workspace_profile(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            profile_id,
            name,
            description,
            status,
            members,
        ) {
            Ok(profile) => IpcResponse::success(profile),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn list_workspace_profiles(
    workspace_id: String,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<Vec<WorkspaceProfile>> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::list_workspace_profiles(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
        ) {
            Ok(profiles) => IpcResponse::success(profiles),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_workspace_profile(
    profile_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfile> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_workspace_profile(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            profile_id,
        ) {
            Ok(profile) => IpcResponse::success(profile),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn generate_workspace_profile_state(
    workspace_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfileState> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::generate_workspace_profile_state(
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
pub fn compare_workspace_profile(
    workspace_id: String,
    profile_id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfileComparison> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::compare_workspace_profile(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            profile_id,
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

#[tauri::command]
pub fn compare_workspace_profile_states(
    left: WorkspaceProfileState,
    right: WorkspaceProfileState,
) -> IpcResponse<WorkspaceProfileStateComparison> {
    IpcResponse::success(CommandHandler::compare_workspace_profile_states(
        &left, &right,
    ))
}

#[tauri::command]
pub fn validate_workspace_profile_state(
    state: WorkspaceProfileState,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<WorkspaceProfileValidation> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::validate_workspace_profile_state(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            &state,
        ) {
            Ok(report) => IpcResponse::success(report),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
