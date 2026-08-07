//! Voice Input IPC — Conversation input device (P16 / P16.10).
//!
//! Does not route through Kernel Operator / Capability Runtime.
//! Transcripts are inserted into Conversation and submitted as typed text.
//!
//! P16.10: WinRT listen/warm run on spawn_blocking so the Tauri IPC/UI
//! runtime is never blocked for seconds–minutes (crash / freeze root cause).

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
        return "Windows needs speech privacy turned on before I can listen. Click the microphone once and I’ll open the right Settings page — then come back here.".into();
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
        return "I couldn’t listen just now. Try the microphone again — if it keeps failing, click once for Settings help.".into();
    }
    text
}

fn voice_port() -> &'static Arc<dyn VoicePort> {
    static PORT: OnceLock<Arc<dyn VoicePort>> = OnceLock::new();
    PORT.get_or_init(platform_voice)
}

/// Warm the speech engine in the background (app startup / UI mount).
/// Never probes MediaCapture on the UI path — warm_up owns one-time setup.
pub fn warm_voice_engine_async() {
    std::thread::spawn(|| {
        let t0 = std::time::Instant::now();
        log::info!("voice.lifecycle: startup_warm_begin");
        match voice_port().warm_up() {
            Ok(()) => log::info!("voice.lifecycle: startup_warm_done +{:?}", t0.elapsed()),
            Err(error) => log::warn!(
                "voice.lifecycle: startup_warm_failed +{:?} err={}",
                t0.elapsed(),
                error
            ),
        }
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

fn status_dto(status: VoiceCapabilityStatus) -> VoiceStatusDto {
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

/// Lightweight — must not compile recognizers or open MediaCapture.
#[tauri::command]
pub fn voice_status() -> IpcResponse<VoiceStatusDto> {
    match voice_port().status() {
        Ok(status) => IpcResponse::success(status_dto(status)),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
        )),
    }
}

#[tauri::command]
pub async fn voice_warm_up() -> IpcResponse<VoiceStatusDto> {
    let warm = tauri::async_runtime::spawn_blocking(|| voice_port().warm_up()).await;
    match warm {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            return IpcResponse::failure(CommandError::new(
                "voice_failed",
                sanitize_voice_user_message(error),
            ));
        }
        Err(error) => {
            return IpcResponse::failure(CommandError::new(
                "voice_failed",
                sanitize_voice_user_message(error),
            ));
        }
    }
    voice_status()
}

/// Long-running WinRT session — never on the async IPC worker thread.
#[tauri::command]
pub async fn voice_listen_once(app: AppHandle) -> IpcResponse<VoiceListenOutcome> {
    set_input_state("preparing");
    let app_for_ready = app.clone();
    let app_for_sound = app.clone();
    let joined = tauri::async_runtime::spawn_blocking(move || {
        log::info!("voice.lifecycle: ipc_listen_blocking_begin");
        let t0 = std::time::Instant::now();
        let outcome = voice_port().listen_once_when_ready(
            Box::new(move || {
                set_input_state("ready");
                let _ = app_for_ready.emit("voice-ready", ());
            }),
            Some(std::sync::Arc::new(move || {
                set_input_state("listening");
                let _ = app_for_sound.emit("voice-sound", ());
                let _ = app_for_sound.emit("voice-listening", ());
            })),
        );
        log::info!(
            "voice.lifecycle: ipc_listen_blocking_end +{:?}",
            t0.elapsed()
        );
        outcome
    })
    .await;
    set_input_state("idle");
    match joined {
        Ok(Ok(mut result)) => {
            result.message = sanitize_voice_user_message(&result.message);
            IpcResponse::success(result)
        }
        Ok(Err(error)) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
        )),
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

/// After the user returns from Windows Settings — re-probe mic once (spawn_blocking).
#[tauri::command]
pub async fn voice_recheck_permission() -> IpcResponse<VoiceStatusDto> {
    let joined =
        tauri::async_runtime::spawn_blocking(|| voice_port().recheck_microphone()).await;
    match joined {
        Ok(Ok(status)) => IpcResponse::success(status_dto(status)),
        Ok(Err(error)) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
        )),
        Err(error) => IpcResponse::failure(CommandError::new(
            "voice_failed",
            sanitize_voice_user_message(error),
        )),
    }
}
