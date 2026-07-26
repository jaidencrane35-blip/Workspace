//! Workspace Experience Layer (Phase 6).
//!
//! Projects WorkspaceSessionState into calm Work-surface presentation groups.
//! Owns nothing. Deterministic visibility only — never cognition, never authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_experience_summary, experience_now_rfc3339, validate_experience_workspace_id,
    ActorContext, ExperienceItem, ExperienceSection, ExperienceSectionKind, ExperienceSummary,
    ExperienceVisibility, IntentContext, ReadinessStatus, WorkspaceExperienceComparison,
    WorkspaceExperienceState, WorkspaceExperienceSummary, WorkspaceSessionState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceSessionService,
};

pub(crate) struct WorkspaceExperienceService;

impl WorkspaceExperienceService {
    /// Standalone generate — loads Session (via Intelligence), then presents.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceExperienceState> {
        let session = WorkspaceSessionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            workspace_name,
            health_label,
        )?;
        Self::generate_with_inputs(db, actor, &session)
    }

    /// Preferred path — project from an already-assembled Session snapshot.
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        session: &WorkspaceSessionState,
    ) -> Result<WorkspaceExperienceState> {
        let _ = validate_experience_workspace_id(session.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();

        let mut sections = Vec::new();

        // Primary Focus — always immediate.
        sections.push(section(
            ExperienceSectionKind::PrimaryFocus,
            ExperienceVisibility::Immediate,
            "session.focus",
            "Primary focus is always visible so users know what they are working on.",
            vec![ExperienceItem {
                id: format!("experience:focus:{}", session.workspace_id),
                title: match (
                    &session.focus.project_label,
                    &session.focus.task_label,
                ) {
                    (Some(p), Some(t)) => format!("{p} / {t}"),
                    (Some(p), None) => p.clone(),
                    (_, Some(t)) => t.clone(),
                    _ => label.clone(),
                },
                summary: session.session_summary.doing_line.clone(),
                visibility: ExperienceVisibility::Immediate,
                why: session.focus.why.clone(),
                source_session_field: "focus".into(),
                source_ref: session.workspace_id.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            }],
        ));

        // Today's Work — members (project/task/purpose/apps), collapse if empty-ish.
        let work_items: Vec<ExperienceItem> = session
            .members
            .iter()
            .filter(|m| {
                matches!(
                    m.kind,
                    workspace_domain::SessionMemberKind::Project
                        | workspace_domain::SessionMemberKind::Task
                        | workspace_domain::SessionMemberKind::Purpose
                        | workspace_domain::SessionMemberKind::Application
                )
            })
            .take(8)
            .map(|m| ExperienceItem {
                id: format!("experience:work:{}", m.id),
                title: m.label.clone(),
                summary: m.why.clone(),
                visibility: ExperienceVisibility::Immediate,
                why: "Session member projected into Today's Work.".into(),
                source_session_field: "members".into(),
                source_ref: m.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        let work_vis = if work_items.is_empty() {
            ExperienceVisibility::Collapsed
        } else {
            ExperienceVisibility::Immediate
        };
        sections.push(section(
            ExperienceSectionKind::TodaysWork,
            work_vis,
            "session.members",
            "Today's Work groups active Session members for calm scanning.",
            work_items,
        ));

        // Session decisions — Decision Queue pointers from Session (not Attention Engine).
        // Vocabulary: distinct from Workspace Attention prioritization.
        let attention_items: Vec<ExperienceItem> = session
            .decisions
            .iter()
            .take(5)
            .map(|d| ExperienceItem {
                id: format!("experience:attention:{}", d.id),
                title: d.title.clone(),
                summary: d.summary.clone(),
                visibility: if d.priority_line == "critical" || d.priority_line == "high" {
                    ExperienceVisibility::Highlighted
                } else {
                    ExperienceVisibility::Immediate
                },
                why: d.why.clone(),
                source_session_field: "decisions".into(),
                source_ref: d.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        let attention_vis = if attention_items.is_empty() {
            ExperienceVisibility::Collapsed
        } else if attention_items
            .iter()
            .any(|i| i.visibility == ExperienceVisibility::Highlighted)
        {
            ExperienceVisibility::Highlighted
        } else {
            ExperienceVisibility::Immediate
        };
        sections.push(section(
            ExperienceSectionKind::SuggestedAttention,
            attention_vis,
            "session.decisions",
            "Session decisions present Decision Queue pointers from Session — \
             not the Attention Engine prioritization layer.",
            attention_items,
        ));

        // Waiting On — interruptions / continuity waiting (collapse if none).
        let waiting_items: Vec<ExperienceItem> = session
            .interruptions
            .iter()
            .take(4)
            .map(|i| ExperienceItem {
                id: format!("experience:waiting:{}", i.id),
                title: i.title.clone(),
                summary: i.summary.clone(),
                visibility: ExperienceVisibility::Immediate,
                why: i.why.clone(),
                source_session_field: "interruptions".into(),
                source_ref: i.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        sections.push(section(
            ExperienceSectionKind::WaitingOn,
            if waiting_items.is_empty() {
                ExperienceVisibility::Collapsed
            } else {
                ExperienceVisibility::Immediate
            },
            "session.interruptions",
            "Waiting On surfaces Continuity interruptions from Session.",
            waiting_items,
        ));

        // Blocked Work — risks (highlighted if any).
        let blocked_items: Vec<ExperienceItem> = session
            .risks
            .iter()
            .take(6)
            .map(|r| ExperienceItem {
                id: format!("experience:blocked:{}", r.id),
                title: r.title.clone(),
                summary: r.explanation.clone(),
                visibility: ExperienceVisibility::Highlighted,
                why: r.why.clone(),
                source_session_field: "risks".into(),
                source_ref: r.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        sections.push(section(
            ExperienceSectionKind::BlockedWork,
            if blocked_items.is_empty() {
                ExperienceVisibility::Collapsed
            } else {
                ExperienceVisibility::Highlighted
            },
            "session.risks",
            "Blocked Work highlights Session risks without granting authority.",
            blocked_items,
        ));

        // Recent Progress — timeline (deferred if long/quiet).
        let progress_items: Vec<ExperienceItem> = session
            .timeline
            .iter()
            .take(5)
            .map(|t| ExperienceItem {
                id: format!("experience:progress:{}", t.id),
                title: t.summary.clone(),
                summary: t.timestamp.clone(),
                visibility: ExperienceVisibility::Deferred,
                why: t.why.clone(),
                source_session_field: "timeline".into(),
                source_ref: t.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        sections.push(section(
            ExperienceSectionKind::RecentProgress,
            if progress_items.is_empty() {
                ExperienceVisibility::Collapsed
            } else {
                ExperienceVisibility::Deferred
            },
            "session.timeline",
            "Recent Progress is deferred so focus stays on what matters now.",
            progress_items,
        ));

        // Recommended Next Step — top recommendation (immediate/highlighted).
        let next_items: Vec<ExperienceItem> = session
            .recommendations
            .iter()
            .take(1)
            .map(|r| ExperienceItem {
                id: format!("experience:next:{}", r.id),
                title: r.title.clone(),
                summary: r.reason.clone(),
                visibility: ExperienceVisibility::Highlighted,
                why: r.why.clone(),
                source_session_field: "recommendations".into(),
                source_ref: r.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        sections.push(section(
            ExperienceSectionKind::RecommendedNextStep,
            if next_items.is_empty() {
                ExperienceVisibility::Collapsed
            } else {
                ExperienceVisibility::Highlighted
            },
            "session.recommendations",
            "Recommended Next Step presents the top Recommendation Engine pointer.",
            next_items,
        ));

        // Helpful Improvements — adaptation members (collapsed by default).
        let improve_items: Vec<ExperienceItem> = session
            .members
            .iter()
            .filter(|m| m.kind == workspace_domain::SessionMemberKind::Adaptation)
            .take(3)
            .map(|m| ExperienceItem {
                id: format!("experience:improve:{}", m.id),
                title: m.label.clone(),
                summary: m.why.clone(),
                visibility: ExperienceVisibility::Collapsed,
                why: "Adaptation opportunities stay collapsed until the user expands them.".into(),
                source_session_field: "members.adaptation".into(),
                source_ref: m.source_ref.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            })
            .collect();
        sections.push(section(
            ExperienceSectionKind::HelpfulImprovements,
            ExperienceVisibility::Collapsed,
            "session.members.adaptation",
            "Helpful Improvements remain collapsed — Adaptations stay proposals only.",
            improve_items,
        ));

        // Session Health — readiness + kernel health.
        let health_vis = match session.readiness.overall_status {
            ReadinessStatus::Blocked => ExperienceVisibility::Highlighted,
            ReadinessStatus::PartiallyReady => ExperienceVisibility::Immediate,
            ReadinessStatus::Ready => ExperienceVisibility::Immediate,
        };
        sections.push(section(
            ExperienceSectionKind::SessionHealth,
            health_vis,
            "session.health+readiness",
            "Session Health distinguishes kernel lifecycle from work readiness.",
            vec![ExperienceItem {
                id: format!("experience:health:{}", session.workspace_id),
                title: format!(
                    "Readiness: {} · Kernel: {}",
                    session.readiness.overall_status.as_str(),
                    session.health.kernel_health
                ),
                summary: session.readiness.status_line.clone(),
                visibility: health_vis,
                why: session.health.why.clone(),
                source_session_field: "health".into(),
                source_ref: session.workspace_id.clone(),
                authority_effect: ExperienceItem::AUTHORITY_EFFECT_NONE.into(),
            }],
        ));

        let immediate_count = sections
            .iter()
            .filter(|s| s.visibility == ExperienceVisibility::Immediate)
            .count();
        let highlighted_count = sections
            .iter()
            .filter(|s| s.visibility == ExperienceVisibility::Highlighted)
            .count();
        let collapsed_count = sections
            .iter()
            .filter(|s| s.visibility == ExperienceVisibility::Collapsed)
            .count();
        let deferred_count = sections
            .iter()
            .filter(|s| s.visibility == ExperienceVisibility::Deferred)
            .count();

        let next_line = session
            .recommendations
            .first()
            .map(|r| r.title.clone())
            .unwrap_or_else(|| "No recommended next step right now.".into());

        let experience_summary = ExperienceSummary {
            headline: format!("Workspace — {label}"),
            focus_line: session.session_summary.doing_line.clone(),
            matters_line: session.session_summary.matters_line.clone(),
            blocked_line: session.session_summary.blocked_line.clone(),
            ready_line: session.session_summary.ready_line.clone(),
            next_line: next_line.clone(),
            narrative: format!(
                "{} {} {} Next: {}. Experience presents Session only — never executes.",
                session.session_summary.doing_line,
                session.session_summary.matters_line,
                session.session_summary.blocked_line,
                next_line
            ),
        };

        let evidence = vec![
            format!("Session generated_at: {}", session.generated_at),
            format!("Session decisions: {}", session.decision_count),
            format!("Session risks: {}", session.risk_count),
            format!("Session recommendations: {}", session.recommendation_count),
            format!(
                "Readiness: {}",
                session.readiness.overall_status.as_str()
            ),
            format!("Immediate sections: {immediate_count}"),
            format!("Highlighted sections: {highlighted_count}"),
        ];

        let explanation = format!(
            "Experience for \"{label}\" is projected solely from WorkspaceSessionState. \
             Sections are deterministic presentation groupings (Immediate / Highlighted / \
             Collapsed / Deferred). Experience owns no cognition data and never launches, \
             restores, prepares, or bypasses the Gateway."
        );
        let summary = build_experience_summary(&label, immediate_count, highlighted_count);

        let state = WorkspaceExperienceState {
            workspace_id: session.workspace_id.clone(),
            workspace_name: session.workspace_name.clone(),
            generated_at: experience_now_rfc3339(),
            label,
            experience_summary,
            section_count: sections.len(),
            immediate_count,
            highlighted_count,
            collapsed_count,
            deferred_count,
            sections,
            session_generated_at: session.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceExperienceState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_projected(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceExperienceState,
        limit: usize,
    ) -> WorkspaceExperienceSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceExperienceState,
        right: &WorkspaceExperienceState,
    ) -> WorkspaceExperienceComparison {
        WorkspaceExperienceComparison::compare(left, right)
    }

    /// Reference Working Style as evidence only — Experience never owns style.
    pub(crate) fn enrich_with_working_style(
        experience: &WorkspaceExperienceState,
        working_style: &workspace_domain::WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceExperienceState> {
        let mut next = experience.clone();
        let marker = format!(
            "working_style:obs={},prefs={}",
            working_style.observation_count, working_style.preference_count
        );
        if !next.evidence.iter().any(|e| e.starts_with("working_style:")) {
            next.evidence.push(marker.clone());
        }
        next.explanation = format!(
            "{} References Working Style as evidence only ({}) — Experience remains presentation only.",
            next.explanation, marker
        );
        Ok(next)
    }

    /// Reference Transitions as evidence only — Experience never restores.
    pub(crate) fn enrich_with_transition(
        experience: &WorkspaceExperienceState,
        transition: &workspace_domain::WorkspaceTransitionState,
    ) -> Result<WorkspaceExperienceState> {
        let mut next = experience.clone();
        let marker = format!(
            "transition:count={},returning={}",
            transition.transition_count, transition.returning_count
        );
        if !next.evidence.iter().any(|e| e.starts_with("transition:")) {
            next.evidence.push(marker.clone());
        }
        next.explanation = format!(
            "{} References Transitions as evidence only ({}) — Experience never restores or executes.",
            next.explanation, marker
        );
        Ok(next)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceExperienceError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceExperienceState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.experience.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "section_count": state.section_count,
                "immediate_count": state.immediate_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_projected(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceExperienceState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.experience.projected",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "session_generated_at": state.session_generated_at,
                "highlighted_count": state.highlighted_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceExperienceState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.experience.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "collapsed_count": state.collapsed_count,
                "deferred_count": state.deferred_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn section(
    kind: ExperienceSectionKind,
    visibility: ExperienceVisibility,
    source_session_field: &str,
    why: &str,
    items: Vec<ExperienceItem>,
) -> ExperienceSection {
    let item_count = items.len();
    let collapsed_hint = match visibility {
        ExperienceVisibility::Collapsed if item_count == 0 => {
            Some(format!("No {} items right now.", kind.title().to_lowercase()))
        }
        ExperienceVisibility::Collapsed => Some(format!(
            "{item_count} item(s) collapsed — expand when needed."
        )),
        ExperienceVisibility::Deferred => Some(format!(
            "{item_count} item(s) deferred from the primary surface."
        )),
        _ => None,
    };
    ExperienceSection {
        kind,
        title: kind.title().into(),
        visibility,
        items,
        item_count,
        collapsed_hint,
        why: why.into(),
        source_session_field: source_session_field.into(),
    }
}
