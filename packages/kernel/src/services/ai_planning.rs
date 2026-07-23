//! AI planning service — goals → proposals only (Sprints 48–51).
//!
//! Never calls ProcessLauncher, approval mutators, or grant APIs.
//! Submissions route through [`crate::commands::CommandHandler`] → pipeline → gateway.
//! Workspace awareness is read-only and does not grant authority.
//! Model providers supply intelligence candidates only (Sprints 62–63).

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    Actor, ActorContext, AiActionProposal, AiGoal, AiMemoryAwareness, AiPersonalizationAwareness,
    AiPlan, AiPlanningContext, AiPlanningError, AiProposalAuthorityOutcome, AiWorkspaceAwareness,
    ApplicationId, IntentContext, WorkspaceContext,
};

use crate::error::{KernelError, Result};
use crate::services::{AuditService, ModelProviderService};

/// Planner that routes intelligence through the model provider boundary.
/// Does not execute actions or grant authority.
pub(crate) struct AiPlanningService;

impl AiPlanningService {
    /// Builds proposals for a goal from available context. Does not submit or execute.
    pub(crate) fn plan(context: &AiPlanningContext) -> Result<AiPlan> {
        Self::plan_internal(context, None)
    }

    pub(crate) fn plan_with_audit(
        context: &AiPlanningContext,
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
    ) -> Result<AiPlan> {
        Self::plan_internal(context, Some((db, actor)))
    }

    fn plan_internal(
        context: &AiPlanningContext,
        audit: Option<(&Arc<Mutex<Database>>, &ActorContext)>,
    ) -> Result<AiPlan> {
        if context.goal.statement.trim().is_empty() {
            return Err(KernelError::from(AiPlanningError::EmptyGoalStatement));
        }
        // Provider → structured candidates → governed AiActionProposal (no execution).
        ModelProviderService::plan_from_context(context, audit)
    }

    pub(crate) fn plan_prepare_workspace(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<ApplicationId>,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlan> {
        let context = Self::build_prepare_context(
            actor_id,
            goal_statement,
            application_ids,
            memory_awareness,
            personalization_awareness,
        )?;
        Self::plan(&context)
    }

    pub(crate) fn plan_prepare_workspace_audited(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<ApplicationId>,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlan> {
        let context = Self::build_prepare_context(
            actor_id,
            goal_statement,
            application_ids,
            memory_awareness,
            personalization_awareness,
        )?;
        if let Some(personalization) = &context.personalization_awareness {
            crate::services::AiPersonalizationService::audit_used_in_planning(
                db,
                actor,
                personalization,
            )?;
        }
        Self::plan_with_audit(&context, db, actor)
    }

    pub(crate) fn plan_with_awareness(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        awareness: AiWorkspaceAwareness,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlan> {
        let context = Self::build_awareness_context(
            actor_id,
            goal_statement,
            awareness,
            memory_awareness,
            personalization_awareness,
        )?;
        Self::plan(&context)
    }

    pub(crate) fn plan_with_awareness_audited(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        awareness: AiWorkspaceAwareness,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlan> {
        let context = Self::build_awareness_context(
            actor_id,
            goal_statement,
            awareness,
            memory_awareness,
            personalization_awareness,
        )?;
        if let Some(personalization) = &context.personalization_awareness {
            crate::services::AiPersonalizationService::audit_used_in_planning(
                db,
                actor,
                personalization,
            )?;
        }
        Self::plan_with_audit(&context, db, actor)
    }

    fn build_prepare_context(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<ApplicationId>,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlanningContext> {
        let goal = AiGoal::new(goal_statement, actor_id).map_err(KernelError::from)?;
        let mut context = AiPlanningContext::new(goal, application_ids);
        if let Ok(action_awareness) = crate::services::ActionCatalogService::ai_awareness() {
            context = context.with_action_awareness(action_awareness);
        }
        if let Some(memory_awareness) = memory_awareness {
            context = context.with_memory_awareness(memory_awareness);
        }
        if let Some(personalization_awareness) = personalization_awareness {
            context = context.with_personalization_awareness(personalization_awareness);
        }
        Ok(context)
    }

    fn build_awareness_context(
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        awareness: AiWorkspaceAwareness,
        memory_awareness: Option<AiMemoryAwareness>,
        personalization_awareness: Option<AiPersonalizationAwareness>,
    ) -> Result<AiPlanningContext> {
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
        if let Some(personalization_awareness) = personalization_awareness {
            context = context.with_personalization_awareness(personalization_awareness);
        }
        Ok(context)
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
