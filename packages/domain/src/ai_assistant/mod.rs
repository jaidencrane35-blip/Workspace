//! Governed AI assistant — user-facing interface layer (Sprints 58–59).
//!
//! The assistant translates human goals into governed plans.
//! It is NOT an authority layer and does NOT execute actions.
//!
//! Separations preserved:
//! - Assistant: what does the user want?
//! - Planner: what actions could achieve it?
//! - Permission Gateway: are those actions allowed?

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_orchestration::{
    AiOrchestratedPlan, AiOrchestratedPlanState, AiPlanStep, AiPlanStepState,
};
use crate::ids::{ActorId, AiAssistantWorkflowId, AiOrchestratedPlanId};

/// Assistant-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiAssistantError {
    #[error("AI assistant goal must not be empty")]
    EmptyGoal,

    #[error("AI assistant goal exceeds maximum length")]
    GoalTooLong,

    #[error("AI assistant workflow is not runnable in state {0}")]
    InvalidWorkflowState(String),

    #[error("AI assistant workflow has no linked plan")]
    MissingPlan,

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

const MAX_GOAL_LEN: usize = 500;

/// User-facing assistant workflow states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiAssistantWorkflowState {
    ReceivingGoal,
    Understanding,
    GeneratingPlan,
    AwaitingConfirmation,
    SubmittingActions,
    WaitingForPermission,
    Completed,
    Failed,
    Cancelled,
}

impl AiAssistantWorkflowState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReceivingGoal => "receiving_goal",
            Self::Understanding => "understanding",
            Self::GeneratingPlan => "generating_plan",
            Self::AwaitingConfirmation => "awaiting_confirmation",
            Self::SubmittingActions => "submitting_actions",
            Self::WaitingForPermission => "waiting_for_permission",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Human-readable preview of one proposed action (not authority).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantActionPreview {
    pub step_id: String,
    pub ordinal: usize,
    pub command_name: String,
    pub target: Option<String>,
    pub explanation: Option<String>,
    pub step_state: String,
    /// Informational capability hint (existence ≠ authorization).
    pub capability_hint: Option<String>,
}

/// Plan preview presented to the user before confirmation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantPlanPreview {
    pub plan_id: String,
    pub goal_statement: String,
    pub actions: Vec<AiAssistantActionPreview>,
    pub permission_note: String,
}

impl AiAssistantPlanPreview {
    pub const PERMISSION_NOTE: &'static str =
        "Each action requires Permission Gateway approval. Seeing a proposal does not authorize it.";

    pub fn from_orchestrated_plan(
        plan: &AiOrchestratedPlan,
        capability_hint_for_command: impl Fn(&str) -> Option<String>,
    ) -> Self {
        let actions = plan
            .steps
            .iter()
            .map(|step| preview_step(step, &capability_hint_for_command))
            .collect();
        Self {
            plan_id: plan.id.to_string(),
            goal_statement: plan.goal.statement.clone(),
            actions,
            permission_note: Self::PERMISSION_NOTE.into(),
        }
    }
}

fn preview_step(
    step: &AiPlanStep,
    capability_hint_for_command: &impl Fn(&str) -> Option<String>,
) -> AiAssistantActionPreview {
    AiAssistantActionPreview {
        step_id: step.id.to_string(),
        ordinal: step.ordinal,
        command_name: step.proposal.command_name.clone(),
        target: step
            .proposal
            .target_resource
            .as_ref()
            .map(|resource| resource.canonical()),
        explanation: step.proposal.explanation.clone(),
        step_state: step.state.as_str().to_string(),
        capability_hint: capability_hint_for_command(&step.proposal.command_name),
    }
}

/// Governed assistant workflow — interface only, not execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantWorkflow {
    pub id: AiAssistantWorkflowId,
    pub user_goal: String,
    pub requesting_actor_id: ActorId,
    pub state: AiAssistantWorkflowState,
    pub orchestrated_plan_id: Option<AiOrchestratedPlanId>,
    pub plan_preview: Option<AiAssistantPlanPreview>,
    pub status_message: String,
    pub created_at: String,
    pub updated_at: String,
}

impl AiAssistantWorkflow {
    pub fn receive_goal(
        user_goal: impl Into<String>,
        requesting_actor_id: impl Into<String>,
    ) -> Result<Self, AiAssistantError> {
        let user_goal = normalize_goal(user_goal.into())?;
        let requesting_actor_id = ActorId::new(requesting_actor_id)?;
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: AiAssistantWorkflowId::generate(),
            user_goal,
            requesting_actor_id,
            state: AiAssistantWorkflowState::ReceivingGoal,
            orchestrated_plan_id: None,
            plan_preview: None,
            status_message: "Goal received. Understanding workspace intent…".into(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn mark_understanding(&mut self) {
        self.state = AiAssistantWorkflowState::Understanding;
        self.status_message = "Understanding your goal…".into();
        self.touch();
    }

    pub fn mark_generating_plan(&mut self) {
        self.state = AiAssistantWorkflowState::GeneratingPlan;
        self.status_message = "Generating a governed plan…".into();
        self.touch();
    }

    pub fn present_plan(
        &mut self,
        plan: &AiOrchestratedPlan,
        preview: AiAssistantPlanPreview,
    ) -> Result<(), AiAssistantError> {
        if !matches!(
            self.state,
            AiAssistantWorkflowState::ReceivingGoal
                | AiAssistantWorkflowState::Understanding
                | AiAssistantWorkflowState::GeneratingPlan
        ) {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        self.orchestrated_plan_id = Some(plan.id.clone());
        self.plan_preview = Some(preview);
        self.state = AiAssistantWorkflowState::AwaitingConfirmation;
        self.status_message =
            "Plan ready for review. Confirm to submit through the Permission Gateway.".into();
        self.touch();
        Ok(())
    }

    pub fn confirm(&mut self) -> Result<(), AiAssistantError> {
        if self.state != AiAssistantWorkflowState::AwaitingConfirmation {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        if self.orchestrated_plan_id.is_none() {
            return Err(AiAssistantError::MissingPlan);
        }
        self.state = AiAssistantWorkflowState::SubmittingActions;
        self.status_message = "Submitting actions through governance…".into();
        self.touch();
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), AiAssistantError> {
        if self.state == AiAssistantWorkflowState::Completed {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        if self.state == AiAssistantWorkflowState::Cancelled {
            return Ok(());
        }
        self.state = AiAssistantWorkflowState::Cancelled;
        self.status_message = "Workflow cancelled. No further actions will run.".into();
        self.touch();
        Ok(())
    }

    /// Syncs assistant UX state from the linked orchestrated plan.
    pub fn sync_from_plan(&mut self, plan: &AiOrchestratedPlan) {
        self.refresh_preview_states(plan);
        match plan.state {
            AiOrchestratedPlanState::Proposed => {
                if self.state == AiAssistantWorkflowState::SubmittingActions {
                    // Still awaiting confirmation or just confirmed but not advanced.
                } else if !self.state.is_terminal()
                    && self.state != AiAssistantWorkflowState::AwaitingConfirmation
                {
                    self.state = AiAssistantWorkflowState::AwaitingConfirmation;
                }
            }
            AiOrchestratedPlanState::Executing => {
                self.state = AiAssistantWorkflowState::SubmittingActions;
                self.status_message = "Actions submitting through Permission Gateway…".into();
            }
            AiOrchestratedPlanState::AwaitingApproval
            | AiOrchestratedPlanState::PartiallyApproved => {
                if plan.awaiting_approval_step_index().is_some() {
                    self.state = AiAssistantWorkflowState::WaitingForPermission;
                    self.status_message =
                        "Waiting for human permission decision on a proposed action.".into();
                } else if plan.next_runnable_step_index().is_some() {
                    self.state = AiAssistantWorkflowState::SubmittingActions;
                    self.status_message = "Continuing remaining governed actions…".into();
                } else {
                    self.state = AiAssistantWorkflowState::WaitingForPermission;
                }
            }
            AiOrchestratedPlanState::Completed => {
                self.state = AiAssistantWorkflowState::Completed;
                self.status_message = "Workflow completed through governed execution.".into();
            }
            AiOrchestratedPlanState::Failed => {
                self.state = AiAssistantWorkflowState::Failed;
                let detail = plan
                    .steps
                    .iter()
                    .rev()
                    .find(|step| {
                        matches!(
                            step.state,
                            AiPlanStepState::Denied | AiPlanStepState::Failed
                        )
                    })
                    .and_then(|step| step.last_outcome_detail.clone())
                    .unwrap_or_else(|| "A governed action was blocked or failed.".into());
                self.status_message = format!("Workflow blocked: {detail}");
            }
            AiOrchestratedPlanState::Cancelled => {
                self.state = AiAssistantWorkflowState::Cancelled;
                self.status_message = "Linked plan cancelled.".into();
            }
        }
        self.touch();
    }

    fn refresh_preview_states(&mut self, plan: &AiOrchestratedPlan) {
        if let Some(preview) = &mut self.plan_preview {
            for action in &mut preview.actions {
                if let Some(step) = plan.steps.iter().find(|s| s.id.as_str() == action.step_id) {
                    action.step_state = step.state.as_str().to_string();
                }
            }
        }
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

fn normalize_goal(value: String) -> Result<String, AiAssistantError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AiAssistantError::EmptyGoal);
    }
    if trimmed.len() > MAX_GOAL_LEN {
        return Err(AiAssistantError::GoalTooLong);
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_planning::{AiActionProposal, AiGoal, AiPlan};
    use crate::ai_orchestration::AiOrchestratedPlan;
    use crate::ids::ApplicationId;

    fn sample_plan() -> AiOrchestratedPlan {
        let goal = AiGoal::new("Prepare my coding workspace", "diagnostic-ai").unwrap();
        let app = ApplicationId::new("app-1").unwrap();
        let proposal = AiActionProposal::propose_application_launch(
            &goal,
            &app,
            Some("Open IDE".into()),
        )
        .unwrap();
        AiOrchestratedPlan::from_ai_plan(AiPlan {
            goal,
            proposals: vec![proposal],
        })
        .unwrap()
    }

    #[test]
    fn goal_creates_workflow_without_plan_execution() {
        let workflow =
            AiAssistantWorkflow::receive_goal("Prepare my coding workspace", "diagnostic-ai")
                .unwrap();
        assert_eq!(workflow.state, AiAssistantWorkflowState::ReceivingGoal);
        assert!(workflow.orchestrated_plan_id.is_none());
    }

    #[test]
    fn presenting_plan_awaits_confirmation() {
        let mut workflow =
            AiAssistantWorkflow::receive_goal("Prepare my coding workspace", "diagnostic-ai")
                .unwrap();
        workflow.mark_understanding();
        workflow.mark_generating_plan();
        let plan = sample_plan();
        let preview = AiAssistantPlanPreview::from_orchestrated_plan(&plan, |_| {
            Some("Requires capability: application.launch".into())
        });
        workflow.present_plan(&plan, preview).unwrap();
        assert_eq!(workflow.state, AiAssistantWorkflowState::AwaitingConfirmation);
        assert!(workflow.plan_preview.is_some());
    }

    #[test]
    fn cancel_before_confirm_blocks_progression() {
        let mut workflow =
            AiAssistantWorkflow::receive_goal("Prepare my coding workspace", "diagnostic-ai")
                .unwrap();
        let plan = sample_plan();
        let preview = AiAssistantPlanPreview::from_orchestrated_plan(&plan, |_| None);
        workflow.present_plan(&plan, preview).unwrap();
        workflow.cancel().unwrap();
        assert_eq!(workflow.state, AiAssistantWorkflowState::Cancelled);
        assert!(workflow.confirm().is_err());
    }
}
