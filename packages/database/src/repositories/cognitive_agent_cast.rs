use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AgentCritique, AgentPerspective, AgentSynthesis, CastEvidenceLink, CastStatus, CognitiveAgent,
    CognitiveAgentCastHistoryEntry, CognitiveAgentCastMeta, CognitiveAgentCastView,
};

/// Persistence guards for Programme II cognitive agent cast projections.
pub struct CognitiveAgentCastRepository<'a> {
    db: &'a Database,
}

impl<'a> CognitiveAgentCastRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &CognitiveAgentCastView) -> Result<()> {
        view.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("agent cast view invalid: {e}"))
        })?;

        let agents_json = serde_json::to_string(&view.agents).map_err(ser_err)?;
        let roles_json = serde_json::to_string(&view.roles).map_err(ser_err)?;
        let perspectives_json = serde_json::to_string(&view.perspectives).map_err(ser_err)?;
        let critiques_json = serde_json::to_string(&view.critiques).map_err(ser_err)?;
        let syntheses_json = serde_json::to_string(&view.syntheses).map_err(ser_err)?;
        let evidence_links_json = serde_json::to_string(&view.evidence_links).map_err(ser_err)?;
        let source_references_json =
            serde_json::to_string(&view.source_references).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_cognitive_agent_cast_snapshots (
                id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                agent_count, perspective_count, critique_count, synthesis_count,
                agents_json, roles_json, perspectives_json, critiques_json, syntheses_json,
                evidence_links_json, source_references_json, authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,'none',0,0)",
            rusqlite::params![
                &view.meta.cast_id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.created_at,
                &view.meta.superseded_at,
                view.meta.confidence as i64,
                view.meta.uncertainty as i64,
                view.meta.agent_count as i64,
                view.meta.perspective_count as i64,
                view.meta.critique_count as i64,
                view.meta.synthesis_count as i64,
                agents_json,
                roles_json,
                perspectives_json,
                critiques_json,
                syntheses_json,
                evidence_links_json,
                source_references_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<CognitiveAgentCastMeta>> {
        let currents = self.list_meta_by_status(workspace_id, CastStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_cognitive_agent_cast_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.cast_id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = CognitiveAgentCastHistoryEntry::from_meta(&meta) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &CognitiveAgentCastHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "agent cast history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("cognitive_cast_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_cognitive_agent_cast_history (
                id, workspace_id, cast_id, status, created_at, superseded_at,
                confidence, uncertainty, agent_count, perspective_count, critique_count,
                synthesis_count, terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,1,0,'none',?13)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.cast_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                entry.confidence as i64,
                entry.uncertainty as i64,
                entry.agent_count as i64,
                entry.perspective_count as i64,
                entry.critique_count as i64,
                entry.synthesis_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<CognitiveAgentCastMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                    agent_count, perspective_count, critique_count, synthesis_count,
                    authority_effect, terminal, actionable
             FROM workspace_cognitive_agent_cast_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_meta_by_status(
        &self,
        workspace_id: &str,
        status: CastStatus,
    ) -> Result<Vec<CognitiveAgentCastMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, confidence, uncertainty,
                    agent_count, perspective_count, critique_count, synthesis_count,
                    authority_effect, terminal, actionable
             FROM workspace_cognitive_agent_cast_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_view(&self, meta: &CognitiveAgentCastMeta) -> Result<CognitiveAgentCastView> {
        let mut stmt = self.db.connection().prepare(
            "SELECT agents_json, roles_json, perspectives_json, critiques_json, syntheses_json,
                    evidence_links_json, source_references_json, confidence, uncertainty
             FROM workspace_cognitive_agent_cast_snapshots
             WHERE id = ?1",
        )?;
        let row = stmt.query_row([&meta.cast_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, i64>(7)? as u8,
                row.get::<_, i64>(8)? as u8,
            ))
        })?;

        Ok(CognitiveAgentCastView {
            meta: meta.clone(),
            agents: de_json::<Vec<CognitiveAgent>>(&row.0)?,
            roles: de_json(&row.1)?,
            perspectives: de_json::<Vec<AgentPerspective>>(&row.2)?,
            critiques: de_json::<Vec<AgentCritique>>(&row.3)?,
            syntheses: de_json::<Vec<AgentSynthesis>>(&row.4)?,
            evidence_links: de_json::<Vec<CastEvidenceLink>>(&row.5)?,
            source_references: de_json::<Vec<CastEvidenceLink>>(&row.6)?,
            confidence: row.7,
            uncertainty: row.8,
            authority_effect: CognitiveAgentCastView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<CognitiveAgentCastHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT cast_id, status, created_at, superseded_at, confidence, uncertainty,
                    agent_count, perspective_count, critique_count, synthesis_count,
                    terminal, actionable, authority_effect
             FROM workspace_cognitive_agent_cast_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_cognitive_agent_cast_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("agent cast serialize: {e}"))
}

fn de_json<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T> {
    serde_json::from_str(raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("agent cast deserialize: {e}"))
    })
}

fn map_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveAgentCastMeta> {
    let status = CastStatus::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(CognitiveAgentCastMeta {
        cast_id: row.get(0)?,
        workspace_id: row.get(1)?,
        status,
        created_at: row.get(3)?,
        superseded_at: row.get(4)?,
        confidence: row.get::<_, i64>(5)? as u8,
        uncertainty: row.get::<_, i64>(6)? as u8,
        agent_count: row.get::<_, i64>(7)? as usize,
        perspective_count: row.get::<_, i64>(8)? as usize,
        critique_count: row.get::<_, i64>(9)? as usize,
        synthesis_count: row.get::<_, i64>(10)? as usize,
        authority_effect: row.get(11)?,
        terminal: row.get::<_, i64>(12)? != 0,
        actionable: row.get::<_, i64>(13)? != 0,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<CognitiveAgentCastHistoryEntry> {
    Ok(CognitiveAgentCastHistoryEntry {
        cast_id: row.get(0)?,
        status: row.get(1)?,
        created_at: row.get(2)?,
        superseded_at: row.get(3)?,
        confidence: row.get::<_, i64>(4)? as u8,
        uncertainty: row.get::<_, i64>(5)? as u8,
        agent_count: row.get::<_, i64>(6)? as usize,
        perspective_count: row.get::<_, i64>(7)? as usize,
        critique_count: row.get::<_, i64>(8)? as usize,
        synthesis_count: row.get::<_, i64>(9)? as usize,
        terminal: row.get::<_, i64>(10)? != 0,
        actionable: row.get::<_, i64>(11)? != 0,
        authority_effect: row.get(12)?,
    })
}
