use std::sync::{Arc, Mutex};

use tauri::State;
use workspace_domain::{
    PilotBaseline, PilotConsent, PilotInterviewRecord, PilotLeaveResumeRecord,
    PilotMeasurementScope, PilotMeasurementSnapshot,
};
use workspace_kernel::{CommandHandler, WorkspaceKernel};

use super::error::CommandError;
use super::response::IpcResponse;
use crate::actor::{ipc_actor_context, ipc_intent_context};

#[tauri::command]
pub fn get_pilot_measurement_scope(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotMeasurementScope> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_pilot_measurement_scope(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
        ) {
            Ok(scope) => IpcResponse::success(scope),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn get_pilot_measurement_snapshot(
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotMeasurementSnapshot> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::get_pilot_measurement_snapshot(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
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

#[tauri::command]
pub fn grant_pilot_consent(
    approved_scope: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotConsent> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::grant_pilot_consent(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            approved_scope,
        ) {
            Ok(consent) => IpcResponse::success(consent),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn withdraw_pilot_consent(
    clear_records: bool,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotConsent> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::withdraw_pilot_consent(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            clear_records,
        ) {
            Ok(consent) => IpcResponse::success(consent),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn record_pilot_baseline(
    return_minutes: u32,
    notes: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotBaseline> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::record_pilot_baseline(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            return_minutes,
            notes,
        ) {
            Ok(baseline) => IpcResponse::success(baseline),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn record_pilot_leave_resume(
    return_minutes: u32,
    correction_needed: bool,
    correction_note: String,
    local_day: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotLeaveResumeRecord> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::record_pilot_leave_resume(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            return_minutes,
            correction_needed,
            correction_note,
            local_day,
        ) {
            Ok(record) => IpcResponse::success(record),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}

#[tauri::command]
pub fn record_pilot_interview(
    phase: String,
    responses: String,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<PilotInterviewRecord> {
    match kernel.lock() {
        Ok(kernel) => match CommandHandler::record_pilot_interview(
            &kernel,
            ipc_actor_context(),
            ipc_intent_context(),
            phase,
            responses,
        ) {
            Ok(record) => IpcResponse::success(record),
            Err(error) => IpcResponse::failure(CommandError::from(error)),
        },
        Err(_) => IpcResponse::failure(CommandError::new(
            "internal_error",
            "Workspace core is temporarily unavailable.",
        )),
    }
}
