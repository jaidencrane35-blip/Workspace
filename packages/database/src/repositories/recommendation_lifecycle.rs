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

/// Same-state terminal writes may update explicitly allowed lifecycle metadata, but must not
/// rewrite identity-bearing evidence (digests, resolution commitment, request identity).
fn reject_weakened_terminal_evidence(
    existing: &RecommendationLifecycleOverlay,
    incoming: &RecommendationLifecycleOverlay,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    // Resolution commitment — once set, identity is frozen (not merely non-null).
    if let Some(existing_resolution) = existing.resolution_type {
        match incoming.resolution_type {
            None => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot erase resolution_type evidence",
                    existing.native_id
                )));
            }
            Some(next) if next != existing_resolution => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot rewrite terminal resolution_type",
                    existing.native_id
                )));
            }
            _ => {}
        }
    }
    if let Some(existing_resolved_at) = &existing.resolved_at {
        match &incoming.resolved_at {
            None => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot erase resolved_at evidence",
                    existing.native_id
                )));
            }
            Some(next) if next != existing_resolved_at => {
                return Err(ImmutableArtifact(format!(
                    "recommendation {} cannot rewrite terminal resolved_at",
                    existing.native_id
                )));
            }
            _ => {}
        }
    }

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
                reject_confirmation_identity_rewrite(
                    &existing.native_id,
                    existing_confirmation,
                    next,
                )?;
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

    match (
        &existing.decision_intake_adapter_preparation,
        &incoming.decision_intake_adapter_preparation,
    ) {
        (Some(_), None) => {
            return Err(ImmutableArtifact(format!(
                "recommendation {} cannot erase adapter preparation evidence",
                existing.native_id
            )));
        }
        (Some(existing_prep), Some(next)) => {
            reject_adapter_preparation_identity_rewrite(&existing.native_id, existing_prep, next)?;
        }
        _ => {}
    }

    match (
        &existing.decision_handoff_request,
        &incoming.decision_handoff_request,
    ) {
        (Some(_), None) => {
            return Err(ImmutableArtifact(format!(
                "recommendation {} cannot erase handoff evidence",
                existing.native_id
            )));
        }
        (Some(existing_handoff), Some(next)) => {
            reject_handoff_identity_rewrite(&existing.native_id, existing_handoff, next)?;
        }
        _ => {}
    }

    match (
        &existing.decision_engine_acceptance,
        &incoming.decision_engine_acceptance,
    ) {
        (Some(_), None) => {
            return Err(ImmutableArtifact(format!(
                "recommendation {} cannot erase decision engine acceptance evidence",
                existing.native_id
            )));
        }
        (Some(existing_acceptance), Some(next)) => {
            reject_acceptance_identity_rewrite(&existing.native_id, existing_acceptance, next)?;
        }
        _ => {}
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

    Ok(())
}

fn reject_confirmation_identity_rewrite(
    native_id: &str,
    existing: &RecommendationDecisionConfirmation,
    incoming: &RecommendationDecisionConfirmation,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    let terminal = matches!(
        existing.confirmation_state.as_str(),
        RecommendationDecisionConfirmation::STATE_CONFIRMED
            | RecommendationDecisionConfirmation::STATE_DECLINED
    );
    if terminal && incoming != existing {
        return Err(ImmutableArtifact(format!(
            "recommendation {native_id} cannot replace terminal confirmation evidence"
        )));
    }

    // Identity-bearing confirmation fields remain frozen even during required→confirmed/declined.
    if incoming.recommendation_id != existing.recommendation_id
        || incoming.recommendation_owner != existing.recommendation_owner
        || incoming.confirmation_owner != existing.confirmation_owner
        || incoming.decision_owner != existing.decision_owner
        || incoming.execution_owner != existing.execution_owner
        || incoming.creates_decision_engine_object != existing.creates_decision_engine_object
        || incoming.creates_intent != existing.creates_intent
        || incoming.grants_execution_authority != existing.grants_execution_authority
        || incoming.handoff_performed != existing.handoff_performed
        || incoming.authority_effect != existing.authority_effect
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {native_id} cannot rewrite confirmation identity evidence"
        )));
    }
    Ok(())
}

fn reject_adapter_preparation_identity_rewrite(
    native_id: &str,
    existing: &RecommendationDecisionIntakeAdapterPreparation,
    incoming: &RecommendationDecisionIntakeAdapterPreparation,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    // Identity / digest fields immutable. Mutable allow-list for revoke/rebind:
    // preparation_state, revoked_at, seal_aligned, note, proceed_authorized,
    // adapter_invoked, mapping_performed, decision_engine_object_id, handoff_performed,
    // permission_effect, authority_effect, current_owner.
    if incoming.recommendation_id != existing.recommendation_id
        || incoming.workspace_id != existing.workspace_id
        || incoming.sealed_intake_package_digest != existing.sealed_intake_package_digest
        || incoming.contract_version != existing.contract_version
        || incoming.continuity_fingerprint_at_prep != existing.continuity_fingerprint_at_prep
        || incoming.confirmation_intent != existing.confirmation_intent
        || incoming.prepared_at != existing.prepared_at
        || incoming.declared_consumer_role != existing.declared_consumer_role
        || incoming.suggested_mapping_notes != existing.suggested_mapping_notes
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {native_id} cannot rewrite adapter preparation identity evidence"
        )));
    }
    Ok(())
}

fn reject_handoff_identity_rewrite(
    native_id: &str,
    existing: &RecommendationDecisionHandoffRequest,
    incoming: &RecommendationDecisionHandoffRequest,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    // Identity / digest fields immutable. Mutable allow-list for revoke/rebind:
    // request_state, handoff_requested, revoked_at, preparation_active, seal_aligned,
    // note, handoff_performed, adapter_invoked, decision_engine_object_id,
    // permission_effect, authority_effect, current_owner.
    if incoming.recommendation_id != existing.recommendation_id
        || incoming.workspace_id != existing.workspace_id
        || incoming.sealed_intake_package_digest != existing.sealed_intake_package_digest
        || incoming.contract_version != existing.contract_version
        || incoming.contract_family != existing.contract_family
        || incoming.confirmation_intent != existing.confirmation_intent
        || incoming.confirmed_at != existing.confirmed_at
        || incoming.continuity_fingerprint != existing.continuity_fingerprint
        || incoming.preparation_state_at_request != existing.preparation_state_at_request
        || incoming.preparation_prepared_at != existing.preparation_prepared_at
        || incoming.requested_at != existing.requested_at
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {native_id} cannot rewrite handoff identity evidence"
        )));
    }
    Ok(())
}

fn reject_acceptance_identity_rewrite(
    native_id: &str,
    existing: &RecommendationDecisionEngineAcceptance,
    incoming: &RecommendationDecisionEngineAcceptance,
) -> Result<()> {
    use crate::error::DatabaseError::ImmutableArtifact;

    // Identity / digest fields immutable. Mutable allow-list for accept/decline/revoke/rebind:
    // acceptance_state, ownership_state, ownership_transferred, current_owner,
    // declared_future_owner, handoff_request_state, handoff_requested, accepted_at,
    // declined_at, revoked_at, decision_engine_object_id, adapter_invoked,
    // handoff_performed, permission_effect, note, authority_effect.
    if incoming.recommendation_id != existing.recommendation_id
        || incoming.workspace_id != existing.workspace_id
        || incoming.sealed_intake_package_digest != existing.sealed_intake_package_digest
        || incoming.contract_version != existing.contract_version
        || incoming.contract_family != existing.contract_family
        || incoming.confirmation_intent != existing.confirmation_intent
    {
        return Err(ImmutableArtifact(format!(
            "recommendation {native_id} cannot rewrite decision engine acceptance identity evidence"
        )));
    }
    Ok(())
}
