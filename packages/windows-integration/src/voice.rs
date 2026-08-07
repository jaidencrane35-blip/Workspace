//! Voice input port — WRAP WinRT speech recognition (P16 / P16.5).
//!
//! Voice is a Conversation **input device**, not a desktop Capability Provider.
//! It never invokes other providers and never owns orchestration.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Windows SPERR_PRIVACY_STATEMENT_DECLINED — speech privacy not accepted.
pub const HRESULT_SPEECH_PRIVACY_DECLINED: u32 = 0x8004_5509;

const SPEECH_PRIVACY_MESSAGE: &str = "Windows needs speech privacy turned on before I can listen. Open Settings → Privacy & security → Speech, turn on Online speech recognition, then try again.";

const MICROPHONE_PERMISSION_MESSAGE: &str = "Workspace can’t use the microphone yet. Open Settings → Privacy & security → Microphone, allow access for Workspace, then try again.";

/// Windows Settings pages Voice may open for the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceSettingsTarget {
    Microphone,
    SpeechPrivacy,
}

impl VoiceSettingsTarget {
    pub fn as_settings_uri(self) -> &'static str {
        match self {
            Self::Microphone => "ms-settings:privacy-microphone",
            Self::SpeechPrivacy => "ms-settings:privacy-speech",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "microphone" | "mic" => Some(Self::Microphone),
            "speech" | "speech_privacy" | "privacy" => Some(Self::SpeechPrivacy),
            _ => None,
        }
    }
}

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
            message: MICROPHONE_PERMISSION_MESSAGE.into(),
        };
    }
    VoiceListenOutcome {
        ok: false,
        transcript: None,
        status: "recognition_failed".into(),
        message: "I couldn’t listen just now. Check that a microphone is connected and try again.".into(),
    }
}

/// Open a Windows Settings URI (best-effort).
pub fn open_windows_settings_uri(uri: &str) -> Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        Command::new("explorer")
            .arg(uri)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|error| {
                WindowsIntegrationError::VoiceFailed(format!(
                    "Couldn’t open Windows Settings ({error})."
                ))
            })?;
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = uri;
        Err(WindowsIntegrationError::VoiceFailed(
            "Windows Settings aren’t available here.".into(),
        ))
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
    /// True when the speech engine is pre-warmed and ready for immediate listen.
    #[serde(default)]
    pub warmed: bool,
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
    /// Pre-create / compile the speech engine so the next listen is immediate.
    fn warm_up(&self) -> Result<()>;
    /// Listen for one utterance. `on_ready` fires only when capture has truly begun.
    fn listen_once_when_ready(
        &self,
        on_ready: Box<dyn FnOnce() + Send>,
    ) -> Result<VoiceListenOutcome>;
    fn listen_once(&self) -> Result<VoiceListenOutcome> {
        self.listen_once_when_ready(Box::new(|| {}))
    }
    fn cancel(&self) -> Result<()>;
    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()>;
}

/// In-process voice port for tests / demo.
#[derive(Debug)]
pub struct MemoryVoicePort {
    next_transcript: Mutex<Option<String>>,
    fail_permission: AtomicBool,
    cancelled: AtomicBool,
    warmed: AtomicBool,
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
            warmed: AtomicBool::new(false),
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
                message: MICROPHONE_PERMISSION_MESSAGE.into(),
                warmed: self.warmed.load(Ordering::SeqCst),
            });
        }
        Ok(VoiceCapabilityStatus {
            available: true,
            microphone_available: true,
            recognition_available: true,
            permission: "granted".into(),
            message: "Voice is ready.".into(),
            warmed: self.warmed.load(Ordering::SeqCst),
        })
    }

    fn warm_up(&self) -> Result<()> {
        self.warmed.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn listen_once_when_ready(
        &self,
        on_ready: Box<dyn FnOnce() + Send>,
    ) -> Result<VoiceListenOutcome> {
        self.cancelled.store(false, Ordering::SeqCst);
        let _ = self.warm_up();
        if self.fail_permission.load(Ordering::SeqCst) {
            let _ = self.open_settings(VoiceSettingsTarget::Microphone);
            return Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "permission_denied".into(),
                message: MICROPHONE_PERMISSION_MESSAGE.into(),
            });
        }
        on_ready();
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

    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()> {
        open_windows_settings_uri(target.as_settings_uri())
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
            warmed: false,
        })
    }

    fn warm_up(&self) -> Result<()> {
        Ok(())
    }

    fn listen_once_when_ready(
        &self,
        _on_ready: Box<dyn FnOnce() + Send>,
    ) -> Result<VoiceListenOutcome> {
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

    fn open_settings(&self, _target: VoiceSettingsTarget) -> Result<()> {
        Ok(())
    }
}

/// Production WRAP of WinRT `SpeechRecognizer` (dictation scenario).
pub struct SystemVoicePort {
    cancel_requested: AtomicBool,
    warmed: AtomicBool,
    #[cfg(windows)]
    engine: Mutex<Option<WinrtVoiceEngine>>,
    #[cfg(windows)]
    mic_access: Mutex<Option<MicAccess>>,
}

impl Default for SystemVoicePort {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for SystemVoicePort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemVoicePort")
            .field("warmed", &self.warmed.load(Ordering::SeqCst))
            .finish()
    }
}

#[cfg(windows)]
struct WinrtVoiceEngine {
    recognizer: windows::Media::SpeechRecognition::SpeechRecognizer,
}

impl SystemVoicePort {
    pub fn new() -> Self {
        Self {
            cancel_requested: AtomicBool::new(false),
            warmed: AtomicBool::new(false),
            #[cfg(windows)]
            engine: Mutex::new(None),
            #[cfg(windows)]
            mic_access: Mutex::new(None),
        }
    }
}

impl VoicePort for SystemVoicePort {
    fn status(&self) -> Result<VoiceCapabilityStatus> {
        #[cfg(windows)]
        {
            let warm_result = self.warm_up();
            let mic = cached_microphone_access(&self.mic_access);
            match (warm_result, mic) {
                (Ok(()), MicAccess::Allowed) => Ok(VoiceCapabilityStatus {
                    available: true,
                    microphone_available: true,
                    recognition_available: true,
                    permission: "granted".into(),
                    message: "Voice is ready.".into(),
                    warmed: self.warmed.load(Ordering::SeqCst),
                }),
                (Ok(()), MicAccess::Denied) => Ok(VoiceCapabilityStatus {
                    available: false,
                    microphone_available: false,
                    recognition_available: true,
                    permission: "denied".into(),
                    message: MICROPHONE_PERMISSION_MESSAGE.into(),
                    warmed: self.warmed.load(Ordering::SeqCst),
                }),
                (Ok(()), MicAccess::Unknown) => Ok(VoiceCapabilityStatus {
                    available: true,
                    microphone_available: true,
                    recognition_available: true,
                    permission: "prompt".into(),
                    message: "Voice is ready.".into(),
                    warmed: self.warmed.load(Ordering::SeqCst),
                }),
                (Err(error), _) => Ok(VoiceCapabilityStatus {
                    available: false,
                    microphone_available: false,
                    recognition_available: false,
                    permission: "unavailable".into(),
                    message: sanitize_setup_message(&error.to_string()),
                    warmed: false,
                }),
            }
        }
        #[cfg(not(windows))]
        {
            UnavailableVoicePort.status()
        }
    }

    fn warm_up(&self) -> Result<()> {
        #[cfg(windows)]
        {
            winrt_warm_up(&self.engine, &self.warmed)
        }
        #[cfg(not(windows))]
        {
            Ok(())
        }
    }

    fn listen_once_when_ready(
        &self,
        on_ready: Box<dyn FnOnce() + Send>,
    ) -> Result<VoiceListenOutcome> {
        self.cancel_requested.store(false, Ordering::SeqCst);
        #[cfg(windows)]
        {
            // Use cached mic state only — never run MediaCapture on the listen hot path.
            if matches!(
                self.mic_access.lock().ok().and_then(|g| *g),
                Some(MicAccess::Denied)
            ) {
                let _ = open_windows_settings_uri(VoiceSettingsTarget::Microphone.as_settings_uri());
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "permission_denied".into(),
                    message: MICROPHONE_PERMISSION_MESSAGE.into(),
                });
            }
            winrt_listen_once_when_ready(
                &self.engine,
                &self.warmed,
                &self.cancel_requested,
                on_ready,
            )
        }
        #[cfg(not(windows))]
        {
            let _ = on_ready;
            UnavailableVoicePort.listen_once()
        }
    }

    fn cancel(&self) -> Result<()> {
        self.cancel_requested.store(true, Ordering::SeqCst);
        #[cfg(windows)]
        {
            if let Ok(guard) = self.engine.lock() {
                if let Some(engine) = guard.as_ref() {
                    let _ = engine.recognizer.StopRecognitionAsync();
                }
            }
        }
        Ok(())
    }

    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()> {
        open_windows_settings_uri(target.as_settings_uri())
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

fn sanitize_setup_message(raw: &str) -> String {
    let outcome = classify_speech_failure(0, raw);
    if outcome.status == "recognition_failed" {
        "Speech recognition isn’t available on this system.".into()
    } else {
        outcome.message
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MicAccess {
    Allowed,
    Denied,
    Unknown,
}

#[cfg(windows)]
fn ensure_com() {
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }
}

#[cfg(windows)]
fn cached_microphone_access(cache: &Mutex<Option<MicAccess>>) -> MicAccess {
    if let Ok(guard) = cache.lock() {
        if let Some(value) = *guard {
            return value;
        }
    }
    let value = probe_microphone_access();
    if let Ok(mut guard) = cache.lock() {
        *guard = Some(value);
    }
    value
}

#[cfg(windows)]
fn probe_microphone_access() -> MicAccess {
    use windows::Media::Capture::{
        MediaCapture, MediaCaptureInitializationSettings, StreamingCaptureMode,
    };

    ensure_com();
    let settings = match MediaCaptureInitializationSettings::new() {
        Ok(value) => value,
        Err(_) => return MicAccess::Unknown,
    };
    if settings
        .SetStreamingCaptureMode(StreamingCaptureMode::Audio)
        .is_err()
    {
        return MicAccess::Unknown;
    }
    let capture = match MediaCapture::new() {
        Ok(value) => value,
        Err(_) => return MicAccess::Unknown,
    };
    match capture
        .InitializeWithSettingsAsync(&settings)
        .and_then(|op| op.get())
    {
        Ok(()) => MicAccess::Allowed,
        Err(error) => {
            let detail = error.to_string().to_ascii_lowercase();
            if detail.contains("access")
                || detail.contains("denied")
                || detail.contains("privacy")
                || error.code().0 as u32 == 0x8007_0005
            {
                MicAccess::Denied
            } else {
                MicAccess::Unknown
            }
        }
    }
}

#[cfg(windows)]
fn create_compiled_recognizer(
) -> std::result::Result<windows::Media::SpeechRecognition::SpeechRecognizer, windows::core::Error>
{
    use windows::core::HSTRING;
    use windows::Foundation::TimeSpan;
    use windows::Media::SpeechRecognition::{
        SpeechRecognitionScenario, SpeechRecognitionTopicConstraint, SpeechRecognizer,
    };

    ensure_com();
    let recognizer = SpeechRecognizer::new()?;
    let topic = SpeechRecognitionTopicConstraint::Create(
        SpeechRecognitionScenario::Dictation,
        &HSTRING::from("workspace-dictation"),
    )?;
    recognizer.Constraints()?.Append(&topic)?;
    recognizer.CompileConstraintsAsync()?.get()?;

    // Give the user time to start speaking after the ready indicator appears.
    if let Ok(timeouts) = recognizer.Timeouts() {
        let _ = timeouts.SetInitialSilenceTimeout(TimeSpan {
            Duration: 12_i64 * 10_000_000,
        });
        let _ = timeouts.SetEndSilenceTimeout(TimeSpan {
            Duration: 2_i64 * 10_000_000,
        });
        let _ = timeouts.SetBabbleTimeout(TimeSpan {
            Duration: 8_i64 * 10_000_000,
        });
    }
    Ok(recognizer)
}

#[cfg(windows)]
fn winrt_warm_up(
    engine: &Mutex<Option<WinrtVoiceEngine>>,
    warmed: &AtomicBool,
) -> Result<()> {
    if warmed.load(Ordering::SeqCst) {
        if let Ok(guard) = engine.lock() {
            if guard.is_some() {
                return Ok(());
            }
        }
    }
    let recognizer = create_compiled_recognizer().map_err(|error| {
        WindowsIntegrationError::VoiceFailed(sanitize_setup_message(&error.to_string()))
    })?;
    if let Ok(mut guard) = engine.lock() {
        *guard = Some(WinrtVoiceEngine { recognizer });
    }
    warmed.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(windows)]
fn outcome_from_winrt_error(error: windows::core::Error) -> VoiceListenOutcome {
    let outcome = classify_speech_failure(error.code().0 as u32, &error.to_string());
    if outcome.status == "permission_denied" {
        let _ = open_windows_settings_uri(VoiceSettingsTarget::SpeechPrivacy.as_settings_uri());
    } else if outcome.status == "microphone_unavailable" {
        let _ = open_windows_settings_uri(VoiceSettingsTarget::Microphone.as_settings_uri());
    }
    outcome
}

#[cfg(windows)]
fn winrt_listen_once_when_ready(
    engine: &Mutex<Option<WinrtVoiceEngine>>,
    warmed: &AtomicBool,
    cancel_requested: &AtomicBool,
    on_ready: Box<dyn FnOnce() + Send>,
) -> Result<VoiceListenOutcome> {
    use windows::Media::SpeechRecognition::SpeechRecognitionResultStatus;

    if let Err(error) = winrt_warm_up(engine, warmed) {
        return Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_unavailable".into(),
            message: sanitize_setup_message(&error.to_string()),
        });
    }

    let recognizer = {
        let guard = engine
            .lock()
            .map_err(|_| WindowsIntegrationError::VoiceFailed("voice engine lock poisoned".into()))?;
        match guard.as_ref() {
            Some(engine) => engine.recognizer.clone(),
            None => {
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "recognition_unavailable".into(),
                    message: "Speech recognition isn’t available on this system.".into(),
                });
            }
        }
    };

    if cancel_requested.load(Ordering::SeqCst) {
        return Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "cancelled".into(),
            message: "Listening stopped.".into(),
        });
    }

    // With a warm engine, RecognizeAsync starts capture immediately. Notify only after
    // the session is started — never while create/compile is still running.
    let recognize = match recognizer.RecognizeAsync() {
        Ok(op) => op,
        Err(error) => return Ok(outcome_from_winrt_error(error)),
    };
    on_ready();

    let result = match recognize.get() {
        Ok(value) => value,
        Err(error) => {
            if cancel_requested.load(Ordering::SeqCst) {
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "cancelled".into(),
                    message: "Listening stopped.".into(),
                });
            }
            return Ok(outcome_from_winrt_error(error));
        }
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
        SpeechRecognitionResultStatus::MicrophoneUnavailable => {
            let _ = open_windows_settings_uri(VoiceSettingsTarget::Microphone.as_settings_uri());
            Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "microphone_unavailable".into(),
                message: MICROPHONE_PERMISSION_MESSAGE.into(),
            })
        }
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
    fn memory_warm_up_marks_ready_and_notifies() {
        let port = MemoryVoicePort::new();
        assert!(!port.status().unwrap().warmed);
        port.warm_up().unwrap();
        assert!(port.status().unwrap().warmed);
        let notified = std::sync::Arc::new(AtomicBool::new(false));
        let notified_ready = std::sync::Arc::clone(&notified);
        let outcome = port
            .listen_once_when_ready(Box::new(move || {
                notified_ready.store(true, Ordering::SeqCst);
            }))
            .unwrap();
        assert!(outcome.ok);
        assert!(notified.load(Ordering::SeqCst));
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
        assert!(!outcome
            .message
            .to_ascii_lowercase()
            .contains("speechrecognizer"));
    }

    #[test]
    fn speech_failure_never_echoes_raw_detail() {
        let outcome = classify_speech_failure(0xDEAD_BEEF, "recognize: boom HRESULT");
        assert!(!outcome.ok);
        assert!(!outcome.message.contains("0x"));
        assert!(!outcome.message.contains("HRESULT"));
        assert!(!outcome.message.contains("recognize:"));
    }

    #[test]
    fn settings_targets_map_to_ms_settings_uris() {
        assert_eq!(
            VoiceSettingsTarget::Microphone.as_settings_uri(),
            "ms-settings:privacy-microphone"
        );
        assert_eq!(
            VoiceSettingsTarget::SpeechPrivacy.as_settings_uri(),
            "ms-settings:privacy-speech"
        );
    }
}
