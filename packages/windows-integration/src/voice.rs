//! Voice input port — WRAP WinRT speech recognition (P16).
//!
//! Voice is a Conversation **input device**, not a desktop Capability Provider.
//! It never invokes other providers and never owns orchestration.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Level 1 voice capability snapshot.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceCapabilityStatus {
    pub available: bool,
    pub microphone_available: bool,
    pub recognition_available: bool,
    pub permission: String,
    pub message: String,
}

/// Outcome of a single listen turn.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceListenOutcome {
    pub ok: bool,
    pub transcript: Option<String>,
    pub status: String,
    pub message: String,
}

/// Speech → text access behind a Workspace-owned port.
pub trait VoicePort: Send + Sync {
    fn status(&self) -> Result<VoiceCapabilityStatus>;
    fn listen_once(&self) -> Result<VoiceListenOutcome>;
    fn cancel(&self) -> Result<()>;
}

/// In-process voice port for tests / demo.
#[derive(Debug)]
pub struct MemoryVoicePort {
    next_transcript: Mutex<Option<String>>,
    fail_permission: AtomicBool,
    cancelled: AtomicBool,
}

impl Default for MemoryVoicePort {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryVoicePort {
    pub fn new() -> Self {
        Self {
            next_transcript: Mutex::new(Some("Open ChatGPT.".into())),
            fail_permission: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
        }
    }

    pub fn set_next_transcript(&self, text: impl Into<String>) {
        if let Ok(mut guard) = self.next_transcript.lock() {
            *guard = Some(text.into());
        }
    }

    pub fn set_permission_denied(&self, denied: bool) {
        self.fail_permission.store(denied, Ordering::SeqCst);
    }
}

impl VoicePort for MemoryVoicePort {
    fn status(&self) -> Result<VoiceCapabilityStatus> {
        if self.fail_permission.load(Ordering::SeqCst) {
            return Ok(VoiceCapabilityStatus {
                available: false,
                microphone_available: true,
                recognition_available: true,
                permission: "denied".into(),
                message: "Microphone permission is denied.".into(),
            });
        }
        Ok(VoiceCapabilityStatus {
            available: true,
            microphone_available: true,
            recognition_available: true,
            permission: "granted".into(),
            message: "Voice input is available.".into(),
        })
    }

    fn listen_once(&self) -> Result<VoiceListenOutcome> {
        self.cancelled.store(false, Ordering::SeqCst);
        if self.fail_permission.load(Ordering::SeqCst) {
            return Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "permission_denied".into(),
                message: "Microphone permission is denied.".into(),
            });
        }
        if self.cancelled.load(Ordering::SeqCst) {
            return Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "cancelled".into(),
                message: "Listening stopped.".into(),
            });
        }
        let transcript = self
            .next_transcript
            .lock()
            .map_err(|_| WindowsIntegrationError::VoiceFailed("lock poisoned".into()))?
            .clone()
            .unwrap_or_default();
        if transcript.trim().is_empty() {
            return Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "no_speech".into(),
                message: "I didn’t catch that. Try again when you’re ready.".into(),
            });
        }
        Ok(VoiceListenOutcome {
            ok: true,
            transcript: Some(transcript),
            status: "recognized".into(),
            message: "Got it.".into(),
        })
    }

    fn cancel(&self) -> Result<()> {
        self.cancelled.store(true, Ordering::SeqCst);
        Ok(())
    }
}

/// Unavailable stub (non-Windows / missing speech stack).
#[derive(Debug, Default)]
pub struct UnavailableVoicePort;

impl VoicePort for UnavailableVoicePort {
    fn status(&self) -> Result<VoiceCapabilityStatus> {
        Ok(VoiceCapabilityStatus {
            available: false,
            microphone_available: false,
            recognition_available: false,
            permission: "unavailable".into(),
            message: "Voice input isn’t available on this system.".into(),
        })
    }

    fn listen_once(&self) -> Result<VoiceListenOutcome> {
        Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "unavailable".into(),
            message: "Voice input isn’t available on this system.".into(),
        })
    }

    fn cancel(&self) -> Result<()> {
        Ok(())
    }
}

/// Production WRAP of WinRT `SpeechRecognizer` (dictation scenario).
#[derive(Debug, Default)]
pub struct SystemVoicePort {
    cancel_requested: AtomicBool,
}

impl SystemVoicePort {
    pub fn new() -> Self {
        Self {
            cancel_requested: AtomicBool::new(false),
        }
    }
}

impl VoicePort for SystemVoicePort {
    fn status(&self) -> Result<VoiceCapabilityStatus> {
        #[cfg(windows)]
        {
            match winrt_probe() {
                Ok(()) => Ok(VoiceCapabilityStatus {
                    available: true,
                    microphone_available: true,
                    recognition_available: true,
                    permission: "prompt".into(),
                    message: "Voice input is available.".into(),
                }),
                Err(message) => Ok(VoiceCapabilityStatus {
                    available: false,
                    microphone_available: false,
                    recognition_available: false,
                    permission: "unavailable".into(),
                    message,
                }),
            }
        }
        #[cfg(not(windows))]
        {
            UnavailableVoicePort.status()
        }
    }

    fn listen_once(&self) -> Result<VoiceListenOutcome> {
        self.cancel_requested.store(false, Ordering::SeqCst);
        #[cfg(windows)]
        {
            if self.cancel_requested.load(Ordering::SeqCst) {
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "cancelled".into(),
                    message: "Listening stopped.".into(),
                });
            }
            winrt_listen_once()
        }
        #[cfg(not(windows))]
        {
            UnavailableVoicePort.listen_once()
        }
    }

    fn cancel(&self) -> Result<()> {
        self.cancel_requested.store(true, Ordering::SeqCst);
        Ok(())
    }
}

pub fn platform_voice() -> std::sync::Arc<dyn VoicePort> {
    #[cfg(windows)]
    {
        std::sync::Arc::new(SystemVoicePort::new())
    }
    #[cfg(not(windows))]
    {
        std::sync::Arc::new(UnavailableVoicePort)
    }
}

#[cfg(windows)]
fn ensure_com() {
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }
}

#[cfg(windows)]
fn winrt_probe() -> std::result::Result<(), String> {
    use windows::core::HSTRING;
    use windows::Media::SpeechRecognition::{
        SpeechRecognitionTopicConstraint, SpeechRecognitionScenario, SpeechRecognizer,
    };

    ensure_com();
    let recognizer = SpeechRecognizer::new().map_err(|e| {
        format!("Speech recognition isn’t available ({e}).")
    })?;
    let topic = SpeechRecognitionTopicConstraint::Create(
        SpeechRecognitionScenario::Dictation,
        &HSTRING::from("workspace-dictation"),
    )
    .map_err(|e| format!("Speech recognition isn’t available ({e})."))?;
    recognizer
        .Constraints()
        .map_err(|e| format!("Speech recognition isn’t available ({e})."))?
        .Append(&topic)
        .map_err(|e| format!("Speech recognition isn’t available ({e})."))?;
    let _ = recognizer
        .CompileConstraintsAsync()
        .map_err(|e| format!("Speech recognition isn’t available ({e})."))?
        .get()
        .map_err(|e| format!("Speech recognition isn’t available ({e})."))?;
    Ok(())
}

#[cfg(windows)]
fn winrt_listen_once() -> Result<VoiceListenOutcome> {
    use windows::core::HSTRING;
    use windows::Media::SpeechRecognition::{
        SpeechRecognitionResultStatus, SpeechRecognitionScenario,
        SpeechRecognitionTopicConstraint, SpeechRecognizer,
    };

    ensure_com();
    let recognizer = SpeechRecognizer::new().map_err(|error| {
        WindowsIntegrationError::VoiceFailed(format!("create recognizer: {error}"))
    })?;
    let topic = SpeechRecognitionTopicConstraint::Create(
        SpeechRecognitionScenario::Dictation,
        &HSTRING::from("workspace-dictation"),
    )
    .map_err(|error| {
        WindowsIntegrationError::VoiceFailed(format!("create constraint: {error}"))
    })?;
    recognizer
        .Constraints()
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("constraints: {error}"))
        })?
        .Append(&topic)
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("append constraint: {error}"))
        })?;
    recognizer
        .CompileConstraintsAsync()
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("compile: {error}"))
        })?
        .get()
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("compile wait: {error}"))
        })?;

    let result = recognizer
        .RecognizeAsync()
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("recognize: {error}"))
        })?
        .get()
        .map_err(|error| {
            WindowsIntegrationError::VoiceFailed(format!("recognize wait: {error}"))
        })?;

    let status = result.Status().map_err(|error| {
        WindowsIntegrationError::VoiceFailed(format!("result status: {error}"))
    })?;

    match status {
        SpeechRecognitionResultStatus::Success => {
            let text = result.Text().map_err(|error| {
                WindowsIntegrationError::VoiceFailed(format!("result text: {error}"))
            })?;
            let transcript = text.to_string().trim().to_string();
            if transcript.is_empty() {
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "no_speech".into(),
                    message: "I didn’t catch that. Try again when you’re ready.".into(),
                });
            }
            Ok(VoiceListenOutcome {
                ok: true,
                transcript: Some(transcript),
                status: "recognized".into(),
                message: "Got it.".into(),
            })
        }
        SpeechRecognitionResultStatus::UserCanceled => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "cancelled".into(),
            message: "Listening stopped.".into(),
        }),
        SpeechRecognitionResultStatus::AudioQualityFailure => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "microphone_unavailable".into(),
            message: "I couldn’t hear the microphone clearly.".into(),
        }),
        SpeechRecognitionResultStatus::MicrophoneUnavailable => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "microphone_unavailable".into(),
            message: "I couldn’t reach a microphone. Check that one is connected and allowed.".into(),
        }),
        SpeechRecognitionResultStatus::NetworkFailure => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_unavailable".into(),
            message: "Speech recognition isn’t available right now.".into(),
        }),
        other => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_failed".into(),
            message: format!(
                "I couldn’t recognize that ({}).",
                other.0
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_voice_listen_returns_transcript() {
        let port = MemoryVoicePort::new();
        port.set_next_transcript("Take a screenshot.");
        let outcome = port.listen_once().unwrap();
        assert!(outcome.ok);
        assert_eq!(outcome.transcript.as_deref(), Some("Take a screenshot."));
    }

    #[test]
    fn memory_permission_denied_is_truthful() {
        let port = MemoryVoicePort::new();
        port.set_permission_denied(true);
        let status = port.status().unwrap();
        assert!(!status.available);
        let outcome = port.listen_once().unwrap();
        assert!(!outcome.ok);
        assert_eq!(outcome.status, "permission_denied");
    }
}
