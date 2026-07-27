use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    DecisionCandidateEvaluationOriginContract, DecisionCandidateEvaluationResolution,
    DecisionCandidateScore, DecisionCandidateSelection, DecisionEngineCandidateCreation,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeCandidateLifecycle,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEvaluation, DecisionEngineOverlay,
    DecisionOutcome, DecisionScore,
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

    /// Persist a created DecisionCandidate provenance record (DE-owned only).
    pub fn upsert_candidate_creation(
        &self,
        creation: &DecisionEngineCandidateCreation,
    ) -> Result<()> {
        if !creation.is_created() {
            return Ok(());
        }
        let decision_candidate_id = creation
            .decision_candidate_id
            .as_deref()
            .unwrap_or_default();
        let title = creation.title.as_deref().unwrap_or_default();
        let goal_statement = creation.goal_statement.as_deref().unwrap_or_default();
        let created_at = creation.created_at.as_deref().unwrap_or_default();
        self.db.connection().execute(
            "INSERT INTO decision_engine_candidate_creation (
                workspace_id, creation_id, intake_candidate_id, creation_request_id,
                recommendation_reference, package_seal_digest, decision_candidate_id,
                title, goal_statement, created_at, creation_state, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?10)
             ON CONFLICT(workspace_id, creation_id) DO UPDATE SET
                intake_candidate_id = excluded.intake_candidate_id,
                creation_request_id = excluded.creation_request_id,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                decision_candidate_id = excluded.decision_candidate_id,
                title = excluded.title,
                goal_statement = excluded.goal_statement,
                created_at = excluded.created_at,
                creation_state = excluded.creation_state,
                updated_at = excluded.updated_at",
            (
                &creation.workspace_id,
                &creation.creation_id,
                &creation.intake_candidate_id,
                &creation.creation_request_id,
                &creation.recommendation_reference,
                &creation.package_seal_digest,
                decision_candidate_id,
                title,
                goal_statement,
                created_at,
                &creation.creation_state,
            ),
        )?;
        Ok(())
    }

    pub fn list_candidate_creations(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineCandidateCreation>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, creation_id, intake_candidate_id, creation_request_id,
                    recommendation_reference, package_seal_digest, decision_candidate_id,
                    title, goal_statement, created_at, creation_state
             FROM decision_engine_candidate_creation
             WHERE workspace_id = ?1
             ORDER BY creation_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_candidate_creation)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn get_candidate_creation_for_intake(
        &self,
        workspace_id: &str,
        intake_candidate_id: &str,
    ) -> Result<Option<DecisionEngineCandidateCreation>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, creation_id, intake_candidate_id, creation_request_id,
                    recommendation_reference, package_seal_digest, decision_candidate_id,
                    title, goal_statement, created_at, creation_state
             FROM decision_engine_candidate_creation
             WHERE workspace_id = ?1 AND intake_candidate_id = ?2
             LIMIT 1",
        )?;
        let mut rows =
            stmt.query_map((workspace_id, intake_candidate_id), map_candidate_creation)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Persist an evaluated origin contract acknowledgment (never scores).
    pub fn upsert_evaluation_origin_contract(
        &self,
        contract: &DecisionCandidateEvaluationOriginContract,
    ) -> Result<()> {
        if !contract.is_evaluated() {
            return Ok(());
        }
        let evaluated_at = contract.evaluated_at.as_deref().unwrap_or_default();
        self.db.connection().execute(
            "INSERT INTO decision_candidate_evaluation_origin (
                workspace_id, evaluation_id, decision_candidate_id, origin, evaluation_state,
                recommendation_reference, package_seal_digest, intake_candidate_id,
                creation_request_id, evaluated_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)
             ON CONFLICT(workspace_id, evaluation_id) DO UPDATE SET
                decision_candidate_id = excluded.decision_candidate_id,
                origin = excluded.origin,
                evaluation_state = excluded.evaluation_state,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                intake_candidate_id = excluded.intake_candidate_id,
                creation_request_id = excluded.creation_request_id,
                evaluated_at = excluded.evaluated_at,
                updated_at = excluded.updated_at",
            (
                &contract.workspace_id,
                &contract.evaluation_id,
                &contract.decision_candidate_id,
                &contract.origin,
                &contract.evaluation_state,
                &contract.recommendation_reference,
                &contract.package_seal_digest,
                &contract.intake_candidate_id,
                &contract.creation_request_id,
                evaluated_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_evaluation_origin_contracts(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateEvaluationOriginContract>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, evaluation_id, decision_candidate_id, origin, evaluation_state,
                    recommendation_reference, package_seal_digest, intake_candidate_id,
                    creation_request_id, evaluated_at
             FROM decision_candidate_evaluation_origin
             WHERE workspace_id = ?1
             ORDER BY evaluation_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_evaluation_origin_contract)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Persist accepted/rejected scoring-path resolutions only.
    pub fn upsert_evaluation_resolution(
        &self,
        resolution: &DecisionCandidateEvaluationResolution,
    ) -> Result<()> {
        if !resolution.is_accepted_for_scoring() && !resolution.is_rejected_for_scoring() {
            return Ok(());
        }
        let resolved_at = resolution.resolved_at.as_deref().unwrap_or_default();
        self.db.connection().execute(
            "INSERT INTO decision_candidate_evaluation_resolution (
                workspace_id, resolution_id, decision_candidate_id, origin, resolution_state,
                recommendation_reference, package_seal_digest, intake_candidate_id,
                creation_request_id, resolution_reason, resolved_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)
             ON CONFLICT(workspace_id, resolution_id) DO UPDATE SET
                decision_candidate_id = excluded.decision_candidate_id,
                origin = excluded.origin,
                resolution_state = excluded.resolution_state,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                intake_candidate_id = excluded.intake_candidate_id,
                creation_request_id = excluded.creation_request_id,
                resolution_reason = excluded.resolution_reason,
                resolved_at = excluded.resolved_at,
                updated_at = excluded.updated_at",
            (
                &resolution.workspace_id,
                &resolution.resolution_id,
                &resolution.decision_candidate_id,
                &resolution.origin,
                &resolution.resolution_state,
                &resolution.recommendation_reference,
                &resolution.package_seal_digest,
                &resolution.intake_candidate_id,
                &resolution.creation_request_id,
                &resolution.resolution_reason,
                resolved_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_evaluation_resolutions(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateEvaluationResolution>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, resolution_id, decision_candidate_id, origin, resolution_state,
                    recommendation_reference, package_seal_digest, intake_candidate_id,
                    creation_request_id, resolution_reason, resolved_at
             FROM decision_candidate_evaluation_resolution
             WHERE workspace_id = ?1
             ORDER BY resolution_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_evaluation_resolution)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Persist a DE-owned DecisionScore result (historical identity).
    pub fn upsert_candidate_score(&self, score: &DecisionCandidateScore) -> Result<()> {
        let factors_json = serde_json::to_string(&score.scoring_factors).unwrap_or_else(|_| "[]".into());
        self.db.connection().execute(
            "INSERT INTO decision_candidate_score (
                workspace_id, score_id, decision_candidate_id, origin, resolution_id,
                score_total, attention_contribution, memory_contribution,
                personalization_contribution, goal_contribution, scoring_factors_json,
                scored_at, recommendation_reference, package_seal_digest,
                intake_candidate_id, creation_request_id, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?12)
             ON CONFLICT(workspace_id, score_id) DO UPDATE SET
                decision_candidate_id = excluded.decision_candidate_id,
                origin = excluded.origin,
                resolution_id = excluded.resolution_id,
                score_total = excluded.score_total,
                attention_contribution = excluded.attention_contribution,
                memory_contribution = excluded.memory_contribution,
                personalization_contribution = excluded.personalization_contribution,
                goal_contribution = excluded.goal_contribution,
                scoring_factors_json = excluded.scoring_factors_json,
                scored_at = excluded.scored_at,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                intake_candidate_id = excluded.intake_candidate_id,
                creation_request_id = excluded.creation_request_id,
                updated_at = excluded.updated_at",
            (
                &score.workspace_id,
                &score.score_id,
                &score.decision_candidate_id,
                &score.origin,
                &score.resolution_id,
                score.score.total,
                score.score.attention_contribution,
                score.score.memory_contribution,
                score.score.personalization_contribution,
                score.score.goal_contribution,
                factors_json,
                &score.scored_at,
                &score.recommendation_reference,
                &score.package_seal_digest,
                &score.intake_candidate_id,
                &score.creation_request_id,
            ),
        )?;
        Ok(())
    }

    pub fn list_candidate_scores(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateScore>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, score_id, decision_candidate_id, origin, resolution_id,
                    score_total, attention_contribution, memory_contribution,
                    personalization_contribution, goal_contribution, scoring_factors_json,
                    scored_at, recommendation_reference, package_seal_digest,
                    intake_candidate_id, creation_request_id
             FROM decision_candidate_score
             WHERE workspace_id = ?1
             ORDER BY score_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_candidate_score)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Persist selected/rejected progression decisions only.
    pub fn upsert_candidate_selection(
        &self,
        selection: &DecisionCandidateSelection,
    ) -> Result<()> {
        if !selection.is_selected() && !selection.is_rejected() {
            return Ok(());
        }
        let selected_at = selection.selected_at.as_deref().unwrap_or_default();
        self.db.connection().execute(
            "INSERT INTO decision_candidate_selection (
                workspace_id, selection_id, decision_candidate_id, origin, selection_state,
                ranking_id, ranking_position, score_id, recommendation_reference,
                package_seal_digest, intake_candidate_id, creation_request_id,
                selection_reason, selected_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?14)
             ON CONFLICT(workspace_id, selection_id) DO UPDATE SET
                decision_candidate_id = excluded.decision_candidate_id,
                origin = excluded.origin,
                selection_state = excluded.selection_state,
                ranking_id = excluded.ranking_id,
                ranking_position = excluded.ranking_position,
                score_id = excluded.score_id,
                recommendation_reference = excluded.recommendation_reference,
                package_seal_digest = excluded.package_seal_digest,
                intake_candidate_id = excluded.intake_candidate_id,
                creation_request_id = excluded.creation_request_id,
                selection_reason = excluded.selection_reason,
                selected_at = excluded.selected_at,
                updated_at = excluded.updated_at",
            (
                &selection.workspace_id,
                &selection.selection_id,
                &selection.decision_candidate_id,
                &selection.origin,
                &selection.selection_state,
                &selection.ranking_id,
                selection.ranking_position,
                &selection.score_id,
                &selection.recommendation_reference,
                &selection.package_seal_digest,
                &selection.intake_candidate_id,
                &selection.creation_request_id,
                &selection.selection_reason,
                selected_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_candidate_selections(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateSelection>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, selection_id, decision_candidate_id, origin, selection_state,
                    ranking_id, ranking_position, score_id, recommendation_reference,
                    package_seal_digest, intake_candidate_id, creation_request_id,
                    selection_reason, selected_at
             FROM decision_candidate_selection
             WHERE workspace_id = ?1
             ORDER BY selection_id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_candidate_selection)?;
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

fn map_candidate_creation(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<DecisionEngineCandidateCreation> {
    let creation_state: String = row.get(10)?;
    Ok(DecisionEngineCandidateCreation {
        workspace_id: row.get(0)?,
        creation_id: row.get(1)?,
        intake_candidate_id: row.get(2)?,
        creation_request_id: row.get(3)?,
        recommendation_reference: row.get(4)?,
        package_seal_digest: row.get(5)?,
        decision_candidate_id: Some(row.get(6)?),
        title: Some(row.get(7)?),
        goal_statement: Some(row.get(8)?),
        created_at: Some(row.get(9)?),
        creation_state: creation_state.clone(),
        evidence: vec!["decision_candidate_created".into()],
        creates_decision_candidate: creation_state
            == DecisionEngineCandidateCreation::STATE_CREATED,
        creates_decision_score: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: "Decision Engine candidate creation (persisted). Native DecisionCandidate \
               provenance only — no scoring, planner handoff, or Gateway grant."
            .into(),
        authority_effect: DecisionEngineCandidateCreation::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_evaluation_origin_contract(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<DecisionCandidateEvaluationOriginContract> {
    let origin: String = row.get(3)?;
    let evaluation_state: String = row.get(4)?;
    let recommendation_reference: Option<String> = row.get(5)?;
    let package_seal_digest: Option<String> = row.get(6)?;
    let intake_candidate_id: Option<String> = row.get(7)?;
    let creation_request_id: Option<String> = row.get(8)?;
    let evaluated_at: String = row.get(9)?;
    let recommendation_visible = origin
        == workspace_domain::DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
        && recommendation_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty());
    Ok(DecisionCandidateEvaluationOriginContract {
        workspace_id: row.get(0)?,
        evaluation_id: row.get(1)?,
        decision_candidate_id: row.get(2)?,
        origin: origin.clone(),
        evaluation_state: evaluation_state.clone(),
        lifecycle_valid: true,
        provenance_valid: true,
        origin_supported: DecisionCandidateEvaluationOriginContract::origin_supported(&origin),
        recommendation_visible,
        package_identity_traceable: origin
            == workspace_domain::DecisionCandidate::ORIGIN_NATIVE
            || (package_seal_digest.as_ref().is_some_and(|v| !v.is_empty())
                && recommendation_visible),
        intake_candidate_id,
        creation_request_id,
        package_seal_digest,
        recommendation_reference,
        evaluated_at: Some(evaluated_at),
        scoring_applied: false,
        ranking_applied: false,
        creates_decision_score: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        mutates_recommendation_engine: false,
        handoff_command: None,
        note: format!(
            "Decision Engine evaluation origin contract ({evaluation_state}; origin={origin}). \
             Persisted acknowledgment only — no scoring or ranking."
        ),
        authority_effect: DecisionCandidateEvaluationOriginContract::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_evaluation_resolution(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<DecisionCandidateEvaluationResolution> {
    let origin: String = row.get(3)?;
    let resolution_state: String = row.get(4)?;
    Ok(DecisionCandidateEvaluationResolution {
        workspace_id: row.get(0)?,
        resolution_id: row.get(1)?,
        decision_candidate_id: row.get(2)?,
        origin: origin.clone(),
        resolution_state: resolution_state.clone(),
        evaluation_complete: true,
        lifecycle_valid: true,
        provenance_valid: true,
        candidate_active: true,
        recommendation_reference: row.get(5)?,
        package_seal_digest: row.get(6)?,
        intake_candidate_id: row.get(7)?,
        creation_request_id: row.get(8)?,
        resolution_reason: row.get(9)?,
        resolved_at: Some(row.get(10)?),
        scoring_applied: false,
        ranking_applied: false,
        creates_decision_score: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        mutates_recommendation_engine: false,
        handoff_command: None,
        note: format!(
            "Decision Engine evaluation resolution ({resolution_state}; origin={origin}). \
             Persisted scoring-path admission only — no DecisionScore or ranking."
        ),
        authority_effect: DecisionCandidateEvaluationResolution::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_candidate_score(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionCandidateScore> {
    let origin: String = row.get(3)?;
    let factors_json: String = row.get(10)?;
    let scoring_factors: Vec<String> =
        serde_json::from_str(&factors_json).unwrap_or_default();
    let score = DecisionScore {
        total: row.get(5)?,
        attention_contribution: row.get(6)?,
        memory_contribution: row.get(7)?,
        personalization_contribution: row.get(8)?,
        goal_contribution: row.get(9)?,
        factors: scoring_factors.clone(),
    };
    Ok(DecisionCandidateScore {
        workspace_id: row.get(0)?,
        score_id: row.get(1)?,
        decision_candidate_id: row.get(2)?,
        origin: origin.clone(),
        resolution_id: row.get(4)?,
        score,
        scoring_factors,
        scored_at: row.get(11)?,
        recommendation_reference: row.get(12)?,
        package_seal_digest: row.get(13)?,
        intake_candidate_id: row.get(14)?,
        creation_request_id: row.get(15)?,
        ranking_applied: false,
        selects_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        mutates_recommendation_engine: false,
        handoff_command: None,
        note: format!(
            "Decision Engine DecisionScore for origin {origin}. \
             Persisted scoring result only — no ranking or selection."
        ),
        authority_effect: DecisionCandidateScore::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_candidate_selection(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<DecisionCandidateSelection> {
    let origin: String = row.get(3)?;
    let selection_state: String = row.get(4)?;
    let ranking_id: Option<String> = row.get(5)?;
    let ranking_position: Option<i64> = row.get(6)?;
    let score_id: Option<String> = row.get(7)?;
    Ok(DecisionCandidateSelection {
        workspace_id: row.get(0)?,
        selection_id: row.get(1)?,
        decision_candidate_id: row.get(2)?,
        origin: origin.clone(),
        selection_state: selection_state.clone(),
        ranking_id,
        ranking_position: ranking_position.map(|p| p as u32),
        score_id,
        has_ranking_entry: true,
        has_score: true,
        provenance_valid: true,
        lifecycle_valid: true,
        candidate_active: true,
        recommendation_reference: row.get(8)?,
        package_seal_digest: row.get(9)?,
        intake_candidate_id: row.get(10)?,
        creation_request_id: row.get(11)?,
        selection_reason: row.get(12)?,
        selected_at: Some(row.get(13)?),
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        mutates_recommendation_engine: false,
        mutates_candidate_outcome: false,
        handoff_command: None,
        note: format!(
            "Decision Engine candidate selection ({selection_state}; origin={origin}). \
             Persisted progression decision only — no execution or planner."
        ),
        authority_effect: DecisionCandidateSelection::AUTHORITY_EFFECT_NONE.into(),
    })
}
