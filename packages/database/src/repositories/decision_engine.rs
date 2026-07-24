use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{DecisionEngineOverlay, DecisionOutcome};

/// Persistence for Decision Engine lifecycle overlay only.
pub struct DecisionEngineRepository<'a> {
    db: &'a Database,
}

impl<'a> DecisionEngineRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_overlay(&self, overlay: &DecisionEngineOverlay) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO decision_engine_lifecycle (
                workspace_id, candidate_key, outcome, updated_at, actor_id
             ) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(workspace_id, candidate_key) DO UPDATE SET
                outcome = excluded.outcome,
                updated_at = excluded.updated_at,
                actor_id = excluded.actor_id",
            (
                &overlay.workspace_id,
                &overlay.candidate_key,
                overlay.outcome.as_str(),
                &overlay.updated_at,
                &overlay.actor_id,
            ),
        )?;
        Ok(())
    }

    pub fn list_overlays(&self, workspace_id: &str) -> Result<Vec<DecisionEngineOverlay>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, candidate_key, outcome, updated_at, actor_id
             FROM decision_engine_lifecycle
             WHERE workspace_id = ?1",
        )?;
        let rows = stmt.query_map([workspace_id], map_overlay)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn delete_overlay(&self, workspace_id: &str, candidate_key: &str) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM decision_engine_lifecycle
             WHERE workspace_id = ?1 AND candidate_key = ?2",
            (workspace_id, candidate_key),
        )?;
        Ok(())
    }
}

fn map_overlay(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionEngineOverlay> {
    let outcome_raw: String = row.get(2)?;
    let outcome = DecisionOutcome::parse(&outcome_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "outcome".into(), rusqlite::types::Type::Text)
    })?;
    Ok(DecisionEngineOverlay {
        workspace_id: row.get(0)?,
        candidate_key: row.get(1)?,
        outcome,
        updated_at: row.get(3)?,
        actor_id: row.get(4)?,
    })
}
