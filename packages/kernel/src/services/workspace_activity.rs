//! Workspace Activity Graph (Phase 4 Batch 9).
//!
//! Aggregates existing Workspace objects into one coherent history.
//! Pure read model — no authority, no execution, no duplicate ownership.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{ApplicationRepository, Database};
use workspace_domain::{
    ActorContext, ActivitySourceType, ActivityType, AiOrchestratedPlan, AiOrchestratedPlanState,
    AiPlanStepState, AutomationIntentProposalStatus, DecisionQueue, DecisionSourceType,
    IntentContext, PermissionApprovalRequest, PermissionApprovalStatus, WorkspaceActivity,
    WorkspaceActivityGraph, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{
    approval_belongs_to_workspace, plan_belongs_to_workspace, AssistantWorkflowStore, AuditService,
    AutomationContractService, DecisionQueueService, ExecutionOutcomeService,
    OrchestratedPlanStore, PermissionApprovalService, TriggerEvaluatorService,
    WorkspaceIntentService,
};

pub(crate) struct WorkspaceActivityGraphService;

impl WorkspaceActivityGraphService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceActivityGraph> {
        Self::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            None,
        )
    }

    /// Prefer passing a Decision Queue from Intelligence to avoid nested overlay writes.
    pub(crate) fn generate_with_decision_queue(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        decision_queue: Option<&DecisionQueue>,
    ) -> Result<WorkspaceActivityGraph> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let ws = workspace_id.as_str();

        let workflow_context = WorkspaceIntentService::get_workflow_context_readonly(db, ws)?;
        let applications = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            ApplicationRepository::new(&guard).list_by_workspace(&workspace_id)?
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

        let mut by_id: HashMap<String, WorkspaceActivity> = HashMap::new();

        for activity in Self::adapt_work_context(db, ws)? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_contracts(db, ws)? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_triggers_and_proposals(db, ws)? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            ws,
            decision_queue,
        )? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_permission_approvals(
            db,
            ws,
            &app_ids,
            &linked_approval_ids,
        )? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_planning(&scoped_plans, &workflows, ws)? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_execution_outcomes(db, ws)? {
            by_id.insert(activity.id.to_string(), activity);
        }
        for activity in Self::adapt_audit_gap_fill(db, ws, &by_id)? {
            by_id.insert(activity.id.to_string(), activity);
        }

        let mut activities: Vec<WorkspaceActivity> = by_id.into_values().collect();
        Self::link_bidirectional(&mut activities);

        let graph = WorkspaceActivityGraph::from_activities(ws, activities);

        Self::audit(
            db,
            actor,
            "workspace.relationship.generated",
            json!({
                "workspace_id": ws,
                "relationship_count": graph.relationship_count,
                "authority_effect": "none",
            }),
        )?;
        Self::audit(
            db,
            actor,
            "workspace.activity.related",
            json!({
                "workspace_id": ws,
                "activity_count": graph.activities.len(),
                "authority_effect": "none",
            }),
        )?;
        Self::audit(
            db,
            actor,
            "workspace.timeline.generated",
            json!({
                "workspace_id": ws,
                "timeline_count": graph.timeline.len(),
                "authority_effect": "none",
            }),
        )?;
        Self::audit(
            db,
            actor,
            "workspace.activity.summary.generated",
            json!({
                "workspace_id": ws,
                "activity_count": graph.activities.len(),
                "unresolved_count": graph.unresolved_count,
                "relationship_count": graph.relationship_count,
                "authority_effect": "none",
            }),
        )?;

        Ok(graph)
    }

    /// Architecture guard — Activity Graph must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceActivityError::CannotExecute,
        ))
    }

    fn adapt_work_context(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceActivity>> {
        let projects = WorkspaceIntentService::list_projects(db, workspace_id, 100)?;
        let tasks = WorkspaceIntentService::list_tasks(db, workspace_id, None, 200)?;
        let goals = WorkspaceIntentService::list_goals(db, workspace_id, 50)?;
        let mut out = Vec::new();

        for project in projects {
            if project.deleted {
                continue;
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::Project,
                ActivitySourceType::WorkContext,
                project.id.to_string(),
                format!("Project \"{}\"", project.name),
                format!(
                    "Work project in status '{}'. Projects anchor tasks and automation contracts.",
                    project.status.as_str()
                ),
                project.created_at.clone(),
                String::from("local-user"),
                String::from("LocalUser"),
                Some(project.id.to_string()),
                None,
                None,
                Vec::new(),
                false,
            )?);
        }

        for task in tasks {
            if task.deleted {
                continue;
            }
            let parent = Some(
                WorkspaceActivity::synthetic_id(ActivityType::Project, task.project_id.as_str())
                    .to_string(),
            );
            let unresolved = !matches!(task.status.as_str(), "done" | "cancelled");
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::Task,
                ActivitySourceType::WorkContext,
                task.id.to_string(),
                format!("Task \"{}\"", task.title),
                format!(
                    "Task in status '{}'. Belongs to project {}.",
                    task.status.as_str(),
                    task.project_id
                ),
                task.created_at.clone(),
                String::from("local-user"),
                String::from("LocalUser"),
                Some(task.project_id.to_string()),
                Some(task.id.to_string()),
                parent,
                vec![WorkspaceActivity::synthetic_id(
                    ActivityType::Project,
                    task.project_id.as_str(),
                )
                .to_string()],
                unresolved,
            )?);
        }

        for goal in goals {
            let parent = goal
                .project_id
                .as_ref()
                .map(|pid| {
                    WorkspaceActivity::synthetic_id(ActivityType::Project, pid.as_str()).to_string()
                })
                .or_else(|| {
                    goal.task_id.as_ref().map(|tid| {
                        WorkspaceActivity::synthetic_id(ActivityType::Task, tid.as_str())
                            .to_string()
                    })
                });
            let mut related = Vec::new();
            if let Some(ref pid) = goal.project_id {
                related.push(
                    WorkspaceActivity::synthetic_id(ActivityType::Project, pid.as_str())
                        .to_string(),
                );
            }
            if let Some(ref tid) = goal.task_id {
                related.push(
                    WorkspaceActivity::synthetic_id(ActivityType::Task, tid.as_str()).to_string(),
                );
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::WorkGoal,
                ActivitySourceType::WorkContext,
                goal.id.to_string(),
                format!("Work goal: {}", goal.description),
                String::from("A durable work goal attached to this workspace's project or task."),
                goal.created_at.clone(),
                String::from("local-user"),
                String::from("LocalUser"),
                goal.project_id.as_ref().map(|id| id.to_string()),
                goal.task_id.as_ref().map(|id| id.to_string()),
                parent,
                related,
                !matches!(goal.status.as_str(), "achieved" | "abandoned"),
            )?);
        }

        Ok(out)
    }

    fn adapt_contracts(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceActivity>> {
        let contracts = AutomationContractService::list(db, workspace_id, Some(100))?;
        let mut out = Vec::new();
        for contract in contracts {
            if contract.deleted {
                continue;
            }
            let parent = contract
                .task_id
                .as_ref()
                .map(|tid| {
                    WorkspaceActivity::synthetic_id(ActivityType::Task, tid.as_str()).to_string()
                })
                .unwrap_or_else(|| {
                    WorkspaceActivity::synthetic_id(
                        ActivityType::Project,
                        contract.project_id.as_str(),
                    )
                    .to_string()
                });
            let mut related = vec![WorkspaceActivity::synthetic_id(
                ActivityType::Project,
                contract.project_id.as_str(),
            )
            .to_string()];
            if let Some(ref tid) = contract.task_id {
                related.push(
                    WorkspaceActivity::synthetic_id(ActivityType::Task, tid.as_str()).to_string(),
                );
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::AutomationContract,
                ActivitySourceType::AutomationContract,
                contract.id.to_string(),
                format!("Automation contract \"{}\"", contract.name),
                format!(
                    "Stored future-intent definition (status {}, approval {}). Not execution authority.",
                    contract.status.as_str(),
                    contract.approval_state.as_str()
                ),
                contract.created_at.clone(),
                contract.created_by_actor.clone(),
                String::from("LocalUser"),
                Some(contract.project_id.to_string()),
                contract.task_id.as_ref().map(|id| id.to_string()),
                Some(parent),
                related,
                matches!(
                    contract.status.as_str(),
                    "pending_approval" | "draft"
                ),
            )?);
        }
        Ok(out)
    }

    fn adapt_triggers_and_proposals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceActivity>> {
        let events = TriggerEvaluatorService::list_events(db, workspace_id, Some(100))?;
        let proposals = TriggerEvaluatorService::list_proposals(db, workspace_id, None, Some(100))?;
        let mut out = Vec::new();

        for event in events {
            let parent = event
                .project_id
                .as_ref()
                .map(|pid| {
                    WorkspaceActivity::synthetic_id(ActivityType::Project, pid.as_str()).to_string()
                });
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::TriggerEvent,
                ActivitySourceType::TriggerEvaluation,
                event.id.to_string(),
                format!("Trigger: {}", event.event_type.as_str()),
                format!(
                    "Informational trigger from {} — evaluation may produce intent proposals, never execution.",
                    event.source
                ),
                event.created_at.clone(),
                event.actor_id.clone(),
                event.actor_type.clone(),
                event.project_id.as_ref().map(|id| id.to_string()),
                event.task_id.as_ref().map(|id| id.to_string()),
                parent,
                Vec::new(),
                false,
            )?);
        }

        for proposal in proposals {
            let contract_activity = WorkspaceActivity::synthetic_id(
                ActivityType::AutomationContract,
                proposal.contract_id.as_str(),
            )
            .to_string();
            let event_activity = WorkspaceActivity::synthetic_id(
                ActivityType::TriggerEvent,
                proposal.trigger_event_id.as_str(),
            )
            .to_string();
            let related = vec![
                contract_activity.clone(),
                event_activity,
                WorkspaceActivity::synthetic_id(ActivityType::Project, proposal.project_id.as_str())
                    .to_string(),
            ];
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::IntentProposal,
                ActivitySourceType::TriggerEvaluation,
                proposal.id.to_string(),
                format!("Intent proposal: {}", proposal.intent_definition.statement),
                proposal.explanation.clone(),
                proposal.created_at.clone(),
                String::from("system"),
                String::from("System"),
                Some(proposal.project_id.to_string()),
                proposal.task_id.as_ref().map(|id| id.to_string()),
                Some(contract_activity),
                related,
                proposal.status == AutomationIntentProposalStatus::PendingReview,
            )?);
        }

        Ok(out)
    }

    fn adapt_decision_queue(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: &str,
        decision_queue: Option<&DecisionQueue>,
    ) -> Result<Vec<WorkspaceActivity>> {
        let owned;
        let queue = if let Some(queue) = decision_queue {
            queue
        } else {
            // Nested consumers must not persist Decision Queue overlays.
            owned = DecisionQueueService::aggregate_readonly(
                db,
                actor,
                orchestrated_plans,
                assistant_workflows,
                workspace_id,
            )?;
            &owned
        };
        let mut out = Vec::new();
        for item in &queue.items {
            let related_source = match item.source_type {
                DecisionSourceType::IntentProposal => WorkspaceActivity::synthetic_id(
                    ActivityType::IntentProposal,
                    &item.source_id,
                )
                .to_string(),
                DecisionSourceType::PendingApproval => {
                    if let Some(contract_id) = item.source_id.strip_prefix("contract:") {
                        WorkspaceActivity::synthetic_id(
                            ActivityType::AutomationContract,
                            contract_id,
                        )
                        .to_string()
                    } else {
                        WorkspaceActivity::synthetic_id(
                            ActivityType::PermissionApproval,
                            &item.source_id,
                        )
                        .to_string()
                    }
                }
                DecisionSourceType::BlockedAction => WorkspaceActivity::synthetic_id(
                    ActivityType::BlockedAction,
                    &item.source_id,
                )
                .to_string(),
                DecisionSourceType::PlanningContinuation => WorkspaceActivity::synthetic_id(
                    ActivityType::PlanningContinuation,
                    &item.source_id,
                )
                .to_string(),
            };
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::DecisionItem,
                ActivitySourceType::DecisionQueue,
                item.id.to_string(),
                item.title.clone(),
                format!(
                    "{}. Recommended: {}. Decision Queue aggregates; sources remain authoritative.",
                    item.explanation, item.recommended_action
                ),
                item.created_at.clone(),
                item.actor_id.clone(),
                item.actor_type.clone(),
                item.project_id.clone(),
                None,
                Some(related_source.clone()),
                vec![related_source],
                matches!(
                    item.decision_state.as_str(),
                    "pending" | "viewed" | "deferred"
                ),
            )?);
        }
        Ok(out)
    }

    fn adapt_permission_approvals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        app_ids: &HashSet<String>,
        linked_approval_ids: &HashSet<String>,
    ) -> Result<Vec<WorkspaceActivity>> {
        let approvals = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            PermissionApprovalService::list_recent(&guard, Some(100))?
        };
        let mut out = Vec::new();
        for item in approvals
            .into_iter()
            .filter(|a| approval_belongs_to_workspace(a, workspace_id, app_ids, linked_approval_ids))
        {
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::PermissionApproval,
                ActivitySourceType::PermissionApproval,
                item.id.to_string(),
                format!("{} permission decision", item.command_name),
                format!(
                    "Permission approval for capability {} (status {}). Gateway still required.",
                    item.capability,
                    item.status.as_str()
                ),
                item.created_at.clone(),
                item.requesting_actor_id.clone(),
                format!("{:?}", item.requesting_actor_type),
                None,
                None,
                None,
                Vec::new(),
                item.status == PermissionApprovalStatus::Pending,
            )?);
        }
        Ok(out)
    }

    fn adapt_planning(
        scoped_plans: &[&AiOrchestratedPlan],
        workflows: &[workspace_domain::AiAssistantWorkflow],
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceActivity>> {
        let mut out = Vec::new();
        for plan in scoped_plans {
            if matches!(
                plan.state,
                AiOrchestratedPlanState::Proposed
                    | AiOrchestratedPlanState::AwaitingApproval
                    | AiOrchestratedPlanState::PartiallyApproved
                    | AiOrchestratedPlanState::Executing
            ) {
                out.push(WorkspaceActivity::aggregate(
                    workspace_id,
                    ActivityType::PlanningContinuation,
                    ActivitySourceType::Planning,
                    format!("plan:{}", plan.id),
                    format!("Plan continuation: {}", plan.goal.statement),
                    format!(
                        "Orchestrated plan in state '{}'. Continue only through governed commands.",
                        plan.state.as_str()
                    ),
                    plan.created_at.clone(),
                    plan.requesting_actor_id.to_string(),
                    String::from("AIAssistant"),
                    None,
                    None,
                    None,
                    Vec::new(),
                    true,
                )?);
            }
            for step in &plan.steps {
                if matches!(
                    step.state,
                    AiPlanStepState::Denied | AiPlanStepState::Failed
                ) {
                    let mut related = vec![WorkspaceActivity::synthetic_id(
                        ActivityType::PlanningContinuation,
                        &format!("plan:{}", plan.id),
                    )
                    .to_string()];
                    if let Some(ref approval_id) = step.approval_request_id {
                        related.push(
                            WorkspaceActivity::synthetic_id(
                                ActivityType::PermissionApproval,
                                approval_id,
                            )
                            .to_string(),
                        );
                    }
                    out.push(WorkspaceActivity::aggregate(
                        workspace_id,
                        ActivityType::BlockedAction,
                        ActivitySourceType::Planning,
                        step.id.to_string(),
                        format!("{} blocked", step.proposal.command_name),
                        step.last_outcome_detail.clone().unwrap_or_else(|| {
                            String::from(
                                "Blocked because Permission Gateway denied or the step failed.",
                            )
                        }),
                        plan.updated_at.clone(),
                        plan.requesting_actor_id.to_string(),
                        String::from("AIAssistant"),
                        None,
                        None,
                        Some(WorkspaceActivity::synthetic_id(
                            ActivityType::PlanningContinuation,
                            &format!("plan:{}", plan.id),
                        )
                        .to_string()),
                        related,
                        true,
                    )?);
                }
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
            let mut related = Vec::new();
            if let Some(ref plan_id) = workflow.orchestrated_plan_id {
                related.push(
                    WorkspaceActivity::synthetic_id(
                        ActivityType::PlanningContinuation,
                        &format!("plan:{plan_id}"),
                    )
                    .to_string(),
                );
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::PlanningContinuation,
                ActivitySourceType::Planning,
                format!("workflow:{}", workflow.id),
                format!("Assistant workflow: {}", workflow.user_goal),
                format!(
                    "Assistant workflow in state '{}'. Assistant cannot execute — you decide.",
                    workflow.state.as_str()
                ),
                workflow.updated_at.clone(),
                String::from("assistant"),
                String::from("AIAssistant"),
                None,
                None,
                related.first().cloned(),
                related,
                true,
            )?);
        }
        Ok(out)
    }

    fn adapt_execution_outcomes(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceActivity>> {
        let outcomes = ExecutionOutcomeService::list_recent(db, 50)?;
        let mut out = Vec::new();
        for outcome in outcomes {
            // Outcomes lack workspace_id — attribute via request id / failure text heuristics.
            let haystack = format!(
                "{} {} {}",
                outcome.execution_request_id,
                outcome.command_name,
                outcome.failure_reason.as_deref().unwrap_or("")
            );
            if !haystack.contains(workspace_id) {
                continue;
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::ExecutionOutcome,
                ActivitySourceType::Execution,
                outcome.execution_request_id.clone(),
                format!("{} {}", outcome.command_name, outcome.status.as_str()),
                format!(
                    "Execution outcome recorded after Command Pipeline → Permission Gateway (success={}).",
                    outcome.success
                ),
                outcome.completed_at.clone(),
                String::from("system"),
                String::from("System"),
                None,
                None,
                None,
                Vec::new(),
                !outcome.success,
            )?);
        }
        Ok(out)
    }

    fn adapt_audit_gap_fill(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        existing: &HashMap<String, WorkspaceActivity>,
    ) -> Result<Vec<WorkspaceActivity>> {
        let events = AuditService::list_recent(db, 40)?;
        let covered_event_types: HashSet<&str> = [
            "workspace.timeline.generated",
            "workspace.relationship.generated",
            "workspace.activity.summary.generated",
            "workspace.activity.related",
            "decision.queue.generated",
            "workspace.intelligence.generated",
        ]
        .into_iter()
        .collect();
        let mut out = Vec::new();
        for event in events {
            let meta = event.metadata.as_deref().unwrap_or("");
            if !meta.contains(workspace_id) {
                continue;
            }
            if covered_event_types.contains(event.event_type.as_str()) {
                continue;
            }
            // Skip if an activity already covers this audit id-ish source.
            let source_id = event.id.to_string();
            let activity_id =
                WorkspaceActivity::synthetic_id(ActivityType::AuditSignal, &source_id).to_string();
            if existing.contains_key(&activity_id) {
                continue;
            }
            // Prefer not duplicating automation/decision audits already represented by adapters.
            if event.event_type.starts_with("automation.")
                || event.event_type.starts_with("decision.")
            {
                continue;
            }
            out.push(WorkspaceActivity::aggregate(
                workspace_id,
                ActivityType::AuditSignal,
                ActivitySourceType::Audit,
                source_id,
                event
                    .command_name
                    .clone()
                    .unwrap_or_else(|| event.event_type.clone()),
                format!(
                    "Operational audit signal '{}' — informational history only.",
                    event.event_type
                ),
                event.timestamp.clone(),
                event
                    .actor_id
                    .clone()
                    .unwrap_or_else(|| String::from("system")),
                String::from("System"),
                None,
                None,
                None,
                Vec::new(),
                false,
            )?);
        }
        Ok(out)
    }

    fn link_bidirectional(activities: &mut [WorkspaceActivity]) {
        let mut extra: HashMap<String, HashSet<String>> = HashMap::new();
        for activity in activities.iter() {
            let id = activity.id.to_string();
            for related in &activity.related_activity_ids {
                extra.entry(related.clone()).or_default().insert(id.clone());
            }
            if let Some(ref parent) = activity.parent_activity_id {
                extra.entry(parent.clone()).or_default().insert(id.clone());
            }
        }
        for activity in activities.iter_mut() {
            if let Some(peers) = extra.get(activity.id.as_str()) {
                for peer in peers {
                    if !activity.related_activity_ids.iter().any(|r| r == peer)
                        && activity.id.as_str() != peer
                    {
                        activity.related_activity_ids.push(peer.clone());
                    }
                }
            }
        }
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
