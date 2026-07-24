//! Workspace Evolution Model (Phase 5).
//!
//! Aggregates Activity Graph + Task Graph + Purpose + Composition + Continuity +
//! Decision Queue into how work changed. Never persists evolution rows, never
//! predicts, never executes. Activity Graph remains history SoT.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_evolution_summary, evolution_now_rfc3339, validate_evolution_workspace_id, ActorContext,
    DecisionQueue, EvolutionEvent, EvolutionInsight, EvolutionInsightKind, EvolutionRelationship,
    EvolutionSourceModel, IntentContext, TaskGraph, WorkspaceActivityGraph,
    WorkspaceCompositionState, WorkspaceContinuityState, WorkspaceEvolutionState,
    WorkspaceEvolutionSummary, WorkspacePurposeState, WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceCompositionService,
    WorkspaceContinuityService, WorkspaceEnvironmentService, WorkspaceIntentService,
    WorkspacePurposeService,
};

pub(crate) struct WorkspaceEvolutionService;

impl WorkspaceEvolutionService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceEvolutionState> {
        let workspace_id = workspace_id.into();
        let purpose = WorkspacePurposeService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| WorkspaceIntentService::get_project(db, id.as_str()).ok());
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let decision_queue = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let activity = WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            Some(&decision_queue),
        )?;
        let continuity = WorkspaceContinuityService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &decision_queue,
            &activity,
        )?;
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
        let composition = WorkspaceCompositionService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &environment,
            Some(&task_graph),
            &continuity,
            &activity,
            &workflow,
            &decision_queue,
            project.as_ref(),
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &activity,
            Some(&task_graph),
            &purpose,
            &composition,
            &continuity,
            &decision_queue,
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        activity: &WorkspaceActivityGraph,
        task_graph: Option<&TaskGraph>,
        purpose: &WorkspacePurposeState,
        composition: &WorkspaceCompositionState,
        continuity: &WorkspaceContinuityState,
        decision_queue: &DecisionQueue,
    ) -> Result<WorkspaceEvolutionState> {
        let workspace_id =
            validate_evolution_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let now = evolution_now_rfc3339();
        let label = if !purpose.label.is_empty() {
            purpose.label.clone()
        } else {
            composition.label.clone()
        };

        let mut events = Vec::new();
        let mut insights = Vec::new();
        let mut relationships = Vec::new();
        let mut evidence = Vec::new();

        // Activity timeline backbone (skip AuditSignal — Activity Graph remains SoT).
        for activity_item in activity.timeline.iter().rev().take(20) {
            if matches!(
                activity_item.activity_type,
                workspace_domain::ActivityType::AuditSignal
            ) {
                continue;
            }
            let eid = format!("evolution:activity:{}", activity_item.id);
            events.push(EvolutionEvent {
                id: eid.clone(),
                kind: activity_item.activity_type.as_str().into(),
                ref_id: activity_item.id.to_string(),
                source_model: EvolutionSourceModel::Activity,
                title: activity_item.summary.clone(),
                change: activity_item.summary.clone(),
                evidence: vec![
                    format!("type={}", activity_item.activity_type.as_str()),
                    activity_item.explanation.clone(),
                    "source Activity Graph timeline".into(),
                ],
                impact: "Recorded work history that shaped the current Workspace state.".into(),
                timestamp: activity_item.timestamp.clone(),
                authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        evidence.push(format!(
            "Activity Graph: {} timeline item(s) considered (audits skipped)",
            events.len()
        ));

        // Task Graph progression.
        if let Some(graph) = task_graph {
            let mut related = Vec::new();
            for node in graph
                .nodes
                .iter()
                .filter(|n| n.task.status == WorkspaceTaskStatus::Completed)
                .take(8)
            {
                let eid = format!("evolution:task_completed:{}", node.task.id);
                related.push(eid.clone());
                events.push(EvolutionEvent {
                    id: eid,
                    kind: EvolutionInsightKind::TaskProgression.as_str().into(),
                    ref_id: node.task.id.to_string(),
                    source_model: EvolutionSourceModel::TaskGraph,
                    title: format!("Completed: {}", node.task.title),
                    change: format!("Task \"{}\" reached completed status.", node.task.title),
                    evidence: vec![
                        format!("status={}", node.task.status.as_str()),
                        "source Task Graph".into(),
                    ],
                    impact: "Completed work advances Purpose and reduces open load.".into(),
                    timestamp: now.clone(),
                    authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
                });
            }
            for node in graph.open_incomplete_nodes().into_iter().take(6) {
                let eid = format!("evolution:task_open:{}", node.task.id);
                events.push(EvolutionEvent {
                    id: eid.clone(),
                    kind: "task_open".into(),
                    ref_id: node.task.id.to_string(),
                    source_model: EvolutionSourceModel::TaskGraph,
                    title: format!("Open: {}", node.task.title),
                    change: format!(
                        "Task \"{}\" remains {}.",
                        node.task.title,
                        node.task.status.as_str()
                    ),
                    evidence: vec![
                        format!("status={}", node.task.status.as_str()),
                        "source Task Graph open nodes".into(),
                    ],
                    impact: "Open work still shapes current Attention and Purpose progress.".into(),
                    timestamp: now.clone(),
                    authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
                });
                if node.task.status == WorkspaceTaskStatus::Blocked {
                    related.push(eid);
                }
            }
            if graph.completed_count > 0 || !related.is_empty() {
                insights.push(EvolutionInsight {
                    id: format!("insight:task_progression:{ws}"),
                    kind: EvolutionInsightKind::TaskProgression,
                    title: format!(
                        "Task Graph progressed — {} completed, {} open",
                        graph.completed_count,
                        graph.open_incomplete_nodes().len()
                    ),
                    explanation: format!(
                        "Task Graph shows {}% progress. Completed and open nodes explain how \
                         work evolved without creating a second history store.",
                        graph.progress_percent
                    ),
                    evidence: vec![
                        format!("completed={}", graph.completed_count),
                        format!("blocked={}", graph.blocked_count),
                        "source Task Graph".into(),
                    ],
                    related_event_ids: related,
                });
            }
            evidence.push(format!(
                "Task Graph: {}% · {} completed · {} blocked",
                graph.progress_percent, graph.completed_count, graph.blocked_count
            ));
        }

        // Purpose progression.
        if purpose.progress_percent > 0
            || !purpose.recent_progress.is_empty()
            || purpose.primary_work_goal_id.is_some()
        {
            let eid = format!("evolution:purpose:{}", purpose.workspace_id);
            events.push(EvolutionEvent {
                id: eid.clone(),
                kind: EvolutionInsightKind::PurposeProgression.as_str().into(),
                ref_id: purpose.workspace_id.clone(),
                source_model: EvolutionSourceModel::Purpose,
                title: format!("Purpose: {}", purpose.label),
                change: format!(
                    "Purpose \"{}\" is at {}% with {} open task(s).",
                    purpose.label, purpose.progress_percent, purpose.open_task_count
                ),
                evidence: {
                    let mut e = purpose.recent_progress.clone();
                    e.push("source Purpose Model".into());
                    e
                },
                impact: "Purpose progression frames why recent Task Graph and Continuity changes matter."
                    .into(),
                timestamp: purpose.generated_at.clone(),
                authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
            });
            insights.push(EvolutionInsight {
                id: format!("insight:purpose_progression:{ws}"),
                kind: EvolutionInsightKind::PurposeProgression,
                title: format!("Working toward \"{}\"", purpose.label),
                explanation: purpose.explanation.clone(),
                evidence: purpose.evidence.clone(),
                related_event_ids: vec![eid],
            });
            evidence.push(format!(
                "Purpose: {} ({}%)",
                purpose.label, purpose.progress_percent
            ));
        }

        // Composition shifts.
        if composition.missing_application_count > 0 || !composition.gaps.is_empty() {
            let eid = format!("evolution:composition:{}", composition.workspace_id);
            events.push(EvolutionEvent {
                id: eid.clone(),
                kind: EvolutionInsightKind::CompositionShift.as_str().into(),
                ref_id: composition.workspace_id.clone(),
                source_model: EvolutionSourceModel::Composition,
                title: format!("Composition: {}", composition.label),
                change: format!(
                    "Working environment \"{}\" has {} present and {} missing application(s).",
                    composition.label,
                    composition.present_application_count,
                    composition.missing_application_count
                ),
                evidence: composition
                    .gaps
                    .iter()
                    .take(5)
                    .map(|g| g.title.clone())
                    .chain(std::iter::once("source Composition Engine".into()))
                    .collect(),
                impact: "Composition gaps affect how Purpose can be realized on the desktop.".into(),
                timestamp: composition.generated_at.clone(),
                authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
            });
            insights.push(EvolutionInsight {
                id: format!("insight:composition_shift:{ws}"),
                kind: EvolutionInsightKind::CompositionShift,
                title: format!(
                    "Environment \"{}\" is incomplete for current work",
                    composition.label
                ),
                explanation: composition.explanation.clone(),
                evidence: vec![composition.summary.clone()],
                related_event_ids: vec![eid],
            });
        } else if composition.present_application_count > 0 {
            insights.push(EvolutionInsight {
                id: format!("insight:composition_stable:{ws}"),
                kind: EvolutionInsightKind::CompositionShift,
                title: format!(
                    "\"{}\" is the active working environment",
                    composition.label
                ),
                explanation: format!(
                    "Composition reports {} present application(s) with no missing registered apps.",
                    composition.present_application_count
                ),
                evidence: vec![composition.summary.clone()],
                related_event_ids: Vec::new(),
            });
        }
        evidence.push(format!("Composition: {}", composition.label));

        // Continuity interrupted / focus.
        if let Some(focus) = &continuity.current_focus {
            let eid = format!("evolution:focus:{}", focus.id);
            events.push(EvolutionEvent {
                id: eid.clone(),
                kind: EvolutionInsightKind::FocusChange.as_str().into(),
                ref_id: focus.id.to_string(),
                source_model: EvolutionSourceModel::Continuity,
                title: focus.title.clone(),
                change: focus.what_changed.clone(),
                evidence: {
                    let mut refs = vec!["source Continuity Engine".into()];
                    refs.extend(focus.evidence_refs.iter().cloned());
                    refs
                },
                impact: "Current focus explains where Purpose work is situated now.".into(),
                timestamp: now.clone(),
                authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
            });
            insights.push(EvolutionInsight {
                id: format!("insight:focus_change:{ws}"),
                kind: EvolutionInsightKind::FocusChange,
                title: format!("Focus: {}", focus.title),
                explanation: focus.why.clone(),
                evidence: vec![focus.summary.clone()],
                related_event_ids: vec![eid],
            });
        }
        if !continuity.interrupted_work.is_empty() {
            let related: Vec<_> = continuity
                .interrupted_work
                .iter()
                .take(5)
                .map(|f| {
                    let eid = format!("evolution:interrupted:{}", f.id);
                    events.push(EvolutionEvent {
                        id: eid.clone(),
                        kind: EvolutionInsightKind::InterruptedWork.as_str().into(),
                        ref_id: f.id.to_string(),
                        source_model: EvolutionSourceModel::Continuity,
                        title: f.title.clone(),
                        change: format!("Work was interrupted: {}", f.title),
                        evidence: vec![f.why.clone(), "source Continuity interrupted_work".into()],
                        impact: "Interrupted work remains part of how the current state emerged."
                            .into(),
                        timestamp: now.clone(),
                        authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
                    });
                    eid
                })
                .collect();
            insights.push(EvolutionInsight {
                id: format!("insight:interrupted_work:{ws}"),
                kind: EvolutionInsightKind::InterruptedWork,
                title: format!(
                    "{} interrupted work item(s)",
                    continuity.interrupted_work.len()
                ),
                explanation: "Continuity interrupted facets show where work was disrupted over time."
                    .into(),
                evidence: continuity
                    .interrupted_work
                    .iter()
                    .take(3)
                    .map(|f| f.title.clone())
                    .collect(),
                related_event_ids: related,
            });
        }

        // Decision outcomes / pressure.
        let decision_activities: Vec<_> = activity
            .timeline
            .iter()
            .filter(|a| {
                matches!(
                    a.activity_type,
                    workspace_domain::ActivityType::DecisionItem
                )
            })
            .rev()
            .take(5)
            .collect();
        if !decision_activities.is_empty() {
            let related: Vec<_> = decision_activities
                .iter()
                .map(|a| {
                    let eid = format!("evolution:decision:{}", a.id);
                    events.push(EvolutionEvent {
                        id: eid.clone(),
                        kind: EvolutionInsightKind::DecisionOutcome.as_str().into(),
                        ref_id: a.id.to_string(),
                        source_model: EvolutionSourceModel::DecisionQueue,
                        title: a.summary.clone(),
                        change: a.summary.clone(),
                        evidence: vec![
                            a.explanation.clone(),
                            "source Activity Graph DecisionItem".into(),
                        ],
                        impact: "Decision activity explains how human choices shaped current work."
                            .into(),
                        timestamp: a.timestamp.clone(),
                        authority_effect: EvolutionEvent::AUTHORITY_EFFECT_NONE.into(),
                    });
                    eid
                })
                .collect();
            insights.push(EvolutionInsight {
                id: format!("insight:decision_outcome:{ws}"),
                kind: EvolutionInsightKind::DecisionOutcome,
                title: format!(
                    "{} decision-related change(s) on the Activity Graph",
                    decision_activities.len()
                ),
                explanation: "Decision outcomes appear via Activity Graph — Evolution does not own Decision Queue history."
                    .into(),
                evidence: vec![format!(
                    "Decision Queue pending now: {}",
                    decision_queue.pending_count
                )],
                related_event_ids: related,
            });
        } else if decision_queue.pending_count > 0 {
            insights.push(EvolutionInsight {
                id: format!("insight:decision_pressure:{ws}"),
                kind: EvolutionInsightKind::DecisionOutcome,
                title: format!(
                    "{} outstanding decision(s) awaiting attention",
                    decision_queue.pending_count
                ),
                explanation: "Pending Decision Queue items are current pressure on how work can evolve next — not predictions."
                    .into(),
                evidence: vec!["source Decision Queue pending_count".into()],
                related_event_ids: Vec::new(),
            });
        }
        evidence.push(format!(
            "Decision Queue pending: {}",
            decision_queue.pending_count
        ));

        // Relationships: purpose event ↔ task progression / composition.
        if let Some(purpose_event) = events
            .iter()
            .find(|e| e.source_model == EvolutionSourceModel::Purpose)
        {
            if let Some(task_insight) = insights
                .iter()
                .find(|i| i.kind == EvolutionInsightKind::TaskProgression)
            {
                relationships.push(EvolutionRelationship {
                    id: format!("rel:purpose-tasks:{ws}"),
                    from_id: purpose_event.id.clone(),
                    to_id: task_insight.id.clone(),
                    kind: "advanced_by".into(),
                    explanation: "Purpose progression is advanced by Task Graph completed work."
                        .into(),
                    evidence: vec!["Purpose Model + Task Graph".into()],
                });
            }
            if let Some(comp_event) = events
                .iter()
                .find(|e| e.source_model == EvolutionSourceModel::Composition)
            {
                relationships.push(EvolutionRelationship {
                    id: format!("rel:purpose-composition:{ws}"),
                    from_id: purpose_event.id.clone(),
                    to_id: comp_event.id.clone(),
                    kind: "realized_in".into(),
                    explanation: "Purpose is realized through the evolving working environment."
                        .into(),
                    evidence: vec!["Purpose Model + Composition Engine".into()],
                });
            }
        }

        // Stable sort for determinism: events by timestamp then id.
        events.sort_by(|a, b| {
            a.timestamp
                .cmp(&b.timestamp)
                .then_with(|| a.id.cmp(&b.id))
        });
        insights.sort_by(|a, b| a.id.cmp(&b.id));
        relationships.sort_by(|a, b| a.id.cmp(&b.id));

        let explanation = format!(
            "Evolution of \"{label}\" explains how the current Workspace state emerged from \
             Activity Graph history, Task Graph progression, Purpose, Composition, Continuity, \
             and decisions. It does not store a second history and does not predict the future."
        );
        let summary = build_evolution_summary(&label, events.len(), insights.len());

        let state = WorkspaceEvolutionState {
            workspace_id: ws.to_string(),
            generated_at: now,
            label,
            event_count: events.len(),
            insight_count: insights.len(),
            relationship_count: relationships.len(),
            events,
            insights,
            relationships,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceEvolutionState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceEvolutionState,
        limit: usize,
    ) -> WorkspaceEvolutionSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceEvolutionError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceEvolutionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.evolution.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "event_count": state.event_count,
                "insight_count": state.insight_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
