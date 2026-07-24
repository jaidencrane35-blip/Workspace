use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AutomationContractId, AutomationIntentDefinition, AutomationIntentProposal,
    AutomationIntentProposalId, AutomationIntentProposalStatus, ProjectId, TaskId, TriggerEvent,
    TriggerEventId, TriggerEventType, TriggerRejection, WorkspaceId,
};

/// Persistence for trigger events and intent proposals (non-executable).
pub struct AutomationTriggerRepository<'a> {
    db: &'a Database,
}

impl<'a> AutomationTriggerRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn insert_event(&self, event: &TriggerEvent) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO trigger_events (
                id, workspace_id, event_type, source, context, project_id, task_id,
                actor_id, actor_type, created_at, authority_effect
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (
                event.id.as_str(),
                event.workspace_id.as_str(),
                event.event_type.as_str(),
                &event.source,
                &event.context,
                event.project_id.as_ref().map(|id| id.as_str().to_string()),
                event.task_id.as_ref().map(|id| id.as_str().to_string()),
                &event.actor_id,
                &event.actor_type,
                &event.created_at,
                &event.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn get_event(&self, id: &TriggerEventId) -> Result<Option<TriggerEvent>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, event_type, source, context, project_id, task_id,
                    actor_id, actor_type, created_at, authority_effect
             FROM trigger_events WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_event(row)?));
        }
        Ok(None)
    }

    pub fn list_events(&self, workspace_id: &str, limit: usize) -> Result<Vec<TriggerEvent>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, event_type, source, context, project_id, task_id,
                    actor_id, actor_type, created_at, authority_effect
             FROM trigger_events
             WHERE workspace_id = ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_event)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn upsert_proposal(&self, proposal: &AutomationIntentProposal) -> Result<()> {
        let caps = serde_json::to_string(&proposal.required_capabilities).unwrap_or_else(|_| {
            "[]".into()
        });
        self.db.connection().execute(
            "INSERT INTO automation_intent_proposals (
                id, contract_id, workspace_id, project_id, task_id, trigger_event_id,
                intent_statement, required_capabilities, status, explanation,
                definition_fingerprint, created_at, updated_at
             ) VALUES (
                :id, :contract_id, :workspace_id, :project_id, :task_id, :trigger_event_id,
                :intent_statement, :required_capabilities, :status, :explanation,
                :definition_fingerprint, :created_at, :updated_at
             )
             ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                explanation = excluded.explanation,
                updated_at = excluded.updated_at",
            rusqlite::named_params! {
                ":id": proposal.id.as_str(),
                ":contract_id": proposal.contract_id.as_str(),
                ":workspace_id": proposal.workspace_id.as_str(),
                ":project_id": proposal.project_id.as_str(),
                ":task_id": proposal.task_id.as_ref().map(|id| id.as_str().to_string()),
                ":trigger_event_id": proposal.trigger_event_id.as_str(),
                ":intent_statement": &proposal.intent_definition.statement,
                ":required_capabilities": caps,
                ":status": proposal.status.as_str(),
                ":explanation": &proposal.explanation,
                ":definition_fingerprint": &proposal.definition_fingerprint,
                ":created_at": &proposal.created_at,
                ":updated_at": &proposal.updated_at,
            },
        )?;
        Ok(())
    }

    pub fn get_proposal(
        &self,
        id: &AutomationIntentProposalId,
    ) -> Result<Option<AutomationIntentProposal>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, contract_id, workspace_id, project_id, task_id, trigger_event_id,
                    intent_statement, required_capabilities, status, explanation,
                    definition_fingerprint, created_at, updated_at
             FROM automation_intent_proposals WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_proposal(row)?));
        }
        Ok(None)
    }

    pub fn insert_rejection(
        &self,
        id: &str,
        event: &TriggerEvent,
        rejection: &TriggerRejection,
        created_at: &str,
    ) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO trigger_evaluation_rejections (
                id, trigger_event_id, workspace_id, contract_id, contract_name, reason, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                id,
                event.id.as_str(),
                event.workspace_id.as_str(),
                &rejection.contract_id,
                &rejection.contract_name,
                &rejection.reason,
                created_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_rejections(
        &self,
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<TriggerRejection>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT contract_id, contract_name, reason
             FROM trigger_evaluation_rejections
             WHERE workspace_id = ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), |row| {
            Ok(TriggerRejection {
                contract_id: row.get(0)?,
                contract_name: row.get(1)?,
                reason: row.get(2)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_proposals(
        &self,
        workspace_id: &str,
        status: Option<&str>,
        limit: usize,
    ) -> Result<Vec<AutomationIntentProposal>> {
        let limit = limit.clamp(1, 200) as i64;
        if let Some(status) = status {
            let mut stmt = self.db.connection().prepare(
                "SELECT id, contract_id, workspace_id, project_id, task_id, trigger_event_id,
                        intent_statement, required_capabilities, status, explanation,
                        definition_fingerprint, created_at, updated_at
                 FROM automation_intent_proposals
                 WHERE workspace_id = ?1 AND status = ?2
                 ORDER BY updated_at DESC
                 LIMIT ?3",
            )?;
            let rows = stmt.query_map((workspace_id, status, limit), map_proposal)?;
            return rows
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into);
        }
        let mut stmt = self.db.connection().prepare(
            "SELECT id, contract_id, workspace_id, project_id, task_id, trigger_event_id,
                    intent_statement, required_capabilities, status, explanation,
                    definition_fingerprint, created_at, updated_at
             FROM automation_intent_proposals
             WHERE workspace_id = ?1
             ORDER BY updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_proposal)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_event(row: &rusqlite::Row<'_>) -> rusqlite::Result<TriggerEvent> {
    Ok(TriggerEvent {
        id: TriggerEventId::new(row.get::<_, String>(0)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        event_type: TriggerEventType::parse(&row.get::<_, String>(2)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        source: row.get(3)?,
        context: row.get(4)?,
        project_id: row
            .get::<_, Option<String>>(5)?
            .map(ProjectId::new)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        task_id: row
            .get::<_, Option<String>>(6)?
            .map(TaskId::new)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        actor_id: row.get(7)?,
        actor_type: row.get(8)?,
        created_at: row.get(9)?,
        authority_effect: row.get(10)?,
    })
}

fn map_proposal(row: &rusqlite::Row<'_>) -> rusqlite::Result<AutomationIntentProposal> {
    let caps_raw: String = row.get(7)?;
    let required_capabilities: Vec<String> =
        serde_json::from_str(&caps_raw).unwrap_or_default();
    Ok(AutomationIntentProposal {
        id: AutomationIntentProposalId::new(row.get::<_, String>(0)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        contract_id: AutomationContractId::new(row.get::<_, String>(1)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(2)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        project_id: ProjectId::new(row.get::<_, String>(3)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        task_id: row
            .get::<_, Option<String>>(4)?
            .map(TaskId::new)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        trigger_event_id: TriggerEventId::new(row.get::<_, String>(5)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        intent_definition: AutomationIntentDefinition {
            statement: row.get(6)?,
        },
        required_capabilities,
        status: AutomationIntentProposalStatus::parse(&row.get::<_, String>(8)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        explanation: row.get(9)?,
        definition_fingerprint: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}
