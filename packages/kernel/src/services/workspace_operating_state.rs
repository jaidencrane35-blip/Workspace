//! Workspace Operating State (Phase 5).
//!
//! Aggregates existing understanding systems into a unified current-situation
//! snapshot. Never executes, never persists, never grants authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_operating_state_summary, operating_state_now_rfc3339, validate_operating_state_workspace_id,
    ActorContext, AttentionCategory, DecisionQueue, DecisionState, IntentContext, OperatingContext,
    OperatingRelationship, OperatingSignal, OperatingSignalKind, OperatingSummary, Project, Task,
    TaskGraph, WorkflowContext, WorkspaceActivityGraph, WorkspaceAttentionState,
    WorkspaceCompositionState, WorkspaceContinuityState, WorkspaceEnvironmentState,
    WorkspaceEvolutionState, WorkspaceOperatingState, WorkspaceOperatingStateSummary,
    WorkspacePurposeState, WorkspaceRecommendationEngineState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceAttentionService,
    WorkspaceCompositionService, WorkspaceContinuityService, WorkspaceEnvironmentService,
    WorkspaceEvolutionService, WorkspaceIntentService, WorkspacePurposeService,
    WorkspaceRecommendationEngineService,
};

pub(crate) struct WorkspaceOperatingStateService;

impl WorkspaceOperatingStateService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceOperatingState> {
        let workspace_id = workspace_id.into();
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
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| WorkspaceIntentService::get_project(db, id.as_str()).ok());
        let task = workflow
            .active_task_id
            .as_ref()
            .and_then(|id| WorkspaceIntentService::get_task(db, id.as_str()).ok());
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
        let goals =
            WorkspaceIntentService::list_goals(db, &workspace_id, 20).unwrap_or_default();
        let purpose = WorkspacePurposeService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &goals,
            &workflow,
            project.as_ref(),
            Some(&task_graph),
            &composition,
            &continuity,
            &activity,
            &decision_queue,
        )?;
        let evolution = WorkspaceEvolutionService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &activity,
            Some(&task_graph),
            &purpose,
            &composition,
            &continuity,
            &decision_queue,
        )?;
        let base_attention = WorkspaceAttentionService::generate_with_task_graph(
            db,
            actor,
            workspace_id.clone(),
            &decision_queue,
            &activity,
            &continuity,
            Some(&task_graph),
            Some(&environment),
            Some(&composition),
            Some(&purpose),
            Some(&evolution),
        )?;
        let recommendations = WorkspaceRecommendationEngineService::seal_consumer_projection(
            WorkspaceRecommendationEngineService::generate_with_inputs(
                db,
                actor,
                workspace_id.clone(),
                &base_attention,
                &continuity,
                &evolution,
                &purpose,
                Some(&task_graph),
                &composition,
                &decision_queue,
                &environment,
            )?,
        );
        let attention = WorkspaceAttentionService::enrich_with_recommendations(
            &base_attention,
            &recommendations,
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &workflow,
            project.as_ref(),
            task.as_ref(),
            &purpose,
            &environment,
            &composition,
            Some(&task_graph),
            &continuity,
            &activity,
            &decision_queue,
            &attention,
            &recommendations,
            &evolution,
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        workflow: &WorkflowContext,
        project: Option<&Project>,
        task: Option<&Task>,
        purpose: &WorkspacePurposeState,
        environment: &WorkspaceEnvironmentState,
        composition: &WorkspaceCompositionState,
        task_graph: Option<&TaskGraph>,
        continuity: &WorkspaceContinuityState,
        activity: &WorkspaceActivityGraph,
        decision_queue: &DecisionQueue,
        attention: &WorkspaceAttentionState,
        recommendations: &WorkspaceRecommendationEngineState,
        evolution: &WorkspaceEvolutionState,
    ) -> Result<WorkspaceOperatingState> {
        let workspace_id =
            validate_operating_state_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();

        let mut signals = Vec::new();
        let mut relationships = Vec::new();
        let mut evidence = Vec::new();

        let purpose_label = if purpose.label.is_empty() {
            composition.label.clone()
        } else {
            purpose.label.clone()
        };

        // Purpose
        signals.push(OperatingSignal {
            id: format!("os:purpose:{ws}"),
            kind: OperatingSignalKind::Purpose,
            current_value: purpose_label.clone(),
            source_model: "purpose".into(),
            source_ref: purpose.workspace_id.clone(),
            evidence: vec![purpose.summary.clone()],
            authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
        });

        // Active project / work
        let active_project_label = project.map(|p| p.name.clone());
        if let Some(p) = project {
            signals.push(OperatingSignal {
                id: format!("os:project:{}", p.id),
                kind: OperatingSignalKind::Project,
                current_value: p.name.clone(),
                source_model: "workflow_context".into(),
                source_ref: p.id.to_string(),
                evidence: vec!["WorkflowContext.active_project_id".into()],
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
            relationships.push(OperatingRelationship {
                id: format!("rel:purpose-project:{}", p.id),
                from_id: format!("os:purpose:{ws}"),
                to_id: format!("os:project:{}", p.id),
                kind: "supports".into(),
                explanation: format!(
                    "Active project \"{}\" belongs under Purpose \"{}\".",
                    p.name, purpose_label
                ),
                evidence: vec!["source Purpose".into(), "source WorkflowContext".into()],
            });
        }
        let active_task_label = task.map(|t| t.title.clone());
        if let Some(t) = task {
            signals.push(OperatingSignal {
                id: format!("os:task:{}", t.id),
                kind: OperatingSignalKind::ActiveWork,
                current_value: t.title.clone(),
                source_model: "workflow_context".into(),
                source_ref: t.id.to_string(),
                evidence: vec!["WorkflowContext.active_task_id".into()],
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Environment + Composition
        signals.push(OperatingSignal {
            id: format!("os:environment:{ws}"),
            kind: OperatingSignalKind::Environment,
            current_value: environment.summary.clone(),
            source_model: "environment".into(),
            source_ref: environment.workspace_id.clone(),
            evidence: vec![format!(
                "apps={} disconnected={}",
                environment.applications.len(),
                environment.disconnected_work
            )],
            authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
        });
        signals.push(OperatingSignal {
            id: format!("os:composition:{ws}"),
            kind: OperatingSignalKind::Composition,
            current_value: composition.label.clone(),
            source_model: "composition".into(),
            source_ref: composition.workspace_id.clone(),
            evidence: vec![composition.summary.clone()],
            authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
        });
        relationships.push(OperatingRelationship {
            id: format!("rel:env-composition:{ws}"),
            from_id: format!("os:environment:{ws}"),
            to_id: format!("os:composition:{ws}"),
            kind: "composes".into(),
            explanation: "Composition organizes Environment resources into a working set.".into(),
            evidence: vec!["source Environment".into(), "source Composition".into()],
        });

        // Progress from Evolution + Purpose recent progress
        let mut recent_progress = purpose.recent_progress.clone();
        for insight in evolution.insights.iter().take(3) {
            let line = format!("{}: {}", insight.kind.as_str(), insight.title);
            if !recent_progress.iter().any(|p| p == &line) {
                recent_progress.push(line.clone());
            }
            signals.push(OperatingSignal {
                id: format!("os:progress:{}", insight.id),
                kind: OperatingSignalKind::Progress,
                current_value: insight.title.clone(),
                source_model: "evolution".into(),
                source_ref: insight.id.clone(),
                evidence: insight.evidence.clone(),
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if let Some(graph) = task_graph {
            let open = graph.open_incomplete_nodes().len();
            let completed = graph
                .nodes
                .iter()
                .filter(|n| {
                    matches!(
                        n.task.status,
                        workspace_domain::WorkspaceTaskStatus::Completed
                    )
                })
                .count();
            signals.push(OperatingSignal {
                id: format!("os:task_graph:{ws}"),
                kind: OperatingSignalKind::Progress,
                current_value: format!("{completed} completed · {open} open Task Graph node(s)"),
                source_model: "task_graph".into(),
                source_ref: ws.to_string(),
                evidence: vec![format!("node_count={}", graph.nodes.len())],
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Blockers from Attention
        let mut current_blockers = Vec::new();
        for item in attention
            .items
            .iter()
            .filter(|i| i.category == AttentionCategory::Blocker)
            .take(5)
        {
            current_blockers.push(item.title.clone());
            signals.push(OperatingSignal {
                id: format!("os:blocker:{}", item.id),
                kind: OperatingSignalKind::Blocker,
                current_value: item.title.clone(),
                source_model: "attention".into(),
                source_ref: item.id.to_string(),
                evidence: vec![item.explanation.clone()],
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Pending decisions
        let mut pending_decisions = Vec::new();
        for item in decision_queue.items.iter().take(5) {
            if matches!(
                item.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            ) {
                pending_decisions.push(item.title.clone());
                signals.push(OperatingSignal {
                    id: format!("os:decision:{}", item.id),
                    kind: OperatingSignalKind::PendingDecision,
                    current_value: item.title.clone(),
                    source_model: "decision_queue".into(),
                    source_ref: item.id.to_string(),
                    evidence: vec![item.explanation.clone()],
                    authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }
        if pending_decisions.is_empty() && decision_queue.pending_count > 0 {
            pending_decisions.push(format!(
                "{} pending Decision Queue item(s)",
                decision_queue.pending_count
            ));
        }

        // Recommendations
        let mut top_recommendations = Vec::new();
        for candidate in recommendations.candidates.iter().take(5) {
            top_recommendations.push(candidate.title.clone());
            signals.push(OperatingSignal {
                id: format!("os:rec:{}", candidate.id),
                kind: OperatingSignalKind::Recommendation,
                current_value: candidate.title.clone(),
                source_model: "recommendation_engine".into(),
                source_ref: candidate.id.clone(),
                evidence: candidate
                    .evidence
                    .iter()
                    .map(|e| e.summary.clone())
                    .collect(),
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Attention priorities
        let mut attention_priorities = Vec::new();
        for item in attention.top_items.iter().take(5) {
            attention_priorities.push(item.title.clone());
            signals.push(OperatingSignal {
                id: format!("os:attention:{}", item.id),
                kind: OperatingSignalKind::Attention,
                current_value: item.title.clone(),
                source_model: "attention".into(),
                source_ref: item.id.to_string(),
                // Sprint 130: keep score_factors as arithmetic evidence. Structured
                // AttentionReason translation belongs to Experience rendering, not here —
                // replacing this with DisplayReason titles would lose the score trail.
                evidence: item.score_factors.clone(),
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Continuity focus
        let continuity_focus = continuity
            .current_focus
            .as_ref()
            .map(|f| f.title.clone())
            .or_else(|| {
                continuity
                    .suggested_next_step
                    .as_ref()
                    .map(|s| s.title.clone())
            });
        if let Some(ref focus) = continuity_focus {
            signals.push(OperatingSignal {
                id: format!("os:continuity:{ws}"),
                kind: OperatingSignalKind::Continuity,
                current_value: focus.clone(),
                source_model: "continuity".into(),
                source_ref: ws.to_string(),
                evidence: vec![continuity.summary.clone()],
                authority_effect: OperatingSignal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Activity evidence (count only — Activity Graph remains SoT)
        evidence.push(format!("Activity Graph activities: {}", activity.activities.len()));
        evidence.push(format!("Purpose: {}", purpose_label));
        evidence.push(format!(
            "Decision Queue pending: {}",
            decision_queue.pending_count
        ));
        evidence.push(format!(
            "Recommendation candidates: {}",
            recommendations.candidate_count
        ));
        evidence.push(format!("Attention items: {}", attention.items.len()));
        let _ = workflow; // workflow already consumed via project/task

        signals.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        relationships.sort_by(|a, b| a.id.cmp(&b.id));

        let context = OperatingContext {
            purpose_label: purpose_label.clone(),
            active_project_label: active_project_label.clone(),
            active_task_label: active_task_label.clone(),
            environment_summary: environment.summary.clone(),
            composition_label: composition.label.clone(),
            recent_progress: recent_progress.clone(),
            current_blockers: current_blockers.clone(),
            pending_decisions: pending_decisions.clone(),
            top_recommendations: top_recommendations.clone(),
            attention_priorities: attention_priorities.clone(),
            continuity_focus: continuity_focus.clone(),
        };

        let progress_line = if recent_progress.is_empty() {
            "No recent progress signals.".into()
        } else {
            recent_progress
                .first()
                .cloned()
                .unwrap_or_else(|| "Progress underway.".into())
        };
        let pending_line = if pending_decisions.is_empty() {
            "No pending decisions.".into()
        } else {
            format!("Pending: {}", pending_decisions[0])
        };
        let suggested_line = if top_recommendations.is_empty() {
            "No recommendations right now.".into()
        } else {
            format!("Suggested: {}", top_recommendations[0])
        };
        let project_bit = active_project_label
            .as_deref()
            .unwrap_or("No active project");
        let operating_summary = OperatingSummary {
            headline: format!("Working on \"{purpose_label}\""),
            purpose_line: format!("Current Purpose: {purpose_label}"),
            environment_line: format!(
                "Current Environment: {} · Composition: {}",
                environment.summary, composition.label
            ),
            progress_line: format!("Progress: {progress_line}"),
            pending_line: pending_line.clone(),
            suggested_line: suggested_line.clone(),
            narrative: format!(
                "You are working toward \"{purpose_label}\" in project \"{project_bit}\". \
                 {pending_line} {suggested_line}"
            ),
        };

        let explanation = format!(
            "Operating State unifies Purpose, Workflow, Environment, Composition, Task Graph, \
             Evolution, Continuity, Activity, Decision Queue, Attention, and Recommendation Engine \
             into one current-situation snapshot. Sources remain authoritative; this layer aggregates only."
        );
        let summary = build_operating_state_summary(&purpose_label, signals.len());

        let state = WorkspaceOperatingState {
            workspace_id: ws.to_string(),
            generated_at: operating_state_now_rfc3339(),
            context,
            signal_count: signals.len(),
            relationship_count: relationships.len(),
            signals,
            relationships,
            operating_summary,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceOperatingState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceOperatingState,
        limit: usize,
    ) -> WorkspaceOperatingStateSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceOperatingStateError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceOperatingState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.operating_state.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "signal_count": state.signal_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceOperatingState,
    ) -> Result<()> {
        // Ephemeral refresh marker — OS does not persist; each generate refreshes the snapshot.
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.operating_state.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "signal_count": state.signal_count,
                "purpose_label": state.context.purpose_label,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn kind_rank(kind: OperatingSignalKind) -> u8 {
    match kind {
        OperatingSignalKind::Purpose => 0,
        OperatingSignalKind::Project => 1,
        OperatingSignalKind::ActiveWork => 2,
        OperatingSignalKind::Environment => 3,
        OperatingSignalKind::Composition => 4,
        OperatingSignalKind::Progress => 5,
        OperatingSignalKind::Blocker => 6,
        OperatingSignalKind::PendingDecision => 7,
        OperatingSignalKind::Recommendation => 8,
        OperatingSignalKind::Attention => 9,
        OperatingSignalKind::Continuity => 10,
        OperatingSignalKind::Evolution => 11,
    }
}
