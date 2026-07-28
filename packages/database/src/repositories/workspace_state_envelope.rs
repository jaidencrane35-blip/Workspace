use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    EnvelopeStatus, WorkspaceStateEnvelope, WorkspaceStateHistoryEntry,
};

/// Persistence guards for Programme III unified workspace state envelopes.
/// Read-model only — no transitions, repair, or authority.
pub struct WorkspaceStateEnvelopeRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceStateEnvelopeRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_envelope(&self, envelope: &WorkspaceStateEnvelope) -> Result<()> {
        envelope.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("state envelope invalid: {e}"))
        })?;

        let sources_json = serde_json::to_string(&envelope.sources).map_err(ser_err)?;
        let contradictions_json =
            serde_json::to_string(&envelope.contradictions).map_err(ser_err)?;
        let unknowns_json = serde_json::to_string(&envelope.unknowns).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_state_envelope_snapshots (
                id, workspace_id, revision, status, created_at, superseded_at,
                freshness, completeness, consistency, composition_status,
                source_count, conflict_count, unknown_count,
                sources_json, contradictions_json, unknowns_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,'none',0,0)",
            rusqlite::params![
                &envelope.state_id,
                &envelope.workspace_id,
                &envelope.revision,
                envelope.status.as_str(),
                &envelope.generated_at,
                &envelope.superseded_at,
                envelope.freshness.as_str(),
                envelope.completeness.as_str(),
                envelope.consistency.as_str(),
                &envelope.composition_status,
                envelope.sources.len() as i64,
                envelope.contradictions.len() as i64,
                envelope.unknowns.len() as i64,
                sources_json,
                contradictions_json,
                unknowns_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<WorkspaceStateEnvelope>> {
        let currents = self.list_current(workspace_id)?;
        let mut out = Vec::new();
        for mut envelope in currents {
            envelope.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_state_envelope_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &envelope.state_id,
                    envelope.status.as_str(),
                    &envelope.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = WorkspaceStateHistoryEntry::from_envelope(&envelope) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(envelope);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &WorkspaceStateHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "state envelope history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_state_envelope_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_state_envelope_history (
                id, workspace_id, state_id, revision, status, created_at, superseded_at,
                freshness, completeness, consistency, composition_status,
                source_count, conflict_count, unknown_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,1,0,'none',?15)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.state_id,
                &entry.revision,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.freshness,
                &entry.completeness,
                &entry.consistency,
                &entry.composition_status,
                entry.source_count as i64,
                entry.conflict_count as i64,
                entry.unknown_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_current(&self, workspace_id: &str) -> Result<Vec<WorkspaceStateEnvelope>> {
        self.list_by_status(workspace_id, EnvelopeStatus::Current)
    }

    /// Load all durable envelope bodies (current + superseded + archived), oldest first.
    /// Read-only evidence for historical reconstruction — never generates or repairs.
    pub fn list_durable_envelopes(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceStateEnvelope>> {
        let mut out = Vec::new();
        out.extend(self.list_by_status(workspace_id, EnvelopeStatus::Current)?);
        out.extend(self.list_by_status(workspace_id, EnvelopeStatus::Superseded)?);
        out.extend(self.list_by_status(workspace_id, EnvelopeStatus::Archived)?);
        out.sort_by(|a, b| a.generated_at.cmp(&b.generated_at));
        Ok(out)
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: EnvelopeStatus,
    ) -> Result<Vec<WorkspaceStateEnvelope>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, revision, status, created_at, superseded_at,
                    freshness, completeness, consistency, composition_status,
                    sources_json, contradictions_json, unknowns_json,
                    authority_effect, terminal, actionable
             FROM workspace_state_envelope_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![workspace_id, status.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, i64>(14)?,
                row.get::<_, i64>(15)?,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (
                state_id,
                workspace_id,
                revision,
                status_raw,
                created_at,
                superseded_at,
                freshness,
                completeness,
                consistency,
                composition_status,
                sources_json,
                contradictions_json,
                unknowns_json,
                authority_effect,
                terminal,
                actionable,
            ) = row?;
            let status = EnvelopeStatus::parse(&status_raw).map_err(|e| {
                crate::error::DatabaseError::Migration(format!("bad envelope status: {e}"))
            })?;
            let freshness = workspace_domain::FreshnessStatus::parse(&freshness).map_err(|e| {
                crate::error::DatabaseError::Migration(format!("bad freshness: {e}"))
            })?;
            let completeness =
                workspace_domain::CompletenessStatus::parse(&completeness).map_err(|e| {
                    crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
                })?;
            let consistency =
                workspace_domain::ConsistencyStatus::parse(&consistency).map_err(|e| {
                    crate::error::DatabaseError::Migration(format!("bad consistency: {e}"))
                })?;
            let sources = serde_json::from_str(&sources_json).map_err(ser_err)?;
            let contradictions = serde_json::from_str(&contradictions_json).map_err(ser_err)?;
            let unknowns = serde_json::from_str(&unknowns_json).map_err(ser_err)?;
            out.push(WorkspaceStateEnvelope {
                state_id,
                workspace_id,
                revision,
                generated_at: created_at,
                status,
                superseded_at,
                sources,
                freshness,
                completeness,
                consistency,
                unknowns,
                contradictions,
                composition_status,
                authority_effect,
                actionable: actionable != 0,
                terminal: terminal != 0,
            });
        }
        Ok(out)
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<WorkspaceStateHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT state_id, revision, status, created_at, superseded_at,
                    freshness, completeness, consistency, composition_status,
                    source_count, conflict_count, unknown_count,
                    terminal, actionable, authority_effect
             FROM workspace_state_envelope_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(WorkspaceStateHistoryEntry {
                state_id: row.get(0)?,
                revision: row.get(1)?,
                status: row.get(2)?,
                created_at: row.get(3)?,
                superseded_at: row.get(4)?,
                freshness: row.get(5)?,
                completeness: row.get(6)?,
                consistency: row.get(7)?,
                composition_status: row.get(8)?,
                source_count: row.get::<_, i64>(9)? as usize,
                conflict_count: row.get::<_, i64>(10)? as usize,
                unknown_count: row.get::<_, i64>(11)? as usize,
                terminal: row.get::<_, i64>(12)? != 0,
                actionable: row.get::<_, i64>(13)? != 0,
                authority_effect: row.get(14)?,
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
            "SELECT COUNT(*) FROM workspace_state_envelope_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
