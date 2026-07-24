//! Workspace Attention Engine (Phase 5 Batch 2).
//!
//! Deterministic prioritization over Continuity, Decision Queue, and Activity Graph.
//! Never executes, never grants authority, never persists payloads.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, AttentionCategory, AttentionConfidence, AttentionItem, AttentionPriority,
    AttentionSourceType, AttentionState, AttentionUrgency, ContinuityFacetKind, DecisionPriority,
    DecisionQueue, DecisionSourceType, DecisionState, IntentContext, WorkspaceActivityGraph,
    WorkspaceAttentionState, WorkspaceContinuityState, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    WorkspaceActivityGraphService, WorkspaceContinuityService,
};

pub(crate) struct WorkspaceAttentionService;

impl WorkspaceAttentionService {
    /// Standalone generate — builds Continuity then Attention without Intelligence.
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
        let task_graph = crate::services::TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let environment =
            crate::services::WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone()).ok();
        let workflow =
            crate::services::WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)
                .ok();
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

    /// Preferred path — Intelligence injects DQ + AG + Continuity (+ optional Task Graph).
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        decision_queue: &DecisionQueue,
        activity_graph: &WorkspaceActivityGraph,
        continuity: &WorkspaceContinuityState,
    ) -> Result<WorkspaceAttentionState> {
        Self::generate_with_task_graph(
            db,
            actor,
            workspace_id,
            decision_queue,
            activity_graph,
            continuity,
            None,
            None,
            None,
            None,
            None,
        )
    }

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
            for item in Self::from_composition(ws, comp, &now)? {
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
        for pattern in patterns.patterns.iter().take(4) {
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
                pattern.title.clone(),
                format!(
                    "{}. Impact: {}. Pattern context only — Attention surfaces; never executes.",
                    pattern.observation, pattern.impact
                ),
                now,
                AttentionState::New,
            )?);
        }
        Ok(out)
    }

    fn from_recommendation_engine(
        ws: &str,
        recommendations: &workspace_domain::WorkspaceRecommendationEngineState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for candidate in recommendations.candidates.iter().take(4) {
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
                candidate.title.clone(),
                format!(
                    "{}. Impact: {}. Suggestion only — Attention surfaces; never executes.",
                    candidate.reason, candidate.impact
                ),
                now,
                AttentionState::New,
            )?);
        }
        Ok(out)
    }

    fn from_evolution(
        ws: &str,
        evolution: &workspace_domain::WorkspaceEvolutionState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for insight in evolution.insights.iter().take(4) {
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Evolution,
                format!("{}:{}", insight.kind.as_str(), insight.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors.clone(),
                insight.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    insight.explanation,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(out)
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
        out.push(AttentionItem::project(
            ws,
            AttentionSourceType::Purpose,
            format!("purpose_label:{}", purpose.label),
            AttentionCategory::Commitment,
            AttentionPriority::Normal,
            AttentionUrgency::Whenever,
            AttentionConfidence::Medium,
            42,
            factors.clone(),
            format!("Working toward: {}", purpose.label),
            format!(
                "{}. Score factors: {}.",
                purpose.explanation,
                score_factors_join(&factors)
            ),
            now.to_string(),
            AttentionState::Visible,
        )?);
        for obstacle in purpose.obstacles.iter().take(4) {
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Purpose,
                format!("{}:{}", obstacle.kind, obstacle.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors.clone(),
                obstacle.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    obstacle.explanation,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(out)
    }

    fn from_composition(
        ws: &str,
        composition: &workspace_domain::WorkspaceCompositionState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for gap in composition.gaps.iter().take(5) {
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Composition,
                format!("{}:{}", gap.kind, gap.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors.clone(),
                gap.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    gap.explanation,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(out)
    }

    fn from_environment(
        ws: &str,
        environment: &workspace_domain::WorkspaceEnvironmentState,
        now: &str,
    ) -> Result<Vec<AttentionItem>> {
        let mut out = Vec::new();
        for gap in environment.gaps.iter().take(5) {
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Environment,
                format!("{}:{}", gap.kind, gap.title),
                category,
                score_to_priority(score),
                urgency,
                AttentionConfidence::Medium,
                score,
                factors.clone(),
                gap.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    gap.explanation,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }
        Ok(out)
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
            score += u32::from(node.task.priority.rank()) * 4;
            factors.push(format!(
                "+{} priority {}",
                u32::from(node.task.priority.rank()) * 4,
                node.task.priority.as_str()
            ));
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
                factors.clone(),
                node.task.title.clone(),
                format!(
                    "{explanation}. Score factors: {}.",
                    score_factors_join(&factors)
                ),
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
            match item.priority {
                DecisionPriority::Critical => {
                    score += 20;
                    factors.push("+20 DecisionPriority::Critical".into());
                }
                DecisionPriority::High => {
                    score += 12;
                    factors.push("+12 DecisionPriority::High".into());
                }
                DecisionPriority::Normal => {
                    score += 4;
                    factors.push("+4 DecisionPriority::Normal".into());
                }
                DecisionPriority::Low => {
                    factors.push("+0 DecisionPriority::Low".into());
                }
            }
            if item.decision_state == DecisionState::Deferred {
                score = score.saturating_sub(8);
                factors.push("-8 deferred in Decision Queue".into());
            }

            let priority = score_to_priority(score);
            let state = if item.decision_state == DecisionState::Pending {
                AttentionState::New
            } else {
                AttentionState::Visible
            };
            let factors_text = score_factors_join(&factors);

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
                item.title.clone(),
                format!(
                    "{}. Score factors: {}. Decision Queue remains authoritative.",
                    item.explanation, factors_text
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::Interrupted,
                AttentionPriority::High,
                AttentionUrgency::Soon,
                AttentionConfidence::Medium,
                score,
                factors.clone(),
                facet.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    facet.why,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
            let _ = score;
        }

        for facet in &continuity.resumable_work {
            let factors = vec![
                "base 40 for resumable work".into(),
                "source Continuity::ResumableWork".into(),
            ];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::Resumable,
                AttentionPriority::Normal,
                AttentionUrgency::Soon,
                AttentionConfidence::Medium,
                40,
                factors.clone(),
                facet.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    facet.why,
                    score_factors_join(&factors)
                ),
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::AutomationContract,
                facet.source_id.clone(),
                category,
                priority,
                urgency,
                AttentionConfidence::High,
                score,
                factors.clone(),
                facet.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    facet.why,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        for facet in &continuity.dormant_projects {
            let factors = vec![
                "base 15 for dormant project (can wait)".into(),
                "source Continuity::DormantProject".into(),
            ];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::Continuity,
                facet.id.to_string(),
                AttentionCategory::CanWait,
                AttentionPriority::Low,
                AttentionUrgency::Whenever,
                AttentionConfidence::Low,
                15,
                factors.clone(),
                facet.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    facet.why,
                    score_factors_join(&factors)
                ),
                now.to_string(),
                AttentionState::Visible,
            )?);
        }

        if let Some(focus) = &continuity.current_focus {
            let factors = vec![
                "base 35 for current focus".into(),
                "source WorkflowContext via Continuity".into(),
            ];
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::WorkflowContext,
                focus.source_id.clone(),
                AttentionCategory::Informative,
                AttentionPriority::Normal,
                AttentionUrgency::Whenever,
                AttentionConfidence::High,
                35,
                factors.clone(),
                focus.title.clone(),
                format!(
                    "{}. Score factors: {}.",
                    focus.why,
                    score_factors_join(&factors)
                ),
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
            out.push(AttentionItem::project(
                ws,
                AttentionSourceType::ActivityGraph,
                activity.id.to_string(),
                AttentionCategory::Informative,
                AttentionPriority::Low,
                AttentionUrgency::Whenever,
                AttentionConfidence::Medium,
                10,
                factors.clone(),
                activity.summary.clone(),
                format!(
                    "Showing because Activity Graph lists recent related work. Score factors: {}.",
                    score_factors_join(&factors)
                ),
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

fn score_factors_join(factors: &[String]) -> String {
    factors.join("; ")
}
