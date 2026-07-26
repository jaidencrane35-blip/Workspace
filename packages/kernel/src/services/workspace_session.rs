//! Workspace Session Engine (Phase 6).
//!
//! Projects Workspace Intelligence into one coherent runtime session snapshot.
//! Owns nothing. Never plans, prepares, restores, launches, or executes.
//! Intelligence remains the understanding envelope; Session is the UI runtime layer.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_session_summary, session_now_rfc3339, validate_session_workspace_id, ActorContext,
    AttentionCategory, IntentContext, SessionDecisionRef, SessionFocus, SessionHealth,
    SessionInterruption, SessionMember, SessionMemberKind, SessionMomentum, SessionReadinessView,
    SessionRecommendationRef, SessionRisk, SessionSummary, SessionTimelineItem,
    WorkspaceIntelligenceState, WorkspaceSessionComparison, WorkspaceSessionState,
    WorkspaceSessionSummary,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceIntelligenceService,
};

pub(crate) struct WorkspaceSessionService;

impl WorkspaceSessionService {
    /// Standalone generate — loads Intelligence once, then projects (no second cognition).
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceSessionState> {
        let workspace_id = workspace_id.into();
        let intelligence = WorkspaceIntelligenceService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            workspace_name,
            health_label,
        )?;
        Self::generate_with_inputs(db, actor, &intelligence)
    }

    /// Preferred path — project from an already-assembled Intelligence snapshot.
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
    ) -> Result<WorkspaceSessionState> {
        let _ = validate_session_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let ws = intelligence.workspace_id.as_str();
        let label = if !intelligence.purpose.label.is_empty() {
            intelligence.purpose.label.clone()
        } else if !intelligence.composition.label.is_empty() {
            intelligence.composition.label.clone()
        } else {
            intelligence.workspace_name.clone()
        };

        let focus = SessionFocus {
            project_label: intelligence.current_project.as_ref().map(|p| p.name.clone()),
            task_label: intelligence.current_task.as_ref().map(|t| t.title.clone()),
            purpose_label: intelligence.purpose.label.clone(),
            continuity_focus: intelligence
                .continuity
                .current_focus
                .as_ref()
                .map(|f| f.title.clone()),
            composition_label: Some(intelligence.composition.label.clone()),
            why: "Focus is projected from WorkflowContext, Continuity, Purpose, and Composition."
                .into(),
            source_projection: "workflow_context+continuity+purpose+composition".into(),
            authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
        };

        let mut members = Vec::new();
        if let Some(project) = &intelligence.current_project {
            members.push(SessionMember {
                id: format!("session:member:project:{}", project.id.as_str()),
                kind: SessionMemberKind::Project,
                label: project.name.clone(),
                why: "Active project from WorkflowContext.".into(),
                source_projection: "workflow_context".into(),
                source_ref: project.id.as_str().to_string(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if let Some(task) = &intelligence.current_task {
            members.push(SessionMember {
                id: format!("session:member:task:{}", task.id.as_str()),
                kind: SessionMemberKind::Task,
                label: task.title.clone(),
                why: "Active task from WorkflowContext.".into(),
                source_projection: "workflow_context".into(),
                source_ref: task.id.as_str().to_string(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if !intelligence.purpose.label.is_empty() {
            members.push(SessionMember {
                id: format!("session:member:purpose:{ws}"),
                kind: SessionMemberKind::Purpose,
                label: intelligence.purpose.label.clone(),
                why: "Purpose Model explains why current work exists.".into(),
                source_projection: "purpose".into(),
                source_ref: intelligence.purpose.workspace_id.clone(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for app in intelligence.current_applications.iter().take(6) {
            members.push(SessionMember {
                id: format!("session:member:app:{}", app.id),
                kind: SessionMemberKind::Application,
                label: app.name.clone(),
                why: "Environment / Intelligence current applications.".into(),
                source_projection: "environment".into(),
                source_ref: app.id.clone(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for pattern in intelligence.pattern.top_patterns.iter().take(3) {
            members.push(SessionMember {
                id: format!("session:member:pattern:{}", pattern.id),
                kind: SessionMemberKind::Pattern,
                label: pattern.title.clone(),
                why: "Pattern Model observation relevant to this session.".into(),
                source_projection: "pattern".into(),
                source_ref: pattern.id.clone(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for proposal in intelligence.adaptation.top_proposals.iter().take(2) {
            members.push(SessionMember {
                id: format!("session:member:adaptation:{}", proposal.id),
                kind: SessionMemberKind::Adaptation,
                label: proposal.title.clone(),
                why: "Adaptation Proposal opportunity (proposal only).".into(),
                source_projection: "adaptation".into(),
                source_ref: proposal.id.clone(),
                authority_effect: SessionMember::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        let timeline: Vec<SessionTimelineItem> = intelligence
            .recent_activity
            .iter()
            .take(10)
            .enumerate()
            .map(|(i, item)| SessionTimelineItem {
                id: format!("session:timeline:{i}:{}", item.timestamp),
                summary: item.summary.clone(),
                timestamp: item.timestamp.clone(),
                why: "Activity Graph / Intelligence recent activity.".into(),
                source_projection: "activity_graph".into(),
                source_ref: item.event_type.clone(),
            })
            .collect();

        let decisions: Vec<SessionDecisionRef> = intelligence
            .decision_queue
            .items
            .iter()
            .take(8)
            .map(|item| SessionDecisionRef {
                id: format!("session:decision:{}", item.id),
                title: item.title.clone(),
                summary: item.summary.clone(),
                priority_line: item.priority.as_str().to_string(),
                why: "Decision Queue remains the inbox SoT — Session only points here.".into(),
                source_projection: "decision_queue".into(),
                source_ref: item.id.as_str().to_string(),
            })
            .collect();

        let recommendations: Vec<SessionRecommendationRef> = intelligence
            .recommendation_engine
            .top_candidates
            .iter()
            .take(6)
            .map(|item| SessionRecommendationRef {
                id: format!("session:recommendation:{}", item.id),
                title: item.title.clone(),
                reason: item.reason.clone(),
                why: "Recommendation Engine remains independent — Session only points here."
                    .into(),
                source_projection: "recommendation_engine".into(),
                source_ref: item.id.clone(),
            })
            .collect();

        let readiness = SessionReadinessView {
            overall_status: intelligence.readiness.overall_status,
            status_line: intelligence
                .readiness
                .readiness_summary
                .status_line
                .clone(),
            gap_line: intelligence.readiness.readiness_summary.gap_line.clone(),
            gap_count: intelligence.readiness.gap_count,
            why: "Readiness Model remains independent — Session mirrors status only.".into(),
            source_projection: "readiness".into(),
        };

        let mut risks = Vec::new();
        for item in &intelligence.blocked_actions {
            risks.push(SessionRisk {
                id: format!("session:risk:blocked:{}", item.id),
                title: item.summary.clone(),
                explanation: item.explanation.clone(),
                why: "Blocked action projected from Decision Queue via Intelligence.".into(),
                source_projection: "decision_queue".into(),
                source_ref: item.id.clone(),
            });
        }
        for gap in intelligence.readiness.top_gaps.iter().take(4) {
            risks.push(SessionRisk {
                id: format!("session:risk:readiness:{}", gap.id),
                title: gap.title.clone(),
                explanation: gap.explanation.clone(),
                why: "Readiness gap — Session does not own or fix readiness.".into(),
                source_projection: "readiness".into(),
                source_ref: gap.id.clone(),
            });
        }
        for item in intelligence.attention.top_items.iter().take(4) {
            if matches!(
                item.category,
                AttentionCategory::Blocker | AttentionCategory::RequiresDecision
            ) || item.score >= 50
            {
                risks.push(SessionRisk {
                    id: format!("session:risk:attention:{}", item.id.as_str()),
                    title: item.title.clone(),
                    explanation: item.explanation.clone(),
                    why: "Attention priority surfaced as session risk context.".into(),
                    source_projection: "attention".into(),
                    source_ref: item.id.as_str().to_string(),
                });
            }
        }
        if risks.len() > 10 {
            risks.truncate(10);
        }

        let health = SessionHealth {
            kernel_health: intelligence.workspace_health.clone(),
            readiness_status: intelligence.readiness.overall_status,
            note: "Kernel health is lifecycle status; readiness is work preparedness. Session does not conflate them."
                .into(),
            why: "Health combines kernel WorkspaceHealth label with Readiness status.".into(),
            source_projection: "workspace_health+readiness".into(),
        };

        let mut interruptions = Vec::new();
        if intelligence.continuity.interrupted_count > 0 {
            if let Some(focus) = &intelligence.continuity.current_focus {
                interruptions.push(SessionInterruption {
                    id: format!("session:interruption:{}", focus.id.as_str()),
                    title: format!("Interrupted context near \"{}\"", focus.title),
                    summary: focus.summary.clone(),
                    why: "Continuity reports interrupted work — Session does not restore.".into(),
                    source_projection: "continuity".into(),
                    source_ref: focus.id.as_str().to_string(),
                });
            } else {
                interruptions.push(SessionInterruption {
                    id: format!("session:interruption:count:{ws}"),
                    title: format!(
                        "{} interrupted work item(s)",
                        intelligence.continuity.interrupted_count
                    ),
                    summary: intelligence.continuity.summary.clone(),
                    why: "Continuity interrupted_count — Session does not restore.".into(),
                    source_projection: "continuity".into(),
                    source_ref: intelligence.continuity.workspace_id.clone(),
                });
            }
        }
        let interruption_count = intelligence.continuity.interrupted_count.max(interruptions.len());

        let momentum = SessionMomentum {
            progress_line: format!(
                "Task Graph: {} active, {} blocked, {}% overall",
                intelligence.task_graph.active_count,
                intelligence.task_graph.blocked_count,
                intelligence.task_graph.progress_percent
            ),
            evolution_line: intelligence.evolution.summary.clone(),
            activity_count: intelligence.activity_graph.activity_count,
            open_task_count: intelligence.task_graph.active_count
                + intelligence.task_graph.blocked_count
                + intelligence.task_graph.waiting_count,
            why: "Momentum is projected from Task Graph, Evolution, and Activity Graph.".into(),
            source_projection: "task_graph+evolution+activity_graph".into(),
        };

        let doing_line = match (
            &focus.project_label,
            &focus.task_label,
            &focus.continuity_focus,
        ) {
            (Some(p), Some(t), _) => format!("You were working on {p} / {t}."),
            (Some(p), None, _) => format!("You were working on project {p}."),
            (_, _, Some(c)) => format!("Continuity focus: {c}."),
            _ => format!("No active work focus set for \"{label}\"."),
        };
        let matters_line = if !intelligence.attention.summary.is_empty() {
            intelligence.attention.summary.clone()
        } else if intelligence.decision_queue.pending_count > 0 {
            format!(
                "{} decision(s) need attention.",
                intelligence.decision_queue.pending_count
            )
        } else {
            "Nothing urgent in Attention or Decision Queue.".into()
        };
        let blocked_line = if risks.is_empty() {
            "No blockers projected right now.".into()
        } else {
            format!(
                "{} risk(s)/blocker(s): {}",
                risks.len(),
                risks
                    .iter()
                    .take(2)
                    .map(|r| r.title.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        };
        let ready_line = readiness.status_line.clone();
        let changed_line = if !intelligence.operating_state.summary.is_empty() {
            intelligence.operating_state.operating_summary.progress_line.clone()
        } else {
            intelligence.evolution.summary.clone()
        };

        let session_summary = SessionSummary {
            headline: format!("Session — {label}"),
            doing_line: doing_line.clone(),
            matters_line: matters_line.clone(),
            blocked_line: blocked_line.clone(),
            ready_line: ready_line.clone(),
            changed_line: changed_line.clone(),
            narrative: format!(
                "{doing_line} {matters_line} {blocked_line} {ready_line} \
                 Session orchestrates existing projections only — never executes."
            ),
        };

        let mut evidence = vec![
            format!("Intelligence generated_at: {}", intelligence.generated_at),
            format!("Decision Queue pending: {}", intelligence.decision_queue.pending_count),
            format!("Recommendation candidates: {}", intelligence.recommendation_engine.candidate_count),
            format!("Readiness: {}", intelligence.readiness.overall_status.as_str()),
            format!("Operating signals: {}", intelligence.operating_state.signal_count),
            format!("Patterns: {}", intelligence.pattern.pattern_count),
            format!("Adaptation open: {}", intelligence.adaptation.open_count),
            format!("Continuity interrupted: {}", intelligence.continuity.interrupted_count),
        ];
        evidence.push(format!("Composition: {}", intelligence.composition.label));

        let explanation = format!(
            "Session for \"{label}\" is projected from Workspace Intelligence \
             (Purpose, Continuity, Environment, Composition, Task Graph, Decision Queue, \
             Attention, Recommendation Engine, Operating State, Pattern, Readiness, \
             Adaptation, Activity, Evolution). Session owns no source data and never \
             launches, restores, prepares, or bypasses the Gateway."
        );
        let summary = build_session_summary(&label, decisions.len(), risks.len());

        let state = WorkspaceSessionState {
            workspace_id: ws.to_string(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: session_now_rfc3339(),
            label,
            member_count: members.len(),
            decision_count: decisions.len(),
            recommendation_count: recommendations.len(),
            risk_count: risks.len(),
            interruption_count,
            session_summary,
            members,
            focus,
            timeline,
            decisions,
            recommendations,
            readiness,
            risks,
            health,
            interruptions,
            momentum,
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceSessionState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_projected(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceSessionState,
        limit: usize,
    ) -> WorkspaceSessionSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceSessionState,
        right: &WorkspaceSessionState,
    ) -> WorkspaceSessionComparison {
        WorkspaceSessionComparison::compare(left, right)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceSessionError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceSessionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.session.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "decision_count": state.decision_count,
                "risk_count": state.risk_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_projected(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceSessionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.session.projected",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "intelligence_generated_at": state.intelligence_generated_at,
                "member_count": state.member_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceSessionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.session.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "readiness_status": state.readiness.overall_status.as_str(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
