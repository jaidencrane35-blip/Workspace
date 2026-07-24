//! Workspace Pattern Model (Phase 5).
//!
//! Aggregates Activity, Evolution, Operating State, Composition, Task Graph,
//! Environment, Purpose, Continuity, and Decision Queue into recurring-structure
//! observations. Never executes, never profiles users, never persists patterns.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_pattern_model_summary, pattern_now_rfc3339, validate_pattern_workspace_id, ActivityType,
    ActorContext, CompositionMemberKind, DecisionQueue, IntentContext, PatternConfidence,
    PatternEvidence, PatternKind, PatternRelationship, PatternSummary, TaskGraph,
    TaskRelationshipKind, WorkspaceActivityGraph, WorkspaceCompositionState,
    WorkspaceContinuityState, WorkspaceEnvironmentState, WorkspaceEvolutionState,
    WorkspaceOperatingState, WorkspacePattern, WorkspacePatternState, WorkspacePatternSummary,
    WorkspacePurposeState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceCompositionService,
    WorkspaceContinuityService, WorkspaceEnvironmentService, WorkspaceEvolutionService,
    WorkspaceIntentService, WorkspaceOperatingStateService, WorkspacePurposeService,
};

pub(crate) struct WorkspacePatternService;

impl WorkspacePatternService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspacePatternState> {
        let workspace_id = workspace_id.into();
        let operating = WorkspaceOperatingStateService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let purpose = WorkspacePurposeService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let evolution = WorkspaceEvolutionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
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
            &evolution,
            &operating,
            &composition,
            Some(&task_graph),
            &environment,
            &purpose,
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
        evolution: &WorkspaceEvolutionState,
        operating: &WorkspaceOperatingState,
        composition: &WorkspaceCompositionState,
        task_graph: Option<&TaskGraph>,
        environment: &WorkspaceEnvironmentState,
        purpose: &WorkspacePurposeState,
        continuity: &WorkspaceContinuityState,
        decision_queue: &DecisionQueue,
    ) -> Result<WorkspacePatternState> {
        let workspace_id =
            validate_pattern_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let label = if !purpose.label.is_empty() {
            purpose.label.clone()
        } else if !operating.context.purpose_label.is_empty() {
            operating.context.purpose_label.clone()
        } else {
            composition.label.clone()
        };

        let mut patterns = Vec::new();
        let mut relationships = Vec::new();
        let mut evidence = Vec::new();
        let mut seen = HashSet::new();

        // ApplicationPattern — Composition applications that co-appear.
        let app_members: Vec<_> = composition
            .members
            .iter()
            .filter(|m| m.kind == CompositionMemberKind::Application)
            .take(6)
            .collect();
        if app_members.len() >= 2 {
            let names: Vec<_> = app_members.iter().map(|m| m.label.as_str()).collect();
            let id = format!("pattern:application:{ws}");
            if seen.insert(id.clone()) {
                let observation = format!(
                    "{} commonly appear together in composition \"{}\".",
                    names.join(" and "),
                    composition.label
                );
                patterns.push(WorkspacePattern {
                    id: id.clone(),
                    kind: PatternKind::ApplicationPattern,
                    title: format!("Application group: {}", composition.label),
                    observation: observation.clone(),
                    evidence: vec![
                        PatternEvidence {
                            id: format!("ev:composition:{ws}"),
                            source_model: "composition".into(),
                            source_ref: composition.workspace_id.clone(),
                            summary: composition.summary.clone(),
                        },
                        PatternEvidence {
                            id: format!("ev:environment:{ws}"),
                            source_model: "environment".into(),
                            source_ref: environment.workspace_id.clone(),
                            summary: environment.summary.clone(),
                        },
                    ],
                    confidence: if app_members.len() >= 3 {
                        PatternConfidence::High
                    } else {
                        PatternConfidence::Medium
                    },
                    impact: "Knowing co-appearing apps helps restore a familiar working set."
                        .into(),
                    authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                });
                relationships.push(PatternRelationship {
                    id: format!("rel:pattern-composition:{ws}"),
                    from_id: id,
                    to_id: format!("composition:{ws}"),
                    kind: "observed_in".into(),
                    explanation: "Application co-occurrence is projected from Composition members."
                        .into(),
                    evidence: vec!["source Composition".into()],
                });
            }
        }

        // EnvironmentPattern — composition/environment tied to purpose/project.
        if !composition.label.is_empty() {
            let id = format!("pattern:environment:{ws}");
            if seen.insert(id.clone()) {
                let project_bit = operating
                    .context
                    .active_project_label
                    .clone()
                    .unwrap_or_else(|| "this workspace".into());
                patterns.push(WorkspacePattern {
                    id: id.clone(),
                    kind: PatternKind::EnvironmentPattern,
                    title: format!("Environment linked to {}", project_bit),
                    observation: format!(
                        "Working environment \"{}\" is associated with \"{}\" / Purpose \"{}\".",
                        composition.label, project_bit, label
                    ),
                    evidence: vec![
                        PatternEvidence {
                            id: format!("ev:operating:{ws}"),
                            source_model: "operating_state".into(),
                            source_ref: operating.workspace_id.clone(),
                            summary: operating.summary.clone(),
                        },
                        PatternEvidence {
                            id: format!("ev:composition-env:{ws}"),
                            source_model: "composition".into(),
                            source_ref: composition.workspace_id.clone(),
                            summary: composition.label.clone(),
                        },
                    ],
                    confidence: PatternConfidence::Medium,
                    impact: "Environment associations help explain where work usually happens."
                        .into(),
                    authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // WorkflowPattern — planning before implementation from Activity / Continuity.
        let has_planning = activity.activities.iter().any(|a| {
            matches!(
                a.activity_type,
                ActivityType::PlanningContinuation | ActivityType::IntentProposal
            )
        });
        let has_task_work = activity
            .activities
            .iter()
            .any(|a| matches!(a.activity_type, ActivityType::Task | ActivityType::WorkGoal));
        if has_planning && has_task_work {
            let id = format!("pattern:workflow:planning:{ws}");
            if seen.insert(id.clone()) {
                patterns.push(WorkspacePattern {
                    id: id.clone(),
                    kind: PatternKind::WorkflowPattern,
                    title: "Planning precedes implementation".into(),
                    observation: "Planning or intent activity appears alongside task/work-goal activity in this workspace."
                        .into(),
                    evidence: vec![
                        PatternEvidence {
                            id: format!("ev:activity:planning:{ws}"),
                            source_model: "activity".into(),
                            source_ref: ws.to_string(),
                            summary: format!(
                                "activity_count={} (Activity Graph remains history SoT)",
                                activity.activities.len()
                            ),
                        },
                        PatternEvidence {
                            id: format!("ev:continuity:{ws}"),
                            source_model: "continuity".into(),
                            source_ref: ws.to_string(),
                            summary: continuity.summary.clone(),
                        },
                    ],
                    confidence: PatternConfidence::Medium,
                    impact: "Workflow order context helps explain what usually comes next — without predicting."
                        .into(),
                    authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        } else if continuity.suggested_next_step.is_some() || continuity.current_focus.is_some() {
            let id = format!("pattern:workflow:focus:{ws}");
            if seen.insert(id.clone()) {
                let focus = continuity
                    .current_focus
                    .as_ref()
                    .or(continuity.suggested_next_step.as_ref())
                    .map(|f| f.title.clone())
                    .unwrap_or_else(|| "current focus".into());
                patterns.push(WorkspacePattern {
                    id,
                    kind: PatternKind::WorkflowPattern,
                    title: format!("Recurring focus: {focus}"),
                    observation: format!(
                        "Continuity repeatedly surfaces focus around \"{focus}\" for Purpose \"{label}\"."
                    ),
                    evidence: vec![PatternEvidence {
                        id: format!("ev:continuity:focus:{ws}"),
                        source_model: "continuity".into(),
                        source_ref: ws.to_string(),
                        summary: continuity.summary.clone(),
                    }],
                    confidence: PatternConfidence::Low,
                    impact: "Focus recurrence explains unfinished workflows without resuming them."
                        .into(),
                    authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // TaskPattern — depends_on / blocks relationships.
        if let Some(graph) = task_graph {
            let dep_edges: Vec<_> = graph
                .relationships
                .iter()
                .filter(|r| {
                    matches!(
                        r.kind,
                        TaskRelationshipKind::DependsOn | TaskRelationshipKind::Blocks
                    )
                })
                .take(4)
                .collect();
            if !dep_edges.is_empty() {
                let id = format!("pattern:task:{ws}");
                if seen.insert(id.clone()) {
                    let sample = &dep_edges[0];
                    patterns.push(WorkspacePattern {
                        id: id.clone(),
                        kind: PatternKind::TaskPattern,
                        title: "Task ordering relationships recur".into(),
                        observation: format!(
                            "Task Graph encodes {} relationship(s) such as {} → {} ({}).",
                            dep_edges.len(),
                            sample.from_task_id,
                            sample.to_task_id,
                            sample.kind.as_str()
                        ),
                        evidence: vec![PatternEvidence {
                            id: format!("ev:task_graph:{ws}"),
                            source_model: "task_graph".into(),
                            source_ref: ws.to_string(),
                            summary: format!(
                                "nodes={} relationships={}",
                                graph.nodes.len(),
                                graph.relationships.len()
                            ),
                        }],
                        confidence: if dep_edges.len() >= 2 {
                            PatternConfidence::High
                        } else {
                            PatternConfidence::Medium
                        },
                        impact: "Task ordering patterns explain how work usually sequences."
                            .into(),
                        authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                    });
                }
            }
        }

        // DecisionPattern — pending / blocked decisions recurring.
        if decision_queue.pending_count > 0
            || activity
                .activities
                .iter()
                .any(|a| matches!(a.activity_type, ActivityType::DecisionItem | ActivityType::BlockedAction))
        {
            let id = format!("pattern:decision:{ws}");
            if seen.insert(id.clone()) {
                patterns.push(WorkspacePattern {
                    id: id.clone(),
                    kind: PatternKind::DecisionPattern,
                    title: "Decisions frequently pause progress".into(),
                    observation: format!(
                        "Decision Queue reports {} pending item(s); decision/blocked activity appears in history.",
                        decision_queue.pending_count
                    ),
                    evidence: vec![
                        PatternEvidence {
                            id: format!("ev:dq:{ws}"),
                            source_model: "decision_queue".into(),
                            source_ref: ws.to_string(),
                            summary: format!("pending_count={}", decision_queue.pending_count),
                        },
                        PatternEvidence {
                            id: format!("ev:evolution:{ws}"),
                            source_model: "evolution".into(),
                            source_ref: ws.to_string(),
                            summary: evolution.summary.clone(),
                        },
                    ],
                    confidence: if decision_queue.pending_count > 0 {
                        PatternConfidence::High
                    } else {
                        PatternConfidence::Medium
                    },
                    impact: "Recognizing decision pauses explains where workflows often stop."
                        .into(),
                    authority_effect: WorkspacePattern::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // Evolution-backed recurrence note (not a second history).
        if !evolution.insights.is_empty() {
            evidence.push(format!(
                "Evolution insights (narrative only): {}",
                evolution.insight_count
            ));
        }
        evidence.push(format!(
            "Activity Graph activities (SoT): {}",
            activity.activities.len()
        ));
        evidence.push(format!("Operating State signals: {}", operating.signal_count));
        evidence.push(format!("Composition members: {}", composition.members.len()));

        patterns.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        if patterns.len() > 12 {
            patterns.truncate(12);
        }
        relationships.sort_by(|a, b| a.id.cmp(&b.id));

        let app_line = patterns
            .iter()
            .find(|p| p.kind == PatternKind::ApplicationPattern)
            .map(|p| p.observation.clone())
            .unwrap_or_else(|| "No application co-occurrence pattern yet.".into());
        let workflow_line = patterns
            .iter()
            .find(|p| p.kind == PatternKind::WorkflowPattern)
            .map(|p| p.observation.clone())
            .unwrap_or_else(|| "No workflow recurrence pattern yet.".into());
        let env_line = patterns
            .iter()
            .find(|p| p.kind == PatternKind::EnvironmentPattern)
            .map(|p| p.observation.clone())
            .unwrap_or_else(|| "No environment association pattern yet.".into());

        let pattern_summary = PatternSummary {
            headline: format!("Things \"{label}\" often does"),
            recurring_line: app_line,
            workflow_line: workflow_line.clone(),
            environment_line: env_line,
            narrative: format!(
                "Recurring structures for \"{label}\" are projected from Activity, Evolution, \
                 Operating State, Composition, and Task Graph. {workflow_line}"
            ),
        };

        let explanation = format!(
            "Pattern Model observes recurring structures for \"{label}\" from existing workspace \
             signals. Activity Graph remains history SoT. Patterns are not predictions, profiling, \
             memory, or automation."
        );
        let summary = build_pattern_model_summary(&label, patterns.len());

        let state = WorkspacePatternState {
            workspace_id: ws.to_string(),
            generated_at: pattern_now_rfc3339(),
            label,
            pattern_count: patterns.len(),
            relationship_count: relationships.len(),
            patterns,
            relationships,
            pattern_summary,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspacePatternState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspacePatternState,
        limit: usize,
    ) -> WorkspacePatternSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspacePatternError::CannotExecute,
        ))
    }

    pub(crate) fn audit_used_for_recommendation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        pattern: &WorkspacePattern,
        recommendation_id: &str,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.pattern.used_for_recommendation",
            true,
            json!({
                "pattern_id": pattern.id,
                "pattern_kind": pattern.kind.as_str(),
                "recommendation_id": recommendation_id,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspacePatternState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.pattern.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "pattern_count": state.pattern_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspacePatternState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.pattern.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "pattern_count": state.pattern_count,
                "label": state.label,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn kind_rank(kind: PatternKind) -> u8 {
    match kind {
        PatternKind::ApplicationPattern => 0,
        PatternKind::EnvironmentPattern => 1,
        PatternKind::WorkflowPattern => 2,
        PatternKind::TaskPattern => 3,
        PatternKind::DecisionPattern => 4,
    }
}
