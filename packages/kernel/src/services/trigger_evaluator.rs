//! Governed Trigger Evaluation (Phase 4 Batch 7).
//!
//! Evaluates whether approved Automation Contracts are relevant for a TriggerEvent.
//! Produces Intent Proposals only — never executes, approves, or grants authority.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;
use workspace_database::{
    AutomationContractRepository, AutomationTriggerRepository, Database, WorkspaceIntentRepository,
};
use workspace_domain::{
    ActorContext, AutomationContract, AutomationContractApprovalState, AutomationContractStatus,
    AutomationIntentProposal, AutomationIntentProposalId, AutomationIntentProposalStatus,
    AutomationIntentProposalSummary, CapabilitySet, IntentContext, TriggerEvaluationResult,
    TriggerEvent, TriggerEventId, TriggerEventType, TriggerRejection, TriggerRejectionSummary,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct TriggerEvaluatorService;

impl TriggerEvaluatorService {
    pub(crate) fn record_event(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        event_type: TriggerEventType,
        source: impl Into<String>,
        context: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<TriggerEvent> {
        let event = TriggerEvent::new(
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
            actor.actor.id.as_str(),
            format!("{:?}", actor.actor.actor_type),
        )
        .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard).insert_event(&event)?;
        }
        Self::audit(
            db,
            actor,
            "automation.trigger.received",
            json!({
                "trigger_event_id": event.id.as_str(),
                "workspace_id": event.workspace_id.as_str(),
                "event_type": event.event_type.as_str(),
                "authority_effect": "none",
            }),
        )?;
        Ok(event)
    }

    /// Evaluate contracts for a trigger event and persist proposals.
    /// Never executes commands or grants capabilities.
    pub(crate) fn evaluate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        granted: &CapabilitySet,
        event_id: impl Into<String>,
    ) -> Result<TriggerEvaluationResult> {
        let event_id = TriggerEventId::new(event_id.into()).map_err(KernelError::Domain)?;
        let event = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard)
                .get_event(&event_id)?
                .ok_or_else(|| KernelError::AutomationTriggerValidation {
                    message: "trigger event not found".into(),
                })?
        };

        let contracts = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationContractRepository::new(&guard)
                .list_by_workspace(event.workspace_id.as_str(), 200)?
        };

        let mut proposals = Vec::new();
        let mut rejections = Vec::new();

        for contract in contracts {
            match Self::validate_candidate(&event, &contract, db, granted) {
                Ok(explanation) => {
                    let proposal =
                        AutomationIntentProposal::from_match(&contract, &event, explanation);
                    {
                        let guard = db.lock().map_err(|_| {
                            KernelError::lock_poisoned("database")
                        })?;
                        AutomationTriggerRepository::new(&guard).upsert_proposal(&proposal)?;
                    }
                    Self::audit(
                        db,
                        actor,
                        "automation.intent_proposal.created",
                        json!({
                            "proposal_id": proposal.id.as_str(),
                            "contract_id": proposal.contract_id.as_str(),
                            "trigger_event_id": event.id.as_str(),
                            "workspace_id": event.workspace_id.as_str(),
                            "authority_effect": "none",
                            "execution_authorized": false,
                        }),
                    )?;
                    proposals.push(proposal);
                }
                Err(reason) => {
                    let rejection = TriggerRejection {
                        contract_id: contract.id.to_string(),
                        contract_name: contract.name.clone(),
                        reason,
                    };
                    {
                        let guard = db.lock().map_err(|_| {
                            KernelError::lock_poisoned("database")
                        })?;
                        AutomationTriggerRepository::new(&guard).insert_rejection(
                            &Uuid::new_v4().to_string(),
                            &event,
                            &rejection,
                            &Utc::now().to_rfc3339(),
                        )?;
                    }
                    rejections.push(rejection);
                }
            }
        }

        Self::audit(
            db,
            actor,
            "automation.trigger.evaluated",
            json!({
                "trigger_event_id": event.id.as_str(),
                "workspace_id": event.workspace_id.as_str(),
                "proposal_count": proposals.len(),
                "rejection_count": rejections.len(),
                "authority_effect": "none",
            }),
        )?;
        if proposals.is_empty() && !rejections.is_empty() {
            Self::audit(
                db,
                actor,
                "automation.trigger.rejected",
                json!({
                    "trigger_event_id": event.id.as_str(),
                    "workspace_id": event.workspace_id.as_str(),
                    "rejection_count": rejections.len(),
                    "authority_effect": "none",
                }),
            )?;
        }

        Ok(TriggerEvaluationResult::new(event, proposals, rejections))
    }

    /// Record event and evaluate in one governed write path (manual evaluation UX).
    pub(crate) fn record_and_evaluate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        granted: &CapabilitySet,
        workspace_id: impl Into<String>,
        event_type: TriggerEventType,
        source: impl Into<String>,
        context: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<TriggerEvaluationResult> {
        let event = Self::record_event(
            db,
            actor,
            workspace_id,
            event_type,
            source,
            context,
            project_id,
            task_id,
        )?;
        Self::evaluate(db, actor, granted, event.id.to_string())
    }

    pub(crate) fn list_events(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        limit: Option<usize>,
    ) -> Result<Vec<TriggerEvent>> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        AutomationTriggerRepository::new(&guard)
            .list_events(&workspace_id, limit.unwrap_or(50))
            .map_err(Into::into)
    }

    pub(crate) fn list_proposals(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        status: Option<AutomationIntentProposalStatus>,
        limit: Option<usize>,
    ) -> Result<Vec<AutomationIntentProposal>> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        AutomationTriggerRepository::new(&guard)
            .list_proposals(
                &workspace_id,
                status.map(|s| s.as_str()),
                limit.unwrap_or(50),
            )
            .map_err(Into::into)
    }

    pub(crate) fn accept_proposal(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        proposal_id: impl Into<String>,
    ) -> Result<AutomationIntentProposal> {
        let proposal_id =
            AutomationIntentProposalId::new(proposal_id.into()).map_err(KernelError::Domain)?;
        let mut proposal = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard)
                .get_proposal(&proposal_id)?
                .ok_or_else(|| KernelError::AutomationTriggerValidation {
                    message: "intent proposal not found".into(),
                })?
        };
        proposal.accept().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard).upsert_proposal(&proposal)?;
        }
        Self::audit(
            db,
            actor,
            "automation.intent_proposal.accepted",
            json!({
                "proposal_id": proposal.id.as_str(),
                "contract_id": proposal.contract_id.as_str(),
                "workspace_id": proposal.workspace_id.as_str(),
                "authority_effect": "none",
                "execution_authorized": false,
                "governance_note": "Accepted proposal still requires Command Pipeline → Permission Gateway before any action.",
            }),
        )?;
        Ok(proposal)
    }

    pub(crate) fn reject_proposal(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        proposal_id: impl Into<String>,
    ) -> Result<AutomationIntentProposal> {
        let proposal_id =
            AutomationIntentProposalId::new(proposal_id.into()).map_err(KernelError::Domain)?;
        let mut proposal = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard)
                .get_proposal(&proposal_id)?
                .ok_or_else(|| KernelError::AutomationTriggerValidation {
                    message: "intent proposal not found".into(),
                })?
        };
        proposal.reject().map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            AutomationTriggerRepository::new(&guard).upsert_proposal(&proposal)?;
        }
        Self::audit(
            db,
            actor,
            "automation.intent_proposal.rejected",
            json!({
                "proposal_id": proposal.id.as_str(),
                "contract_id": proposal.contract_id.as_str(),
                "workspace_id": proposal.workspace_id.as_str(),
                "authority_effect": "none",
            }),
        )?;
        Ok(proposal)
    }

    pub(crate) fn pending_summaries(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<AutomationIntentProposalSummary>> {
        Ok(Self::list_proposals(
            db,
            workspace_id,
            Some(AutomationIntentProposalStatus::PendingReview),
            Some(limit),
        )?
        .iter()
        .map(AutomationIntentProposalSummary::from)
        .collect())
    }

    /// Validation pipeline for one candidate contract.
    fn validate_candidate(
        event: &TriggerEvent,
        contract: &AutomationContract,
        db: &Arc<Mutex<Database>>,
        granted: &CapabilitySet,
    ) -> std::result::Result<String, String> {
        if contract.deleted {
            return Err("Contract deleted.".into());
        }
        if contract.workspace_id.as_str() != event.workspace_id.as_str() {
            return Err("Workspace scope mismatch.".into());
        }
        if contract.status == AutomationContractStatus::Paused {
            return Err("Contract paused.".into());
        }
        if contract.status == AutomationContractStatus::Revoked {
            return Err("Contract revoked.".into());
        }
        if contract.status == AutomationContractStatus::Completed {
            return Err("Contract completed.".into());
        }
        if contract.status != AutomationContractStatus::Approved
            || contract.approval_state != AutomationContractApprovalState::Approved
        {
            return Err("Contract is not an approved definition.".into());
        }
        if !contract.approval_matches_current_definition() {
            return Err("Approval fingerprint no longer matches.".into());
        }
        if !event
            .event_type
            .matches_trigger_kind(contract.trigger_definition.kind)
        {
            return Err(format!(
                "Trigger kind '{}' is not compatible with event '{}'.",
                contract.trigger_definition.kind.as_str(),
                event.event_type.as_str()
            ));
        }

        // Project / task scope
        if let Some(ref event_project) = event.project_id {
            if event_project.as_str() != contract.project_id.as_str() {
                return Err("Project scope mismatch.".into());
            }
        }
        if let Some(ref contract_task) = contract.task_id {
            if let Some(ref event_task) = event.task_id {
                if event_task.as_str() != contract_task.as_str() {
                    return Err("Task scope mismatch.".into());
                }
            }
        }

        // Ensure project still exists in workspace.
        {
            let guard = db
                .lock()
                .map_err(|_| "database lock poisoned".to_string())?;
            let project = WorkspaceIntentRepository::new(&guard)
                .get_project(&contract.project_id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "Project not found for contract.".to_string())?;
            if project.deleted || project.workspace_id.as_str() != event.workspace_id.as_str() {
                return Err("Project does not belong to this workspace.".into());
            }
        }

        // Capability compatibility (informational — does not grant).
        for cap in &contract.required_capabilities {
            if !granted.iter().any(|g| g.as_str() == cap.as_str()) {
                // Local user standard may not include application.launch in CapabilitySet
                // the same way — check by string against granted ids.
                // If actor is local user evaluating, we still surface a soft note only when
                // the capability id is completely unknown to the actor set.
                // For integrity: reject when the evaluating actor lacks the capability string.
                return Err(format!("Required capability unavailable: {cap}."));
            }
        }

        Ok(format!(
            "Contract \"{}\" matched event '{}' because it is approved, fingerprint-valid, \
             scope-aligned, and trigger-compatible. This is a proposal only — not execution.",
            contract.name,
            event.event_type.as_str()
        ))
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event_type: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
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

pub(crate) fn list_rejection_summaries(
    db: &Arc<Mutex<Database>>,
    workspace_id: &str,
    limit: usize,
) -> Result<Vec<TriggerRejectionSummary>> {
    let guard = db
        .lock()
        .map_err(|_| KernelError::lock_poisoned("database"))?;
    Ok(AutomationTriggerRepository::new(&guard)
        .list_rejections(workspace_id, limit)?
        .into_iter()
        .map(|r| TriggerRejectionSummary {
            contract_id: r.contract_id,
            reason: r.reason,
        })
        .collect())
}
