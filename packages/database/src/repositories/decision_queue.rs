use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{DecisionLifecycleOverlay, DecisionSourceType, DecisionState};

/// Persistence for Decision Queue lifecycle overlay only.
pub struct DecisionQueueRepository<'a> {
    db: &'a Database,
}

impl<'a> DecisionQueueRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_overlay(&self, overlay: &DecisionLifecycleOverlay) -> Result<()> {
        if !overlay.decision_state.is_overlay_state() {
            return Err(crate::error::DatabaseError::InvalidTransition(format!(
                "decision queue overlay cannot persist source-owned state {}",
                overlay.decision_state.as_str()
            )));
        }
        let changed = self.db.connection().execute(
            "INSERT INTO decision_item_lifecycle (
                workspace_id, source_type, source_id, decision_state, updated_at, actor_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(workspace_id, source_type, source_id) DO UPDATE SET
                decision_state = excluded.decision_state,
                updated_at = excluded.updated_at,
                actor_id = excluded.actor_id
             WHERE decision_item_lifecycle.decision_state = excluded.decision_state
                OR (decision_item_lifecycle.decision_state = 'pending'
                    AND excluded.decision_state IN ('viewed','deferred','dismissed'))
                OR (decision_item_lifecycle.decision_state = 'viewed'
                    AND excluded.decision_state IN ('deferred','dismissed'))
                OR (decision_item_lifecycle.decision_state = 'deferred'
                    AND excluded.decision_state IN ('viewed','dismissed'))",
            (
                &overlay.workspace_id,
                overlay.source_type.as_str(),
                &overlay.source_id,
                overlay.decision_state.as_str(),
                &overlay.updated_at,
                &overlay.actor_id,
            ),
        )?;
        if changed == 0 {
            return Err(crate::error::DatabaseError::InvalidTransition(format!(
                "decision item {} cannot transition to {}",
                overlay.source_id,
                overlay.decision_state.as_str()
            )));
        }
        Ok(())
    }

    pub fn get_overlay(
        &self,
        workspace_id: &str,
        source_type: DecisionSourceType,
        source_id: &str,
    ) -> Result<Option<DecisionLifecycleOverlay>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, source_type, source_id, decision_state, updated_at, actor_id
             FROM decision_item_lifecycle
             WHERE workspace_id = ?1 AND source_type = ?2 AND source_id = ?3",
        )?;
        let mut rows = stmt.query((workspace_id, source_type.as_str(), source_id))?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_overlay(row)?));
        }
        Ok(None)
    }

    pub fn list_overlays(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionLifecycleOverlay>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, source_type, source_id, decision_state, updated_at, actor_id
             FROM decision_item_lifecycle
             WHERE workspace_id = ?1",
        )?;
        let rows = stmt.query_map([workspace_id], map_overlay)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn delete_overlay(
        &self,
        workspace_id: &str,
        source_type: DecisionSourceType,
        source_id: &str,
    ) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM decision_item_lifecycle
             WHERE workspace_id = ?1 AND source_type = ?2 AND source_id = ?3",
            (workspace_id, source_type.as_str(), source_id),
        )?;
        Ok(())
    }

    /// Resolve workspace for a source key (lifecycle overlay lookup).
    pub fn list_overlays_by_source(
        &self,
        source_type: DecisionSourceType,
        source_id: &str,
    ) -> Result<Option<String>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id FROM decision_item_lifecycle
             WHERE source_type = ?1 AND source_id = ?2
             LIMIT 1",
        )?;
        let mut rows = stmt.query((source_type.as_str(), source_id))?;
        if let Some(row) = rows.next()? {
            return Ok(Some(row.get(0)?));
        }
        Ok(None)
    }
}

fn map_overlay(row: &rusqlite::Row<'_>) -> rusqlite::Result<DecisionLifecycleOverlay> {
    let source_type_raw: String = row.get(1)?;
    let state_raw: String = row.get(3)?;
    let source_type = DecisionSourceType::parse(&source_type_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(1, "source_type".into(), rusqlite::types::Type::Text)
    })?;
    let decision_state = DecisionState::parse(&state_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(3, "decision_state".into(), rusqlite::types::Type::Text)
    })?;
    Ok(DecisionLifecycleOverlay {
        workspace_id: row.get(0)?,
        source_type,
        source_id: row.get(2)?,
        decision_state,
        updated_at: row.get(4)?,
        actor_id: row.get(5)?,
    })
}
