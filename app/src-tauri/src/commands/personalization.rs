use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    AiPlan, PersonalizedPlanComparison, PreferenceCategory, PreferenceSource, UserPreference,
    UserPreferenceProfile,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use crate::actor::{ipc_actor_context, ipc_intent_context};
use super::error::CommandError;
use super::response::IpcResponse;

const DIAGNOSTIC_AI_ACTOR_ID: &str = "diagnostic-ai";

#[tauri::command]
pub fn create_user_preference(
    category: String,
    key: String,
    value: String,
    source: Option<String>,
    workspace_id: Option<String>,
    label: Option<String>,
    attributes: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<UserPreference> {
    let category = match PreferenceCategory::parse(&category) {
        Ok(value) => value,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "ai_personalization_validation_error",
                error.to_string(),
            ));
        }
    };
    let source = match PreferenceSource::parse(source.as_deref().unwrap_or("user_defined")) {
        Ok(value) => value,
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "ai_personalization_validation_error",
                error.to_string(),
            ));
        }
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::create_user_preference(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            category,
            key,
            value,
            source,
            workspace_id,
            label,
            attributes,
        ) {
            Ok(preference) => IpcResponse::success(preference),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn update_user_preference(
    id: String,
    value: Option<String>,
    label: Option<String>,
    attributes: Option<String>,
    confidence: Option<u8>,
    clear_label: Option<bool>,
    clear_attributes: Option<bool>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<UserPreference> {
    let label = if clear_label.unwrap_or(false) {
        Some(None)
    } else {
        label.map(Some)
    };
    let attributes = if clear_attributes.unwrap_or(false) {
        Some(None)
    } else {
        attributes.map(Some)
    };
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::update_user_preference(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
            value,
            label,
            attributes,
            confidence,
        ) {
            Ok(preference) => IpcResponse::success(preference),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_preference_profile(
    workspace_id: Option<String>,
    limit: Option<usize>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<UserPreferenceProfile> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_preference_profile(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            workspace_id,
            limit,
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
pub fn delete_user_preference(
    id: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<UserPreference> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::delete_user_preference(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            id,
        ) {
            Ok(preference) => IpcResponse::success(preference),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn set_personalization_enabled(
    enabled: bool,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<bool> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::set_personalization_enabled(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            enabled,
        ) {
            Ok(value) => IpcResponse::success(value),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn diagnose_ai_plan_with_personalization(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    personalization_enabled: Option<bool>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<AiPlan> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::diagnose_ai_plan_with_personalization(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
            personalization_enabled.unwrap_or(true),
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

#[tauri::command]
pub fn compare_personalized_vs_neutral_plan(
    goal: Option<String>,
    application_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PersonalizedPlanComparison> {
    let goal = goal.unwrap_or_else(|| "Prepare my workspace".into());
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::compare_personalized_vs_neutral_plan(
            &kernel,
            DIAGNOSTIC_AI_ACTOR_ID,
            goal,
            application_ids.unwrap_or_default(),
            workspace_id,
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
