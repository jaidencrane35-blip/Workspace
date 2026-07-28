use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    RecommendationDecisionConfirmation, RecommendationDecisionEngineAcceptance,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakePackageSeal, RecommendationLifecycleOverlay,
    RecommendationLifecycleState, RecommendationOutcome, RecommendationResolutionType,
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
        if let Some(existing) =
            self.get_overlay(&overlay.workspace_id, &overlay.native_id)?
        {
            if existing.lifecycle_state.is_terminal()
                && existing.lifecycle_state == overlay.lifecycle_state
            {
                reject_weakened_terminal_evidence(&existing, overlay)?;
            }
        }

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
        let confirmation_json = match &overlay.decision_confirmation {
            Some(confirmation) => Some(serde_json::to_string(confirmation).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation confirmation serialize: {e}"
                ))
            })?),
            None => None,
        };
        let intake_seal_json = match &overlay.decision_intake_package_seal {
            Some(seal) => Some(serde_json::to_string(seal).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation intake seal serialize: {e}"
                ))
            })?),
            None => None,
        };
        let adapter_preparation_json = match &overlay.decision_intake_adapter_preparation {
            Some(prep) => Some(serde_json::to_string(prep).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation adapter preparation serialize: {e}"
                ))
            })?),
            None => None,
        };
        let handoff_request_json = match &overlay.decision_handoff_request {
            Some(request) => Some(serde_json::to_string(request).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation handoff request serialize: {e}"
                ))
            })?),
            None => None,
        };
        let decision_engine_acceptance_json = match &overlay.decision_engine_acceptance {
            Some(acceptance) => Some(serde_json::to_string(acceptance).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "recommendation decision engine acceptance serialize: {e}"
                ))
            })?),
            None => None,
        };
        let changed = self.db.connection().execute(
            "INSERT INTO recommendation_lifecycle (
                workspace_id, native_id, lifecycle_state, created_at, presented_at,
                resolved_at, resolution_type, actor_id, outcome_json, prior_outcomes_json,
                content_fingerprint, confirmation_json, intake_seal_json,
                adapter_preparation_json, handoff_request_json, decision_engine_acceptance_json,
                updated_at, authority_effect
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
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
                confirmation_json = excluded.confirmation_json,
                intake_seal_json = excluded.intake_seal_json,
                adapter_preparation_json = excluded.adapter_preparation_json,
                handoff_request_json = excluded.handoff_request_json,
                decision_engine_acceptance_json = excluded.decision_engine_acceptance_json,
                updated_at = excluded.updated_at,
                authority_effect = excluded.authority_effect
             WHERE recommendation_lifecycle.lifecycle_state = excluded.lifecycle_state
                OR (recommendation_lifecycle.lifecycle_state = 'created'
                    AND excluded.lifecycle_state IN (
                        'available','presented','accepted','rejected','expired','superseded'
                    ))
                OR (recommendation_lifecycle.lifecycle_state = 'available'
                    AND excluded.lifecycle_state IN (
                        'presented','accepted','rejected','expired','superseded'
                    ))
                OR (recommendation_lifecycle.lifecycle_state = 'presented'
                    AND excluded.lifecycle_state IN ('available','accepted','rejected','expired','superseded'))
                OR (
                    recommendation_lifecycle.lifecycle_state IN ('accepted','rejected','expired','superseded')
                    AND excluded.lifecycle_state = 'available'
                    AND excluded.content_fingerprint IS NOT NULL
                    AND excluded.content_fingerprint != COALESCE(recommendation_lifecycle.content_fingerprint, '')
                    AND excluded.prior_outcomes_json IS NOT NULL
                )",
            rusqlite::params![
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
                &confirmation_json,
                &intake_seal_json,
                &adapter_preparation_json,
                &handoff_request_json,
                &decision_engine_acceptance_json,
                &overlay.updated_at,
                &overlay.authority_effect,
            ],
        )?;
        if changed == 0 {
            return Err(crate::error::DatabaseError::InvalidTransition(format!(
                "recommendation {} cannot transition to {}",
                overlay.native_id,
                overlay.lifecycle_state.as_str()
            )));
        }
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
                    content_fingerprint, confirmation_json, intake_seal_json,
                    adapter_preparation_json, handoff_request_json, decision_engine_acceptance_json,
                    updated_at, authority_effect
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
                    content_fingerprint, confirmation_json, intake_seal_json,
                    adapter_preparation_json, handoff_request_json, decision_engine_acceptance_json,
                    updated_at, authority_effect
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
    let confirmation_json: Option<String> = row.get(11)?;
    let decision_confirmation = match confirmation_json.as_deref() {
        Some(raw) => Some(
            serde_json::from_str::<RecommendationDecisionConfirmation>(raw).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    11,
                    "confirmation_json".into(),
                    rusqlite::types::Type::Text,
                )
            })?,
        ),
        None => None,
    };
    let intake_seal_json: Option<String> = row.get(12)?;
    let decision_intake_package_seal = match intake_seal_json.as_deref() {
        Some(raw) => Some(
            serde_json::from_str::<RecommendationDecisionIntakePackageSeal>(raw).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    12,
                    "intake_seal_json".into(),
                    rusqlite::types::Type::Text,
                )
            })?,
        ),
        None => None,
    };
    let adapter_preparation_json: Option<String> = row.get(13)?;
    let decision_intake_adapter_preparation = match adapter_preparation_json.as_deref() {
        Some(raw) => Some(
            serde_json::from_str::<RecommendationDecisionIntakeAdapterPreparation>(raw).map_err(
                |_| {
                    rusqlite::Error::InvalidColumnType(
                        13,
                        "adapter_preparation_json".into(),
                        rusqlite::types::Type::Text,
                    )
                },
            )?,
        ),
        None => None,
    };
    let handoff_request_json: Option<String> = row.get(14)?;
    let decision_handoff_request = match handoff_request_json.as_deref() {
        Some(raw) => Some(
            serde_json::from_str::<RecommendationDecisionHandoffRequest>(raw).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    14,
                    "handoff_request_json".into(),
                    rusqlite::types::Type::Text,
                )
            })?,
        ),
        None => None,
    };
    let decision_engine_acceptance_json: Option<String> = row.get(15)?;
    let decision_engine_acceptance = match decision_engine_acceptance_json.as_deref() {
        Some(raw) => Some(
            serde_json::from_str::<RecommendationDecisionEngineAcceptance>(raw).map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    15,
                    "decision_engine_acceptance_json".into(),
                    rusqlite::types::Type::Text,
                )
            })?,
        ),
        None => None,
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
        decision_confirmation,
        decision_intake_package_seal,
        decision_intake_adapter_preparation,
        decision_handoff_request,
        decision_engine_acceptance,
        updated_at: row.get(16)?,
        authority_effect: row.get(17)?,
    })
}

/// Same-state terminal writes may update metadata and accumulate evidence, but must not
/// erase or replace authoritative terminal artifacts.
fn reject_weakened_terminal_evidence(
    existing: &RecommendationLifecycleOverlay,
    incoming: &RecommendationLifecycleOverlay,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    if let Some(existing_outcome) = &existing.outcome {
        match &incoming.outcome {
            None => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot erase terminal outcome evidence",
                    existing.native_id
                )));
            }
            Some(next) if next != existing_outcome => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot replace terminal outcome evidence",
                    existing.native_id
                )));
            }
            _ => {}
        }
    }

    if !existing.prior_outcomes.is_empty()
        && incoming.prior_outcomes != existing.prior_outcomes
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot rewrite prior outcome history",
            existing.native_id
        )));
    }

    if let Some(existing_confirmation) = &existing.decision_confirmation {
        match &incoming.decision_confirmation {
            None => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot erase confirmation evidence",
                    existing.native_id
                )));
            }
            Some(next) => {
                let terminal = matches!(
                    existing_confirmation.confirmation_state.as_str(),
                    RecommendationDecisionConfirmation::STATE_CONFIRMED
                        | RecommendationDecisionConfirmation::STATE_DECLINED
                );
                if terminal && next != existing_confirmation {
                    return Err(ImmutableArtifact(format!(
                        "recommendation {} cannot replace terminal confirmation evidence",
                        existing.native_id
                    )));
                }
            }
        }
    }

    if let Some(existing_seal) = &existing.decision_intake_package_seal {
        if existing_seal.sealed {
            match &incoming.decision_intake_package_seal {
                None => {
                    return Err(ImmutableArtifact(format!(
                        "recommendation {} cannot erase sealed intake evidence",
                        existing.native_id
                    )));
                }
                Some(next) if next != existing_seal => {
                    return Err(ImmutableArtifact(format!(
                        "recommendation {} cannot replace sealed intake evidence",
                        existing.native_id
                    )));
                }
                _ => {}
            }
        }
    }

    if existing.decision_intake_adapter_preparation.is_some()
        && incoming.decision_intake_adapter_preparation.is_none()
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot erase adapter preparation evidence",
            existing.native_id
        )));
    }

    if existing.decision_handoff_request.is_some() && incoming.decision_handoff_request.is_none()
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot erase handoff evidence",
            existing.native_id
        )));
    }

    if existing.decision_engine_acceptance.is_some()
        && incoming.decision_engine_acceptance.is_none()
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot erase decision engine acceptance evidence",
            existing.native_id
        )));
    }

    if let Some(existing_fp) = &existing.content_fingerprint {
        match &incoming.content_fingerprint {
            None => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot erase content fingerprint evidence",
                    existing.native_id
                )));
            }
            Some(next) if next != existing_fp => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot replace content fingerprint without generation reopen",
                    existing.native_id
                )));
            }
            _ => {}
        }
    }

    if existing.resolved_at.is_some() && incoming.resolved_at.is_none() {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot erase resolved_at evidence",
            existing.native_id
        )));
    }
    if existing.resolution_type.is_some() && incoming.resolution_type.is_none() {
        return Err(ImmutableArtifact(format!(
            "recommendation {} cannot erase resolution_type evidence",
            existing.native_id
        )));
    }

    Ok(())
}
