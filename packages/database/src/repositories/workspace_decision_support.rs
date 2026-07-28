use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    DecisionSupportContext, DecisionDependency, DecisionSupportAssessment, DecisionSupportCompleteness,
    DecisionSupportEvidenceRef, DecisionSupportFrame, DecisionSupportGap,
    DecisionSupportHistoryEntry, DecisionSupportStatus, EvidenceBundle, TradeoffSummary,
    WorkspaceDecisionSupportSnapshot,
};

/// Persistence for Programme III workspace decision support.
/// Derived decision support artefacts only — never decisions, recommendations, or upstream mutation.
pub struct WorkspaceDecisionSupportRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceDecisionSupportRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceDecisionSupportSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("decision support invalid: {e}"))
        })?;
        let frame_json = serde_json::to_string(&snap.frame).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let contexts_json = serde_json::to_string(&snap.contexts).map_err(ser_err)?;
        let evidence_bundles_json =
            serde_json::to_string(&snap.evidence_bundles).map_err(ser_err)?;
        let tradeoffs_json = serde_json::to_string(&snap.tradeoffs).map_err(ser_err)?;
        let dependencies_json = serde_json::to_string(&snap.dependencies).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let assessment_json = serde_json::to_string(&snap.assessment).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_decision_support_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, context_count, bundle_count, tradeoff_count,
                dependency_count, gap_count, source_revision_count, summary, narrative,
                frame_json, source_revisions_json, contexts_json, evidence_bundles_json,
                tradeoffs_json, dependencies_json, gaps_json, assessment_json,
                provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,'none',0,0)",
            rusqlite::params![
                &snap.support_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.contexts.len() as i64,
                snap.evidence_bundles.len() as i64,
                snap.tradeoffs.len() as i64,
                snap.dependencies.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.summary,
                &snap.narrative,
                frame_json,
                source_revisions_json,
                contexts_json,
                evidence_bundles_json,
                tradeoffs_json,
                dependencies_json,
                gaps_json,
                assessment_json,
                provenance_links_json,
                limitations_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<WorkspaceDecisionSupportSnapshot>> {
        let currents = self.list_by_status(workspace_id, DecisionSupportStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_decision_support_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.support_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = DecisionSupportHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &DecisionSupportHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "decision support history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_decision_support_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_decision_support_history (
                id, workspace_id, support_id, status, created_at, superseded_at,
                completeness, context_count, bundle_count, tradeoff_count,
                dependency_count, gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,1,0,'none',?14)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.support_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.context_count as i64,
                entry.bundle_count as i64,
                entry.tradeoff_count as i64,
                entry.dependency_count as i64,
                entry.gap_count as i64,
                entry.source_revision_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: DecisionSupportStatus,
    ) -> Result<Vec<WorkspaceDecisionSupportSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, contexts_json, evidence_bundles_json,
                    tradeoffs_json, dependencies_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_decision_support_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![workspace_id, status.as_str()], map_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<WorkspaceDecisionSupportSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, contexts_json, evidence_bundles_json,
                    tradeoffs_json, dependencies_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_decision_support_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], map_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<DecisionSupportHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT support_id, status, created_at, superseded_at,
                    completeness, context_count, bundle_count, tradeoff_count,
                    dependency_count, gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_decision_support_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(DecisionSupportHistoryEntry {
                support_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                context_count: row.get::<_, i64>(5)? as usize,
                bundle_count: row.get::<_, i64>(6)? as usize,
                tradeoff_count: row.get::<_, i64>(7)? as usize,
                dependency_count: row.get::<_, i64>(8)? as usize,
                gap_count: row.get::<_, i64>(9)? as usize,
                source_revision_count: row.get::<_, i64>(10)? as usize,
                terminal: row.get::<_, i64>(11)? != 0,
                actionable: row.get::<_, i64>(12)? != 0,
                authority_effect: row.get(13)?,
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
            "SELECT COUNT(*) FROM workspace_decision_support_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

type SnapshotRow = (
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
    String,
    String,
    String,
    String,
    String,
    i64,
    i64,
);

fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SnapshotRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
        row.get(12)?,
        row.get(13)?,
        row.get(14)?,
        row.get(15)?,
        row.get(16)?,
        row.get(17)?,
        row.get(18)?,
        row.get(19)?,
        row.get(20)?,
    ))
}

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceDecisionSupportSnapshot> {
    let (
        support_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        summary,
        narrative,
        frame_json,
        source_revisions_json,
        contexts_json,
        evidence_bundles_json,
        tradeoffs_json,
        dependencies_json,
        gaps_json,
        assessment_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = DecisionSupportStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad decision support status: {e}"))
    })?;
    let completeness = DecisionSupportCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let frame: DecisionSupportFrame = serde_json::from_str(&frame_json).map_err(ser_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(ser_err)?;
    let contexts: Vec<DecisionSupportContext> = serde_json::from_str(&contexts_json).map_err(ser_err)?;
    let evidence_bundles: Vec<EvidenceBundle> =
        serde_json::from_str(&evidence_bundles_json).map_err(ser_err)?;
    let tradeoffs: Vec<TradeoffSummary> = serde_json::from_str(&tradeoffs_json).map_err(ser_err)?;
    let dependencies: Vec<DecisionDependency> =
        serde_json::from_str(&dependencies_json).map_err(ser_err)?;
    let gaps: Vec<DecisionSupportGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let assessment: DecisionSupportAssessment =
        serde_json::from_str(&assessment_json).map_err(ser_err)?;
    let provenance_links: Vec<DecisionSupportEvidenceRef> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(WorkspaceDecisionSupportSnapshot {
        support_id,
        workspace_id,
        generated_at: created_at,
        status,
        superseded_at,
        frame,
        source_revisions,
        contexts,
        evidence_bundles,
        tradeoffs,
        dependencies,
        gaps,
        assessment,
        completeness,
        provenance_links,
        summary,
        narrative,
        limitations,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("decision support json: {err}"))
}
