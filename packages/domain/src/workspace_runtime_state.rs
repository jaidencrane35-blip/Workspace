//! Canonical live desktop runtime state (observation + cache + execution).
//!
//! Distinct from:
//! - kernel lifecycle `WorkspaceState` (Ready/version)
//! - cognition `WorkspaceRuntimeContext` (does not own desktop capture)
//! - projection `WorkspaceSnapshot` (zones/apps/widgets)
//!
//! Desktop window projection remains [`crate::WorkspaceState`]. This type is the
//! process-facing bundle every runtime consumer should read.

use serde::{Deserialize, Serialize};

use crate::desktop_action::{
    OperationOutcome, RestoreCompatibilitySummary, RestoreExecutionSummary,
};
use crate::runtime_health::RuntimeHealth;
use crate::workspace_observation::WorkspaceObservationStatus;
use crate::workspace_state::WorkspaceState;

/// Observation cache / refresh lifecycle published by the capture coordinator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationCachePhase {
    /// No refresh outstanding; last observation may still be stale by age.
    Idle,
    /// A consumer requested a refresh that has not started yet.
    RefreshRequested,
    /// Single-flight capture is running.
    RefreshInProgress,
    /// Last refresh completed successfully.
    RefreshCompleted,
    /// Last refresh failed (see observation.last_failure).
    RefreshFailed,
}

impl ObservationCachePhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::RefreshRequested => "refresh_requested",
            Self::RefreshInProgress => "refresh_in_progress",
            Self::RefreshCompleted => "refresh_completed",
            Self::RefreshFailed => "refresh_failed",
        }
    }
}

/// Restore planning / execution lifecycle published into runtime state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreExecutionPhase {
    Idle,
    Planning,
    Validating,
    Executing,
    Completed,
    Partial,
    Failed,
}

impl RestoreExecutionPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Planning => "planning",
            Self::Validating => "validating",
            Self::Executing => "executing",
            Self::Completed => "completed",
            Self::Partial => "partial",
            Self::Failed => "failed",
        }
    }

    pub fn from_operation_outcome(outcome: &OperationOutcome) -> Self {
        match outcome {
            OperationOutcome::Completed => Self::Completed,
            OperationOutcome::PartiallyCompleted => Self::Partial,
            OperationOutcome::Failed
            | OperationOutcome::Cancelled
            | OperationOutcome::Indeterminate => Self::Failed,
        }
    }
}

/// One restore execution recorded for runtime history (bounded ring).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreHistoryEntry {
    pub recorded_at: String,
    pub operation_id: String,
    pub outcome: OperationOutcome,
    pub summary: RestoreExecutionSummary,
    pub purpose: String,
}

/// Canonical live workspace runtime state — sole process desktop representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeState {
    /// Interpreted desktop projection (windows, processes, focus).
    pub desktop: WorkspaceState,
    /// Observation freshness / age / last failure.
    pub observation: WorkspaceObservationStatus,
    /// Capture cache phase (prevents duplicate observation work at the owner).
    pub cache_phase: ObservationCachePhase,
    /// Restore plan/execute phase.
    pub execution_phase: RestoreExecutionPhase,
    /// Active product workspace id when known.
    pub active_workspace_id: Option<String>,
    /// Primary / focused window monitor index when known.
    pub active_monitor_index: Option<i32>,
    /// Capture timestamp of the latest observation pass.
    pub capture_timestamp: Option<String>,
    /// Last restore compatibility band (`high`/`steady`/`limited`/`empty`) when planned.
    pub confidence_band: Option<String>,
    /// Recent restore executions (newest first).
    pub restore_history: Vec<RestoreHistoryEntry>,
    /// Internal resilience health (not an Experience surface).
    pub health: RuntimeHealth,
    /// Monotonic generation bumped on invalidate / successful refresh.
    pub generation: u64,
    pub authority_effect: String,
}

impl WorkspaceRuntimeState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const RESTORE_HISTORY_LIMIT: usize = 20;

    pub fn empty() -> Self {
        Self {
            desktop: WorkspaceState::empty(),
            observation: WorkspaceObservationStatus::unavailable(None),
            cache_phase: ObservationCachePhase::Idle,
            execution_phase: RestoreExecutionPhase::Idle,
            active_workspace_id: None,
            active_monitor_index: None,
            capture_timestamp: None,
            confidence_band: None,
            restore_history: Vec::new(),
            health: RuntimeHealth::healthy(),
            generation: 0,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_compatibility(mut self, summary: &RestoreCompatibilitySummary) -> Self {
        self.confidence_band = Some(summary.confidence_band.clone());
        self
    }
}
