use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AutonomyEvidenceLink, AutonomyOpportunity, AutonomyRecommendation, AutonomySafetyAssessment,
    AutonomyStatus, AutomationProposal, CognitiveAutonomyHistoryEntry, CognitiveAutonomyMeta,
    CognitiveAutonomyView,
};

/// Persistence guards for Programme II cognitive autonomy projections.
pub struct CognitiveAutonomyRepository<'a> {
    db: &'a Database,
}

impl<'a> CognitiveAutonomyRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &CognitiveAutonomyView) -> Result<()> {
        view.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("autonomy view invalid: {e}"))
        })?;

        let opportunities_json = serde_json::to_string(&view.opportunities).map_err(ser_err)?;
        let proposals_json = serde_json::to_string(&view.proposals).map_err(ser_err)?;
        let recommendations_json =
            serde_json::to_string(&view.recommendations).map_err(ser_err)?;
        let risk_assessments_json =
            serde_json::to_string(&view.risk_assessments).map_err(ser_err)?;
        let approval_requirements_json =
            serde_json::to_string(&view.approval_requirements).map_err(ser_err)?;
        let evidence_links_json = serde_json::to_string(&view.evidence_links).map_err(ser_err)?;
        let constraints_json = serde_json::to_string(&view.constraints).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_cognitive_autonomy_snapshots (
                id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                opportunity_count, proposal_count, recommendation_count,
                opportunities_json, proposals_json, recommendations_json, risk_assessments_json,
                approval_requirements_json, evidence_links_json, constraints_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,'none',0,0)",
            rusqlite::params![
                &view.meta.autonomy_id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.created_at,
                &view.meta.superseded_at,
                view.meta.confidence as i64,
                view.meta.uncertainty as i64,
                view.meta.opportunity_count as i64,
                view.meta.proposal_count as i64,
                view.meta.recommendation_count as i64,
                opportunities_json,
                proposals_json,
                recommendations_json,
                risk_assessments_json,
                approval_requirements_json,
                evidence_links_json,
                constraints_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<CognitiveAutonomyMeta>> {
        let currents = self.list_meta_by_status(workspace_id, AutonomyStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_cognitive_autonomy_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.autonomy_id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = CognitiveAutonomyHistoryEntry::from_meta(&meta) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &CognitiveAutonomyHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "autonomy history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("cognitive_autonomy_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_cognitive_autonomy_history (
                id, workspace_id, autonomy_id, status, created_at, superseded_at,
                confidence, uncertainty, opportunity_count, proposal_count, recommendation_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,1,0,'none',?12)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.autonomy_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                entry.confidence as i64,
                entry.uncertainty as i64,
                entry.opportunity_count as i64,
                entry.proposal_count as i64,
                entry.recommendation_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<CognitiveAutonomyMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                    opportunity_count, proposal_count, recommendation_count,
                    authority_effect, terminal, actionable
             FROM workspace_cognitive_autonomy_snapshots
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
        status: AutonomyStatus,
    ) -> Result<Vec<CognitiveAutonomyMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                    opportunity_count, proposal_count, recommendation_count,
                    authority_effect, terminal, actionable
             FROM workspace_cognitive_autonomy_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_view(&self, meta: &CognitiveAutonomyMeta) -> Result<CognitiveAutonomyView> {
        let mut stmt = self.db.connection().prepare(
            "SELECT opportunities_json, proposals_json, recommendations_json,
                    risk_assessments_json, approval_requirements_json, evidence_links_json,
                    constraints_json, confidence, uncertainty
             FROM workspace_cognitive_autonomy_snapshots
             WHERE id = ?1",
        )?;
        let row = stmt.query_row([&meta.autonomy_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, i64>(7)? as u8,
                row.get::<_, i64>(8)? as u8,
            ))
        })?;

        Ok(CognitiveAutonomyView {
            meta: meta.clone(),
            opportunities: de_json::<Vec<AutonomyOpportunity>>(&row.0)?,
            proposals: de_json::<Vec<AutomationProposal>>(&row.1)?,
            recommendations: de_json::<Vec<AutonomyRecommendation>>(&row.2)?,
            risk_assessments: de_json::<Vec<AutonomySafetyAssessment>>(&row.3)?,
            approval_requirements: de_json(&row.4)?,
            evidence_links: de_json::<Vec<AutonomyEvidenceLink>>(&row.5)?,
            constraints: de_json(&row.6)?,
            confidence: row.7,
            uncertainty: row.8,
            authority_effect: CognitiveAutonomyView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<CognitiveAutonomyHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT autonomy_id, status, created_at, superseded_at, confidence, uncertainty,
                    opportunity_count, proposal_count, recommendation_count,
                    terminal, actionable, authority_effect
             FROM workspace_cognitive_autonomy_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_cognitive_autonomy_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("autonomy serialize: {e}"))
}

fn de_json<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T> {
    serde_json::from_str(raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("autonomy deserialize: {e}"))
    })
}

fn map_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveAutonomyMeta> {
    let status = AutonomyStatus::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveAutonomyMeta {
        autonomy_id: row.get(0)?,
        workspace_id: row.get(1)?,
        status,
        created_at: row.get(3)?,
        superseded_at: row.get(4)?,
        confidence: row.get::<_, i64>(5)? as u8,
        uncertainty: row.get::<_, i64>(6)? as u8,
        opportunity_count: row.get::<_, i64>(7)? as usize,
        proposal_count: row.get::<_, i64>(8)? as usize,
        recommendation_count: row.get::<_, i64>(9)? as usize,
        authority_effect: row.get(10)?,
        terminal: row.get::<_, i64>(11)? != 0,
        actionable: row.get::<_, i64>(12)? != 0,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveAutonomyHistoryEntry> {
    Ok(CognitiveAutonomyHistoryEntry {
        autonomy_id: row.get(0)?,
        status: row.get(1)?,
        created_at: row.get(2)?,
        superseded_at: row.get(3)?,
        confidence: row.get::<_, i64>(4)? as u8,
        uncertainty: row.get::<_, i64>(5)? as u8,
        opportunity_count: row.get::<_, i64>(6)? as usize,
        proposal_count: row.get::<_, i64>(7)? as usize,
        recommendation_count: row.get::<_, i64>(8)? as usize,
        terminal: row.get::<_, i64>(9)? != 0,
        actionable: row.get::<_, i64>(10)? != 0,
        authority_effect: row.get(11)?,
    })
}
