//! AI planning foundation — goals and action proposals (Sprint 48).
//!
//! Pure domain. Describes *what should happen*, never executes.
//! Proposals convert to [`crate::ai_request::AiActionRequest`] for governance.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_request::{AiActionRequest, AiRequestError};
use crate::ids::{
    ActionIntentId, ActorId, AiActionProposalId, AiGoalId, ApplicationId,
};
use crate::resource::{ResourceId, ResourceKind, ResourceRef};

/// Planning-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiPlanningError {
    #[error("AI goal statement must not be empty")]
    EmptyGoalStatement,

    #[error("AI goal statement exceeds maximum length")]
    GoalStatementTooLong,

    #[error("AI planning actor id must not be empty")]
    EmptyActorId,

    #[error("AI proposal explanation exceeds maximum length")]
    ExplanationTooLong,

    #[error("AI proposal command name must not be empty")]
    EmptyCommandName,

    #[error("AI proposal requires a target resource")]
    MissingTarget,

    #[error("AI proposal target must be an application")]
    InvalidTargetKind,

    #[error(transparent)]
    Request(#[from] AiRequestError),

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

const MAX_GOAL_LEN: usize = 500;
const MAX_EXPLANATION_LEN: usize = 500;
const LAUNCH_APPLICATION_INTENT: &str = "launch-application";
const LAUNCH_APPLICATION_COMMAND: &str = "LaunchApplication";

/// "What is the user trying to achieve?" — not executable commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGoal {
    pub id: AiGoalId,
    pub statement: String,
    pub requesting_actor_id: ActorId,
    pub created_at: String,
}

impl AiGoal {
    pub fn new(
        statement: impl Into<String>,
        requesting_actor_id: impl Into<String>,
    ) -> Result<Self, AiPlanningError> {
        let statement = normalize_text(statement.into(), MAX_GOAL_LEN)?
            .ok_or(AiPlanningError::EmptyGoalStatement)?;
        let requesting_actor_id = ActorId::new(requesting_actor_id)?;
        if requesting_actor_id.as_str().trim().is_empty() {
            return Err(AiPlanningError::EmptyActorId);
        }
        Ok(Self {
            id: AiGoalId::generate(),
            statement,
            requesting_actor_id,
            created_at: Utc::now().to_rfc3339(),
        })
    }
}

/// Boundary for AI reasoning inputs — not long-term memory.
///
/// `User Goal + Available Context + System State hints = Possible Action Proposals`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPlanningContext {
    pub goal: AiGoal,
    /// Minimal system-state hint: applications the planner may consider.
    pub available_application_ids: Vec<ApplicationId>,
}

impl AiPlanningContext {
    pub fn new(goal: AiGoal, available_application_ids: Vec<ApplicationId>) -> Self {
        Self {
            goal,
            available_application_ids,
        }
    }
}

/// An action AI thinks may help achieve a goal — not execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiActionProposal {
    pub id: AiActionProposalId,
    pub goal_id: AiGoalId,
    pub action_intent_id: ActionIntentId,
    pub target_resource: Option<ResourceRef>,
    pub command_name: String,
    pub explanation: Option<String>,
    pub created_at: String,
}

impl AiActionProposal {
    pub fn propose_application_launch(
        goal: &AiGoal,
        application_id: &ApplicationId,
        explanation: Option<String>,
    ) -> Result<Self, AiPlanningError> {
        let explanation = match explanation {
            None => None,
            Some(value) => normalize_text(value, MAX_EXPLANATION_LEN)?,
        };
        let proposal = Self {
            id: AiActionProposalId::generate(),
            goal_id: goal.id.clone(),
            action_intent_id: ActionIntentId::new(LAUNCH_APPLICATION_INTENT)?,
            target_resource: Some(ResourceRef::new(
                ResourceKind::Application,
                ResourceId::new(application_id.as_str())?,
            )),
            command_name: LAUNCH_APPLICATION_COMMAND.into(),
            explanation,
            created_at: Utc::now().to_rfc3339(),
        };
        proposal.validate()?;
        Ok(proposal)
    }

    pub fn validate(&self) -> Result<(), AiPlanningError> {
        if self.command_name.trim().is_empty() {
            return Err(AiPlanningError::EmptyCommandName);
        }
        if let Some(explanation) = &self.explanation {
            if explanation.len() > MAX_EXPLANATION_LEN {
                return Err(AiPlanningError::ExplanationTooLong);
            }
        }
        if self.command_name == LAUNCH_APPLICATION_COMMAND {
            let Some(target) = &self.target_resource else {
                return Err(AiPlanningError::MissingTarget);
            };
            if target.kind != ResourceKind::Application {
                return Err(AiPlanningError::InvalidTargetKind);
            }
        }
        Ok(())
    }

    pub fn application_id(&self) -> Result<ApplicationId, AiPlanningError> {
        let Some(target) = &self.target_resource else {
            return Err(AiPlanningError::MissingTarget);
        };
        if target.kind != ResourceKind::Application {
            return Err(AiPlanningError::InvalidTargetKind);
        }
        Ok(ApplicationId::new(target.id.as_str())?)
    }

    /// Converts this proposal into a governed [`AiActionRequest`] (still not execution).
    pub fn to_action_request(
        &self,
        requesting_actor_id: &ActorId,
    ) -> Result<AiActionRequest, AiPlanningError> {
        self.validate()?;
        let application_id = self.application_id()?;
        let mut request = AiActionRequest::propose_application_launch(
            requesting_actor_id.as_str(),
            &application_id,
            self.explanation.clone(),
        )?;
        request.goal_id = Some(self.goal_id.to_string());
        request.proposal_id = Some(self.id.to_string());
        Ok(request)
    }
}

/// Result of planning against a goal — proposals only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPlan {
    pub goal: AiGoal,
    pub proposals: Vec<AiActionProposal>,
}

/// Authority outcome after submitting a proposal (no silent retries).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiProposalAuthorityOutcome {
    Allowed,
    Denied { reason: String },
    ApprovalRequired {
        reason: String,
        approval_request_id: String,
    },
}

/// One proposal after the governance path has evaluated it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiProposalSubmission {
    pub proposal: AiActionProposal,
    pub request: AiActionRequest,
    pub outcome: AiProposalAuthorityOutcome,
}

/// Full diagnostic: plan + optional governed submissions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPlanSubmissionResult {
    pub plan: AiPlan,
    pub submissions: Vec<AiProposalSubmission>,
}

fn normalize_text(value: String, max_len: usize) -> Result<Option<String>, AiPlanningError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if trimmed.len() > max_len {
        return Err(if max_len == MAX_GOAL_LEN {
            AiPlanningError::GoalStatementTooLong
        } else {
            AiPlanningError::ExplanationTooLong
        });
    }
    Ok(Some(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_describes_intent_not_commands() {
        let goal = AiGoal::new("Prepare my workspace", "diagnostic-ai").unwrap();
        assert_eq!(goal.statement, "Prepare my workspace");
        assert!(!goal.statement.contains("LaunchApplication"));
    }

    #[test]
    fn proposal_converts_to_action_request_without_executing() {
        let goal = AiGoal::new("Prepare my workspace", "ai-1").unwrap();
        let app = ApplicationId::new("app-1").unwrap();
        let proposal = AiActionProposal::propose_application_launch(
            &goal,
            &app,
            Some("Open Notepad for notes".into()),
        )
        .unwrap();

        let request = proposal
            .to_action_request(&goal.requesting_actor_id)
            .unwrap();
        assert_eq!(request.command_name, "LaunchApplication");
        assert_eq!(request.goal_id.as_deref(), Some(goal.id.as_str()));
        assert_eq!(request.proposal_id.as_deref(), Some(proposal.id.as_str()));
    }
}
