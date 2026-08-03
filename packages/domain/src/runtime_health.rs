//! Process-facing runtime health for desktop session resilience.
//!
//! Distinct from cognition [`crate::workspace_runtime::WorkspaceRuntimeHealth`]
//! and kernel lifecycle [`WorkspaceHealth`]. This model is owned by the
//! desktop runtime owner and is never a new Experience surface.

use serde::{Deserialize, Serialize};

use crate::workspace_observation::observation_now_rfc3339;

/// Integrity of the durable session after load / recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionIntegrity {
    Ok,
    MissingRecovered,
    CorruptRecovered,
    MigrationRecovered,
    IncompatibleRecovered,
}

impl SessionIntegrity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::MissingRecovered => "missing_recovered",
            Self::CorruptRecovered => "corrupt_recovered",
            Self::MigrationRecovered => "migration_recovered",
            Self::IncompatibleRecovered => "incompatible_recovered",
        }
    }

    pub fn is_degraded(self) -> bool {
        !matches!(self, Self::Ok)
    }
}

/// How an interrupted or recovered operation is classified at startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDisposition {
    None,
    Recovered,
    Incomplete,
    NeedsRefresh,
}

impl RecoveryDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Recovered => "recovered",
            Self::Incomplete => "incomplete",
            Self::NeedsRefresh => "needs_refresh",
        }
    }
}

/// Durable fence written before a mutating runtime operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingOperationKind {
    Observation,
    Save,
    RestorePlanning,
    RestoreExecution,
    Persistence,
}

impl PendingOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::Save => "save",
            Self::RestorePlanning => "restore_planning",
            Self::RestoreExecution => "restore_execution",
            Self::Persistence => "persistence",
        }
    }

    /// Startup disposition when this fence is found uncleared.
    pub fn interruption_disposition(self) -> RecoveryDisposition {
        match self {
            Self::Observation => RecoveryDisposition::NeedsRefresh,
            Self::Save | Self::RestorePlanning | Self::RestoreExecution => {
                RecoveryDisposition::Incomplete
            }
            // Atomic SQLite replace: prior valid session retained.
            Self::Persistence => RecoveryDisposition::Recovered,
        }
    }
}

/// Written before a mutation; cleared only after successful completion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingOperationFence {
    pub kind: PendingOperationKind,
    pub started_at: String,
    pub correlation_id: Option<String>,
}

impl PendingOperationFence {
    pub fn new(kind: PendingOperationKind, correlation_id: Option<String>) -> Self {
        Self {
            kind,
            started_at: observation_now_rfc3339(),
            correlation_id,
        }
    }
}

/// Acknowledged recovery — never silently discarded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRecord {
    pub disposition: RecoveryDisposition,
    pub kind: PendingOperationKind,
    pub recorded_at: String,
    pub detail: String,
}

impl RecoveryRecord {
    pub fn from_fence(fence: &PendingOperationFence, detail: impl Into<String>) -> Self {
        Self {
            disposition: fence.kind.interruption_disposition(),
            kind: fence.kind,
            recorded_at: observation_now_rfc3339(),
            detail: detail.into(),
        }
    }
}

/// Internal runtime health — published via existing runtime services only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub last_successful_observation_at: Option<String>,
    pub last_persistence_at: Option<String>,
    pub last_restore_at: Option<String>,
    pub session_integrity: SessionIntegrity,
    pub pending_recovery: Option<RecoveryRecord>,
    pub degraded: bool,
    pub recovery_disposition: RecoveryDisposition,
}

impl RuntimeHealth {
    pub fn healthy() -> Self {
        Self {
            last_successful_observation_at: None,
            last_persistence_at: None,
            last_restore_at: None,
            session_integrity: SessionIntegrity::Ok,
            pending_recovery: None,
            degraded: false,
            recovery_disposition: RecoveryDisposition::None,
        }
    }

    pub fn recompute_degraded(&mut self) {
        self.degraded = self.session_integrity.is_degraded()
            || matches!(
                self.recovery_disposition,
                RecoveryDisposition::Incomplete | RecoveryDisposition::NeedsRefresh
            )
            || self.pending_recovery.is_some();
    }

    pub fn with_integrity(mut self, integrity: SessionIntegrity) -> Self {
        self.session_integrity = integrity;
        if integrity.is_degraded() && self.recovery_disposition == RecoveryDisposition::None {
            self.recovery_disposition = RecoveryDisposition::Recovered;
        }
        self.recompute_degraded();
        self
    }

    pub fn with_recovery(mut self, record: RecoveryRecord) -> Self {
        self.recovery_disposition = record.disposition;
        self.pending_recovery = Some(record);
        self.recompute_degraded();
        self
    }

    pub fn note_observation_success(&mut self, at: impl Into<String>) {
        self.last_successful_observation_at = Some(at.into());
        if matches!(
            self.recovery_disposition,
            RecoveryDisposition::NeedsRefresh
        ) {
            self.recovery_disposition = RecoveryDisposition::Recovered;
            self.pending_recovery = None;
        }
        self.recompute_degraded();
    }

    pub fn note_persistence_success(&mut self, at: impl Into<String>) {
        self.last_persistence_at = Some(at.into());
        self.recompute_degraded();
    }

    pub fn note_restore_success(&mut self, at: impl Into<String>) {
        self.last_restore_at = Some(at.into());
        if matches!(self.recovery_disposition, RecoveryDisposition::Incomplete) {
            self.recovery_disposition = RecoveryDisposition::Recovered;
            self.pending_recovery = None;
        }
        self.recompute_degraded();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_interruption_needs_refresh() {
        assert_eq!(
            PendingOperationKind::Observation.interruption_disposition(),
            RecoveryDisposition::NeedsRefresh
        );
    }

    #[test]
    fn persistence_interruption_is_recovered() {
        assert_eq!(
            PendingOperationKind::Persistence.interruption_disposition(),
            RecoveryDisposition::Recovered
        );
    }

    #[test]
    fn degraded_when_incomplete() {
        let health = RuntimeHealth::healthy().with_recovery(RecoveryRecord {
            disposition: RecoveryDisposition::Incomplete,
            kind: PendingOperationKind::RestoreExecution,
            recorded_at: "t".into(),
            detail: "crash".into(),
        });
        assert!(health.degraded);
        assert_eq!(health.recovery_disposition, RecoveryDisposition::Incomplete);
    }
}
