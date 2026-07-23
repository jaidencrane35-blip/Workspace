//! Governed AI plan orchestration — sequences steps through the existing
//! Permission Gateway path (Sprints 56–57). Never launches or grants directly.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, AiOrchestratedPlan, AiOrchestratedPlanState, AiOrchestrationError, AiPlan,
    AiProposalAuthorityOutcome, IntentContext,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

/// In-memory diagnostic store for orchestrated plans (not durable authority).
#[derive(Default)]
pub(crate) struct OrchestratedPlanStore {
    plans: HashMap<String, AiOrchestratedPlan>,
}

impl OrchestratedPlanStore {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, plan: AiOrchestratedPlan) -> AiOrchestratedPlan {
        self.plans.insert(plan.id.to_string(), plan.clone());
        plan
    }

    pub(crate) fn get(&self, id: &str) -> Option<AiOrchestratedPlan> {
        self.plans.get(id).cloned()
    }

    pub(crate) fn upsert(&mut self, plan: AiOrchestratedPlan) -> AiOrchestratedPlan {
        self.plans.insert(plan.id.to_string(), plan.clone());
        plan
    }
}

pub(crate) struct AiOrchestrationService;

impl AiOrchestrationService {
    pub(crate) fn create_from_plan(plan: AiPlan) -> Result<AiOrchestratedPlan> {
        AiOrchestratedPlan::from_ai_plan(plan).map_err(KernelError::from)
    }

    pub(crate) fn store_plan(
        store: &Arc<Mutex<OrchestratedPlanStore>>,
        plan: AiOrchestratedPlan,
    ) -> Result<AiOrchestratedPlan> {
        let mut guard = store
            .lock()
            .map_err(|_| KernelError::Config("orchestrated plan store lock poisoned".into()))?;
        Ok(guard.insert(plan))
    }

    pub(crate) fn get_plan(
        store: &Arc<Mutex<OrchestratedPlanStore>>,
        plan_id: &str,
    ) -> Result<AiOrchestratedPlan> {
        let guard = store
            .lock()
            .map_err(|_| KernelError::Config("orchestrated plan store lock poisoned".into()))?;
        guard
            .get(plan_id)
            .ok_or_else(|| KernelError::AiOrchestrationValidation {
                message: format!("orchestrated plan not found: {plan_id}"),
            })
    }

    pub(crate) fn save_plan(
        store: &Arc<Mutex<OrchestratedPlanStore>>,
        plan: AiOrchestratedPlan,
    ) -> Result<AiOrchestratedPlan> {
        let mut guard = store
            .lock()
            .map_err(|_| KernelError::Config("orchestrated plan store lock poisoned".into()))?;
        Ok(guard.upsert(plan))
    }

    pub(crate) fn audit_plan_created(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiOrchestratedPlan,
    ) -> Result<()> {
        let metadata = json!({
            "plan_id": plan.id.as_str(),
            "goal_id": plan.goal.id.as_str(),
            "goal_statement": plan.goal.statement,
            "step_count": plan.steps.len(),
            "step_ids": plan.steps.iter().map(|s| s.id.to_string()).collect::<Vec<_>>(),
            "state": plan.state.as_str(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.plan.created",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_action_started(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiOrchestratedPlan,
        step_index: usize,
    ) -> Result<()> {
        let step = &plan.steps[step_index];
        let metadata = json!({
            "plan_id": plan.id.as_str(),
            "step_id": step.id.as_str(),
            "ordinal": step.ordinal,
            "proposal_id": step.proposal.id.as_str(),
            "command": step.proposal.command_name,
            "target": step.proposal.target_resource.as_ref().map(|r| r.canonical()),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.plan.action_started",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_action_completed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiOrchestratedPlan,
        step_index: usize,
    ) -> Result<()> {
        let step = &plan.steps[step_index];
        let metadata = json!({
            "plan_id": plan.id.as_str(),
            "step_id": step.id.as_str(),
            "ordinal": step.ordinal,
            "proposal_id": step.proposal.id.as_str(),
            "command": step.proposal.command_name,
            "permission_result": "allowed",
            "execution_result": "completed",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.plan.action_completed",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_action_failed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiOrchestratedPlan,
        step_index: usize,
        permission_result: &str,
    ) -> Result<()> {
        let step = &plan.steps[step_index];
        let metadata = json!({
            "plan_id": plan.id.as_str(),
            "step_id": step.id.as_str(),
            "ordinal": step.ordinal,
            "proposal_id": step.proposal.id.as_str(),
            "command": step.proposal.command_name,
            "permission_result": permission_result,
            "execution_result": step.state.as_str(),
            "detail": step.last_outcome_detail,
            "approval_request_id": step.approval_request_id,
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.plan.action_failed",
            true,
            metadata,
        )
    }

    pub(crate) fn audit_plan_cancelled(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        plan: &AiOrchestratedPlan,
    ) -> Result<()> {
        let metadata = json!({
            "plan_id": plan.id.as_str(),
            "goal_id": plan.goal.id.as_str(),
            "state": plan.state.as_str(),
            "completed_steps": plan.completed_count(),
            "total_steps": plan.steps.len(),
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.plan.cancelled",
            true,
            metadata,
        )
    }

    pub(crate) fn map_submission_error(error: KernelError) -> (AiProposalAuthorityOutcome, bool) {
        crate::services::AiPlanningService::map_submission_error(error)
    }

    pub(crate) fn ensure_can_advance(plan: &AiOrchestratedPlan) -> Result<()> {
        if plan.state.is_terminal() {
            return Err(KernelError::from(AiOrchestrationError::InvalidPlanState(
                plan.state.as_str().into(),
            )));
        }
        if plan.awaiting_approval_step_index().is_some() {
            // Paused — caller must resume after human DecideApproval.
            return Err(KernelError::from(AiOrchestrationError::InvalidPlanState(
                AiOrchestratedPlanState::AwaitingApproval.as_str().into(),
            )));
        }
        if plan.next_runnable_step_index().is_none() {
            return Err(KernelError::from(AiOrchestrationError::InvalidPlanState(
                plan.state.as_str().into(),
            )));
        }
        Ok(())
    }

    pub(crate) fn ensure_can_resume(plan: &AiOrchestratedPlan) -> Result<usize> {
        if plan.state.is_terminal() {
            return Err(KernelError::from(AiOrchestrationError::InvalidPlanState(
                plan.state.as_str().into(),
            )));
        }
        plan.awaiting_approval_step_index()
            .ok_or_else(|| {
                KernelError::from(AiOrchestrationError::InvalidPlanState(
                    plan.state.as_str().into(),
                ))
            })
    }

    pub(crate) fn step_application_id(plan: &AiOrchestratedPlan, index: usize) -> Result<String> {
        let step = plan
            .steps
            .get(index)
            .ok_or(AiOrchestrationError::InvalidStepIndex)
            .map_err(KernelError::from)?;
        Ok(step
            .proposal
            .application_id()
            .map_err(KernelError::from)?
            .to_string())
    }

    pub(crate) fn step_reason(plan: &AiOrchestratedPlan, index: usize) -> Option<String> {
        plan.steps
            .get(index)
            .and_then(|step| step.proposal.explanation.clone())
    }

}
