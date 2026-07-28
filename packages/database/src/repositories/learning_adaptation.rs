use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AdaptationCandidate, ConfidenceUpdate, LearningEvidenceLink, LearningHistoryEntry,
    LearningMeta, LearningObservation, LearningPattern, LearningSignal, LearningStatus,
    LearningView,
};

/// Persistence guards for Programme II learning & adaptation projections.
pub struct LearningAdaptationRepository<'a> {
    db: &'a Database,
}

impl<'a> LearningAdaptationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &LearningView) -> Result<()> {
        view.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("learning view invalid: {e}"))
        })?;

        let observations_json = serde_json::to_string(&view.observations).map_err(ser_err)?;
        let patterns_json = serde_json::to_string(&view.patterns).map_err(ser_err)?;
        let success_signals_json =
            serde_json::to_string(&view.success_signals).map_err(ser_err)?;
        let failure_signals_json =
            serde_json::to_string(&view.failure_signals).map_err(ser_err)?;
        let confidence_updates_json =
            serde_json::to_string(&view.confidence_updates).map_err(ser_err)?;
        let adaptation_candidates_json =
            serde_json::to_string(&view.adaptation_candidates).map_err(ser_err)?;
        let evidence_links_json = serde_json::to_string(&view.evidence_links).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_learning_snapshots (
                id, workspace_id, status, created_at, superseded_at, uncertainty,
                observation_count, pattern_count, adaptation_count,
                observations_json, patterns_json, success_signals_json, failure_signals_json,
                confidence_updates_json, adaptation_candidates_json, evidence_links_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,'none',0,0)",
            rusqlite::params![
                &view.meta.learning_id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.created_at,
                &view.meta.superseded_at,
                view.meta.uncertainty as i64,
                view.meta.observation_count as i64,
                view.meta.pattern_count as i64,
                view.meta.adaptation_count as i64,
                observations_json,
                patterns_json,
                success_signals_json,
                failure_signals_json,
                confidence_updates_json,
                adaptation_candidates_json,
                evidence_links_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<LearningMeta>> {
        let currents = self.list_meta_by_status(workspace_id, LearningStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_learning_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.learning_id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = LearningHistoryEntry::from_meta(&meta) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &LearningHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "learning history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("learning_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_learning_history (
                id, workspace_id, learning_id, status, created_at, superseded_at,
                uncertainty, observation_count, pattern_count, adaptation_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.learning_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                entry.uncertainty as i64,
                entry.observation_count as i64,
                entry.pattern_count as i64,
                entry.adaptation_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<LearningMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, uncertainty,
                    observation_count, pattern_count, adaptation_count,
                    authority_effect, terminal, actionable
             FROM workspace_learning_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_meta_by_status(
        &self,
        workspace_id: &str,
        status: LearningStatus,
    ) -> Result<Vec<LearningMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, uncertainty,
                    observation_count, pattern_count, adaptation_count,
                    authority_effect, terminal, actionable
             FROM workspace_learning_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_view(&self, meta: &LearningMeta) -> Result<LearningView> {
        let mut stmt = self.db.connection().prepare(
            "SELECT observations_json, patterns_json, success_signals_json,
                    failure_signals_json, confidence_updates_json,
                    adaptation_candidates_json, evidence_links_json, uncertainty
             FROM workspace_learning_snapshots
             WHERE id = ?1",
        )?;
        let row = stmt.query_row([&meta.learning_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, i64>(7)? as u8,
            ))
        })?;

        Ok(LearningView {
            meta: meta.clone(),
            observations: de_json(&row.0)?,
            patterns: de_json::<Vec<LearningPattern>>(&row.1)?,
            success_signals: de_json::<Vec<LearningSignal>>(&row.2)?,
            failure_signals: de_json::<Vec<LearningSignal>>(&row.3)?,
            confidence_updates: de_json::<Vec<ConfidenceUpdate>>(&row.4)?,
            adaptation_candidates: de_json::<Vec<AdaptationCandidate>>(&row.5)?,
            evidence_links: de_json::<Vec<LearningEvidenceLink>>(&row.6)?,
            uncertainty: row.7,
            authority_effect: LearningView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<LearningHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT learning_id, status, created_at, superseded_at, uncertainty,
                    observation_count, pattern_count, adaptation_count,
                    terminal, actionable, authority_effect
             FROM workspace_learning_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_learning_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("learning serialize: {e}"))
}

fn de_json<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T> {
    serde_json::from_str(raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("learning deserialize: {e}"))
    })
}

fn map_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<LearningMeta> {
    let status = LearningStatus::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(LearningMeta {
        learning_id: row.get(0)?,
        workspace_id: row.get(1)?,
        status,
        created_at: row.get(3)?,
        superseded_at: row.get(4)?,
        uncertainty: row.get::<_, i64>(5)? as u8,
        observation_count: row.get::<_, i64>(6)? as usize,
        pattern_count: row.get::<_, i64>(7)? as usize,
        adaptation_count: row.get::<_, i64>(8)? as usize,
        authority_effect: row.get(9)?,
        terminal: row.get::<_, i64>(10)? != 0,
        actionable: row.get::<_, i64>(11)? != 0,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<LearningHistoryEntry> {
    Ok(LearningHistoryEntry {
        learning_id: row.get(0)?,
        status: row.get(1)?,
        created_at: row.get(2)?,
        superseded_at: row.get(3)?,
        uncertainty: row.get::<_, i64>(4)? as u8,
        observation_count: row.get::<_, i64>(5)? as usize,
        pattern_count: row.get::<_, i64>(6)? as usize,
        adaptation_count: row.get::<_, i64>(7)? as usize,
        terminal: row.get::<_, i64>(8)? != 0,
        actionable: row.get::<_, i64>(9)? != 0,
        authority_effect: row.get(10)?,
    })
}
