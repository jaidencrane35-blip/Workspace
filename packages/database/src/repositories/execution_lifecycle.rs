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
                execution_request_id, suggestion_id, intent_id, state,
                claimed_at, completed_at, updated_at
             ) VALUES (?1, ?2, ?3, 'in_progress', ?4, NULL, ?4)
             ON CONFLICT(execution_request_id) DO NOTHING",
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
                    claimed_at, completed_at, updated_at
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
                 completed_at = ?3,
                 updated_at = ?3
             WHERE execution_request_id = ?1 AND state = 'in_progress'",
            (execution_request_id, intent_id, completed_at),
        )?;
        Ok(changed == 1)
    }

    /// Releases only a non-terminal claim after dispatch returned an error.
    pub fn release_in_progress(&self, execution_request_id: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "DELETE FROM execution_lifecycle
             WHERE execution_request_id = ?1 AND state = 'in_progress'",
            [execution_request_id],
        )?;
        Ok(changed == 1)
    }

    pub fn list_recent(&self, limit: usize) -> Result<Vec<ExecutionLifecycleRecord>> {
        let limit = limit.clamp(1, 500) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT execution_request_id, suggestion_id, intent_id, state,
                    claimed_at, completed_at, updated_at
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
        claimed_at: row.get(4)?,
        completed_at: row.get(5)?,
        updated_at: row.get(6)?,
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
            .release_in_progress(&record.execution_request_id)
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
    fn migration_backfills_historical_completed_execution() {
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
        db.connection()
            .execute_batch(include_str!("../../migrations/041_execution_lifecycle.sql"))
            .unwrap();

        let record = ExecutionLifecycleRepository::new(&db)
            .get("execution:legacy")
            .unwrap()
            .unwrap();
        assert_eq!(record.state, ExecutionState::Completed);
        assert_eq!(record.suggestion_id, "legacy");
    }
}
