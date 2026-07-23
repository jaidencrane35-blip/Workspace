//! Governed AI assistant — user-facing interface layer (Sprints 58–59, 66–67).
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

    #[error("AI assistant plan revision not found")]
    RevisionNotFound,

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

const MAX_GOAL_LEN: usize = 500;

/// User-facing assistant workflow states (product + diagnostic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiAssistantWorkflowState {
    ReceivingGoal,
    Understanding,
    GeneratingPlan,
    Evaluating,
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
            Self::Evaluating => "evaluating",
            Self::AwaitingConfirmation => "awaiting_confirmation",
            Self::SubmittingActions => "submitting_actions",
            Self::WaitingForPermission => "waiting_for_permission",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Product UX label (maps submitting_actions → executing).
    pub fn product_label(self) -> &'static str {
        match self {
            Self::ReceivingGoal => "idle",
            Self::Understanding => "understanding",
            Self::GeneratingPlan => "planning",
            Self::Evaluating => "evaluating",
            Self::AwaitingConfirmation => "awaiting_confirmation",
            Self::SubmittingActions => "executing",
            Self::WaitingForPermission => "awaiting_permission",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Structured, non-CoT explanation for one proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantActionExplanation {
    pub why_suggested: String,
    pub why_permission: String,
    pub what_if_approve: String,
    /// Informational tags such as `preference`, `memory`, `context`.
    pub influence_tags: Vec<String>,
}

impl AiAssistantActionExplanation {
    pub fn from_preview_parts(
        explanation: Option<&str>,
        capability_hint: Option<&str>,
        command_name: &str,
        target: Option<&str>,
    ) -> Self {
        let why_suggested = explanation
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("Suggested to help achieve your stated goal.")
            .to_string();

        let why_permission = capability_hint
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|hint| {
                if hint.to_lowercase().contains("permission")
                    || hint.to_lowercase().contains("capability")
                {
                    hint.to_string()
                } else {
                    format!("Permission required: {hint}")
                }
            })
            .unwrap_or_else(|| {
                if command_name == "LaunchApplication" {
                    "Permission required because this launches a desktop application.".into()
                } else {
                    format!("Permission required before `{command_name}` can run.")
                }
            });

        let what_if_approve = match target {
            Some(target) => format!(
                "If you approve, `{command_name}` will run through the Permission Gateway for {target}."
            ),
            None => format!(
                "If you approve, `{command_name}` will run through the Permission Gateway."
            ),
        };

        let mut influence_tags = Vec::new();
        let lower = why_suggested.to_lowercase();
        if lower.contains("preferred because") || lower.contains("personalization") {
            influence_tags.push("preference".into());
        }
        if lower.contains("memory:") || lower.contains("preferred from memory") {
            influence_tags.push("memory".into());
        }
        if lower.contains("workspace") || lower.contains("not appearing active") {
            influence_tags.push("context".into());
        }
        if influence_tags.is_empty() {
            influence_tags.push("planning".into());
        }

        Self {
            why_suggested,
            why_permission,
            what_if_approve,
            influence_tags,
        }
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
    /// Structured explanation for product UX (no chain-of-thought).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_explanation: Option<AiAssistantActionExplanation>,
}

/// Plan preview presented to the user before confirmation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantPlanPreview {
    pub plan_id: String,
    pub goal_statement: String,
    pub actions: Vec<AiAssistantActionPreview>,
    pub permission_note: String,
    /// Summary of preference/memory influence for the product panel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influence_summary: Option<String>,
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
            .collect::<Vec<_>>();
        let influence_summary = summarize_influences(&actions);
        Self {
            plan_id: plan.id.to_string(),
            goal_statement: plan.goal.statement.clone(),
            actions,
            permission_note: Self::PERMISSION_NOTE.into(),
            influence_summary,
        }
    }
}

/// Snapshot of a previous plan for revise/compare UX.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantPlanRevision {
    pub revision: u32,
    pub plan_id: String,
    pub goal_statement: String,
    pub preview: AiAssistantPlanPreview,
    pub created_at: String,
}

/// Side-by-side comparison of two plan revisions (informational only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantPlanComparison {
    pub workflow_id: String,
    pub left: AiAssistantPlanRevision,
    pub right: AiAssistantPlanRevision,
    pub differences: Vec<String>,
}

impl AiAssistantPlanComparison {
    pub fn compare(left: AiAssistantPlanRevision, right: AiAssistantPlanRevision) -> Self {
        let mut differences = Vec::new();
        if left.goal_statement != right.goal_statement {
            differences.push(format!(
                "Goal changed: \"{}\" → \"{}\"",
                left.goal_statement, right.goal_statement
            ));
        }
        if left.preview.actions.len() != right.preview.actions.len() {
            differences.push(format!(
                "Action count: {} → {}",
                left.preview.actions.len(),
                right.preview.actions.len()
            ));
        }
        let left_commands: Vec<_> = left
            .preview
            .actions
            .iter()
            .map(|a| {
                format!(
                    "{}:{}",
                    a.command_name,
                    a.target.clone().unwrap_or_default()
                )
            })
            .collect();
        let right_commands: Vec<_> = right
            .preview
            .actions
            .iter()
            .map(|a| {
                format!(
                    "{}:{}",
                    a.command_name,
                    a.target.clone().unwrap_or_default()
                )
            })
            .collect();
        if left_commands != right_commands {
            differences.push(format!(
                "Actions differ: [{}] vs [{}]",
                left_commands.join(", "),
                right_commands.join(", ")
            ));
        }
        if left.preview.influence_summary != right.preview.influence_summary {
            differences.push("Memory/preference influence summary differs.".into());
        }
        if differences.is_empty() {
            differences.push("No material differences between these plan revisions.".into());
        }
        Self {
            workflow_id: String::new(),
            left,
            right,
            differences,
        }
    }
}

fn summarize_influences(actions: &[AiAssistantActionPreview]) -> Option<String> {
    let mut tags = Vec::new();
    for action in actions {
        if let Some(detail) = &action.structured_explanation {
            for tag in &detail.influence_tags {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            }
        }
    }
    if tags.is_empty() {
        None
    } else {
        Some(format!("Plan influenced by: {}", tags.join(", ")))
    }
}

fn preview_step(
    step: &AiPlanStep,
    capability_hint_for_command: &impl Fn(&str) -> Option<String>,
) -> AiAssistantActionPreview {
    let capability_hint = capability_hint_for_command(&step.proposal.command_name);
    let target = step
        .proposal
        .target_resource
        .as_ref()
        .map(|resource| resource.canonical());
    let structured_explanation = Some(AiAssistantActionExplanation::from_preview_parts(
        step.proposal.explanation.as_deref(),
        capability_hint.as_deref(),
        &step.proposal.command_name,
        target.as_deref(),
    ));
    AiAssistantActionPreview {
        step_id: step.id.to_string(),
        ordinal: step.ordinal,
        command_name: step.proposal.command_name.clone(),
        target,
        explanation: step.proposal.explanation.clone(),
        step_state: step.state.as_str().to_string(),
        capability_hint,
        structured_explanation,
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
    /// Prior plan snapshots for revise/compare (informational).
    #[serde(default)]
    pub plan_revisions: Vec<AiAssistantPlanRevision>,
    /// Context retained so regenerate/revise reuse the same planning inputs.
    #[serde(default)]
    pub application_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default)]
    pub revision_counter: u32,
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
            plan_revisions: Vec::new(),
            application_ids: Vec::new(),
            workspace_id: None,
            revision_counter: 0,
        })
    }

    pub fn with_planning_context(
        mut self,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Self {
        self.application_ids = application_ids;
        self.workspace_id = workspace_id;
        self
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

    pub fn mark_evaluating(&mut self) {
        self.state = AiAssistantWorkflowState::Evaluating;
        self.status_message = "Evaluating plan quality and permission needs…".into();
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
                | AiAssistantWorkflowState::Evaluating
                | AiAssistantWorkflowState::AwaitingConfirmation
                | AiAssistantWorkflowState::Cancelled
        ) {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        self.archive_current_preview();
        self.orchestrated_plan_id = Some(plan.id.clone());
        self.plan_preview = Some(preview);
        self.state = AiAssistantWorkflowState::AwaitingConfirmation;
        self.status_message =
            "Plan ready for review. Confirm to submit through the Permission Gateway.".into();
        self.touch();
        Ok(())
    }

    pub fn revise_goal(&mut self, new_goal: impl Into<String>) -> Result<(), AiAssistantError> {
        if !matches!(
            self.state,
            AiAssistantWorkflowState::AwaitingConfirmation
                | AiAssistantWorkflowState::Cancelled
                | AiAssistantWorkflowState::Failed
        ) {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        self.user_goal = normalize_goal(new_goal.into())?;
        self.archive_current_preview();
        self.orchestrated_plan_id = None;
        self.plan_preview = None;
        self.state = AiAssistantWorkflowState::Understanding;
        self.status_message = "Goal updated. Regenerating governed plan…".into();
        self.touch();
        Ok(())
    }

    pub fn prepare_regenerate(&mut self) -> Result<(), AiAssistantError> {
        if self.state != AiAssistantWorkflowState::AwaitingConfirmation {
            return Err(AiAssistantError::InvalidWorkflowState(
                self.state.as_str().into(),
            ));
        }
        self.archive_current_preview();
        self.orchestrated_plan_id = None;
        self.plan_preview = None;
        self.state = AiAssistantWorkflowState::GeneratingPlan;
        self.status_message = "Regenerating governed plan…".into();
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

    pub fn revision_by_number(&self, revision: u32) -> Option<&AiAssistantPlanRevision> {
        self.plan_revisions
            .iter()
            .find(|item| item.revision == revision)
    }

    pub fn current_as_revision(&self) -> Option<AiAssistantPlanRevision> {
        let preview = self.plan_preview.as_ref()?;
        Some(AiAssistantPlanRevision {
            revision: self.revision_counter,
            plan_id: preview.plan_id.clone(),
            goal_statement: preview.goal_statement.clone(),
            preview: preview.clone(),
            created_at: self.updated_at.clone(),
        })
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
                self.status_message = format!(
                    "Workflow blocked: {detail}. You can revise the goal or cancel."
                );
            }
            AiOrchestratedPlanState::Cancelled => {
                self.state = AiAssistantWorkflowState::Cancelled;
                self.status_message = "Linked plan cancelled.".into();
            }
        }
        self.touch();
    }

    fn archive_current_preview(&mut self) {
        if let Some(preview) = self.plan_preview.take() {
            self.revision_counter = self.revision_counter.saturating_add(1);
            self.plan_revisions.push(AiAssistantPlanRevision {
                revision: self.revision_counter,
                plan_id: preview.plan_id.clone(),
                goal_statement: preview.goal_statement.clone(),
                preview,
                created_at: Utc::now().to_rfc3339(),
            });
        }
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
    use crate::ai_orchestration::AiOrchestratedPlan;
    use crate::ai_planning::{AiActionProposal, AiGoal, AiPlan};
    use crate::ids::ApplicationId;

    fn sample_plan() -> AiOrchestratedPlan {
        let goal = AiGoal::new("Prepare my coding workspace", "diagnostic-ai").unwrap();
        let app = ApplicationId::new("app-1").unwrap();
        let proposal = AiActionProposal::propose_application_launch(
            &goal,
            &app,
            Some("Preferred because you marked VS Code as your default editor.".into()),
        )
        .unwrap();
        AiOrchestratedPlan::from_ai_plan(AiPlan::new(goal, vec![proposal])).unwrap()
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
    fn presenting_plan_awaits_confirmation_with_structured_explanations() {
        let mut workflow =
            AiAssistantWorkflow::receive_goal("Prepare my coding workspace", "diagnostic-ai")
                .unwrap();
        workflow.mark_understanding();
        workflow.mark_generating_plan();
        workflow.mark_evaluating();
        let plan = sample_plan();
        let preview = AiAssistantPlanPreview::from_orchestrated_plan(&plan, |_| {
            Some("Requires capability: application.launch".into())
        });
        assert!(preview.actions[0].structured_explanation.is_some());
        workflow.present_plan(&plan, preview).unwrap();
        assert_eq!(workflow.state, AiAssistantWorkflowState::AwaitingConfirmation);
        assert_eq!(
            workflow.state.product_label(),
            "awaiting_confirmation"
        );
    }

    #[test]
    fn revise_goal_archives_previous_preview() {
        let mut workflow =
            AiAssistantWorkflow::receive_goal("Prepare my coding workspace", "diagnostic-ai")
                .unwrap();
        let plan = sample_plan();
        let preview = AiAssistantPlanPreview::from_orchestrated_plan(&plan, |_| None);
        workflow.present_plan(&plan, preview).unwrap();
        workflow.revise_goal("Prepare a lighter workspace").unwrap();
        assert_eq!(workflow.user_goal, "Prepare a lighter workspace");
        assert_eq!(workflow.plan_revisions.len(), 1);
        assert!(workflow.plan_preview.is_none());
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
