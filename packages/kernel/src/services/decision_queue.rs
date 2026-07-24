//! Governed Decision Queue (Phase 4 Batch 8).
//!
//! Aggregates pending human decisions from existing sources.
//! Lifecycle overlay is presentation-only — sources remain authoritative.
//! Never executes, never grants permissions, never becomes a second source of truth.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{
    ApplicationRepository, Database, DecisionQueueRepository,
};
use workspace_domain::{
    ActorContext, AiOrchestratedPlan, AiOrchestratedPlanState, AiPlanStepState,
    AutomationContractApprovalState, AutomationContractStatus, AutomationIntentProposalStatus,
    DecisionActionResult, DecisionCategory, DecisionHandoff, DecisionItem,
    DecisionLifecycleOverlay, DecisionPriority, DecisionQueue, DecisionQueueError,
    DecisionSourceType, DecisionState, IntentContext, PermissionApprovalRequest,
    PermissionApprovalStatus, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, AutomationContractService, OrchestratedPlanStore,
    PermissionApprovalService, TriggerEvaluatorService, WorkspaceIntentService,
};

pub(crate) struct DecisionQueueService;

impl DecisionQueueService {
    /// Aggregate live sources + lifecycle overlay into one ordered queue.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<DecisionQueue> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let ws = workspace_id.as_str();

        let workflow_context = WorkspaceIntentService::get_workflow_context_readonly(db, ws)?;
        let active_project = workflow_context
            .active_project_id
            .as_ref()
            .map(|id| id.to_string());

        let applications = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            ApplicationRepository::new(&guard)
                .list_by_workspace(&workspace_id)?
        };
        let app_ids: HashSet<String> = applications.iter().map(|a| a.id.to_string()).collect();

        let plans = {
            let guard = orchestrated_plans
                .lock()
                .map_err(|_| KernelError::Config("orchestrated plan store lock poisoned".into()))?;
            guard.list_all()
        };
        let workflows = {
            let guard = assistant_workflows.lock().map_err(|_| {
                KernelError::Config("assistant workflow store lock poisoned".into())
            })?;
            guard.list_all()
        };

        let linked_plan_ids: HashSet<String> = workflow_context
            .related_plan_ids
            .iter()
            .cloned()
            .chain(
                workflows
                    .iter()
                    .filter(|w| w.workspace_id.as_deref() == Some(ws))
                    .filter_map(|w| w.orchestrated_plan_id.as_ref().map(|id| id.to_string())),
            )
            .collect();

        let scoped_plans: Vec<&AiOrchestratedPlan> = plans
            .iter()
            .filter(|plan| plan_belongs_to_workspace(plan, &app_ids, &linked_plan_ids))
            .collect();
        let linked_approval_ids: HashSet<String> = scoped_plans
            .iter()
            .flat_map(|plan| plan.steps.iter())
            .filter_map(|step| step.approval_request_id.clone())
            .collect();

        let mut items = Vec::new();
        items.extend(Self::adapt_permission_approvals(
            db,
            ws,
            &app_ids,
            &linked_approval_ids,
            active_project.as_deref(),
        )?);
        items.extend(Self::adapt_contract_definition_approvals(
            db,
            ws,
            active_project.as_deref(),
        )?);
        items.extend(Self::adapt_intent_proposals(
            db,
            ws,
            active_project.as_deref(),
        )?);
        items.extend(Self::adapt_blocked_actions(
            &scoped_plans,
            ws,
            active_project.as_deref(),
        )?);
        items.extend(Self::adapt_planning_continuations(
            &scoped_plans,
            &workflows,
            ws,
            active_project.as_deref(),
        )?);

        let overlays = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            DecisionQueueRepository::new(&guard).list_overlays(ws)?
        };
        let overlay_map: HashMap<(String, String), DecisionLifecycleOverlay> = overlays
            .into_iter()
            .map(|o| ((o.source_type.as_str().to_string(), o.source_id.clone()), o))
            .collect();

        let live_keys: HashSet<(String, String)> = items
            .iter()
            .map(|i| (i.source_type.as_str().to_string(), i.source_id.clone()))
            .collect();

        for item in &mut items {
            let key = (
                item.source_type.as_str().to_string(),
                item.source_id.clone(),
            );
            if let Some(overlay) = overlay_map.get(&key) {
                item.apply_overlay(overlay.decision_state);
            } else {
                Self::audit(
                    db,
                    actor,
                    "decision.item.created",
                    json!({
                        "decision_item_id": item.id.as_str(),
                        "workspace_id": ws,
                        "source_type": item.source_type.as_str(),
                        "source_id": item.source_id,
                        "authority_effect": "none",
                    }),
                )?;
                Self::write_overlay(
                    db,
                    actor,
                    ws,
                    item.source_type,
                    &item.source_id,
                    DecisionState::Pending,
                )?;
            }
        }

        // Drop stale overlays whose sources no longer exist (aggregation wins).
        for ((source_type, source_id), overlay) in &overlay_map {
            if !live_keys.contains(&(source_type.clone(), source_id.clone())) {
                let Ok(source_type) = DecisionSourceType::parse(source_type) else {
                    continue;
                };
                let guard = db
                    .lock()
                    .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
                DecisionQueueRepository::new(&guard).delete_overlay(ws, source_type, source_id)?;
                let _ = overlay;
            }
        }

        let queue = DecisionQueue::from_items(ws, items);
        Self::audit(
            db,
            actor,
            "decision.queue.generated",
            json!({
                "workspace_id": ws,
                "item_count": queue.items.len(),
                "pending_count": queue.pending_count,
                "high_priority_count": queue.high_priority_count,
                "authority_effect": "none",
            }),
        )?;
        Ok(queue)
    }

    pub(crate) fn mark_viewed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionItem> {
        Self::apply_overlay_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            decision_item_id,
            DecisionState::Viewed,
            "decision.item.viewed",
        )
    }

    pub(crate) fn defer(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionItem> {
        Self::apply_overlay_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            decision_item_id,
            DecisionState::Deferred,
            "decision.item.deferred",
        )
    }

    pub(crate) fn dismiss(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionItem> {
        // Overlay only — never mutates underlying source.
        Self::apply_overlay_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            decision_item_id,
            DecisionState::Dismissed,
            "decision.item.dismissed",
        )
    }

    /// Accept delegates to source subsystems where safe; otherwise returns handoff.
    pub(crate) fn accept(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionActionResult> {
        let workspace_id = workspace_id.into();
        let item = Self::find_item(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            &workspace_id,
            decision_item_id,
        )?;
        match item.source_type {
            DecisionSourceType::IntentProposal => {
                TriggerEvaluatorService::accept_proposal(db, actor, item.source_id.clone())?;
                let mut updated = item;
                updated.decision_state = DecisionState::Accepted;
                Ok(DecisionActionResult {
                    item: Some(updated),
                    handoff: None,
                    delegated: true,
                    authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
                })
            }
            DecisionSourceType::PendingApproval => {
                let next = item
                    .handoff_command
                    .clone()
                    .unwrap_or_else(|| "decide_approval".into());
                Ok(DecisionActionResult {
                    handoff: Some(DecisionHandoff::new(
                        &item,
                        next,
                        "Decision Queue cannot approve permissions. Use the Permission Gateway decide path.",
                    )),
                    item: Some(item),
                    delegated: false,
                    authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
                })
            }
            DecisionSourceType::BlockedAction | DecisionSourceType::PlanningContinuation => {
                let next = item
                    .handoff_command
                    .clone()
                    .unwrap_or_else(|| "resume_orchestrated_ai_plan".into());
                Ok(DecisionActionResult {
                    handoff: Some(DecisionHandoff::new(
                        &item,
                        next,
                        "Continue via the owning planning/assistant subsystem. Queue does not execute.",
                    )),
                    item: Some(item),
                    delegated: false,
                    authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
                })
            }
        }
    }

    pub(crate) fn reject(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionActionResult> {
        let workspace_id = workspace_id.into();
        let item = Self::find_item(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            &workspace_id,
            decision_item_id,
        )?;
        match item.source_type {
            DecisionSourceType::IntentProposal => {
                TriggerEvaluatorService::reject_proposal(db, actor, item.source_id.clone())?;
                let mut updated = item;
                updated.decision_state = DecisionState::Rejected;
                Ok(DecisionActionResult {
                    item: Some(updated),
                    handoff: None,
                    delegated: true,
                    authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
                })
            }
            DecisionSourceType::PendingApproval => Ok(DecisionActionResult {
                handoff: Some(DecisionHandoff::new(
                    &item,
                    "decide_approval",
                    "Decision Queue cannot deny permissions. Use decide_approval with an explicit Deny.",
                )),
                item: Some(item),
                delegated: false,
                authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
            }),
            _ => {
                let dismissed = Self::dismiss(
                    db,
                    actor,
                    orchestrated_plans,
                    assistant_workflows,
                    workspace_id,
                    item.id.to_string(),
                )?;
                Ok(DecisionActionResult {
                    item: Some(dismissed),
                    handoff: None,
                    delegated: false,
                    authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
                })
            }
        }
    }

    fn apply_overlay_transition(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_item_id: impl Into<String>,
        state: DecisionState,
        audit_event: &str,
    ) -> Result<DecisionItem> {
        let workspace_id = workspace_id.into();
        let mut item = Self::find_item(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            &workspace_id,
            decision_item_id,
        )?;
        if matches!(
            item.decision_state,
            DecisionState::Accepted | DecisionState::Rejected | DecisionState::Expired
        ) {
            return Err(KernelError::DecisionQueueValidation {
                message: DecisionQueueError::InvalidTransition {
                    from: item.decision_state.as_str().into(),
                    to: state.as_str().into(),
                }
                .to_string(),
            });
        }
        Self::write_overlay(
            db,
            actor,
            item.workspace_id.as_str(),
            item.source_type,
            &item.source_id,
            state,
        )?;
        item.decision_state = state;
        Self::audit(
            db,
            actor,
            audit_event,
            json!({
                "decision_item_id": item.id.as_str(),
                "workspace_id": item.workspace_id.as_str(),
                "source_type": item.source_type.as_str(),
                "source_id": item.source_id,
                "decision_state": state.as_str(),
                "authority_effect": "none",
            }),
        )?;
        Ok(item)
    }

    fn find_item(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: &str,
        decision_item_id: impl Into<String>,
    ) -> Result<DecisionItem> {
        let decision_item_id = decision_item_id.into();
        let queue = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
        )?;
        queue
            .items
            .into_iter()
            .find(|i| i.id.as_str() == decision_item_id)
            .ok_or_else(|| KernelError::DecisionQueueValidation {
                message: "decision item not found".into(),
            })
    }

    fn write_overlay(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: &str,
        source_type: DecisionSourceType,
        source_id: &str,
        decision_state: DecisionState,
    ) -> Result<()> {
        let overlay = DecisionLifecycleOverlay {
            workspace_id: workspace_id.into(),
            source_type,
            source_id: source_id.into(),
            decision_state,
            updated_at: Utc::now().to_rfc3339(),
            actor_id: actor.actor.id.to_string(),
        };
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        DecisionQueueRepository::new(&guard)
            .upsert_overlay(&overlay)
            .map_err(Into::into)
    }

    fn adapt_permission_approvals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        app_ids: &HashSet<String>,
        linked_approval_ids: &HashSet<String>,
        active_project: Option<&str>,
    ) -> Result<Vec<DecisionItem>> {
        let approvals = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            PermissionApprovalService::list_recent(&guard, Some(50))?
        };
        let mut items = Vec::new();
        for item in approvals
            .into_iter()
            .filter(|a| a.status == PermissionApprovalStatus::Pending)
            .filter(|a| approval_belongs_to_workspace(a, workspace_id, app_ids, linked_approval_ids))
        {
            let priority = if active_project.is_some() {
                DecisionPriority::High
            } else {
                DecisionPriority::Critical
            };
            items.push(DecisionItem::aggregate(
                workspace_id,
                DecisionSourceType::PendingApproval,
                item.id.to_string(),
                DecisionCategory::Permission,
                format!("{} requires a decision", item.command_name),
                format!("{} requires approval", item.command_name),
                format!(
                    "Waiting because permission approval is required for capability {}.",
                    item.capability
                ),
                "Decide allow once or deny via Permission Gateway (decide_approval).",
                priority,
                item.created_at.clone(),
                item.requesting_actor_id.clone(),
                format!("{:?}", item.requesting_actor_type),
                None,
                vec![item.capability.clone()],
                Some("decide_approval".into()),
            )?);
        }
        Ok(items)
    }

    fn adapt_contract_definition_approvals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        active_project: Option<&str>,
    ) -> Result<Vec<DecisionItem>> {
        let contracts = AutomationContractService::list(db, workspace_id, Some(50))?;
        let mut items = Vec::new();
        for contract in contracts.into_iter().filter(|c| {
            !c.deleted
                && (c.status == AutomationContractStatus::PendingApproval
                    || c.approval_state == AutomationContractApprovalState::Pending)
        }) {
            let priority = if active_project == Some(contract.project_id.as_str()) {
                DecisionPriority::High
            } else {
                DecisionPriority::Normal
            };
            items.push(DecisionItem::aggregate(
                workspace_id,
                DecisionSourceType::PendingApproval,
                format!("contract:{}", contract.id),
                DecisionCategory::ContractDefinition,
                format!("Approve automation contract \"{}\"", contract.name),
                String::from("Automation contract definition awaits approval"),
                format!(
                    "Definition approval is consent for this exact intent — not execution. Intent: {}",
                    contract.intent_definition.statement
                ),
                String::from(
                    "Approve or revoke the contract definition via existing contract commands.",
                ),
                priority,
                contract.created_at.clone(),
                contract.created_by_actor.clone(),
                String::from("LocalUser"),
                Some(contract.project_id.to_string()),
                contract.required_capabilities.clone(),
                Some(String::from("approve_automation_contract")),
            )?);
        }
        Ok(items)
    }

    fn adapt_intent_proposals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        active_project: Option<&str>,
    ) -> Result<Vec<DecisionItem>> {
        let proposals = TriggerEvaluatorService::list_proposals(
            db,
            workspace_id,
            Some(AutomationIntentProposalStatus::PendingReview),
            Some(50),
        )?;
        let mut items = Vec::new();
        for proposal in proposals {
            let priority = if active_project == Some(proposal.project_id.as_str()) {
                DecisionPriority::High
            } else {
                DecisionPriority::Normal
            };
            items.push(DecisionItem::aggregate(
                workspace_id,
                DecisionSourceType::IntentProposal,
                proposal.id.to_string(),
                DecisionCategory::Automation,
                String::from("Automation intent proposal needs review"),
                proposal.intent_definition.statement.clone(),
                proposal.explanation.clone(),
                String::from(
                    "Accept for review or reject. Accepting still requires prepare → pipeline → gateway.",
                ),
                priority,
                proposal.created_at.clone(),
                String::from("system"),
                String::from("System"),
                Some(proposal.project_id.to_string()),
                proposal.required_capabilities.clone(),
                Some(String::from("accept_automation_intent_proposal")),
            )?);
        }
        Ok(items)
    }

    fn adapt_blocked_actions(
        scoped_plans: &[&AiOrchestratedPlan],
        workspace_id: &str,
        _active_project: Option<&str>,
    ) -> Result<Vec<DecisionItem>> {
        let mut items = Vec::new();
        for plan in scoped_plans {
            for step in &plan.steps {
                if !matches!(
                    step.state,
                    AiPlanStepState::Denied | AiPlanStepState::Failed
                ) {
                    continue;
                }
                items.push(DecisionItem::aggregate(
                    workspace_id,
                    DecisionSourceType::BlockedAction,
                    step.id.to_string(),
                    DecisionCategory::Blocked,
                    format!("{} is blocked", step.proposal.command_name),
                    format!(
                        "{} blocked ({})",
                        step.proposal.command_name,
                        step.state.as_str()
                    ),
                    step.last_outcome_detail.clone().unwrap_or_else(|| {
                        "Blocked because Permission Gateway denied or the step failed.".into()
                    }),
                    String::from(
                        "Review the blocked step. Retry only through the governed planning path.",
                    ),
                    DecisionPriority::Critical,
                    plan.created_at.clone(),
                    plan.requesting_actor_id.to_string(),
                    String::from("AIAssistant"),
                    None,
                    Vec::new(),
                    Some(String::from("resume_orchestrated_ai_plan")),
                )?);
            }
        }
        Ok(items)
    }

    fn adapt_planning_continuations(
        scoped_plans: &[&AiOrchestratedPlan],
        workflows: &[workspace_domain::AiAssistantWorkflow],
        workspace_id: &str,
        _active_project: Option<&str>,
    ) -> Result<Vec<DecisionItem>> {
        let mut items = Vec::new();
        for plan in scoped_plans {
            if matches!(
                plan.state,
                AiOrchestratedPlanState::Proposed
                    | AiOrchestratedPlanState::AwaitingApproval
                    | AiOrchestratedPlanState::PartiallyApproved
                    | AiOrchestratedPlanState::Executing
            ) {
                let priority = if plan.state == AiOrchestratedPlanState::AwaitingApproval {
                    DecisionPriority::High
                } else {
                    DecisionPriority::Normal
                };
                items.push(DecisionItem::aggregate(
                    workspace_id,
                    DecisionSourceType::PlanningContinuation,
                    format!("plan:{}", plan.id),
                    DecisionCategory::Planning,
                    String::from("Planning continuation requires attention"),
                    format!("Plan {} ({})", plan.id, plan.state.as_str()),
                    format!(
                        "Unfinished plan for goal: {}. Continue through Command Pipeline → Permission Gateway.",
                        plan.goal.statement
                    ),
                    String::from(
                        "Resume or advance the orchestrated plan via existing governed commands.",
                    ),
                    priority,
                    plan.created_at.clone(),
                    plan.requesting_actor_id.to_string(),
                    String::from("AIAssistant"),
                    None,
                    Vec::new(),
                    Some(String::from("advance_orchestrated_ai_plan")),
                )?);
            }
        }
        for workflow in workflows {
            if workflow.workspace_id.as_deref() != Some(workspace_id) {
                continue;
            }
            if matches!(
                workflow.state.as_str(),
                "completed" | "cancelled" | "failed"
            ) {
                continue;
            }
            let priority = if matches!(
                workflow.state.as_str(),
                "awaiting_confirmation" | "waiting_for_permission"
            ) {
                DecisionPriority::High
            } else {
                DecisionPriority::Normal
            };
            items.push(DecisionItem::aggregate(
                workspace_id,
                DecisionSourceType::PlanningContinuation,
                format!("workflow:{}", workflow.id),
                DecisionCategory::Planning,
                String::from("Assistant workflow needs a human decision"),
                format!("{} ({})", workflow.user_goal, workflow.state.as_str()),
                format!(
                    "Assistant workflow is in state '{}'. Assistant cannot accept or execute — you decide.",
                    workflow.state.as_str()
                ),
                String::from(
                    "Confirm, resume, or cancel via existing assistant workflow commands.",
                ),
                priority,
                workflow.updated_at.clone(),
                String::from("assistant"),
                String::from("AIAssistant"),
                None,
                Vec::new(),
                Some(String::from("confirm_assistant_workflow")),
            )?);
        }
        Ok(items)
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

fn plan_belongs_to_workspace(
    plan: &AiOrchestratedPlan,
    app_ids: &HashSet<String>,
    linked_plan_ids: &HashSet<String>,
) -> bool {
    if linked_plan_ids.contains(plan.id.as_str()) {
        return true;
    }
    plan.steps.iter().any(|step| {
        step.proposal
            .target_resource
            .as_ref()
            .is_some_and(|resource| app_ids.contains(resource.id.as_str()))
    })
}

fn approval_belongs_to_workspace(
    item: &PermissionApprovalRequest,
    workspace_id: &str,
    app_ids: &HashSet<String>,
    linked_approval_ids: &HashSet<String>,
) -> bool {
    if linked_approval_ids.contains(item.id.as_str()) {
        return true;
    }
    if item.subject.contains(workspace_id) || item.reason.contains(workspace_id) {
        return true;
    }
    app_ids.iter().any(|app_id| {
        item.subject.contains(app_id.as_str()) || item.reason.contains(app_id.as_str())
    })
}
