use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{ExecutionLifecycleRecord, ExecutionState};

/// Persistence for durable governed-execution claims and completion facts.
pub struct ExecutionLifecycleRepository<'a> {
    db: &'a Database,
}

impl<'a> ExecutionLifecycleRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Atomically claims an execution identity. Returns false when it exists.
    pub fn insert_claim(&self, record: &ExecutionLifecycleRecord) -> Result<bool> {
        let changed = self.db.connection().execute(
            "INSERT INTO execution_lifecycle (
                execution_request_id, suggestion_id, intent_id, state, retry_allowed,
                failure_reason, claimed_at, completed_at, updated_at
             ) VALUES (?1, ?2, ?3, 'in_progress', 0, NULL, ?4, NULL, ?4)
             ON CONFLICT(execution_request_id) DO UPDATE SET
                suggestion_id = excluded.suggestion_id,
                intent_id = excluded.intent_id,
                state = 'in_progress',
                retry_allowed = 0,
                failure_reason = NULL,
                claimed_at = excluded.claimed_at,
                completed_at = NULL,
                updated_at = excluded.updated_at
             WHERE execution_lifecycle.state = 'cancelled'
                OR (execution_lifecycle.state = 'failed'
                    AND execution_lifecycle.retry_allowed = 1)",
            (
                &record.execution_request_id,
                &record.suggestion_id,
                &record.intent_id,
                &record.claimed_at,
            ),
        )?;
        Ok(changed == 1)
    }

    pub fn get(&self, execution_request_id: &str) -> Result<Option<ExecutionLifecycleRecord>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT execution_request_id, suggestion_id, intent_id, state,
                    retry_allowed, failure_reason, claimed_at, completed_at, updated_at
             FROM execution_lifecycle
             WHERE execution_request_id = ?1
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map([execution_request_id], map_record)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Moves an existing claim to its terminal completed state.
    pub fn mark_completed(
        &self,
        execution_request_id: &str,
        intent_id: Option<&str>,
        completed_at: &str,
    ) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE execution_lifecycle
             SET state = 'completed',
                 intent_id = COALESCE(?2, intent_id),
                 retry_allowed = 0,
                 failure_reason = NULL,
                 completed_at = ?3,
                 updated_at = ?3
             WHERE execution_request_id = ?1 AND state = 'in_progress'",
            (execution_request_id, intent_id, completed_at),
        )?;
        Ok(changed == 1)
    }

    pub fn mark_failed(
        &self,
        execution_request_id: &str,
        retry_allowed: bool,
        failure_reason: &str,
        failed_at: &str,
    ) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE execution_lifecycle
             SET state = 'failed',
                 retry_allowed = ?2,
                 failure_reason = ?3,
                 completed_at = NULL,
                 updated_at = ?4
             WHERE execution_request_id = ?1 AND state = 'in_progress'",
            (
                execution_request_id,
                i32::from(retry_allowed),
                failure_reason,
                failed_at,
            ),
        )?;
        Ok(changed == 1)
    }

    pub fn record_cancelled(
        &self,
        execution_request_id: &str,
        suggestion_id: &str,
        cancelled_at: &str,
    ) -> Result<bool> {
        let changed = self.db.connection().execute(
            "INSERT INTO execution_lifecycle (
                execution_request_id, suggestion_id, intent_id, state, retry_allowed,
                failure_reason, claimed_at, completed_at, updated_at
             ) VALUES (?1, ?2, NULL, 'cancelled', 1, NULL, ?3, NULL, ?3)
             ON CONFLICT(execution_request_id) DO UPDATE SET
                state = 'cancelled',
                retry_allowed = 1,
                failure_reason = NULL,
                completed_at = NULL,
                updated_at = excluded.updated_at
             WHERE execution_lifecycle.state = 'failed'
               AND execution_lifecycle.retry_allowed = 1",
            (execution_request_id, suggestion_id, cancelled_at),
        )?;
        Ok(changed == 1)
    }

    pub fn list_recent(&self, limit: usize) -> Result<Vec<ExecutionLifecycleRecord>> {
        let limit = limit.clamp(1, 500) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT execution_request_id, suggestion_id, intent_id, state,
                    retry_allowed, failure_reason, claimed_at, completed_at, updated_at
             FROM execution_lifecycle
             ORDER BY updated_at DESC, execution_request_id ASC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], map_record)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<ExecutionLifecycleRecord> {
    let state_raw: String = row.get(3)?;
    let state = ExecutionState::parse(&state_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(3, "state".into(), rusqlite::types::Type::Text)
    })?;
    Ok(ExecutionLifecycleRecord {
        execution_request_id: row.get(0)?,
        suggestion_id: row.get(1)?,
        intent_id: row.get(2)?,
        state,
        retry_allowed: row.get::<_, i32>(4)? != 0,
        failure_reason: row.get(5)?,
        claimed_at: row.get(6)?,
        completed_at: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;

    fn claim() -> ExecutionLifecycleRecord {
        ExecutionLifecycleRecord {
            execution_request_id: "execution:s-1".into(),
            suggestion_id: "s-1".into(),
            intent_id: Some("intent:test".into()),
            state: ExecutionState::InProgress,
            retry_allowed: false,
            failure_reason: None,
            claimed_at: "2026-07-28T00:00:00Z".into(),
            completed_at: None,
            updated_at: "2026-07-28T00:00:00Z".into(),
        }
    }

    #[test]
    fn claim_is_unique_and_completion_is_terminal() {
        let dir = tempfile::tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        let repository = ExecutionLifecycleRepository::new(&db);
        let record = claim();

        assert!(repository.insert_claim(&record).unwrap());
        assert!(!repository.insert_claim(&record).unwrap());
        assert!(repository
            .mark_completed(
                &record.execution_request_id,
                record.intent_id.as_deref(),
                "2026-07-28T00:01:00Z",
            )
            .unwrap());
        assert!(!repository
            .mark_failed(
                &record.execution_request_id,
                true,
                "must remain completed",
                "2026-07-28T00:02:00Z",
            )
            .unwrap());
        assert!(!repository
            .mark_completed(
                &record.execution_request_id,
                record.intent_id.as_deref(),
                "2026-07-28T00:02:00Z",
            )
            .unwrap());

        let stored = repository
            .get(&record.execution_request_id)
            .unwrap()
            .unwrap();
        assert_eq!(stored.state, ExecutionState::Completed);
        assert_eq!(
            stored.completed_at.as_deref(),
            Some("2026-07-28T00:01:00Z")
        );
    }

    #[test]
    fn restored_041_upgrades_and_backfills_historical_outcomes() {
        let db = Database::open_in_memory().unwrap();
        db.connection()
            .execute_batch(include_str!("../../migrations/004_audit.sql"))
            .unwrap();
        db.connection()
            .execute(
                "INSERT INTO audit_events (
                    id, timestamp, event_type, actor_type, actor_id,
                    command_name, success, metadata
                 ) VALUES (?1, ?2, 'command.executed', 'local_user', 'local-user',
                           'ExecuteIntentRequest', 1, ?3)",
                (
                    "audit-1",
                    "2026-07-28T00:00:00Z",
                    r#"{"execution_request":true,"execution_request_id":"execution:legacy","suggestion_id":"legacy","intent_id":"intent:legacy","execution_status":"executed"}"#,
                ),
            )
            .unwrap();
        db.connection().execute(
            "INSERT INTO audit_events (
                id, timestamp, event_type, actor_type, actor_id,
                command_name, success, metadata
             ) VALUES ('audit-2', '2026-07-28T00:01:00Z', 'command.executed',
                       'local_user', 'local-user', 'RequestExecutionCancellation', 1, ?1)",
            [r#"{"execution_request_id":"execution:cancelled-legacy","cancellation_status":"requested"}"#],
        ).unwrap();
        db.connection()
            .execute_batch(include_str!("../../migrations/041_execution_lifecycle.sql"))
            .unwrap();
        db.connection()
            .execute_batch(include_str!(
                "../../migrations/042_execution_lifecycle_failure_reconciliation.sql"
            ))
            .unwrap();

        let record = ExecutionLifecycleRepository::new(&db)
            .get("execution:legacy")
            .unwrap()
            .unwrap();
        assert_eq!(record.state, ExecutionState::Completed);
        assert_eq!(record.suggestion_id, "legacy");
        assert_eq!(
            ExecutionLifecycleRepository::new(&db)
                .get("execution:cancelled-legacy")
                .unwrap()
                .unwrap()
                .state,
            ExecutionState::Cancelled
        );
    }

    #[test]
    fn fresh_database_applies_042_constraints() {
        let dir = tempfile::tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        let repository = ExecutionLifecycleRepository::new(&db);
        assert!(repository
            .record_cancelled("execution:fresh", "fresh", "2026-07-28T00:00:00Z")
            .unwrap());
        let cancelled = repository.get("execution:fresh").unwrap().unwrap();
        assert_eq!(cancelled.state, ExecutionState::Cancelled);
        assert!(cancelled.retry_allowed);
        assert!(db
            .connection()
            .execute(
                "INSERT INTO execution_lifecycle (
                    execution_request_id, suggestion_id, state, retry_allowed,
                    claimed_at, updated_at
                 ) VALUES ('execution:invalid', 'invalid', 'cancelled', 0, 't', 't')",
                [],
            )
            .is_err());
    }

    #[test]
    fn amended_041_cancelled_rows_upgrade_without_data_loss() {
        let db = Database::open_in_memory().unwrap();
        db.connection()
            .execute_batch(include_str!("../../migrations/004_audit.sql"))
            .unwrap();
        db.connection()
            .execute_batch(
                "CREATE TABLE execution_lifecycle (
                    execution_request_id TEXT PRIMARY KEY NOT NULL,
                    suggestion_id TEXT NOT NULL,
                    intent_id TEXT,
                    state TEXT NOT NULL CHECK (state IN ('in_progress', 'completed', 'cancelled')),
                    claimed_at TEXT NOT NULL,
                    completed_at TEXT,
                    updated_at TEXT NOT NULL,
                    CHECK (
                        (state IN ('in_progress', 'cancelled') AND completed_at IS NULL)
                        OR (state = 'completed' AND completed_at IS NOT NULL)
                    )
                 );
                 CREATE INDEX idx_execution_lifecycle_updated
                    ON execution_lifecycle (updated_at DESC);
                 INSERT INTO execution_lifecycle VALUES
                    ('execution:cancelled-dev', 'cancelled-dev', NULL, 'cancelled',
                     '2026-07-28T00:00:00Z', NULL, '2026-07-28T00:00:00Z'),
                    ('execution:active-dev', 'active-dev', NULL, 'in_progress',
                     '2026-07-28T00:01:00Z', NULL, '2026-07-28T00:01:00Z'),
                    ('execution:done-dev', 'done-dev', 'intent:done', 'completed',
                     '2026-07-28T00:02:00Z', '2026-07-28T00:03:00Z',
                     '2026-07-28T00:03:00Z');",
            )
            .unwrap();
        db.connection()
            .execute_batch(include_str!(
                "../../migrations/042_execution_lifecycle_failure_reconciliation.sql"
            ))
            .unwrap();

        let repository = ExecutionLifecycleRepository::new(&db);
        let cancelled = repository
            .get("execution:cancelled-dev")
            .unwrap()
            .unwrap();
        assert_eq!(cancelled.state, ExecutionState::Cancelled);
        assert!(cancelled.retry_allowed);
        assert_eq!(
            repository
                .get("execution:active-dev")
                .unwrap()
                .unwrap()
                .state,
            ExecutionState::InProgress
        );
        assert_eq!(
            repository
                .get("execution:done-dev")
                .unwrap()
                .unwrap()
                .state,
            ExecutionState::Completed
        );
        assert_eq!(
            db.connection()
                .query_row("SELECT COUNT(*) FROM execution_lifecycle", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            3
        );
    }

    #[test]
    fn historical_failed_row_is_preserved_nonretryable() {
        let db = Database::open_in_memory().unwrap();
        db.connection()
            .execute_batch(include_str!("../../migrations/004_audit.sql"))
            .unwrap();
        db.connection()
            .execute_batch(
                "CREATE TABLE execution_lifecycle (
                    execution_request_id TEXT PRIMARY KEY NOT NULL,
                    suggestion_id TEXT NOT NULL,
                    intent_id TEXT,
                    state TEXT NOT NULL,
                    claimed_at TEXT NOT NULL,
                    completed_at TEXT,
                    updated_at TEXT NOT NULL
                 );
                 CREATE INDEX idx_execution_lifecycle_updated
                    ON execution_lifecycle (updated_at DESC);
                 INSERT INTO execution_lifecycle VALUES (
                    'execution:failed-dev', 'failed-dev', NULL, 'failed',
                    '2026-07-28T00:00:00Z', NULL, '2026-07-28T00:01:00Z'
                 );",
            )
            .unwrap();
        db.connection()
            .execute_batch(include_str!(
                "../../migrations/042_execution_lifecycle_failure_reconciliation.sql"
            ))
            .unwrap();
        let failed = ExecutionLifecycleRepository::new(&db)
            .get("execution:failed-dev")
            .unwrap()
            .unwrap();
        assert_eq!(failed.state, ExecutionState::Failed);
        assert!(!failed.retry_allowed);
        assert_eq!(
            failed.failure_reason.as_deref(),
            Some("historical failure requires reconciliation")
        );
    }

    #[test]
    fn cancelled_execution_can_be_reclaimed_for_retry() {
        let dir = tempfile::tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        let repository = ExecutionLifecycleRepository::new(&db);
        assert!(repository
            .record_cancelled(
                "execution:cancelled",
                "cancelled",
                "2026-07-28T00:00:00Z",
            )
            .unwrap());
        assert!(!repository
            .record_cancelled(
                "execution:cancelled",
                "cancelled",
                "2026-07-28T00:01:00Z",
            )
            .unwrap());

        let mut retry = claim();
        retry.execution_request_id = "execution:cancelled".into();
        retry.suggestion_id = "cancelled".into();
        assert!(repository.insert_claim(&retry).unwrap());
        assert_eq!(
            repository
                .get(&retry.execution_request_id)
                .unwrap()
                .unwrap()
                .state,
            ExecutionState::InProgress
        );
    }

    #[test]
    fn retryable_failed_execution_can_be_reclaimed_but_terminal_failed_cannot() {
        let dir = tempfile::tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        let repository = ExecutionLifecycleRepository::new(&db);
        let mut retryable = claim();
        retryable.execution_request_id = "execution:retryable".into();
        retryable.suggestion_id = "retryable".into();
        assert!(repository.insert_claim(&retryable).unwrap());
        assert!(repository.mark_failed(
            &retryable.execution_request_id,
            true,
            "rolled back",
            "2026-07-28T00:01:00Z",
        ).unwrap());
        assert!(repository.insert_claim(&retryable).unwrap());

        let mut terminal = claim();
        terminal.execution_request_id = "execution:terminal-failed".into();
        terminal.suggestion_id = "terminal-failed".into();
        assert!(repository.insert_claim(&terminal).unwrap());
        assert!(repository.mark_failed(
            &terminal.execution_request_id,
            false,
            "outcome uncertain",
            "2026-07-28T00:01:00Z",
        ).unwrap());
        assert!(!repository.insert_claim(&terminal).unwrap());
    }
}
