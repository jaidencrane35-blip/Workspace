//! Governed Trigger Evaluation foundation (Phase 4 Batch 7).
//!
//! Trigger events and intent proposals are informational.
//! Evaluation notices relevance; it never executes or grants authority.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::automation_contract::{
    AutomationContract, AutomationIntentDefinition, AutomationTriggerKind,
};
use crate::errors::DomainError;
use crate::ids::{
    AutomationContractId, AutomationIntentProposalId, ProjectId, TaskId, TriggerEventId,
    WorkspaceId,
};

/// Trigger-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AutomationTriggerError {
    #[error("trigger event context must not be empty when provided as blank")]
    EmptyContext,

    #[error("invalid trigger event type: {0}")]
    InvalidEventType(String),

    #[error("invalid intent proposal status: {0}")]
    InvalidProposalStatus(String),

    #[error("trigger event not found")]
    EventNotFound,

    #[error("intent proposal not found")]
    ProposalNotFound,

    #[error("intent proposal is not pending review")]
    ProposalNotPending,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Informational event that may make a contract relevant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerEventType {
    ApplicationOpened,
    WorkspaceChanged,
    ProjectContextChanged,
    TaskStateChanged,
    UserRequestedEvaluation,
    ManualEvaluationRequested,
}

impl TriggerEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplicationOpened => "application_opened",
            Self::WorkspaceChanged => "workspace_changed",
            Self::ProjectContextChanged => "project_context_changed",
            Self::TaskStateChanged => "task_state_changed",
            Self::UserRequestedEvaluation => "user_requested_evaluation",
            Self::ManualEvaluationRequested => "manual_evaluation_requested",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationTriggerError> {
        match value {
            "application_opened" => Ok(Self::ApplicationOpened),
            "workspace_changed" => Ok(Self::WorkspaceChanged),
            "project_context_changed" => Ok(Self::ProjectContextChanged),
            "task_state_changed" => Ok(Self::TaskStateChanged),
            "user_requested_evaluation" => Ok(Self::UserRequestedEvaluation),
            "manual_evaluation_requested" => Ok(Self::ManualEvaluationRequested),
            other => Err(AutomationTriggerError::InvalidEventType(other.into())),
        }
    }

    /// Whether this event type is compatible with a contract trigger kind.
    pub fn matches_trigger_kind(self, kind: AutomationTriggerKind) -> bool {
        match kind {
            AutomationTriggerKind::Manual => matches!(
                self,
                Self::ManualEvaluationRequested | Self::UserRequestedEvaluation
            ),
            AutomationTriggerKind::Event => matches!(
                self,
                Self::ApplicationOpened
                    | Self::WorkspaceChanged
                    | Self::ProjectContextChanged
                    | Self::TaskStateChanged
                    | Self::ManualEvaluationRequested
                    | Self::UserRequestedEvaluation
            ),
            AutomationTriggerKind::Pattern => matches!(
                self,
                Self::ApplicationOpened
                    | Self::WorkspaceChanged
                    | Self::ProjectContextChanged
                    | Self::TaskStateChanged
                    | Self::ManualEvaluationRequested
                    | Self::UserRequestedEvaluation
            ),
            // Scheduled definitions are never auto-fired in Batch 7.
            // Manual evaluation may still surface them for review.
            AutomationTriggerKind::Scheduled => matches!(
                self,
                Self::ManualEvaluationRequested | Self::UserRequestedEvaluation
            ),
        }
    }
}

/// Something happened that may be relevant to an Automation Contract.
///
/// Informational only — `authority_effect` is always none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub id: TriggerEventId,
    pub workspace_id: WorkspaceId,
    pub event_type: TriggerEventType,
    pub source: String,
    pub context: String,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub actor_id: String,
    pub actor_type: String,
    pub created_at: String,
    pub authority_effect: String,
}

impl TriggerEvent {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(
        workspace_id: impl Into<String>,
        event_type: TriggerEventType,
        source: impl Into<String>,
        context: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
        actor_id: impl Into<String>,
        actor_type: impl Into<String>,
    ) -> Result<Self, AutomationTriggerError> {
        let context = context.into();
        let context = if context.trim().is_empty() {
            "{}".into()
        } else {
            context.trim().to_string()
        };
        let source = {
            let s = source.into();
            let t = s.trim();
            if t.is_empty() {
                "system".into()
            } else {
                t.to_string()
            }
        };
        Ok(Self {
            id: TriggerEventId::generate(),
            workspace_id: WorkspaceId::new(workspace_id)?,
            event_type,
            source,
            context,
            project_id: project_id
                .map(ProjectId::new)
                .transpose()
                .map_err(AutomationTriggerError::Domain)?,
            task_id: task_id
                .map(TaskId::new)
                .transpose()
                .map_err(AutomationTriggerError::Domain)?,
            actor_id: actor_id.into().trim().to_string(),
            actor_type: actor_type.into().trim().to_string(),
            created_at: Utc::now().to_rfc3339(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Proposal status — never execution authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationIntentProposalStatus {
    PendingReview,
    Accepted,
    Rejected,
    Expired,
}

impl AutomationIntentProposalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationTriggerError> {
        match value {
            "pending_review" => Ok(Self::PendingReview),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            other => Err(AutomationTriggerError::InvalidProposalStatus(other.into())),
        }
    }
}

/// A governed automation contract appears relevant.
///
/// Not an execution grant. Accepting still requires Command Pipeline → Gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationIntentProposal {
    pub id: AutomationIntentProposalId,
    pub contract_id: AutomationContractId,
    pub workspace_id: WorkspaceId,
    pub project_id: ProjectId,
    pub task_id: Option<TaskId>,
    pub trigger_event_id: TriggerEventId,
    pub intent_definition: AutomationIntentDefinition,
    pub required_capabilities: Vec<String>,
    pub status: AutomationIntentProposalStatus,
    pub explanation: String,
    pub definition_fingerprint: String,
    pub created_at: String,
    pub updated_at: String,
}

impl AutomationIntentProposal {
    pub fn from_match(
        contract: &AutomationContract,
        event: &TriggerEvent,
        explanation: impl Into<String>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: AutomationIntentProposalId::generate(),
            contract_id: contract.id.clone(),
            workspace_id: contract.workspace_id.clone(),
            project_id: contract.project_id.clone(),
            task_id: contract.task_id.clone(),
            trigger_event_id: event.id.clone(),
            intent_definition: contract.intent_definition.clone(),
            required_capabilities: contract.required_capabilities.clone(),
            status: AutomationIntentProposalStatus::PendingReview,
            explanation: explanation.into(),
            definition_fingerprint: contract.definition_fingerprint(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn accept(&mut self) -> Result<(), AutomationTriggerError> {
        if self.status != AutomationIntentProposalStatus::PendingReview {
            return Err(AutomationTriggerError::ProposalNotPending);
        }
        self.status = AutomationIntentProposalStatus::Accepted;
        self.updated_at = Utc::now().to_rfc3339();
        Ok(())
    }

    pub fn reject(&mut self) -> Result<(), AutomationTriggerError> {
        if self.status != AutomationIntentProposalStatus::PendingReview {
            return Err(AutomationTriggerError::ProposalNotPending);
        }
        self.status = AutomationIntentProposalStatus::Rejected;
        self.updated_at = Utc::now().to_rfc3339();
        Ok(())
    }
}

/// Explainable rejection of a candidate contract during evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerRejection {
    pub contract_id: String,
    pub contract_name: String,
    pub reason: String,
}

/// Result of evaluating one trigger event against workspace contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerEvaluationResult {
    pub trigger_event: TriggerEvent,
    pub proposals: Vec<AutomationIntentProposal>,
    pub rejections: Vec<TriggerRejection>,
    pub authority_effect: String,
}

impl TriggerEvaluationResult {
    pub fn new(
        trigger_event: TriggerEvent,
        proposals: Vec<AutomationIntentProposal>,
        rejections: Vec<TriggerRejection>,
    ) -> Self {
        Self {
            trigger_event,
            proposals,
            rejections,
            authority_effect: TriggerEvent::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Read-only summary for Workspace Intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationIntentProposalSummary {
    pub id: String,
    pub contract_id: String,
    pub status: String,
    pub explanation: String,
    pub intent_statement: String,
    pub trigger_event_id: String,
}

impl From<&AutomationIntentProposal> for AutomationIntentProposalSummary {
    fn from(proposal: &AutomationIntentProposal) -> Self {
        Self {
            id: proposal.id.to_string(),
            contract_id: proposal.contract_id.to_string(),
            status: proposal.status.as_str().into(),
            explanation: proposal.explanation.clone(),
            intent_statement: proposal.intent_definition.statement.clone(),
            trigger_event_id: proposal.trigger_event_id.to_string(),
        }
    }
}

/// Read-only rejection summary for intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerRejectionSummary {
    pub contract_id: String,
    pub reason: String,
}
