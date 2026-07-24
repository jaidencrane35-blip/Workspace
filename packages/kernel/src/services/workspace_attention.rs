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
        Self::generate_with_inputs(db, actor, &workspace_id, &queue, &graph, &continuity)
    }

    /// Preferred path — Intelligence injects DQ + AG + Continuity.
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        decision_queue: &DecisionQueue,
        activity_graph: &WorkspaceActivityGraph,
        continuity: &WorkspaceContinuityState,
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

        let state = WorkspaceAttentionState::from_items(ws, items);
        Self::audit_generated(db, actor, &state)?;
        Ok(state)
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
        for activity in graph.timeline.iter().rev().take(8) {
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
