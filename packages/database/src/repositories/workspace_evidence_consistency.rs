use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ConsistencyAssessment, ConsistencyConflict, ConsistencyDiagnostics, ConsistencyGap,
    ConsistencyLineage, ConsistencyObservation, ConsistencyScope, EvidenceConsistencyCompleteness,
    EvidenceConsistencyEvidenceRef, EvidenceConsistencyHistoryEntry, EvidenceConsistencyStatus,
    WorkspaceEvidenceConsistencySnapshot,
};

/// Persistence for Programme IV workspace evidence consistency.
/// Observe only — never resolve conflicts or fabricate agreement.
pub struct WorkspaceEvidenceConsistencyRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceEvidenceConsistencyRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceEvidenceConsistencySnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence consistency invalid: {e}"))
        })?;
        let scope_json = serde_json::to_string(&snap.scope).map_err(ser_err)?;
        let assessment_json = serde_json::to_string(&snap.assessment).map_err(ser_err)?;
        let observations_json = serde_json::to_string(&snap.observations).map_err(ser_err)?;
        let conflicts_json = serde_json::to_string(&snap.conflicts).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let lineage_json = serde_json::to_string(&snap.lineage).map_err(ser_err)?;
        let diagnostics_json = serde_json::to_string(&snap.diagnostics).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_evidence_consistency_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, observation_count, conflict_count, gap_count, source_revision_count,
                narrative_summary, narrative,
                scope_json, assessment_json, observations_json, conflicts_json, gaps_json,
                lineage_json, diagnostics_json, provenance_links_json, source_revisions_json,
                limitations_json, authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,'none',0,0)",
            rusqlite::params![
                &snap.consistency_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.observations.len() as i64,
                snap.conflicts.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.narrative_summary,
                &snap.narrative,
                scope_json,
                assessment_json,
                observations_json,
                conflicts_json,
                gaps_json,
                lineage_json,
                diagnostics_json,
                provenance_links_json,
                source_revisions_json,
                limitations_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<WorkspaceEvidenceConsistencySnapshot>> {
        let currents = self.list_by_status(workspace_id, EvidenceConsistencyStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_evidence_consistency_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.consistency_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = EvidenceConsistencyHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &EvidenceConsistencyHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "evidence consistency history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_evidence_consistency_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_evidence_consistency_history (
                id, workspace_id, consistency_id, status, created_at, superseded_at,
                completeness, observation_count, conflict_count, gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,1,0,'none',?12)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.consistency_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.observation_count as i64,
                entry.conflict_count as i64,
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
        status: EvidenceConsistencyStatus,
    ) -> Result<Vec<WorkspaceEvidenceConsistencySnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    scope_json, assessment_json, observations_json, conflicts_json, gaps_json,
                    lineage_json, diagnostics_json, provenance_links_json, source_revisions_json,
                    limitations_json, authority_effect, terminal, actionable
             FROM workspace_evidence_consistency_snapshots
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

    pub fn list_views(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceEvidenceConsistencySnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    scope_json, assessment_json, observations_json, conflicts_json, gaps_json,
                    lineage_json, diagnostics_json, provenance_links_json, source_revisions_json,
                    limitations_json, authority_effect, terminal, actionable
             FROM workspace_evidence_consistency_snapshots
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
    ) -> Result<Vec<EvidenceConsistencyHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT consistency_id, status, created_at, superseded_at,
                    completeness, observation_count, conflict_count, gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_evidence_consistency_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(EvidenceConsistencyHistoryEntry {
                consistency_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                observation_count: row.get::<_, i64>(5)? as usize,
                conflict_count: row.get::<_, i64>(6)? as usize,
                gap_count: row.get::<_, i64>(7)? as usize,
                source_revision_count: row.get::<_, i64>(8)? as usize,
                terminal: row.get::<_, i64>(9)? != 0,
                actionable: row.get::<_, i64>(10)? != 0,
                authority_effect: row.get(11)?,
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
            "SELECT COUNT(*) FROM workspace_evidence_consistency_history WHERE workspace_id = ?1",
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

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceEvidenceConsistencySnapshot> {
    let (
        consistency_id,
        workspace_id,
        status,
        generated_at,
        superseded_at,
        completeness,
        narrative_summary,
        narrative,
        scope_json,
        assessment_json,
        observations_json,
        conflicts_json,
        gaps_json,
        lineage_json,
        diagnostics_json,
        provenance_links_json,
        source_revisions_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;

    Ok(WorkspaceEvidenceConsistencySnapshot {
        consistency_id,
        workspace_id,
        generated_at,
        status: EvidenceConsistencyStatus::parse(&status).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence consistency status: {e}"))
        })?,
        superseded_at,
        scope: serde_json::from_str::<ConsistencyScope>(&scope_json).map_err(de_err)?,
        assessment: serde_json::from_str::<ConsistencyAssessment>(&assessment_json)
            .map_err(de_err)?,
        observations: serde_json::from_str::<Vec<ConsistencyObservation>>(&observations_json)
            .map_err(de_err)?,
        conflicts: serde_json::from_str::<Vec<ConsistencyConflict>>(&conflicts_json)
            .map_err(de_err)?,
        gaps: serde_json::from_str::<Vec<ConsistencyGap>>(&gaps_json).map_err(de_err)?,
        lineage: serde_json::from_str::<ConsistencyLineage>(&lineage_json).map_err(de_err)?,
        diagnostics: serde_json::from_str::<ConsistencyDiagnostics>(&diagnostics_json)
            .map_err(de_err)?,
        completeness: EvidenceConsistencyCompleteness::parse(&completeness).map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "evidence consistency completeness: {e}"
            ))
        })?,
        provenance_links: serde_json::from_str::<Vec<EvidenceConsistencyEvidenceRef>>(
            &provenance_links_json,
        )
        .map_err(de_err)?,
        source_revisions: serde_json::from_str::<Vec<String>>(&source_revisions_json)
            .map_err(de_err)?,
        narrative_summary,
        narrative,
        limitations: serde_json::from_str::<Vec<String>>(&limitations_json).map_err(de_err)?,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("evidence consistency serialize: {e}"))
}

fn de_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("evidence consistency deserialize: {e}"))
}
