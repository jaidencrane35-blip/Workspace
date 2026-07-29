use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AssistantRetrievalDiagnostics, AssistantRetrievalGap, AssistantRetrievalHistoryEntry,
    AssistantRetrievalItem, AssistantRetrievalLineage, AssistantRetrievalRequest,
    AssistantRetrievalStatus, AssistantSurfaceScope, WorkspaceAssistantRetrievalSnapshot,
};

/// Persistence for Programme IV workspace assistant retrieval packages.
/// Retrieval records are durable evidence only; they never carry command or ranking authority.
pub struct WorkspaceAssistantRetrievalRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceAssistantRetrievalRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceAssistantRetrievalSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("assistant retrieval invalid: {e}"))
        })?;
        let request_json = serde_json::to_string(&snap.request).map_err(ser_err)?;
        let items_json = serde_json::to_string(&snap.items).map_err(ser_err)?;
        let lineage_json = serde_json::to_string(&snap.lineage).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let diagnostics_json = serde_json::to_string(&snap.diagnostics).map_err(ser_err)?;
        let scope_json = serde_json::to_string(&snap.request.scope).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_assistant_retrieval_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                item_count, gap_count, narrative_summary, narrative,
                request_json, items_json, lineage_json, gaps_json, diagnostics_json,
                scope_json, limitations_json, authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,'none',0,0)",
            rusqlite::params![
                &snap.retrieval_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.items.len() as i64,
                snap.gaps.len() as i64,
                &snap.narrative_summary,
                &snap.narrative,
                request_json,
                items_json,
                lineage_json,
                gaps_json,
                diagnostics_json,
                scope_json,
                limitations_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<WorkspaceAssistantRetrievalSnapshot>> {
        let currents = self.list_by_status(workspace_id, AssistantRetrievalStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_assistant_retrieval_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.retrieval_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = AssistantRetrievalHistoryEntry::from_snapshot(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &AssistantRetrievalHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "assistant retrieval history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_assistant_retrieval_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_assistant_retrieval_history (
                id, workspace_id, retrieval_id, status, created_at, superseded_at,
                item_count, gap_count, terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,1,0,'none',?9)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.retrieval_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                entry.item_count as i64,
                entry.gap_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: AssistantRetrievalStatus,
    ) -> Result<Vec<WorkspaceAssistantRetrievalSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    narrative_summary, narrative, request_json, items_json, lineage_json,
                    gaps_json, diagnostics_json, scope_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_assistant_retrieval_snapshots
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
    ) -> Result<Vec<WorkspaceAssistantRetrievalSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    narrative_summary, narrative, request_json, items_json, lineage_json,
                    gaps_json, diagnostics_json, scope_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_assistant_retrieval_snapshots
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

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<AssistantRetrievalHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT retrieval_id, status, created_at, superseded_at,
                    item_count, gap_count, terminal, actionable, authority_effect
             FROM workspace_assistant_retrieval_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(AssistantRetrievalHistoryEntry {
                retrieval_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                item_count: row.get::<_, i64>(4)? as usize,
                gap_count: row.get::<_, i64>(5)? as usize,
                terminal: row.get::<_, i64>(6)? != 0,
                actionable: row.get::<_, i64>(7)? != 0,
                authority_effect: row.get(8)?,
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
            "SELECT COUNT(*) FROM workspace_assistant_retrieval_history WHERE workspace_id = ?1",
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
    ))
}

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceAssistantRetrievalSnapshot> {
    let (
        retrieval_id,
        workspace_id,
        status,
        generated_at,
        superseded_at,
        narrative_summary,
        narrative,
        request_json,
        items_json,
        lineage_json,
        gaps_json,
        diagnostics_json,
        scope_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;

    let mut request: AssistantRetrievalRequest =
        serde_json::from_str(&request_json).map_err(de_err)?;
    // Prefer denormalized scope_json when present for dual-channel parity.
    if let Ok(scope) = serde_json::from_str::<AssistantSurfaceScope>(&scope_json) {
        request.scope = scope;
    }

    Ok(WorkspaceAssistantRetrievalSnapshot {
        retrieval_id,
        workspace_id,
        generated_at,
        status: AssistantRetrievalStatus::parse(&status).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("assistant retrieval status: {e}"))
        })?,
        superseded_at,
        request,
        items: serde_json::from_str::<Vec<AssistantRetrievalItem>>(&items_json).map_err(de_err)?,
        lineage: serde_json::from_str::<AssistantRetrievalLineage>(&lineage_json).map_err(de_err)?,
        gaps: serde_json::from_str::<Vec<AssistantRetrievalGap>>(&gaps_json).map_err(de_err)?,
        diagnostics: serde_json::from_str::<AssistantRetrievalDiagnostics>(&diagnostics_json)
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
    crate::error::DatabaseError::Migration(format!("assistant retrieval serialize: {e}"))
}

fn de_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("assistant retrieval deserialize: {e}"))
}
