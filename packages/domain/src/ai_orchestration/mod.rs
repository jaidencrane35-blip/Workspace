//! Governed AI action orchestration — multi-step plans (Sprints 56–57).
//!
//! A plan is a structured collection of requested actions.
//! It is NOT permission and NOT execution authority.
//! Each step must still pass through the Permission Gateway.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_planning::{
    AiActionProposal, AiGoal, AiPlan, AiProposalAuthorityOutcome,
};
use crate::ids::{ActorId, AiOrchestratedPlanId, AiPlanStepId};

/// Orchestration-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiOrchestrationError {
    #[error("AI orchestrated plan has no steps")]
    EmptyPlan,

    #[error("AI orchestrated plan step index out of range")]
    InvalidStepIndex,

    #[error("AI orchestrated plan is not runnable in state {0}")]
    InvalidPlanState(String),

    #[error("AI orchestrated plan step is not runnable in state {0}")]
    InvalidStepState(String),

    #[error("AI orchestrated plan dependency cycle or missing dependency")]
    InvalidDependency,

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

/// Lifecycle state of an orchestrated multi-step plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiOrchestratedPlanState {
    Proposed,
    AwaitingApproval,
    PartiallyApproved,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

impl AiOrchestratedPlanState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::AwaitingApproval => "awaiting_approval",
            Self::PartiallyApproved => "partially_approved",
            Self::Executing => "executing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Per-step lifecycle within an orchestrated plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiPlanStepState {
    Pending,
    AwaitingApproval,
    Running,
    Completed,
    Denied,
    Failed,
    Cancelled,
}

impl AiPlanStepState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::AwaitingApproval => "awaiting_approval",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Denied | Self::Failed | Self::Cancelled
        )
    }
}

/// One ordered action inside an orchestrated plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPlanStep {
    pub id: AiPlanStepId,
    pub ordinal: usize,
    pub proposal: AiActionProposal,
    /// Steps that must complete before this one may run (empty = ordinal order only).
    pub depends_on: Vec<AiPlanStepId>,
    pub state: AiPlanStepState,
    pub approval_request_id: Option<String>,
    pub last_outcome_detail: Option<String>,
}

/// Structured multi-step AI plan — not permission, not execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiOrchestratedPlan {
    pub id: AiOrchestratedPlanId,
    pub goal: AiGoal,
    pub requesting_actor_id: ActorId,
    pub steps: Vec<AiPlanStep>,
    pub state: AiOrchestratedPlanState,
    pub current_step_index: Option<usize>,
    pub created_at: String,
    pub updated_at: String,
}

impl AiOrchestratedPlan {
    /// Builds an orchestrated plan from a diagnostic [`AiPlan`] (no execution).
    pub fn from_ai_plan(plan: AiPlan) -> Result<Self, AiOrchestrationError> {
        if plan.proposals.is_empty() {
            return Err(AiOrchestrationError::EmptyPlan);
        }

        let mut steps = Vec::with_capacity(plan.proposals.len());
        let mut previous_step_id: Option<AiPlanStepId> = None;

        for (ordinal, proposal) in plan.proposals.into_iter().enumerate() {
            let id = AiPlanStepId::generate();
            let depends_on = previous_step_id
                .clone()
                .map(|dep| vec![dep])
                .unwrap_or_default();
            previous_step_id = Some(id.clone());
            steps.push(AiPlanStep {
                id,
                ordinal,
                proposal,
                depends_on,
                state: AiPlanStepState::Pending,
                approval_request_id: None,
                last_outcome_detail: None,
            });
        }

        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: AiOrchestratedPlanId::generate(),
            requesting_actor_id: plan.goal.requesting_actor_id.clone(),
            goal: plan.goal,
            steps,
            state: AiOrchestratedPlanState::Proposed,
            current_step_index: None,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn completed_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|step| step.state == AiPlanStepState::Completed)
            .count()
    }

    pub fn all_steps_completed(&self) -> bool {
        self.steps
            .iter()
            .all(|step| step.state == AiPlanStepState::Completed)
    }

    /// Next step that may run: pending, dependencies satisfied, plan not terminal.
    pub fn next_runnable_step_index(&self) -> Option<usize> {
        if self.state.is_terminal()
            || self.state == AiOrchestratedPlanState::AwaitingApproval
        {
            return None;
        }

        self.steps.iter().position(|step| {
            step.state == AiPlanStepState::Pending && self.dependencies_satisfied(step)
        })
    }

    /// Step currently paused for human approval.
    pub fn awaiting_approval_step_index(&self) -> Option<usize> {
        self.steps
            .iter()
            .position(|step| step.state == AiPlanStepState::AwaitingApproval)
    }

    fn dependencies_satisfied(&self, step: &AiPlanStep) -> bool {
        step.depends_on.iter().all(|dep| {
            self.steps.iter().any(|candidate| {
                candidate.id == *dep && candidate.state == AiPlanStepState::Completed
            })
        })
    }

    pub fn begin_step(&mut self, index: usize) -> Result<(), AiOrchestrationError> {
        if self.state.is_terminal() {
            return Err(AiOrchestrationError::InvalidPlanState(
                self.state.as_str().into(),
            ));
        }
        let step = self
            .steps
            .get_mut(index)
            .ok_or(AiOrchestrationError::InvalidStepIndex)?;
        if !matches!(
            step.state,
            AiPlanStepState::Pending | AiPlanStepState::AwaitingApproval
        ) {
            return Err(AiOrchestrationError::InvalidStepState(
                step.state.as_str().into(),
            ));
        }
        step.state = AiPlanStepState::Running;
        step.approval_request_id = None;
        self.current_step_index = Some(index);
        self.state = AiOrchestratedPlanState::Executing;
        self.touch();
        Ok(())
    }

    /// Applies a Permission Gateway authority outcome to the current/running step.
    pub fn apply_authority_outcome(
        &mut self,
        index: usize,
        outcome: &AiProposalAuthorityOutcome,
    ) -> Result<(), AiOrchestrationError> {
        let completed_before = self.completed_count();
        let step = self
            .steps
            .get_mut(index)
            .ok_or(AiOrchestrationError::InvalidStepIndex)?;

        match outcome {
            AiProposalAuthorityOutcome::Allowed => {
                step.state = AiPlanStepState::Completed;
                step.approval_request_id = None;
                step.last_outcome_detail = None;
            }
            AiProposalAuthorityOutcome::Denied { reason } => {
                step.state = AiPlanStepState::Denied;
                step.approval_request_id = None;
                step.last_outcome_detail = Some(reason.clone());
                self.state = AiOrchestratedPlanState::Failed;
                self.touch();
                return Ok(());
            }
            AiProposalAuthorityOutcome::ApprovalRequired {
                reason,
                approval_request_id,
            } => {
                step.state = AiPlanStepState::AwaitingApproval;
                step.approval_request_id = Some(approval_request_id.clone());
                step.last_outcome_detail = Some(reason.clone());
                self.state = if completed_before > 0 {
                    AiOrchestratedPlanState::PartiallyApproved
                } else {
                    AiOrchestratedPlanState::AwaitingApproval
                };
                self.touch();
                return Ok(());
            }
        }

        if self.all_steps_completed() {
            self.state = AiOrchestratedPlanState::Completed;
        } else if self.completed_count() > 0 {
            self.state = AiOrchestratedPlanState::PartiallyApproved;
        } else {
            self.state = AiOrchestratedPlanState::Proposed;
        }
        self.touch();
        Ok(())
    }

    /// Records an unexpected execution/infrastructure failure (not a deny).
    pub fn apply_step_failure(
        &mut self,
        index: usize,
        detail: impl Into<String>,
    ) -> Result<(), AiOrchestrationError> {
        let step = self
            .steps
            .get_mut(index)
            .ok_or(AiOrchestrationError::InvalidStepIndex)?;
        step.state = AiPlanStepState::Failed;
        step.last_outcome_detail = Some(detail.into());
        self.state = AiOrchestratedPlanState::Failed;
        self.touch();
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), AiOrchestrationError> {
        if self.state == AiOrchestratedPlanState::Completed {
            return Err(AiOrchestrationError::InvalidPlanState(
                self.state.as_str().into(),
            ));
        }
        if self.state == AiOrchestratedPlanState::Cancelled {
            return Ok(());
        }
        for step in &mut self.steps {
            if !step.state.is_terminal() {
                step.state = AiPlanStepState::Cancelled;
            }
        }
        self.state = AiOrchestratedPlanState::Cancelled;
        self.touch();
        Ok(())
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ApplicationId;

    fn two_step_plan() -> AiOrchestratedPlan {
        let goal = AiGoal::new("Prepare my coding workspace", "ai-1").unwrap();
        let app_a = ApplicationId::new("app-1").unwrap();
        let app_b = ApplicationId::new("app-2").unwrap();
        let proposals = vec![
            AiActionProposal::propose_application_launch(&goal, &app_a, Some("Open IDE".into()))
                .unwrap(),
            AiActionProposal::propose_application_launch(
                &goal,
                &app_b,
                Some("Open terminal".into()),
            )
            .unwrap(),
        ];
        AiOrchestratedPlan::from_ai_plan(AiPlan { goal, proposals }).unwrap()
    }

    #[test]
    fn creates_plan_without_execution_signals() {
        let plan = two_step_plan();
        assert_eq!(plan.state, AiOrchestratedPlanState::Proposed);
        assert_eq!(plan.steps.len(), 2);
        assert!(plan.steps.iter().all(|s| s.state == AiPlanStepState::Pending));
        assert_eq!(plan.steps[1].depends_on, vec![plan.steps[0].id.clone()]);
    }

    #[test]
    fn approval_required_pauses_plan() {
        let mut plan = two_step_plan();
        plan.begin_step(0).unwrap();
        plan.apply_authority_outcome(
            0,
            &AiProposalAuthorityOutcome::ApprovalRequired {
                reason: "needs approval".into(),
                approval_request_id: "apr-1".into(),
            },
        )
        .unwrap();
        assert_eq!(plan.state, AiOrchestratedPlanState::AwaitingApproval);
        assert_eq!(plan.steps[0].state, AiPlanStepState::AwaitingApproval);
        assert!(plan.next_runnable_step_index().is_none());
    }

    #[test]
    fn denial_fails_plan_and_blocks_later_steps() {
        let mut plan = two_step_plan();
        plan.begin_step(0).unwrap();
        plan.apply_authority_outcome(
            0,
            &AiProposalAuthorityOutcome::Allowed,
        )
        .unwrap();
        plan.begin_step(1).unwrap();
        plan.apply_authority_outcome(
            1,
            &AiProposalAuthorityOutcome::Denied {
                reason: "denied".into(),
            },
        )
        .unwrap();
        assert_eq!(plan.state, AiOrchestratedPlanState::Failed);
        assert_eq!(plan.steps[1].state, AiPlanStepState::Denied);
        assert!(plan.next_runnable_step_index().is_none());
    }

    #[test]
    fn cancel_stops_pending_steps() {
        let mut plan = two_step_plan();
        plan.cancel().unwrap();
        assert_eq!(plan.state, AiOrchestratedPlanState::Cancelled);
        assert!(plan
            .steps
            .iter()
            .all(|s| s.state == AiPlanStepState::Cancelled));
    }
}
