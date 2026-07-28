use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    RetrievalDiagnostics, RetrievalGap, RetrievalLineage, SemanticQuery,
    SemanticQueryCompleteness, SemanticQueryEvidenceRef, SemanticQueryHistoryEntry,
    SemanticQueryResult, SemanticQueryStatus, WorkspaceSemanticQuerySnapshot,
};

/// Persistence for Programme IV workspace semantic query retrieval.
/// Retrieved evidence only — never synthesis, reasoning, or recommendations.
pub struct WorkspaceSemanticQueryRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceSemanticQueryRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceSemanticQuerySnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("semantic query invalid: {e}"))
        })?;
        let query_json = serde_json::to_string(&snap.query).map_err(ser_err)?;
        let result_json = serde_json::to_string(&snap.result).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let lineage_json = serde_json::to_string(&snap.lineage).map_err(ser_err)?;
        let diagnostics_json = serde_json::to_string(&snap.diagnostics).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_semantic_query_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, match_count, gap_count, source_revision_count,
                narrative_summary, narrative,
                query_json, result_json, gaps_json, lineage_json, diagnostics_json,
                provenance_links_json, source_revisions_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,'none',0,0)",
            rusqlite::params![
                &snap.query_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.result.match_count as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.narrative_summary,
                &snap.narrative,
                query_json,
                result_json,
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
    ) -> Result<Vec<WorkspaceSemanticQuerySnapshot>> {
        let currents = self.list_by_status(workspace_id, SemanticQueryStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_semantic_query_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.query_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = SemanticQueryHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &SemanticQueryHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "semantic query history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_semantic_query_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_semantic_query_history (
                id, workspace_id, query_id, status, created_at, superseded_at,
                completeness, match_count, gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.query_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.match_count as i64,
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
        status: SemanticQueryStatus,
    ) -> Result<Vec<WorkspaceSemanticQuerySnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    query_json, result_json, gaps_json, lineage_json, diagnostics_json,
                    provenance_links_json, source_revisions_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_semantic_query_snapshots
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

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<WorkspaceSemanticQuerySnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    query_json, result_json, gaps_json, lineage_json, diagnostics_json,
                    provenance_links_json, source_revisions_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_semantic_query_snapshots
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
    ) -> Result<Vec<SemanticQueryHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT query_id, status, created_at, superseded_at,
                    completeness, match_count, gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_semantic_query_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(SemanticQueryHistoryEntry {
                query_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                match_count: row.get::<_, i64>(5)? as usize,
                gap_count: row.get::<_, i64>(6)? as usize,
                source_revision_count: row.get::<_, i64>(7)? as usize,
                terminal: row.get::<_, i64>(8)? != 0,
                actionable: row.get::<_, i64>(9)? != 0,
                authority_effect: row.get(10)?,
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
            "SELECT COUNT(*) FROM workspace_semantic_query_history WHERE workspace_id = ?1",
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
    ))
}

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceSemanticQuerySnapshot> {
    let (
        query_id,
        workspace_id,
        status,
        generated_at,
        superseded_at,
        completeness,
        narrative_summary,
        narrative,
        query_json,
        result_json,
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

    let query: SemanticQuery = serde_json::from_str(&query_json).map_err(de_err)?;
    let result: SemanticQueryResult = serde_json::from_str(&result_json).map_err(de_err)?;
    let gaps: Vec<RetrievalGap> = serde_json::from_str(&gaps_json).map_err(de_err)?;
    let lineage: RetrievalLineage = serde_json::from_str(&lineage_json).map_err(de_err)?;
    let diagnostics: RetrievalDiagnostics =
        serde_json::from_str(&diagnostics_json).map_err(de_err)?;
    let provenance_links: Vec<SemanticQueryEvidenceRef> =
        serde_json::from_str(&provenance_links_json).map_err(de_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(de_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(de_err)?;

    let _ = (
        SemanticQueryCompleteness::parse(&completeness),
        SemanticQueryStatus::parse(&status),
    );

    Ok(WorkspaceSemanticQuerySnapshot {
        query_id,
        workspace_id,
        generated_at,
        status: SemanticQueryStatus::parse(&status).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("semantic query status: {e}"))
        })?,
        superseded_at,
        query,
        result,
        gaps,
        lineage,
        diagnostics,
        completeness: SemanticQueryCompleteness::parse(&completeness).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("semantic query completeness: {e}"))
        })?,
        provenance_links,
        source_revisions,
        narrative_summary,
        narrative,
        limitations,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("semantic query serialize: {e}"))
}

fn de_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("semantic query deserialize: {e}"))
}
