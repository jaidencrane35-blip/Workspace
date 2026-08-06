//! Consented local pilot measurement (PP-P01E / LEDGER-0013).
//!
//! Pilot evaluation data is distinct from product saved-context data. Nothing is
//! recorded until the participant consents to a named scope. Measurements are
//! participant-authored or explicitly confirmed — never ambient observation.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

/// Scope identifier participants consent to. Bumping this invalidates prior consent.
pub const PILOT_MEASUREMENT_SCOPE_ID: &str = "pilot-measurement-scope-v1";

const NOTES_MAX_CHARS: usize = 4000;
const RETURN_MINUTES_MAX: u32 = 24 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotScopeItem {
    pub key: String,
    pub summary: String,
}

impl PilotScopeItem {
    pub fn new(key: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            summary: summary.into(),
        }
    }
}

/// What pilot measurement includes and excludes — shown before consent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotMeasurementScope {
    pub id: String,
    pub purpose: String,
    pub measured: Vec<PilotScopeItem>,
    pub not_measured: Vec<PilotScopeItem>,
}

impl PilotMeasurementScope {
    pub fn current() -> Self {
        Self {
            id: PILOT_MEASUREMENT_SCOPE_ID.to_string(),
            purpose: "To evaluate whether Workspace helps you return to work faster \
                      after interruptions. These records are pilot evaluation data, \
                      stored only on this computer, and are separate from your saved \
                      contexts."
                .to_string(),
            measured: vec![
                PilotScopeItem::new(
                    "baseline",
                    "Your estimate of how many minutes returning to work usually takes \
                     without Workspace.",
                ),
                PilotScopeItem::new(
                    "leave_resume",
                    "Leave→resume times and correction notes you explicitly enter after \
                     using Resume.",
                ),
                PilotScopeItem::new(
                    "habit_days",
                    "Which local calendar days you chose to record a Resume for the pilot \
                     (for the week-four habit metric).",
                ),
                PilotScopeItem::new(
                    "interview",
                    "Optional written answers to baseline and week-four interview prompts.",
                ),
            ],
            not_measured: vec![
                PilotScopeItem::new(
                    "ambient",
                    "No background watching. Nothing is recorded while you work unless you \
                     enter it or confirm a pilot record.",
                ),
                PilotScopeItem::new(
                    "network",
                    "Nothing is uploaded. No telemetry, account, or remote reporting.",
                ),
                PilotScopeItem::new(
                    "desktop_contents",
                    "No screenshots, documents, keystrokes, or window contents beyond what \
                     you already saved under Save consent.",
                ),
                PilotScopeItem::new(
                    "product_merge",
                    "Pilot records are not merged into saved contexts or Memory.",
                ),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotConsent {
    pub scope_id: String,
    pub consented_at: String,
    pub withdrawn_at: Option<String>,
}

impl PilotConsent {
    pub fn is_active(&self) -> bool {
        self.withdrawn_at.is_none() && self.scope_id == PILOT_MEASUREMENT_SCOPE_ID
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotBaseline {
    pub return_minutes: u32,
    pub recorded_at: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotLeaveResumeRecord {
    pub id: String,
    pub recorded_at: String,
    /// Local calendar day (YYYY-MM-DD) for week-four habit counting.
    pub local_day: String,
    pub return_minutes: u32,
    pub correction_needed: bool,
    pub correction_note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum PilotInterviewPhase {
    Baseline,
    WeekFour,
}

impl PilotInterviewPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::WeekFour => "week_four",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PilotMeasurementError> {
        match value {
            "baseline" => Ok(Self::Baseline),
            "week_four" => Ok(Self::WeekFour),
            _ => Err(PilotMeasurementError::InvalidInterviewPhase),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotInterviewRecord {
    pub phase: PilotInterviewPhase,
    pub recorded_at: String,
    pub responses: String,
}

/// Read model for Experience — evaluation data only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PilotMeasurementSnapshot {
    pub scope: PilotMeasurementScope,
    pub consent: Option<PilotConsent>,
    pub baseline: Option<PilotBaseline>,
    pub leave_resume: Vec<PilotLeaveResumeRecord>,
    pub interview_baseline: Option<PilotInterviewRecord>,
    pub interview_week_four: Option<PilotInterviewRecord>,
    pub distinct_resume_days: u32,
    pub median_return_minutes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantPilotConsentRequest {
    pub approved_scope: String,
}

impl GrantPilotConsentRequest {
    pub fn validate(&self) -> Result<(), PilotMeasurementError> {
        if self.approved_scope.trim() != PILOT_MEASUREMENT_SCOPE_ID {
            return Err(PilotMeasurementError::ScopeMismatch {
                approved: self.approved_scope.trim().to_string(),
                current: PILOT_MEASUREMENT_SCOPE_ID.to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordPilotBaselineRequest {
    pub return_minutes: u32,
    pub notes: String,
}

impl RecordPilotBaselineRequest {
    pub fn validate(&self) -> Result<(), PilotMeasurementError> {
        validate_minutes(self.return_minutes)?;
        validate_notes(&self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordPilotLeaveResumeRequest {
    pub return_minutes: u32,
    pub correction_needed: bool,
    pub correction_note: String,
    /// Optional YYYY-MM-DD; when empty the host supplies the local day.
    pub local_day: String,
}

impl RecordPilotLeaveResumeRequest {
    pub fn validate(&self) -> Result<(), PilotMeasurementError> {
        validate_minutes(self.return_minutes)?;
        validate_notes(&self.correction_note)?;
        if !self.local_day.trim().is_empty() {
            validate_local_day(self.local_day.trim())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordPilotInterviewRequest {
    pub phase: PilotInterviewPhase,
    pub responses: String,
}

impl RecordPilotInterviewRequest {
    pub fn validate(&self) -> Result<(), PilotMeasurementError> {
        let trimmed = self.responses.trim();
        if trimmed.is_empty() {
            return Err(PilotMeasurementError::InterviewEmpty);
        }
        validate_notes(trimmed)?;
        Ok(())
    }
}

fn validate_minutes(minutes: u32) -> Result<(), PilotMeasurementError> {
    if minutes == 0 {
        return Err(PilotMeasurementError::MinutesMissing);
    }
    if minutes > RETURN_MINUTES_MAX {
        return Err(PilotMeasurementError::MinutesTooLarge {
            max: RETURN_MINUTES_MAX,
        });
    }
    Ok(())
}

fn validate_notes(notes: &str) -> Result<(), PilotMeasurementError> {
    if notes.chars().count() > NOTES_MAX_CHARS {
        return Err(PilotMeasurementError::NotesTooLong {
            max: NOTES_MAX_CHARS,
        });
    }
    Ok(())
}

fn validate_local_day(day: &str) -> Result<(), PilotMeasurementError> {
    let bytes = day.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes[0..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
        || !bytes[8..10].iter().all(u8::is_ascii_digit)
    {
        return Err(PilotMeasurementError::InvalidLocalDay);
    }
    Ok(())
}

/// Median of return minutes for leave→resume records (LEDGER-0013 success metric input).
pub fn median_return_minutes(values: &[u32]) -> Option<u32> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(sorted[mid])
    } else {
        let a = sorted[mid - 1] as u64;
        let b = sorted[mid] as u64;
        Some(((a + b) / 2) as u32)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PilotMeasurementError {
    #[error("pilot measurement requires consent to the current scope")]
    ConsentRequired,
    #[error("pilot measurement consent was withdrawn")]
    ConsentWithdrawn,
    #[error("approved scope {approved} does not match current scope {current}")]
    ScopeMismatch { approved: String, current: String },
    #[error("return minutes must be provided")]
    MinutesMissing,
    #[error("return minutes exceed the maximum of {max}")]
    MinutesTooLarge { max: u32 },
    #[error("notes exceed the maximum of {max} characters")]
    NotesTooLong { max: usize },
    #[error("interview responses cannot be empty")]
    InterviewEmpty,
    #[error("interview phase is not recognised")]
    InvalidInterviewPhase,
    #[error("local day must be YYYY-MM-DD")]
    InvalidLocalDay,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_refuses_stale_consent_ids() {
        let req = GrantPilotConsentRequest {
            approved_scope: "old".into(),
        };
        assert!(matches!(
            req.validate(),
            Err(PilotMeasurementError::ScopeMismatch { .. })
        ));
    }

    #[test]
    fn leave_resume_requires_minutes() {
        let req = RecordPilotLeaveResumeRequest {
            return_minutes: 0,
            correction_needed: false,
            correction_note: String::new(),
            local_day: String::new(),
        };
        assert_eq!(req.validate(), Err(PilotMeasurementError::MinutesMissing));
    }

    #[test]
    fn median_handles_even_counts() {
        assert_eq!(median_return_minutes(&[10, 20, 30, 40]), Some(25));
        assert_eq!(median_return_minutes(&[5]), Some(5));
        assert_eq!(median_return_minutes(&[]), None);
    }

    #[test]
    fn consent_inactive_when_withdrawn_or_stale_scope() {
        let active = PilotConsent {
            scope_id: PILOT_MEASUREMENT_SCOPE_ID.into(),
            consented_at: "t".into(),
            withdrawn_at: None,
        };
        assert!(active.is_active());
        let withdrawn = PilotConsent {
            withdrawn_at: Some("t2".into()),
            ..active.clone()
        };
        assert!(!withdrawn.is_active());
    }
}
