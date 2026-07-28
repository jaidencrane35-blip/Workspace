use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ReasoningEvidenceReference, ReasoningHistoryEntry, ReasoningLink, ReasoningRecord,
    ReasoningRecordStatus,
};

/// Persistence guards for Programme II reasoning memory (evidence only).
pub struct ReasoningMemoryRepository<'a> {
    db: &'a Database,
}

impl<'a> ReasoningMemoryRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_record(&self, record: &ReasoningRecord) -> Result<()> {
        record.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("reasoning record invalid: {e}"))
        })?;
        let assumptions_json = serde_json::to_string(&record.assumptions).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("assumptions serialize: {e}"))
        })?;
        let alternatives_json = serde_json::to_string(&record.alternatives).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("alternatives serialize: {e}"))
        })?;
        let rejected_json = serde_json::to_string(&record.rejected_alternatives).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("rejected serialize: {e}"))
        })?;
        let evidence_refs_json = serde_json::to_string(&record.evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence_refs serialize: {e}"))
        })?;
        let links_json = serde_json::to_string(&record.links).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("links serialize: {e}"))
        })?;
        let confidence_evolution_json =
            serde_json::to_string(&record.confidence_evolution).map_err(|e| {
                crate::error::DatabaseError::Migration(format!("confidence_evolution serialize: {e}"))
            })?;
        let uncertainty_evolution_json =
            serde_json::to_string(&record.uncertainty_evolution).map_err(|e| {
                crate::error::DatabaseError::Migration(format!(
                    "uncertainty_evolution serialize: {e}"
                ))
            })?;
        let lessons_json = serde_json::to_string(&record.lessons).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("lessons serialize: {e}"))
        })?;

        self.db.connection().execute(
            "INSERT INTO reasoning_records (
                id, workspace_id, title, status, hypothesis,
                assumptions_json, alternatives_json, rejected_alternatives_json,
                evidence_refs_json, links_json, confidence, uncertainty,
                confidence_evolution_json, uncertainty_evolution_json,
                reflection, lessons_json, rationale,
                created_at, updated_at, superseded_at, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                status = excluded.status,
                hypothesis = excluded.hypothesis,
                assumptions_json = excluded.assumptions_json,
                alternatives_json = excluded.alternatives_json,
                rejected_alternatives_json = excluded.rejected_alternatives_json,
                evidence_refs_json = excluded.evidence_refs_json,
                links_json = excluded.links_json,
                confidence = excluded.confidence,
                uncertainty = excluded.uncertainty,
                confidence_evolution_json = excluded.confidence_evolution_json,
                uncertainty_evolution_json = excluded.uncertainty_evolution_json,
                reflection = excluded.reflection,
                lessons_json = excluded.lessons_json,
                rationale = excluded.rationale,
                updated_at = excluded.updated_at,
                superseded_at = excluded.superseded_at,
                authority_effect = excluded.authority_effect
             WHERE reasoning_records.workspace_id = excluded.workspace_id",
            rusqlite::params![
                &record.id,
                &record.workspace_id,
                &record.title,
                record.status.as_str(),
                &record.hypothesis,
                assumptions_json,
                alternatives_json,
                rejected_json,
                evidence_refs_json,
                links_json,
                record.confidence as i64,
                record.uncertainty as i64,
                confidence_evolution_json,
                uncertainty_evolution_json,
                &record.reflection,
                lessons_json,
                &record.rationale,
                &record.created_at,
                &record.updated_at,
                &record.superseded_at,
                &record.authority_effect,
            ],
        )?;
        Ok(())
    }

    /// Mark all current records superseded and append history rows (append-only).
    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<ReasoningRecord>> {
        let currents = self.list_by_status(workspace_id, ReasoningRecordStatus::Current)?;
        let mut superseded = Vec::new();
        for mut record in currents {
            record.mark_superseded(superseded_at);
            self.upsert_record(&record)?;
            if let Some(entry) = ReasoningHistoryEntry::from_record(&record) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            superseded.push(record);
        }
        Ok(superseded)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &ReasoningHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "reasoning history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("reasoning_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO reasoning_history (
                id, workspace_id, record_id, title, status, confidence, uncertainty,
                created_at, superseded_at, reflection_excerpt, terminal, actionable,
                authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.record_id,
                &entry.title,
                &entry.status,
                entry.confidence as i64,
                entry.uncertainty as i64,
                &entry.created_at,
                &entry.superseded_at,
                &entry.reflection_excerpt,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_records(&self, workspace_id: &str) -> Result<Vec<ReasoningRecord>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, title, status, hypothesis,
                    assumptions_json, alternatives_json, rejected_alternatives_json,
                    evidence_refs_json, links_json, confidence, uncertainty,
                    confidence_evolution_json, uncertainty_evolution_json,
                    reflection, lessons_json, rationale,
                    created_at, updated_at, superseded_at, authority_effect
             FROM reasoning_records
             WHERE workspace_id = ?1
             ORDER BY updated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_record)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: ReasoningRecordStatus,
    ) -> Result<Vec<ReasoningRecord>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, title, status, hypothesis,
                    assumptions_json, alternatives_json, rejected_alternatives_json,
                    evidence_refs_json, links_json, confidence, uncertainty,
                    confidence_evolution_json, uncertainty_evolution_json,
                    reflection, lessons_json, rationale,
                    created_at, updated_at, superseded_at, authority_effect
             FROM reasoning_records
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY updated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_record)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<ReasoningHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT record_id, title, status, confidence, uncertainty,
                    created_at, superseded_at, reflection_excerpt,
                    terminal, actionable, authority_effect
             FROM reasoning_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM reasoning_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn parse_string_vec(raw: String) -> Vec<String> {
    serde_json::from_str(&raw).unwrap_or_default()
}

fn parse_u8_vec(raw: String) -> Vec<u8> {
    serde_json::from_str(&raw).unwrap_or_default()
}

fn map_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<ReasoningRecord> {
    let status = ReasoningRecordStatus::parse(&row.get::<_, String>(3)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(3, "status".into(), rusqlite::types::Type::Text)
    })?;
    let evidence_refs: Vec<ReasoningEvidenceReference> =
        serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default();
    let links: Vec<ReasoningLink> =
        serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default();
    Ok(ReasoningRecord {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        title: row.get(2)?,
        status,
        hypothesis: row.get(4)?,
        assumptions: parse_string_vec(row.get(5)?),
        alternatives: parse_string_vec(row.get(6)?),
        rejected_alternatives: parse_string_vec(row.get(7)?),
        evidence_refs,
        links,
        confidence: row.get::<_, i64>(10)? as u8,
        uncertainty: row.get::<_, i64>(11)? as u8,
        confidence_evolution: parse_u8_vec(row.get(12)?),
        uncertainty_evolution: parse_u8_vec(row.get(13)?),
        reflection: row.get(14)?,
        lessons: parse_string_vec(row.get(15)?),
        rationale: row.get(16)?,
        created_at: row.get(17)?,
        updated_at: row.get(18)?,
        superseded_at: row.get(19)?,
        authority_effect: row.get(20)?,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<ReasoningHistoryEntry> {
    Ok(ReasoningHistoryEntry {
        record_id: row.get(0)?,
        title: row.get(1)?,
        status: row.get(2)?,
        confidence: row.get::<_, i64>(3)? as u8,
        uncertainty: row.get::<_, i64>(4)? as u8,
        created_at: row.get(5)?,
        superseded_at: row.get(6)?,
        reflection_excerpt: row.get(7)?,
        terminal: row.get::<_, i64>(8)? != 0,
        actionable: row.get::<_, i64>(9)? != 0,
        authority_effect: row.get(10)?,
    })
}
