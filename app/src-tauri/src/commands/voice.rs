//! Voice Input IPC — Conversation input device (P16 / P16.5).
//!
//! Does not route through Kernel Operator / Capability Runtime.
//! Transcripts are inserted into Conversation and submitted as typed text.

use std::sync::{Arc, Mutex, OnceLock};

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use workspace_windows_integration::{
    platform_voice, VoiceCapabilityStatus, VoiceListenOutcome, VoicePort, VoiceSettingsTarget,
};

use super::error::CommandError;
use super::response::IpcResponse;

fn sanitize_voice_user_message(raw: impl std::fmt::Display) -> String {
    let text = raw.to_string();
    let lower = text.to_ascii_lowercase();
    if lower.contains("privacy policy")
        || lower.contains("privacy statement")
        || lower.contains("0x80045509")
    {
        return "Windows needs speech privacy turned on before I can listen. Click the microphone again and I’ll open the right Settings page for you.".into();
    }
    if lower.contains("microphone") && lower.contains("settings") {
        return text;
    }
    if lower.contains("0x")
        || lower.contains("recognize:")
        || lower.contains("winrt")
        || lower.contains("speechrecognizer")
        || lower.contains("hresult")
    {
        return "I couldn’t listen just now. Check that a microphone is connected and try again.".into();
    }
    text
}

fn voice_port() -> &'static Arc<dyn VoicePort> {
    static PORT: OnceLock<Arc<dyn VoicePort>> = OnceLock::new();
    PORT.get_or_init(platform_voice)
}

/// Warm the speech engine in the background (app startup / UI mount).
pub fn warm_voice_engine_async() {
    std::thread::spawn(|| {
        let _ = voice_port().warm_up();
        let _ = voice_port().status();
    });
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
    pub warmed: bool,
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
            warmed: status.warmed,
        }),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
        )),
    }
}

#[tauri::command]
pub fn voice_warm_up() -> IpcResponse<VoiceStatusDto> {
    let _ = voice_port().warm_up();
    voice_status()
}

#[tauri::command]
pub fn voice_listen_once(app: AppHandle) -> IpcResponse<VoiceListenOutcome> {
    set_input_state("preparing");
    let app_for_ready = app.clone();
    let app_for_sound = app.clone();
    let outcome = voice_port().listen_once_when_ready(
        Box::new(move || {
            // Ready contract: Capturing established — invite speech (not yet "Listening").
            set_input_state("ready");
            let _ = app_for_ready.emit("voice-ready", ());
        }),
        Some(std::sync::Arc::new(move || {
            // Speech detected — Listening UI may show.
            set_input_state("listening");
            let _ = app_for_sound.emit("voice-sound", ());
            let _ = app_for_sound.emit("voice-listening", ());
        })),
    );
    set_input_state("idle");
    match outcome {
        Ok(mut result) => {
            result.message = sanitize_voice_user_message(&result.message);
            IpcResponse::success(result)
        }
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
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
            sanitize_voice_user_message(error),
        )),
    }
}

#[tauri::command]
pub fn voice_open_settings(target: String) -> IpcResponse<()> {
    let parsed = VoiceSettingsTarget::parse(&target).unwrap_or(VoiceSettingsTarget::Microphone);
    match voice_port().open_settings(parsed) {
        Ok(()) => IpcResponse::success(()),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
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
        warmed: status.warmed,
    }
}
