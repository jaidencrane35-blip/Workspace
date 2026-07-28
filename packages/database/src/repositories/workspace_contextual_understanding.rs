use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ContextFrame, ContextualCompleteness, ContextualEvidenceReference, ContextualGap,
    ContextualUnderstandingConfidence, ContextualUnderstandingHistoryEntry,
    ContextualUnderstandingStatus, ContextualWorkspaceSnapshot, SituationalTheme,
};

/// Persistence guards for Programme III contextual workspace understanding.
/// Stores understanding artefacts / history — never executes, decides, or mutates reality.
pub struct WorkspaceContextualUnderstandingRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceContextualUnderstandingRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &ContextualWorkspaceSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "contextual understanding snapshot invalid: {e}"
            ))
        })?;
        let frame_json = serde_json::to_string(&snap.frame).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let themes_json = serde_json::to_string(&snap.themes).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let confidence_json = serde_json::to_string(&snap.confidence).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_contextual_understanding_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, theme_count, gap_count, source_revision_count,
                situation_summary, narrative,
                frame_json, source_revisions_json, themes_json, gaps_json,
                confidence_json, provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,'none',0,0)",
            rusqlite::params![
                &snap.understanding_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.themes.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.situation_summary,
                &snap.narrative,
                frame_json,
                source_revisions_json,
                themes_json,
                gaps_json,
                confidence_json,
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
    ) -> Result<Vec<ContextualWorkspaceSnapshot>> {
        let currents = self.list_by_status(workspace_id, ContextualUnderstandingStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_contextual_understanding_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.understanding_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = ContextualUnderstandingHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &ContextualUnderstandingHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "contextual understanding history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_contextual_understanding_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_contextual_understanding_history (
                id, workspace_id, understanding_id, status, created_at, superseded_at,
                completeness, theme_count, gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.understanding_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.theme_count as i64,
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
        status: ContextualUnderstandingStatus,
    ) -> Result<Vec<ContextualWorkspaceSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, situation_summary, narrative,
                    frame_json, source_revisions_json, themes_json, gaps_json,
                    confidence_json, provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_contextual_understanding_snapshots
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
                row.get::<_, String>(14)?,
                row.get::<_, String>(15)?,
                row.get::<_, i64>(16)?,
                row.get::<_, i64>(17)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<ContextualWorkspaceSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, situation_summary, narrative,
                    frame_json, source_revisions_json, themes_json, gaps_json,
                    confidence_json, provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_contextual_understanding_snapshots
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
                row.get::<_, String>(14)?,
                row.get::<_, String>(15)?,
                row.get::<_, i64>(16)?,
                row.get::<_, i64>(17)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ContextualUnderstandingHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT understanding_id, status, created_at, superseded_at,
                    completeness, theme_count, gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_contextual_understanding_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(ContextualUnderstandingHistoryEntry {
                understanding_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                theme_count: row.get::<_, i64>(5)? as usize,
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
            "SELECT COUNT(*) FROM workspace_contextual_understanding_history WHERE workspace_id = ?1",
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
    i64,
    i64,
);

fn map_snapshot(row: SnapshotRow) -> Result<ContextualWorkspaceSnapshot> {
    let (
        understanding_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        situation_summary,
        narrative,
        frame_json,
        source_revisions_json,
        themes_json,
        gaps_json,
        confidence_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = ContextualUnderstandingStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad contextual understanding status: {e}"))
    })?;
    let completeness = ContextualCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let frame: ContextFrame = serde_json::from_str(&frame_json).map_err(ser_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(ser_err)?;
    let themes: Vec<SituationalTheme> = serde_json::from_str(&themes_json).map_err(ser_err)?;
    let gaps: Vec<ContextualGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let confidence: ContextualUnderstandingConfidence =
        serde_json::from_str(&confidence_json).map_err(ser_err)?;
    let provenance_links: Vec<ContextualEvidenceReference> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(ContextualWorkspaceSnapshot {
        understanding_id,
        workspace_id,
        generated_at: created_at,
        status,
        superseded_at,
        frame,
        source_revisions,
        themes,
        gaps,
        confidence,
        completeness,
        provenance_links,
        situation_summary,
        narrative,
        limitations,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
