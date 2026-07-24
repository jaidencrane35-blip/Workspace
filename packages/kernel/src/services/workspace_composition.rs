//! Workspace Composition Engine (Phase 5).
//!
//! Aggregates Environment + Task Graph + Continuity + Activity + Workflow +
//! Decision Queue into a logical working environment. Never persists, executes,
//! launches, or groups windows.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_composition_summary, composition_now_rfc3339, validate_composition_workspace_id,
    ActorContext, CompositionGap, CompositionMember, CompositionMemberKind,
    CompositionRelationship, DecisionQueue, IntentContext, Project, TaskGraph,
    WorkflowContext, WorkspaceActivityGraph, WorkspaceCompositionState,
    WorkspaceCompositionSummary, WorkspaceContinuityState, WorkspaceEnvironmentState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceContinuityService,
    WorkspaceEnvironmentService, WorkspaceIntentService,
};

pub(crate) struct WorkspaceCompositionService;

impl WorkspaceCompositionService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceCompositionState> {
        let workspace_id = workspace_id.into();
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
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
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| WorkspaceIntentService::get_project(db, id.as_str()).ok());
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &environment,
            Some(&task_graph),
            &continuity,
            &activity,
            &workflow,
            &decision_queue,
            project.as_ref(),
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        environment: &WorkspaceEnvironmentState,
        task_graph: Option<&TaskGraph>,
        continuity: &WorkspaceContinuityState,
        activity: &WorkspaceActivityGraph,
        workflow: &WorkflowContext,
        decision_queue: &DecisionQueue,
        project: Option<&Project>,
    ) -> Result<WorkspaceCompositionState> {
        let workspace_id =
            validate_composition_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();

        let active_project_id = workflow
            .active_project_id
            .as_ref()
            .map(|id| id.to_string());
        let active_task_id = workflow.active_task_id.as_ref().map(|id| id.to_string());
        let active_project_name = project.map(|p| p.name.clone());
        let label = active_project_name
            .clone()
            .unwrap_or_else(|| "Workspace composition".into());

        let focus_label = continuity
            .current_focus
            .as_ref()
            .map(|f| f.title.clone())
            .or_else(|| {
                // Fall back to active task id label when Continuity has no focus facet.
                active_task_id.clone()
            });

        let mut members = Vec::new();
        let mut relationships = Vec::new();
        let mut gaps = Vec::new();
        let mut evidence = Vec::new();

        // Environment as a whole.
        let env_member_id = "member:environment".to_string();
        members.push(CompositionMember {
            id: env_member_id.clone(),
            kind: CompositionMemberKind::Environment,
            ref_id: environment.workspace_id.clone(),
            label: "Desktop environment".into(),
            present: true,
            explanation: environment.summary.clone(),
            evidence: vec![
                format!("{} window(s)", environment.windows.len()),
                format!(
                    "{} running application(s)",
                    environment.running_application_count
                ),
            ],
            authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
        });
        evidence.push(format!(
            "Environment Model: {} window(s), disconnected_work={}",
            environment.windows.len(),
            environment.disconnected_work
        ));

        // Applications (present + missing).
        let mut present_application_count = 0usize;
        let mut missing_application_count = 0usize;
        for app in &environment.applications {
            let member_id = format!("member:app:{}", app.application_id);
            let present = app.appears_running;
            if present {
                present_application_count += 1;
            } else {
                missing_application_count += 1;
            }
            let app_evidence = vec![
                app.explanation.clone(),
                format!("window_count={}", app.window_count),
            ];
            members.push(CompositionMember {
                id: member_id.clone(),
                kind: CompositionMemberKind::Application,
                ref_id: app.application_id.clone(),
                label: app.name.clone(),
                present,
                explanation: if present {
                    format!("\"{}\" belongs in this working environment and appears running.", app.name)
                } else {
                    format!(
                        "\"{}\" belongs in this working environment but is not observed on the desktop.",
                        app.name
                    )
                },
                evidence: app_evidence.clone(),
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
            relationships.push(CompositionRelationship {
                id: format!("rel:env-app:{}", app.application_id),
                from_member_id: env_member_id.clone(),
                to_member_id: member_id.clone(),
                kind: if present {
                    "includes".into()
                } else {
                    "expects".into()
                },
                explanation: format!(
                    "Application \"{}\" is registered for this Workspace environment.",
                    app.name
                ),
                evidence: app_evidence,
            });
            if !present {
                gaps.push(CompositionGap {
                    kind: "missing_application".into(),
                    title: format!("Missing: {}", app.name),
                    explanation: format!(
                        "Composition expects \"{}\" for coherent work; Environment reports it missing.",
                        app.name
                    ),
                    evidence: vec![app.explanation.clone()],
                    member_id: Some(member_id),
                });
            }
        }

        // Windows as members (capped).
        for window in environment.windows.iter().take(12) {
            let member_id = format!("member:window:{}", window.id);
            members.push(CompositionMember {
                id: member_id.clone(),
                kind: CompositionMemberKind::Window,
                ref_id: window.id.clone(),
                label: window.title.clone(),
                present: true,
                explanation: window.explanation.clone(),
                evidence: vec![format!("state={}", window.state.as_str())],
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
            if let Some(app_id) = &window.matched_application_id {
                relationships.push(CompositionRelationship {
                    id: format!("rel:win-app:{}", window.id),
                    from_member_id: member_id,
                    to_member_id: format!("member:app:{app_id}"),
                    kind: "belongs_to".into(),
                    explanation: format!(
                        "Window \"{}\" matched application in the Environment Model.",
                        window.title
                    ),
                    evidence: vec![window.explanation.clone()],
                });
            }
        }

        // Layout associations.
        for layout in &environment.layout_associations {
            let member_id = format!("member:layout:{}", layout.layout_id);
            members.push(CompositionMember {
                id: member_id.clone(),
                kind: CompositionMemberKind::Layout,
                ref_id: layout.layout_id.clone(),
                label: layout.layout_name.clone(),
                present: true,
                explanation: layout.explanation.clone(),
                evidence: vec!["source Environment layout association".into()],
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
            relationships.push(CompositionRelationship {
                id: format!("rel:env-layout:{}", layout.layout_id),
                from_member_id: env_member_id.clone(),
                to_member_id: member_id,
                kind: "presents".into(),
                explanation: "Canvas layout is part of this working environment presentation."
                    .into(),
                evidence: vec![layout.explanation.clone()],
            });
        }

        // Active project / work.
        if let Some(project_id) = &active_project_id {
            let project_member_id = format!("member:project:{project_id}");
            let project_label = active_project_name
                .clone()
                .unwrap_or_else(|| project_id.clone());
            members.push(CompositionMember {
                id: project_member_id.clone(),
                kind: CompositionMemberKind::Project,
                ref_id: project_id.clone(),
                label: project_label.clone(),
                present: true,
                explanation: format!(
                    "Project \"{project_label}\" is the active work this composition supports."
                ),
                evidence: vec!["source WorkflowContext.active_project_id".into()],
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
            evidence.push(format!("Active project: {project_label}"));

            // Link running apps to project.
            for app in environment.applications.iter().filter(|a| a.appears_running) {
                relationships.push(CompositionRelationship {
                    id: format!("rel:app-project:{}", app.application_id),
                    from_member_id: format!("member:app:{}", app.application_id),
                    to_member_id: project_member_id.clone(),
                    kind: "supports".into(),
                    explanation: format!(
                        "Running application \"{}\" supports active project \"{project_label}\".",
                        app.name
                    ),
                    evidence: vec!["WorkflowContext active project + Environment running match".into()],
                });
            }
        }

        if active_project_id.is_some() || active_task_id.is_some() {
            let work_id = format!(
                "member:active_work:{}:{}",
                active_project_id.as_deref().unwrap_or("none"),
                active_task_id.as_deref().unwrap_or("none")
            );
            members.push(CompositionMember {
                id: work_id,
                kind: CompositionMemberKind::ActiveWork,
                ref_id: active_task_id
                    .clone()
                    .or_else(|| active_project_id.clone())
                    .unwrap_or_default(),
                label: focus_label
                    .clone()
                    .unwrap_or_else(|| "Active work".into()),
                present: true,
                explanation: "Active work from WorkflowContext anchors this composition.".into(),
                evidence: vec![
                    format!("project={:?}", active_project_id),
                    format!("task={:?}", active_task_id),
                ],
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Continuity focus.
        if let Some(focus) = &continuity.current_focus {
            let member_id = format!("member:continuity:{}", focus.id);
            members.push(CompositionMember {
                id: member_id.clone(),
                kind: CompositionMemberKind::Continuity,
                ref_id: focus.id.to_string(),
                label: focus.title.clone(),
                present: true,
                explanation: focus.why.clone(),
                evidence: {
                    let mut refs = vec!["source Continuity Engine current_focus".into()];
                    refs.extend(focus.evidence_refs.iter().cloned());
                    refs
                },
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
            evidence.push(format!("Continuity focus: {}", focus.title));
            if let Some(project_id) = &active_project_id {
                relationships.push(CompositionRelationship {
                    id: format!("rel:continuity-project:{}", focus.id),
                    from_member_id: member_id,
                    to_member_id: format!("member:project:{project_id}"),
                    kind: "focuses".into(),
                    explanation: "Continuity current focus belongs to the active project.".into(),
                    evidence: vec![focus.why.clone()],
                });
            }
        }

        // Task Graph nodes.
        let mut task_node_count = 0usize;
        if let Some(graph) = task_graph {
            let open = graph.open_incomplete_nodes();
            task_node_count = open.len();
            for node in open.into_iter().take(8) {
                let member_id = format!("member:task:{}", node.task.id);
                members.push(CompositionMember {
                    id: member_id.clone(),
                    kind: CompositionMemberKind::TaskNode,
                    ref_id: node.task.id.to_string(),
                    label: node.task.title.clone(),
                    present: true,
                    explanation: format!(
                        "Task Graph node \"{}\" is open work belonging to this composition.",
                        node.task.title
                    ),
                    evidence: vec![
                        format!("status={}", node.task.status.as_str()),
                        "source Task Graph".into(),
                    ],
                    authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
                });
                if let Some(project_id) = &active_project_id {
                    relationships.push(CompositionRelationship {
                        id: format!("rel:task-project:{}", node.task.id),
                        from_member_id: member_id.clone(),
                        to_member_id: format!("member:project:{project_id}"),
                        kind: "belongs_to".into(),
                        explanation: "Open Task Graph work belongs with the active project.".into(),
                        evidence: vec!["Task Graph + WorkflowContext".into()],
                    });
                }
            }
            for rel in graph.relationships.iter().take(12) {
                relationships.push(CompositionRelationship {
                    id: format!("rel:tg:{}:{}:{}", rel.kind.as_str(), rel.from_task_id, rel.to_task_id),
                    from_member_id: format!("member:task:{}", rel.from_task_id),
                    to_member_id: format!("member:task:{}", rel.to_task_id),
                    kind: format!("task_graph:{}", rel.kind.as_str()),
                    explanation: format!(
                        "Task Graph relationship {} between work items.",
                        rel.kind.as_str()
                    ),
                    evidence: vec!["source Task Graph relationships".into()],
                });
            }
            evidence.push(format!(
                "Task Graph: {} open node(s), {} relationship(s)",
                task_node_count,
                graph.relationships.len()
            ));
        }

        // Recent activity (informative members).
        for activity_item in activity.timeline.iter().rev().take(5) {
            if matches!(
                activity_item.activity_type,
                workspace_domain::ActivityType::AuditSignal
            ) {
                continue;
            }
            let member_id = format!("member:activity:{}", activity_item.id);
            members.push(CompositionMember {
                id: member_id,
                kind: CompositionMemberKind::Activity,
                ref_id: activity_item.id.to_string(),
                label: activity_item.summary.clone(),
                present: true,
                explanation: "Recent Activity Graph entry contributes evidence for this composition."
                    .into(),
                evidence: vec![
                    format!("type={}", activity_item.activity_type.as_str()),
                    activity_item.timestamp.clone(),
                ],
                authority_effect: CompositionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Environment gaps → composition gaps (semantic layer).
        for gap in &environment.gaps {
            if gap.kind == "missing_application" {
                // Already represented via missing app members.
                continue;
            }
            gaps.push(CompositionGap {
                kind: gap.kind.clone(),
                title: gap.title.clone(),
                explanation: format!(
                    "Composition inherits Environment gap: {}",
                    gap.explanation
                ),
                evidence: vec![gap.explanation.clone()],
                member_id: None,
            });
        }

        if environment.disconnected_work {
            evidence.push("Active work appears disconnected from desktop applications.".into());
        }

        let outstanding_decision_count = decision_queue.pending_count;
        evidence.push(format!(
            "Decision Queue pending: {outstanding_decision_count}"
        ));

        let explanation = format!(
            "\"{label}\" groups registered applications, open Task Graph work, Continuity focus, \
             and Environment observations into one logical working environment. \
             Evidence comes only from existing read models — nothing is invented or executed."
        );

        let summary = build_composition_summary(
            &label,
            present_application_count,
            missing_application_count,
            focus_label.as_deref(),
            outstanding_decision_count,
        );

        let state = WorkspaceCompositionState {
            workspace_id: ws.to_string(),
            generated_at: composition_now_rfc3339(),
            label,
            active_project_id,
            active_project_name,
            active_task_id,
            focus_label,
            members,
            relationships,
            gaps,
            present_application_count,
            missing_application_count,
            task_node_count,
            window_count: environment.windows.len(),
            outstanding_decision_count,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceCompositionState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceCompositionState,
        limit: usize,
    ) -> WorkspaceCompositionSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceCompositionError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceCompositionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.composition.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "member_count": state.members.len(),
                "relationship_count": state.relationships.len(),
                "gap_count": state.gaps.len(),
                "present_application_count": state.present_application_count,
                "missing_application_count": state.missing_application_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
