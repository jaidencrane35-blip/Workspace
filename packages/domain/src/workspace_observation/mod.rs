//! Workspace Observation Layer — durable desktop perception (Phase 6 / Sprint 103).
//!
//! System-scoped, versioned observation passes. Read-only. Never executes,
//! moves windows, or grants authority. Environment Model aggregates from here.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Observation-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceObservationError {
    #[error("observation pass id is required")]
    MissingPassId,

    #[error("observation captured_at is required")]
    MissingCapturedAt,

    #[error("observation source is required")]
    MissingSource,

    #[error("observation schema_version must be at least 1")]
    InvalidSchemaVersion,

    #[error("observation pass window_count does not match windows")]
    WindowCountMismatch,

    #[error("observation pass monitor_count does not match monitors")]
    MonitorCountMismatch,

    #[error("observation window pass_id mismatch")]
    WindowPassMismatch,

    #[error("observation monitor pass_id mismatch")]
    MonitorPassMismatch,

    #[error("observation window hwnd is required")]
    MissingWindowHwnd,

    #[error("observation snapshot validation failed: {0}")]
    Invalid(String),

    #[error("observation numeric field overflow: {0}")]
    NumericOverflow(String),

    #[error("observation layer cannot execute, move windows, or authorize")]
    CannotExecute,

    #[error("observation pass not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, WorkspaceObservationError>;

/// Confidence for cross-pass stable window identity (best-effort).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowIdentityConfidence {
    High,
    Medium,
    Low,
    Ephemeral,
}

impl WindowIdentityConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Ephemeral => "ephemeral",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            "ephemeral" => Ok(Self::Ephemeral),
            other => Err(WorkspaceObservationError::Invalid(format!(
                "unknown identity confidence: {other}"
            ))),
        }
    }
}

/// Header for one immutable desktop observation pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceObservationPass {
    pub id: String,
    pub captured_at: String,
    pub schema_version: i32,
    pub source: String,
    pub foreground_hwnd: Option<String>,
    pub window_count: i32,
    pub monitor_count: i32,
    pub duration_ms: Option<i32>,
    pub metadata_json: String,
    pub authority_effect: String,
}

impl WorkspaceObservationPass {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const DEFAULT_SCHEMA_VERSION: i32 = 1;
}

/// One observed monitor within a pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedMonitor {
    pub id: String,
    pub pass_id: String,
    pub monitor_index: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_w: i32,
    pub work_h: i32,
    pub is_primary: bool,
    pub dpi_scale: Option<f64>,
    pub authority_effect: String,
}

impl ObservedMonitor {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// One observed window within a pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedWindow {
    pub id: String,
    pub pass_id: String,
    pub hwnd: String,
    pub stable_window_id: Option<String>,
    pub title: String,
    pub process_id: i32,
    pub process_name: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_id: Option<String>,
    pub visible: bool,
    pub minimized: bool,
    pub focused: bool,
    pub z_order: Option<i32>,
    pub authority_effect: String,
}

impl ObservedWindow {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Cross-pass stable window identity registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationWindowIdentity {
    pub id: String,
    pub process_id: i32,
    pub title_fingerprint: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub last_hwnd: String,
    pub confidence: WindowIdentityConfidence,
    pub authority_effect: String,
}

impl ObservationWindowIdentity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Full observation snapshot: pass header plus child rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceObservationSnapshot {
    pub pass: WorkspaceObservationPass,
    pub windows: Vec<ObservedWindow>,
    pub monitors: Vec<ObservedMonitor>,
    pub identities: Vec<ObservationWindowIdentity>,
    pub authority_effect: String,
}

impl WorkspaceObservationSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Architecture guard — observation never executes.
    pub fn attempt_execute() -> Result<()> {
        Err(WorkspaceObservationError::CannotExecute)
    }

    /// Validates snapshot invariants before persistence or downstream use.
    pub fn validate(&self) -> Result<()> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceObservationError::Invalid(
                "snapshot authority_effect must be none".into(),
            ));
        }
        if self.pass.authority_effect != WorkspaceObservationPass::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceObservationError::Invalid(
                "pass authority_effect must be none".into(),
            ));
        }
        if self.pass.id.trim().is_empty() {
            return Err(WorkspaceObservationError::MissingPassId);
        }
        if self.pass.captured_at.trim().is_empty() {
            return Err(WorkspaceObservationError::MissingCapturedAt);
        }
        if self.pass.source.trim().is_empty() {
            return Err(WorkspaceObservationError::MissingSource);
        }
        if self.pass.schema_version < 1 {
            return Err(WorkspaceObservationError::InvalidSchemaVersion);
        }
        if self.pass.window_count as usize != self.windows.len() {
            return Err(WorkspaceObservationError::WindowCountMismatch);
        }
        if self.pass.monitor_count as usize != self.monitors.len() {
            return Err(WorkspaceObservationError::MonitorCountMismatch);
        }

        let monitor_ids: std::collections::HashSet<&str> =
            self.monitors.iter().map(|monitor| monitor.id.as_str()).collect();
        let focused_count = self.windows.iter().filter(|window| window.focused).count();
        if focused_count > 1 {
            return Err(WorkspaceObservationError::Invalid(
                "at most one window may be focused".into(),
            ));
        }
        if let Some(foreground) = self.pass.foreground_hwnd.as_deref() {
            let focused = self.windows.iter().find(|window| window.focused);
            match focused {
                Some(window) if window.hwnd != foreground => {
                    return Err(WorkspaceObservationError::Invalid(
                        "focused window hwnd does not match foreground_hwnd".into(),
                    ));
                }
                None => {
                    return Err(WorkspaceObservationError::Invalid(
                        "foreground_hwnd set but no focused window".into(),
                    ));
                }
                Some(_) => {}
            }
        } else if focused_count == 1 {
            return Err(WorkspaceObservationError::Invalid(
                "focused window present but foreground_hwnd is missing".into(),
            ));
        }

        for window in &self.windows {
            if window.pass_id != self.pass.id {
                return Err(WorkspaceObservationError::WindowPassMismatch);
            }
            if window.hwnd.trim().is_empty() {
                return Err(WorkspaceObservationError::MissingWindowHwnd);
            }
            if window.authority_effect != ObservedWindow::AUTHORITY_EFFECT_NONE {
                return Err(WorkspaceObservationError::Invalid(format!(
                    "window {} has authority",
                    window.id
                )));
            }
            if let Some(monitor_id) = window.monitor_id.as_deref() {
                if !monitor_ids.contains(monitor_id) {
                    return Err(WorkspaceObservationError::Invalid(format!(
                        "window {} references unknown monitor {}",
                        window.id, monitor_id
                    )));
                }
            }
        }

        for monitor in &self.monitors {
            if monitor.pass_id != self.pass.id {
                return Err(WorkspaceObservationError::MonitorPassMismatch);
            }
            if monitor.authority_effect != ObservedMonitor::AUTHORITY_EFFECT_NONE {
                return Err(WorkspaceObservationError::Invalid(format!(
                    "monitor {} has authority",
                    monitor.id
                )));
            }
        }

        for identity in &self.identities {
            if identity.authority_effect != ObservationWindowIdentity::AUTHORITY_EFFECT_NONE {
                return Err(WorkspaceObservationError::Invalid(format!(
                    "identity {} has authority",
                    identity.id
                )));
            }
        }

        // Identity linkage is enforced only when identities are present (capture path).
        // Loaded historical snapshots omit live registry enrichment and may carry
        // stable_window_id references without embedding identity rows.
        if !self.identities.is_empty() {
            let identity_ids: std::collections::HashSet<&str> = self
                .identities
                .iter()
                .map(|identity| identity.id.as_str())
                .collect();
            for window in &self.windows {
                if let Some(stable_id) = window.stable_window_id.as_deref() {
                    if !identity_ids.contains(stable_id) {
                        return Err(WorkspaceObservationError::Invalid(format!(
                            "window {} references unknown identity {}",
                            window.id, stable_id
                        )));
                    }
                }
            }
            for identity in &self.identities {
                if !self
                    .windows
                    .iter()
                    .any(|window| window.stable_window_id.as_deref() == Some(identity.id.as_str()))
                {
                    return Err(WorkspaceObservationError::Invalid(format!(
                        "identity {} is not linked to any window",
                        identity.id
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Checked conversion helpers for observation numeric fields.
pub fn observation_u64_to_i32(value: u64, field: &str) -> Result<i32> {
    i32::try_from(value).map_err(|_| {
        WorkspaceObservationError::NumericOverflow(format!("{field} exceeds i32 range"))
    })
}

pub fn observation_usize_to_i32(value: usize, field: &str) -> Result<i32> {
    i32::try_from(value).map_err(|_| {
        WorkspaceObservationError::NumericOverflow(format!("{field} exceeds i32 range"))
    })
}

pub fn observation_u32_to_i32(value: u32, field: &str) -> Result<i32> {
    i32::try_from(value).map_err(|_| {
        WorkspaceObservationError::NumericOverflow(format!("{field} exceeds i32 range"))
    })
}

/// Freshness of the latest observation pass (informational only — never triggers capture).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationFreshness {
    /// No observation pass exists.
    Unavailable,
    /// Within the fresh threshold.
    Fresh,
    /// Older than fresh, still within the recent window.
    Recent,
    /// At or beyond the stale threshold.
    Stale,
}

impl ObservationFreshness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
        }
    }
}

/// Default: younger than this many seconds is `Fresh`.
pub const OBSERVATION_FRESH_THRESHOLD_SECS: i64 = 60;
/// Default: at or beyond this many seconds is `Stale` (between fresh and stale is `Recent`).
pub const OBSERVATION_STALE_THRESHOLD_SECS: i64 = 300;

/// Classification for the last capture failure (read-only diagnostics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationCaptureErrorClass {
    WindowsIntegration,
    Validation,
    Persistence,
    Internal,
}

impl ObservationCaptureErrorClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WindowsIntegration => "windows_integration",
            Self::Validation => "validation",
            Self::Persistence => "persistence",
            Self::Internal => "internal",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "windows_integration" => Ok(Self::WindowsIntegration),
            "validation" => Ok(Self::Validation),
            "persistence" => Ok(Self::Persistence),
            "internal" => Ok(Self::Internal),
            other => Err(WorkspaceObservationError::Invalid(format!(
                "unknown capture error class: {other}"
            ))),
        }
    }
}

/// Last capture failure recorded for observation diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationCaptureFailure {
    pub failed_at: String,
    pub error_class: ObservationCaptureErrorClass,
    pub message: String,
    pub source: Option<String>,
    pub authority_effect: String,
}

impl ObservationCaptureFailure {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Lightweight pass metadata for status queries (no child row collections).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceObservationPassMetadata {
    pub id: String,
    pub captured_at: String,
    pub source: String,
    pub window_count: i32,
    pub monitor_count: i32,
    pub identity_count: i32,
}

/// Operational status of the observation pipeline (not a full snapshot).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceObservationStatus {
    pub has_observation: bool,
    pub freshness: ObservationFreshness,
    pub pass_id: Option<String>,
    pub captured_at: Option<String>,
    pub age_seconds: Option<i64>,
    pub window_count: Option<i32>,
    pub monitor_count: Option<i32>,
    pub identity_count: Option<i32>,
    pub source: Option<String>,
    pub last_failure: Option<ObservationCaptureFailure>,
    pub authority_effect: String,
}

impl WorkspaceObservationStatus {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn unavailable(last_failure: Option<ObservationCaptureFailure>) -> Self {
        Self {
            has_observation: false,
            freshness: ObservationFreshness::Unavailable,
            pass_id: None,
            captured_at: None,
            age_seconds: None,
            window_count: None,
            monitor_count: None,
            identity_count: None,
            source: None,
            last_failure,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Age in seconds between `captured_at` (RFC3339) and `now`.
pub fn observation_age_seconds(captured_at: &str, now: chrono::DateTime<Utc>) -> Result<i64> {
    let captured = chrono::DateTime::parse_from_rfc3339(captured_at)
        .map_err(|error| {
            WorkspaceObservationError::Invalid(format!("invalid captured_at: {error}"))
        })?
        .with_timezone(&Utc);
    Ok((now - captured).num_seconds().max(0))
}

/// Shared freshness definition used by all observation status consumers.
pub fn observation_freshness(
    age_seconds: Option<i64>,
    fresh_threshold_secs: i64,
    stale_threshold_secs: i64,
) -> ObservationFreshness {
    let Some(age) = age_seconds else {
        return ObservationFreshness::Unavailable;
    };
    if age < fresh_threshold_secs {
        ObservationFreshness::Fresh
    } else if age < stale_threshold_secs {
        ObservationFreshness::Recent
    } else {
        ObservationFreshness::Stale
    }
}

/// Build status from pass metadata + optional failure (deterministic when `now` is fixed).
pub fn build_observation_status(
    metadata: Option<&WorkspaceObservationPassMetadata>,
    last_failure: Option<ObservationCaptureFailure>,
    now: chrono::DateTime<Utc>,
) -> Result<WorkspaceObservationStatus> {
    let Some(metadata) = metadata else {
        return Ok(WorkspaceObservationStatus::unavailable(last_failure));
    };
    let age_seconds = observation_age_seconds(&metadata.captured_at, now)?;
    let freshness = observation_freshness(
        Some(age_seconds),
        OBSERVATION_FRESH_THRESHOLD_SECS,
        OBSERVATION_STALE_THRESHOLD_SECS,
    );
    Ok(WorkspaceObservationStatus {
        has_observation: true,
        freshness,
        pass_id: Some(metadata.id.clone()),
        captured_at: Some(metadata.captured_at.clone()),
        age_seconds: Some(age_seconds),
        window_count: Some(metadata.window_count),
        monitor_count: Some(metadata.monitor_count),
        identity_count: Some(metadata.identity_count),
        source: Some(metadata.source.clone()),
        last_failure,
        authority_effect: WorkspaceObservationStatus::AUTHORITY_EFFECT_NONE.into(),
    })
}

/// RFC3339 timestamp helper for observation passes.
pub fn observation_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Empty stub snapshot for non-Windows / test fixtures.
pub fn empty_stub_snapshot(pass_id: impl Into<String>, captured_at: impl Into<String>) -> WorkspaceObservationSnapshot {
    let pass_id = pass_id.into();
    WorkspaceObservationSnapshot {
        pass: WorkspaceObservationPass {
            id: pass_id.clone(),
            captured_at: captured_at.into(),
            schema_version: WorkspaceObservationPass::DEFAULT_SCHEMA_VERSION,
            source: "stub".into(),
            foreground_hwnd: None,
            window_count: 0,
            monitor_count: 0,
            duration_ms: Some(0),
            metadata_json: "{}".into(),
            authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
        },
        windows: Vec::new(),
        monitors: Vec::new(),
        identities: Vec::new(),
        authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> WorkspaceObservationSnapshot {
        let pass_id: String = "pass-1".into();
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.clone(),
                captured_at: "2026-07-26T10:00:00Z".into(),
                schema_version: 1,
                source: "test_inject".into(),
                foreground_hwnd: Some("0x0000000000000001".into()),
                window_count: 1,
                monitor_count: 1,
                duration_ms: Some(12),
                metadata_json: r#"{"fixture":true}"#.into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: "mon-1".into(),
                pass_id: pass_id.clone(),
                monitor_index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                work_x: 0,
                work_y: 0,
                work_w: 1920,
                work_h: 1040,
                is_primary: true,
                dpi_scale: Some(1.0),
                authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
            }],
            windows: vec![ObservedWindow {
                id: "win-1".into(),
                pass_id: pass_id.clone(),
                hwnd: "0x0000000000000001".into(),
                stable_window_id: Some("identity-1".into()),
                title: "Fixture Window".into(),
                process_id: 4242,
                process_name: Some("fixture.exe".into()),
                x: 100,
                y: 100,
                width: 800,
                height: 600,
                monitor_id: Some("mon-1".into()),
                visible: true,
                minimized: false,
                focused: true,
                z_order: Some(0),
                authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
            }],
            identities: vec![ObservationWindowIdentity {
                id: "identity-1".into(),
                process_id: 4242,
                title_fingerprint: "fixture-window".into(),
                first_seen_at: "2026-07-26T10:00:00Z".into(),
                last_seen_at: "2026-07-26T10:00:00Z".into(),
                last_hwnd: "0x0000000000000001".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn valid_snapshot_passes_validation() {
        assert!(sample_snapshot().validate().is_ok());
    }

    #[test]
    fn invalid_snapshot_rejected_for_count_mismatch() {
        let mut snapshot = sample_snapshot();
        snapshot.pass.window_count = 99;
        assert_eq!(
            snapshot.validate(),
            Err(WorkspaceObservationError::WindowCountMismatch)
        );
    }

    #[test]
    fn identity_confidence_serializes() {
        let json = serde_json::to_string(&WindowIdentityConfidence::Medium).unwrap();
        assert_eq!(json, "\"medium\"");
        assert_eq!(
            WindowIdentityConfidence::parse("high").unwrap(),
            WindowIdentityConfidence::High
        );
    }

    #[test]
    fn empty_stub_snapshot_is_valid() {
        let snapshot = empty_stub_snapshot("stub-pass", "2026-07-26T10:00:00Z");
        assert!(snapshot.validate().is_ok());
        assert_eq!(snapshot.pass.source, "stub");
        assert!(snapshot.windows.is_empty());
        assert!(snapshot.monitors.is_empty());
    }

    #[test]
    fn authority_guard_fails_execution() {
        assert_eq!(
            WorkspaceObservationSnapshot::attempt_execute(),
            Err(WorkspaceObservationError::CannotExecute)
        );
    }

    #[test]
    fn rejects_multiple_focused_windows() {
        let mut snapshot = sample_snapshot();
        snapshot.windows.push(ObservedWindow {
            id: "win-2".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0x0000000000000002".into(),
            stable_window_id: None,
            title: "Other".into(),
            process_id: 1,
            process_name: None,
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            monitor_id: Some("mon-1".into()),
            visible: true,
            minimized: false,
            focused: true,
            z_order: Some(1),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.pass.window_count = 2;
        assert!(matches!(
            snapshot.validate(),
            Err(WorkspaceObservationError::Invalid(message)) if message.contains("at most one")
        ));
    }

    #[test]
    fn rejects_unknown_monitor_reference() {
        let mut snapshot = sample_snapshot();
        snapshot.windows[0].monitor_id = Some("missing-mon".into());
        assert!(matches!(
            snapshot.validate(),
            Err(WorkspaceObservationError::Invalid(message)) if message.contains("unknown monitor")
        ));
    }

    #[test]
    fn checked_numeric_conversions() {
        assert_eq!(observation_u32_to_i32(42, "process_id").unwrap(), 42);
        assert!(matches!(
            observation_u64_to_i32(u64::from(u32::MAX) + 1, "duration_ms"),
            Err(WorkspaceObservationError::NumericOverflow(_))
        ));
    }

    #[test]
    fn observation_age_and_freshness_are_deterministic() {
        let now = chrono::DateTime::parse_from_rfc3339("2026-07-26T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(
            observation_age_seconds("2026-07-26T11:59:30Z", now).unwrap(),
            30
        );
        assert_eq!(
            observation_freshness(Some(30), 60, 300),
            ObservationFreshness::Fresh
        );
        assert_eq!(
            observation_freshness(Some(120), 60, 300),
            ObservationFreshness::Recent
        );
        assert_eq!(
            observation_freshness(Some(400), 60, 300),
            ObservationFreshness::Stale
        );
        assert_eq!(
            observation_freshness(None, 60, 300),
            ObservationFreshness::Unavailable
        );
    }

    #[test]
    fn build_status_unavailable_without_metadata() {
        let now = Utc::now();
        let status = build_observation_status(None, None, now).unwrap();
        assert!(!status.has_observation);
        assert_eq!(status.freshness, ObservationFreshness::Unavailable);
    }
}
