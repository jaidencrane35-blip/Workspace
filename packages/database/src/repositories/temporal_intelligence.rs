use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    EvidenceGap, EvidenceQualityAssessment, RevisionChainSummary, TemporalAnalysisStatus,
    TemporalAnalysisView, TemporalAnalysisWindow, TemporalConflictExplanation,
    TemporalIntelligenceHistoryEntry, TemporalProvenanceLink, UnderstandingCompleteness,
};

/// Persistence guards for Programme III temporal intelligence evidence.
/// Stores analysis artefacts / history — never simulates, forecasts, repairs, or replays.
pub struct TemporalIntelligenceRepository<'a> {
    db: &'a Database,
}

impl<'a> TemporalIntelligenceRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &TemporalAnalysisView) -> Result<()> {
        view.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "temporal analysis view invalid: {e}"
            ))
        })?;
        let window_json = serde_json::to_string(&view.window).map_err(ser_err)?;
        let chain_summary_json = serde_json::to_string(&view.chain_summary).map_err(ser_err)?;
        let conflict_explanations_json =
            serde_json::to_string(&view.conflict_explanations).map_err(ser_err)?;
        let evidence_quality_json =
            serde_json::to_string(&view.evidence_quality).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&view.gaps).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&view.provenance_links).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_temporal_intelligence_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                from_revision, to_revision, completeness,
                conflict_count, gap_count, chain_ref_count, narrative,
                window_json, chain_summary_json, conflict_explanations_json,
                evidence_quality_json, gaps_json, provenance_links_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,'none',0,0)",
            rusqlite::params![
                &view.analysis_id,
                &view.workspace_id,
                view.status.as_str(),
                &view.generated_at,
                &view.superseded_at,
                &view.window.from_revision,
                &view.window.to_revision,
                view.completeness.as_str(),
                view.conflict_explanations.len() as i64,
                view.gaps.len() as i64,
                view.chain_summary.ordered_refs.len() as i64,
                &view.narrative,
                window_json,
                chain_summary_json,
                conflict_explanations_json,
                evidence_quality_json,
                gaps_json,
                provenance_links_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<TemporalAnalysisView>> {
        let currents = self.list_by_status(workspace_id, TemporalAnalysisStatus::Current)?;
        let mut out = Vec::new();
        for mut view in currents {
            view.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_temporal_intelligence_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &view.analysis_id,
                    view.status.as_str(),
                    &view.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = TemporalIntelligenceHistoryEntry::from_view(&view) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(view);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &TemporalIntelligenceHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "temporal intelligence history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("temporal_intelligence_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_temporal_intelligence_history (
                id, workspace_id, analysis_id, status, created_at, superseded_at,
                from_revision, to_revision, completeness,
                conflict_count, gap_count, chain_ref_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,1,0,'none',?13)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.analysis_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.from_revision,
                &entry.to_revision,
                &entry.completeness,
                entry.conflict_count as i64,
                entry.gap_count as i64,
                entry.chain_ref_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: TemporalAnalysisStatus,
    ) -> Result<Vec<TemporalAnalysisView>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative,
                    window_json, chain_summary_json, conflict_explanations_json,
                    evidence_quality_json, gaps_json, provenance_links_json,
                    authority_effect, terminal, actionable
             FROM workspace_temporal_intelligence_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![workspace_id, status.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, i64>(14)?,
                row.get::<_, i64>(15)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_view(row?)?);
        }
        Ok(out)
    }

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<TemporalAnalysisView>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative,
                    window_json, chain_summary_json, conflict_explanations_json,
                    evidence_quality_json, gaps_json, provenance_links_json,
                    authority_effect, terminal, actionable
             FROM workspace_temporal_intelligence_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, i64>(14)?,
                row.get::<_, i64>(15)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_view(row?)?);
        }
        Ok(out)
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<TemporalIntelligenceHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT analysis_id, status, created_at, superseded_at,
                    from_revision, to_revision, completeness,
                    conflict_count, gap_count, chain_ref_count,
                    terminal, actionable, authority_effect
             FROM workspace_temporal_intelligence_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(TemporalIntelligenceHistoryEntry {
                analysis_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                from_revision: row.get(4)?,
                to_revision: row.get(5)?,
                completeness: row.get(6)?,
                conflict_count: row.get::<_, i64>(7)? as usize,
                gap_count: row.get::<_, i64>(8)? as usize,
                chain_ref_count: row.get::<_, i64>(9)? as usize,
                terminal: row.get::<_, i64>(10)? != 0,
                actionable: row.get::<_, i64>(11)? != 0,
                authority_effect: row.get(12)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_temporal_intelligence_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

type ViewRow = (
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    i64,
);

fn map_view(row: ViewRow) -> Result<TemporalAnalysisView> {
    let (
        analysis_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        narrative,
        window_json,
        chain_summary_json,
        conflict_explanations_json,
        evidence_quality_json,
        gaps_json,
        provenance_links_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = TemporalAnalysisStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad temporal analysis status: {e}"))
    })?;
    let completeness = UnderstandingCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let window: TemporalAnalysisWindow = serde_json::from_str(&window_json).map_err(ser_err)?;
    let chain_summary: RevisionChainSummary =
        serde_json::from_str(&chain_summary_json).map_err(ser_err)?;
    let conflict_explanations: Vec<TemporalConflictExplanation> =
        serde_json::from_str(&conflict_explanations_json).map_err(ser_err)?;
    let evidence_quality: EvidenceQualityAssessment =
        serde_json::from_str(&evidence_quality_json).map_err(ser_err)?;
    let gaps: Vec<EvidenceGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let provenance_links: Vec<TemporalProvenanceLink> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    Ok(TemporalAnalysisView {
        analysis_id,
        workspace_id,
        window,
        generated_at: created_at,
        status,
        superseded_at,
        chain_summary,
        conflict_explanations,
        evidence_quality,
        completeness,
        gaps,
        provenance_links,
        narrative,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
