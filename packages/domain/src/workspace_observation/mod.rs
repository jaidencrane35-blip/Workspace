//! Workspace Observation Layer — durable desktop perception (Phase 6 / Sprint 103 / DAF-1b).
//!
//! Why: factual current-state desktop window observation and identity for DAF.
//! Owner: domain contracts here; capture in `windows-integration`; orchestration in kernel.
//! Deliberately does not: move/resize windows, match ApplicationId, suggest layouts, or assist.
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

/// Whether a referenced window is present in a factual snapshot (DAF-1b diagnostics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservedWindowAvailability {
    /// HWND (or stable id) maps to a window row in this pass.
    Available,
    /// Identity registry knows the window, but it is absent from current pass windows.
    IdentityKnownWindowMissing,
    /// Neither a matching window row nor identity entry exists.
    Unavailable,
}

impl ObservedWindowAvailability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::IdentityKnownWindowMissing => "identity_known_window_missing",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
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

    /// Architecture guard — observation never moves or resizes windows.
    pub fn attempt_control_window() -> Result<()> {
        Err(WorkspaceObservationError::CannotExecute)
    }

    /// Availability of a capture-format HWND in this snapshot.
    pub fn availability_for_hwnd(&self, hwnd: &str) -> ObservedWindowAvailability {
        let hwnd = hwnd.trim();
        if self.windows.iter().any(|window| window.hwnd == hwnd) {
            return ObservedWindowAvailability::Available;
        }
        if self
            .identities
            .iter()
            .any(|identity| identity.last_hwnd == hwnd)
        {
            return ObservedWindowAvailability::IdentityKnownWindowMissing;
        }
        ObservedWindowAvailability::Unavailable
    }

    /// Availability of a stable window identity in this snapshot.
    pub fn availability_for_stable_id(&self, stable_window_id: &str) -> ObservedWindowAvailability {
        let stable_window_id = stable_window_id.trim();
        if self.windows.iter().any(|window| {
            window
                .stable_window_id
                .as_deref()
                .is_some_and(|id| id == stable_window_id)
        }) {
            return ObservedWindowAvailability::Available;
        }
        if self
            .identities
            .iter()
            .any(|identity| identity.id == stable_window_id)
        {
            return ObservedWindowAvailability::IdentityKnownWindowMissing;
        }
        ObservedWindowAvailability::Unavailable
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

/// Who requested an observation capture (orchestration contract only).
///
/// `Event` callers must enter through `ObservationEventGateway` (Sprint 117).
/// `Plugin` callers are not implemented yet — contract only. Admission still
/// rejects `Event` / `Plugin` until those sources are enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureRequestSource {
    Manual,
    System,
    Scheduled,
    Event,
    Plugin,
}

impl CaptureRequestSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::System => "system",
            Self::Scheduled => "scheduled",
            Self::Event => "event",
            Self::Plugin => "plugin",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "manual" => Ok(Self::Manual),
            "system" => Ok(Self::System),
            "scheduled" => Ok(Self::Scheduled),
            "event" => Ok(Self::Event),
            "plugin" => Ok(Self::Plugin),
            other => Err(WorkspaceObservationError::Invalid(format!(
                "unknown capture request source: {other}"
            ))),
        }
    }
}

/// Provenance for a capture request — attachable to observation lifecycle audits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureProvenance {
    pub source: CaptureRequestSource,
    /// Optional reason the capture was requested.
    pub reason: Option<String>,
    /// Optional opaque context (consumer id, trigger id, etc.).
    pub context: Option<String>,
}

impl CaptureProvenance {
    pub fn new(source: CaptureRequestSource) -> Self {
        Self {
            source,
            reason: None,
            context: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Capture request contract for the observation orchestration boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub source: CaptureRequestSource,
    pub reason: Option<String>,
    pub context: Option<String>,
}

impl CaptureRequest {
    pub fn new(source: CaptureRequestSource) -> Self {
        Self {
            source,
            reason: None,
            context: None,
        }
    }

    pub fn from_provenance(provenance: CaptureProvenance) -> Self {
        Self {
            source: provenance.source,
            reason: provenance.reason,
            context: provenance.context,
        }
    }

    pub fn manual() -> Self {
        Self::new(CaptureRequestSource::Manual)
    }

    pub fn system() -> Self {
        Self::new(CaptureRequestSource::System)
    }

    pub fn scheduled() -> Self {
        Self::new(CaptureRequestSource::Scheduled)
    }

    pub fn event() -> Self {
        Self::new(CaptureRequestSource::Event)
    }

    pub fn plugin() -> Self {
        Self::new(CaptureRequestSource::Plugin)
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Provenance projection for lifecycle attachment / auditing.
    pub fn provenance(&self) -> CaptureProvenance {
        CaptureProvenance {
            source: self.source,
            reason: self.reason.clone(),
            context: self.context.clone(),
        }
    }
}

/// How fresh an observation must be for a consumer (contract only — never triggers capture).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ObservationFreshnessRequirement {
    /// Any persisted observation is acceptable.
    AnyAvailable,
    /// Must be classified `Fresh`.
    Fresh,
    /// Must not be `Stale` (`Fresh` or `Recent`).
    NotStale,
    /// Must be at most this many seconds old.
    MaxAgeSeconds { max_age_seconds: i64 },
}

impl ObservationFreshnessRequirement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AnyAvailable => "any_available",
            Self::Fresh => "fresh",
            Self::NotStale => "not_stale",
            Self::MaxAgeSeconds { .. } => "max_age_seconds",
        }
    }
}

/// Optional context for a refresh decision evaluation (never executes capture).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ObservationRefreshContext {
    pub consumer: Option<String>,
    pub purpose: Option<String>,
}

impl ObservationRefreshContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_consumer(mut self, consumer: impl Into<String>) -> Self {
        self.consumer = Some(consumer.into());
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }
}

/// Why a refresh decision was blocked (read-only policy outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationRefreshBlockedReason {
    CaptureInProgress,
}

impl ObservationRefreshBlockedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CaptureInProgress => "capture_in_progress",
        }
    }
}

/// Read-only answer to: should Workspace request a new observation?
///
/// Never captures. Never grants authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum ObservationRefreshDecision {
    FreshEnough,
    RefreshRequired,
    ObservationUnavailable,
    RefreshBlocked {
        reason: ObservationRefreshBlockedReason,
    },
}

impl ObservationRefreshDecision {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FreshEnough => "fresh_enough",
            Self::RefreshRequired => "refresh_required",
            Self::ObservationUnavailable => "observation_unavailable",
            Self::RefreshBlocked { .. } => "refresh_blocked",
        }
    }
}

/// Consumer contract: express a freshness need without triggering capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationConsumerFreshnessNeed {
    pub consumer_id: String,
    pub requirement: ObservationFreshnessRequirement,
    pub context: Option<String>,
}

impl ObservationConsumerFreshnessNeed {
    pub const ENVIRONMENT_CONSUMER: &'static str = "workspace_environment";
    pub const INTELLIGENCE_CONSUMER: &'static str = "workspace_intelligence";

    /// Canonical Environment consumer need — never triggers capture by itself.
    pub fn for_environment() -> Self {
        Self::new(Self::ENVIRONMENT_CONSUMER, ObservationFreshnessRequirement::NotStale)
            .with_context("environment_generate")
    }

    /// Canonical Intelligence consumer need — never triggers capture by itself.
    pub fn for_intelligence() -> Self {
        Self::new(
            Self::INTELLIGENCE_CONSUMER,
            ObservationFreshnessRequirement::NotStale,
        )
        .with_context("intelligence_generate")
    }

    pub fn new(
        consumer_id: impl Into<String>,
        requirement: ObservationFreshnessRequirement,
    ) -> Self {
        Self {
            consumer_id: consumer_id.into(),
            requirement,
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Trigger source for observation refresh requests (callers not implemented yet).
///
/// Includes `Plugin` beyond capture-only sources used by direct capture commands.
pub type ObservationTriggerSource = CaptureRequestSource;

/// Domain contract for an observation trigger (Sprint 110).
///
/// Evaluated by ObservationTriggerAuthority — does not itself capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationTriggerRequest {
    pub source: ObservationTriggerSource,
    pub reason: Option<String>,
    pub context: Option<String>,
    pub freshness_requirement: ObservationFreshnessRequirement,
}

impl ObservationTriggerRequest {
    pub fn new(
        source: ObservationTriggerSource,
        freshness_requirement: ObservationFreshnessRequirement,
    ) -> Self {
        Self {
            source,
            reason: None,
            context: None,
            freshness_requirement,
        }
    }

    pub fn manual(freshness_requirement: ObservationFreshnessRequirement) -> Self {
        Self::new(CaptureRequestSource::Manual, freshness_requirement)
    }

    pub fn system(freshness_requirement: ObservationFreshnessRequirement) -> Self {
        Self::new(CaptureRequestSource::System, freshness_requirement)
    }

    pub fn scheduled(freshness_requirement: ObservationFreshnessRequirement) -> Self {
        Self::new(CaptureRequestSource::Scheduled, freshness_requirement)
    }

    pub fn event(freshness_requirement: ObservationFreshnessRequirement) -> Self {
        Self::new(CaptureRequestSource::Event, freshness_requirement)
    }

    pub fn plugin(freshness_requirement: ObservationFreshnessRequirement) -> Self {
        Self::new(CaptureRequestSource::Plugin, freshness_requirement)
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Build the CaptureRequest that CaptureCoordinator should receive.
    pub fn to_capture_request(&self) -> CaptureRequest {
        CaptureRequest {
            source: self.source,
            reason: self.reason.clone(),
            context: self.context.clone(),
        }
    }

    pub fn provenance(&self) -> CaptureProvenance {
        CaptureProvenance {
            source: self.source,
            reason: self.reason.clone(),
            context: self.context.clone(),
        }
    }
}

/// Minimal observation schedule configuration (Sprint 114).
///
/// Not persisted and not exposed via UI in this sprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationScheduleConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
}

impl ObservationScheduleConfig {
    pub const DEFAULT_INTERVAL_SECONDS: u64 = 300;

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            interval_seconds: Self::DEFAULT_INTERVAL_SECONDS,
        }
    }

    pub fn enabled_with_interval(interval_seconds: u64) -> Self {
        Self {
            enabled: true,
            interval_seconds: interval_seconds.max(1),
        }
    }

    pub fn enabled_default() -> Self {
        Self::enabled_with_interval(Self::DEFAULT_INTERVAL_SECONDS)
    }
}

impl Default for ObservationScheduleConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

/// Read-only runtime health of the observation schedule loop (Sprint 115).
///
/// Does not include observation snapshots. `authority_effect` is always `"none"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationSchedulerStatus {
    pub running: bool,
    pub enabled: bool,
    pub interval_seconds: u64,
    pub started_at: Option<String>,
    pub last_tick_at: Option<String>,
    pub last_tick_duration_ms: Option<u64>,
    pub ticks_emitted: u64,
    pub captures_requested: u64,
    pub captures_skipped: u64,
    pub rate_limited_count: u64,
    pub consecutive_failures: u64,
    pub authority_effect: String,
}

impl ObservationSchedulerStatus {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn idle(config: &ObservationScheduleConfig) -> Self {
        Self {
            running: false,
            enabled: config.enabled,
            interval_seconds: config.interval_seconds,
            started_at: None,
            last_tick_at: None,
            last_tick_duration_ms: None,
            ticks_emitted: 0,
            captures_requested: 0,
            captures_skipped: 0,
            rate_limited_count: 0,
            consecutive_failures: 0,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Auditable trigger authority outcome (capture payload attached in kernel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationTriggerOutcome {
    AcceptedCapture,
    IgnoredFresh,
    BlockedCaptureInProgress,
    /// Observation was unavailable and no capture completed (soft decline / failed path).
    Unavailable,
    /// Trigger storm protection: cooldown / rate window exceeded.
    RateLimited,
    /// Trigger source is not admitted for execution yet.
    RejectedSource,
}

impl ObservationTriggerOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedCapture => "accepted_capture",
            Self::IgnoredFresh => "ignored_fresh",
            Self::BlockedCaptureInProgress => "blocked_capture_in_progress",
            Self::Unavailable => "unavailable",
            Self::RateLimited => "rate_limited",
            Self::RejectedSource => "rejected_source",
        }
    }
}

/// Explicit freshness ensure result (operator/manual path via TriggerAuthority).
///
/// Never grants execution authority. Capture only occurs when TriggerAuthority admits
/// Manual/System/Scheduled and refresh policy requires it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationFreshnessEnsureResult {
    pub consumer_id: String,
    pub trigger_outcome: String,
    pub refresh_decision: String,
    pub freshness_before: String,
    pub freshness_after: String,
    pub age_seconds_after: Option<i64>,
    pub captured: bool,
    pub explanation: String,
    pub authority_effect: String,
}

impl ObservationFreshnessEnsureResult {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Whether a trigger request may proceed past admission (Sprint 112).
///
/// Pure decision — never captures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum ObservationTriggerAdmissionDecision {
    Admitted,
    RateLimited { explanation: String },
    RejectedSource { explanation: String },
}

impl ObservationTriggerAdmissionDecision {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::RateLimited { .. } => "rate_limited",
            Self::RejectedSource { .. } => "rejected_source",
        }
    }

    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted)
    }

    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Admitted => None,
            Self::RateLimited { explanation } | Self::RejectedSource { explanation } => {
                Some(explanation.as_str())
            }
        }
    }
}

/// Default minimum gap between admitted triggers (process-local rate state).
pub const OBSERVATION_TRIGGER_MIN_ADMIT_INTERVAL_SECS: u64 = 30;
/// Sliding window for admission counting.
pub const OBSERVATION_TRIGGER_ADMIT_WINDOW_SECS: u64 = 300;
/// Max admissions inside the sliding window.
pub const OBSERVATION_TRIGGER_MAX_ADMITS_PER_WINDOW: u32 = 8;

/// Whether the current status satisfies a freshness requirement.
pub fn observation_meets_freshness_requirement(
    status: &WorkspaceObservationStatus,
    requirement: &ObservationFreshnessRequirement,
) -> bool {
    if !status.has_observation || status.freshness == ObservationFreshness::Unavailable {
        return false;
    }
    match requirement {
        ObservationFreshnessRequirement::AnyAvailable => true,
        ObservationFreshnessRequirement::Fresh => {
            status.freshness == ObservationFreshness::Fresh
        }
        ObservationFreshnessRequirement::NotStale => matches!(
            status.freshness,
            ObservationFreshness::Fresh | ObservationFreshness::Recent
        ),
        ObservationFreshnessRequirement::MaxAgeSeconds { max_age_seconds } => status
            .age_seconds
            .map(|age| age <= *max_age_seconds)
            .unwrap_or(false),
    }
}

/// Pure refresh policy: decide whether a new observation should be requested.
///
/// Does not capture, schedule, or mutate observation state.
pub fn decide_observation_refresh(
    status: &WorkspaceObservationStatus,
    requirement: &ObservationFreshnessRequirement,
    capture_in_progress: bool,
) -> ObservationRefreshDecision {
    if capture_in_progress {
        return ObservationRefreshDecision::RefreshBlocked {
            reason: ObservationRefreshBlockedReason::CaptureInProgress,
        };
    }
    if !status.has_observation || status.freshness == ObservationFreshness::Unavailable {
        return ObservationRefreshDecision::ObservationUnavailable;
    }
    if observation_meets_freshness_requirement(status, requirement) {
        ObservationRefreshDecision::FreshEnough
    } else {
        ObservationRefreshDecision::RefreshRequired
    }
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

    #[test]
    fn capture_request_source_contract() {
        assert_eq!(CaptureRequest::manual().source, CaptureRequestSource::Manual);
        assert_eq!(CaptureRequest::system().source, CaptureRequestSource::System);
        assert_eq!(
            CaptureRequest::scheduled().source,
            CaptureRequestSource::Scheduled
        );
        assert_eq!(CaptureRequest::event().source, CaptureRequestSource::Event);
        assert_eq!(CaptureRequest::plugin().source, CaptureRequestSource::Plugin);
        assert_eq!(
            CaptureRequestSource::parse("plugin").unwrap(),
            CaptureRequestSource::Plugin
        );
        assert_eq!(
            CaptureRequestSource::parse("manual").unwrap(),
            CaptureRequestSource::Manual
        );
        assert!(CaptureRequestSource::parse("unknown").is_err());
    }

    #[test]
    fn observation_trigger_request_maps_to_capture_request() {
        let trigger = ObservationTriggerRequest::plugin(
            ObservationFreshnessRequirement::MaxAgeSeconds {
                max_age_seconds: 45,
            },
        )
        .with_reason("sync")
        .with_context("plugin:x");
        let capture = trigger.to_capture_request();
        assert_eq!(capture.source, CaptureRequestSource::Plugin);
        assert_eq!(capture.reason.as_deref(), Some("sync"));
        assert_eq!(capture.context.as_deref(), Some("plugin:x"));
        assert_eq!(
            ObservationTriggerOutcome::IgnoredFresh.as_str(),
            "ignored_fresh"
        );
    }

    #[test]
    fn capture_request_provenance_carries_reason_and_context() {
        let request = CaptureRequest::system()
            .with_reason("startup_seed")
            .with_context("kernel:boot");
        let provenance = request.provenance();
        assert_eq!(provenance.source, CaptureRequestSource::System);
        assert_eq!(provenance.reason.as_deref(), Some("startup_seed"));
        assert_eq!(provenance.context.as_deref(), Some("kernel:boot"));

        let round_trip = CaptureRequest::from_provenance(
            CaptureProvenance::new(CaptureRequestSource::Manual)
                .with_reason("user")
                .with_context("ipc"),
        );
        assert_eq!(round_trip.source, CaptureRequestSource::Manual);
        assert_eq!(round_trip.reason.as_deref(), Some("user"));
        assert_eq!(round_trip.context.as_deref(), Some("ipc"));
    }

    fn status_with_freshness(
        freshness: ObservationFreshness,
        age_seconds: Option<i64>,
    ) -> WorkspaceObservationStatus {
        WorkspaceObservationStatus {
            has_observation: freshness != ObservationFreshness::Unavailable,
            freshness,
            pass_id: Some("pass-1".into()),
            captured_at: Some("2026-07-26T12:00:00Z".into()),
            age_seconds,
            window_count: Some(1),
            monitor_count: Some(1),
            identity_count: Some(1),
            source: Some("stub".into()),
            last_failure: None,
            authority_effect: WorkspaceObservationStatus::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn refresh_decision_outcomes() {
        let fresh = status_with_freshness(ObservationFreshness::Fresh, Some(10));
        assert_eq!(
            decide_observation_refresh(
                &fresh,
                &ObservationFreshnessRequirement::Fresh,
                false
            ),
            ObservationRefreshDecision::FreshEnough
        );

        let recent = status_with_freshness(ObservationFreshness::Recent, Some(120));
        assert_eq!(
            decide_observation_refresh(
                &recent,
                &ObservationFreshnessRequirement::Fresh,
                false
            ),
            ObservationRefreshDecision::RefreshRequired
        );
        assert_eq!(
            decide_observation_refresh(
                &recent,
                &ObservationFreshnessRequirement::NotStale,
                false
            ),
            ObservationRefreshDecision::FreshEnough
        );

        let stale = status_with_freshness(ObservationFreshness::Stale, Some(400));
        assert_eq!(
            decide_observation_refresh(
                &stale,
                &ObservationFreshnessRequirement::NotStale,
                false
            ),
            ObservationRefreshDecision::RefreshRequired
        );

        assert_eq!(
            decide_observation_refresh(
                &WorkspaceObservationStatus::unavailable(None),
                &ObservationFreshnessRequirement::AnyAvailable,
                false
            ),
            ObservationRefreshDecision::ObservationUnavailable
        );

        assert_eq!(
            decide_observation_refresh(
                &fresh,
                &ObservationFreshnessRequirement::AnyAvailable,
                true
            ),
            ObservationRefreshDecision::RefreshBlocked {
                reason: ObservationRefreshBlockedReason::CaptureInProgress,
            }
        );
    }

    #[test]
    fn freshness_requirement_max_age_comparison() {
        let status = status_with_freshness(ObservationFreshness::Recent, Some(90));
        assert!(observation_meets_freshness_requirement(
            &status,
            &ObservationFreshnessRequirement::MaxAgeSeconds {
                max_age_seconds: 120
            }
        ));
        assert!(!observation_meets_freshness_requirement(
            &status,
            &ObservationFreshnessRequirement::MaxAgeSeconds {
                max_age_seconds: 60
            }
        ));

        let need = ObservationConsumerFreshnessNeed::new(
            "environment",
            ObservationFreshnessRequirement::MaxAgeSeconds {
                max_age_seconds: 30,
            },
        )
        .with_context("generate");
        assert_eq!(need.consumer_id, "environment");
        assert_eq!(need.context.as_deref(), Some("generate"));
    }

    #[test]
    fn empty_snapshot_marks_windows_unavailable() {
        let snapshot = empty_stub_snapshot("pass-empty", "2026-07-29T12:00:00Z");
        assert!(snapshot.windows.is_empty());
        assert_eq!(
            snapshot.availability_for_hwnd("0x1"),
            ObservedWindowAvailability::Unavailable
        );
        assert_eq!(
            snapshot.availability_for_stable_id("missing"),
            ObservedWindowAvailability::Unavailable
        );
    }

    #[test]
    fn availability_distinguishes_missing_present_and_identity_only() {
        let mut snapshot = empty_stub_snapshot("pass-a", "2026-07-29T12:00:00Z");
        snapshot.windows.push(ObservedWindow {
            id: "w1".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xAA".into(),
            stable_window_id: Some("stable-1".into()),
            title: "Present".into(),
            process_id: 10,
            process_name: Some("app.exe".into()),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.pass.window_count = 1;
        snapshot.identities.push(ObservationWindowIdentity {
            id: "stable-gone".into(),
            process_id: 11,
            title_fingerprint: "gone".into(),
            first_seen_at: "2026-07-29T11:00:00Z".into(),
            last_seen_at: "2026-07-29T11:00:00Z".into(),
            last_hwnd: "0xBB".into(),
            confidence: WindowIdentityConfidence::High,
            authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
        });

        assert_eq!(
            snapshot.availability_for_hwnd("0xAA"),
            ObservedWindowAvailability::Available
        );
        assert_eq!(
            snapshot.availability_for_stable_id("stable-1"),
            ObservedWindowAvailability::Available
        );
        assert_eq!(
            snapshot.availability_for_hwnd("0xBB"),
            ObservedWindowAvailability::IdentityKnownWindowMissing
        );
        assert_eq!(
            snapshot.availability_for_stable_id("stable-gone"),
            ObservedWindowAvailability::IdentityKnownWindowMissing
        );
    }

    #[test]
    fn observation_refuses_control_and_execution() {
        assert!(matches!(
            WorkspaceObservationSnapshot::attempt_execute(),
            Err(WorkspaceObservationError::CannotExecute)
        ));
        assert!(matches!(
            WorkspaceObservationSnapshot::attempt_control_window(),
            Err(WorkspaceObservationError::CannotExecute)
        ));
    }
}
