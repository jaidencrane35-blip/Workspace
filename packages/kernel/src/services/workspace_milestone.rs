//! Workspace Milestone Engine (Phase 6).
//!
//! Coordination projection over Intelligence + Session + Experience + Work Context + Navigation.
//! Owns nothing. Deterministic outcomes only — never planner, scheduler, or executor.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_milestone_summary, milestone_now_rfc3339, validate_milestone_workspace_id, ActorContext,
    ExperienceSectionKind, IntentContext, MilestoneAssociation, MilestoneEvidence,
    MilestoneReadinessBand, MilestoneRelationKind, MilestoneRelationship, MilestoneStatus,
    MilestoneSummary, ReadinessStatus, WorkspaceExperienceState, WorkspaceIntelligenceState,
    WorkspaceMilestone, WorkspaceMilestoneComparison, WorkspaceMilestoneState,
    WorkspaceMilestoneSummary, WorkspaceMilestoneValidation, WorkspaceNavigationState,
    WorkspaceSessionState, WorkspaceWorkContextState, WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceNavigationService, WorkspaceSessionService,
    WorkspaceWorkContextService,
};

pub(crate) struct WorkspaceMilestoneService;

impl WorkspaceMilestoneService {
    /// Standalone generate — loads Intelligence stack, then projects milestones.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceMilestoneState> {
        let workspace_id = workspace_id.into();
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();
        let intelligence = WorkspaceIntelligenceService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            workspace_name.clone(),
            health_label,
        )?;
        let session =
            WorkspaceSessionService::generate_with_inputs(db, actor, &intelligence)?;
        let experience = WorkspaceExperienceService::generate_with_inputs(db, actor, &session)?;
        let work_context = WorkspaceWorkContextService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
        )?;
        let navigation = WorkspaceNavigationService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
        )
    }

    /// Preferred path — consume already-assembled projections (never regenerate).
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
        session: &WorkspaceSessionState,
        experience: &WorkspaceExperienceState,
        work_context: &WorkspaceWorkContextState,
        navigation: &WorkspaceNavigationState,
    ) -> Result<WorkspaceMilestoneState> {
        let _ = validate_milestone_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();
        let ws = intelligence.workspace_id.as_str();

        let related_projects = collect_projects(intelligence, work_context);
        let related_tasks = collect_tasks(intelligence, work_context);
        let supporting_contexts = collect_contexts(work_context);
        let outstanding_decisions = collect_decisions(session, intelligence);
        let dependencies = collect_dependencies(intelligence);
        let progress_percent = intelligence.task_graph.progress_percent;
        let readiness_band = readiness_band(intelligence, session);

        let mut milestones = Vec::new();

        // Current milestone from Purpose / active goal / focus.
        let current_title = intelligence
            .purpose
            .primary_work_goal_description
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                intelligence
                    .recent_goals
                    .first()
                    .map(|g| g.description.clone())
            })
            .unwrap_or_else(|| {
                if !intelligence.purpose.label.is_empty() {
                    intelligence.purpose.label.clone()
                } else {
                    format!("Advance {label}")
                }
            });
        let current_id = format!("milestone:current:{ws}");
        let mut current_evidence = vec![
            MilestoneEvidence {
                label: "purpose".into(),
                source_projection: "purpose".into(),
                source_ref: intelligence.purpose.workspace_id.clone(),
                why: "Purpose Model describes the meaningful outcome being pursued".into(),
            },
            MilestoneEvidence {
                label: "task_graph_progress".into(),
                source_projection: "task_graph".into(),
                source_ref: intelligence.task_graph.workspace_id.clone(),
                why: format!("Task Graph reports {progress_percent}% progress toward structured work"),
            },
        ];
        if let Some(ctx) = work_context.primary_context() {
            current_evidence.push(MilestoneEvidence {
                label: ctx.name.clone(),
                source_projection: "work_context".into(),
                source_ref: ctx.id.clone(),
                why: "Primary Work Context frames the kind of work for this outcome".into(),
            });
        }
        let current_status = if readiness_band == MilestoneReadinessBand::Blocked
            || !session.risks.is_empty()
            || !outstanding_decisions.is_empty() && readiness_band != MilestoneReadinessBand::Ready
        {
            if readiness_band == MilestoneReadinessBand::Blocked || !session.risks.is_empty() {
                MilestoneStatus::Blocked
            } else {
                MilestoneStatus::Current
            }
        } else {
            MilestoneStatus::Current
        };
        milestones.push(WorkspaceMilestone {
            id: current_id.clone(),
            title: current_title.clone(),
            summary: session.session_summary.doing_line.clone(),
            status: current_status,
            readiness: readiness_band,
            progress_percent,
            evidence: current_evidence,
            related_tasks: related_tasks.clone(),
            related_projects: related_projects.clone(),
            supporting_contexts: supporting_contexts.clone(),
            outstanding_decisions: outstanding_decisions.clone(),
            dependencies: dependencies.clone(),
            why: "Current milestone is projected from Purpose, Session focus, and Task Graph progress"
                .into(),
            authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
        });

        // Upcoming from Experience next / Navigation suggested / Recommendations.
        let upcoming_title = navigation
            .navigation_summary
            .next_inspection_line
            .clone()
            .if_empty(|| experience.experience_summary.next_line.clone())
            .if_empty(|| {
                session
                    .recommendations
                    .first()
                    .map(|r| r.title.clone())
                    .unwrap_or_else(|| "Inspect next related work".into())
            });
        let upcoming_id = format!("milestone:upcoming:{ws}");
        milestones.push(WorkspaceMilestone {
            id: upcoming_id.clone(),
            title: upcoming_title.clone(),
            summary: navigation.navigation_summary.related_line.clone(),
            status: MilestoneStatus::Upcoming,
            readiness: if readiness_band == MilestoneReadinessBand::Blocked {
                MilestoneReadinessBand::Blocked
            } else {
                MilestoneReadinessBand::PartiallyReady
            },
            progress_percent: progress_percent.saturating_sub(10).min(90),
            evidence: vec![
                MilestoneEvidence {
                    label: "navigation_next".into(),
                    source_projection: "navigation".into(),
                    source_ref: navigation.workspace_id.clone(),
                    why: "Navigation suggests the next inspection toward the upcoming outcome"
                        .into(),
                },
                MilestoneEvidence {
                    label: "experience_next".into(),
                    source_projection: "experience".into(),
                    source_ref: experience.workspace_id.clone(),
                    why: "Experience Layer presents the recommended next step".into(),
                },
            ],
            related_tasks: related_tasks.clone(),
            related_projects: related_projects.clone(),
            supporting_contexts: supporting_contexts.clone(),
            outstanding_decisions: outstanding_decisions.clone(),
            dependencies: dependencies.clone(),
            why: "Upcoming milestone is projected from Navigation and Experience next-step signals"
                .into(),
            authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
        });

        // Blocked milestone when decisions/risks/gaps exist.
        if !outstanding_decisions.is_empty()
            || !session.risks.is_empty()
            || !intelligence.readiness.top_gaps.is_empty()
        {
            let blocked_title = outstanding_decisions
                .first()
                .map(|d| format!("Resolve: {}", d.label))
                .or_else(|| session.risks.first().map(|r| format!("Unblock: {}", r.title)))
                .unwrap_or_else(|| "Clear readiness blockers".into());
            milestones.push(WorkspaceMilestone {
                id: format!("milestone:blocked:{ws}"),
                title: blocked_title,
                summary: navigation.navigation_summary.blocked_line.clone(),
                status: MilestoneStatus::Blocked,
                readiness: MilestoneReadinessBand::Blocked,
                progress_percent: progress_percent.min(40),
                evidence: blocked_evidence(session, intelligence),
                related_tasks: related_tasks.clone(),
                related_projects: related_projects.clone(),
                supporting_contexts: supporting_contexts.clone(),
                outstanding_decisions: outstanding_decisions.clone(),
                dependencies: dependencies.clone(),
                why: "Blocked milestone is projected from Decision Queue, Session risks, and Readiness gaps"
                    .into(),
                authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Completed from evolution insights / experience recent progress / completed task graph.
        let mut completed_titles: Vec<(String, String, String)> = Vec::new();
        for insight in intelligence.evolution.top_insights.iter().take(2) {
            completed_titles.push((
                insight.title.clone(),
                insight.id.clone(),
                "evolution".into(),
            ));
        }
        for section in &experience.sections {
            if section.kind == ExperienceSectionKind::RecentProgress {
                for item in section.items.iter().take(2) {
                    completed_titles.push((
                        item.title.clone(),
                        item.source_ref.clone(),
                        "experience.recent_progress".into(),
                    ));
                }
            }
        }
        for node in intelligence.task_graph.top_nodes.iter().filter(|n| {
            matches!(n.task.status, WorkspaceTaskStatus::Completed)
        }) {
            completed_titles.push((
                node.task.title.clone(),
                node.task.id.to_string(),
                "task_graph".into(),
            ));
        }
        if completed_titles.is_empty() && progress_percent >= 100 {
            completed_titles.push((
                "Task Graph reports full progress".into(),
                intelligence.task_graph.workspace_id.clone(),
                "task_graph".into(),
            ));
        }
        for (i, (title, ref_id, source)) in completed_titles.into_iter().take(3).enumerate() {
            milestones.push(WorkspaceMilestone {
                id: format!("milestone:completed:{ws}:{i}"),
                title,
                summary: "Recently completed progress signal".into(),
                status: MilestoneStatus::Completed,
                readiness: MilestoneReadinessBand::Ready,
                progress_percent: 100,
                evidence: vec![MilestoneEvidence {
                    label: source.clone(),
                    source_projection: source,
                    source_ref: ref_id,
                    why: "Completed milestone is projected from Evolution, Experience progress, or Task Graph completion"
                        .into(),
                }],
                related_tasks: related_tasks.clone(),
                related_projects: related_projects.clone(),
                supporting_contexts: supporting_contexts.clone(),
                outstanding_decisions: Vec::new(),
                dependencies: Vec::new(),
                why: "Completed milestones are observational progress markers — never auto-completed by authority"
                    .into(),
                authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Ensure at least current exists (already pushed).
        let current_milestone_id = Some(current_id.clone());
        let relationships = build_relationships(&milestones, &current_id, &upcoming_id);

        let current_count = milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Current)
            .count();
        let upcoming_count = milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Upcoming)
            .count();
        let blocked_count = milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Blocked)
            .count();
        let completed_count = milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Completed)
            .count();

        let closest = milestones
            .iter()
            .filter(|m| m.status == MilestoneStatus::Upcoming || m.status == MilestoneStatus::Current)
            .max_by_key(|m| m.progress_percent)
            .map(|m| m.title.clone())
            .unwrap_or_else(|| current_title.clone());
        let blocked_line = if blocked_count == 0 {
            "No blocked milestones".into()
        } else {
            format!("{blocked_count} blocked milestone(s)")
        };
        let completed_line = if completed_count == 0 {
            "No recently completed milestones".into()
        } else {
            format!("{completed_count} completed milestone signal(s)")
        };
        let next_attention = milestones
            .iter()
            .find(|m| m.status == MilestoneStatus::Blocked)
            .or_else(|| milestones.iter().find(|m| m.status == MilestoneStatus::Upcoming))
            .map(|m| m.title.clone())
            .unwrap_or_else(|| upcoming_title);

        let milestone_summary = MilestoneSummary {
            headline: format!("Milestones for {label}"),
            current_line: current_title.clone(),
            closest_line: closest,
            blocked_line: blocked_line.clone(),
            completed_line: completed_line.clone(),
            next_attention_line: next_attention.clone(),
            narrative: format!(
                "Working toward {current_title}. {blocked_line}. Next attention: {next_attention}. \
                 Milestones coordinate understanding only — they never plan or execute."
            ),
        };

        let evidence = vec![
            format!("Intelligence generated_at: {}", intelligence.generated_at),
            format!("Session generated_at: {}", session.generated_at),
            format!("Experience generated_at: {}", experience.generated_at),
            format!("Work Context generated_at: {}", work_context.generated_at),
            format!("Navigation generated_at: {}", navigation.generated_at),
            format!("Purpose label: {}", intelligence.purpose.label),
            format!("Task Graph progress: {progress_percent}%"),
            format!("Readiness: {}", intelligence.readiness.overall_status.as_str()),
            format!("Milestones: {}", milestones.len()),
        ];

        let explanation = format!(
            "Milestones for \"{label}\" are a coordination projection over WorkspaceIntelligenceState, \
             WorkspaceSessionState, WorkspaceExperienceState, WorkspaceWorkContextState, and \
             WorkspaceNavigationState (plus Task Graph, Purpose, Composition, Continuity, Evolution, \
             Decision Queue, Recommendations, Readiness, Patterns via Intelligence). Outcomes are \
             deterministic and explainable. Milestones own no cognition data and never plan, \
             schedule, auto-complete, or bypass the Gateway."
        );
        let summary =
            build_milestone_summary(&label, milestones.len(), Some(current_title.as_str()));

        let state = WorkspaceMilestoneState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: milestone_now_rfc3339(),
            label,
            milestone_count: milestones.len(),
            current_count,
            upcoming_count,
            blocked_count,
            completed_count,
            current_milestone_id,
            milestone_summary,
            milestones,
            relationships,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            work_context_generated_at: work_context.generated_at.clone(),
            navigation_generated_at: navigation.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceMilestoneState,
        limit: usize,
    ) -> WorkspaceMilestoneSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceMilestoneState,
        right: &WorkspaceMilestoneState,
    ) -> WorkspaceMilestoneComparison {
        WorkspaceMilestoneComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceMilestoneState) -> WorkspaceMilestoneValidation {
        WorkspaceMilestoneValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceMilestoneState,
    ) -> Result<WorkspaceMilestoneValidation> {
        let report = Self::validate(state);
        Self::audit_validated(db, actor, state, &report)?;
        Ok(report)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceMilestoneError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceMilestoneState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.milestones.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "milestone_count": state.milestone_count,
                "blocked_count": state.blocked_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceMilestoneState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.milestones.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "current_count": state.current_count,
                "completed_count": state.completed_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_validated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceMilestoneState,
        report: &WorkspaceMilestoneValidation,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.milestones.validated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "valid": report.valid,
                "message_count": report.messages.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

trait IfEmpty {
    fn if_empty(self, f: impl FnOnce() -> String) -> String;
}

impl IfEmpty for String {
    fn if_empty(self, f: impl FnOnce() -> String) -> String {
        if self.trim().is_empty() {
            f()
        } else {
            self
        }
    }
}

fn readiness_band(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> MilestoneReadinessBand {
    if !session.risks.is_empty()
        || matches!(
            intelligence.readiness.overall_status,
            ReadinessStatus::Blocked
        )
    {
        MilestoneReadinessBand::Blocked
    } else {
        match intelligence.readiness.overall_status {
            ReadinessStatus::Ready => MilestoneReadinessBand::Ready,
            ReadinessStatus::PartiallyReady => MilestoneReadinessBand::PartiallyReady,
            ReadinessStatus::Blocked => MilestoneReadinessBand::Blocked,
        }
    }
}

fn blocked_evidence(
    session: &WorkspaceSessionState,
    intelligence: &WorkspaceIntelligenceState,
) -> Vec<MilestoneEvidence> {
    let mut out = Vec::new();
    for risk in session.risks.iter().take(3) {
        out.push(MilestoneEvidence {
            label: risk.title.clone(),
            source_projection: risk.source_projection.clone(),
            source_ref: risk.source_ref.clone(),
            why: risk.why.clone(),
        });
    }
    for gap in intelligence.readiness.top_gaps.iter().take(3) {
        out.push(MilestoneEvidence {
            label: gap.title.clone(),
            source_projection: format!("readiness:{}", gap.source_model),
            source_ref: gap.source_ref.clone(),
            why: gap.explanation.clone(),
        });
    }
    for decision in session.decisions.iter().take(3) {
        out.push(MilestoneEvidence {
            label: decision.title.clone(),
            source_projection: decision.source_projection.clone(),
            source_ref: decision.source_ref.clone(),
            why: decision.why.clone(),
        });
    }
    if out.is_empty() {
        out.push(MilestoneEvidence {
            label: "readiness".into(),
            source_projection: "readiness".into(),
            source_ref: intelligence.readiness.workspace_id.clone(),
            why: "Readiness indicates blockers affecting milestone progress".into(),
        });
    }
    out
}

fn collect_projects(
    intelligence: &WorkspaceIntelligenceState,
    work_context: &WorkspaceWorkContextState,
) -> Vec<MilestoneAssociation> {
    let mut out = Vec::new();
    if let Some(p) = &intelligence.current_project {
        out.push(MilestoneAssociation {
            id: p.id.to_string(),
            label: p.name.clone(),
            kind: "project".into(),
            source_projection: "intelligence.current_project".into(),
            source_ref: p.id.to_string(),
            why: "Active project contributes to the milestone".into(),
        });
    }
    if let Some(ctx) = work_context.primary_context() {
        for p in ctx.associated_projects.iter().take(4) {
            if out.iter().any(|a| a.source_ref == p.source_ref) {
                continue;
            }
            out.push(MilestoneAssociation {
                id: p.id.clone(),
                label: p.label.clone(),
                kind: "project".into(),
                source_projection: p.source_projection.clone(),
                source_ref: p.source_ref.clone(),
                why: p.why.clone(),
            });
        }
    }
    out
}

fn collect_tasks(
    intelligence: &WorkspaceIntelligenceState,
    work_context: &WorkspaceWorkContextState,
) -> Vec<MilestoneAssociation> {
    let mut out = Vec::new();
    if let Some(t) = &intelligence.current_task {
        out.push(MilestoneAssociation {
            id: t.id.to_string(),
            label: t.title.clone(),
            kind: "task".into(),
            source_projection: "intelligence.current_task".into(),
            source_ref: t.id.to_string(),
            why: "Active task contributes to the milestone".into(),
        });
    }
    if let Some(ctx) = work_context.primary_context() {
        for t in ctx.associated_tasks.iter().take(4) {
            if out.iter().any(|a| a.source_ref == t.source_ref) {
                continue;
            }
            out.push(MilestoneAssociation {
                id: t.id.clone(),
                label: t.label.clone(),
                kind: "task".into(),
                source_projection: t.source_projection.clone(),
                source_ref: t.source_ref.clone(),
                why: t.why.clone(),
            });
        }
    }
    out
}

fn collect_contexts(work_context: &WorkspaceWorkContextState) -> Vec<MilestoneAssociation> {
    work_context
        .contexts
        .iter()
        .take(4)
        .map(|c| MilestoneAssociation {
            id: c.id.clone(),
            label: c.name.clone(),
            kind: "work_context".into(),
            source_projection: "work_context".into(),
            source_ref: c.id.clone(),
            why: c.why.clone(),
        })
        .collect()
}

fn collect_decisions(
    session: &WorkspaceSessionState,
    intelligence: &WorkspaceIntelligenceState,
) -> Vec<MilestoneAssociation> {
    let mut out: Vec<_> = session
        .decisions
        .iter()
        .take(5)
        .map(|d| MilestoneAssociation {
            id: d.id.clone(),
            label: d.title.clone(),
            kind: "decision".into(),
            source_projection: d.source_projection.clone(),
            source_ref: d.source_ref.clone(),
            why: d.why.clone(),
        })
        .collect();
    if out.is_empty() {
        out = intelligence
            .decision_queue
            .items
            .iter()
            .take(5)
            .map(|d| MilestoneAssociation {
                id: d.id.to_string(),
                label: d.title.clone(),
                kind: "decision".into(),
                source_projection: "decision_queue".into(),
                source_ref: d.id.to_string(),
                why: "Outstanding Decision Queue item may block milestone progress".into(),
            })
            .collect();
    }
    out
}

fn collect_dependencies(intelligence: &WorkspaceIntelligenceState) -> Vec<MilestoneAssociation> {
    intelligence
        .task_graph
        .top_nodes
        .iter()
        .take(5)
        .map(|n| MilestoneAssociation {
            id: n.task.id.to_string(),
            label: n.task.title.clone(),
            kind: "dependency".into(),
            source_projection: "task_graph".into(),
            source_ref: n.task.id.to_string(),
            why: if n.dependency_ids.is_empty() {
                "Task Graph node related to milestone progress".into()
            } else {
                format!("Depends on {} upstream task(s)", n.dependency_ids.len())
            },
        })
        .collect()
}

fn build_relationships(
    milestones: &[WorkspaceMilestone],
    current_id: &str,
    upcoming_id: &str,
) -> Vec<MilestoneRelationship> {
    let mut out = Vec::new();
    for m in milestones {
        let kind = match m.status {
            MilestoneStatus::Current => MilestoneRelationKind::Current,
            MilestoneStatus::Upcoming => MilestoneRelationKind::Upcoming,
            MilestoneStatus::Blocked => MilestoneRelationKind::Blocked,
            MilestoneStatus::Completed => MilestoneRelationKind::Completed,
        };
        out.push(MilestoneRelationship {
            id: format!("milestone:rel:{}:{}", current_id, m.id),
            from_milestone_id: current_id.into(),
            to_milestone_id: m.id.clone(),
            kind,
            why: format!("{} is {} relative to the current outcome", m.title, kind.as_str()),
            authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
        });
        if m.id == upcoming_id && m.id != current_id {
            out.push(MilestoneRelationship {
                id: format!("milestone:rel:contributes:{upcoming_id}"),
                from_milestone_id: upcoming_id.into(),
                to_milestone_id: current_id.into(),
                kind: MilestoneRelationKind::ContributesTo,
                why: "Upcoming milestone contributes toward the current outcome".into(),
                authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if m.status == MilestoneStatus::Blocked {
            out.push(MilestoneRelationship {
                id: format!("milestone:rel:depends:{}", m.id),
                from_milestone_id: current_id.into(),
                to_milestone_id: m.id.clone(),
                kind: MilestoneRelationKind::DependsOn,
                why: "Current outcome depends on clearing this blocked milestone".into(),
                authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if m.status == MilestoneStatus::Completed {
            out.push(MilestoneRelationship {
                id: format!("milestone:rel:supports:{}", m.id),
                from_milestone_id: m.id.clone(),
                to_milestone_id: current_id.into(),
                kind: MilestoneRelationKind::Supports,
                why: "Completed progress supports the current outcome".into(),
                authority_effect: WorkspaceMilestone::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }
    out
}
