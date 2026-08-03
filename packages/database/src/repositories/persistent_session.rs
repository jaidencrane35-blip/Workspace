//! Atomic singleton store for [`PersistentWorkspaceSession`].

use workspace_domain::{PersistentWorkspaceSession, WORKSPACE_SESSION_SCHEMA_VERSION};

use crate::connection::Database;
use crate::error::{DatabaseError, Result};

const SINGLETON_ID: &str = "singleton";

pub struct PersistentWorkspaceSessionRepository<'a> {
    db: &'a Database,
}

impl<'a> PersistentWorkspaceSessionRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Load singleton session. Missing row → `None`. Corrupt checksum/JSON → error.
    pub fn load(&self) -> Result<Option<PersistentWorkspaceSession>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT schema_version, updated_at, active_workspace_id, last_observation_pass_id,
                    last_capture_timestamp, last_confidence_band, last_saved_context_id,
                    last_active_monitor_index, restore_history_json, payload_checksum,
                    pending_operation_json, last_recovery_json
             FROM workspace_persistent_session WHERE id = ?1",
        )?;
        let mut rows = stmt.query([SINGLETON_ID])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };

        let schema_version: i32 = row.get(0)?;
        let updated_at: String = row.get(1)?;
        let active_workspace_id: Option<String> = row.get(2)?;
        let last_observation_pass_id: Option<String> = row.get(3)?;
        let last_capture_timestamp: Option<String> = row.get(4)?;
        let last_confidence_band: Option<String> = row.get(5)?;
        let last_saved_context_id: Option<String> = row.get(6)?;
        let last_active_monitor_index: Option<i32> = row.get(7)?;
        let restore_history_json: String = row.get(8)?;
        let payload_checksum: String = row.get(9)?;
        let pending_operation_json: Option<String> = row.get(10)?;
        let last_recovery_json: Option<String> = row.get(11)?;

        let restore_history = serde_json::from_str(&restore_history_json).map_err(|error| {
            DatabaseError::Migration(format!("session restore_history corrupt: {error}"))
        })?;

        let pending_operation = match pending_operation_json.as_deref() {
            None | Some("") => None,
            Some(raw) => Some(serde_json::from_str(raw).map_err(|error| {
                DatabaseError::Migration(format!("session pending_operation corrupt: {error}"))
            })?),
        };

        let last_recovery = match last_recovery_json.as_deref() {
            None | Some("") => None,
            Some(raw) => Some(serde_json::from_str(raw).map_err(|error| {
                DatabaseError::Migration(format!("session last_recovery corrupt: {error}"))
            })?),
        };

        let session = PersistentWorkspaceSession {
            schema_version,
            updated_at,
            active_workspace_id,
            last_observation_pass_id,
            last_capture_timestamp,
            last_confidence_band,
            last_saved_context_id,
            last_active_monitor_index,
            restore_history,
            pending_operation,
            last_recovery,
        };

        if !session.validate_checksum(&payload_checksum) {
            return Err(DatabaseError::Migration(
                "session payload_checksum mismatch".into(),
            ));
        }

        Ok(Some(session))
    }

    /// Atomic replace of the singleton row (INSERT … ON CONFLICT DO UPDATE).
    pub fn save(&self, session: &PersistentWorkspaceSession) -> Result<()> {
        if session.schema_version != WORKSPACE_SESSION_SCHEMA_VERSION {
            return Err(DatabaseError::Migration(format!(
                "refusing to persist session schema {}",
                session.schema_version
            )));
        }
        let history_json = serde_json::to_string(&session.restore_history).map_err(|error| {
            DatabaseError::Migration(format!("session history serialize: {error}"))
        })?;
        let pending_json = match &session.pending_operation {
            Some(fence) => Some(serde_json::to_string(fence).map_err(|error| {
                DatabaseError::Migration(format!("session pending serialize: {error}"))
            })?),
            None => None,
        };
        let recovery_json = match &session.last_recovery {
            Some(record) => Some(serde_json::to_string(record).map_err(|error| {
                DatabaseError::Migration(format!("session recovery serialize: {error}"))
            })?),
            None => None,
        };
        let checksum = session.checksum();

        self.db.transaction(|tx| {
            tx.connection().execute(
                "INSERT INTO workspace_persistent_session (
                    id, schema_version, updated_at, active_workspace_id,
                    last_observation_pass_id, last_capture_timestamp, last_confidence_band,
                    last_saved_context_id, last_active_monitor_index, restore_history_json,
                    payload_checksum, pending_operation_json, last_recovery_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                 ON CONFLICT(id) DO UPDATE SET
                    schema_version = excluded.schema_version,
                    updated_at = excluded.updated_at,
                    active_workspace_id = excluded.active_workspace_id,
                    last_observation_pass_id = excluded.last_observation_pass_id,
                    last_capture_timestamp = excluded.last_capture_timestamp,
                    last_confidence_band = excluded.last_confidence_band,
                    last_saved_context_id = excluded.last_saved_context_id,
                    last_active_monitor_index = excluded.last_active_monitor_index,
                    restore_history_json = excluded.restore_history_json,
                    payload_checksum = excluded.payload_checksum,
                    pending_operation_json = excluded.pending_operation_json,
                    last_recovery_json = excluded.last_recovery_json",
                (
                    SINGLETON_ID,
                    session.schema_version,
                    session.updated_at.as_str(),
                    session.active_workspace_id.as_deref(),
                    session.last_observation_pass_id.as_deref(),
                    session.last_capture_timestamp.as_deref(),
                    session.last_confidence_band.as_deref(),
                    session.last_saved_context_id.as_deref(),
                    session.last_active_monitor_index,
                    history_json.as_str(),
                    checksum.as_str(),
                    pending_json.as_deref(),
                    recovery_json.as_deref(),
                ),
            )?;
            Ok(())
        })
    }

    /// Delete singleton (recovery / tests).
    pub fn clear(&self) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM workspace_persistent_session WHERE id = ?1",
            [SINGLETON_ID],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{
        OperationOutcome, PendingOperationFence, PendingOperationKind, RestoreExecutionSummary,
        RestoreHistoryEntry,
    };

    fn db() -> crate::connection::Database {
        let database = crate::connection::Database::open_in_memory().unwrap();
        crate::migration::MigrationRunner::bundled()
            .apply_all(&database)
            .unwrap();
        database
    }

    #[test]
    fn save_load_round_trip() {
        let database = db();
        let repo = PersistentWorkspaceSessionRepository::new(&database);
        let mut session = PersistentWorkspaceSession::empty();
        session.active_workspace_id = Some("ws-1".into());
        session.last_observation_pass_id = Some("pass-1".into());
        session.restore_history.push(RestoreHistoryEntry {
            recorded_at: "2026-08-03T00:00:00Z".into(),
            operation_id: "op-1".into(),
            outcome: OperationOutcome::Completed,
            summary: RestoreExecutionSummary {
                restored_windows: 2,
                skipped_windows: 0,
                missing_applications: 0,
                failed_operations: 0,
                duration_ms: 5,
            },
            purpose: "Resume".into(),
        });
        repo.save(&session).unwrap();
        let loaded = repo.load().unwrap().expect("row");
        assert_eq!(loaded.active_workspace_id.as_deref(), Some("ws-1"));
        assert_eq!(loaded.restore_history.len(), 1);
        assert_eq!(loaded.schema_version, WORKSPACE_SESSION_SCHEMA_VERSION);
    }

    #[test]
    fn missing_session_is_none() {
        let database = db();
        let repo = PersistentWorkspaceSessionRepository::new(&database);
        assert!(repo.load().unwrap().is_none());
    }

    #[test]
    fn corrupt_checksum_detected() {
        let database = db();
        let repo = PersistentWorkspaceSessionRepository::new(&database);
        let session = PersistentWorkspaceSession::empty();
        repo.save(&session).unwrap();
        database
            .connection()
            .execute(
                "UPDATE workspace_persistent_session SET payload_checksum = 'bad' WHERE id = 'singleton'",
                [],
            )
            .unwrap();
        let err = repo.load().unwrap_err();
        assert!(err.to_string().contains("checksum"));
    }

    #[test]
    fn atomic_replace_overwrites() {
        let database = db();
        let repo = PersistentWorkspaceSessionRepository::new(&database);
        let mut first = PersistentWorkspaceSession::empty();
        first.active_workspace_id = Some("a".into());
        repo.save(&first).unwrap();
        let mut second = PersistentWorkspaceSession::empty();
        second.active_workspace_id = Some("b".into());
        repo.save(&second).unwrap();
        assert_eq!(
            repo.load()
                .unwrap()
                .unwrap()
                .active_workspace_id
                .as_deref(),
            Some("b")
        );
    }

    #[test]
    fn pending_fence_round_trip() {
        let database = db();
        let repo = PersistentWorkspaceSessionRepository::new(&database);
        let mut session = PersistentWorkspaceSession::empty();
        session.pending_operation = Some(PendingOperationFence::new(
            PendingOperationKind::RestoreExecution,
            Some("op-fence".into()),
        ));
        repo.save(&session).unwrap();
        let loaded = repo.load().unwrap().unwrap();
        assert_eq!(
            loaded.pending_operation.as_ref().map(|f| f.kind),
            Some(PendingOperationKind::RestoreExecution)
        );
        assert_eq!(
            loaded
                .pending_operation
                .as_ref()
                .and_then(|f| f.correlation_id.as_deref()),
            Some("op-fence")
        );
    }
}
