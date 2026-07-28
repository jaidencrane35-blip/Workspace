use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    EvidenceNavigationCompleteness, EvidenceNavigationHistoryEntry, EvidenceNavigationSession,
    EvidenceNavigationStatus, EvidencePath, NavigationGap, NavigationLineage, EvidenceNavigationDiagSummary,
    EvidenceNavigationEvidenceRef, WorkspaceEvidenceNavigationSnapshot,
};

/// Persistence for Programme IV workspace evidence navigation.
/// Existing evidence paths only — never interpretation or inferred edges.
pub struct WorkspaceEvidenceNavigationRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceEvidenceNavigationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceEvidenceNavigationSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence navigation invalid: {e}"))
        })?;
        let session_json = serde_json::to_string(&snap.session).map_err(ser_err)?;
        let paths_json = serde_json::to_string(&snap.paths).map_err(ser_err)?;
        let summary_json = serde_json::to_string(&snap.summary).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let lineage_json = serde_json::to_string(&snap.lineage).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_evidence_navigation_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, path_count, gap_count, source_revision_count,
                narrative_summary, narrative,
                session_json, paths_json, summary_json, gaps_json, lineage_json,
                provenance_links_json, source_revisions_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,'none',0,0)",
            rusqlite::params![
                &snap.navigation_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.paths.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.narrative_summary,
                &snap.narrative,
                session_json,
                paths_json,
                summary_json,
                gaps_json,
                lineage_json,
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
    ) -> Result<Vec<WorkspaceEvidenceNavigationSnapshot>> {
        let currents = self.list_by_status(workspace_id, EvidenceNavigationStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_evidence_navigation_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.navigation_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = EvidenceNavigationHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &EvidenceNavigationHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "evidence navigation history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_evidence_navigation_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_evidence_navigation_history (
                id, workspace_id, navigation_id, status, created_at, superseded_at,
                completeness, path_count, gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.navigation_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.path_count as i64,
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
        status: EvidenceNavigationStatus,
    ) -> Result<Vec<WorkspaceEvidenceNavigationSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    session_json, paths_json, summary_json, gaps_json, lineage_json,
                    provenance_links_json, source_revisions_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_evidence_navigation_snapshots
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
    ) -> Result<Vec<WorkspaceEvidenceNavigationSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative_summary, narrative,
                    session_json, paths_json, summary_json, gaps_json, lineage_json,
                    provenance_links_json, source_revisions_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_evidence_navigation_snapshots
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
    ) -> Result<Vec<EvidenceNavigationHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT navigation_id, status, created_at, superseded_at,
                    completeness, path_count, gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_evidence_navigation_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(EvidenceNavigationHistoryEntry {
                navigation_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                path_count: row.get::<_, i64>(5)? as usize,
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
            "SELECT COUNT(*) FROM workspace_evidence_navigation_history WHERE workspace_id = ?1",
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

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceEvidenceNavigationSnapshot> {
    let (
        navigation_id,
        workspace_id,
        status,
        generated_at,
        superseded_at,
        completeness,
        narrative_summary,
        narrative,
        session_json,
        paths_json,
        summary_json,
        gaps_json,
        lineage_json,
        provenance_links_json,
        source_revisions_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;

    let session: EvidenceNavigationSession = serde_json::from_str(&session_json).map_err(de_err)?;
    let paths: Vec<EvidencePath> = serde_json::from_str(&paths_json).map_err(de_err)?;
    let summary: EvidenceNavigationDiagSummary = serde_json::from_str(&summary_json).map_err(de_err)?;
    let gaps: Vec<NavigationGap> = serde_json::from_str(&gaps_json).map_err(de_err)?;
    let lineage: NavigationLineage = serde_json::from_str(&lineage_json).map_err(de_err)?;
    let provenance_links: Vec<EvidenceNavigationEvidenceRef> =
        serde_json::from_str(&provenance_links_json).map_err(de_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(de_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(de_err)?;

    let _ = (
        EvidenceNavigationCompleteness::parse(&completeness),
        EvidenceNavigationStatus::parse(&status),
    );

    Ok(WorkspaceEvidenceNavigationSnapshot {
        navigation_id,
        workspace_id,
        generated_at,
        status: EvidenceNavigationStatus::parse(&status).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence navigation status: {e}"))
        })?,
        superseded_at,
        session,
        paths,
        summary,
        gaps,
        lineage,
        completeness: EvidenceNavigationCompleteness::parse(&completeness).map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "evidence navigation completeness: {e}"
            ))
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
    crate::error::DatabaseError::Migration(format!("evidence navigation serialize: {e}"))
}

fn de_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("evidence navigation deserialize: {e}"))
}
