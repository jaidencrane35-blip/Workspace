use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    DecisionEngineIntakeCandidate, DecisionEngineIntakeCandidateLifecycle,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEvaluation, DecisionEngineOverlay,
    DecisionOutcome,
};

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
                compatibility_version, state, created_at, updated_at,
                lifecycle_state, lifecycle_reason, lifecycle_updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(workspace_id, intake_candidate_id) DO UPDATE SET
                intake_receipt_reference = excluded.intake_receipt_reference,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                acceptance_reference = excluded.acceptance_reference,
                compatibility_version = excluded.compatibility_version,
                state = excluded.state,
                updated_at = excluded.updated_at,
                lifecycle_state = excluded.lifecycle_state,
                lifecycle_reason = excluded.lifecycle_reason,
                lifecycle_updated_at = excluded.lifecycle_updated_at",
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
                &candidate.lifecycle.updated_at,
                &candidate.lifecycle.lifecycle_state,
                &candidate.lifecycle.reason,
                &candidate.lifecycle.updated_at,
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
                    compatibility_version, state, created_at,
                    lifecycle_state, lifecycle_reason, lifecycle_updated_at
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
                    compatibility_version, state, created_at,
                    lifecycle_state, lifecycle_reason, lifecycle_updated_at
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

    pub fn get_intake_candidate(
        &self,
        workspace_id: &str,
        intake_candidate_id: &str,
    ) -> Result<Option<DecisionEngineIntakeCandidate>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, intake_candidate_id, intake_receipt_reference,
                    recommendation_reference, package_seal_digest, acceptance_reference,
                    compatibility_version, state, created_at,
                    lifecycle_state, lifecycle_reason, lifecycle_updated_at
             FROM decision_engine_intake_candidate
             WHERE workspace_id = ?1 AND intake_candidate_id = ?2
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map((workspace_id, intake_candidate_id), map_intake_candidate)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn upsert_intake_evaluation(
        &self,
        evaluation: &DecisionEngineIntakeEvaluation,
    ) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO decision_engine_intake_evaluation (
                workspace_id, evaluation_id, intake_candidate_id, evaluated_at,
                evaluation_state, evaluation_reason, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?4)
             ON CONFLICT(workspace_id, evaluation_id) DO UPDATE SET
                intake_candidate_id = excluded.intake_candidate_id,
                evaluated_at = excluded.evaluated_at,
                evaluation_state = excluded.evaluation_state,
                evaluation_reason = excluded.evaluation_reason,
                updated_at = excluded.updated_at",
            (
                &evaluation.workspace_id,
                &evaluation.evaluation_id,
                &evaluation.intake_candidate_id,
                &evaluation.evaluated_at,
                &evaluation.evaluation_state,
                &evaluation.evaluation_reason,
            ),
        )?;
        Ok(())
    }

    pub fn list_intake_evaluations(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineIntakeEvaluation>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, evaluation_id, intake_candidate_id, evaluated_at,
                    evaluation_state, evaluation_reason
             FROM decision_engine_intake_evaluation
             WHERE workspace_id = ?1
             ORDER BY evaluation_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_intake_evaluation)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn get_intake_evaluation_for_candidate(
        &self,
        workspace_id: &str,
        intake_candidate_id: &str,
    ) -> Result<Option<DecisionEngineIntakeEvaluation>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, evaluation_id, intake_candidate_id, evaluated_at,
                    evaluation_state, evaluation_reason
             FROM decision_engine_intake_evaluation
             WHERE workspace_id = ?1 AND intake_candidate_id = ?2
             LIMIT 1",
        )?;
        let mut rows =
            stmt.query_map((workspace_id, intake_candidate_id), map_intake_evaluation)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn upsert_intake_disposition(
        &self,
        disposition: &DecisionEngineIntakeDisposition,
    ) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO decision_engine_intake_disposition (
                workspace_id, disposition_id, intake_candidate_id, evaluation_id,
                disposed_at, disposition_state, disposition_reason, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?5)
             ON CONFLICT(workspace_id, disposition_id) DO UPDATE SET
                intake_candidate_id = excluded.intake_candidate_id,
                evaluation_id = excluded.evaluation_id,
                disposed_at = excluded.disposed_at,
                disposition_state = excluded.disposition_state,
                disposition_reason = excluded.disposition_reason,
                updated_at = excluded.updated_at",
            (
                &disposition.workspace_id,
                &disposition.disposition_id,
                &disposition.intake_candidate_id,
                &disposition.evaluation_id,
                &disposition.disposed_at,
                &disposition.disposition_state,
                &disposition.disposition_reason,
            ),
        )?;
        Ok(())
    }

    pub fn list_intake_dispositions(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineIntakeDisposition>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, disposition_id, intake_candidate_id, evaluation_id,
                    disposed_at, disposition_state, disposition_reason
             FROM decision_engine_intake_disposition
             WHERE workspace_id = ?1
             ORDER BY disposition_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_intake_disposition)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
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
    let created_at: String = row.get(8)?;
    let lifecycle_state: String = row
        .get::<_, Option<String>>(9)?
        .unwrap_or_else(|| DecisionEngineIntakeCandidateLifecycle::STATE_ACTIVE.into());
    let lifecycle_reason: Option<String> = row.get(10)?;
    let lifecycle_updated_at: String = row
        .get::<_, Option<String>>(11)?
        .unwrap_or_else(|| created_at.clone());
    Ok(DecisionEngineIntakeCandidate {
        workspace_id: row.get(0)?,
        intake_candidate_id: row.get(1)?,
        intake_receipt_reference: row.get(2)?,
        recommendation_reference: row.get(3)?,
        package_seal_digest: row.get(4)?,
        acceptance_reference: row.get(5)?,
        compatibility_version: row.get(6)?,
        state: row.get(7)?,
        created_at,
        lifecycle: DecisionEngineIntakeCandidateLifecycle {
            lifecycle_state,
            reason: lifecycle_reason,
            updated_at: lifecycle_updated_at,
            authority_effect: DecisionEngineIntakeCandidateLifecycle::AUTHORITY_EFFECT_NONE.into(),
        },
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

fn map_intake_evaluation(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionEngineIntakeEvaluation> {
    let evaluation_state: String = row.get(4)?;
    Ok(DecisionEngineIntakeEvaluation {
        workspace_id: row.get(0)?,
        evaluation_id: row.get(1)?,
        intake_candidate_id: row.get(2)?,
        evaluated_at: row.get(3)?,
        evaluation_state: evaluation_state.clone(),
        evaluation_reason: row.get(5)?,
        creates_decision_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: format!(
            "Decision Engine intake evaluation ({evaluation_state}). Examination record only."
        ),
        authority_effect: DecisionEngineIntakeEvaluation::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_intake_disposition(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<DecisionEngineIntakeDisposition> {
    let disposition_state: String = row.get(5)?;
    Ok(DecisionEngineIntakeDisposition {
        workspace_id: row.get(0)?,
        disposition_id: row.get(1)?,
        intake_candidate_id: row.get(2)?,
        evaluation_id: row.get(3)?,
        disposed_at: row.get(4)?,
        disposition_state: disposition_state.clone(),
        disposition_reason: row.get(6)?,
        creates_decision_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: format!(
            "Decision Engine intake disposition ({disposition_state}). Lifecycle decision only."
        ),
        authority_effect: DecisionEngineIntakeDisposition::AUTHORITY_EFFECT_NONE.into(),
    })
}
