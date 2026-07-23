//! AI planning service — goals → proposals only (Sprints 48–51).
//!
//! Never calls ProcessLauncher, approval mutators, or grant APIs.
//! Submissions route through [`crate::commands::CommandHandler`] → pipeline → gateway.
//! Workspace awareness is read-only and does not grant authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    Actor, ActorContext, AiActionProposal, AiGoal, AiMemoryAwareness, AiPlan, AiPlanningContext,
    AiPlanningError, AiProposalAuthorityOutcome, AiWorkspaceAwareness, ApplicationId,
    IntentContext, WorkspaceContext,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

/// Deterministic diagnostic planner — no LLM, no memory, no execution.
pub(crate) struct AiPlanningService;

impl AiPlanningService {
    /// Builds proposals for a goal from available context. Does not submit or execute.
    pub(crate) fn plan(context: &AiPlanningContext) -> Result<AiPlan> {
        if context.goal.statement.trim().is_empty() {
            return Err(KernelError::from(AiPlanningError::EmptyGoalStatement));
        }

        let capability_note = context
            .action_awareness
            .as_ref()
            .and_then(|awareness| awareness.explain_capability_for_command("LaunchApplication"));
        let memory_notes = context
            .memory_awareness
            .as_ref()
            .map(|awareness| awareness.planning_notes())
            .unwrap_or_default();

        let candidates = Self::candidate_launches(context);
        let mut proposals = Vec::new();

        for (application_id, mut explanation) in candidates.into_iter().take(5) {
            if let Some(note) = &capability_note {
                explanation = format!("{explanation} {note}");
            }
            if !memory_notes.is_empty() {
                explanation = format!("{explanation} Memory: {}", memory_notes.join("; "));
            }
            let proposal = AiActionProposal::propose_application_launch(
                &context.goal,
                &application_id,
                Some(explanation),
            )
            .map_err(KernelError::from)?;
            proposals.push(proposal);
        }

        Ok(AiPlan {
            goal: context.goal.clone(),
            proposals,
        })
    }

    /// Context-aware launch candidates: skip apps that already appear active.
    /// Preferred applications from memory are ranked first (informational only).
    fn candidate_launches(context: &AiPlanningContext) -> Vec<(ApplicationId, String)> {
        let preferred = context
            .memory_awareness
            .as_ref()
            .map(|awareness| awareness.preferred_application_ids())
            .unwrap_or_default();

        let mut candidates: Vec<(ApplicationId, String)> = if let Some(awareness) = &context.awareness
        {
            awareness
                .applications
                .iter()
                .filter(|app| !app.appears_active)
                .map(|app| {
                    let preferred_note = if preferred.iter().any(|id| id == app.id.as_str()) {
                        " (preferred from memory)"
                    } else {
                        ""
                    };
                    let explanation = format!(
                        "Goal '{}': launch '{}' (registered in workspace '{}', not appearing active){preferred_note}",
                        context.goal.statement, app.name, awareness.workspace_name
                    );
                    (app.id.clone(), explanation)
                })
                .collect()
        } else {
            context
                .available_application_ids
                .iter()
                .map(|application_id| {
                    let preferred_note = if preferred.iter().any(|id| id == application_id.as_str())
                    {
                        " (preferred from memory)"
                    } else {
                        ""
                    };
                    let explanation = format!(
                        "Goal '{}': launch application to prepare workspace{preferred_note}",
                        context.goal.statement
                    );
                    (application_id.clone(), explanation)
                })
                .collect()
        };

        candidates.sort_by_key(|(id, _)| {
            if preferred.iter().any(|preferred_id| preferred_id == id.as_str()) {
                0_u8
            } else {
                1_u8
            }
        });
        candidates
    }

    pub(crate) fn plan_prepare_workspace(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<ApplicationId>,
        memory_awareness: Option<AiMemoryAwareness>,
    ) -> Result<AiPlan> {
        let goal = AiGoal::new(goal_statement, actor_id).map_err(KernelError::from)?;
        let mut context = AiPlanningContext::new(goal, application_ids);
        if let Ok(action_awareness) = crate::services::ActionCatalogService::ai_awareness() {
            context = context.with_action_awareness(action_awareness);
        }
        if let Some(memory_awareness) = memory_awareness {
            context = context.with_memory_awareness(memory_awareness);
        }
        Self::plan(&context)
    }

    pub(crate) fn plan_with_awareness(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        awareness: AiWorkspaceAwareness,
        memory_awareness: Option<AiMemoryAwareness>,
    ) -> Result<AiPlan> {
        let goal = AiGoal::new(goal_statement, actor_id).map_err(KernelError::from)?;
        let fallback_ids: Vec<ApplicationId> = awareness
            .applications
            .iter()
            .map(|app| app.id.clone())
            .collect();
        let mut context = AiPlanningContext::new(goal, fallback_ids).with_awareness(awareness);
        if let Ok(action_awareness) = crate::services::ActionCatalogService::ai_awareness() {
            context = context.with_action_awareness(action_awareness);
        }
        if let Some(memory_awareness) = memory_awareness {
            context = context.with_memory_awareness(memory_awareness);
        }
        Self::plan(&context)
    }

    /// Builds read-only awareness from workspace context + environment titles.
    pub(crate) fn awareness_from_context(
        workspace_context: &WorkspaceContext,
        environment_window_titles: Vec<String>,
    ) -> AiWorkspaceAwareness {
        AiWorkspaceAwareness::from_workspace_context(workspace_context, environment_window_titles)
    }

    /// Records operational planning audits (not chain-of-thought).
    pub(crate) fn audit_plan_created(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiPlan,
    ) -> Result<()> {
        let metadata = json!({
            "goal_id": plan.goal.id.as_str(),
            "goal_statement": plan.goal.statement,
            "proposal_count": plan.proposals.len(),
            "proposal_ids": plan.proposals.iter().map(|p| p.id.to_string()).collect::<Vec<_>>(),
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.planning.plan_created",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_proposal_created(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        proposal: &AiActionProposal,
    ) -> Result<()> {
        let metadata = json!({
            "goal_id": proposal.goal_id.as_str(),
            "proposal_id": proposal.id.as_str(),
            "command": proposal.command_name,
            "target": proposal.target_resource.as_ref().map(|r| r.canonical()),
            "explanation": proposal.explanation,
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.planning.proposal_created",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_awareness_used(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        awareness: &AiWorkspaceAwareness,
    ) -> Result<()> {
        let metadata = json!({
            "workspace_id": awareness.workspace_id,
            "workspace_name": awareness.workspace_name,
            "zone_count": awareness.zone_count,
            "application_count": awareness.applications.len(),
            "active_application_count": awareness.applications.iter().filter(|a| a.appears_active).count(),
            "observation_count": awareness.recent_observation_count,
            "environment_window_count": awareness.environment_window_titles.len(),
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.planning.awareness_used",
            true,
            metadata,
        )
    }

    pub(crate) fn map_submission_error(error: KernelError) -> (AiProposalAuthorityOutcome, bool) {
        match error {
            KernelError::ApprovalRequired {
                reason,
                approval_request_id,
            } => (
                AiProposalAuthorityOutcome::ApprovalRequired {
                    reason,
                    approval_request_id,
                },
                true,
            ),
            KernelError::PermissionDenied(reason) => {
                (AiProposalAuthorityOutcome::Denied { reason }, true)
            }
            other => (
                AiProposalAuthorityOutcome::Denied {
                    reason: other.to_string(),
                },
                false,
            ),
        }
    }

    pub(crate) fn ensure_ai_actor(actor_id: &str) -> Result<ActorContext> {
        Ok(ActorContext::new(
            Actor::ai_assistant(actor_id).map_err(KernelError::Domain)?,
        ))
    }
}
