//! Durable subset of [`crate::WorkspaceRuntimeState`] (session persistence).
//!
//! Classification authority: `architecture/27_Workspace_Session_Persistence.md`.
//! Recovery fences: `architecture/28_Runtime_Recovery_Model.md`.
//! Only this record is written. Desktop projection and cache/execution phases
//! are never serialized.

use serde::{Deserialize, Serialize};

use crate::runtime_health::{PendingOperationFence, RecoveryRecord};
use crate::workspace_observation::observation_now_rfc3339;
use crate::workspace_runtime_state::{RestoreHistoryEntry, WorkspaceRuntimeState};

/// Current on-disk session schema version. Explicit migrations only.
pub const WORKSPACE_SESSION_SCHEMA_VERSION: i32 = 2;

/// Persistent fields extracted from the live runtime (never includes desktop).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentWorkspaceSession {
    pub schema_version: i32,
    pub updated_at: String,
    pub active_workspace_id: Option<String>,
    pub last_observation_pass_id: Option<String>,
    pub last_capture_timestamp: Option<String>,
    pub last_confidence_band: Option<String>,
    pub last_saved_context_id: Option<String>,
    pub last_active_monitor_index: Option<i32>,
    pub restore_history: Vec<RestoreHistoryEntry>,
    /// Durable fence for crash detection (cleared on success).
    pub pending_operation: Option<PendingOperationFence>,
    /// Last acknowledged interruption — never silently discarded.
    pub last_recovery: Option<RecoveryRecord>,
}

impl PersistentWorkspaceSession {
    pub fn empty() -> Self {
        Self {
            schema_version: WORKSPACE_SESSION_SCHEMA_VERSION,
            updated_at: observation_now_rfc3339(),
            active_workspace_id: None,
            last_observation_pass_id: None,
            last_capture_timestamp: None,
            last_confidence_band: None,
            last_saved_context_id: None,
            last_active_monitor_index: None,
            restore_history: Vec::new(),
            pending_operation: None,
            last_recovery: None,
        }
    }

    /// Extract only persistent fields from a live runtime snapshot.
    /// Success-path extraction clears the operation fence.
    pub fn from_runtime(runtime: &WorkspaceRuntimeState) -> Self {
        Self {
            schema_version: WORKSPACE_SESSION_SCHEMA_VERSION,
            updated_at: observation_now_rfc3339(),
            active_workspace_id: runtime.active_workspace_id.clone(),
            last_observation_pass_id: runtime.observation.pass_id.clone(),
            last_capture_timestamp: runtime.capture_timestamp.clone(),
            last_confidence_band: runtime.confidence_band.clone(),
            last_saved_context_id: None,
            last_active_monitor_index: runtime.active_monitor_index,
            restore_history: runtime
                .restore_history
                .iter()
                .take(WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT)
                .cloned()
                .collect(),
            pending_operation: None,
            last_recovery: None,
        }
    }

    pub fn with_saved_context_id(mut self, id: Option<String>) -> Self {
        self.last_saved_context_id = id;
        self
    }

    /// Migrate an on-disk record to the current schema. Explicit versions only.
    pub fn migrate(mut self) -> Result<Self, PersistentWorkspaceSessionError> {
        if self.schema_version > WORKSPACE_SESSION_SCHEMA_VERSION {
            return Err(PersistentWorkspaceSessionError::IncompatibleSchema {
                found: self.schema_version,
                supported: WORKSPACE_SESSION_SCHEMA_VERSION,
            });
        }
        while self.schema_version < WORKSPACE_SESSION_SCHEMA_VERSION {
            self = match self.schema_version {
                0 => migrate_v0_to_v1(self)?,
                1 => migrate_v1_to_v2(self)?,
                other => {
                    return Err(PersistentWorkspaceSessionError::IncompatibleSchema {
                        found: other,
                        supported: WORKSPACE_SESSION_SCHEMA_VERSION,
                    });
                }
            };
        }
        Ok(self)
    }

    /// FNV-1a checksum over durable payload (corruption detection).
    pub fn checksum(&self) -> String {
        let pending = self
            .pending_operation
            .as_ref()
            .and_then(|f| serde_json::to_string(f).ok())
            .unwrap_or_default();
        let recovery = self
            .last_recovery
            .as_ref()
            .and_then(|r| serde_json::to_string(r).ok())
            .unwrap_or_default();
        let material = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.schema_version,
            self.active_workspace_id.as_deref().unwrap_or(""),
            self.last_observation_pass_id.as_deref().unwrap_or(""),
            self.last_capture_timestamp.as_deref().unwrap_or(""),
            self.last_confidence_band.as_deref().unwrap_or(""),
            self.last_saved_context_id.as_deref().unwrap_or(""),
            self.last_active_monitor_index
                .map(|v| v.to_string())
                .unwrap_or_default(),
            serde_json::to_string(&self.restore_history).unwrap_or_else(|_| "[]".into()),
            pending,
            recovery,
        );
        format!("{:016x}", fnv1a64(material.as_bytes()))
    }

    pub fn validate_checksum(&self, stored: &str) -> bool {
        if stored.trim().is_empty() {
            // Legacy / empty checksum — accept and recompute on next save.
            return true;
        }
        stored == self.checksum()
    }
}

fn migrate_v0_to_v1(
    mut session: PersistentWorkspaceSession,
) -> Result<PersistentWorkspaceSession, PersistentWorkspaceSessionError> {
    session.schema_version = 1;
    session.updated_at = observation_now_rfc3339();
    if session.restore_history.len() > WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT {
        session
            .restore_history
            .truncate(WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT);
    }
    Ok(session)
}

fn migrate_v1_to_v2(
    mut session: PersistentWorkspaceSession,
) -> Result<PersistentWorkspaceSession, PersistentWorkspaceSessionError> {
    session.schema_version = 2;
    session.updated_at = observation_now_rfc3339();
    // v1 had no fences; leave pending/last_recovery as deserialized (None).
    if session.pending_operation.is_none() {
        // ok
    }
    Ok(session)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PersistentWorkspaceSessionError {
    #[error("incompatible session schema {found} (supported {supported})")]
    IncompatibleSchema { found: i32, supported: i32 },

    #[error("session payload corrupt: {0}")]
    Corrupt(String),

    #[error("session migration failed: {0}")]
    MigrationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_health::PendingOperationKind;

    #[test]
    fn migrate_identity_at_current_version() {
        let session = PersistentWorkspaceSession::empty();
        let migrated = session.clone().migrate().unwrap();
        assert_eq!(migrated.schema_version, WORKSPACE_SESSION_SCHEMA_VERSION);
        assert_eq!(migrated, session);
    }

    #[test]
    fn migrate_v0_to_v2() {
        let mut session = PersistentWorkspaceSession::empty();
        session.schema_version = 0;
        session.active_workspace_id = Some("ws-1".into());
        let migrated = session.migrate().unwrap();
        assert_eq!(migrated.schema_version, 2);
        assert_eq!(migrated.active_workspace_id.as_deref(), Some("ws-1"));
        assert!(migrated.pending_operation.is_none());
    }

    #[test]
    fn migrate_v1_to_v2() {
        let mut session = PersistentWorkspaceSession::empty();
        session.schema_version = 1;
        session.last_observation_pass_id = Some("pass-1".into());
        let migrated = session.migrate().unwrap();
        assert_eq!(migrated.schema_version, 2);
        assert_eq!(
            migrated.last_observation_pass_id.as_deref(),
            Some("pass-1")
        );
    }

    #[test]
    fn future_schema_rejected() {
        let mut session = PersistentWorkspaceSession::empty();
        session.schema_version = 99;
        assert!(matches!(
            session.migrate(),
            Err(PersistentWorkspaceSessionError::IncompatibleSchema { .. })
        ));
    }

    #[test]
    fn checksum_includes_pending_fence() {
        let mut session = PersistentWorkspaceSession::empty();
        let without = session.checksum();
        session.pending_operation = Some(PendingOperationFence::new(
            PendingOperationKind::Observation,
            Some("c1".into()),
        ));
        assert_ne!(session.checksum(), without);
        assert!(session.validate_checksum(&session.checksum()));
    }
}
