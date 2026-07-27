use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{DecisionEngineIntakeCandidate, DecisionEngineOverlay, DecisionOutcome};

/// Persistence for Decision Engine lifecycle overlay and intake candidates.
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

    /// Insert or update a DE-owned intake candidate. Preserves created_at on conflict.
    pub fn upsert_intake_candidate(&self, candidate: &DecisionEngineIntakeCandidate) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO decision_engine_intake_candidate (
                workspace_id, intake_candidate_id, intake_receipt_reference,
                recommendation_reference, package_seal_digest, acceptance_reference,
                compatibility_version, state, created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
             ON CONFLICT(workspace_id, intake_candidate_id) DO UPDATE SET
                intake_receipt_reference = excluded.intake_receipt_reference,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                acceptance_reference = excluded.acceptance_reference,
                compatibility_version = excluded.compatibility_version,
                state = excluded.state,
                updated_at = excluded.updated_at",
            (
                &candidate.workspace_id,
                &candidate.intake_candidate_id,
                &candidate.intake_receipt_reference,
                &candidate.recommendation_reference,
                &candidate.package_seal_digest,
                &candidate.acceptance_reference,
                &candidate.compatibility_version,
                &candidate.state,
                &candidate.created_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_intake_candidates(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineIntakeCandidate>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, intake_candidate_id, intake_receipt_reference,
                    recommendation_reference, package_seal_digest, acceptance_reference,
                    compatibility_version, state, created_at
             FROM decision_engine_intake_candidate
             WHERE workspace_id = ?1
             ORDER BY intake_candidate_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_intake_candidate)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn get_intake_candidate_by_digest(
        &self,
        workspace_id: &str,
        package_seal_digest: &str,
    ) -> Result<Option<DecisionEngineIntakeCandidate>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, intake_candidate_id, intake_receipt_reference,
                    recommendation_reference, package_seal_digest, acceptance_reference,
                    compatibility_version, state, created_at
             FROM decision_engine_intake_candidate
             WHERE workspace_id = ?1 AND package_seal_digest = ?2
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map((workspace_id, package_seal_digest), map_intake_candidate)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
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

fn map_intake_candidate(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionEngineIntakeCandidate> {
    Ok(DecisionEngineIntakeCandidate {
        workspace_id: row.get(0)?,
        intake_candidate_id: row.get(1)?,
        intake_receipt_reference: row.get(2)?,
        recommendation_reference: row.get(3)?,
        package_seal_digest: row.get(4)?,
        acceptance_reference: row.get(5)?,
        compatibility_version: row.get(6)?,
        state: row.get(7)?,
        created_at: row.get(8)?,
        is_decision_candidate: false,
        creates_decision_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: "Decision Engine intake candidate (persisted acknowledgement). \
               Not a DecisionCandidate, goal, intent, planner handoff, or ownership transfer."
            .into(),
        authority_effect: DecisionEngineIntakeCandidate::AUTHORITY_EFFECT_NONE.into(),
    })
}
