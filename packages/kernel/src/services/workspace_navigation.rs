//! Workspace Navigation Engine (Phase 6).
//!
//! Interaction projection over Intelligence + Session + Experience + Work Context.
//! Owns nothing. Deterministic paths only — never AI routing, never authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_navigation_summary, navigation_now_rfc3339, validate_navigation_workspace_id,
    ActorContext, ExperienceSectionKind, IntentContext, NavigationEdge, NavigationNode,
    NavigationPath, NavigationPathKind, NavigationRelationKind, NavigationSummary,
    WorkspaceExperienceState, WorkspaceIntelligenceState, WorkspaceNavigationComparison,
    WorkspaceNavigationState, WorkspaceNavigationSummary, WorkspaceNavigationValidation,
    WorkspaceSessionState, WorkspaceWorkContextState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceSessionService, WorkspaceWorkContextService,
};

pub(crate) struct WorkspaceNavigationService;

impl WorkspaceNavigationService {
    /// Standalone generate — loads Intelligence stack, then projects navigation.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceNavigationState> {
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
        Self::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
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
    ) -> Result<WorkspaceNavigationState> {
        let _ = validate_navigation_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();
        let ws = intelligence.workspace_id.as_str();

        let mut nodes: Vec<NavigationNode> = Vec::new();
        let mut edges: Vec<NavigationEdge> = Vec::new();
        let mut breadcrumbs: Vec<NavigationNode> = Vec::new();

        // Current focus
        let focus_label = match (
            &session.focus.project_label,
            &session.focus.task_label,
        ) {
            (Some(p), Some(t)) => format!("{p} / {t}"),
            (Some(p), None) => p.clone(),
            (_, Some(t)) => t.clone(),
            _ => label.clone(),
        };
        let focus_id = format!("nav:focus:{ws}");
        let focus_node = NavigationNode {
            id: focus_id.clone(),
            label: focus_label.clone(),
            kind: NavigationPathKind::CurrentFocus,
            summary: session.session_summary.doing_line.clone(),
            why: session.focus.why.clone(),
            source_projection: "session.focus".into(),
            source_ref: ws.into(),
            authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
        };
        breadcrumbs.push(NavigationNode {
            id: format!("nav:crumb:workspace:{ws}"),
            label: intelligence.workspace_name.clone(),
            kind: NavigationPathKind::Breadcrumb,
            summary: "Workspace root".into(),
            why: "Breadcrumb starts at the Workspace".into(),
            source_projection: "intelligence".into(),
            source_ref: ws.into(),
            authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
        });
        if let Some(p) = &session.focus.project_label {
            breadcrumbs.push(NavigationNode {
                id: format!("nav:crumb:project:{ws}"),
                label: p.clone(),
                kind: NavigationPathKind::Breadcrumb,
                summary: "Active project".into(),
                why: "Project is part of the current focus path".into(),
                source_projection: "session.focus".into(),
                source_ref: ws.into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if let Some(t) = &session.focus.task_label {
            breadcrumbs.push(NavigationNode {
                id: format!("nav:crumb:task:{ws}"),
                label: t.clone(),
                kind: NavigationPathKind::Breadcrumb,
                summary: "Active task".into(),
                why: "Task is the tip of the current focus path".into(),
                source_projection: "session.focus".into(),
                source_ref: ws.into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        nodes.push(focus_node);
        edges.push(NavigationEdge {
            id: format!("nav:edge:current:{ws}"),
            from_node_id: focus_id.clone(),
            to_node_id: focus_id.clone(),
            kind: NavigationRelationKind::Current,
            why: "This is the user's current focus".into(),
            authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
        });

        // Connected projects / tasks / contexts
        for (i, project) in work_context
            .primary_context()
            .into_iter()
            .flat_map(|c| c.associated_projects.iter())
            .take(5)
            .enumerate()
        {
            let id = format!("nav:project:{}:{i}", project.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: project.label.clone(),
                kind: NavigationPathKind::ConnectedProject,
                summary: format!("Project linked via {}", project.source_projection),
                why: project.why.clone(),
                source_projection: project.source_projection.clone(),
                source_ref: project.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:related-project:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::Related,
                why: "Project is associated with the current work context".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for (i, task) in work_context
            .primary_context()
            .into_iter()
            .flat_map(|c| c.associated_tasks.iter())
            .take(5)
            .enumerate()
        {
            let id = format!("nav:task:{}:{i}", task.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: task.label.clone(),
                kind: NavigationPathKind::ConnectedTask,
                summary: format!("Task linked via {}", task.source_projection),
                why: task.why.clone(),
                source_projection: task.source_projection.clone(),
                source_ref: task.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:related-task:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::Related,
                why: "Task is associated with the current work context".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for (i, ctx) in work_context.contexts.iter().take(4).enumerate() {
            let id = format!("nav:context:{}", ctx.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: ctx.name.clone(),
                kind: NavigationPathKind::ConnectedContext,
                summary: format!("{} · {}", ctx.context_type.as_str(), ctx.current_status.as_str()),
                why: ctx.why.clone(),
                source_projection: "work_context".into(),
                source_ref: ctx.id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            let rel = if Some(&ctx.id) == work_context.primary_context_id.as_ref() {
                NavigationRelationKind::Current
            } else if ctx.current_status.as_str() == "dormant" {
                NavigationRelationKind::Dormant
            } else {
                NavigationRelationKind::Related
            };
            edges.push(NavigationEdge {
                id: format!("nav:edge:context:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: rel,
                why: "Work Context participates in the navigation map".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Blocking items from session risks + readiness gaps
        let mut blocked_count = 0usize;
        for (i, risk) in session.risks.iter().take(5).enumerate() {
            let id = format!("nav:block:risk:{}", risk.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: risk.title.clone(),
                kind: NavigationPathKind::BlockingItem,
                summary: risk.explanation.clone(),
                why: risk.why.clone(),
                source_projection: risk.source_projection.clone(),
                source_ref: risk.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:blocked-by-risk:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::BlockedBy,
                why: "Session risk indicates a blocked navigation path".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            blocked_count += 1;
        }
        for (i, gap) in intelligence.readiness.top_gaps.iter().take(4).enumerate() {
            let id = format!("nav:block:gap:{}", gap.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: gap.title.clone(),
                kind: NavigationPathKind::BlockingItem,
                summary: gap.explanation.clone(),
                why: format!("Readiness gap: {}", gap.explanation),
                source_projection: format!("readiness:{}", gap.source_model),
                source_ref: gap.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:blocked-by-gap:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::BlockedBy,
                why: "Readiness gap should be inspected before proceeding".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            blocked_count += 1;
        }

        // Decisions
        for (i, decision) in session.decisions.iter().take(5).enumerate() {
            let id = format!("nav:decision:{}", decision.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: decision.title.clone(),
                kind: NavigationPathKind::RelevantDecision,
                summary: decision.summary.clone(),
                why: decision.why.clone(),
                source_projection: decision.source_projection.clone(),
                source_ref: decision.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:decision:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::LeadsTo,
                why: "Decision Queue item deserves inspection".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Recommendations
        let mut suggested_count = 0usize;
        for (i, rec) in session.recommendations.iter().take(4).enumerate() {
            let id = format!("nav:rec:{}", rec.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: rec.title.clone(),
                kind: NavigationPathKind::RelevantRecommendation,
                summary: rec.reason.clone(),
                why: rec.why.clone(),
                source_projection: rec.source_projection.clone(),
                source_ref: rec.source_ref.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:rec:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::SuggestedNext,
                why: "Recommendation Engine candidate is a suggested next inspection".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            suggested_count += 1;
        }

        // Suggested destination / next inspection from Experience
        let next_line = experience.experience_summary.next_line.clone();
        if !next_line.trim().is_empty() {
            let id = format!("nav:next:{ws}");
            nodes.push(NavigationNode {
                id: id.clone(),
                label: next_line.clone(),
                kind: NavigationPathKind::SuggestedDestination,
                summary: experience.experience_summary.matters_line.clone(),
                why: "Experience Layer highlights the recommended next step for presentation"
                    .into(),
                source_projection: "experience".into(),
                source_ref: experience.workspace_id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:suggested:{ws}"),
                from_node_id: focus_id.clone(),
                to_node_id: id.clone(),
                kind: NavigationRelationKind::SuggestedNext,
                why: "Suggested destination from Experience next-line".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            nodes.push(NavigationNode {
                id: format!("nav:inspect:{ws}"),
                label: next_line,
                kind: NavigationPathKind::PossibleNextInspection,
                summary: "Inspect this before planning or execution".into(),
                why: "Navigation guides inspection only — never executes".into(),
                source_projection: "experience".into(),
                source_ref: experience.workspace_id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            suggested_count += 1;
        }

        // Related work from purpose / composition
        if !intelligence.purpose.label.is_empty() {
            let id = format!("nav:related:purpose:{ws}");
            nodes.push(NavigationNode {
                id: id.clone(),
                label: intelligence.purpose.label.clone(),
                kind: NavigationPathKind::RelatedWork,
                summary: intelligence.purpose.summary.clone(),
                why: "Purpose explains why related work belongs together".into(),
                source_projection: "purpose".into(),
                source_ref: intelligence.purpose.workspace_id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:related-purpose"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::Supports,
                why: "Purpose supports the current focus".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if let Some(focus) = &intelligence.composition.focus_label {
            let id = format!("nav:related:composition:{ws}");
            nodes.push(NavigationNode {
                id: id.clone(),
                label: focus.clone(),
                kind: NavigationPathKind::RelatedWork,
                summary: intelligence.composition.summary.clone(),
                why: "Composition focus is related working-environment work".into(),
                source_projection: "composition".into(),
                source_ref: intelligence.composition.workspace_id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:related-composition"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::Related,
                why: "Composition relates environment to focus".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Recent changes from evolution / experience recent progress
        for (i, insight) in intelligence.evolution.top_insights.iter().take(3).enumerate() {
            let id = format!("nav:recent:evo:{}", insight.id);
            nodes.push(NavigationNode {
                id: id.clone(),
                label: insight.title.clone(),
                kind: NavigationPathKind::RecentChange,
                summary: intelligence.evolution.summary.clone(),
                why: "Evolution insight marks a recent change worth revisiting".into(),
                source_projection: "evolution".into(),
                source_ref: insight.id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:recent-evo:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::RecentlyVisited,
                why: "Recent evolution change is informational navigation history".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for section in experience.sections.iter() {
            if section.kind == ExperienceSectionKind::RecentProgress {
                for (i, item) in section.items.iter().take(3).enumerate() {
                    let id = format!("nav:recent:exp:{}", item.id);
                    nodes.push(NavigationNode {
                        id: id.clone(),
                        label: item.title.clone(),
                        kind: NavigationPathKind::RecentChange,
                        summary: item.summary.clone(),
                        why: item.why.clone(),
                        source_projection: "experience.recent_progress".into(),
                        source_ref: item.source_ref.clone(),
                        authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
                    });
                    edges.push(NavigationEdge {
                        id: format!("nav:edge:recent-exp:{i}"),
                        from_node_id: focus_id.clone(),
                        to_node_id: id,
                        kind: NavigationRelationKind::RecentlyVisited,
                        why: "Experience recent progress is a navigation breadcrumb candidate"
                            .into(),
                        authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
                    });
                }
            }
        }

        // Dependency chain from task graph top nodes
        for (i, node) in intelligence.task_graph.top_nodes.iter().take(4).enumerate() {
            let id = format!("nav:dep:{}", node.task.id);
            let dep_summary = if node.dependency_ids.is_empty() {
                "No upstream dependencies listed".into()
            } else {
                format!("Depends on {} task(s)", node.dependency_ids.len())
            };
            nodes.push(NavigationNode {
                id: id.clone(),
                label: node.task.title.clone(),
                kind: NavigationPathKind::DependencyChain,
                summary: dep_summary,
                why: "Task Graph node participates in a dependency chain (inspection only)".into(),
                source_projection: "task_graph".into(),
                source_ref: node.task.id.to_string(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            let rel = if node.blocker_ids.is_empty() {
                NavigationRelationKind::DependsOn
            } else {
                NavigationRelationKind::BlockedBy
            };
            edges.push(NavigationEdge {
                id: format!("nav:edge:dep:{i}"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: rel,
                why: "Task Graph relationship is informational only".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            if !node.blocker_ids.is_empty() {
                blocked_count += 1;
            }
        }

        // Continuity interrupted → dormant/disconnected signal
        if intelligence
            .continuity
            .summary
            .to_lowercase()
            .contains("interrupt")
        {
            let id = format!("nav:dormant:continuity:{ws}");
            nodes.push(NavigationNode {
                id: id.clone(),
                label: "Interrupted continuity".into(),
                kind: NavigationPathKind::RelatedWork,
                summary: intelligence.continuity.summary.clone(),
                why: "Continuity reports interrupted work — dormant relative path".into(),
                source_projection: "continuity".into(),
                source_ref: intelligence.continuity.workspace_id.clone(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
            edges.push(NavigationEdge {
                id: format!("nav:edge:dormant-continuity"),
                from_node_id: focus_id.clone(),
                to_node_id: id,
                kind: NavigationRelationKind::Dormant,
                why: "Interrupted work is dormant relative to current focus".into(),
                authority_effect: NavigationNode::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        let paths = group_paths(&nodes);
        let related_line = path_line(&paths, NavigationPathKind::RelatedWork, "No related work");
        let blocked_line = if blocked_count == 0 {
            "No blocked navigation paths".into()
        } else {
            format!("{blocked_count} blocked path signal(s)")
        };
        let next_inspection = nodes
            .iter()
            .find(|n| n.kind == NavigationPathKind::PossibleNextInspection)
            .map(|n| n.label.clone())
            .unwrap_or_else(|| "Inspect current focus".into());
        let breadcrumb_line = breadcrumbs
            .iter()
            .map(|b| b.label.as_str())
            .collect::<Vec<_>>()
            .join(" › ");

        let navigation_summary = NavigationSummary {
            headline: format!("Navigate {label}"),
            current_path_line: focus_label,
            related_line,
            blocked_line: blocked_line.clone(),
            next_inspection_line: next_inspection.clone(),
            breadcrumb_line: breadcrumb_line.clone(),
            narrative: format!(
                "You are at {}. {}. Next inspection: {}. Navigation guides inspection only.",
                session.session_summary.doing_line, blocked_line, next_inspection
            ),
        };

        let evidence = vec![
            format!("Intelligence generated_at: {}", intelligence.generated_at),
            format!("Session generated_at: {}", session.generated_at),
            format!("Experience generated_at: {}", experience.generated_at),
            format!("Work Context generated_at: {}", work_context.generated_at),
            format!("Task Graph nodes: {}", intelligence.task_graph.node_count),
            format!("Decisions: {}", session.decision_count),
            format!("Blocked signals: {blocked_count}"),
            format!("Suggested signals: {suggested_count}"),
        ];

        let explanation = format!(
            "Navigation for \"{label}\" is an interaction projection over WorkspaceIntelligenceState, \
             WorkspaceSessionState, WorkspaceExperienceState, and WorkspaceWorkContextState (plus \
             Task Graph, Decision Queue, Composition, Purpose, Environment, Evolution, Continuity, \
             Recommendations, Readiness, Patterns via Intelligence). Paths are deterministic and \
             explainable. Navigation owns no cognition data and never plans, launches, routes \
             autonomously, or bypasses the Gateway."
        );
        let summary = build_navigation_summary(&label, nodes.len(), blocked_count);

        let state = WorkspaceNavigationState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: navigation_now_rfc3339(),
            label,
            path_count: paths.len(),
            node_count: nodes.len(),
            edge_count: edges.len(),
            blocked_count,
            suggested_count,
            navigation_summary,
            paths,
            nodes,
            edges,
            breadcrumbs,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            work_context_generated_at: work_context.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceNavigationState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceNavigationState,
        limit: usize,
    ) -> WorkspaceNavigationSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceNavigationState,
        right: &WorkspaceNavigationState,
    ) -> WorkspaceNavigationComparison {
        WorkspaceNavigationComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceNavigationState) -> WorkspaceNavigationValidation {
        WorkspaceNavigationValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceNavigationState,
    ) -> Result<WorkspaceNavigationValidation> {
        let report = Self::validate(state);
        Self::audit_validated(db, actor, state, &report)?;
        Ok(report)
    }

    /// Reference Milestones as evidence only — Navigation never owns milestones.
    pub(crate) fn enrich_with_milestones(
        navigation: &WorkspaceNavigationState,
        milestones: &workspace_domain::WorkspaceMilestoneState,
    ) -> Result<WorkspaceNavigationState> {
        let mut next = navigation.clone();
        let marker = format!(
            "milestones:current={:?},count={}",
            milestones.current_milestone().map(|m| &m.title),
            milestones.milestone_count
        );
        if !next.evidence.iter().any(|e| e.starts_with("milestones:")) {
            next.evidence.push(marker.clone());
        }
        next.explanation = format!(
            "{} References Milestones as evidence only ({}) — Navigation does not own milestones.",
            next.explanation, marker
        );
        Ok(next)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceNavigationError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceNavigationState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.navigation.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "node_count": state.node_count,
                "blocked_count": state.blocked_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceNavigationState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.navigation.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "path_count": state.path_count,
                "suggested_count": state.suggested_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_validated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceNavigationState,
        report: &WorkspaceNavigationValidation,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.navigation.validated",
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

fn group_paths(nodes: &[NavigationNode]) -> Vec<NavigationPath> {
    let mut paths = Vec::new();
    for kind in [
        NavigationPathKind::CurrentFocus,
        NavigationPathKind::SuggestedDestination,
        NavigationPathKind::RelatedWork,
        NavigationPathKind::BlockingItem,
        NavigationPathKind::ConnectedTask,
        NavigationPathKind::ConnectedProject,
        NavigationPathKind::ConnectedContext,
        NavigationPathKind::RelevantDecision,
        NavigationPathKind::RelevantRecommendation,
        NavigationPathKind::RecentChange,
        NavigationPathKind::PossibleNextInspection,
        NavigationPathKind::DependencyChain,
        NavigationPathKind::Breadcrumb,
    ] {
        let group: Vec<_> = nodes.iter().filter(|n| n.kind == kind).cloned().collect();
        if group.is_empty() {
            continue;
        }
        paths.push(NavigationPath {
            kind,
            title: kind.title().into(),
            node_count: group.len(),
            why: format!("{} grouped for inspection-only navigation", kind.title()),
            nodes: group,
        });
    }
    paths
}

fn path_line(paths: &[NavigationPath], kind: NavigationPathKind, empty: &str) -> String {
    paths
        .iter()
        .find(|p| p.kind == kind)
        .and_then(|p| p.nodes.first().map(|n| n.label.clone()))
        .unwrap_or_else(|| empty.into())
}
