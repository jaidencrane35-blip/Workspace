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

        Ok(())
    }
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
}
