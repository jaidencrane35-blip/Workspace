//! Workspace Purpose Model (Phase 5).
//!
//! Aggregates WorkGoals + Projects + Task Graph + Composition + Continuity +
//! Activity + Decision Queue into why work exists. Never persists purpose rows,
//! never executes, never invents goals.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_purpose_summary, purpose_now_rfc3339, validate_purpose_workspace_id, ActorContext,
    DecisionQueue, IntentContext, Project, PurposeEvidence, PurposeEvidenceKind, PurposeObstacle,
    PurposeRelationship, TaskGraph, WorkGoal, WorkGoalStatus, WorkflowContext,
    WorkspaceActivityGraph, WorkspaceCompositionState, WorkspaceContinuityState,
    WorkspacePurposeState, WorkspacePurposeSummary, WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceCompositionService,
    WorkspaceContinuityService, WorkspaceEnvironmentService, WorkspaceIntentService,
};

pub(crate) struct WorkspacePurposeService;

impl WorkspacePurposeService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspacePurposeState> {
        let workspace_id = workspace_id.into();
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let goals = WorkspaceIntentService::list_goals(db, &workspace_id, 20)?;
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
            &goals,
            &workflow,
            project.as_ref(),
            Some(&task_graph),
            &composition,
            &continuity,
            &activity,
            &decision_queue,
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        goals: &[WorkGoal],
        workflow: &WorkflowContext,
        project: Option<&Project>,
        task_graph: Option<&TaskGraph>,
        composition: &WorkspaceCompositionState,
        continuity: &WorkspaceContinuityState,
        activity: &WorkspaceActivityGraph,
        decision_queue: &DecisionQueue,
    ) -> Result<WorkspacePurposeState> {
        let workspace_id =
            validate_purpose_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();

        let active_project_id = workflow
            .active_project_id
            .as_ref()
            .map(|id| id.to_string());
        let active_task_id = workflow.active_task_id.as_ref().map(|id| id.to_string());
        let active_project_name = project.map(|p| p.name.clone());

        let primary_goal = select_primary_goal(goals, active_project_id.as_deref());
        let primary_work_goal_id = primary_goal.map(|g| g.id.to_string());
        let primary_work_goal_description = primary_goal.map(|g| g.description.clone());

        let label = primary_work_goal_description
            .clone()
            .or_else(|| active_project_name.clone())
            .or_else(|| Some(composition.label.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Workspace purpose".into());

        let focus_label = continuity
            .current_focus
            .as_ref()
            .map(|f| f.title.clone())
            .or_else(|| composition.focus_label.clone());

        let mut evidence_items = Vec::new();
        let mut relationships = Vec::new();
        let mut obstacles = Vec::new();
        let mut evidence = Vec::new();
        let mut recent_progress = Vec::new();

        let purpose_id = format!("purpose:{ws}");

        // WorkGoal evidence (durable SoT — Purpose does not own goals).
        if let Some(goal) = primary_goal {
            let eid = format!("evidence:work_goal:{}", goal.id);
            evidence_items.push(PurposeEvidence {
                id: eid.clone(),
                kind: PurposeEvidenceKind::WorkGoal,
                ref_id: goal.id.to_string(),
                label: goal.description.clone(),
                explanation: format!(
                    "Declared WorkGoal \"{}\" is the durable outcome this Purpose projects.",
                    goal.description
                ),
                evidence: vec![
                    format!("status={}", goal.status.as_str()),
                    "source WorkspaceIntentService / work_goals".into(),
                ],
            });
            relationships.push(PurposeRelationship {
                id: format!("rel:purpose-goal:{}", goal.id),
                from_id: purpose_id.clone(),
                to_id: eid,
                kind: "declared_as".into(),
                explanation: "Purpose label derives from an existing WorkGoal — Purpose does not own goals."
                    .into(),
                evidence: vec!["WorkGoal remains DurableStore SoT".into()],
                progress_note: if goal.status == WorkGoalStatus::Achieved {
                    Some("WorkGoal marked achieved.".into())
                } else {
                    None
                },
                incomplete_note: if goal.status == WorkGoalStatus::Active {
                    Some("WorkGoal remains active.".into())
                } else {
                    None
                },
            });
            evidence.push(format!("Primary WorkGoal: {}", goal.description));
        }

        // Project support.
        if let Some(project) = project {
            let eid = format!("evidence:project:{}", project.id);
            evidence_items.push(PurposeEvidence {
                id: eid.clone(),
                kind: PurposeEvidenceKind::Project,
                ref_id: project.id.to_string(),
                label: project.name.clone(),
                explanation: format!(
                    "Active project \"{}\" supports this Purpose.",
                    project.name
                ),
                evidence: vec!["source WorkflowContext.active_project_id".into()],
            });
            relationships.push(PurposeRelationship {
                id: format!("rel:purpose-project:{}", project.id),
                from_id: purpose_id.clone(),
                to_id: eid,
                kind: "supported_by".into(),
                explanation: format!(
                    "Project \"{}\" is the active work container for this Purpose.",
                    project.name
                ),
                evidence: vec!["WorkflowContext + Project DurableStore".into()],
                progress_note: None,
                incomplete_note: None,
            });
            evidence.push(format!("Supporting project: {}", project.name));
        }

        // Composition support.
        let comp_eid = format!("evidence:composition:{}", composition.workspace_id);
        evidence_items.push(PurposeEvidence {
            id: comp_eid.clone(),
            kind: PurposeEvidenceKind::Composition,
            ref_id: composition.workspace_id.clone(),
            label: composition.label.clone(),
            explanation: format!(
                "Composition \"{}\" is the working environment that supports this Purpose.",
                composition.label
            ),
            evidence: vec![
                format!(
                    "{} present app(s), {} missing",
                    composition.present_application_count, composition.missing_application_count
                ),
                "source Composition Engine".into(),
            ],
        });
        relationships.push(PurposeRelationship {
            id: "rel:purpose-composition".into(),
            from_id: purpose_id.clone(),
            to_id: comp_eid,
            kind: "realized_in".into(),
            explanation: "Purpose is realized through the current working environment Composition."
                .into(),
            evidence: vec![composition.summary.clone()],
            progress_note: if composition.missing_application_count == 0 {
                Some("All registered applications appear present.".into())
            } else {
                None
            },
            incomplete_note: if composition.missing_application_count > 0 {
                Some(format!(
                    "{} application(s) missing from the composition.",
                    composition.missing_application_count
                ))
            } else {
                None
            },
        });
        evidence.push(format!("Composition: {}", composition.label));

        for gap in composition.gaps.iter().take(5) {
            obstacles.push(PurposeObstacle {
                kind: format!("composition:{}", gap.kind),
                title: gap.title.clone(),
                explanation: format!(
                    "Composition gap affects Purpose readiness: {}",
                    gap.explanation
                ),
                evidence: gap.evidence.clone(),
            });
        }

        // Task Graph progress.
        let (open_task_count, completed_task_count, blocked_task_count, progress_percent) =
            if let Some(graph) = task_graph {
                let open = graph.open_incomplete_nodes().len();
                let completed = graph.completed_count;
                let blocked = graph.blocked_count;
                let progress = graph.progress_percent;
                let eid = format!("evidence:task_graph:{}", graph.workspace_id);
                evidence_items.push(PurposeEvidence {
                    id: eid.clone(),
                    kind: PurposeEvidenceKind::TaskGraph,
                    ref_id: graph.workspace_id.clone(),
                    label: "Task Graph".into(),
                    explanation: format!(
                        "Task Graph shows {progress}% progress with {open} open and {completed} completed node(s)."
                    ),
                    evidence: vec![
                        format!("blocked={blocked}"),
                        "source Task Graph".into(),
                    ],
                });
                relationships.push(PurposeRelationship {
                    id: "rel:purpose-task-graph".into(),
                    from_id: purpose_id.clone(),
                    to_id: eid,
                    kind: "advanced_by".into(),
                    explanation: "Open and completed Task Graph work advances this Purpose.".into(),
                    evidence: vec![format!("progress={progress}%")],
                    progress_note: if completed > 0 {
                        Some(format!("{completed} Task Graph item(s) completed."))
                    } else {
                        None
                    },
                    incomplete_note: if open > 0 {
                        Some(format!("{open} Task Graph item(s) still open."))
                    } else {
                        None
                    },
                });
                for node in graph.open_incomplete_nodes().into_iter().take(6) {
                    let node_eid = format!("evidence:task:{}", node.task.id);
                    evidence_items.push(PurposeEvidence {
                        id: node_eid.clone(),
                        kind: PurposeEvidenceKind::TaskGraph,
                        ref_id: node.task.id.to_string(),
                        label: node.task.title.clone(),
                        explanation: format!(
                            "Task \"{}\" contributes toward this Purpose ({}).",
                            node.task.title,
                            node.task.status.as_str()
                        ),
                        evidence: vec![format!(
                            "progress={}%",
                            node.task.progress_percent
                        )],
                    });
                    relationships.push(PurposeRelationship {
                        id: format!("rel:purpose-task:{}", node.task.id),
                        from_id: purpose_id.clone(),
                        to_id: node_eid,
                        kind: "contributed_by".into(),
                        explanation: format!(
                            "Task Graph node \"{}\" belongs to work toward this Purpose.",
                            node.task.title
                        ),
                        evidence: vec!["source Task Graph open nodes".into()],
                        progress_note: None,
                        incomplete_note: Some(format!(
                            "Status: {}.",
                            node.task.status.as_str()
                        )),
                    });
                    if node.task.status == WorkspaceTaskStatus::Blocked {
                        obstacles.push(PurposeObstacle {
                            kind: "blocked_task".into(),
                            title: format!("Blocked: {}", node.task.title),
                            explanation: node
                                .waiting_reason
                                .clone()
                                .unwrap_or_else(|| {
                                    format!(
                                        "Task \"{}\" is blocked and stalls Purpose progress.",
                                        node.task.title
                                    )
                                }),
                            evidence: vec![node.task.explanation.clone()],
                        });
                    }
                }
                for node in graph
                    .nodes
                    .iter()
                    .filter(|n| n.task.status == WorkspaceTaskStatus::Completed)
                    .take(5)
                {
                    recent_progress.push(format!("Completed: {}", node.task.title));
                }
                evidence.push(format!(
                    "Task Graph: {progress}% · {open} open · {completed} completed"
                ));
                (open, completed, blocked, progress)
            } else {
                (0, 0, 0, 0)
            };

        if blocked_task_count > 0 {
            obstacles.push(PurposeObstacle {
                kind: "blocked_work".into(),
                title: format!("{blocked_task_count} blocked Task Graph item(s)"),
                explanation: "Blocked work stands between the user and this Purpose.".into(),
                evidence: vec!["source Task Graph blocked_count".into()],
            });
        }

        // Continuity (embedded evidence — Continuity service unchanged).
        if let Some(focus) = &continuity.current_focus {
            let eid = format!("evidence:continuity:{}", focus.id);
            evidence_items.push(PurposeEvidence {
                id: eid.clone(),
                kind: PurposeEvidenceKind::Continuity,
                ref_id: focus.id.to_string(),
                label: focus.title.clone(),
                explanation: format!(
                    "Continuity focus \"{}\" situates Purpose in current session context.",
                    focus.title
                ),
                evidence: {
                    let mut refs = vec!["source Continuity Engine".into()];
                    refs.extend(focus.evidence_refs.iter().cloned());
                    refs
                },
            });
            relationships.push(PurposeRelationship {
                id: format!("rel:purpose-continuity:{}", focus.id),
                from_id: purpose_id.clone(),
                to_id: eid,
                kind: "focused_via".into(),
                explanation: "Continuity current focus explains where Purpose work left off.".into(),
                evidence: vec![focus.why.clone()],
                progress_note: None,
                incomplete_note: None,
            });
            evidence.push(format!("Continuity focus: {}", focus.title));
        }

        let interrupted_count = continuity.interrupted_work.len();
        if interrupted_count > 0 {
            obstacles.push(PurposeObstacle {
                kind: "interrupted_work".into(),
                title: format!("{interrupted_count} interrupted work item(s)"),
                explanation: "Interrupted Continuity facets indicate Purpose work was disrupted."
                    .into(),
                evidence: continuity
                    .interrupted_work
                    .iter()
                    .take(3)
                    .map(|f| f.title.clone())
                    .collect(),
            });
        }

        // Activity recent progress (skip audits).
        for activity_item in activity.timeline.iter().rev().take(8) {
            if matches!(
                activity_item.activity_type,
                workspace_domain::ActivityType::AuditSignal
            ) {
                continue;
            }
            let eid = format!("evidence:activity:{}", activity_item.id);
            evidence_items.push(PurposeEvidence {
                id: eid,
                kind: PurposeEvidenceKind::Activity,
                ref_id: activity_item.id.to_string(),
                label: activity_item.summary.clone(),
                explanation: "Recent Activity Graph entry shows progress toward Purpose.".into(),
                evidence: vec![
                    format!("type={}", activity_item.activity_type.as_str()),
                    activity_item.timestamp.clone(),
                ],
            });
            if recent_progress.len() < 5 {
                recent_progress.push(activity_item.summary.clone());
            }
        }

        // Decision Queue.
        let outstanding_decision_count = decision_queue.pending_count;
        let dq_eid = format!("evidence:decision_queue:{ws}");
        evidence_items.push(PurposeEvidence {
            id: dq_eid.clone(),
            kind: PurposeEvidenceKind::DecisionQueue,
            ref_id: ws.to_string(),
            label: format!("{outstanding_decision_count} outstanding decision(s)"),
            explanation: "Decision Queue pending items may block Purpose progress.".into(),
            evidence: vec!["source Decision Queue".into()],
        });
        relationships.push(PurposeRelationship {
            id: "rel:purpose-decisions".into(),
            from_id: purpose_id,
            to_id: dq_eid,
            kind: "gated_by".into(),
            explanation: "Outstanding human decisions affect Purpose readiness.".into(),
            evidence: vec![format!("pending={outstanding_decision_count}")],
            progress_note: None,
            incomplete_note: if outstanding_decision_count > 0 {
                Some(format!(
                    "{outstanding_decision_count} decision(s) still need attention."
                ))
            } else {
                None
            },
        });
        evidence.push(format!(
            "Decision Queue pending: {outstanding_decision_count}"
        ));
        if outstanding_decision_count > 0 {
            obstacles.push(PurposeObstacle {
                kind: "outstanding_decisions".into(),
                title: format!("{outstanding_decision_count} outstanding decision(s)"),
                explanation: "Pending decisions require human attention before Purpose advances."
                    .into(),
                evidence: vec!["source Decision Queue pending_count".into()],
            });
        }

        let explanation = format!(
            "\"{label}\" is the outcome this Workspace is working toward. \
             Purpose projects WorkGoals, projects, Task Graph progress, Composition, \
             Continuity, and decisions — it does not replace Projects, Tasks, or Goals, \
             and it never executes."
        );
        let summary = build_purpose_summary(
            &label,
            progress_percent,
            open_task_count,
            obstacles.len(),
            outstanding_decision_count,
        );

        let state = WorkspacePurposeState {
            workspace_id: ws.to_string(),
            generated_at: purpose_now_rfc3339(),
            label,
            primary_work_goal_id,
            primary_work_goal_description,
            active_project_id,
            active_project_name,
            active_task_id,
            composition_label: Some(composition.label.clone()),
            focus_label,
            progress_percent,
            open_task_count,
            completed_task_count,
            blocked_task_count,
            outstanding_decision_count,
            interrupted_count,
            evidence_items,
            relationships,
            obstacles,
            recent_progress,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspacePurposeState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspacePurposeState,
        limit: usize,
    ) -> WorkspacePurposeSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspacePurposeError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspacePurposeState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.purpose.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "label": state.label,
                "progress_percent": state.progress_percent,
                "obstacle_count": state.obstacles.len(),
                "relationship_count": state.relationships.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn select_primary_goal<'a>(
    goals: &'a [WorkGoal],
    active_project_id: Option<&str>,
) -> Option<&'a WorkGoal> {
    let active: Vec<_> = goals
        .iter()
        .filter(|g| !g.deleted && g.status == WorkGoalStatus::Active)
        .collect();
    if let Some(pid) = active_project_id {
        if let Some(goal) = active.iter().find(|g| {
            g.project_id
                .as_ref()
                .map(|id| id.as_str() == pid)
                .unwrap_or(false)
        }) {
            return Some(*goal);
        }
    }
    active.first().copied().or_else(|| {
        goals
            .iter()
            .find(|g| !g.deleted)
    })
}
