use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    OrchestrationArtefactKind, OrchestrationCycleDetected, OrchestrationDependency,
    OrchestrationEvidenceLink, OrchestrationHistoryEntry, OrchestrationObservation,
    OrchestrationRefreshStage, OrchestrationStatus, WorkspaceOrchestrationMeta,
    WorkspaceOrchestrationView,
};

/// Persistence guards for Programme II cognitive orchestration projections.
pub struct CognitiveOrchestrationRepository<'a> {
    db: &'a Database,
}

impl<'a> CognitiveOrchestrationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, view: &WorkspaceOrchestrationView) -> Result<()> {
        view.meta.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("orchestration meta invalid: {e}"))
        })?;
        if view.authority_effect != WorkspaceOrchestrationView::AUTHORITY_EFFECT_NONE {
            return Err(crate::error::DatabaseError::Migration(
                "orchestration view authority_effect must be none".into(),
            ));
        }

        let refresh_plan_json = serde_json::to_string(&view.refresh_plan).map_err(ser_err)?;
        let dependency_order_json =
            serde_json::to_string(&view.dependency_order).map_err(ser_err)?;
        let dependencies_json = serde_json::to_string(&view.dependencies).map_err(ser_err)?;
        let blocked_items_json = serde_json::to_string(&view.blocked_items).map_err(ser_err)?;
        let stale_items_json = serde_json::to_string(&view.stale_items).map_err(ser_err)?;
        let skipped_items_json = serde_json::to_string(&view.skipped_items).map_err(ser_err)?;
        let cycles_json = serde_json::to_string(&view.cycles).map_err(ser_err)?;
        let evidence_links_json = serde_json::to_string(&view.evidence_links).map_err(ser_err)?;
        let graph_links_json = serde_json::to_string(&view.graph_links).map_err(ser_err)?;
        let planning_links_json = serde_json::to_string(&view.planning_links).map_err(ser_err)?;
        let reasoning_links_json = serde_json::to_string(&view.reasoning_links).map_err(ser_err)?;
        let execution_links_json = serde_json::to_string(&view.execution_links).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_orchestration_snapshots (
                id, workspace_id, status, created_at, superseded_at, current_generation,
                uncertainty, rationale, refresh_plan_json, dependency_order_json,
                dependencies_json, blocked_items_json, stale_items_json, skipped_items_json,
                cycles_json, evidence_links_json, graph_links_json, planning_links_json,
                reasoning_links_json, execution_links_json, authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,'none',0,0)",
            rusqlite::params![
                &view.meta.orchestration_id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.created_at,
                &view.meta.superseded_at,
                view.meta.current_generation as i64,
                view.meta.uncertainty as i64,
                &view.meta.rationale,
                refresh_plan_json,
                dependency_order_json,
                dependencies_json,
                blocked_items_json,
                stale_items_json,
                skipped_items_json,
                cycles_json,
                evidence_links_json,
                graph_links_json,
                planning_links_json,
                reasoning_links_json,
                execution_links_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<WorkspaceOrchestrationMeta>> {
        let currents = self.list_meta_by_status(workspace_id, OrchestrationStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            let view = self.load_view(&meta)?;
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_orchestration_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.orchestration_id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = OrchestrationHistoryEntry::from_meta(
                &meta,
                view.refresh_plan.len(),
                view.cycles.len(),
            ) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &OrchestrationHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "orchestration history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("orchestration_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_orchestration_history (
                id, workspace_id, orchestration_id, status, created_at, superseded_at,
                current_generation, uncertainty, rationale_excerpt, stage_count, cycle_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,1,0,'none',?12)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.orchestration_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                entry.current_generation as i64,
                entry.uncertainty as i64,
                &entry.rationale_excerpt,
                entry.stage_count as i64,
                entry.cycle_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<WorkspaceOrchestrationMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, current_generation,
                    uncertainty, rationale, authority_effect, terminal, actionable
             FROM workspace_orchestration_snapshots
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
        status: OrchestrationStatus,
    ) -> Result<Vec<WorkspaceOrchestrationMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at, current_generation,
                    uncertainty, rationale, authority_effect, terminal, actionable
             FROM workspace_orchestration_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC, id ASC",
        )?;
        let rows = stmt.query_map((workspace_id, status.as_str()), map_meta)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_view(&self, meta: &WorkspaceOrchestrationMeta) -> Result<WorkspaceOrchestrationView> {
        let mut stmt = self.db.connection().prepare(
            "SELECT refresh_plan_json, dependency_order_json, dependencies_json,
                    blocked_items_json, stale_items_json, skipped_items_json, cycles_json,
                    evidence_links_json, graph_links_json, planning_links_json,
                    reasoning_links_json, execution_links_json
             FROM workspace_orchestration_snapshots
             WHERE id = ?1",
        )?;
        let row = stmt.query_row([&meta.orchestration_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;

        Ok(WorkspaceOrchestrationView {
            meta: meta.clone(),
            refresh_plan: de_json(&row.0)?,
            dependency_order: de_json::<Vec<OrchestrationArtefactKind>>(&row.1)?,
            dependencies: de_json::<Vec<OrchestrationDependency>>(&row.2)?,
            blocked_items: de_json::<Vec<OrchestrationObservation>>(&row.3)?,
            stale_items: de_json::<Vec<OrchestrationObservation>>(&row.4)?,
            skipped_items: de_json::<Vec<OrchestrationObservation>>(&row.5)?,
            cycles: de_json::<Vec<OrchestrationCycleDetected>>(&row.6)?,
            evidence_links: de_json::<Vec<OrchestrationEvidenceLink>>(&row.7)?,
            graph_links: de_json::<Vec<OrchestrationEvidenceLink>>(&row.8)?,
            planning_links: de_json::<Vec<OrchestrationEvidenceLink>>(&row.9)?,
            reasoning_links: de_json::<Vec<OrchestrationEvidenceLink>>(&row.10)?,
            execution_links: de_json::<Vec<OrchestrationEvidenceLink>>(&row.11)?,
            authority_effect: WorkspaceOrchestrationView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<OrchestrationHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT orchestration_id, status, created_at, superseded_at, current_generation,
                    uncertainty, rationale_excerpt, stage_count, cycle_count,
                    terminal, actionable, authority_effect
             FROM workspace_orchestration_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_history)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_orchestration_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(e: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("orchestration serialize: {e}"))
}

fn de_json<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T> {
    serde_json::from_str(raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("orchestration deserialize: {e}"))
    })
}

fn map_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceOrchestrationMeta> {
    let status = OrchestrationStatus::parse(&row.get::<_, String>(2)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(WorkspaceOrchestrationMeta {
        orchestration_id: row.get(0)?,
        workspace_id: row.get(1)?,
        status,
        created_at: row.get(3)?,
        superseded_at: row.get(4)?,
        current_generation: row.get::<_, i64>(5)? as u64,
        uncertainty: row.get::<_, i64>(6)? as u8,
        rationale: row.get(7)?,
        authority_effect: row.get(8)?,
        terminal: row.get::<_, i64>(9)? != 0,
        actionable: row.get::<_, i64>(10)? != 0,
    })
}

fn map_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<OrchestrationHistoryEntry> {
    Ok(OrchestrationHistoryEntry {
        orchestration_id: row.get(0)?,
        status: row.get(1)?,
        created_at: row.get(2)?,
        superseded_at: row.get(3)?,
        current_generation: row.get::<_, i64>(4)? as u64,
        uncertainty: row.get::<_, i64>(5)? as u8,
        rationale_excerpt: row.get(6)?,
        stage_count: row.get::<_, i64>(7)? as usize,
        cycle_count: row.get::<_, i64>(8)? as usize,
        terminal: row.get::<_, i64>(9)? != 0,
        actionable: row.get::<_, i64>(10)? != 0,
        authority_effect: row.get(11)?,
    })
}
