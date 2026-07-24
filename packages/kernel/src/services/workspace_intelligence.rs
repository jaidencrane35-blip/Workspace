//! Workspace Intelligence aggregator — read-only (Phase 4 Batch 5 / 5.5).
//!
//! Consumes existing awareness, memory, personalization, plans, approvals.
//! Cannot execute, approve, grant, or bypass Permission Gateway.
//! Batch 5.5: workspace-scoped aggregation, no create-on-read.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{ApplicationRepository, Database};
use workspace_domain::{
    ActorContext, AiOrchestratedPlan, AiOrchestratedPlanState, AiPlanStepState,
    AutomationContractSummary, BlockedActionSummary, IntelligenceApplicationSummary,
    IntelligenceHighlight, IntentContext, PendingDecisionSummary, PermissionApprovalRequest,
    PermissionApprovalStatus, RecentActivityItem, WorkspaceId, WorkspaceIntelligenceState,
    WorkspaceRecommendation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    list_rejection_summaries, AiMemoryService, AiPersonalizationService, AssistantWorkflowStore,
    AuditService, AutomationContractService, DesktopWindowService, OrchestratedPlanStore,
    PermissionApprovalService, TriggerEvaluatorService, WorkspaceIntentService,
};

pub(crate) struct WorkspaceIntelligenceService;

impl WorkspaceIntelligenceService {
    /// Build a read-only intelligence snapshot. Never mutates authority or intent tables.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceIntelligenceState> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();
        let ws = workspace_id.as_str();

        // Read-only: never insert WorkflowContext during generate.
        let workflow_context = WorkspaceIntentService::get_workflow_context_readonly(db, ws)?;
        let current_project = match &workflow_context.active_project_id {
            Some(id) => WorkspaceIntentService::get_project(db, id.as_str())
                .ok()
                .filter(|project| project.workspace_id.as_str() == ws && !project.deleted),
            None => WorkspaceIntentService::list_projects(db, ws, 1)?
                .into_iter()
                .next(),
        };
        let current_task = match &workflow_context.active_task_id {
            Some(id) => WorkspaceIntentService::get_task(db, id.as_str())
                .ok()
                .filter(|task| task.workspace_id.as_str() == ws && !task.deleted),
            None => current_project.as_ref().and_then(|project| {
                WorkspaceIntentService::list_tasks(db, ws, Some(project.id.as_str()), 1)
                    .ok()
                    .and_then(|tasks| tasks.into_iter().next())
            }),
        };
        let recent_goals = WorkspaceIntentService::list_goals(db, ws, 10)?;
        // Read-only — never mutate contracts or proposals from intelligence.
        let automation_contracts = AutomationContractService::list(db, ws, Some(20))?
            .iter()
            .map(AutomationContractSummary::from)
            .collect::<Vec<_>>();
        let pending_automation_proposals =
            TriggerEvaluatorService::pending_summaries(db, ws, 20).unwrap_or_default();
        let recent_trigger_rejections =
            list_rejection_summaries(db, ws, 20).unwrap_or_default();

        let memory = AiMemoryService::assemble_awareness(db, Some(ws), 10)
            .unwrap_or_else(|_| workspace_domain::AiMemoryAwareness::from_entries(Vec::new()));
        let personalization = AiPersonalizationService::assemble_awareness(db, Some(ws), 10, None)
            .unwrap_or_else(|_| {
                workspace_domain::AiPersonalizationAwareness::from_preferences(Vec::new(), true)
            });

        let memory_highlights = memory
            .entries
            .iter()
            .take(5)
            .map(|entry| IntelligenceHighlight {
                id: entry.id.to_string(),
                label: entry.key.clone(),
                summary: entry.summary.clone(),
                source: "memory".into(),
            })
            .collect::<Vec<_>>();

        let preference_highlights = if personalization.enabled {
            personalization
                .preferences
                .iter()
                .take(5)
                .map(|pref| IntelligenceHighlight {
                    id: pref.id.to_string(),
                    label: pref.label.clone().unwrap_or_else(|| pref.key.clone()),
                    summary: pref.value.clone(),
                    source: "preference".into(),
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let window_titles = DesktopWindowService::list_recent(Some(50))
            .unwrap_or_default()
            .into_iter()
            .map(|window| window.title.to_lowercase())
            .collect::<Vec<_>>();

        let applications = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            ApplicationRepository::new(&guard)
                .list_by_workspace(&workspace_id)?
                .into_iter()
                .map(|app| {
                    let name_lower = app.name.to_lowercase();
                    let appears_active = window_titles
                        .iter()
                        .any(|title| title.contains(&name_lower));
                    IntelligenceApplicationSummary {
                        id: app.id.to_string(),
                        name: app.name.clone(),
                        appears_active,
                    }
                })
                .collect::<Vec<_>>()
        };
        let app_ids: HashSet<String> = applications.iter().map(|app| app.id.clone()).collect();

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
                    .filter(|workflow| workflow.workspace_id.as_deref() == Some(ws))
                    .filter_map(|workflow| {
                        workflow
                            .orchestrated_plan_id
                            .as_ref()
                            .map(|id| id.to_string())
                    }),
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

        let approvals = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            PermissionApprovalService::list_recent(&guard, Some(50))?
        };
        let pending_approvals = approvals
            .into_iter()
            .filter(|item| item.status == PermissionApprovalStatus::Pending)
            .filter(|item| {
                approval_belongs_to_workspace(item, ws, &app_ids, &linked_approval_ids)
            })
            .map(|item| PendingDecisionSummary {
                id: item.id.to_string(),
                summary: format!("{} requires approval", item.command_name),
                explanation: format!(
                    "Waiting because permission approval is required for capability {}.",
                    item.capability
                ),
            })
            .collect::<Vec<_>>();

        let mut pending_plans = Vec::new();
        let mut blocked_actions = Vec::new();
        for plan in scoped_plans {
            if matches!(
                plan.state,
                AiOrchestratedPlanState::Proposed
                    | AiOrchestratedPlanState::AwaitingApproval
                    | AiOrchestratedPlanState::PartiallyApproved
                    | AiOrchestratedPlanState::Executing
            ) {
                pending_plans.push(format!(
                    "Plan {} ({}) — {}",
                    plan.id,
                    plan.state.as_str(),
                    plan.goal.statement
                ));
            }
            for step in &plan.steps {
                if matches!(
                    step.state,
                    AiPlanStepState::Denied | AiPlanStepState::Failed
                ) {
                    blocked_actions.push(BlockedActionSummary {
                        id: step.id.to_string(),
                        summary: format!(
                            "{} blocked ({})",
                            step.proposal.command_name,
                            step.state.as_str()
                        ),
                        explanation: step.last_outcome_detail.clone().unwrap_or_else(|| {
                            "Blocked because Permission Gateway denied or the step failed.".into()
                        }),
                    });
                }
            }
        }

        // Exact workspace match only — never include unbound workflows.
        for workflow in &workflows {
            if workflow.workspace_id.as_deref() != Some(ws) {
                continue;
            }
            if !matches!(
                workflow.state.as_str(),
                "completed" | "cancelled" | "failed"
            ) {
                pending_plans.push(format!(
                    "Assistant workflow {} — {} ({})",
                    workflow.id,
                    workflow.user_goal,
                    workflow.state.as_str()
                ));
            }
        }

        let recent_activity = AuditService::list_recent(db, 40)?
            .into_iter()
            .filter(|event| {
                event
                    .metadata
                    .as_deref()
                    .is_some_and(|metadata| metadata.contains(ws))
            })
            .take(15)
            .map(|event| RecentActivityItem {
                event_type: event.event_type.clone(),
                summary: event
                    .command_name
                    .clone()
                    .unwrap_or_else(|| event.event_type.clone()),
                timestamp: event.timestamp.clone(),
            })
            .collect::<Vec<_>>();

        let recommended_actions = Self::build_recommendations(
            &current_project,
            &current_task,
            &memory_highlights,
            &preference_highlights,
            &applications,
            &pending_approvals,
            &blocked_actions,
            &automation_contracts,
            personalization.enabled,
        );

        let summary = Self::build_summary(
            &workspace_name,
            &current_project,
            &current_task,
            pending_approvals.len(),
            blocked_actions.len(),
            recommended_actions.len(),
            &automation_contracts,
            pending_automation_proposals.len(),
        );

        let state = WorkspaceIntelligenceState {
            workspace_id: ws.to_string(),
            workspace_name,
            generated_at: Utc::now().to_rfc3339(),
            current_project,
            current_task,
            workflow_context,
            recent_goals,
            recent_activity,
            pending_plans,
            pending_approvals,
            blocked_actions,
            recommended_actions,
            memory_highlights,
            preference_highlights,
            current_applications: applications,
            automation_contracts,
            pending_automation_proposals,
            recent_trigger_rejections,
            workspace_health: health_label,
            summary,
            authority_effect: WorkspaceIntelligenceState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    fn build_recommendations(
        project: &Option<workspace_domain::Project>,
        task: &Option<workspace_domain::Task>,
        memory: &[IntelligenceHighlight],
        preferences: &[IntelligenceHighlight],
        applications: &[IntelligenceApplicationSummary],
        pending_approvals: &[PendingDecisionSummary],
        blocked: &[BlockedActionSummary],
        automation_contracts: &[AutomationContractSummary],
        personalization_enabled: bool,
    ) -> Vec<WorkspaceRecommendation> {
        let mut recommendations = Vec::new();

        if let Some(pending) = pending_approvals.first() {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-approval-{}", pending.id),
                title: "Review pending permission decision".into(),
                explanation: pending.explanation.clone(),
                kind: "pending_decision".into(),
            });
        }

        if let Some(blocked) = blocked.first() {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-blocked-{}", blocked.id),
                title: "Resolve blocked action".into(),
                explanation: blocked.explanation.clone(),
                kind: "blocked_action".into(),
            });
        }

        if let Some(task) = task {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-task-{}", task.id),
                title: format!("Continue task: {}", task.title),
                explanation: "Showing because this task is the active work item for the workspace."
                    .into(),
                kind: "active_task".into(),
            });
        } else if let Some(project) = project {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-project-{}", project.id),
                title: format!("Set an active task in {}", project.name),
                explanation:
                    "Suggested because a project is active but no current task is selected.".into(),
                kind: "active_project".into(),
            });
        }

        if personalization_enabled && !preferences.is_empty() && !applications.is_empty() {
            let app_names = applications
                .iter()
                .take(2)
                .map(|app| app.name.as_str())
                .collect::<Vec<_>>()
                .join(" and ");
            let pref_label = preferences[0].label.clone();
            recommendations.push(WorkspaceRecommendation {
                id: "rec-pref-apps".into(),
                title: format!("Prepare familiar apps ({app_names})"),
                explanation: format!(
                    "Suggested because this project usually uses preferences like \"{pref_label}\" with registered applications."
                ),
                kind: "preference".into(),
            });
        }

        if let Some(memory) = memory.first() {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-memory-{}", memory.id),
                title: format!("Remember: {}", memory.label),
                explanation: format!(
                    "Showing because workspace memory notes \"{}\".",
                    memory.summary
                ),
                kind: "memory".into(),
            });
        }

        if let Some(contract) = automation_contracts
            .iter()
            .find(|c| c.approval_state == "pending")
        {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-contract-pending-{}", contract.id),
                title: format!("Review automation contract: {}", contract.name),
                explanation: format!(
                    "Showing because contract \"{}\" awaits definition approval. \
                     Approving the definition does not authorize execution.",
                    contract.name
                ),
                kind: "automation_contract".into(),
            });
        } else if let Some(contract) = automation_contracts
            .iter()
            .find(|c| c.status == "approved" && c.approval_state == "approved")
        {
            recommendations.push(WorkspaceRecommendation {
                id: format!("rec-contract-active-{}", contract.id),
                title: format!("Approved automation: {}", contract.name),
                explanation: format!(
                    "Showing because an approved contract intends: \"{}\". \
                     Triggers do not run automatically in this release.",
                    contract.intent_statement
                ),
                kind: "automation_contract".into(),
            });
        }

        if recommendations.is_empty() {
            recommendations.push(WorkspaceRecommendation {
                id: "rec-idle".into(),
                title: "Define current work".into(),
                explanation:
                    "Suggested because no active project or task is set for this workspace.".into(),
                kind: "bootstrap".into(),
            });
        }

        recommendations
    }

    fn build_summary(
        workspace_name: &str,
        project: &Option<workspace_domain::Project>,
        task: &Option<workspace_domain::Task>,
        pending_approvals: usize,
        blocked: usize,
        recommendations: usize,
        automation_contracts: &[AutomationContractSummary],
        pending_proposals: usize,
    ) -> String {
        let project_label = project
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or("no active project");
        let task_label = task
            .as_ref()
            .map(|t| t.title.as_str())
            .unwrap_or("no active task");
        let approved_contracts = automation_contracts
            .iter()
            .filter(|c| c.status == "approved" && c.approval_state == "approved")
            .count();
        format!(
            "Workspace \"{workspace_name}\" — working on {project_label} / {task_label}. \
             {pending_approvals} pending decision(s), {blocked} blocked action(s), \
             {recommendations} recommendation(s), {approved_contracts} approved automation \
             contract(s), {pending_proposals} pending automation proposal(s). \
             Intelligence is informational only."
        )
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceIntelligenceState,
    ) -> Result<()> {
        // Single operational event — avoid audit fan-out noise (Batch 5.5).
        let metadata = json!({
            "workspace_id": state.workspace_id,
            "recommendation_count": state.recommended_actions.len(),
            "pending_approvals": state.pending_approvals.len(),
            "blocked_actions": state.blocked_actions.len(),
            "memory_highlights": state.memory_highlights.len(),
            "preference_highlights": state.preference_highlights.len(),
            "summary_len": state.summary.len(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.intelligence.generated",
            true,
            metadata,
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
