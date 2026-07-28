use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    KnowledgeCluster, KnowledgeCompleteness, KnowledgeConcept, KnowledgeConfidence,
    KnowledgeEvidenceReference, KnowledgeGap, KnowledgeRelationship, KnowledgeSynthesisFrame,
    KnowledgeSynthesisHistoryEntry, KnowledgeSynthesisStatus, WorkspaceKnowledgeSynthesis,
};

/// Persistence guards for Programme III knowledge synthesis.
/// Stores derived knowledge artefacts / history — never Memory SoT, decisions, or mutations.
pub struct WorkspaceKnowledgeSynthesisRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceKnowledgeSynthesisRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &WorkspaceKnowledgeSynthesis) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "knowledge synthesis invalid: {e}"
            ))
        })?;
        let frame_json = serde_json::to_string(&snap.frame).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let concepts_json = serde_json::to_string(&snap.concepts).map_err(ser_err)?;
        let clusters_json = serde_json::to_string(&snap.clusters).map_err(ser_err)?;
        let relationships_json = serde_json::to_string(&snap.relationships).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let confidence_json = serde_json::to_string(&snap.confidence).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_knowledge_synthesis_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, concept_count, cluster_count, relationship_count,
                gap_count, source_revision_count, summary, narrative,
                frame_json, source_revisions_json, concepts_json, clusters_json,
                relationships_json, gaps_json, confidence_json,
                provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,'none',0,0)",
            rusqlite::params![
                &snap.synthesis_id,
                &snap.workspace_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.concepts.len() as i64,
                snap.clusters.len() as i64,
                snap.relationships.len() as i64,
                snap.gaps.len() as i64,
                snap.source_revisions.len() as i64,
                &snap.summary,
                &snap.narrative,
                frame_json,
                source_revisions_json,
                concepts_json,
                clusters_json,
                relationships_json,
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
    ) -> Result<Vec<WorkspaceKnowledgeSynthesis>> {
        let currents = self.list_by_status(workspace_id, KnowledgeSynthesisStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_knowledge_synthesis_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &snap.synthesis_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = KnowledgeSynthesisHistoryEntry::from_synthesis(&snap) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &KnowledgeSynthesisHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "knowledge synthesis history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_knowledge_synthesis_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_knowledge_synthesis_history (
                id, workspace_id, synthesis_id, status, created_at, superseded_at,
                completeness, concept_count, cluster_count, relationship_count,
                gap_count, source_revision_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,1,0,'none',?13)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.synthesis_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.concept_count as i64,
                entry.cluster_count as i64,
                entry.relationship_count as i64,
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
        status: KnowledgeSynthesisStatus,
    ) -> Result<Vec<WorkspaceKnowledgeSynthesis>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, concepts_json, clusters_json,
                    relationships_json, gaps_json, confidence_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_knowledge_synthesis_snapshots
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

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<WorkspaceKnowledgeSynthesis>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    frame_json, source_revisions_json, concepts_json, clusters_json,
                    relationships_json, gaps_json, confidence_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_knowledge_synthesis_snapshots
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
    ) -> Result<Vec<KnowledgeSynthesisHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT synthesis_id, status, created_at, superseded_at,
                    completeness, concept_count, cluster_count, relationship_count,
                    gap_count, source_revision_count,
                    terminal, actionable, authority_effect
             FROM workspace_knowledge_synthesis_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(KnowledgeSynthesisHistoryEntry {
                synthesis_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                concept_count: row.get::<_, i64>(5)? as usize,
                cluster_count: row.get::<_, i64>(6)? as usize,
                relationship_count: row.get::<_, i64>(7)? as usize,
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
            "SELECT COUNT(*) FROM workspace_knowledge_synthesis_history WHERE workspace_id = ?1",
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

fn map_snapshot(row: SnapshotRow) -> Result<WorkspaceKnowledgeSynthesis> {
    let (
        synthesis_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        summary,
        narrative,
        frame_json,
        source_revisions_json,
        concepts_json,
        clusters_json,
        relationships_json,
        gaps_json,
        confidence_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = KnowledgeSynthesisStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad knowledge synthesis status: {e}"))
    })?;
    let completeness = KnowledgeCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let frame: KnowledgeSynthesisFrame = serde_json::from_str(&frame_json).map_err(ser_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(ser_err)?;
    let concepts: Vec<KnowledgeConcept> = serde_json::from_str(&concepts_json).map_err(ser_err)?;
    let clusters: Vec<KnowledgeCluster> = serde_json::from_str(&clusters_json).map_err(ser_err)?;
    let relationships: Vec<KnowledgeRelationship> =
        serde_json::from_str(&relationships_json).map_err(ser_err)?;
    let gaps: Vec<KnowledgeGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let confidence: KnowledgeConfidence =
        serde_json::from_str(&confidence_json).map_err(ser_err)?;
    let provenance_links: Vec<KnowledgeEvidenceReference> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(WorkspaceKnowledgeSynthesis {
        synthesis_id,
        workspace_id,
        generated_at: created_at,
        status,
        superseded_at,
        frame,
        source_revisions,
        concepts,
        clusters,
        relationships,
        gaps,
        confidence,
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
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
