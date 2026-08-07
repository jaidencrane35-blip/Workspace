//! Voice Input IPC — Conversation input device (P16).
//!
//! Does not route through Kernel Operator / Capability Runtime.
//! Transcripts are inserted into Conversation and submitted as typed text.

use std::sync::{Arc, Mutex, OnceLock};

use serde::Serialize;
use workspace_windows_integration::{
    platform_voice, VoiceCapabilityStatus, VoiceListenOutcome, VoicePort,
};

use super::error::CommandError;
use super::response::IpcResponse;

fn voice_port() -> &'static Arc<dyn VoicePort> {
    static PORT: OnceLock<Arc<dyn VoicePort>> = OnceLock::new();
    PORT.get_or_init(platform_voice)
}

/// Optional test override (unit / harness).
#[allow(dead_code)]
pub fn install_voice_port_for_tests(port: Arc<dyn VoicePort>) {
    let _ = port;
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceStatusDto {
    pub available: bool,
    pub microphone_available: bool,
    pub recognition_available: bool,
    pub permission: String,
    pub message: String,
    pub input_state: String,
}

static INPUT_STATE: OnceLock<Mutex<String>> = OnceLock::new();

fn input_state() -> &'static Mutex<String> {
    INPUT_STATE.get_or_init(|| Mutex::new("idle".into()))
}

fn set_input_state(next: &str) {
    if let Ok(mut guard) = input_state().lock() {
        *guard = next.into();
    }
}

fn current_input_state() -> String {
    input_state()
        .lock()
        .map(|g| g.clone())
        .unwrap_or_else(|_| "idle".into())
}

#[tauri::command]
pub fn voice_status() -> IpcResponse<VoiceStatusDto> {
    match voice_port().status() {
        Ok(status) => IpcResponse::success(VoiceStatusDto {
            available: status.available,
            microphone_available: status.microphone_available,
            recognition_available: status.recognition_available,
            permission: status.permission,
            message: status.message,
            input_state: current_input_state(),
        }),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            error.to_string(),
        )),
    }
}

#[tauri::command]
pub fn voice_listen_once() -> IpcResponse<VoiceListenOutcome> {
    set_input_state("listening");
    let outcome = voice_port().listen_once();
    set_input_state("idle");
    match outcome {
        Ok(result) => IpcResponse::success(result),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            error.to_string(),
        )),
    }
}

#[tauri::command]
pub fn voice_cancel() -> IpcResponse<()> {
    match voice_port().cancel() {
        Ok(()) => {
            set_input_state("idle");
            IpcResponse::success(())
        }
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            error.to_string(),
        )),
    }
}

#[allow(dead_code)]
fn _status_shape(status: VoiceCapabilityStatus) -> VoiceStatusDto {
    VoiceStatusDto {
        available: status.available,
        microphone_available: status.microphone_available,
        recognition_available: status.recognition_available,
        permission: status.permission,
        message: status.message,
        input_state: current_input_state(),
    }
}
