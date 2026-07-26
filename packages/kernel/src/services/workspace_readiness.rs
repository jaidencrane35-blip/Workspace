//! Workspace Readiness Model (Phase 5).
//!
//! Aggregates Operating State + Environment + Composition + Task Graph + Purpose +
//! Continuity + Evolution + Patterns + Decision Queue into preparedness assessments.
//! Never prepares, fixes, launches, or grants authority.
//! Distinct from runtime WorkspaceHealth and from Recommendation / Adaptation
//! (which may consume readiness gaps as evidence).

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_readiness_summary, readiness_now_rfc3339, validate_readiness_workspace_id, ActorContext,
    DecisionQueue, DecisionSourceType, DecisionState, IntentContext, ReadinessAssessment,
    ReadinessGap, ReadinessKind, ReadinessSignal, ReadinessStatus, ReadinessSummary, TaskGraph,
    WorkspaceCompositionState, WorkspaceContinuityState, WorkspaceEnvironmentState,
    WorkspaceEvolutionState, WorkspaceOperatingState, WorkspacePatternState, WorkspacePurposeState,
    WorkspaceReadinessState, WorkspaceReadinessSummary, WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceCompositionService, WorkspaceContinuityService,
    WorkspaceEnvironmentService, WorkspaceEvolutionService, WorkspaceOperatingStateService,
    WorkspacePatternService, WorkspacePurposeService,
};

pub(crate) struct WorkspaceReadinessService;

impl WorkspaceReadinessService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceReadinessState> {
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
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
        let dq = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let activity = crate::services::WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            Some(&dq),
        )?;
        let continuity = WorkspaceContinuityService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &dq,
            &activity,
        )?;
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let workflow =
            crate::services::WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| crate::services::WorkspaceIntentService::get_project(db, id.as_str()).ok());
        let composition = WorkspaceCompositionService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &environment,
            Some(&task_graph),
            &continuity,
            &activity,
            &workflow,
            &dq,
            project.as_ref(),
        )?;
        let evolution = WorkspaceEvolutionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let patterns = WorkspacePatternService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &operating,
            &environment,
            &composition,
            &task_graph,
            &purpose,
            &continuity,
            &evolution,
            &patterns,
            &dq,
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        operating: &WorkspaceOperatingState,
        environment: &WorkspaceEnvironmentState,
        composition: &WorkspaceCompositionState,
        task_graph: &TaskGraph,
        purpose: &WorkspacePurposeState,
        continuity: &WorkspaceContinuityState,
        evolution: &WorkspaceEvolutionState,
        patterns: &WorkspacePatternState,
        decision_queue: &DecisionQueue,
    ) -> Result<WorkspaceReadinessState> {
        let workspace_id =
            validate_readiness_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let label = if !purpose.label.is_empty() {
            purpose.label.clone()
        } else if !composition.label.is_empty() {
            composition.label.clone()
        } else {
            operating.context.purpose_label.clone()
        };

        let mut assessments = Vec::new();
        let mut evidence = Vec::new();

        assessments.push(assess_environment(ws, environment, composition));
        assessments.push(assess_context(ws, continuity, operating));
        assessments.push(assess_task(ws, task_graph, continuity));
        assessments.push(assess_decision(ws, decision_queue, continuity));
        assessments.push(assess_purpose(ws, purpose, composition, environment));

        assessments.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });

        let overall_status = assessments
            .iter()
            .fold(ReadinessStatus::Ready, |acc, a| acc.combine(a.status));
        let gap_count = assessments.iter().map(|a| a.gaps.len()).sum();
        let ready_count = assessments
            .iter()
            .filter(|a| a.status == ReadinessStatus::Ready)
            .count();
        let partially_ready_count = assessments
            .iter()
            .filter(|a| a.status == ReadinessStatus::PartiallyReady)
            .count();
        let blocked_count = assessments
            .iter()
            .filter(|a| a.status == ReadinessStatus::Blocked)
            .count();

        evidence.push(format!("Operating signals: {}", operating.signal_count));
        evidence.push(format!(
            "Environment apps running: {}",
            environment.running_application_count
        ));
        evidence.push(format!("Composition: {}", composition.label));
        evidence.push(format!("Task Graph nodes: {}", task_graph.nodes.len()));
        evidence.push(format!("Purpose: {}", purpose.label));
        evidence.push(format!(
            "Continuity interrupted: {}",
            continuity.interrupted_work.len()
        ));
        evidence.push(format!("Evolution events: {}", evolution.event_count));
        evidence.push(format!("Patterns: {}", patterns.pattern_count));
        evidence.push(format!(
            "Decision Queue pending: {}",
            decision_queue.pending_count
        ));

        let status_line = match overall_status {
            ReadinessStatus::Ready => {
                format!("Your workspace for \"{label}\" is ready. Previous context and tasks are available.")
            }
            ReadinessStatus::PartiallyReady => {
                format!(
                    "Your project context for \"{label}\" is available, but {gap_count} gap(s) remain."
                )
            }
            ReadinessStatus::Blocked => {
                format!(
                    "Work on \"{label}\" appears blocked — {blocked_count} readiness area(s) need attention."
                )
            }
        };
        let gap_line = if gap_count == 0 {
            "No readiness gaps reported.".into()
        } else {
            let titles: Vec<_> = assessments
                .iter()
                .flat_map(|a| a.gaps.iter().map(|g| g.title.as_str()))
                .take(3)
                .collect();
            format!("{gap_count} gap(s): {}", titles.join("; "))
        };
        let readiness_summary = ReadinessSummary {
            headline: format!("Can you continue \"{label}\"?"),
            status_line: status_line.clone(),
            gap_line,
            narrative: format!(
                "Readiness describes whether the Workspace is prepared for \"{label}\" \
                 ({status}). Gaps are explainable and never auto-fixed. Distinct from \
                 Recommendations (next steps), Adaptations (improvements), and runtime health.",
                status = overall_status.as_str()
            ),
        };

        let explanation = format!(
            "Readiness for \"{label}\" is projected from Operating State, Environment, \
             Composition, Task Graph, Purpose, Continuity, Evolution, Patterns, and Decision Queue. \
             Status: {}. Informational only — never prepares, launches, or bypasses the Gateway. \
             Distinct from WorkspaceHealth.",
            overall_status.as_str()
        );
        let summary = build_readiness_summary(&label, overall_status, gap_count);

        let state = WorkspaceReadinessState {
            workspace_id: ws.to_string(),
            generated_at: readiness_now_rfc3339(),
            label,
            overall_status,
            assessment_count: assessments.len(),
            gap_count,
            ready_count,
            partially_ready_count,
            blocked_count,
            assessments,
            readiness_summary,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceReadinessState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_assessed(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceReadinessState,
        limit: usize,
    ) -> WorkspaceReadinessSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceReadinessError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceReadinessState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.readiness.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "overall_status": state.overall_status.as_str(),
                "assessment_count": state.assessment_count,
                "gap_count": state.gap_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_assessed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceReadinessState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.readiness.assessed",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "ready_count": state.ready_count,
                "partially_ready_count": state.partially_ready_count,
                "blocked_count": state.blocked_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceReadinessState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.readiness.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "overall_status": state.overall_status.as_str(),
                "gap_count": state.gap_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn assess_environment(
    ws: &str,
    environment: &WorkspaceEnvironmentState,
    composition: &WorkspaceCompositionState,
) -> ReadinessAssessment {
    let mut gaps = Vec::new();
    let mut signals = vec![
        ReadinessSignal {
            id: format!("sig:environment:{ws}"),
            source_model: "environment".into(),
            source_ref: environment.workspace_id.clone(),
            summary: environment.summary.clone(),
        },
        ReadinessSignal {
            id: format!("sig:composition:env:{ws}"),
            source_model: "composition".into(),
            source_ref: composition.workspace_id.clone(),
            summary: format!(
                "{} present / {} missing application(s)",
                composition.present_application_count, composition.missing_application_count
            ),
        },
    ];

    if environment.missing_application_count > 0 || composition.missing_application_count > 0 {
        gaps.push(ReadinessGap {
            id: format!("gap:env:missing:{ws}"),
            kind: "missing_applications".into(),
            title: "Required applications are not currently visible".into(),
            explanation: format!(
                "Environment reports {} missing application(s); Composition reports {}.",
                environment.missing_application_count, composition.missing_application_count
            ),
            impact: "Setup may be incomplete for the active working environment.".into(),
            source_model: "environment".into(),
            source_ref: environment.workspace_id.clone(),
        });
    }
    if environment.disconnected_work {
        gaps.push(ReadinessGap {
            id: format!("gap:env:disconnected:{ws}"),
            kind: "disconnected_work".into(),
            title: "Desktop appears disconnected from active work".into(),
            explanation: "Environment Model reports active work appears disconnected from open windows."
                .into(),
            impact: "Environment alignment is weak for continuing current Purpose.".into(),
            source_model: "environment".into(),
            source_ref: environment.workspace_id.clone(),
        });
    }
    for gap in environment.gaps.iter().take(3) {
        signals.push(ReadinessSignal {
            id: format!("sig:environment:gap:{}", gap.title),
            source_model: "environment".into(),
            source_ref: gap.application_id.clone().unwrap_or_else(|| ws.into()),
            summary: gap.explanation.clone(),
        });
    }

    let status = if environment.disconnected_work
        && (environment.missing_application_count > 0 || composition.missing_application_count > 0)
    {
        ReadinessStatus::Blocked
    } else if !gaps.is_empty() {
        ReadinessStatus::PartiallyReady
    } else {
        ReadinessStatus::Ready
    };

    ReadinessAssessment {
        id: format!("readiness:environment:{ws}"),
        kind: ReadinessKind::EnvironmentReadiness,
        title: "Environment readiness".into(),
        status,
        reason: match status {
            ReadinessStatus::Ready => {
                "Required development applications and window associations appear available.".into()
            }
            ReadinessStatus::PartiallyReady => {
                "Some environment resources are missing or misaligned.".into()
            }
            ReadinessStatus::Blocked => {
                "Environment gaps block continuing work without further setup.".into()
            }
        },
        signals,
        gaps,
        impact: "Environment readiness affects whether desktop resources match current work."
            .into(),
        authority_effect: ReadinessAssessment::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn assess_context(
    ws: &str,
    continuity: &WorkspaceContinuityState,
    operating: &WorkspaceOperatingState,
) -> ReadinessAssessment {
    let mut gaps = Vec::new();
    let mut signals = vec![
        ReadinessSignal {
            id: format!("sig:continuity:{ws}"),
            source_model: "continuity".into(),
            source_ref: continuity.workspace_id.clone(),
            summary: continuity.summary.clone(),
        },
        ReadinessSignal {
            id: format!("sig:operating:{ws}"),
            source_model: "operating_state".into(),
            source_ref: operating.workspace_id.clone(),
            summary: operating.summary.clone(),
        },
    ];

    if continuity.current_focus.is_some() || !continuity.resumable_work.is_empty() {
        signals.push(ReadinessSignal {
            id: format!("sig:continuity:available:{ws}"),
            source_model: "continuity".into(),
            source_ref: continuity
                .current_focus
                .as_ref()
                .map(|f| f.id.to_string())
                .unwrap_or_else(|| ws.into()),
            summary: "Previous work context is available.".into(),
        });
    }

    if !continuity.interrupted_work.is_empty() {
        let facet = &continuity.interrupted_work[0];
        gaps.push(ReadinessGap {
            id: format!("gap:context:interrupted:{}", facet.id),
            kind: "interrupted_work".into(),
            title: format!("Interrupted work: {}", facet.title),
            explanation: format!(
                "Continuity reports interrupted work \"{}\"; context may need restoration before continuing.",
                facet.title
            ),
            impact: "Interrupted context slows resumption until the human restores it via Intent."
                .into(),
            source_model: "continuity".into(),
            source_ref: facet.id.to_string(),
        });
    }

    let status = if continuity.interrupted_work.len() > 1 {
        ReadinessStatus::PartiallyReady
    } else if !continuity.interrupted_work.is_empty() {
        ReadinessStatus::PartiallyReady
    } else if continuity.current_focus.is_some() || !continuity.resumable_work.is_empty() {
        ReadinessStatus::Ready
    } else {
        ReadinessStatus::PartiallyReady
    };

    let gaps = if matches!(status, ReadinessStatus::Ready) {
        Vec::new()
    } else if gaps.is_empty() {
        vec![ReadinessGap {
            id: format!("gap:context:thin:{ws}"),
            kind: "thin_context".into(),
            title: "Limited continuity context".into(),
            explanation: "No strong current focus or resumable work was projected.".into(),
            impact: "Continuing may require re-establishing focus via Intent.".into(),
            source_model: "continuity".into(),
            source_ref: continuity.workspace_id.clone(),
        }]
    } else {
        gaps
    };

    ReadinessAssessment {
        id: format!("readiness:context:{ws}"),
        kind: ReadinessKind::ContextReadiness,
        title: "Context readiness".into(),
        status,
        reason: match status {
            ReadinessStatus::Ready => "Previous work context is available.".into(),
            ReadinessStatus::PartiallyReady => {
                "Context is partially available; interrupted or thin continuity remains.".into()
            }
            ReadinessStatus::Blocked => "Context is insufficient to continue safely.".into(),
        },
        signals,
        gaps,
        impact: "Context readiness affects how quickly interrupted work can resume.".into(),
        authority_effect: ReadinessAssessment::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn assess_task(
    ws: &str,
    task_graph: &TaskGraph,
    continuity: &WorkspaceContinuityState,
) -> ReadinessAssessment {
    let mut gaps = Vec::new();
    let signals = vec![
        ReadinessSignal {
            id: format!("sig:task_graph:{ws}"),
            source_model: "task_graph".into(),
            source_ref: task_graph.workspace_id.clone(),
            summary: task_graph.summary.clone(),
        },
        ReadinessSignal {
            id: format!("sig:continuity:blockers:{ws}"),
            source_model: "continuity".into(),
            source_ref: continuity.workspace_id.clone(),
            summary: format!("{} continuity blocker(s)", continuity.blockers.len()),
        },
    ];

    if task_graph.blocked_count > 0 {
        gaps.push(ReadinessGap {
            id: format!("gap:task:blocked:{ws}"),
            kind: "blocked_tasks".into(),
            title: format!("{} blocked task(s)", task_graph.blocked_count),
            explanation: format!(
                "Task Graph reports {} blocked and {} waiting task(s).",
                task_graph.blocked_count, task_graph.waiting_count
            ),
            impact: "Blocked tasks prevent completing dependent work.".into(),
            source_model: "task_graph".into(),
            source_ref: task_graph.workspace_id.clone(),
        });
    }

    for node in task_graph.nodes.iter().filter(|n| {
        matches!(n.task.status, WorkspaceTaskStatus::Blocked)
            || !n.blocker_ids.is_empty()
            || !n.dependency_ids.is_empty()
                && matches!(
                    n.task.status,
                    WorkspaceTaskStatus::Waiting | WorkspaceTaskStatus::Blocked
                )
    }).take(3)
    {
        if !node.dependency_ids.is_empty()
            || matches!(node.task.status, WorkspaceTaskStatus::Blocked)
        {
            let already = gaps.iter().any(|g| g.source_ref == node.task.id.as_str());
            if !already {
                gaps.push(ReadinessGap {
                    id: format!("gap:task:deps:{}", node.task.id.as_str()),
                    kind: "unresolved_dependencies".into(),
                    title: format!("Task \"{}\" has unresolved dependencies", node.task.title),
                    explanation: format!(
                        "Task status {}; blockers {:?}; dependencies {:?}.",
                        node.task.status.as_str(),
                        node.blocker_ids,
                        node.dependency_ids
                    ),
                    impact: "Current task may not be ready until dependencies clear.".into(),
                    source_model: "task_graph".into(),
                    source_ref: node.task.id.as_str().to_string(),
                });
            }
        }
    }

    for blocker in continuity.blockers.iter().take(2) {
        gaps.push(ReadinessGap {
            id: format!("gap:task:continuity:{}", blocker.id),
            kind: "continuity_blocker".into(),
            title: format!("Continuity blocker: {}", blocker.title),
            explanation: blocker.summary.clone(),
            impact: "Continuity blockers affect task readiness for active work.".into(),
            source_model: "continuity".into(),
            source_ref: blocker.id.to_string(),
        });
    }

    let status = if task_graph.blocked_count > 0 {
        ReadinessStatus::Blocked
    } else if !gaps.is_empty() || task_graph.waiting_count > 0 {
        ReadinessStatus::PartiallyReady
    } else {
        ReadinessStatus::Ready
    };

    ReadinessAssessment {
        id: format!("readiness:task:{ws}"),
        kind: ReadinessKind::TaskReadiness,
        title: "Task readiness".into(),
        status,
        reason: match status {
            ReadinessStatus::Ready => "Active tasks do not report unresolved blockers.".into(),
            ReadinessStatus::PartiallyReady => {
                "Some tasks are waiting or have soft continuity friction.".into()
            }
            ReadinessStatus::Blocked => {
                "Current task work has unresolved dependencies or blocked status.".into()
            }
        },
        signals,
        gaps,
        impact: "Task readiness determines whether work units can progress.".into(),
        authority_effect: ReadinessAssessment::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn assess_decision(
    ws: &str,
    decision_queue: &DecisionQueue,
    continuity: &WorkspaceContinuityState,
) -> ReadinessAssessment {
    let open: Vec<_> = decision_queue
        .items
        .iter()
        .filter(|i| {
            matches!(
                i.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            )
        })
        .collect();
    let approvals: Vec<_> = open
        .iter()
        .filter(|i| {
            matches!(
                i.source_type,
                DecisionSourceType::PendingApproval | DecisionSourceType::BlockedAction
            )
        })
        .collect();

    let mut gaps = Vec::new();
    let signals = vec![
        ReadinessSignal {
            id: format!("sig:decision_queue:{ws}"),
            source_model: "decision_queue".into(),
            source_ref: decision_queue.workspace_id.clone(),
            summary: format!(
                "{} pending decision(s), {} high priority",
                decision_queue.pending_count, decision_queue.high_priority_count
            ),
        },
        ReadinessSignal {
            id: format!("sig:continuity:decisions:{ws}"),
            source_model: "continuity".into(),
            source_ref: continuity.workspace_id.clone(),
            summary: format!(
                "{} outstanding continuity decision facet(s)",
                continuity.outstanding_decisions.len()
            ),
        },
    ];

    for item in approvals.iter().take(3) {
        gaps.push(ReadinessGap {
            id: format!("gap:decision:{}", item.id.as_str()),
            kind: "pending_approval".into(),
            title: format!("Pending decision: {}", item.title),
            explanation: item.explanation.clone(),
            impact: "Pending approval blocks continuation until the human decides.".into(),
            source_model: "decision_queue".into(),
            source_ref: item.id.as_str().to_string(),
        });
    }

    if gaps.is_empty() && decision_queue.pending_count > 0 {
        gaps.push(ReadinessGap {
            id: format!("gap:decision:pending:{ws}"),
            kind: "pending_decisions".into(),
            title: format!("{} pending decision(s) remain", decision_queue.pending_count),
            explanation: "Decision Queue has open items that may affect continuation.".into(),
            impact: "Outstanding decisions reduce decision readiness.".into(),
            source_model: "decision_queue".into(),
            source_ref: decision_queue.workspace_id.clone(),
        });
    }

    let status = if !approvals.is_empty() {
        ReadinessStatus::Blocked
    } else if decision_queue.pending_count > 0 {
        ReadinessStatus::PartiallyReady
    } else {
        ReadinessStatus::Ready
    };

    ReadinessAssessment {
        id: format!("readiness:decision:{ws}"),
        kind: ReadinessKind::DecisionReadiness,
        title: "Decision readiness".into(),
        status,
        reason: match status {
            ReadinessStatus::Ready => "No pending approvals block continuation.".into(),
            ReadinessStatus::PartiallyReady => {
                "Pending decisions remain but are not hard approval blockers.".into()
            }
            ReadinessStatus::Blocked => {
                "Pending approval blocks continuation.".into()
            }
        },
        signals,
        gaps,
        impact: "Decision readiness affects whether gated work can proceed.".into(),
        authority_effect: ReadinessAssessment::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn assess_purpose(
    ws: &str,
    purpose: &WorkspacePurposeState,
    composition: &WorkspaceCompositionState,
    environment: &WorkspaceEnvironmentState,
) -> ReadinessAssessment {
    let mut gaps = Vec::new();
    let signals = vec![
        ReadinessSignal {
            id: format!("sig:purpose:{ws}"),
            source_model: "purpose".into(),
            source_ref: purpose.workspace_id.clone(),
            summary: purpose.summary.clone(),
        },
        ReadinessSignal {
            id: format!("sig:composition:purpose:{ws}"),
            source_model: "composition".into(),
            source_ref: composition.workspace_id.clone(),
            summary: composition.summary.clone(),
        },
    ];

    for obstacle in purpose.obstacles.iter().take(3) {
        gaps.push(ReadinessGap {
            id: format!("gap:purpose:{}", obstacle.kind),
            kind: obstacle.kind.clone(),
            title: obstacle.title.clone(),
            explanation: obstacle.explanation.clone(),
            impact: "Purpose obstacles reduce alignment between intent and active work.".into(),
            source_model: "purpose".into(),
            source_ref: purpose.workspace_id.clone(),
        });
    }

    if purpose.blocked_task_count > 0 {
        gaps.push(ReadinessGap {
            id: format!("gap:purpose:blocked_tasks:{ws}"),
            kind: "purpose_blocked_tasks".into(),
            title: format!(
                "{} blocked task(s) under Purpose \"{}\"",
                purpose.blocked_task_count, purpose.label
            ),
            explanation: "Purpose Model reports blocked tasks against the current objective."
                .into(),
            impact: "Purpose progress is constrained until blockers clear via Intent.".into(),
            source_model: "purpose".into(),
            source_ref: purpose.workspace_id.clone(),
        });
    }

    if !composition.gaps.is_empty() {
        gaps.push(ReadinessGap {
            id: format!("gap:purpose:composition:{ws}"),
            kind: "composition_gap".into(),
            title: format!("Composition gaps for \"{}\"", composition.label),
            explanation: format!(
                "{} composition gap(s) affect Purpose readiness.",
                composition.gaps.len()
            ),
            impact: "Incomplete composition weakens Purpose alignment.".into(),
            source_model: "composition".into(),
            source_ref: composition.workspace_id.clone(),
        });
    }

    let aligned = purpose.active_project_id.is_some()
        && !environment.disconnected_work
        && purpose.obstacles.is_empty();

    let status = if purpose.blocked_task_count > 0 && !purpose.obstacles.is_empty() {
        ReadinessStatus::Blocked
    } else if !gaps.is_empty() || !aligned {
        ReadinessStatus::PartiallyReady
    } else {
        ReadinessStatus::Ready
    };

    let gaps = if matches!(status, ReadinessStatus::Ready) {
        Vec::new()
    } else if gaps.is_empty() {
        vec![ReadinessGap {
            id: format!("gap:purpose:alignment:{ws}"),
            kind: "purpose_alignment".into(),
            title: "Purpose alignment is incomplete".into(),
            explanation: "Active work may not fully align with the current objective.".into(),
            impact: "Purpose readiness is partial until work and objective align.".into(),
            source_model: "purpose".into(),
            source_ref: purpose.workspace_id.clone(),
        }]
    } else {
        gaps
    };

    ReadinessAssessment {
        id: format!("readiness:purpose:{ws}"),
        kind: ReadinessKind::PurposeReadiness,
        title: "Purpose readiness".into(),
        status,
        reason: match status {
            ReadinessStatus::Ready => {
                "Active work aligns with the current objective.".into()
            }
            ReadinessStatus::PartiallyReady => {
                "Purpose is partially aligned; obstacles or composition gaps remain.".into()
            }
            ReadinessStatus::Blocked => {
                "Purpose progress is blocked by obstacles and blocked tasks.".into()
            }
        },
        signals,
        gaps,
        impact: "Purpose readiness explains whether current activity serves the objective."
            .into(),
        authority_effect: ReadinessAssessment::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn kind_rank(kind: ReadinessKind) -> u8 {
    match kind {
        ReadinessKind::DecisionReadiness => 0,
        ReadinessKind::TaskReadiness => 1,
        ReadinessKind::EnvironmentReadiness => 2,
        ReadinessKind::ContextReadiness => 3,
        ReadinessKind::PurposeReadiness => 4,
    }
}
