//! Governed Automation Contract foundation (Phase 4 Batch 6).
//!
//! Durable, inspectable, revocable records of user-approved *future intent*.
//! Non-executable. Approval of a contract is not execution authority.
//! Triggers are definitions only — no workers, no automatic runs.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{AutomationContractId, ProjectId, TaskId, WorkspaceId};

/// Contract-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AutomationContractError {
    #[error("automation contract name must not be empty")]
    EmptyName,

    #[error("intent definition statement must not be empty")]
    EmptyIntentStatement,

    #[error("project_id is required for an automation contract")]
    MissingProject,

    #[error("automation contract not found")]
    NotFound,

    #[error("invalid automation contract status: {0}")]
    InvalidStatus(String),

    #[error("invalid automation contract approval state: {0}")]
    InvalidApprovalState(String),

    #[error("invalid automation trigger kind: {0}")]
    InvalidTriggerKind(String),

    #[error("invalid automation contract scope: {0}")]
    InvalidScope(String),

    #[error("automation contract is not active for future intent materialization")]
    NotActiveForIntent,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Lifecycle status of the contract record (orthogonal to approval_state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationContractStatus {
    Draft,
    PendingApproval,
    Approved,
    Paused,
    Revoked,
    Completed,
}

impl AutomationContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::PendingApproval => "pending_approval",
            Self::Approved => "approved",
            Self::Paused => "paused",
            Self::Revoked => "revoked",
            Self::Completed => "completed",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationContractError> {
        match value {
            "draft" => Ok(Self::Draft),
            "pending_approval" => Ok(Self::PendingApproval),
            "approved" => Ok(Self::Approved),
            "paused" => Ok(Self::Paused),
            "revoked" => Ok(Self::Revoked),
            "completed" => Ok(Self::Completed),
            other => Err(AutomationContractError::InvalidStatus(other.into())),
        }
    }
}

/// Whether the user has approved the *definition* (not execution rights).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationContractApprovalState {
    NotApproved,
    Pending,
    Approved,
    Revoked,
}

impl AutomationContractApprovalState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApproved => "not_approved",
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Revoked => "revoked",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationContractError> {
        match value {
            "not_approved" => Ok(Self::NotApproved),
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "revoked" => Ok(Self::Revoked),
            other => Err(AutomationContractError::InvalidApprovalState(other.into())),
        }
    }
}

/// Trigger kinds — definitions only in Batch 6 (no runners).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTriggerKind {
    Manual,
    Scheduled,
    Event,
    Pattern,
}

impl AutomationTriggerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Scheduled => "scheduled",
            Self::Event => "event",
            Self::Pattern => "pattern",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationContractError> {
        match value {
            "manual" => Ok(Self::Manual),
            "scheduled" => Ok(Self::Scheduled),
            "event" => Ok(Self::Event),
            "pattern" => Ok(Self::Pattern),
            other => Err(AutomationContractError::InvalidTriggerKind(other.into())),
        }
    }
}

/// Non-executable trigger definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationTriggerDefinition {
    pub kind: AutomationTriggerKind,
    /// Human-readable or structured definition (cron text, event name, pattern note).
    /// Never interpreted as an executable command in this batch.
    pub definition: String,
}

impl AutomationTriggerDefinition {
    pub fn manual() -> Self {
        Self {
            kind: AutomationTriggerKind::Manual,
            definition: "Manual trigger — user initiates when ready.".into(),
        }
    }

    pub fn new(
        kind: AutomationTriggerKind,
        definition: impl Into<String>,
    ) -> Result<Self, AutomationContractError> {
        let definition = normalize_optional_text(Some(definition.into())).unwrap_or_default();
        Ok(Self { kind, definition })
    }
}

/// Future intent template — describes what to *request*, not what to execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationIntentDefinition {
    /// Example: "When this workspace condition occurs, request opening my development environment."
    pub statement: String,
}

impl AutomationIntentDefinition {
    pub fn new(statement: impl Into<String>) -> Result<Self, AutomationContractError> {
        let statement =
            normalize_required(statement.into(), AutomationContractError::EmptyIntentStatement)?;
        Ok(Self { statement })
    }
}

/// Scope of the contract relative to work context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationContractScope {
    Project,
    Task,
}

impl AutomationContractScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Task => "task",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AutomationContractError> {
        match value {
            "project" => Ok(Self::Project),
            "task" => Ok(Self::Task),
            other => Err(AutomationContractError::InvalidScope(other.into())),
        }
    }
}

/// Durable, non-executable automation contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContract {
    pub id: AutomationContractId,
    pub workspace_id: WorkspaceId,
    pub project_id: ProjectId,
    pub task_id: Option<TaskId>,
    pub name: String,
    pub description: Option<String>,
    pub status: AutomationContractStatus,
    pub trigger_definition: AutomationTriggerDefinition,
    pub intent_definition: AutomationIntentDefinition,
    pub scope: AutomationContractScope,
    pub required_capabilities: Vec<String>,
    pub approval_state: AutomationContractApprovalState,
    pub created_by_actor: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl AutomationContract {
    pub fn new(
        workspace_id: impl Into<String>,
        project_id: impl Into<String>,
        task_id: Option<String>,
        name: impl Into<String>,
        description: Option<String>,
        trigger_definition: AutomationTriggerDefinition,
        intent_definition: AutomationIntentDefinition,
        required_capabilities: Vec<String>,
        created_by_actor: impl Into<String>,
    ) -> Result<Self, AutomationContractError> {
        let name = normalize_required(name.into(), AutomationContractError::EmptyName)?;
        let project_id_raw = project_id.into();
        if project_id_raw.trim().is_empty() {
            return Err(AutomationContractError::MissingProject);
        }
        let task = task_id
            .map(TaskId::new)
            .transpose()
            .map_err(AutomationContractError::Domain)?;
        let scope = if task.is_some() {
            AutomationContractScope::Task
        } else {
            AutomationContractScope::Project
        };
        let now = Utc::now().to_rfc3339();
        let caps = required_capabilities
            .into_iter()
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        Ok(Self {
            id: AutomationContractId::generate(),
            workspace_id: WorkspaceId::new(workspace_id)?,
            project_id: ProjectId::new(project_id_raw)?,
            task_id: task,
            name,
            description: normalize_optional_text(description),
            status: AutomationContractStatus::Draft,
            trigger_definition,
            intent_definition,
            scope,
            required_capabilities: caps,
            approval_state: AutomationContractApprovalState::NotApproved,
            created_by_actor: created_by_actor.into().trim().to_string(),
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        })
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }

    /// True only when both status and approval say the definition is live.
    /// Still does **not** authorize execution.
    pub fn is_active_definition(&self) -> bool {
        !self.deleted
            && self.status == AutomationContractStatus::Approved
            && self.approval_state == AutomationContractApprovalState::Approved
    }

    pub fn request_approval(&mut self) -> Result<(), AutomationContractError> {
        if self.deleted
            || matches!(
                self.status,
                AutomationContractStatus::Revoked | AutomationContractStatus::Completed
            )
        {
            return Err(AutomationContractError::NotActiveForIntent);
        }
        self.status = AutomationContractStatus::PendingApproval;
        self.approval_state = AutomationContractApprovalState::Pending;
        self.touch();
        Ok(())
    }

    pub fn approve_definition(&mut self) -> Result<(), AutomationContractError> {
        if self.deleted || self.status == AutomationContractStatus::Revoked {
            return Err(AutomationContractError::NotActiveForIntent);
        }
        self.status = AutomationContractStatus::Approved;
        self.approval_state = AutomationContractApprovalState::Approved;
        self.touch();
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), AutomationContractError> {
        if !self.is_active_definition() {
            return Err(AutomationContractError::NotActiveForIntent);
        }
        self.status = AutomationContractStatus::Paused;
        self.touch();
        Ok(())
    }

    pub fn revoke(&mut self) -> Result<(), AutomationContractError> {
        if self.deleted || self.status == AutomationContractStatus::Completed {
            return Err(AutomationContractError::NotActiveForIntent);
        }
        self.status = AutomationContractStatus::Revoked;
        self.approval_state = AutomationContractApprovalState::Revoked;
        self.touch();
        Ok(())
    }
}

/// Read-only summary for Workspace Intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractSummary {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub status: String,
    pub approval_state: String,
    pub intent_statement: String,
    pub required_capabilities: Vec<String>,
    pub created_by_actor: String,
}

impl From<&AutomationContract> for AutomationContractSummary {
    fn from(contract: &AutomationContract) -> Self {
        Self {
            id: contract.id.to_string(),
            name: contract.name.clone(),
            project_id: contract.project_id.to_string(),
            task_id: contract.task_id.as_ref().map(|id| id.to_string()),
            status: contract.status.as_str().into(),
            approval_state: contract.approval_state.as_str().into(),
            intent_statement: contract.intent_definition.statement.clone(),
            required_capabilities: contract.required_capabilities.clone(),
            created_by_actor: contract.created_by_actor.clone(),
        }
    }
}

/// Integration boundary result: a future Intent request template.
///
/// Does not authorize or execute. Caller must still enter Command Pipeline →
/// Permission Gateway before any action runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractIntentRequest {
    pub contract_id: String,
    pub workspace_id: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub intent_statement: String,
    pub required_capabilities: Vec<String>,
    pub governance_note: String,
}

impl AutomationContractIntentRequest {
    pub const GOVERNANCE_NOTE: &'static str =
        "Contract approval authorizes the definition only. \
         Materialized intent must enter Command Pipeline → Permission Gateway. \
         No execution authority is granted by this contract.";

    pub fn from_active_contract(
        contract: &AutomationContract,
    ) -> Result<Self, AutomationContractError> {
        if !contract.is_active_definition() {
            return Err(AutomationContractError::NotActiveForIntent);
        }
        Ok(Self {
            contract_id: contract.id.to_string(),
            workspace_id: contract.workspace_id.to_string(),
            project_id: contract.project_id.to_string(),
            task_id: contract.task_id.as_ref().map(|id| id.to_string()),
            intent_statement: contract.intent_definition.statement.clone(),
            required_capabilities: contract.required_capabilities.clone(),
            governance_note: Self::GOVERNANCE_NOTE.into(),
        })
    }
}

fn normalize_required(
    value: String,
    empty: AutomationContractError,
) -> Result<String, AutomationContractError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|v| {
        let t = v.trim();
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_does_not_imply_active_when_paused() {
        let mut contract = AutomationContract::new(
            "ws-1",
            "proj-1",
            None,
            "Dev env",
            None,
            AutomationTriggerDefinition::manual(),
            AutomationIntentDefinition::new(
                "When ready, request opening my development environment.",
            )
            .unwrap(),
            vec!["application.launch".into()],
            "local-user",
        )
        .unwrap();
        contract.approve_definition().unwrap();
        assert!(contract.is_active_definition());
        contract.pause().unwrap();
        assert!(!contract.is_active_definition());
        assert_eq!(
            contract.approval_state,
            AutomationContractApprovalState::Approved
        );
    }

    #[test]
    fn revoked_contract_cannot_materialize_intent() {
        let mut contract = AutomationContract::new(
            "ws-1",
            "proj-1",
            None,
            "Dev env",
            None,
            AutomationTriggerDefinition::manual(),
            AutomationIntentDefinition::new("Request prepare coding workspace.").unwrap(),
            vec![],
            "local-user",
        )
        .unwrap();
        contract.approve_definition().unwrap();
        contract.revoke().unwrap();
        assert!(AutomationContractIntentRequest::from_active_contract(&contract).is_err());
    }
}
