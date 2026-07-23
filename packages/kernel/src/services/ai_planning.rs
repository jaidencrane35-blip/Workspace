//! AI planning service — goals → proposals only (Sprints 48–49).
//!
//! Never calls ProcessLauncher, approval mutators, or grant APIs.
//! Submissions route through [`crate::commands::CommandHandler`] → pipeline → gateway.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    Actor, ActorContext, AiActionProposal, AiGoal, AiPlan, AiPlanningContext, AiPlanningError,
    AiProposalAuthorityOutcome, ApplicationId, IntentContext,
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

        let mut proposals = Vec::new();
        // Deterministic "prepare workspace" heuristic: propose launch for each available app.
        for application_id in context.available_application_ids.iter().take(5) {
            let explanation = format!(
                "Goal '{}': launch application to prepare workspace",
                context.goal.statement
            );
            let proposal = AiActionProposal::propose_application_launch(
                &context.goal,
                application_id,
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

    pub(crate) fn plan_prepare_workspace(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<ApplicationId>,
    ) -> Result<AiPlan> {
        let goal = AiGoal::new(goal_statement, actor_id).map_err(KernelError::from)?;
        let context = AiPlanningContext::new(goal, application_ids);
        Self::plan(&context)
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
