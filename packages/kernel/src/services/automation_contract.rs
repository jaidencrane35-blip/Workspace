//! Governed Automation Contract service (Phase 4 Batch 6 / 6.5).
//!
//! Persists contract definitions and approval lifecycle.
//! Never executes commands, grants capabilities, or runs triggers.
//! Batch 6.5: material edits invalidate consent; fingerprints bind approval.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{AutomationContractRepository, Database, WorkspaceIntentRepository};
use workspace_domain::{
    ActorContext, AutomationContract, AutomationContractApprovalState, AutomationContractId,
    AutomationContractIntentRequest, AutomationContractStatus, AutomationIntentDefinition,
    AutomationTriggerDefinition, AutomationTriggerKind, IntentContext, ProjectId, TaskId,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct AutomationContractService;

impl AutomationContractService {
    pub(crate) fn create(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        project_id: impl Into<String>,
        task_id: Option<String>,
        name: impl Into<String>,
        description: Option<String>,
        trigger_kind: AutomationTriggerKind,
        trigger_definition: Option<String>,
        intent_statement: impl Into<String>,
        required_capabilities: Vec<String>,
    ) -> Result<AutomationContract> {
        let workspace_id = workspace_id.into();
        let project_id = project_id.into();
        Self::ensure_project_belongs(db, &workspace_id, &project_id)?;
        if let Some(ref task_id) = task_id {
            Self::ensure_task_belongs(db, &workspace_id, &project_id, task_id)?;
        }

        let trigger = AutomationTriggerDefinition::new(
            trigger_kind,
            trigger_definition.unwrap_or_else(|| match trigger_kind {
                AutomationTriggerKind::Manual => {
                    "Manual trigger — user initiates when ready.".into()
                }
                AutomationTriggerKind::Scheduled => {
                    "Scheduled trigger definition (not executed automatically).".into()
                }
                AutomationTriggerKind::Event => {
                    "Event trigger definition (not executed automatically).".into()
                }
                AutomationTriggerKind::Pattern => {
                    "Pattern trigger definition (not executed automatically).".into()
                }
            }),
        )
        .map_err(KernelError::from)?;
        let intent = AutomationIntentDefinition::new(intent_statement).map_err(KernelError::from)?;
        let contract = AutomationContract::new(
            workspace_id,
            project_id,
            task_id,
            name,
            description,
            trigger,
            intent,
            required_capabilities,
            actor.actor.id.as_str(),
        )
        .map_err(KernelError::from)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.created",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
                "definition_fingerprint": contract.definition_fingerprint(),
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn update(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
        name: Option<String>,
        description: Option<Option<String>>,
        intent_statement: Option<String>,
        trigger_kind: Option<AutomationTriggerKind>,
        trigger_definition: Option<String>,
        required_capabilities: Option<Vec<String>>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        if contract.deleted
            || matches!(
                contract.status,
                AutomationContractStatus::Revoked | AutomationContractStatus::Completed
            )
        {
            return Err(KernelError::AutomationContractValidation {
                message: "revoked or completed contracts cannot be updated".into(),
            });
        }
        Self::ensure_project_belongs(
            db,
            contract.workspace_id.as_str(),
            contract.project_id.as_str(),
        )?;

        let fingerprint_before = contract.definition_fingerprint();
        let had_consent = matches!(
            contract.approval_state,
            AutomationContractApprovalState::Pending | AutomationContractApprovalState::Approved
        );

        if let Some(name) = name {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err(KernelError::AutomationContractValidation {
                    message: "automation contract name must not be empty".into(),
                });
            }
            contract.name = trimmed.to_string();
        }
        if let Some(description) = description {
            contract.description = description.and_then(|v| {
                let t = v.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            });
        }
        if let Some(statement) = intent_statement {
            contract.intent_definition =
                AutomationIntentDefinition::new(statement).map_err(KernelError::from)?;
        }
        if let Some(kind) = trigger_kind {
            let definition = trigger_definition
                .unwrap_or_else(|| contract.trigger_definition.definition.clone());
            contract.trigger_definition =
                AutomationTriggerDefinition::new(kind, definition).map_err(KernelError::from)?;
        } else if let Some(definition) = trigger_definition {
            contract.trigger_definition = AutomationTriggerDefinition::new(
                contract.trigger_definition.kind,
                definition,
            )
            .map_err(KernelError::from)?;
        }
        if let Some(caps) = required_capabilities {
            contract.required_capabilities = caps
                .into_iter()
                .map(|c| c.trim().to_string())
                .filter(|c| !c.is_empty())
                .collect();
        }

        let material_changed = fingerprint_before != contract.definition_fingerprint();
        let mut approval_invalidated = false;
        if material_changed && had_consent {
            contract.invalidate_approval();
            approval_invalidated = true;
        } else {
            contract.touch();
        }

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.updated",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
                "definition_fingerprint": contract.definition_fingerprint(),
                "material_definition_changed": material_changed,
                "approval_invalidated": approval_invalidated,
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn request_approval(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        Self::ensure_project_belongs(
            db,
            contract.workspace_id.as_str(),
            contract.project_id.as_str(),
        )?;
        contract.request_approval().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.approval.requested",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
                "definition_fingerprint": contract.definition_fingerprint(),
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn approve(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        Self::ensure_project_belongs(
            db,
            contract.workspace_id.as_str(),
            contract.project_id.as_str(),
        )?;
        contract
            .approve_definition(actor.actor.id.as_str())
            .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.approved",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
                "approved_by_actor": contract.approved_by_actor,
                "definition_fingerprint": contract.approved_definition_fingerprint,
                "execution_authorized": false,
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn pause(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        contract.pause().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.paused",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn resume(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        contract.resume().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.updated",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
                "resumed": true,
                "definition_fingerprint": contract.definition_fingerprint(),
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn revoke(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let mut contract = Self::get(db, contract_id)?;
        contract.revoke().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard).upsert(&contract)?;
        }
        Self::audit(
            db,
            actor,
            "automation.contract.revoked",
            &contract,
            json!({
                "status": contract.status.as_str(),
                "approval_state": contract.approval_state.as_str(),
            }),
        )?;
        Ok(contract)
    }

    pub(crate) fn get(
        db: &Arc<Mutex<Database>>,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContract> {
        let contract_id =
            AutomationContractId::new(contract_id.into()).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        AutomationContractRepository::new(&guard)
            .get(&contract_id)?
            .ok_or_else(|| KernelError::AutomationContractValidation {
                message: "automation contract not found".into(),
            })
    }

    pub(crate) fn list(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        limit: Option<usize>,
    ) -> Result<Vec<AutomationContract>> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let contracts = AutomationContractRepository::new(&guard)
            .list_by_workspace(&workspace_id, limit.unwrap_or(50))?;
        let repo = WorkspaceIntentRepository::new(&guard);
        let mut live = Vec::new();
        for contract in contracts {
            match repo.get_project(&contract.project_id)? {
                Some(project)
                    if !project.deleted && project.workspace_id.as_str() == workspace_id =>
                {
                    live.push(contract);
                }
                Some(project) if project.deleted => {
                    let _ = AutomationContractRepository::new(&guard).soft_delete_by_project(
                        contract.project_id.as_str(),
                        &Utc::now().to_rfc3339(),
                    );
                }
                _ => {}
            }
        }
        Ok(live)
    }

    /// Integration boundary: materialize a future Intent request template.
    /// Never executes. Revoked/paused/stale contracts are rejected.
    pub(crate) fn prepare_intent_request(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        contract_id: impl Into<String>,
    ) -> Result<AutomationContractIntentRequest> {
        let mut contract = Self::get(db, contract_id)?;
        Self::ensure_project_belongs(
            db,
            contract.workspace_id.as_str(),
            contract.project_id.as_str(),
        )?;

        // Heal stale approved/paused rows that lost fingerprint match.
        if matches!(
            contract.approval_state,
            AutomationContractApprovalState::Approved
        ) && !contract.approval_matches_current_definition()
        {
            contract.invalidate_approval();
            {
                let guard = db
                    .lock()
                    .map_err(|_| KernelError::lock_poisoned("database"))?;
                AutomationContractRepository::new(&guard).upsert(&contract)?;
            }
            Self::audit(
                db,
                actor,
                "automation.contract.updated",
                &contract,
                json!({
                    "approval_invalidated": true,
                    "reason": "stale_definition_fingerprint",
                }),
            )?;
            return Err(KernelError::AutomationContractValidation {
                message: "automation contract approval is stale for the current definition".into(),
            });
        }

        let prepared = AutomationContractIntentRequest::from_active_contract(
            &contract,
            actor.actor.id.as_str(),
        )
        .map_err(KernelError::from)?;

        Self::audit(
            db,
            actor,
            "automation.contract.intent.prepared",
            &contract,
            json!({
                "definition_fingerprint": prepared.definition_fingerprint,
                "requesting_actor_id": prepared.requesting_actor_id,
                "execution_authorized": false,
            }),
        )?;
        Ok(prepared)
    }

    fn ensure_project_belongs(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        project_id: &str,
    ) -> Result<()> {
        let project_id = ProjectId::new(project_id).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let project = WorkspaceIntentRepository::new(&guard)
            .get_project(&project_id)?
            .ok_or_else(|| KernelError::AutomationContractValidation {
                message: "project not found for automation contract".into(),
            })?;
        if project.deleted || project.workspace_id.as_str() != workspace_id {
            return Err(KernelError::AutomationContractValidation {
                message: "project does not belong to this workspace".into(),
            });
        }
        Ok(())
    }

    fn ensure_task_belongs(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        project_id: &str,
        task_id: &str,
    ) -> Result<()> {
        let task_id = TaskId::new(task_id).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let task = WorkspaceIntentRepository::new(&guard)
            .get_task(&task_id)?
            .ok_or_else(|| KernelError::AutomationContractValidation {
                message: "task not found for automation contract".into(),
            })?;
        if task.deleted
            || task.workspace_id.as_str() != workspace_id
            || task.project_id.as_str() != project_id
        {
            return Err(KernelError::AutomationContractValidation {
                message: "task does not belong to this project/workspace".into(),
            });
        }
        Ok(())
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event_type: &str,
        contract: &AutomationContract,
        extra: serde_json::Value,
    ) -> Result<()> {
        let mut metadata = json!({
            "contract_id": contract.id.as_str(),
            "workspace_id": contract.workspace_id.as_str(),
            "project_id": contract.project_id.as_str(),
            "authority_effect": "none",
        });
        if let Some(obj) = metadata.as_object_mut() {
            if let Some(extra_obj) = extra.as_object() {
                for (k, v) in extra_obj {
                    obj.insert(k.clone(), v.clone());
                }
            }
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event_type,
            true,
            metadata.to_string(),
        )
    }
}
