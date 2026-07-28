use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    GovernanceEvaluationStatus, GovernanceRecommendation, PolicyDefinition, PolicyEvaluation,
    PolicyGovernanceHistoryEntry, PolicyGovernanceMeta, PolicyGovernanceView,
};

/// Persistence guards for Programme III policy governance evidence.
/// Stores policies / evaluations / explanations / history — never evaluates or approves.
pub struct PolicyGovernanceRepository<'a> {
    db: &'a Database,
}

impl<'a> PolicyGovernanceRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_policy(&self, policy: &PolicyDefinition) -> Result<()> {
        policy.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("policy invalid: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO workspace_policy_definitions (
                policy_id, name, description, scope, version, severity, status,
                authority_effect, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,'none',0)
             ON CONFLICT(policy_id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                scope = excluded.scope,
                version = excluded.version,
                severity = excluded.severity,
                status = excluded.status,
                authority_effect = 'none',
                actionable = 0",
            rusqlite::params![
                &policy.policy_id,
                &policy.name,
                &policy.description,
                policy.scope.as_str(),
                &policy.version,
                policy.severity.as_str(),
                policy.status.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn list_policies(&self) -> Result<Vec<PolicyDefinition>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT policy_id, name, description, scope, version, severity, status,
                    authority_effect, actionable
             FROM workspace_policy_definitions
             ORDER BY policy_id ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, i64>(8)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (
                policy_id,
                name,
                description,
                scope,
                version,
                severity,
                status,
                authority_effect,
                actionable,
            ) = row?;
            out.push(PolicyDefinition {
                policy_id,
                name,
                description,
                scope: workspace_domain::PolicyScope::parse(&scope).map_err(|e| {
                    crate::error::DatabaseError::Migration(format!("bad scope: {e}"))
                })?,
                version,
                severity: workspace_domain::PolicySeverity::parse(&severity).map_err(|e| {
                    crate::error::DatabaseError::Migration(format!("bad severity: {e}"))
                })?,
                status: workspace_domain::PolicyDefinitionStatus::parse(&status).map_err(|e| {
                    crate::error::DatabaseError::Migration(format!("bad status: {e}"))
                })?,
                authority_effect,
                actionable: actionable != 0,
            });
        }
        Ok(out)
    }

    pub fn persist_view(&self, view: &PolicyGovernanceView) -> Result<()> {
        view.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("policy governance view invalid: {e}"))
        })?;
        let policies_json = serde_json::to_string(&view.policies).map_err(ser_err)?;
        let evaluations_json = serde_json::to_string(&view.evaluations).map_err(ser_err)?;
        let recommendation_json = match &view.recommendation {
            Some(r) => Some(serde_json::to_string(r).map_err(ser_err)?),
            None => None,
        };
        let unknowns_json = serde_json::to_string(&view.unknowns).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_policy_governance_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                context_revision, policy_catalog_revision, evaluation_count, aggregate_result,
                policies_json, evaluations_json, recommendation_json, unknowns_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,'none',0,0)",
            rusqlite::params![
                &view.meta.evaluation_set_id,
                &view.meta.workspace_id,
                view.meta.status.as_str(),
                &view.meta.created_at,
                &view.meta.superseded_at,
                &view.meta.context_revision,
                &view.meta.policy_catalog_revision,
                view.meta.evaluation_count as i64,
                &view.meta.aggregate_result,
                policies_json,
                evaluations_json,
                recommendation_json,
                unknowns_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<PolicyGovernanceMeta>> {
        let currents = self.list_meta_by_status(workspace_id, GovernanceEvaluationStatus::Current)?;
        let mut out = Vec::new();
        for mut meta in currents {
            meta.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_policy_governance_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &meta.evaluation_set_id,
                    meta.status.as_str(),
                    &meta.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = PolicyGovernanceHistoryEntry::from_meta(&meta) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(meta);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &PolicyGovernanceHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "policy governance history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("policy_governance_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_policy_governance_history (
                id, workspace_id, evaluation_set_id, status, created_at, superseded_at,
                context_revision, policy_catalog_revision, evaluation_count, aggregate_result,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.evaluation_set_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.context_revision,
                &entry.policy_catalog_revision,
                entry.evaluation_count as i64,
                &entry.aggregate_result,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_meta_by_status(
        &self,
        workspace_id: &str,
        status: GovernanceEvaluationStatus,
    ) -> Result<Vec<PolicyGovernanceMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    context_revision, policy_catalog_revision, evaluation_count, aggregate_result,
                    authority_effect, terminal, actionable
             FROM workspace_policy_governance_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![workspace_id, status.as_str()], |row| {
            Ok(PolicyGovernanceMeta {
                evaluation_set_id: row.get(0)?,
                workspace_id: row.get(1)?,
                status: GovernanceEvaluationStatus::parse(&row.get::<_, String>(2)?)
                    .unwrap_or(GovernanceEvaluationStatus::Archived),
                created_at: row.get(3)?,
                superseded_at: row.get(4)?,
                context_revision: row.get(5)?,
                policy_catalog_revision: row.get(6)?,
                evaluation_count: row.get::<_, i64>(7)? as usize,
                aggregate_result: row.get(8)?,
                authority_effect: row.get(9)?,
                terminal: row.get::<_, i64>(10)? != 0,
                actionable: row.get::<_, i64>(11)? != 0,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn list_meta(&self, workspace_id: &str) -> Result<Vec<PolicyGovernanceMeta>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    context_revision, policy_catalog_revision, evaluation_count, aggregate_result,
                    authority_effect, terminal, actionable
             FROM workspace_policy_governance_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(PolicyGovernanceMeta {
                evaluation_set_id: row.get(0)?,
                workspace_id: row.get(1)?,
                status: GovernanceEvaluationStatus::parse(&row.get::<_, String>(2)?)
                    .unwrap_or(GovernanceEvaluationStatus::Archived),
                created_at: row.get(3)?,
                superseded_at: row.get(4)?,
                context_revision: row.get(5)?,
                policy_catalog_revision: row.get(6)?,
                evaluation_count: row.get::<_, i64>(7)? as usize,
                aggregate_result: row.get(8)?,
                authority_effect: row.get(9)?,
                terminal: row.get::<_, i64>(10)? != 0,
                actionable: row.get::<_, i64>(11)? != 0,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn load_view(&self, meta: &PolicyGovernanceMeta) -> Result<PolicyGovernanceView> {
        let row: (String, String, Option<String>, String) = self.db.connection().query_row(
            "SELECT policies_json, evaluations_json, recommendation_json, unknowns_json
             FROM workspace_policy_governance_snapshots WHERE id = ?1",
            [&meta.evaluation_set_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            },
        )?;
        let policies: Vec<PolicyDefinition> = serde_json::from_str(&row.0).map_err(ser_err)?;
        let evaluations: Vec<PolicyEvaluation> = serde_json::from_str(&row.1).map_err(ser_err)?;
        let recommendation: Option<GovernanceRecommendation> = match row.2 {
            Some(raw) => Some(serde_json::from_str(&raw).map_err(ser_err)?),
            None => None,
        };
        let unknowns: Vec<String> = serde_json::from_str(&row.3).map_err(ser_err)?;
        Ok(PolicyGovernanceView {
            meta: meta.clone(),
            policies,
            evaluations,
            recommendation,
            unknowns,
            context_revision: meta.context_revision.clone(),
            authority_effect: PolicyGovernanceView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn list_history(&self, workspace_id: &str) -> Result<Vec<PolicyGovernanceHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT evaluation_set_id, status, created_at, superseded_at,
                    context_revision, policy_catalog_revision, evaluation_count, aggregate_result,
                    terminal, actionable, authority_effect
             FROM workspace_policy_governance_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(PolicyGovernanceHistoryEntry {
                evaluation_set_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                context_revision: row.get(4)?,
                policy_catalog_revision: row.get(5)?,
                evaluation_count: row.get::<_, i64>(6)? as usize,
                aggregate_result: row.get(7)?,
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
            "SELECT COUNT(*) FROM workspace_policy_governance_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
