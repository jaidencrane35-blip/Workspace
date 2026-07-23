use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    MemoryEntry, MemoryEntryId, MemoryLifecycleState, MemoryMetadata, MemoryType, WorkspaceId,
};

/// Persistence for governed AI memory entries (informational only).
pub struct AiMemoryRepository<'a> {
    db: &'a Database,
}

impl<'a> AiMemoryRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert(&self, entry: &MemoryEntry) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO ai_memory_entries (
                id, memory_type, key, summary, source, workspace_id, lifecycle,
                confidence_level, occurrence_count, user_visible, attributes,
                created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(id) DO UPDATE SET
                memory_type = excluded.memory_type,
                key = excluded.key,
                summary = excluded.summary,
                source = excluded.source,
                workspace_id = excluded.workspace_id,
                lifecycle = excluded.lifecycle,
                confidence_level = excluded.confidence_level,
                occurrence_count = excluded.occurrence_count,
                user_visible = excluded.user_visible,
                attributes = excluded.attributes,
                updated_at = excluded.updated_at",
            (
                entry.id.as_str(),
                entry.memory_type.as_str(),
                &entry.key,
                &entry.summary,
                &entry.source,
                entry.workspace_id.as_ref().map(|id| id.as_str().to_string()),
                entry.lifecycle.as_str(),
                i64::from(entry.metadata.confidence_level),
                i64::from(entry.metadata.occurrence_count),
                if entry.metadata.user_visible { 1_i64 } else { 0 },
                &entry.metadata.attributes,
                &entry.created_at,
                &entry.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &MemoryEntryId) -> Result<Option<MemoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, memory_type, key, summary, source, workspace_id, lifecycle,
                    confidence_level, occurrence_count, user_visible, attributes,
                    created_at, updated_at
             FROM ai_memory_entries
             WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_row(row)?));
        }
        Ok(None)
    }

    pub fn list_active(
        &self,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>> {
        let limit = limit.clamp(1, 200) as i64;
        if let Some(workspace_id) = workspace_id {
            let mut stmt = self.db.connection().prepare(
                "SELECT id, memory_type, key, summary, source, workspace_id, lifecycle,
                        confidence_level, occurrence_count, user_visible, attributes,
                        created_at, updated_at
                 FROM ai_memory_entries
                 WHERE lifecycle != 'deleted'
                   AND (workspace_id IS NULL OR workspace_id = ?1)
                 ORDER BY updated_at DESC
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map((workspace_id, limit), map_row)?;
            return rows
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into);
        }

        let mut stmt = self.db.connection().prepare(
            "SELECT id, memory_type, key, summary, source, workspace_id, lifecycle,
                    confidence_level, occurrence_count, user_visible, attributes,
                    created_at, updated_at
             FROM ai_memory_entries
             WHERE lifecycle != 'deleted'
             ORDER BY updated_at DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], map_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn soft_delete(&self, id: &MemoryEntryId, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE ai_memory_entries
             SET lifecycle = 'deleted', updated_at = ?2
             WHERE id = ?1 AND lifecycle != 'deleted'",
            (id.as_str(), updated_at),
        )?;
        Ok(changed > 0)
    }

    pub fn clear_active(
        &self,
        memory_type: Option<MemoryType>,
        workspace_id: Option<&str>,
        updated_at: &str,
    ) -> Result<usize> {
        let changed = match (memory_type, workspace_id) {
            (Some(memory_type), Some(workspace_id)) => self.db.connection().execute(
                "UPDATE ai_memory_entries
                 SET lifecycle = 'deleted', updated_at = ?3
                 WHERE lifecycle != 'deleted'
                   AND memory_type = ?1
                   AND (workspace_id IS NULL OR workspace_id = ?2)",
                (memory_type.as_str(), workspace_id, updated_at),
            )?,
            (Some(memory_type), None) => self.db.connection().execute(
                "UPDATE ai_memory_entries
                 SET lifecycle = 'deleted', updated_at = ?2
                 WHERE lifecycle != 'deleted' AND memory_type = ?1",
                (memory_type.as_str(), updated_at),
            )?,
            (None, Some(workspace_id)) => self.db.connection().execute(
                "UPDATE ai_memory_entries
                 SET lifecycle = 'deleted', updated_at = ?2
                 WHERE lifecycle != 'deleted'
                   AND (workspace_id IS NULL OR workspace_id = ?1)",
                (workspace_id, updated_at),
            )?,
            (None, None) => self.db.connection().execute(
                "UPDATE ai_memory_entries
                 SET lifecycle = 'deleted', updated_at = ?1
                 WHERE lifecycle != 'deleted'",
                (updated_at,),
            )?,
        };
        Ok(changed as usize)
    }
}

fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryEntry> {
    let memory_type = MemoryType::parse(row.get::<_, String>(1)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let lifecycle = MemoryLifecycleState::parse(row.get::<_, String>(6)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let workspace_id = row
        .get::<_, Option<String>>(5)?
        .map(WorkspaceId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let confidence_level = row.get::<_, i64>(7)? as u8;
    let occurrence_count = row.get::<_, i64>(8)? as u32;
    let user_visible = row.get::<_, i64>(9)? != 0;

    Ok(MemoryEntry {
        id: MemoryEntryId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        memory_type,
        key: row.get(2)?,
        summary: row.get(3)?,
        source: row.get(4)?,
        workspace_id,
        lifecycle,
        metadata: MemoryMetadata {
            confidence_level,
            occurrence_count,
            user_visible,
            attributes: row.get(10)?,
        },
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

// AiMemoryError needs std::error::Error for ToSqlConversionFailure - thiserror provides it.
