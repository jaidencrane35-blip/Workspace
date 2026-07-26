//! Workspace Attention Engine (Phase 5 Batch 2 / Sprint 125).
//!
//! Governed prioritization layer over Workspace context.
//!
//! # Contract
//!
//! **Facts (inputs):** Decision Queue, Continuity, Activity Graph, Task Graph,
//! Environment (← WorkspaceState), Composition, Purpose, Evolution.
//! Attention never reads Observation snapshots, DesktopWindow DTOs, or Win32.
//!
//! **Inference (outputs):** score, priority, urgency, category, ranked top items.
//! Attention items are projections — not a second source of truth.
//!
//! Never executes, never grants authority, never persists payloads.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, AttentionCategory, AttentionConfidence, AttentionItem, AttentionPriority,
    AttentionReason, AttentionSignal, AttentionSourceType, AttentionState, AttentionUrgency,
    ContinuityFacetKind, DecisionPriority, DecisionQueue, DecisionSourceType, DecisionState,
    IntentContext, WorkspaceActivityGraph, WorkspaceAttentionState, WorkspaceContinuityState,
    WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    WorkspaceActivityGraphService, WorkspaceContinuityService, WorkspaceEnvironmentService,
    WorkspaceStateEngine,
};

/// Desktop-like gap kinds owned by Environment (← WorkspaceState).
/// Composition must not re-project these when Environment is present.
const ENVIRONMENT_OWNED_GAP_KINDS: &[&str] = &["disconnected_work", "missing_application"];

pub(crate) struct WorkspaceAttentionService;

impl WorkspaceAttentionService {
    /// Standalone / IPC generate — diagnostics refresh.
    ///
    /// Desktop path is canonical: WorkspaceStateEngine → WorkspaceState → Environment.
    /// Preferred product path is [`Self::generate_with_task_graph`] via Intelligence.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAttentionState> {
        let workspace_id = workspace_id.into();
        let queue = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let graph = WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            Some(&queue),
        )?;
        let continuity = WorkspaceContinuityService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &queue,
            &graph,
        )?;
        let task_graph =
            crate::services::TaskGraphService::generate(db, actor, workspace_id.clone())?;

        // Canonical desktop facts: one WorkspaceState load → Environment.
        let workspace_state = WorkspaceStateEngine::get_current(
            db,
            actor,
            &IntentContext::user_request(),
        )?;
        let workflow =
            crate::services::WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)
                .ok();
        let environment = match &workflow {
            Some(wf) => {
                let applications = {
                    let guard = db
                        .lock()
                        .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
                    let wid = WorkspaceId::new(workspace_id.clone()).map_err(KernelError::Domain)?;
                    workspace_database::ApplicationRepository::new(&guard)
                        .list_by_workspace(&wid)
                        .map_err(KernelError::from)
                };
                match applications {
                    Ok(apps) => {
                        let layout = {
                            let guard = db.lock().ok();
                            guard.and_then(|g| {
                                let wid = WorkspaceId::new(&workspace_id).ok()?;
                                crate::services::LayoutService::load_by_workspace(&g, &wid).ok()
                            })
                        };
                        WorkspaceEnvironmentService::generate_from_state(
                            db,
                            actor,
                            &workspace_id,
                            &workspace_state,
                            &apps,
                            wf,
                            Some(&task_graph),
                            layout.as_ref(),
                        )
                        .ok()
                    }
                    Err(_) => None,
                }
            }
            None => None,
        };

        let composition = match (&environment, &workflow) {
            (Some(env), Some(wf)) => {
                crate::services::WorkspaceCompositionService::generate_with_inputs(
                    db,
                    actor,
                    workspace_id.clone(),
                    env,
                    Some(&task_graph),
                    &continuity,
                    &graph,
                    wf,
                    &queue,
                    None,
                )
                .ok()
            }
            _ => None,
        };
        let purpose = match (&composition, &workflow) {
            (Some(comp), Some(wf)) => {
                let goals =
                    crate::services::WorkspaceIntentService::list_goals(db, &workspace_id, 20)
                        .unwrap_or_default();
                let project = wf.active_project_id.as_ref().and_then(|id| {
                    crate::services::WorkspaceIntentService::get_project(db, id.as_str()).ok()
                });
                crate::services::WorkspacePurposeService::generate_with_inputs(
                    db,
                    actor,
                    workspace_id.clone(),
                    &goals,
                    wf,
                    project.as_ref(),
                    Some(&task_graph),
                    comp,
                    &continuity,
                    &graph,
                    &queue,
                )
                .ok()
            }
            _ => None,
        };
        let evolution = match &purpose {
            Some(purp) => composition.as_ref().and_then(|comp| {
                crate::services::WorkspaceEvolutionService::generate_with_inputs(
                    db,
                    actor,
                    workspace_id.clone(),
                    &graph,
                    Some(&task_graph),
                    purp,
                    comp,
                    &continuity,
                    &queue,
                )
                .ok()
            }),
            None => None,
        };
        Self::generate_with_task_graph(
            db,
            actor,
            &workspace_id,
            &queue,
            &graph,
            &continuity,
            Some(&task_graph),
            environment.as_ref(),
            composition.as_ref(),
            purpose.as_ref(),
            evolution.as_ref(),
        )
    }

    /// Canonical shared-input path — Intelligence / Operating State inject upstream facts.
    ///
    /// Desktop facts must arrive as `environment` (built from WorkspaceState). Attention
    /// never loads Observation or DesktopWindow itself.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_task_graph(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        decision_queue: &DecisionQueue,
        activity_graph: &WorkspaceActivityGraph,
        continuity: &WorkspaceContinuityState,
        task_graph: Option<&workspace_domain::TaskGraph>,
        environment: Option<&workspace_domain::WorkspaceEnvironmentState>,
        composition: Option<&workspace_domain::WorkspaceCompositionState>,
        purpose: Option<&workspace_domain::WorkspacePurposeState>,
        evolution: Option<&workspace_domain::WorkspaceEvolutionState>,
    ) -> Result<WorkspaceAttentionState> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let ws = workspace_id.as_str();
        let now = chrono::Utc::now().to_rfc3339();

        let mut items = Vec::new();
        let mut seen = HashSet::new();

        for item in Self::from_decision_queue(ws, decision_queue, &now)? {
            if seen.insert(item.id.to_string()) {
                items.push(item);
            }
        }
        for item in Self::from_continuity(ws, continuity, &now)? {
            if seen.insert(item.id.to_string()) {
                items.push(item);
            }
        }
        for item in Self::from_activity_informative(ws, activity_graph, &now)? {
            if seen.insert(item.id.to_string()) {
                items.push(item);
            }
        }
        if let Some(graph) = task_graph {
            for item in Self::from_task_graph(ws, graph, &now)? {
                if seen.insert(item.id.to_string()) {
                    items.push(item);
                }
            }
        }
        if let Some(env) = environment {
            for item in Self::from_environment(ws, env, &now)? {
                if seen.insert(item.id.to_string()) {
                    items.push(item);
                }
            }
        }
        if let Some(comp) = composition {
            // Prefer Environment for desktop gaps when both are present.
            for item in Self::from_composition(ws, comp, &now, environment.is_some())? {
                if seen.insert(item.id.to_string()) {
                    items.push(item);
                }
            }
        }
        if let Some(purp) = purpose {
            for item in Self::from_purpose(ws, purp, &now)? {
                if seen.insert(item.id.to_string()) {
                    items.push(item);
                }
            }
        }
        if let Some(evo) = evolution {
            for item in Self::from_evolution(ws, evo, &now)? {
                if seen.insert(item.id.to_string()) {
                    items.push(item);
                }
            }
        }

        let state = WorkspaceAttentionState::from_items(ws, items);
        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    /// Merge Recommendation Engine candidates into an Attention snapshot (CASE 5).
    /// Does not regenerate Attention or Recommendation Engine — avoids circular regen.
    pub(crate) fn enrich_with_recommendations(
        attention: &WorkspaceAttentionState,
        recommendations: &workspace_domain::WorkspaceRecommendationEngineState,
    ) -> Result<WorkspaceAttentionState> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut items = attention.items.clone();
        let mut seen: HashSet<String> = items.iter().map(|i| i.id.to_string()).collect();
        for item in Self::from_recommendation_engine(attention.workspace_id.as_str(), recommendations, &now)?
        {
            if seen.insert(item.id.to_string()) {
                items.push(item);
            }
        }
        Ok(WorkspaceAttentionState::from_items(
            attention.workspace_id.clone(),
            items,
        ))
    }

    /// Reference Milestones as evidence only — does not re-rank or add authority.
    pub(crate) fn enrich_with_milestones(
        attention: &WorkspaceAttentionState,
        milestones: &workspace_domain::WorkspaceMilestoneState,
    ) -> Result<WorkspaceAttentionState> {
        let mut next = attention.clone();
        next.summary = format!(
            "{} Milestone evidence (informational only): {} outcome(s), {} blocked — \
             Attention consumes Milestones as evidence only and does not plan from them.",
            next.summary, milestones.milestone_count, milestones.blocked_count
        );
        Ok(next)
    }

    /// Reference Working Style as evidence only — never profiles or controls behaviour.
    pub(crate) fn enrich_with_working_style(
        attention: &WorkspaceAttentionState,
        working_style: &workspace_domain::WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceAttentionState> {
        let mut next = attention.clone();
        next.summary = format!(
            "{} Working Style evidence (informational only): {} observation(s), {} explicit \
             preference(s) — Attention consumes Working Style as evidence only and never \
             treats observation as intent.",
            next.summary, working_style.observation_count, working_style.preference_count
        );
        Ok(next)
    }

    /// Reference Transitions as evidence only — never restores or executes.
    pub(crate) fn enrich_with_transition(
        attention: &WorkspaceAttentionState,
        transition: &workspace_domain::WorkspaceTransitionState,
    ) -> Result<WorkspaceAttentionState> {
        let mut next = attention.clone();
        next.summary = format!(
            "{} Transition evidence (informational only): {} movement(s), {} returning — \
             Attention consumes Transitions as evidence only and never performs them.",
            next.summary, transition.transition_count, transition.returning_count
        );
        Ok(next)
    }

    /// Reference Navigation as evidence only — does not re-rank or add authority.
    pub(crate) fn enrich_with_navigation(
        attention: &WorkspaceAttentionState,
        navigation: &workspace_domain::WorkspaceNavigationState,
    ) -> Result<WorkspaceAttentionState> {
        let mut next = attention.clone();
        next.summary = format!(
            "{} Navigation evidence (informational only): {} stop(s), {} blocked — \
             Attention consumes Navigation as evidence only and does not route from it.",
            next.summary, navigation.node_count, navigation.blocked_count
        );
        Ok(next)
    }

    /// Reference Work Context as evidence only — does not re-rank or add authority.
    /// Does not regenerate Work Context — avoids circular regen.
    pub(crate) fn enrich_with_work_context(
        attention: &WorkspaceAttentionState,
        work_context: &workspace_domain::WorkspaceWorkContextState,
    ) -> Result<WorkspaceAttentionState> {
        let mut next = attention.clone();
        if let Some(primary) = work_context.primary_context() {
            next.summary = format!(
                "{} Context evidence (informational only): {} [{} / {}] — \
                 Attention consumes Work Context as evidence only and does not re-rank from it.",
                next.summary,
                primary.name,
                primary.context_type.as_str(),
                primary.confidence.as_str()
            );
        } else {
            next.summary = format!(
                "{} Context evidence (informational only): {} context(s) — \
                 Attention consumes Work Context as evidence only.",
                next.summary, work_context.context_count
            );
        }
        Ok(next)
    }

    /// Merge Pattern Model observations into an Attention snapshot (CASE 7).
    /// Does not regenerate Pattern — avoids circular regen.
    pub(crate) fn enrich_with_patterns(
        attention: &WorkspaceAttentionState,
        patterns: &workspace_domain::WorkspacePatternState,
    ) -> Result<WorkspaceAttentionState> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut items = attention.items.clone();
        let mut seen: HashSet<String> = items.iter().map(|i| i.id.to_string()).collect();
        for item in Self::from_pattern_model(attention.workspace_id.as_str(), patterns, &now)? {
            if seen.insert(item.id.to_string()) {
                items.push(item);
            }
        }
        Ok(WorkspaceAttentionState::from_items(
            attention.workspace_id.clone(),
            items,
        ))
    }

    fn from_pattern_model(
        ws: &str,
        patterns: &workspace_domain::WorkspacePatternState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for pattern in &patterns.patterns {
            let (category, score, urgency) = match pattern.kind {
                workspace_domain::PatternKind::DecisionPattern => (
                    AttentionCategory::RequiresDecision,
                    44u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::PatternKind::WorkflowPattern => (
                    AttentionCategory::Informative,
                    36u32,
                    AttentionUrgency::Whenever,
                ),
                _ => (
                    AttentionCategory::Informative,
                    32u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for pattern {}", pattern.kind.as_str()),
                format!("confidence {}", pattern.confidence.as_str()),
                "source Pattern Model".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Pattern,
                AttentionSignal::PatternObservation,
                score as i32,
                "pattern.observation",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Pattern,
                pattern.id.clone(),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                pattern.title.clone(),
                format!(
                    "{}. Impact: {}. Pattern context only — Attention surfaces; never executes.",
                    pattern.observation, pattern.impact
                ),
                now,
                AttentionState::New,
            )?);
        }
        Ok(cap_by_rank(out, 4))
    }

    fn from_recommendation_engine(
        ws: &str,
        recommendations: &workspace_domain::WorkspaceRecommendationEngineState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for candidate in &recommendations.candidates {
            let (category, score, urgency) = match candidate.kind {
                workspace_domain::RecommendationKind::ResolveBlocker => (
                    AttentionCategory::Blocker,
                    58u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::RecommendationKind::ReviewDecision => (
                    AttentionCategory::RequiresDecision,
                    56u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::RecommendationKind::RestoreContext => (
                    AttentionCategory::Interrupted,
                    50u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::RecommendationKind::ContinueWork
                | workspace_domain::RecommendationKind::CompleteTask => (
                    AttentionCategory::Resumable,
                    46u32,
                    AttentionUrgency::Whenever,
                ),
                workspace_domain::RecommendationKind::ReorganizeWorkspace
                | workspace_domain::RecommendationKind::ExploreOpportunity => (
                    AttentionCategory::Informative,
                    38u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for recommendation {}", candidate.kind.as_str()),
                format!("confidence {}", candidate.confidence.as_str()),
                "source Recommendation Engine".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::RecommendationEngine,
                AttentionSignal::RecommendationCandidate,
                score as i32,
                "recommendation.candidate",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::RecommendationEngine,
                candidate.id.clone(),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::High,
                score,
                factors,
                reasons,
                candidate.title.clone(),
                format!(
                    "{}. Impact: {}. Suggestion only — Attention surfaces; never executes.",
                    candidate.reason, candidate.impact
                ),
                now,
                AttentionState::New,
            )?);
        }
        Ok(cap_by_rank(out, 4))
    }

    fn from_evolution(
        ws: &str,
        evolution: &workspace_domain::WorkspaceEvolutionState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for insight in &evolution.insights {
            let (category, score, urgency) = match insight.kind {
                workspace_domain::EvolutionInsightKind::InterruptedWork => (
                    AttentionCategory::Interrupted,
                    52u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::EvolutionInsightKind::DecisionOutcome => (
                    AttentionCategory::RequiresDecision,
                    48u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::EvolutionInsightKind::TaskProgression
                | workspace_domain::EvolutionInsightKind::PurposeProgression => (
                    AttentionCategory::Informative,
                    40u32,
                    AttentionUrgency::Whenever,
                ),
                _ => (
                    AttentionCategory::Informative,
                    34u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for evolution insight {}", insight.kind.as_str()),
                "source Evolution".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Evolution,
                AttentionSignal::EvolutionInsight,
                score as i32,
                "evolution.insight",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Evolution,
                format!("{}:{}", insight.kind.as_str(), insight.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                insight.title.clone(),
                insight.explanation.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(cap_by_rank(out, 4))
    }

    fn from_purpose(
        ws: &str,
        purpose: &workspace_domain::WorkspacePurposeState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        // Purpose context as informative commitment.
        let factors = vec![
            "base 42 for Purpose outcome context".into(),
            "source Purpose Model".into(),
        ];
        let reasons = vec![reason(
            AttentionSourceType::Purpose,
            AttentionSignal::PurposeOutcome,
            42,
            "purpose.outcome",
        )];
        out.push(AttentionItem::project(
            ws,
            AttentionSourceType::Purpose,
            format!("purpose_label:{}", purpose.label),
            AttentionCategory::Commitment,
            AttentionPriority::Normal,
            AttentionUrgency::Whenever,
            AttentionConfidence::Medium,
            42,
            factors,
            reasons,
            format!("Working toward: {}", purpose.label),
            purpose.explanation.clone(),
            now.to_string(),
            AttentionState::Visible,
        )?);
        let mut obstacles = Vec::new();
        for obstacle in &purpose.obstacles {
            let (category, score, urgency) = match obstacle.kind.as_str() {
                "blocked_task" | "blocked_work" | "interrupted_work" => (
                    AttentionCategory::Blocker,
                    58u32,
                    AttentionUrgency::Soon,
                ),
                "outstanding_decisions" => (
                    AttentionCategory::RequiresDecision,
                    50u32,
                    AttentionUrgency::Soon,
                ),
                _ => (
                    AttentionCategory::Informative,
                    36u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for purpose obstacle {}", obstacle.kind),
                "source Purpose".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Purpose,
                AttentionSignal::PurposeObstacle,
                score as i32,
                &format!("purpose.obstacle.{}", obstacle.kind),
            )];
            obstacles.push(AttentionItem::project(
                ws,
                AttentionSourceType::Purpose,
                format!("{}:{}", obstacle.kind, obstacle.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                obstacle.title.clone(),
                obstacle.explanation.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        out.extend(cap_by_rank(obstacles, 4));
        Ok(out)
    }

    fn from_composition(
        ws: &str,
        composition: &workspace_domain::WorkspaceCompositionState,
        now: &str,
        environment_present: bool,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for gap in &composition.gaps {
            // Environment owns desktop-like gaps when present (Sprint 125 dedup).
            if environment_present
                && ENVIRONMENT_OWNED_GAP_KINDS
                    .iter()
                    .any(|kind| *kind == gap.kind.as_str())
            {
                continue;
            }
            let (category, score, urgency) = match gap.kind.as_str() {
                "disconnected_work" => (
                    AttentionCategory::Interrupted,
                    55u32,
                    AttentionUrgency::Soon,
                ),
                "missing_application" => (
                    AttentionCategory::Informative,
                    38u32,
                    AttentionUrgency::Whenever,
                ),
                _ => (
                    AttentionCategory::Informative,
                    32u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for composition gap {}", gap.kind),
                "source Composition".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Composition,
                AttentionSignal::CompositionGap,
                score as i32,
                &format!("composition.gap.{}", gap.kind),
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Composition,
                format!("{}:{}", gap.kind, gap.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                gap.title.clone(),
                gap.explanation.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(cap_by_rank(out, 5))
    }

    fn from_environment(
        ws: &str,
        environment: &workspace_domain::WorkspaceEnvironmentState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for gap in &environment.gaps {
            let (category, score, urgency) = match gap.kind.as_str() {
                "disconnected_work" => (
                    AttentionCategory::Interrupted,
                    60u32,
                    AttentionUrgency::Soon,
                ),
                "missing_application" => (
                    AttentionCategory::Informative,
                    40u32,
                    AttentionUrgency::Whenever,
                ),
                _ => (
                    AttentionCategory::Informative,
                    35u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let factors = vec![
                format!("base {score} for environment gap {}", gap.kind),
                "source Environment".into(),
            ];
            let signal = match gap.kind.as_str() {
                "missing_application" => AttentionSignal::MissingApplication,
                _ => AttentionSignal::EnvironmentDisconnect,
            };
            let reasons = vec![reason(
                AttentionSourceType::Environment,
                signal,
                score as i32,
                &format!("environment.gap.{}", gap.kind),
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Environment,
                format!("{}:{}", gap.kind, gap.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                gap.title.clone(),
                gap.explanation.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(cap_by_rank(out, 5))
    }

    fn from_task_graph(
        ws: &str,
        graph: &workspace_domain::TaskGraph,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for node in &graph.nodes {
            if !node.task.status.is_open() {
                continue;
            }
            let (category, base, urgency) = match node.task.status {
                workspace_domain::WorkspaceTaskStatus::Blocked => (
                    AttentionCategory::Blocker,
                    88u32,
                    AttentionUrgency::Immediate,
                ),
                workspace_domain::WorkspaceTaskStatus::Waiting => (
                    AttentionCategory::RequiresDecision,
                    55u32,
                    AttentionUrgency::Soon,
                ),
                workspace_domain::WorkspaceTaskStatus::InProgress => (
                    AttentionCategory::Informative,
                    45u32,
                    AttentionUrgency::Soon,
                ),
                _ => (
                    AttentionCategory::Informative,
                    30u32,
                    AttentionUrgency::Whenever,
                ),
            };
            let mut score = base;
            let mut factors = vec![
                format!("base {base} for Task Graph {}", node.task.status.as_str()),
                "source TaskGraph".into(),
            ];
            let priority_boost = u32::from(node.task.priority.rank()) * 4;
            score += priority_boost;
            factors.push(format!(
                "+{} priority {}",
                priority_boost,
                node.task.priority.as_str()
            ));
            let signal = match node.task.status {
                workspace_domain::WorkspaceTaskStatus::Blocked => AttentionSignal::BlockedTask,
                workspace_domain::WorkspaceTaskStatus::Waiting => AttentionSignal::WaitingTask,
                _ => AttentionSignal::InProgressTask,
            };
            let mut reasons = vec![reason(
                AttentionSourceType::TaskGraph,
                signal,
                base as i32,
                &format!("task.base.{}", node.task.status.as_str()),
            )];
            if priority_boost > 0 {
                reasons.push(reason(
                    AttentionSourceType::TaskGraph,
                    AttentionSignal::HighPriorityIntent,
                    priority_boost as i32,
                    &format!("task.priority.{}", node.task.priority.as_str()),
                ));
            }
            let explanation = node
                .waiting_reason
                .clone()
                .unwrap_or_else(|| node.task.explanation.clone());
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::TaskGraph,
                node.task.id.to_string(),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::High,
                score,
                factors,
                reasons,
                node.task.title.clone(),
                explanation,
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(out)
    }

    fn from_decision_queue(
        ws: &str,
        queue: &DecisionQueue,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for item in &queue.items {
            if !matches!(
                item.decision_state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            ) {
                continue;
            }

            let is_blocker = item.source_type == DecisionSourceType::BlockedAction;
            let (category, base, urgency) = if is_blocker {
                (AttentionCategory::Blocker, 90u32, AttentionUrgency::Immediate)
            } else {
                (
                    AttentionCategory::RequiresDecision,
                    70u32,
                    AttentionUrgency::Soon,
                )
            };

            let mut score = base;
            let mut factors = vec![format!("base {base} for {}", category.as_str())];
            let base_signal = if is_blocker {
                AttentionSignal::BlockedAction
            } else {
                AttentionSignal::OutstandingDecision
            };
            let mut reasons = vec![reason(
                AttentionSourceType::DecisionQueue,
                base_signal,
                base as i32,
                if is_blocker {
                    "decision.base.blocker"
                } else {
                    "decision.base.outstanding"
                },
            )];
            match item.priority {
                DecisionPriority::Critical => {
                    score += 20;
                    factors.push("+20 DecisionPriority::Critical".into());
                    reasons.push(reason(
                        AttentionSourceType::DecisionQueue,
                        AttentionSignal::HighPriorityIntent,
                        20,
                        "decision.priority.critical",
                    ));
                }
                DecisionPriority::High => {
                    score += 12;
                    factors.push("+12 DecisionPriority::High".into());
                    reasons.push(reason(
                        AttentionSourceType::DecisionQueue,
                        AttentionSignal::HighPriorityIntent,
                        12,
                        "decision.priority.high",
                    ));
                }
                DecisionPriority::Normal => {
                    score += 4;
                    factors.push("+4 DecisionPriority::Normal".into());
                    reasons.push(reason(
                        AttentionSourceType::DecisionQueue,
                        AttentionSignal::HighPriorityIntent,
                        4,
                        "decision.priority.normal",
                    ));
                }
                DecisionPriority::Low => {
                    factors.push("+0 DecisionPriority::Low".into());
                }
            }
            if item.decision_state == DecisionState::Deferred {
                score = score.saturating_sub(8);
                factors.push("-8 deferred in Decision Queue".into());
                reasons.push(reason(
                    AttentionSourceType::DecisionQueue,
                    base_signal,
                    -8,
                    "decision.deferred",
                ));
            }

            let priority = score_to_priority(score);
            let state = if item.decision_state == DecisionState::Pending {
                AttentionState::New
            } else {
                AttentionState::Visible
            };

            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::DecisionQueue,
                item.id.to_string(),
                category,
                priority,
                urgency,
                AttentionConfidence::High,
                score,
                factors,
                reasons,
                item.title.clone(),
                format!(
                    "{}. Decision Queue remains authoritative.",
                    item.explanation
                ),
                item.created_at.clone(),
                state,
            )?);
        }
        let _ = now;
        Ok(out)
    }

    fn from_continuity(
        ws: &str,
        continuity: &WorkspaceContinuityState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();

        // Skip continuity outstanding/blockers already covered by Decision Queue ids.
        for facet in &continuity.interrupted_work {
            let score = 60u32;
            let factors = vec![
                "base 60 for interrupted work".into(),
                "source Continuity::InterruptedWork".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Continuity,
                AttentionSignal::InterruptedWork,
                score as i32,
                "continuity.interrupted",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::Interrupted,
                AttentionPriority::High,
                AttentionUrgency::Soon,
                AttentionConfidence::Medium,
                score,
                factors,
                reasons,
                facet.title.clone(),
                facet.why.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        for facet in &continuity.resumable_work {
            let factors = vec![
                "base 40 for resumable work".into(),
                "source Continuity::ResumableWork".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Continuity,
                AttentionSignal::ResumableWork,
                40,
                "continuity.resumable",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::Resumable,
                AttentionPriority::Normal,
                AttentionUrgency::Soon,
                AttentionConfidence::Medium,
                40,
                factors,
                reasons,
                facet.title.clone(),
                facet.why.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        for facet in &continuity.active_commitments {
            if facet.kind != ContinuityFacetKind::ActiveCommitment {
                continue;
            }
            let pending_like = facet.summary.contains("pending") || facet.summary.contains("draft");
            let (score, priority, urgency, category) = if pending_like {
                (
                    65u32,
                    AttentionPriority::High,
                    AttentionUrgency::Soon,
                    AttentionCategory::Commitment,
                )
            } else {
                (
                    25u32,
                    AttentionPriority::Low,
                    AttentionUrgency::Whenever,
                    AttentionCategory::Commitment,
                )
            };
            let factors = vec![
                format!("base {score} for active commitment"),
                "source Continuity::ActiveCommitment (status only)".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::AutomationContract,
                AttentionSignal::CommitmentPending,
                score as i32,
                "continuity.commitment",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::AutomationContract,
                facet.source_id.clone(),
                category,
                priority,
                urgency,
                AttentionConfidence::High,
                score,
                factors,
                reasons,
                facet.title.clone(),
                facet.why.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        for facet in &continuity.dormant_projects {
            let factors = vec![
                "base 15 for dormant project (can wait)".into(),
                "source Continuity::DormantProject".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::Continuity,
                AttentionSignal::DormantWork,
                15,
                "continuity.dormant",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::CanWait,
                AttentionPriority::Low,
                AttentionUrgency::Whenever,
                AttentionConfidence::Low,
                15,
                factors,
                reasons,
                facet.title.clone(),
                facet.why.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        if let Some(focus) = &continuity.current_focus {
            let factors = vec![
                "base 35 for current focus".into(),
                "source WorkflowContext via Continuity".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::WorkflowContext,
                AttentionSignal::CurrentFocus,
                35,
                "continuity.focus",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::WorkflowContext,
                focus.source_id.clone(),
                AttentionCategory::Informative,
                AttentionPriority::Normal,
                AttentionUrgency::Whenever,
                AttentionConfidence::High,
                35,
                factors,
                reasons,
                focus.title.clone(),
                focus.why.clone(),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        Ok(out)
    }

    fn from_activity_informative(
        ws: &str,
        graph: &WorkspaceActivityGraph,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        // The only source whose cut is ordering-based by design: all activity items score
        // equally, so recency is the selection criterion. Timeline order is Activity Graph's
        // contract, not an incidental input order.
        // Walk newest-first, skipping audit noise so recent real work stays visible.
        for activity in graph.timeline.iter().rev() {
            if matches!(
                activity.activity_type,
                workspace_domain::ActivityType::AuditSignal
            ) {
                continue;
            }
            let factors = vec![
                "base 10 for recent progress (informative)".into(),
                "source Activity Graph timeline".into(),
            ];
            let reasons = vec![reason(
                AttentionSourceType::ActivityGraph,
                AttentionSignal::ActivityProgress,
                10,
                "activity.progress",
            )];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::ActivityGraph,
                activity.id.to_string(),
                AttentionCategory::Informative,
                AttentionPriority::Low,
                AttentionUrgency::Whenever,
                AttentionConfidence::Medium,
                10,
                factors,
                reasons,
                activity.summary.clone(),
                "Showing because Activity Graph lists recent related work.".to_string(),
                activity.timestamp.clone(),
                AttentionState::Visible,
            )?);
            if out.len() >= 3 {
                break;
            }
        }
        let _ = now;
        Ok(out)
    }

    /// Architecture guard — Attention must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceAttentionError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceAttentionState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.attention.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "item_count": state.items.len(),
                "requires_decision": state.requires_decision_count,
                "blockers": state.blocker_count,
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.attention.summary.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "summary_len": state.summary.len(),
                "top_count": state.top_items.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn score_to_priority(score: u32) -> AttentionPriority {
    if score >= 95 {
        AttentionPriority::Critical
    } else if score >= 70 {
        AttentionPriority::High
    } else if score >= 35 {
        AttentionPriority::Normal
    } else {
        AttentionPriority::Low
    }
}

fn reason(
    source: AttentionSourceType,
    signal: AttentionSignal,
    weight: i32,
    key: &str,
) -> AttentionReason {
    AttentionReason::new(source, signal, weight, key)
}

/// Cap a source projection by attention rank, never by upstream collection order.
///
/// Truncating the raw input would let an upstream reordering silently change which
/// facts reach Attention. Score DESC then id ASC keeps the cut deterministic.
fn cap_by_rank(mut items: Vec<AttentionItem>, limit: usize) -> Vec<AttentionItem> {
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.id.as_str().cmp(b.id.as_str()))
    });
    items.truncate(limit);
    items
}
