use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    CognitiveNode, CognitiveNodeKind, CognitiveNodeStatus, CognitiveRelation,
    CognitiveRelationKind,
};

/// Persistence guards for Programme II cognitive semantic nodes/relations.
pub struct CognitiveModelRepository<'a> {
    db: &'a Database,
}

impl<'a> CognitiveModelRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_node(&self, node: &CognitiveNode) -> Result<()> {
        node.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("cognitive node invalid: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO cognitive_nodes (
                id, workspace_id, kind, title, description, status,
                importance, confidence, uncertainty, is_current_focus,
                external_ref, parent_id, created_at, updated_at, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)
             ON CONFLICT(id) DO UPDATE SET
                kind = excluded.kind,
                title = excluded.title,
                description = excluded.description,
                status = excluded.status,
                importance = excluded.importance,
                confidence = excluded.confidence,
                uncertainty = excluded.uncertainty,
                is_current_focus = excluded.is_current_focus,
                external_ref = excluded.external_ref,
                parent_id = excluded.parent_id,
                updated_at = excluded.updated_at,
                authority_effect = excluded.authority_effect
             WHERE cognitive_nodes.workspace_id = excluded.workspace_id",
            (
                &node.id,
                &node.workspace_id,
                node.kind.as_str(),
                &node.title,
                &node.description,
                node.status.as_str(),
                node.importance as i64,
                node.confidence as i64,
                node.uncertainty as i64,
                i64::from(node.is_current_focus),
                &node.external_ref,
                &node.parent_id,
                &node.created_at,
                &node.updated_at,
                &node.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn get_node(&self, id: &str) -> Result<Option<CognitiveNode>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, kind, title, description, status,
                    importance, confidence, uncertainty, is_current_focus,
                    external_ref, parent_id, created_at, updated_at, authority_effect
             FROM cognitive_nodes WHERE id = ?1 LIMIT 1",
        )?;
        let mut rows = stmt.query_map([id], map_node)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn list_nodes(&self, workspace_id: &str) -> Result<Vec<CognitiveNode>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, kind, title, description, status,
                    importance, confidence, uncertainty, is_current_focus,
                    external_ref, parent_id, created_at, updated_at, authority_effect
             FROM cognitive_nodes
             WHERE workspace_id = ?1
             ORDER BY is_current_focus DESC, importance DESC, updated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_node)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn clear_focus(&self, workspace_id: &str, updated_at: &str) -> Result<()> {
        self.db.connection().execute(
            "UPDATE cognitive_nodes
             SET is_current_focus = 0, updated_at = ?2
             WHERE workspace_id = ?1 AND is_current_focus = 1",
            (workspace_id, updated_at),
        )?;
        Ok(())
    }

    pub fn upsert_relation(&self, relation: &CognitiveRelation) -> Result<()> {
        relation.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("cognitive relation invalid: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO cognitive_relations (
                id, workspace_id, from_id, to_id, kind, confidence,
                explanation, created_at, updated_at, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
             ON CONFLICT(id) DO UPDATE SET
                kind = excluded.kind,
                confidence = excluded.confidence,
                explanation = excluded.explanation,
                updated_at = excluded.updated_at,
                authority_effect = excluded.authority_effect
             WHERE cognitive_relations.workspace_id = excluded.workspace_id
               AND cognitive_relations.from_id = excluded.from_id
               AND cognitive_relations.to_id = excluded.to_id",
            (
                &relation.id,
                &relation.workspace_id,
                &relation.from_id,
                &relation.to_id,
                relation.kind.as_str(),
                relation.confidence as i64,
                &relation.explanation,
                &relation.created_at,
                &relation.updated_at,
                &relation.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn list_relations(&self, workspace_id: &str) -> Result<Vec<CognitiveRelation>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, from_id, to_id, kind, confidence,
                    explanation, created_at, updated_at, authority_effect
             FROM cognitive_relations
             WHERE workspace_id = ?1
             ORDER BY updated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_relation)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_node(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveNode> {
    let kind = CognitiveNodeKind::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "kind".into(), rusqlite::types::Type::Text)
    })?;
    let status = CognitiveNodeStatus::parse(&row.get::<_, String>(5)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(5, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveNode {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        kind,
        title: row.get(3)?,
        description: row.get(4)?,
        status,
        importance: row.get::<_, i64>(6)? as u8,
        confidence: row.get::<_, i64>(7)? as u8,
        uncertainty: row.get::<_, i64>(8)? as u8,
        is_current_focus: row.get::<_, i64>(9)? != 0,
        external_ref: row.get(10)?,
        parent_id: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
        authority_effect: row.get(14)?,
    })
}

fn map_relation(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveRelation> {
    let kind = CognitiveRelationKind::parse(&row.get::<_, String>(4)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(4, "kind".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveRelation {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        from_id: row.get(2)?,
        to_id: row.get(3)?,
        kind,
        confidence: row.get::<_, i64>(5)? as u8,
        explanation: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
        authority_effect: row.get(9)?,
    })
}
