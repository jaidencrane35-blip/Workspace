//! Workspace Intent Model — durable work context (Phase 4 Batch 5).
//!
//! Intent describes the user's work. It is permission-neutral and non-executable.
//! Distinct from ephemeral planning goals (`AiGoal`) and assistant workflows.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{ProjectId, TaskId, WorkGoalId, WorkspaceId};

/// Intent-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceIntentError {
    #[error("project name must not be empty")]
    EmptyProjectName,

    #[error("task title must not be empty")]
    EmptyTaskTitle,

    #[error("goal description must not be empty")]
    EmptyGoalDescription,

    #[error("project not found")]
    ProjectNotFound,

    #[error("task not found")]
    TaskNotFound,

    #[error("work goal not found")]
    GoalNotFound,

    #[error("invalid status: {0}")]
    InvalidStatus(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

impl ProjectStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceIntentError> {
        match value {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceIntentError::InvalidStatus(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Blocked => "blocked",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceIntentError> {
        match value {
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "blocked" => Ok(Self::Blocked),
            "done" => Ok(Self::Done),
            "cancelled" => Ok(Self::Cancelled),
            other => Err(WorkspaceIntentError::InvalidStatus(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl TaskPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceIntentError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            other => Err(WorkspaceIntentError::InvalidStatus(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkGoalStatus {
    Active,
    Achieved,
    Abandoned,
}

impl WorkGoalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Achieved => "achieved",
            Self::Abandoned => "abandoned",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceIntentError> {
        match value {
            "active" => Ok(Self::Active),
            "achieved" => Ok(Self::Achieved),
            "abandoned" => Ok(Self::Abandoned),
            other => Err(WorkspaceIntentError::InvalidStatus(other.into())),
        }
    }
}

/// Durable container for meaningful user work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub metadata: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl Project {
    pub fn new(
        workspace_id: impl Into<String>,
        name: impl Into<String>,
        description: Option<String>,
        metadata: Option<String>,
    ) -> Result<Self, WorkspaceIntentError> {
        let name = normalize_required(name.into(), WorkspaceIntentError::EmptyProjectName)?;
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: ProjectId::generate(),
            workspace_id: WorkspaceId::new(workspace_id)?,
            name,
            description: normalize_optional(description),
            status: ProjectStatus::Active,
            metadata: normalize_optional(metadata),
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        })
    }

    pub fn with_status(mut self, status: ProjectStatus) -> Self {
        self.status = status;
        self.touch();
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

/// Specific piece of work inside a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub title: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl Task {
    pub fn new(
        project_id: impl Into<String>,
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        priority: TaskPriority,
    ) -> Result<Self, WorkspaceIntentError> {
        let title = normalize_required(title.into(), WorkspaceIntentError::EmptyTaskTitle)?;
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: TaskId::generate(),
            project_id: ProjectId::new(project_id)?,
            workspace_id: WorkspaceId::new(workspace_id)?,
            title,
            status: TaskStatus::Todo,
            priority,
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        })
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self.touch();
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

/// Desired outcome driving planning — durable intent, not an `AiGoal` proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkGoal {
    pub id: WorkGoalId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub description: String,
    pub status: WorkGoalStatus,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl WorkGoal {
    pub fn new(
        workspace_id: impl Into<String>,
        description: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<Self, WorkspaceIntentError> {
        let description =
            normalize_required(description.into(), WorkspaceIntentError::EmptyGoalDescription)?;
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: WorkGoalId::generate(),
            workspace_id: WorkspaceId::new(workspace_id)?,
            project_id: project_id.map(ProjectId::new).transpose()?,
            task_id: task_id.map(TaskId::new).transpose()?,
            description,
            status: WorkGoalStatus::Active,
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        })
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

/// Current work state for a workspace (active project/task + pending notes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowContext {
    pub workspace_id: WorkspaceId,
    pub active_project_id: Option<ProjectId>,
    pub active_task_id: Option<TaskId>,
    pub related_plan_ids: Vec<String>,
    pub pending_decision_notes: Vec<String>,
    pub blocker_notes: Vec<String>,
    pub updated_at: String,
}

impl WorkflowContext {
    pub fn empty(workspace_id: impl Into<String>) -> Result<Self, WorkspaceIntentError> {
        Ok(Self {
            workspace_id: WorkspaceId::new(workspace_id)?,
            active_project_id: None,
            active_task_id: None,
            related_plan_ids: Vec::new(),
            pending_decision_notes: Vec::new(),
            blocker_notes: Vec::new(),
            updated_at: Utc::now().to_rfc3339(),
        })
    }

    pub fn with_active_project(mut self, project_id: Option<ProjectId>) -> Self {
        self.active_project_id = project_id;
        self.touch();
        self
    }

    pub fn with_active_task(mut self, task_id: Option<TaskId>) -> Self {
        self.active_task_id = task_id;
        self.touch();
        self
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

fn normalize_required(
    value: String,
    empty_error: WorkspaceIntentError,
) -> Result<String, WorkspaceIntentError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(empty_error);
    }
    Ok(trimmed.to_string())
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|v| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_and_task_are_non_executable_intent() {
        let project = Project::new("ws-1", "Workspace AI", Some("Core platform".into()), None)
            .unwrap();
        assert_eq!(project.status, ProjectStatus::Active);
        let task = Task::new(
            project.id.to_string(),
            "ws-1",
            "Build Workspace Intelligence Layer",
            TaskPriority::High,
        )
        .unwrap();
        assert_eq!(task.status, TaskStatus::Todo);
    }
}
