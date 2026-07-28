//! Workspace Task Graph — durable model of work (Phase 5).
//!
//! Canonical representation of workspace work items and relationships.
//! Informational only — never executes, authorizes, or bypasses the Planner.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{WorkspaceId, WorkspaceTaskId};

/// Task Graph validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TaskGraphError {
    #[error("task title must not be empty")]
    EmptyTitle,

    #[error("task not found")]
    TaskNotFound,

    #[error("relationship not found")]
    RelationshipNotFound,

    #[error("invalid task status: {0}")]
    InvalidStatus(String),

    #[error("invalid task status transition: {from} -> {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("invalid task priority: {0}")]
    InvalidPriority(String),

    #[error("invalid relationship kind: {0}")]
    InvalidRelationship(String),

    #[error("circular dependency detected")]
    CircularDependency,

    #[error("self-relationship is not allowed")]
    SelfRelationship,

    #[error("task graph cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Graph lifecycle status (distinct from intent `TaskStatus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTaskStatus {
    Proposed,
    Planned,
    Waiting,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

impl WorkspaceTaskStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Planned => "planned",
            Self::Waiting => "waiting",
            Self::InProgress => "in_progress",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TaskGraphError> {
        match value {
            "proposed" => Ok(Self::Proposed),
            "planned" => Ok(Self::Planned),
            "waiting" => Ok(Self::Waiting),
            "in_progress" => Ok(Self::InProgress),
            "blocked" => Ok(Self::Blocked),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            other => Err(TaskGraphError::InvalidStatus(other.into())),
        }
    }

    pub fn is_open(self) -> bool {
        !matches!(self, Self::Completed | Self::Cancelled)
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled)
    }

    pub fn allows_transition(self, next: Self) -> bool {
        self != next && self.is_open()
    }
}

/// Graph priority (namespaced to avoid collision with intent `TaskPriority`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl WorkspaceTaskPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TaskGraphError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            other => Err(TaskGraphError::InvalidPriority(other.into())),
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Critical => 4,
        }
    }
}

/// Directed relationship kinds between task nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskRelationshipKind {
    DependsOn,
    Blocks,
    RelatedTo,
    ChildOf,
    ParentOf,
}

impl TaskRelationshipKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DependsOn => "depends_on",
            Self::Blocks => "blocks",
            Self::RelatedTo => "related_to",
            Self::ChildOf => "child_of",
            Self::ParentOf => "parent_of",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TaskGraphError> {
        match value {
            "depends_on" => Ok(Self::DependsOn),
            "blocks" => Ok(Self::Blocks),
            "related_to" => Ok(Self::RelatedTo),
            "child_of" => Ok(Self::ChildOf),
            "parent_of" => Ok(Self::ParentOf),
            other => Err(TaskGraphError::InvalidRelationship(other.into())),
        }
    }

    /// Whether this kind participates in cycle detection (ordering edges).
    pub fn is_ordering(self) -> bool {
        matches!(
            self,
            Self::DependsOn | Self::Blocks | Self::ChildOf | Self::ParentOf
        )
    }
}

/// Optional structured metadata on a graph task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskMetadata {
    pub labels: Vec<String>,
    pub notes: Option<String>,
    pub attributes_json: Option<String>,
}

/// Canonical workspace work item in the Task Graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTask {
    pub id: WorkspaceTaskId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<String>,
    pub title: String,
    pub status: WorkspaceTaskStatus,
    pub priority: WorkspaceTaskPriority,
    pub metadata: TaskMetadata,
    /// Optional link to legacy intent `work_tasks` row (projection bridge).
    pub source_intent_task_id: Option<String>,
    pub work_goal_id: Option<String>,
    pub progress_percent: u8,
    pub explanation: String,
    pub created_at: String,
    pub updated_at: String,
    pub authority_effect: String,
}

impl WorkspaceTask {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        project_id: Option<String>,
        priority: WorkspaceTaskPriority,
    ) -> Result<Self, TaskGraphError> {
        let title = title.into().trim().to_string();
        if title.is_empty() {
            return Err(TaskGraphError::EmptyTitle);
        }
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: WorkspaceTaskId::generate(),
            workspace_id: WorkspaceId::new(workspace_id)?,
            project_id,
            title,
            status: WorkspaceTaskStatus::Proposed,
            priority,
            metadata: TaskMetadata::default(),
            source_intent_task_id: None,
            work_goal_id: None,
            progress_percent: 0,
            explanation: "Newly proposed work item.".into(),
            created_at: now.clone(),
            updated_at: now,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }

    pub fn transition_status(
        mut self,
        status: WorkspaceTaskStatus,
        explanation: impl Into<String>,
    ) -> Result<Self, TaskGraphError> {
        if !self.status.allows_transition(status) {
            return Err(TaskGraphError::InvalidStatusTransition {
                from: self.status.as_str().into(),
                to: status.as_str().into(),
            });
        }
        self.status = status;
        self.explanation = explanation.into();
        if status == WorkspaceTaskStatus::Completed {
            self.progress_percent = 100;
        }
        self.touch();
        Ok(self)
    }
}

/// Graph node view — addressable work unit (same identity as WorkspaceTask).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskNode {
    pub task: WorkspaceTask,
    pub blocker_ids: Vec<String>,
    pub dependency_ids: Vec<String>,
    pub child_ids: Vec<String>,
    pub parent_ids: Vec<String>,
    pub waiting_reason: Option<String>,
}

/// Persisted directed relationship between two tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskRelationship {
    pub id: String,
    pub workspace_id: String,
    pub from_task_id: String,
    pub to_task_id: String,
    pub kind: TaskRelationshipKind,
    pub created_at: String,
    pub authority_effect: String,
}

impl TaskRelationship {
    pub fn new(
        workspace_id: impl Into<String>,
        from_task_id: impl Into<String>,
        to_task_id: impl Into<String>,
        kind: TaskRelationshipKind,
    ) -> Result<Self, TaskGraphError> {
        let from_task_id = from_task_id.into();
        let to_task_id = to_task_id.into();
        if from_task_id == to_task_id {
            return Err(TaskGraphError::SelfRelationship);
        }
        Ok(Self {
            id: format!(
                "rel:{}:{}:{}",
                kind.as_str(),
                from_task_id,
                to_task_id
            ),
            workspace_id: workspace_id.into(),
            from_task_id,
            to_task_id,
            kind,
            created_at: Utc::now().to_rfc3339(),
            authority_effect: WorkspaceTask::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Explicit dependency edge (depends_on convenience).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDependency {
    pub dependent_id: String,
    pub prerequisite_id: String,
    pub relationship_id: String,
}

/// Compact non-actionable terminal Task Graph evidence for projection consumers.
///
/// History is read-only continuity — never editable, never merged into actionable
/// `nodes` / `top_nodes`, and never a second lifecycle authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskHistoryEntry {
    pub task_id: String,
    pub title: String,
    /// Terminal status (`completed` / `cancelled`).
    pub status: String,
    pub progress_percent: u8,
    pub explanation: String,
    pub updated_at: String,
    /// Always `true` for projected history entries.
    pub terminal: bool,
    /// Always `false` — terminal history never joins active work.
    pub actionable: bool,
    pub authority_effect: String,
}

impl TaskHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_node(node: &TaskNode) -> Option<Self> {
        if !node.task.status.is_terminal() {
            return None;
        }
        Some(Self {
            task_id: node.task.id.to_string(),
            title: node.task.title.clone(),
            status: node.task.status.as_str().into(),
            progress_percent: node.task.progress_percent,
            explanation: node.task.explanation.clone(),
            updated_at: node.task.updated_at.clone(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && matches!(self.status.as_str(), "completed" | "cancelled")
    }
}

/// Full Task Graph snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskGraph {
    pub workspace_id: String,
    pub generated_at: String,
    /// Actionable (open) work nodes only — terminals project into `history`.
    pub nodes: Vec<TaskNode>,
    pub relationships: Vec<TaskRelationship>,
    pub active_count: usize,
    pub blocked_count: usize,
    pub waiting_count: usize,
    pub completed_count: usize,
    pub progress_percent: u8,
    /// Terminal task evidence (completed/cancelled) — never actionable.
    #[serde(default)]
    pub history: Vec<TaskHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
    pub summary: String,
    pub integrity_ok: bool,
    pub integrity_notes: Vec<String>,
    pub authority_effect: String,
}

impl TaskGraph {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_parts(
        workspace_id: impl Into<String>,
        tasks: Vec<WorkspaceTask>,
        relationships: Vec<TaskRelationship>,
        integrity_ok: bool,
        integrity_notes: Vec<String>,
    ) -> Self {
        let workspace_id = workspace_id.into();
        let all_nodes = enrich_nodes(&tasks, &relationships);
        let active_count = tasks
            .iter()
            .filter(|t| matches!(t.status, WorkspaceTaskStatus::InProgress | WorkspaceTaskStatus::Planned))
            .count();
        let blocked_count = tasks
            .iter()
            .filter(|t| t.status == WorkspaceTaskStatus::Blocked)
            .count();
        let waiting_count = tasks
            .iter()
            .filter(|t| t.status == WorkspaceTaskStatus::Waiting)
            .count();
        let completed_count = tasks
            .iter()
            .filter(|t| t.status == WorkspaceTaskStatus::Completed)
            .count();
        let progress_percent = if tasks.is_empty() {
            0
        } else {
            let sum: u32 = tasks.iter().map(|t| u32::from(t.progress_percent)).sum();
            (sum / tasks.len() as u32).min(100) as u8
        };
        // Project terminal evidence before filtering — nodes become actionable-only.
        let mut history: Vec<TaskHistoryEntry> = all_nodes
            .iter()
            .filter_map(TaskHistoryEntry::from_node)
            .collect();
        history.sort_by(|a, b| {
            b.updated_at
                .cmp(&a.updated_at)
                .then_with(|| a.task_id.cmp(&b.task_id))
        });
        let history_count = history.len();
        let nodes: Vec<TaskNode> = all_nodes
            .into_iter()
            .filter(|n| n.task.status.is_open())
            .collect();
        let summary = format!(
            "Task Graph — {total} task(s): {active} active, {blocked} blocked, {waiting} waiting, {completed} completed ({progress}% overall); {history_count} terminal in history. Informational only.",
            total = tasks.len(),
            active = active_count,
            blocked = blocked_count,
            waiting = waiting_count,
            completed = completed_count,
            progress = progress_percent,
            history_count = history_count,
        );
        Self {
            workspace_id,
            generated_at: Utc::now().to_rfc3339(),
            nodes,
            relationships,
            active_count,
            blocked_count,
            waiting_count,
            completed_count,
            progress_percent,
            history,
            history_count,
            summary,
            integrity_ok,
            integrity_notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn summary_projection(&self, limit: usize) -> TaskGraphSummary {
        let actionable: Vec<TaskNode> = self
            .nodes
            .iter()
            .filter(|n| n.task.status.is_open())
            .cloned()
            .collect();
        let history: Vec<TaskHistoryEntry> = self.history.iter().take(limit).cloned().collect();
        TaskGraphSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            // Surface count reflects still-actionable open work.
            node_count: actionable.len(),
            relationship_count: self.relationships.len(),
            active_count: self.active_count,
            blocked_count: self.blocked_count,
            waiting_count: self.waiting_count,
            completed_count: self.completed_count,
            progress_percent: self.progress_percent,
            top_nodes: actionable.into_iter().take(limit).collect(),
            history,
            history_count: self.history_count,
            summary: self.summary.clone(),
            integrity_ok: self.integrity_ok,
            authority_effect: self.authority_effect.clone(),
        }
    }

    pub fn open_incomplete_nodes(&self) -> Vec<&TaskNode> {
        self.nodes
            .iter()
            .filter(|n| n.task.status.is_open())
            .collect()
    }
}

/// Compact projection for Intelligence / Decision Engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskGraphSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub node_count: usize,
    pub relationship_count: usize,
    pub active_count: usize,
    pub blocked_count: usize,
    pub waiting_count: usize,
    pub completed_count: usize,
    pub progress_percent: u8,
    /// Actionable open work only.
    pub top_nodes: Vec<TaskNode>,
    /// Truncated terminal task evidence (never actionable).
    #[serde(default)]
    pub history: Vec<TaskHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
    pub summary: String,
    pub integrity_ok: bool,
    pub authority_effect: String,
}

impl Default for TaskGraphSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            node_count: 0,
            relationship_count: 0,
            active_count: 0,
            blocked_count: 0,
            waiting_count: 0,
            completed_count: 0,
            progress_percent: 0,
            top_nodes: Vec::new(),
            history: Vec::new(),
            history_count: 0,
            summary: String::new(),
            integrity_ok: true,
            authority_effect: WorkspaceTask::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Detect whether adding `from -> to` of an ordering kind would create a cycle.
pub fn would_create_cycle(
    relationships: &[TaskRelationship],
    from: &str,
    to: &str,
    kind: TaskRelationshipKind,
) -> bool {
    if !kind.is_ordering() {
        return false;
    }
    // Walk successors of `to`; if we reach `from`, adding from→to closes a cycle.
    let mut stack = vec![to.to_string()];
    let mut seen = std::collections::HashSet::new();
    while let Some(current) = stack.pop() {
        if current == from {
            return true;
        }
        if !seen.insert(current.clone()) {
            continue;
        }
        for rel in relationships {
            if !rel.kind.is_ordering() || rel.from_task_id != current {
                continue;
            }
            stack.push(rel.to_task_id.clone());
        }
        // ParentOf is inverse of ChildOf for traversal consistency when only one stored.
        for rel in relationships {
            if rel.kind == TaskRelationshipKind::ChildOf && rel.to_task_id == current {
                stack.push(rel.from_task_id.clone());
            }
            if rel.kind == TaskRelationshipKind::ParentOf && rel.from_task_id == current {
                stack.push(rel.to_task_id.clone());
            }
        }
    }
    false
}

fn enrich_nodes(tasks: &[WorkspaceTask], relationships: &[TaskRelationship]) -> Vec<TaskNode> {
    tasks
        .iter()
        .map(|task| {
            let id = task.id.as_str();
            let mut blocker_ids = Vec::new();
            let mut dependency_ids = Vec::new();
            let mut child_ids = Vec::new();
            let mut parent_ids = Vec::new();

            for rel in relationships {
                match rel.kind {
                    TaskRelationshipKind::DependsOn if rel.from_task_id == id => {
                        dependency_ids.push(rel.to_task_id.clone());
                    }
                    TaskRelationshipKind::Blocks if rel.to_task_id == id => {
                        blocker_ids.push(rel.from_task_id.clone());
                    }
                    TaskRelationshipKind::Blocks if rel.from_task_id == id => {
                        // this task blocks others — not a blocker of self
                    }
                    TaskRelationshipKind::ChildOf if rel.from_task_id == id => {
                        parent_ids.push(rel.to_task_id.clone());
                    }
                    TaskRelationshipKind::ChildOf if rel.to_task_id == id => {
                        child_ids.push(rel.from_task_id.clone());
                    }
                    TaskRelationshipKind::ParentOf if rel.from_task_id == id => {
                        child_ids.push(rel.to_task_id.clone());
                    }
                    TaskRelationshipKind::ParentOf if rel.to_task_id == id => {
                        parent_ids.push(rel.from_task_id.clone());
                    }
                    _ => {}
                }
            }

            let waiting_reason = if task.status == WorkspaceTaskStatus::Waiting {
                if let Some(dep) = dependency_ids.first() {
                    Some(format!(
                        "This task is waiting because task {dep} must complete first."
                    ))
                } else if let Some(blocker) = blocker_ids.first() {
                    Some(format!(
                        "This task is waiting because it is blocked by task {blocker}."
                    ))
                } else {
                    Some(task.explanation.clone())
                }
            } else if task.status == WorkspaceTaskStatus::Blocked {
                if let Some(blocker) = blocker_ids.first() {
                    Some(format!("Blocked by task {blocker}."))
                } else {
                    Some(task.explanation.clone())
                }
            } else {
                None
            };

            TaskNode {
                task: task.clone(),
                blocker_ids,
                dependency_ids,
                child_ids,
                parent_ids,
                waiting_reason,
            }
        })
        .collect()
}

#[cfg(test)]
mod projection_tests {
    use super::*;

    fn task(
        title: &str,
        status: WorkspaceTaskStatus,
        progress: u8,
        explanation: &str,
    ) -> WorkspaceTask {
        let mut t =
            WorkspaceTask::new("ws-1", title, None, WorkspaceTaskPriority::Medium).unwrap();
        if status != WorkspaceTaskStatus::Proposed {
            t = t.transition_status(status, explanation).unwrap();
        }
        t.progress_percent = if status == WorkspaceTaskStatus::Completed {
            100
        } else {
            progress
        };
        t.explanation = explanation.into();
        t
    }

    #[test]
    fn terminal_tasks_absent_from_actionable_present_in_history() {
        let open = task("Open work", WorkspaceTaskStatus::InProgress, 40, "doing");
        let open_id = open.id.to_string();
        let done = task("Done work", WorkspaceTaskStatus::Completed, 100, "finished");
        let done_id = done.id.to_string();
        let cancelled = task("Dropped", WorkspaceTaskStatus::Cancelled, 10, "cancelled");
        let cancelled_id = cancelled.id.to_string();

        let graph = TaskGraph::from_parts(
            "ws-1",
            vec![open, done, cancelled],
            Vec::new(),
            true,
            Vec::new(),
        );

        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].task.id.to_string(), open_id);
        assert!(graph.nodes.iter().all(|n| n.task.status.is_open()));
        assert_eq!(graph.history_count, 2);
        assert!(graph.history.iter().all(|h| h.is_non_actionable()));
        assert!(graph.history.iter().any(|h| h.task_id == done_id));
        assert!(graph.history.iter().any(|h| h.task_id == cancelled_id));
        assert!(!graph.nodes.iter().any(|n| n.task.id.to_string() == done_id));

        let summary = graph.summary_projection(1);
        assert_eq!(summary.top_nodes.len(), 1);
        assert_eq!(summary.top_nodes[0].task.id.to_string(), open_id);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
        assert_eq!(summary.completed_count, 1);
        assert!(summary.history.iter().all(|h| !h.actionable && h.terminal));
    }

    #[test]
    fn history_preserves_completed_progress_and_explanation() {
        let done = task(
            "Ship it",
            WorkspaceTaskStatus::Completed,
            100,
            "Delivered milestone A",
        );
        let graph = TaskGraph::from_parts("ws-1", vec![done], Vec::new(), true, Vec::new());
        assert!(graph.nodes.is_empty());
        assert_eq!(graph.history_count, 1);
        let entry = &graph.history[0];
        assert_eq!(entry.progress_percent, 100);
        assert_eq!(entry.explanation, "Delivered milestone A");
        assert_eq!(entry.status, "completed");
        let json = serde_json::to_value(entry).unwrap();
        assert!(json.get("handoff_command").is_none());
        assert!(json.get("blocker_ids").is_none());
    }

    #[test]
    fn no_duplicate_identity_across_channels() {
        let open = task("Live", WorkspaceTaskStatus::Planned, 0, "queued");
        let done = task("Past", WorkspaceTaskStatus::Completed, 100, "done");
        let graph = TaskGraph::from_parts(
            "ws-1",
            vec![open, done],
            Vec::new(),
            true,
            Vec::new(),
        );
        let actionable_ids: std::collections::HashSet<_> =
            graph.nodes.iter().map(|n| n.task.id.to_string()).collect();
        let history_ids: std::collections::HashSet<_> =
            graph.history.iter().map(|h| h.task_id.clone()).collect();
        assert!(actionable_ids.is_disjoint(&history_ids));
    }
}
