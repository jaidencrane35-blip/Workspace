//! Voice input port — WRAP WinRT speech recognition (P16 / P16.8).
//!
//! Voice is a Conversation **input device**, not a desktop Capability Provider.
//! It never invokes other providers and never owns orchestration.
//!
//! P16.8 Conversation Continuity: listening uses ContinuousRecognitionSession so
//! natural pauses do not end the session (RecognizeAsync + short EndSilenceTimeout did).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Windows SPERR_PRIVACY_STATEMENT_DECLINED — speech privacy not accepted.
pub const HRESULT_SPEECH_PRIVACY_DECLINED: u32 = 0x8004_5509;

const SPEECH_PRIVACY_MESSAGE: &str = "Windows needs speech privacy turned on before I can listen. Click the microphone once and I’ll open the right Settings page — then come back here.";

const MICROPHONE_PERMISSION_MESSAGE: &str = "Workspace can’t use the microphone yet. Click the microphone once and I’ll open Windows Settings so you can allow access — then come back here.";

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
        || lower.contains("privacy-microphone")
        || (lower.contains("microphone")
            && (lower.contains("denied")
                || lower.contains("not allowed")
                || lower.contains("unavailable")
                || lower.contains("access")))
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
        message: "I couldn’t listen just now. Try the microphone again — if it keeps failing, click once for Settings help.".into(),
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
    /// Continuous listen turn (Conversation Continuity).
    /// `on_ready` fires only when Capturing (Ready contract — speech will be retained).
    /// `on_sound_started` fires on SpeechDetected / SoundStarted (Listening UI).
    fn listen_once_when_ready(
        &self,
        on_ready: Box<dyn FnOnce() + Send>,
        on_sound_started: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    ) -> Result<VoiceListenOutcome>;
    fn listen_once(&self) -> Result<VoiceListenOutcome> {
        self.listen_once_when_ready(Box::new(|| {}), None)
    }
    fn cancel(&self) -> Result<()>;
    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()>;
    /// Clear cached mic probe and re-detect (Permission Guidance — after Settings return).
    fn recheck_microphone(&self) -> Result<VoiceCapabilityStatus> {
        self.status()
    }
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
        on_sound_started: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    ) -> Result<VoiceListenOutcome> {
        self.cancelled.store(false, Ordering::SeqCst);
        let _ = self.warm_up();
        if self.fail_permission.load(Ordering::SeqCst) {
            // Permission Guidance: never auto-open Settings — Conversation guides the user.
            return Ok(VoiceListenOutcome {
                ok: false,
                transcript: None,
                status: "permission_denied".into(),
                message: MICROPHONE_PERMISSION_MESSAGE.into(),
            });
        }
        // Tests: capturing contract is immediate after warm.
        on_ready();
        if let Some(sound) = on_sound_started {
            sound();
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

    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()> {
        open_windows_settings_uri(target.as_settings_uri())
    }

    fn recheck_microphone(&self) -> Result<VoiceCapabilityStatus> {
        if self.fail_permission.load(Ordering::SeqCst) {
            return self.status();
        }
        self.status()
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
        _on_sound_started: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
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
    /// User asked to end the session (mic toggle / cancel). Finalizes via StopAsync.
    cancel_requested: AtomicBool,
    warmed: AtomicBool,
    #[cfg(windows)]
    engine: Mutex<Option<WinrtVoiceEngine>>,
    #[cfg(windows)]
    active_session: Mutex<Option<windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession>>,
    #[cfg(windows)]
    mic_access: Mutex<Option<MicAccess>>,
    /// Serializes warm / recover so startup + UI warm never contend on WinRT.
    #[cfg(windows)]
    warm_lock: Mutex<()>,
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
            active_session: Mutex::new(None),
            #[cfg(windows)]
            mic_access: Mutex::new(None),
            #[cfg(windows)]
            warm_lock: Mutex::new(()),
        }
    }
}

impl VoicePort for SystemVoicePort {
    /// Lightweight snapshot — never compiles a recognizer or opens MediaCapture.
    fn status(&self) -> Result<VoiceCapabilityStatus> {
        #[cfg(windows)]
        {
            let warmed = self.warmed.load(Ordering::SeqCst);
            let mic = peek_microphone_access(&self.mic_access);
            let denied = matches!(
                mic,
                MicAccess::ConfirmedDenied | MicAccess::Denied
            );
            if !warmed {
                return Ok(VoiceCapabilityStatus {
                    available: !denied,
                    microphone_available: !denied,
                    recognition_available: true,
                    permission: if denied {
                        "denied".into()
                    } else if matches!(mic, MicAccess::Allowed) {
                        "granted".into()
                    } else {
                        "prompt".into()
                    },
                    message: "Voice is preparing…".into(),
                    warmed: false,
                });
            }
            Ok(match mic {
                MicAccess::Allowed => VoiceCapabilityStatus {
                    available: true,
                    microphone_available: true,
                    recognition_available: true,
                    permission: "granted".into(),
                    message: "Voice is ready.".into(),
                    warmed: true,
                },
                MicAccess::ConfirmedDenied | MicAccess::Denied => VoiceCapabilityStatus {
                    available: false,
                    microphone_available: false,
                    recognition_available: true,
                    permission: "denied".into(),
                    message: MICROPHONE_PERMISSION_MESSAGE.into(),
                    warmed: true,
                },
                MicAccess::Unknown => VoiceCapabilityStatus {
                    available: true,
                    microphone_available: true,
                    recognition_available: true,
                    permission: "prompt".into(),
                    message: "Voice is ready.".into(),
                    warmed: true,
                },
            })
        }
        #[cfg(not(windows))]
        {
            UnavailableVoicePort.status()
        }
    }

    fn warm_up(&self) -> Result<()> {
        #[cfg(windows)]
        {
            let _warm = self
                .warm_lock
                .lock()
                .map_err(|_| WindowsIntegrationError::VoiceFailed("voice warm lock poisoned".into()))?;
            let t0 = std::time::Instant::now();
            log::info!("voice.lifecycle: warm_up_begin");
            // P16.14: warm compiles the recognizer only.
            // MediaCapture probes race SpeechRecognizer for exclusive mic access and
            // produced “I couldn’t listen just now” after previously successful runs.
            winrt_warm_up(&self.engine, &self.warmed)?;
            log::info!("voice.lifecycle: warm_up_done +{:?}", t0.elapsed());
            Ok(())
        }
        #[cfg(not(windows))]
        {
            Ok(())
        }
    }

    fn listen_once_when_ready(
        &self,
        on_ready: Box<dyn FnOnce() + Send>,
        on_sound_started: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    ) -> Result<VoiceListenOutcome> {
        self.cancel_requested.store(false, Ordering::SeqCst);
        #[cfg(windows)]
        {
            // Never hard-block listen on MediaCapture cache.
            // ConfirmedDenied only after SpeechRecognizer MicrophoneUnavailable.
            if matches!(
                self.mic_access.lock().ok().and_then(|g| *g),
                Some(MicAccess::ConfirmedDenied)
            ) {
                return Ok(VoiceListenOutcome {
                    ok: false,
                    transcript: None,
                    status: "permission_denied".into(),
                    message: MICROPHONE_PERMISSION_MESSAGE.into(),
                });
            }
            let outcome = winrt_listen_continuous_when_ready(
                &self.engine,
                &self.warmed,
                &self.active_session,
                &self.cancel_requested,
                on_ready,
                on_sound_started,
            )?;
            // P16.14: never reset the engine on idle outcomes (no_speech / cancelled).
            // P16.13 reset-on-any-failure forced cold recompile every quiet click →
            // latency + “I couldn’t listen just now” after previously working runs.
            if outcome.ok {
                if let Ok(mut guard) = self.mic_access.lock() {
                    *guard = Some(MicAccess::Allowed);
                }
            } else if matches!(
                outcome.status.as_str(),
                "no_speech" | "cancelled"
            ) {
                log::info!(
                    "voice.lifecycle: listen_idle_keep_engine status={}",
                    outcome.status
                );
            } else if outcome.status == "microphone_unavailable" {
                log::warn!("voice.lifecycle: listen_fail_recover status=microphone_unavailable");
                if let Ok(mut guard) = self.mic_access.lock() {
                    *guard = Some(MicAccess::ConfirmedDenied);
                }
                reset_voice_engine(&self.engine, &self.warmed, &self.active_session);
            } else if matches!(
                outcome.status.as_str(),
                "recognition_failed" | "recognition_unavailable"
            ) {
                log::warn!(
                    "voice.lifecycle: listen_fail_recover status={} — resetting engine",
                    outcome.status
                );
                reset_voice_engine(&self.engine, &self.warmed, &self.active_session);
            } else {
                log::info!(
                    "voice.lifecycle: listen_fail_keep_engine status={}",
                    outcome.status
                );
            }
            Ok(outcome)
        }
        #[cfg(not(windows))]
        {
            let _ = on_ready;
            let _ = on_sound_started;
            UnavailableVoicePort.listen_once()
        }
    }

    fn cancel(&self) -> Result<()> {
        // Mic toggle while listening = finish the turn (keep transcript), not discard.
        self.cancel_requested.store(true, Ordering::SeqCst);
        #[cfg(windows)]
        {
            if let Ok(guard) = self.active_session.lock() {
                if let Some(session) = guard.as_ref() {
                    let _ = session.StopAsync();
                }
            }
        }
        Ok(())
    }

    fn recheck_microphone(&self) -> Result<VoiceCapabilityStatus> {
        #[cfg(windows)]
        {
            let _warm = self
                .warm_lock
                .lock()
                .map_err(|_| WindowsIntegrationError::VoiceFailed("voice warm lock poisoned".into()))?;
            let t0 = std::time::Instant::now();
            log::info!("voice.lifecycle: permission_recheck_begin");
            if let Ok(mut guard) = self.mic_access.lock() {
                *guard = None;
            }
            // Recheck may confirm Allowed; MediaCapture Denied stays non-sticky.
            let mic = probe_microphone_access_soft(&self.mic_access);
            if matches!(mic, MicAccess::Allowed) {
                // Successful Settings return — ensure engine is ready.
                let _ = winrt_warm_up(&self.engine, &self.warmed);
            }
            let status = self.status()?;
            log::info!(
                "voice.lifecycle: permission_recheck_done permission={} mic={mic:?} +{:?}",
                status.permission,
                t0.elapsed()
            );
            Ok(status)
        }
        #[cfg(not(windows))]
        {
            self.status()
        }
    }

    fn open_settings(&self, target: VoiceSettingsTarget) -> Result<()> {
        // Next return from Settings must re-probe — never trust a stale deny cache.
        #[cfg(windows)]
        if let Ok(mut guard) = self.mic_access.lock() {
            *guard = None;
        }
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
    /// Only set after SpeechRecognizer reports MicrophoneUnavailable (not MediaCapture).
    ConfirmedDenied,
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
fn peek_microphone_access(cache: &Mutex<Option<MicAccess>>) -> MicAccess {
    cache
        .lock()
        .ok()
        .and_then(|g| *g)
        .unwrap_or(MicAccess::Unknown)
}

/// Soft probe used at warm / Settings return.
/// **Never persists MediaCapture Denied** — that was the P16.13 regression
/// (false “mic unavailable” while Windows mic still worked for other apps).
#[cfg(windows)]
fn probe_microphone_access_soft(cache: &Mutex<Option<MicAccess>>) -> MicAccess {
    if let Ok(guard) = cache.lock() {
        if let Some(MicAccess::Allowed) = *guard {
            return MicAccess::Allowed;
        }
        if let Some(MicAccess::ConfirmedDenied) = *guard {
            // Cleared only by recheck (cache reset) or successful listen.
        }
    }
    let t0 = std::time::Instant::now();
    log::info!("voice.lifecycle: mic_probe_begin");
    let value = probe_microphone_access();
    log::info!(
        "voice.lifecycle: mic_probe_done result={value:?} sticky={} +{:?}",
        matches!(value, MicAccess::Allowed),
        t0.elapsed()
    );
    if let Ok(mut guard) = cache.lock() {
        match value {
            MicAccess::Allowed => *guard = Some(MicAccess::Allowed),
            MicAccess::Denied => {
                // Soft deny for logging only — leave Unknown so listen can still try.
                if !matches!(*guard, Some(MicAccess::Allowed) | Some(MicAccess::ConfirmedDenied))
                {
                    *guard = None;
                }
                log::warn!(
                    "voice.lifecycle: mic_probe_denied_soft — not caching (prevents false block)"
                );
            }
            MicAccess::Unknown | MicAccess::ConfirmedDenied => {}
        }
    }
    if matches!(value, MicAccess::Denied) {
        MicAccess::Unknown
    } else {
        value
    }
}

#[cfg(windows)]
fn reset_voice_engine(
    engine: &Mutex<Option<WinrtVoiceEngine>>,
    warmed: &AtomicBool,
    active_session: &Mutex<
        Option<windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession>,
    >,
) {
    if let Ok(mut guard) = active_session.lock() {
        *guard = None;
    }
    if let Ok(mut guard) = engine.lock() {
        *guard = None;
    }
    warmed.store(false, Ordering::SeqCst);
    log::info!("voice.lifecycle: engine_reset");
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

    // Initial silence: allow the user to settle after Ready.
    // End silence is not the primary stop for P16.8 — ContinuousRecognitionSession
    // AutoStopSilenceTimeout owns "user finished speaking" after natural pauses.
    if let Ok(timeouts) = recognizer.Timeouts() {
        let _ = timeouts.SetInitialSilenceTimeout(TimeSpan {
            Duration: 15_i64 * 10_000_000,
        });
        let _ = timeouts.SetEndSilenceTimeout(TimeSpan {
            Duration: 10_i64 * 10_000_000,
        });
        // Keep babble generous — continuous session owns finish semantics.
        let _ = timeouts.SetBabbleTimeout(TimeSpan {
            Duration: 20_i64 * 10_000_000,
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
    let t0 = std::time::Instant::now();
    log::info!("voice.lifecycle: compile_recognizer_begin");
    let recognizer = create_compiled_recognizer().map_err(|error| {
        WindowsIntegrationError::VoiceFailed(sanitize_setup_message(&error.to_string()))
    })?;
    log::info!(
        "voice.lifecycle: compile_recognizer_done +{:?}",
        t0.elapsed()
    );
    if let Ok(mut guard) = engine.lock() {
        *guard = Some(WinrtVoiceEngine { recognizer });
    }
    warmed.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(windows)]
fn outcome_from_winrt_error(error: windows::core::Error) -> VoiceListenOutcome {
    // Permission Guidance Principle: never auto-open Windows Settings from failures.
    classify_speech_failure(error.code().0 as u32, &error.to_string())
}

/// P16.9 — continuous listen with Ready contract + session stitching.
/// Ready = Capturing only. Listening UI = SpeechDetected/SoundStarted.
/// WinRT AutoStop (max ~10s silence) stitches until user Stop or 5 minutes.
#[cfg(windows)]
fn winrt_listen_continuous_when_ready(
    engine: &Mutex<Option<WinrtVoiceEngine>>,
    warmed: &AtomicBool,
    active_session: &Mutex<
        Option<windows::Media::SpeechRecognition::SpeechContinuousRecognitionSession>,
    >,
    cancel_requested: &AtomicBool,
    on_ready: Box<dyn FnOnce() + Send>,
    on_sound_started: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
) -> Result<VoiceListenOutcome> {
    use std::sync::{Arc, Condvar, Mutex as StdMutex};
    use std::time::{Duration, Instant};
    use windows::Foundation::{TimeSpan, TypedEventHandler};
    use windows::Media::SpeechRecognition::{
        SpeechContinuousRecognitionCompletedEventArgs, SpeechContinuousRecognitionMode,
        SpeechContinuousRecognitionResultGeneratedEventArgs, SpeechContinuousRecognitionSession,
        SpeechRecognitionResultStatus, SpeechRecognizerState, SpeechRecognizerStateChangedEventArgs,
    };

    const MAX_WALL: Duration = Duration::from_secs(5 * 60);
    let t0 = Instant::now();
    log::info!("voice.lifecycle: click_to_warm_begin");

    if let Err(error) = winrt_warm_up(engine, warmed) {
        log::warn!("voice.lifecycle: warm_failed +{:?}", t0.elapsed());
        return Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: "recognition_unavailable".into(),
            message: sanitize_setup_message(&error.to_string()),
        });
    }
    log::info!("voice.lifecycle: warm_done +{:?}", t0.elapsed());

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

    let parts: Arc<StdMutex<Vec<String>>> = Arc::new(StdMutex::new(Vec::new()));
    let speech_hook = on_sound_started.clone();
    let speech_fired = Arc::new(AtomicBool::new(false));
    let ready_emitted = Arc::new(AtomicBool::new(false));
    let on_ready_cell: Arc<StdMutex<Option<Box<dyn FnOnce() + Send>>>> =
        Arc::new(StdMutex::new(Some(on_ready)));

    let capture_gate = Arc::new((StdMutex::new(false), Condvar::new()));
    let gate_for_handler = Arc::clone(&capture_gate);
    let speech_hook_state = speech_hook.clone();
    let speech_fired_state = Arc::clone(&speech_fired);
    let state_token = recognizer.StateChanged(&TypedEventHandler::<
        windows::Media::SpeechRecognition::SpeechRecognizer,
        SpeechRecognizerStateChangedEventArgs,
    >::new(move |_sender, args| {
        let Some(args) = args else {
            return Ok(());
        };
        let Ok(state) = args.State() else {
            return Ok(());
        };
        log::info!(
            "voice.lifecycle: state={state:?} +{elapsed:?}",
            elapsed = t0.elapsed()
        );
        // Ready contract: Capturing alone proves audio capture exists.
        if state == SpeechRecognizerState::Capturing {
            let (lock, cvar) = &*gate_for_handler;
            if let Ok(mut ready) = lock.lock() {
                if !*ready {
                    *ready = true;
                    cvar.notify_one();
                }
            }
        }
        if state == SpeechRecognizerState::SpeechDetected
            || state == SpeechRecognizerState::SoundStarted
        {
            if !speech_fired_state.swap(true, Ordering::SeqCst) {
                log::info!("voice.lifecycle: speech_detected +{:?}", t0.elapsed());
                if let Some(hook) = speech_hook_state.as_ref() {
                    hook();
                }
            }
        }
        Ok(())
    }));

    let mut hard_fail: Option<VoiceListenOutcome> = None;
    let mut stitch: u32 = 0;

    while !cancel_requested.load(Ordering::SeqCst) && t0.elapsed() < MAX_WALL {
        if hard_fail.is_some() {
            break;
        }
        stitch += 1;
        log::info!(
            "voice.lifecycle: stitch={stitch} begin +{:?}",
            t0.elapsed()
        );

        let session = match recognizer.ContinuousRecognitionSession() {
            Ok(s) => s,
            Err(error) => {
                hard_fail = Some(outcome_from_winrt_error(error));
                break;
            }
        };

        let _ = session.SetAutoStopSilenceTimeout(TimeSpan {
            Duration: 10_i64 * 10_000_000,
        });

        let parts_for_result = Arc::clone(&parts);
        let speech_hook_result = speech_hook.clone();
        let speech_fired_result = Arc::clone(&speech_fired);
        let result_token = session.ResultGenerated(&TypedEventHandler::<
            SpeechContinuousRecognitionSession,
            SpeechContinuousRecognitionResultGeneratedEventArgs,
        >::new(move |_session, args| {
            let Some(args) = args else {
                return Ok(());
            };
            let Ok(result) = args.Result() else {
                return Ok(());
            };
            if let Ok(status) = result.Status() {
                if status != SpeechRecognitionResultStatus::Success {
                    return Ok(());
                }
            }
            if let Ok(text) = result.Text() {
                let piece = text.to_string().trim().to_string();
                if !piece.is_empty() {
                    if !speech_fired_result.swap(true, Ordering::SeqCst) {
                        if let Some(hook) = speech_hook_result.as_ref() {
                            hook();
                        }
                    }
                    if let Ok(mut guard) = parts_for_result.lock() {
                        guard.push(piece);
                    }
                }
            }
            Ok(())
        }));

        let session_done = Arc::new((StdMutex::new(false), Condvar::new()));
        let complete_status: Arc<StdMutex<Option<SpeechRecognitionResultStatus>>> =
            Arc::new(StdMutex::new(None));
        let done_for_complete = Arc::clone(&session_done);
        let status_for_complete = Arc::clone(&complete_status);
        let complete_token = session.Completed(&TypedEventHandler::<
            SpeechContinuousRecognitionSession,
            SpeechContinuousRecognitionCompletedEventArgs,
        >::new(move |_session, args| {
            if let Some(args) = args {
                if let Ok(status) = args.Status() {
                    if let Ok(mut guard) = status_for_complete.lock() {
                        *guard = Some(status);
                    }
                }
            }
            let (lock, cvar) = &*done_for_complete;
            if let Ok(mut done) = lock.lock() {
                *done = true;
                cvar.notify_one();
            }
            Ok(())
        }));

        if let Ok(mut guard) = active_session.lock() {
            *guard = Some(session.clone());
        }

        log::info!(
            "voice.lifecycle: continuous_start stitch={stitch} +{:?}",
            t0.elapsed()
        );
        if let Err(error) = session
            .StartWithModeAsync(SpeechContinuousRecognitionMode::Default)
            .and_then(|op| op.get())
        {
            if let Ok(mut guard) = active_session.lock() {
                *guard = None;
            }
            if let Ok(token) = result_token {
                let _ = session.RemoveResultGenerated(token);
            }
            if let Ok(token) = complete_token {
                let _ = session.RemoveCompleted(token);
            }
            // Later stitches: prefer returning speech already captured over crashing the turn.
            if stitch > 1 {
                log::warn!(
                    "voice.lifecycle: stitch_start_failed_soft +{:?} — finalizing transcript",
                    t0.elapsed()
                );
                break;
            }
            hard_fail = Some(outcome_from_winrt_error(error));
            break;
        }
        log::info!(
            "voice.lifecycle: continuous_started stitch={stitch} +{:?}",
            t0.elapsed()
        );

        if let Ok(state) = recognizer.State() {
            if state == SpeechRecognizerState::Capturing {
                let (lock, cvar) = &*capture_gate;
                if let Ok(mut ready) = lock.lock() {
                    *ready = true;
                    cvar.notify_one();
                }
            }
        }

        if !ready_emitted.load(Ordering::SeqCst) {
            let (lock, cvar) = &*capture_gate;
            if let Ok(mut ready) = lock.lock() {
                let deadline = Duration::from_millis(2500);
                while !*ready {
                    let (guard, wait) = cvar
                        .wait_timeout(ready, deadline)
                        .unwrap_or_else(|e| e.into_inner());
                    ready = guard;
                    if wait.timed_out() {
                        log::warn!(
                            "voice.lifecycle: capturing_wait_timeout +{:?}",
                            t0.elapsed()
                        );
                        break;
                    }
                }
                log::info!(
                    "voice.lifecycle: capturing_contract ready={} +{:?}",
                    *ready,
                    t0.elapsed()
                );
            }
            // Brief settle after Capturing so first frames are retained before Ready.
            std::thread::sleep(Duration::from_millis(45));
            if let Ok(mut cell) = on_ready_cell.lock() {
                if let Some(cb) = cell.take() {
                    cb();
                    ready_emitted.store(true, Ordering::SeqCst);
                    log::info!("voice.lifecycle: on_ready_emitted +{:?}", t0.elapsed());
                }
            }
        }

        {
            let (lock, cvar) = &*session_done;
            if let Ok(mut done) = lock.lock() {
                while !*done {
                    if cancel_requested.load(Ordering::SeqCst) {
                        log::info!("voice.lifecycle: user_stop +{:?}", t0.elapsed());
                        let _ = session.StopAsync().and_then(|op| op.get());
                    }
                    let (guard, wait) = cvar
                        .wait_timeout(done, Duration::from_millis(250))
                        .unwrap_or_else(|e| e.into_inner());
                    done = guard;
                    if *done {
                        break;
                    }
                    if cancel_requested.load(Ordering::SeqCst) && wait.timed_out() {
                        *done = true;
                        break;
                    }
                    if t0.elapsed() >= MAX_WALL {
                        let _ = session.StopAsync().and_then(|op| op.get());
                    }
                }
            }
        }

        if let Ok(mut guard) = active_session.lock() {
            *guard = None;
        }
        if let Ok(token) = result_token {
            let _ = session.RemoveResultGenerated(token);
        }
        if let Ok(token) = complete_token {
            let _ = session.RemoveCompleted(token);
        }

        let finished_status = complete_status.lock().ok().and_then(|g| *g);
        log::info!(
            "voice.lifecycle: stitch={stitch} complete status={finished_status:?} +{:?}",
            t0.elapsed()
        );

        if let Some(status) = finished_status {
            match status {
                SpeechRecognitionResultStatus::MicrophoneUnavailable => {
                    hard_fail = Some(VoiceListenOutcome {
                        ok: false,
                        transcript: None,
                        status: "microphone_unavailable".into(),
                        message: MICROPHONE_PERMISSION_MESSAGE.into(),
                    });
                    break;
                }
                SpeechRecognitionResultStatus::NetworkFailure => {
                    hard_fail = Some(VoiceListenOutcome {
                        ok: false,
                        transcript: None,
                        status: "recognition_unavailable".into(),
                        message: "Speech recognition isn’t available right now. Check your network and try again.".into(),
                    });
                    break;
                }
                SpeechRecognitionResultStatus::AudioQualityFailure => {
                    // Not a permission deny — must not become ConfirmedDenied.
                    hard_fail = Some(VoiceListenOutcome {
                        ok: false,
                        transcript: None,
                        status: "audio_quality".into(),
                        message: "I couldn’t hear the microphone clearly. Try again.".into(),
                    });
                    break;
                }
                _ => {}
            }
        }

        if cancel_requested.load(Ordering::SeqCst) || t0.elapsed() >= MAX_WALL {
            break;
        }
        // Let WinRT fully release the prior session before restart (crash prevention).
        std::thread::sleep(Duration::from_millis(280));
        if cancel_requested.load(Ordering::SeqCst) {
            break;
        }
        log::info!("voice.lifecycle: stitch_resume +{:?}", t0.elapsed());
    }

    if let Ok(token) = state_token {
        let _ = recognizer.RemoveStateChanged(token);
    }
    if let Ok(mut guard) = active_session.lock() {
        *guard = None;
    }

    log::info!(
        "voice.lifecycle: continuous_complete stitches={stitch} +{:?}",
        t0.elapsed()
    );

    if let Some(fail) = hard_fail {
        let transcript = parts
            .lock()
            .map(|g| g.join(" ").trim().to_string())
            .unwrap_or_default();
        // Prefer truthful speech already captured over a hard empty failure.
        if !transcript.is_empty() {
            return Ok(VoiceListenOutcome {
                ok: true,
                transcript: Some(transcript),
                status: "recognized".into(),
                message: "Got it.".into(),
            });
        }
        return Ok(fail);
    }

    let transcript = parts
        .lock()
        .map(|g| g.join(" ").trim().to_string())
        .unwrap_or_default();

    if transcript.is_empty() {
        let user_stopped = cancel_requested.load(Ordering::SeqCst);
        return Ok(VoiceListenOutcome {
            ok: false,
            transcript: None,
            status: if user_stopped {
                "cancelled".into()
            } else {
                "no_speech".into()
            },
            message: if user_stopped {
                "Listening stopped.".into()
            } else {
                "I didn’t catch that. Try again when you’re ready.".into()
            },
        });
    }

    Ok(VoiceListenOutcome {
        ok: true,
        transcript: Some(transcript),
        status: "recognized".into(),
        message: "Got it.".into(),
    })
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
            .listen_once_when_ready(
                Box::new(move || {
                    notified_ready.store(true, Ordering::SeqCst);
                }),
                None,
            )
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
