//! Workspace Continuity Engine (Phase 5 Batch 1).
//!
//! Read-only projection of where work left off. Aggregates WorkflowContext,
//! Decision Queue, Activity Graph, contracts (status), and outcomes.
//! Never executes, never grants authority, never persists payloads.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActivityType, ActorContext, ContinuityFacet, ContinuityFacetKind, DecisionQueue, DecisionState,
    IntentContext, WorkspaceActivityGraph, WorkspaceContinuityState, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, AutomationContractService, DecisionQueueService,
    OrchestratedPlanStore, WorkspaceActivityGraphService, WorkspaceIntentService,
};

pub(crate) struct WorkspaceContinuityService;

impl WorkspaceContinuityService {
    /// Standalone generate — uses readonly Decision Queue + injected AG path.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceContinuityState> {
        let workspace_id = workspace_id.into();
        let queue = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let graph = WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            Some(&queue),
        )?;
        Self::generate_with_inputs(db, actor, &workspace_id, &queue, &graph)
    }

    /// Preferred path — Intelligence injects already-generated DQ + AG.
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        decision_queue: &DecisionQueue,
        activity_graph: &WorkspaceActivityGraph,
    ) -> Result<WorkspaceContinuityState> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let ws = workspace_id.as_str();

        let workflow = WorkspaceIntentService::get_workflow_context_readonly(db, ws)?;
        let projects = WorkspaceIntentService::list_projects(db, ws, 50)?;
        let contracts = AutomationContractService::list(db, ws, Some(50)).unwrap_or_default();

        let session_anchor = Self::session_anchor(&workflow.updated_at, activity_graph);

        let current_focus = Self::project_current_focus(db, ws, &workflow, &session_anchor)?;
        let outstanding_decisions =
            Self::project_outstanding_decisions(ws, decision_queue, &session_anchor)?;
        let blockers = Self::project_blockers(ws, decision_queue, activity_graph, &session_anchor)?;
        let interrupted_work =
            Self::project_interrupted(ws, decision_queue, activity_graph, &session_anchor)?;
        let resumable_work = Self::project_resumable(
            ws,
            &current_focus,
            &interrupted_work,
            &outstanding_decisions,
            &session_anchor,
        )?;
        let dormant_projects =
            Self::project_dormant(ws, &projects, &workflow, activity_graph, &session_anchor)?;
        let active_commitments =
            Self::project_commitments(ws, &contracts, &session_anchor)?;
        let recent_progress =
            Self::project_recent_progress(ws, activity_graph, &session_anchor)?;
        let recent_outcomes =
            Self::project_recent_outcomes(ws, activity_graph, &session_anchor)?;
        let suggested_next_step = Self::project_suggested_next(
            ws,
            &current_focus,
            &outstanding_decisions,
            &blockers,
            &resumable_work,
            &session_anchor,
        )?;

        let summary = Self::build_summary(
            &current_focus,
            outstanding_decisions.len(),
            interrupted_work.len(),
            resumable_work.len(),
            blockers.len(),
            &suggested_next_step,
        );

        let state = WorkspaceContinuityState {
            workspace_id: ws.to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            session_anchor,
            current_focus,
            interrupted_work,
            resumable_work,
            outstanding_decisions,
            dormant_projects,
            active_commitments,
            recent_progress,
            recent_outcomes,
            blockers,
            suggested_next_step,
            summary,
            authority_effect: WorkspaceContinuityState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    fn session_anchor(workflow_updated_at: &str, graph: &WorkspaceActivityGraph) -> String {
        let latest_activity = graph
            .timeline
            .iter()
            .map(|a| a.timestamp.as_str())
            .max()
            .unwrap_or("");
        if workflow_updated_at.is_empty() {
            return latest_activity.to_string();
        }
        if latest_activity.is_empty() {
            return workflow_updated_at.to_string();
        }
        if workflow_updated_at >= latest_activity {
            workflow_updated_at.to_string()
        } else {
            latest_activity.to_string()
        }
    }

    fn project_current_focus(
        db: &Arc<Mutex<Database>>,
        ws: &str,
        workflow: &workspace_domain::WorkflowContext,
        session_anchor: &str,
    ) -> Result<Option<ContinuityFacet>> {
        let project = match &workflow.active_project_id {
            Some(id) => WorkspaceIntentService::get_project(db, id.as_str())
                .ok()
                .filter(|p| p.workspace_id.as_str() == ws && !p.deleted),
            None => None,
        };
        let task = match &workflow.active_task_id {
            Some(id) => WorkspaceIntentService::get_task(db, id.as_str())
                .ok()
                .filter(|t| t.workspace_id.as_str() == ws && !t.deleted),
            None => None,
        };

        match (project, task) {
            (None, None) => Ok(None),
            (project, task) => {
                let project_name = project
                    .as_ref()
                    .map(|p| p.name.as_str())
                    .unwrap_or("no active project");
                let task_title = task
                    .as_ref()
                    .map(|t| t.title.as_str())
                    .unwrap_or("no active task");
                let source_key = format!(
                    "{}:{}",
                    project
                        .as_ref()
                        .map(|p| p.id.as_str())
                        .unwrap_or("none"),
                    task.as_ref().map(|t| t.id.as_str()).unwrap_or("none")
                );
                let mut evidence = Vec::new();
                if let Some(p) = &project {
                    evidence.push(format!("project:{}", p.id));
                }
                if let Some(t) = &task {
                    evidence.push(format!("task:{}", t.id));
                }
                Ok(Some(ContinuityFacet::project(
                    ws,
                    ContinuityFacetKind::CurrentFocus,
                    "workflow_context",
                    source_key,
                    format!("Current Focus: {project_name} / {task_title}"),
                    format!("Active work is {project_name} / {task_title}."),
                    "Showing because WorkflowContext records this as active work via set_active_work.",
                    evidence,
                    format!("Focus last aligned at session anchor {session_anchor}."),
                    project.as_ref().map(|p| p.id.to_string()),
                    task.as_ref().map(|t| t.id.to_string()),
                )?))
            }
        }
    }

    fn project_outstanding_decisions(
        ws: &str,
        queue: &DecisionQueue,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        for item in &queue.items {
            if !matches!(
                item.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            ) {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::OutstandingDecision,
                item.source_type.as_str(),
                item.id.to_string(),
                item.title.clone(),
                item.summary.clone(),
                format!(
                    "Showing because Decision Queue lists this as {} needing attention.",
                    item.decision_state.as_str()
                ),
                vec![item.id.to_string()],
                format!(
                    "Still outstanding relative to session anchor {session_anchor}."
                ),
                item.project_id.clone(),
                None,
            )?);
        }
        Ok(out)
    }

    fn project_blockers(
        ws: &str,
        queue: &DecisionQueue,
        graph: &WorkspaceActivityGraph,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for item in &queue.items {
            if item.source_type != workspace_domain::DecisionSourceType::BlockedAction {
                continue;
            }
            if !matches!(
                item.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            ) {
                continue;
            }
            seen.insert(item.source_id.clone());
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::Blocker,
                "blocked_action",
                item.source_id.clone(),
                item.title.clone(),
                item.summary.clone(),
                "Showing because Decision Queue surfaces a blocked action requiring human attention.",
                    
                vec![item.id.to_string()],
                format!("Blocker remains open at session anchor {session_anchor}."),
                item.project_id.clone(),
                None,
            )?);
        }
        for activity in &graph.activities {
            if activity.activity_type != ActivityType::BlockedAction || !activity.unresolved {
                continue;
            }
            if seen.contains(&activity.source_id) {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::Blocker,
                activity.source_type.as_str(),
                activity.source_id.clone(),
                activity.summary.clone(),
                activity.explanation.clone(),
                "Showing because Activity Graph marks this blocked action unresolved.",
                vec![activity.id.to_string()],
                format!("Unresolved since session anchor {session_anchor}."),
                activity.project_id.clone(),
                activity.task_id.clone(),
            )?);
        }
        Ok(out)
    }

    fn project_interrupted(
        ws: &str,
        queue: &DecisionQueue,
        graph: &WorkspaceActivityGraph,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        for item in &queue.items {
            if item.source_type != workspace_domain::DecisionSourceType::PlanningContinuation {
                continue;
            }
            if !matches!(
                item.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            ) {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::InterruptedWork,
                "planning_continuation",
                item.source_id.clone(),
                item.title.clone(),
                item.summary.clone(),
                "Showing because a non-terminal plan or Assistant workflow is still open.",
                vec![item.id.to_string()],
                format!("Interrupted relative to session anchor {session_anchor}."),
                item.project_id.clone(),
                None,
            )?);
        }
        for activity in graph.timeline.iter().rev().take(20) {
            if !activity.unresolved {
                continue;
            }
            if matches!(
                activity.activity_type,
                ActivityType::Task | ActivityType::PlanningContinuation | ActivityType::WorkGoal
            ) {
                let already = out.iter().any(|f| f.source_id == activity.source_id);
                if already {
                    continue;
                }
                out.push(ContinuityFacet::project(
                    ws,
                    ContinuityFacetKind::InterruptedWork,
                    activity.source_type.as_str(),
                    activity.source_id.clone(),
                    activity.summary.clone(),
                    activity.explanation.clone(),
                    "Showing because Activity Graph marks related work unresolved.",
                    vec![activity.id.to_string()],
                    format!("Still unresolved at session anchor {session_anchor}."),
                    activity.project_id.clone(),
                    activity.task_id.clone(),
                )?);
            }
        }
        Ok(out)
    }

    fn project_resumable(
        ws: &str,
        focus: &Option<ContinuityFacet>,
        interrupted: &[ContinuityFacet],
        outstanding: &[ContinuityFacet],
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        if let Some(focus) = focus {
            // Active focus without an outstanding decision on the same task is resumable.
            let blocked_by_decision = outstanding.iter().any(|d| {
                d.project_id == focus.project_id
                    && (focus.task_id.is_none() || d.task_id == focus.task_id)
            });
            if !blocked_by_decision {
                out.push(ContinuityFacet::project(
                    ws,
                    ContinuityFacetKind::ResumableWork,
                    "workflow_context",
                    focus.source_id.clone(),
                    format!("Resume: {}", focus.title),
                    focus.summary.clone(),
                    "Showing because Current Focus is set and no outstanding Decision blocks it.",
                        
                    focus.evidence_refs.clone(),
                    format!("Safe to continue from session anchor {session_anchor}."),
                    focus.project_id.clone(),
                    focus.task_id.clone(),
                )?);
            }
        }
        for item in interrupted.iter().take(5) {
            if out.iter().any(|f| f.source_id == item.source_id) {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::ResumableWork,
                item.source_type.clone(),
                format!("resume:{}", item.source_id),
                format!("Resume: {}", item.title),
                item.summary.clone(),
                "Showing because interrupted work remains open and can be continued deliberately.",
                    
                item.evidence_refs.clone(),
                item.what_changed.clone(),
                item.project_id.clone(),
                item.task_id.clone(),
            )?);
        }
        Ok(out)
    }

    fn project_dormant(
        ws: &str,
        projects: &[workspace_domain::Project],
        workflow: &workspace_domain::WorkflowContext,
        graph: &WorkspaceActivityGraph,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let active = workflow
            .active_project_id
            .as_ref()
            .map(|id| id.to_string());
        let recent_project_ids: HashSet<String> = graph
            .timeline
            .iter()
            .rev()
            .take(30)
            .filter_map(|a| a.project_id.clone())
            .collect();
        let mut out = Vec::new();
        for project in projects {
            if project.deleted {
                continue;
            }
            if Some(project.id.as_str()) == active.as_deref() {
                continue;
            }
            if recent_project_ids.contains(project.id.as_str()) {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::DormantProject,
                "project",
                project.id.to_string(),
                format!("Dormant: {}", project.name),
                format!("Project \"{}\" has no recent Activity Graph signal.", project.name),
                "Showing because this project is not active and has no recent related activity.",
                    
                vec![format!("project:{}", project.id)],
                format!("Quiet relative to session anchor {session_anchor}."),
                Some(project.id.to_string()),
                None,
            )?);
        }
        Ok(out)
    }

    fn project_commitments(
        ws: &str,
        contracts: &[workspace_domain::AutomationContract],
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        for contract in contracts {
            let status = contract.status.as_str();
            let approval = contract.approval_state.as_str();
            if !matches!(status, "approved" | "pending_approval" | "paused" | "draft")
                && approval != "pending"
            {
                continue;
            }
            if matches!(status, "revoked" | "completed") {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::ActiveCommitment,
                "automation_contract",
                contract.id.to_string(),
                contract.name.clone(),
                format!("Automation Contract {status} / approval {approval}"),
                "Showing because an Automation Contract is an active commitment (status only).",
                    
                vec![format!("activity:automation_contract:{}", contract.id)],
                format!("Commitment status observed at session anchor {session_anchor}."),
                Some(contract.project_id.to_string()),
                contract.task_id.as_ref().map(|id| id.to_string()),
            )?);
        }
        Ok(out)
    }

    fn project_recent_progress(
        ws: &str,
        graph: &WorkspaceActivityGraph,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        for activity in graph.timeline.iter().rev().take(8) {
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::RecentProgress,
                activity.source_type.as_str(),
                activity.id.to_string(),
                activity.summary.clone(),
                activity.explanation.clone(),
                "Showing because Activity Graph records this as recent related work.",
                vec![activity.id.to_string()],
                format!(
                    "Observed at {}; continuity anchor is {session_anchor}.",
                    activity.timestamp
                ),
                activity.project_id.clone(),
                activity.task_id.clone(),
            )?);
        }
        Ok(out)
    }

    fn project_recent_outcomes(
        ws: &str,
        graph: &WorkspaceActivityGraph,
        session_anchor: &str,
    ) -> Result<Vec<ContinuityFacet>> {
        let mut out = Vec::new();
        for activity in graph.timeline.iter().rev() {
            if activity.activity_type != ActivityType::ExecutionOutcome {
                continue;
            }
            out.push(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::RecentOutcome,
                activity.source_type.as_str(),
                activity.source_id.clone(),
                activity.summary.clone(),
                activity.explanation.clone(),
                "Showing because Activity Graph includes a workspace-attributable execution outcome.",
                    
                vec![activity.id.to_string()],
                format!(
                    "Outcome at {}; continuity anchor is {session_anchor}.",
                    activity.timestamp
                ),
                activity.project_id.clone(),
                activity.task_id.clone(),
            )?);
            if out.len() >= 5 {
                break;
            }
        }
        Ok(out)
    }

    fn project_suggested_next(
        ws: &str,
        focus: &Option<ContinuityFacet>,
        outstanding: &[ContinuityFacet],
        blockers: &[ContinuityFacet],
        resumable: &[ContinuityFacet],
        session_anchor: &str,
    ) -> Result<Option<ContinuityFacet>> {
        if let Some(blocker) = blockers.first() {
            return Ok(Some(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::SuggestedNextStep,
                blocker.source_type.clone(),
                format!("next:{}", blocker.source_id),
                format!("Resolve blocker: {}", blocker.title),
                blocker.summary.clone(),
                "Suggested because a blocker is outstanding in Decision Queue / Activity Graph.",
                    
                blocker.evidence_refs.clone(),
                format!("Priority relative to session anchor {session_anchor}."),
                blocker.project_id.clone(),
                blocker.task_id.clone(),
            )?));
        }
        if let Some(decision) = outstanding.first() {
            return Ok(Some(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::SuggestedNextStep,
                decision.source_type.clone(),
                format!("next:{}", decision.source_id),
                format!("Review decision: {}", decision.title),
                decision.summary.clone(),
                "Suggested because Decision Queue has items needing attention.",
                decision.evidence_refs.clone(),
                format!("Priority relative to session anchor {session_anchor}."),
                decision.project_id.clone(),
                decision.task_id.clone(),
            )?));
        }
        if let Some(resume) = resumable.first() {
            return Ok(Some(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::SuggestedNextStep,
                resume.source_type.clone(),
                format!("next:{}", resume.source_id),
                resume.title.clone(),
                resume.summary.clone(),
                "Suggested because resumable work is available without an outstanding Decision.",
                    
                resume.evidence_refs.clone(),
                resume.what_changed.clone(),
                resume.project_id.clone(),
                resume.task_id.clone(),
            )?));
        }
        if let Some(focus) = focus {
            return Ok(Some(ContinuityFacet::project(
                ws,
                ContinuityFacetKind::SuggestedNextStep,
                "workflow_context",
                format!("next:{}", focus.source_id),
                format!("Continue current focus"),
                focus.summary.clone(),
                "Suggested because Current Focus is set.",
                focus.evidence_refs.clone(),
                focus.what_changed.clone(),
                focus.project_id.clone(),
                focus.task_id.clone(),
            )?));
        }
        Ok(None)
    }

    fn build_summary(
        focus: &Option<ContinuityFacet>,
        outstanding: usize,
        interrupted: usize,
        resumable: usize,
        blockers: usize,
        next: &Option<ContinuityFacet>,
    ) -> String {
        let focus_label = focus
            .as_ref()
            .map(|f| f.title.as_str())
            .unwrap_or("no current focus");
        let next_label = next
            .as_ref()
            .map(|n| n.title.as_str())
            .unwrap_or("nothing suggested");
        format!(
            "Continuity — {focus_label}. {outstanding} outstanding decision(s), \
             {interrupted} interrupted, {resumable} resumable, {blockers} blocker(s). \
             Suggested next: {next_label}. Continuity is informational only."
        )
    }

    /// Architecture guard — Continuity must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceContinuityError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceContinuityState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.continuity.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "outstanding_decisions": state.outstanding_decisions.len(),
                "interrupted": state.interrupted_work.len(),
                "resumable": state.resumable_work.len(),
                "blockers": state.blockers.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.continuity.summary.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "summary_len": state.summary.len(),
                "has_focus": state.current_focus.is_some(),
                "has_next": state.suggested_next_step.is_some(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
