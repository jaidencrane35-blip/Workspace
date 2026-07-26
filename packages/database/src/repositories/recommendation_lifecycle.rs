use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    RecommendationLifecycleOverlay, RecommendationLifecycleState, RecommendationOutcome,
    RecommendationResolutionType,
};

/// Persistence for Recommendation Engine lifecycle overlay only.
pub struct RecommendationLifecycleRepository<'a> {
    db: &'a Database,
}

impl<'a> RecommendationLifecycleRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_overlay(&self, overlay: &RecommendationLifecycleOverlay) -> Result<()> {
        let outcome_json = match &overlay.outcome {
            Some(outcome) => Some(serde_json::to_string(outcome).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation outcome serialize: {e}"
                ))
            })?),
            None => None,
        };
        let prior_outcomes_json = if overlay.prior_outcomes.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&overlay.prior_outcomes).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation prior outcomes serialize: {e}"
                ))
            })?)
        };
        self.db.connection().execute(
            "INSERT INTO recommendation_lifecycle (
                workspace_id, native_id, lifecycle_state, created_at, presented_at,
                resolved_at, resolution_type, actor_id, outcome_json, prior_outcomes_json,
                content_fingerprint, updated_at, authority_effect
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(workspace_id, native_id) DO UPDATE SET
                lifecycle_state = excluded.lifecycle_state,
                created_at = excluded.created_at,
                presented_at = excluded.presented_at,
                resolved_at = excluded.resolved_at,
                resolution_type = excluded.resolution_type,
                actor_id = excluded.actor_id,
                outcome_json = excluded.outcome_json,
                prior_outcomes_json = excluded.prior_outcomes_json,
                content_fingerprint = excluded.content_fingerprint,
                updated_at = excluded.updated_at,
                authority_effect = excluded.authority_effect",
            (
                &overlay.workspace_id,
                &overlay.native_id,
                overlay.lifecycle_state.as_str(),
                &overlay.created_at,
                &overlay.presented_at,
                &overlay.resolved_at,
                overlay.resolution_type.map(|r| r.as_str().to_string()),
                &overlay.actor_id,
                &outcome_json,
                &prior_outcomes_json,
                &overlay.content_fingerprint,
                &overlay.updated_at,
                &overlay.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn get_overlay(
        &self,
        workspace_id: &str,
        native_id: &str,
    ) -> Result<Option<RecommendationLifecycleOverlay>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, native_id, lifecycle_state, created_at, presented_at,
                    resolved_at, resolution_type, actor_id, outcome_json, prior_outcomes_json,
                    content_fingerprint, updated_at, authority_effect
             FROM recommendation_lifecycle
             WHERE workspace_id = ?1 AND native_id = ?2",
        )?;
        let mut rows = stmt.query_map((workspace_id, native_id), map_overlay)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn list_overlays(&self, workspace_id: &str) -> Result<Vec<RecommendationLifecycleOverlay>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, native_id, lifecycle_state, created_at, presented_at,
                    resolved_at, resolution_type, actor_id, outcome_json, prior_outcomes_json,
                    content_fingerprint, updated_at, authority_effect
             FROM recommendation_lifecycle
             WHERE workspace_id = ?1",
        )?;
        let rows = stmt.query_map([workspace_id], map_overlay)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_overlay(row: &rusqlite::Row<'_>) -> rusqlite::Result<RecommendationLifecycleOverlay> {
    let state_raw: String = row.get(2)?;
    let lifecycle_state = RecommendationLifecycleState::parse(&state_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "lifecycle_state".into(), rusqlite::types::Type::Text)
    })?;
    let resolution_raw: Option<String> = row.get(6)?;
    let resolution_type = match resolution_raw.as_deref() {
        Some(raw) => Some(RecommendationResolutionType::parse(raw).map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                6,
                "resolution_type".into(),
                rusqlite::types::Type::Text,
            )
        })?),
        None => None,
    };
    let outcome_json: Option<String> = row.get(8)?;
    let outcome = match outcome_json.as_deref() {
        Some(raw) => Some(serde_json::from_str::<RecommendationOutcome>(raw).map_err(|_| {
            rusqlite::Error::InvalidColumnType(8, "outcome_json".into(), rusqlite::types::Type::Text)
        })?),
        None => None,
    };
    let prior_json: Option<String> = row.get(9)?;
    let prior_outcomes = match prior_json.as_deref() {
        Some(raw) => serde_json::from_str::<Vec<RecommendationOutcome>>(raw).map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                9,
                "prior_outcomes_json".into(),
                rusqlite::types::Type::Text,
            )
        })?,
        None => Vec::new(),
    };
    Ok(RecommendationLifecycleOverlay {
        workspace_id: row.get(0)?,
        native_id: row.get(1)?,
        lifecycle_state,
        created_at: row.get(3)?,
        presented_at: row.get(4)?,
        resolved_at: row.get(5)?,
        resolution_type,
        actor_id: row.get(7)?,
        outcome,
        prior_outcomes,
        content_fingerprint: row.get(10)?,
        updated_at: row.get(11)?,
        authority_effect: row.get(12)?,
    })
}
