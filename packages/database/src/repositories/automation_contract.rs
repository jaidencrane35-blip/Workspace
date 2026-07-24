use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    AutomationContract, AutomationContractApprovalState, AutomationContractId,
    AutomationContractScope, AutomationContractStatus, AutomationIntentDefinition,
    AutomationTriggerDefinition, AutomationTriggerKind, ProjectId, TaskId, WorkspaceId,
};

/// Persistence for governed automation contracts (definition only).
pub struct AutomationContractRepository<'a> {
    db: &'a Database,
}

impl<'a> AutomationContractRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert(&self, contract: &AutomationContract) -> Result<()> {
        let caps = serde_json::to_string(&contract.required_capabilities).unwrap_or_else(|_| {
            "[]".into()
        });
        let task_id = contract
            .task_id
            .as_ref()
            .map(|id| id.as_str().to_string());
        let deleted = if contract.deleted { 1_i64 } else { 0 };
        self.db.connection().execute(
            "INSERT INTO automation_contracts (
                id, workspace_id, project_id, task_id, name, description, status,
                trigger_kind, trigger_definition, intent_statement, scope,
                required_capabilities, approval_state, created_by_actor,
                approved_by_actor, approved_at, approved_definition_fingerprint,
                created_at, updated_at, deleted
             ) VALUES (
                :id, :workspace_id, :project_id, :task_id, :name, :description, :status,
                :trigger_kind, :trigger_definition, :intent_statement, :scope,
                :required_capabilities, :approval_state, :created_by_actor,
                :approved_by_actor, :approved_at, :approved_definition_fingerprint,
                :created_at, :updated_at, :deleted
             )
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                trigger_kind = excluded.trigger_kind,
                trigger_definition = excluded.trigger_definition,
                intent_statement = excluded.intent_statement,
                scope = excluded.scope,
                required_capabilities = excluded.required_capabilities,
                approval_state = excluded.approval_state,
                approved_by_actor = excluded.approved_by_actor,
                approved_at = excluded.approved_at,
                approved_definition_fingerprint = excluded.approved_definition_fingerprint,
                updated_at = excluded.updated_at,
                deleted = excluded.deleted",
            rusqlite::named_params! {
                ":id": contract.id.as_str(),
                ":workspace_id": contract.workspace_id.as_str(),
                ":project_id": contract.project_id.as_str(),
                ":task_id": task_id,
                ":name": &contract.name,
                ":description": &contract.description,
                ":status": contract.status.as_str(),
                ":trigger_kind": contract.trigger_definition.kind.as_str(),
                ":trigger_definition": &contract.trigger_definition.definition,
                ":intent_statement": &contract.intent_definition.statement,
                ":scope": contract.scope.as_str(),
                ":required_capabilities": caps,
                ":approval_state": contract.approval_state.as_str(),
                ":created_by_actor": &contract.created_by_actor,
                ":approved_by_actor": &contract.approved_by_actor,
                ":approved_at": &contract.approved_at,
                ":approved_definition_fingerprint": &contract.approved_definition_fingerprint,
                ":created_at": &contract.created_at,
                ":updated_at": &contract.updated_at,
                ":deleted": deleted,
            },
        )?;
        Ok(())
    }

    pub fn get(&self, id: &AutomationContractId) -> Result<Option<AutomationContract>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, task_id, name, description, status,
                    trigger_kind, trigger_definition, intent_statement, scope,
                    required_capabilities, approval_state, created_by_actor,
                    approved_by_actor, approved_at, approved_definition_fingerprint,
                    created_at, updated_at, deleted
             FROM automation_contracts WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_contract(row)?));
        }
        Ok(None)
    }

    pub fn list_by_workspace(
        &self,
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<AutomationContract>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, task_id, name, description, status,
                    trigger_kind, trigger_definition, intent_statement, scope,
                    required_capabilities, approval_state, created_by_actor,
                    approved_by_actor, approved_at, approved_definition_fingerprint,
                    created_at, updated_at, deleted
             FROM automation_contracts
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_contract)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn soft_delete_by_project(&self, project_id: &str, updated_at: &str) -> Result<usize> {
        let changed = self.db.connection().execute(
            "UPDATE automation_contracts
             SET deleted = 1, updated_at = ?1
             WHERE project_id = ?2 AND deleted = 0",
            (updated_at, project_id),
        )?;
        Ok(changed)
    }
}

fn map_contract(row: &rusqlite::Row<'_>) -> rusqlite::Result<AutomationContract> {
    let id = AutomationContractId::new(row.get::<_, String>(0)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let workspace_id = WorkspaceId::new(row.get::<_, String>(1)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let project_id = ProjectId::new(row.get::<_, String>(2)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let task_id = row
        .get::<_, Option<String>>(3)?
        .map(TaskId::new)
        .transpose()
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let status = AutomationContractStatus::parse(&row.get::<_, String>(6)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let trigger_kind = AutomationTriggerKind::parse(&row.get::<_, String>(7)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let scope = AutomationContractScope::parse(&row.get::<_, String>(10)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let caps_raw: String = row.get(11)?;
    let required_capabilities: Vec<String> =
        serde_json::from_str(&caps_raw).unwrap_or_default();
    let approval_state = AutomationContractApprovalState::parse(&row.get::<_, String>(12)?)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    Ok(AutomationContract {
        id,
        workspace_id,
        project_id,
        task_id,
        name: row.get(4)?,
        description: row.get(5)?,
        status,
        trigger_definition: AutomationTriggerDefinition {
            kind: trigger_kind,
            definition: row.get(8)?,
        },
        intent_definition: AutomationIntentDefinition {
            statement: row.get(9)?,
        },
        scope,
        required_capabilities,
        approval_state,
        created_by_actor: row.get(13)?,
        approved_by_actor: row.get(14)?,
        approved_at: row.get(15)?,
        approved_definition_fingerprint: row.get(16)?,
        created_at: row.get(17)?,
        updated_at: row.get(18)?,
        deleted: row.get::<_, i64>(19)? != 0,
    })
}
