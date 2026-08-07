//! Voice input port — WRAP WinRT speech recognition (P16).
//!
//! Voice is a Conversation **input device**, not a desktop Capability Provider.
//! It never invokes other providers and never owns orchestration.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Windows SPERR_PRIVACY_STATEMENT_DECLINED — speech privacy not accepted.
pub const HRESULT_SPEECH_PRIVACY_DECLINED: u32 = 0x8004_5509;

const SPEECH_PRIVACY_MESSAGE: &str = "Windows needs speech privacy turned on before I can listen. Open Settings → Privacy & security → Speech, turn on Online speech recognition, then try again.";

/// Map OS / WinRT speech failures to ordinary-language listen outcomes.
/// Never exposes HRESULT, stack traces, or provider terminology.
pub fn classify_speech_failure(code: u32, detail: &str) -> VoiceListenOutcome {
    let lower = detail.to_ascii_lowercase();
    if code == HRESULT_SPEECH_PRIVACY_DECLINED
        || lower.contains("privacy policy")
        || lower.contains("privacy statement")
    {
        return VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "permission_denied".into(),
            message: SPEECH_PRIVACY_MESSAGE.into(),
        };
    }
    if code == 0x8004_503A
        || (lower.contains("recognizer") && lower.contains("not found"))
    {
        return VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_unavailable".into(),
            message: "Speech recognition isn’t set up for this language. Check Windows speech settings and try again.".into(),
        };
    }
    if code == 0x8007_0005
        || lower.contains("access is denied")
        || lower.contains("microphone")
    {
        return VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "microphone_unavailable".into(),
            message: "I couldn’t reach a microphone. Check that one is connected and allowed for Workspace.".into(),
        };
    }
    VoiceListenOutcome {
        ok: false,
        transcript: None,
        status: "recognition_failed".into(),
        message: "I couldn’t listen just now. Check that a microphone is connected and try again.".into(),
    }
}

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
                    message: "Voice can listen after Windows speech privacy is allowed.".into(),
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
    let unavailable = || "Speech recognition isn’t available on this system.".to_string();
    let recognizer = SpeechRecognizer::new().map_err(|_| unavailable())?;
    let topic = SpeechRecognitionTopicConstraint::Create(
        SpeechRecognitionScenario::Dictation,
        &HSTRING::from("workspace-dictation"),
    )
    .map_err(|_| unavailable())?;
    recognizer
        .Constraints()
        .map_err(|_| unavailable())?
        .Append(&topic)
        .map_err(|_| unavailable())?;
    let _ = recognizer
        .CompileConstraintsAsync()
        .map_err(|_| unavailable())?
        .get()
        .map_err(|_| unavailable())?;
    Ok(())
}

#[cfg(windows)]
fn outcome_from_winrt_error(error: windows::core::Error) -> VoiceListenOutcome {
    classify_speech_failure(error.code().0 as u32, &error.to_string())
}

#[cfg(windows)]
fn winrt_listen_once() -> Result<VoiceListenOutcome> {
    use windows::core::HSTRING;
    use windows::Media::SpeechRecognition::{
        SpeechRecognitionResultStatus, SpeechRecognitionScenario,
        SpeechRecognitionTopicConstraint, SpeechRecognizer,
    };

    ensure_com();
    let recognizer = match SpeechRecognizer::new() {
        Ok(value) => value,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };
    let topic = match SpeechRecognitionTopicConstraint::Create(
        SpeechRecognitionScenario::Dictation,
        &HSTRING::from("workspace-dictation"),
    ) {
        Ok(value) => value,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };
    if let Err(error) = recognizer
        .Constraints()
        .and_then(|constraints| constraints.Append(&topic))
    {
        return Ok(outcome_from_winrt_error(error));
    }
    let compile = match recognizer.CompileConstraintsAsync() {
        Ok(op) => op,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };
    if let Err(error) = compile.get() {
        return Ok(outcome_from_winrt_error(error));
    }

    // First architectural failure observed in Product Owner review:
    // RecognizeAsync → 0x80045509 when Windows speech privacy is not accepted.
    let recognize = match recognizer.RecognizeAsync() {
        Ok(op) => op,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };
    let result = match recognize.get() {
        Ok(value) => value,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };

    let status = match result.Status() {
        Ok(value) => value,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };

    match status {
        SpeechRecognitionResultStatus::Success => {
            let text = match result.Text() {
                Ok(value) => value,
                Err(error) => return Ok(outcome_from_winrt_error(error)),
            };
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
            message: "Speech recognition isn’t available right now. Check your network and try again.".into(),
        }),
        _ => Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_failed".into(),
            message: "I couldn’t recognize that. Try again when you’re ready.".into(),
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

    #[test]
    fn speech_privacy_hresult_maps_to_desktop_language() {
        let outcome = classify_speech_failure(
            HRESULT_SPEECH_PRIVACY_DECLINED,
            "The speech privacy policy was not accepted prior to attempting a speech recognition. (0x80045509)",
        );
        assert!(!outcome.ok);
        assert_eq!(outcome.status, "permission_denied");
        assert!(outcome.message.contains("speech privacy"));
        assert!(outcome.message.contains("Settings"));
        assert!(!outcome.message.contains("0x"));
        assert!(!outcome.message.to_ascii_lowercase().contains("winrt"));
        assert!(!outcome.message.to_ascii_lowercase().contains("speechrecognizer"));
    }

    #[test]
    fn speech_failure_never_echoes_raw_detail() {
        let outcome = classify_speech_failure(0xDEAD_BEEF, "recognize: boom HRESULT");
        assert!(!outcome.ok);
        assert!(!outcome.message.contains("0x"));
        assert!(!outcome.message.contains("HRESULT"));
        assert!(!outcome.message.contains("recognize:"));
    }
}
