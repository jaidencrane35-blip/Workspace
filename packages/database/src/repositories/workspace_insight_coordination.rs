use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    CoordinationAssessment, InsightAttentionSignal, InsightCluster, InsightCoordinationCompleteness,
    InsightCoordinationFrame, InsightCoordinationHistoryEntry, InsightCoordinationSnapshot,
    InsightCoordinationStatus, InsightEvidenceRef, InsightGap, InsightIntersection,
};

/// Persistence for Programme III insight coordination.
/// Derived coordination artefacts only — never Memory SoT, decisions, or upstream mutation.
pub struct InsightCoordinationRepository<'a> {
    db: &'a Database,
}

impl<'a> InsightCoordinationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &InsightCoordinationSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("insight coordination invalid: {e}"))
        })?;
        let frame_json = serde_json::to_string(&snap.frame).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let clusters_json = serde_json::to_string(&snap.clusters).map_err(ser_err)?;
        let intersections_json = serde_json::to_string(&snap.intersections).map_err(ser_err)?;
        let attention_signals_json =
            serde_json::to_string(&snap.attention_signals).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let assessment_json = serde_json::to_string(&snap.assessment).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_insight_coordination_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, cluster_count, intersection_count, attention_signal_count,
                gap_count, source_revision_count, summary, narrative,
                frame_json, source_revisions_json, clusters_json, intersections_json,
                attention_signals_json, gaps_json, assessment_json,
                provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,'none',0,0)",
            rusqlite::params![
                &snap.coordination_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.clusters.len() as i64,
                snap.intersections.len() as i64,
                snap.attention_signals.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.summary,
                &snap.narrative,
                frame_json,
                source_revisions_json,
                clusters_json,
                intersections_json,
                attention_signals_json,
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
    ) -> Result<Vec<InsightCoordinationSnapshot>> {
        let currents = self.list_by_status(workspace_id, InsightCoordinationStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_insight_coordination_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.coordination_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = InsightCoordinationHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &InsightCoordinationHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "insight coordination history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_insight_coordination_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_insight_coordination_history (
                id, workspace_id, coordination_id, status, created_at, superseded_at,
                completeness, cluster_count, intersection_count, attention_signal_count,
                gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,1,0,'none',?13)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.coordination_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.cluster_count as i64,
                entry.intersection_count as i64,
                entry.attention_signal_count as i64,
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
        status: InsightCoordinationStatus,
    ) -> Result<Vec<InsightCoordinationSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, clusters_json, intersections_json,
                    attention_signals_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_insight_coordination_snapshots
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

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<InsightCoordinationSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, clusters_json, intersections_json,
                    attention_signals_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_insight_coordination_snapshots
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
    ) -> Result<Vec<InsightCoordinationHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT coordination_id, status, created_at, superseded_at,
                    completeness, cluster_count, intersection_count, attention_signal_count,
                    gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_insight_coordination_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(InsightCoordinationHistoryEntry {
                coordination_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                cluster_count: row.get::<_, i64>(5)? as usize,
                intersection_count: row.get::<_, i64>(6)? as usize,
                attention_signal_count: row.get::<_, i64>(7)? as usize,
                gap_count: row.get::<_, i64>(8)? as usize,
                source_revision_count: row.get::<_, i64>(9)? as usize,
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
            "SELECT COUNT(*) FROM workspace_insight_coordination_history WHERE workspace_id = ?1",
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
    ))
}

fn map_snapshot(row: SnapshotRow) -> Result<InsightCoordinationSnapshot> {
    let (
        coordination_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        summary,
        narrative,
        frame_json,
        source_revisions_json,
        clusters_json,
        intersections_json,
        attention_signals_json,
        gaps_json,
        assessment_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = InsightCoordinationStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad insight coordination status: {e}"))
    })?;
    let completeness = InsightCoordinationCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let frame: InsightCoordinationFrame = serde_json::from_str(&frame_json).map_err(ser_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(ser_err)?;
    let clusters: Vec<InsightCluster> = serde_json::from_str(&clusters_json).map_err(ser_err)?;
    let intersections: Vec<InsightIntersection> =
        serde_json::from_str(&intersections_json).map_err(ser_err)?;
    let attention_signals: Vec<InsightAttentionSignal> =
        serde_json::from_str(&attention_signals_json).map_err(ser_err)?;
    let gaps: Vec<InsightGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let assessment: CoordinationAssessment =
        serde_json::from_str(&assessment_json).map_err(ser_err)?;
    let provenance_links: Vec<InsightEvidenceRef> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(InsightCoordinationSnapshot {
        coordination_id,
        workspace_id,
        generated_at: created_at,
        status,
        superseded_at,
        frame,
        source_revisions,
        clusters,
        intersections,
        attention_signals,
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
    crate::error::DatabaseError::Migration(format!("insight coordination json: {err}"))
}
