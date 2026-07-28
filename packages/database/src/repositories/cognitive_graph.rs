use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    CognitiveGraphEdge, CognitiveGraphEdgeKind, CognitiveGraphHistoryEntry, CognitiveGraphMeta,
    CognitiveGraphNode, CognitiveGraphNodeKind, CognitiveGraphStatus, CognitiveGraphView,
};

/// Persistence guards for Programme II cognitive graph projections.
pub struct CognitiveGraphRepository<'a> {
    db: &'a Database,
}

impl<'a> CognitiveGraphRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &CognitiveGraphView) -> Result<()> {
        view.meta.validate_authority()?;
        for node in &view.nodes {
            node.validate().map_err(|e| {
                crate::error::DatabaseError::Migration(format!("graph node invalid: {e}"))
            })?;
        }
        for edge in &view.edges {
            edge.validate().map_err(|e| {
                crate::error::DatabaseError::Migration(format!("graph edge invalid: {e}"))
            })?;
        }

        self.db.connection().execute(
            "INSERT INTO cognitive_graph_snapshots (
                id, workspace_id, status, generated_at, superseded_at,
                node_count, edge_count, broken_node_count, broken_edge_count, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![
                &view.meta.id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.generated_at,
                &view.meta.superseded_at,
                view.meta.node_count as i64,
                view.meta.edge_count as i64,
                view.meta.broken_node_count as i64,
                view.meta.broken_edge_count as i64,
                &view.meta.authority_effect,
            ],
        )?;

        for node in &view.nodes {
            self.db.connection().execute(
                "INSERT INTO cognitive_graph_nodes (
                    snapshot_id, external_ref, workspace_id, kind, title, broken, authority_effect
                 ) VALUES (?1,?2,?3,?4,?5,?6,?7)",
                (
                    &view.meta.id,
                    &node.external_ref,
                    &node.workspace_id,
                    node.kind.as_str(),
                    &node.title,
                    i64::from(node.broken),
                    &node.authority_effect,
                ),
            )?;
        }

        for edge in &view.edges {
            let evidence_refs_json = serde_json::to_string(&edge.evidence_refs).map_err(|e| {
                crate::error::DatabaseError::Migration(format!("edge evidence serialize: {e}"))
            })?;
            self.db.connection().execute(
                "INSERT INTO cognitive_graph_edges (
                    id, snapshot_id, workspace_id, from_ref, to_ref, kind, explanation,
                    confidence, evidence_refs_json, broken, authority_effect
                 ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                rusqlite::params![
                    &edge.id,
                    &view.meta.id,
                    &edge.workspace_id,
                    &edge.from_ref,
                    &edge.to_ref,
                    edge.kind.as_str(),
                    &edge.explanation,
                    edge.confidence as i64,
                    evidence_refs_json,
                    i64::from(edge.broken),
                    &edge.authority_effect,
                ],
            )?;
        }
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<CognitiveGraphMeta>> {
        let currents = self.list_meta_by_status(workspace_id, CognitiveGraphStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE cognitive_graph_snapshots
                 SET status = ?2, superseded_at = ?3
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = CognitiveGraphHistoryEntry::from_meta(&meta) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &CognitiveGraphHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "cognitive graph history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("cognitive_graph_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO cognitive_graph_history (
                id, workspace_id, snapshot_id, status, generated_at, superseded_at,
                node_count, edge_count, broken_node_count, broken_edge_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.snapshot_id,
                &entry.status,
                &entry.generated_at,
                &entry.superseded_at,
                entry.node_count as i64,
                entry.edge_count as i64,
                entry.broken_node_count as i64,
                entry.broken_edge_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<CognitiveGraphMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, generated_at, superseded_at,
                    node_count, edge_count, broken_node_count, broken_edge_count, authority_effect
             FROM cognitive_graph_snapshots
             WHERE workspace_id = ?1
             ORDER BY generated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_meta_by_status(
        &self,
        workspace_id: &str,
        status: CognitiveGraphStatus,
    ) -> Result<Vec<CognitiveGraphMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, generated_at, superseded_at,
                    node_count, edge_count, broken_node_count, broken_edge_count, authority_effect
             FROM cognitive_graph_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY generated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_view(&self, meta: &CognitiveGraphMeta) -> Result<CognitiveGraphView> {
        let nodes = self.list_nodes(&meta.id)?;
        let edges = self.list_edges(&meta.id)?;
        Ok(CognitiveGraphView {
            meta: meta.clone(),
            nodes,
            edges,
            authority_effect: CognitiveGraphView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn list_nodes(&self, snapshot_id: &str) -> Result<Vec<CognitiveGraphNode>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT external_ref, workspace_id, kind, title, broken, authority_effect
             FROM cognitive_graph_nodes
             WHERE snapshot_id = ?1
             ORDER BY external_ref ASC",
        )?;
        let rows = stmt.query_map([snapshot_id], map_node)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_edges(&self, snapshot_id: &str) -> Result<Vec<CognitiveGraphEdge>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, from_ref, to_ref, kind, explanation,
                    confidence, evidence_refs_json, broken, authority_effect
             FROM cognitive_graph_edges
             WHERE snapshot_id = ?1
             ORDER BY from_ref ASC, kind ASC, to_ref ASC",
        )?;
        let rows = stmt.query_map([snapshot_id], map_edge)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<CognitiveGraphHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT snapshot_id, status, generated_at, superseded_at,
                    node_count, edge_count, broken_node_count, broken_edge_count,
                    terminal, actionable, authority_effect
             FROM cognitive_graph_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM cognitive_graph_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

trait MetaAuthority {
    fn validate_authority(&self) -> Result<()>;
}

impl MetaAuthority for CognitiveGraphMeta {
    fn validate_authority(&self) -> Result<()> {
        if self.authority_effect != CognitiveGraphMeta::AUTHORITY_EFFECT_NONE {
            return Err(crate::error::DatabaseError::Migration(
                "graph meta authority_effect must be none".into(),
            ));
        }
        Ok(())
    }
}

fn map_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveGraphMeta> {
    let status = CognitiveGraphStatus::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveGraphMeta {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        status,
        generated_at: row.get(3)?,
        superseded_at: row.get(4)?,
        node_count: row.get::<_, i64>(5)? as usize,
        edge_count: row.get::<_, i64>(6)? as usize,
        broken_node_count: row.get::<_, i64>(7)? as usize,
        broken_edge_count: row.get::<_, i64>(8)? as usize,
        authority_effect: row.get(9)?,
    })
}

fn map_node(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveGraphNode> {
    let kind = CognitiveGraphNodeKind::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "kind".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveGraphNode {
        external_ref: row.get(0)?,
        workspace_id: row.get(1)?,
        kind,
        title: row.get(3)?,
        broken: row.get::<_, i64>(4)? != 0,
        authority_effect: row.get(5)?,
    })
}

fn map_edge(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveGraphEdge> {
    let kind = CognitiveGraphEdgeKind::parse(&row.get::<_, String>(4)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(4, "kind".into(), rusqlite::types::Type::Text)
    })?;
    let evidence_refs: Vec<String> =
        serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default();
    Ok(CognitiveGraphEdge {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        from_ref: row.get(2)?,
        to_ref: row.get(3)?,
        kind,
        explanation: row.get(5)?,
        confidence: row.get::<_, i64>(6)? as u8,
        evidence_refs,
        broken: row.get::<_, i64>(8)? != 0,
        authority_effect: row.get(9)?,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveGraphHistoryEntry> {
    Ok(CognitiveGraphHistoryEntry {
        snapshot_id: row.get(0)?,
        status: row.get(1)?,
        generated_at: row.get(2)?,
        superseded_at: row.get(3)?,
        node_count: row.get::<_, i64>(4)? as usize,
        edge_count: row.get::<_, i64>(5)? as usize,
        broken_node_count: row.get::<_, i64>(6)? as usize,
        broken_edge_count: row.get::<_, i64>(7)? as usize,
        terminal: row.get::<_, i64>(8)? != 0,
        actionable: row.get::<_, i64>(9)? != 0,
        authority_effect: row.get(10)?,
    })
}
