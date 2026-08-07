//! Temporary Product Proof instrumentation for Voice (P16.23).
//!
//! Enable with `WORKSPACE_VOICE_PRODUCT_PROOF=1`.
//! Ordinary users never see this surface — no Conversation copy, no UI chrome.
//! Remove after P16 permanently closes (Repository Quality Before Milestone Closure).

use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use serde::Serialize;

static SESSION_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize)]
pub struct VoiceProofEvent {
    pub name: String,
    pub ms_from_click: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoiceProofReport {
    pub session_id: u64,
    pub enabled: bool,
    pub cold_start: bool,
    pub recognizer_reused: bool,
    pub recognizer_recreated: bool,
    pub permission_classification: Option<String>,
    pub reset_reason: Option<String>,
    pub recovery_reason: Option<String>,
    pub retry_reason: Option<String>,
    pub final_status: String,
    pub events: Vec<VoiceProofEvent>,
    /// Measured intervals in milliseconds (None = event missing).
    pub click_to_capturing_ms: Option<u64>,
    pub click_to_ready_ms: Option<u64>,
    pub ready_to_first_speech_ms: Option<u64>,
    pub first_speech_to_first_token_ms: Option<u64>,
    pub total_recognition_ms: Option<u64>,
    pub report_path: Option<String>,
}

struct ActiveSession {
    id: u64,
    t0: Instant,
    cold_start: bool,
    recognizer_reused: bool,
    recognizer_recreated: bool,
    permission_classification: Option<String>,
    reset_reason: Option<String>,
    recovery_reason: Option<String>,
    retry_reason: Option<String>,
    events: Vec<VoiceProofEvent>,
}

static ACTIVE: Mutex<Option<ActiveSession>> = Mutex::new(None);
static LAST_REPORT: Mutex<Option<VoiceProofReport>> = Mutex::new(None);

pub fn voice_proof_enabled() -> bool {
    matches!(
        std::env::var("WORKSPACE_VOICE_PRODUCT_PROOF").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

pub fn proof_begin_listen(cold_start: bool) {
    if !voice_proof_enabled() {
        return;
    }
    let id = SESSION_SEQ.fetch_add(1, Ordering::SeqCst);
    let session = ActiveSession {
        id,
        t0: Instant::now(),
        cold_start,
        recognizer_reused: !cold_start,
        recognizer_recreated: cold_start,
        permission_classification: None,
        reset_reason: None,
        recovery_reason: None,
        retry_reason: None,
        events: vec![VoiceProofEvent {
            name: "mic_click".into(),
            ms_from_click: 0,
        }],
    };
    if let Ok(mut guard) = ACTIVE.lock() {
        *guard = Some(session);
    }
    log::info!("voice.proof: begin session={id} cold={cold_start}");
}

pub fn proof_mark(name: &str) {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            // First occurrence only for milestone marks (avoid stitch spam).
            if matches!(
                name,
                "capturing_entered"
                    | "ready_emitted"
                    | "first_speech_detected"
                    | "first_token_received"
                    | "transcript_completed"
                    | "compile_start"
                    | "compile_finish"
                    | "warm_start"
                    | "warm_already"
            ) && session.events.iter().any(|e| e.name == name)
            {
                return;
            }
            let ms = session.t0.elapsed().as_millis() as u64;
            session.events.push(VoiceProofEvent {
                name: name.into(),
                ms_from_click: ms,
            });
            log::info!("voice.proof: mark={name} +{ms}ms session={}", session.id);
        }
    }
}

pub fn proof_note_permission(classification: &str) {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.permission_classification = Some(classification.into());
        }
    }
    proof_mark(&format!("permission_{classification}"));
}

pub fn proof_note_reset(reason: &str) {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.reset_reason = Some(reason.into());
            session.recognizer_recreated = true;
            session.recognizer_reused = false;
        }
    }
    proof_mark(&format!("engine_reset:{reason}"));
}

pub fn proof_note_recovery(reason: &str) {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.recovery_reason = Some(reason.into());
        }
    }
    proof_mark(&format!("recovery:{reason}"));
}

pub fn proof_note_retry(reason: &str) {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.retry_reason = Some(reason.into());
        }
    }
    proof_mark(&format!("retry:{reason}"));
}

pub fn proof_mark_reused() {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.recognizer_reused = true;
            session.recognizer_recreated = false;
            session.cold_start = false;
        }
    }
    proof_mark("warm_already");
}

pub fn proof_mark_recreated() {
    if !voice_proof_enabled() {
        return;
    }
    if let Ok(mut guard) = ACTIVE.lock() {
        if let Some(session) = guard.as_mut() {
            session.recognizer_recreated = true;
            session.recognizer_reused = false;
            session.cold_start = true;
        }
    }
    proof_mark("warm_start");
}

fn ms_of(events: &[VoiceProofEvent], name: &str) -> Option<u64> {
    events
        .iter()
        .find(|e| e.name == name)
        .map(|e| e.ms_from_click)
}

fn build_report(session: ActiveSession, final_status: &str) -> VoiceProofReport {
    let click_to_capturing_ms = ms_of(&session.events, "capturing_entered");
    let click_to_ready_ms = ms_of(&session.events, "ready_emitted");
    let ready_ms = click_to_ready_ms;
    let speech_ms = ms_of(&session.events, "first_speech_detected");
    let token_ms = ms_of(&session.events, "first_token_received");
    let complete_ms = ms_of(&session.events, "transcript_completed")
        .or_else(|| ms_of(&session.events, "listen_finished"));

    let ready_to_first_speech_ms = match (ready_ms, speech_ms) {
        (Some(r), Some(s)) if s >= r => Some(s - r),
        _ => None,
    };
    let first_speech_to_first_token_ms = match (speech_ms, token_ms) {
        (Some(s), Some(t)) if t >= s => Some(t - s),
        _ => None,
    };

    let mut report = VoiceProofReport {
        session_id: session.id,
        enabled: true,
        cold_start: session.cold_start,
        recognizer_reused: session.recognizer_reused,
        recognizer_recreated: session.recognizer_recreated,
        permission_classification: session.permission_classification,
        reset_reason: session.reset_reason,
        recovery_reason: session.recovery_reason,
        retry_reason: session.retry_reason,
        final_status: final_status.into(),
        events: session.events,
        click_to_capturing_ms,
        click_to_ready_ms,
        ready_to_first_speech_ms,
        first_speech_to_first_token_ms,
        total_recognition_ms: complete_ms,
        report_path: None,
    };

    if let Some(path) = persist_report(&report) {
        report.report_path = Some(path);
    }

    log::info!(
        "voice.proof.report: session={} status={} cold={} reused={} recreated={} click_to_capturing_ms={:?} click_to_ready_ms={:?} ready_to_first_speech_ms={:?} first_speech_to_first_token_ms={:?} total_ms={:?} path={:?}",
        report.session_id,
        report.final_status,
        report.cold_start,
        report.recognizer_reused,
        report.recognizer_recreated,
        report.click_to_capturing_ms,
        report.click_to_ready_ms,
        report.ready_to_first_speech_ms,
        report.first_speech_to_first_token_ms,
        report.total_recognition_ms,
        report.report_path
    );

    report
}

fn proof_dir() -> PathBuf {
    std::env::temp_dir().join("workspace-voice-proof")
}

fn persist_report(report: &VoiceProofReport) -> Option<String> {
    let dir = proof_dir();
    if create_dir_all(&dir).is_err() {
        return None;
    }
    let path = dir.join(format!("session-{}.json", report.session_id));
    let json = serde_json::to_string_pretty(report).ok()?;
    std::fs::write(&path, json).ok()?;
    let jsonl = dir.join("sessions.jsonl");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&jsonl) {
        if let Ok(line) = serde_json::to_string(report) {
            let _ = writeln!(file, "{line}");
        }
    }
    Some(path.display().to_string())
}

pub fn proof_end_listen(final_status: &str) -> Option<VoiceProofReport> {
    if !voice_proof_enabled() {
        return None;
    }
    let session = {
        let Ok(mut guard) = ACTIVE.lock() else {
            return None;
        };
        guard.take()
    }?;
    let report = build_report(session, final_status);
    if let Ok(mut last) = LAST_REPORT.lock() {
        *last = Some(report.clone());
    }
    Some(report)
}

pub fn proof_last_report() -> Option<VoiceProofReport> {
    LAST_REPORT.lock().ok().and_then(|g| g.clone())
}

pub fn proof_status_summary() -> String {
    if !voice_proof_enabled() {
        return "Voice Product Proof instrumentation is off (set WORKSPACE_VOICE_PRODUCT_PROOF=1)."
            .into();
    }
    match proof_last_report() {
        Some(r) => format!(
            "Last Voice proof session {}: click→capturing {:?}ms, click→ready {:?}ms, ready→speech {:?}ms, speech→token {:?}ms, total {:?}ms (cold={}, reused={}).",
            r.session_id,
            r.click_to_capturing_ms,
            r.click_to_ready_ms,
            r.ready_to_first_speech_ms,
            r.first_speech_to_first_token_ms,
            r.total_recognition_ms,
            r.cold_start,
            r.recognizer_reused
        ),
        None => format!(
            "Voice Product Proof instrumentation is on. Reports write to {}.",
            proof_dir().display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_disabled_by_default_is_noop() {
        // Default env in tests is off — marks must not panic.
        proof_mark("ready_emitted");
        assert!(proof_last_report().is_none() || !voice_proof_enabled());
    }

    #[test]
    fn proof_session_records_intervals_when_enabled() {
        std::env::set_var("WORKSPACE_VOICE_PRODUCT_PROOF", "1");
        proof_begin_listen(true);
        proof_mark_recreated();
        proof_mark("compile_start");
        proof_mark("compile_finish");
        proof_mark("capturing_entered");
        proof_mark("ready_emitted");
        proof_mark("first_speech_detected");
        proof_mark("first_token_received");
        proof_mark("transcript_completed");
        let report = proof_end_listen("recognized").expect("report");
        assert!(report.cold_start);
        assert!(report.recognizer_recreated);
        assert!(report.click_to_capturing_ms.is_some());
        assert!(report.click_to_ready_ms.is_some());
        assert!(report.ready_to_first_speech_ms.is_some());
        assert!(report.first_speech_to_first_token_ms.is_some());
        assert!(report.total_recognition_ms.is_some());
        std::env::remove_var("WORKSPACE_VOICE_PRODUCT_PROOF");
    }
}
