//! Workspace Activity Graph foundation (Phase 4 Batch 9).
//!
//! Aggregates existing Workspace objects into one coherent history.
//! Informational only — no authority, no execution, no duplicate ownership.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{WorkspaceActivityId, WorkspaceId};

/// Activity-graph validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceActivityError {
    #[error("invalid activity type: {0}")]
    InvalidActivityType(String),

    #[error("invalid activity source type: {0}")]
    InvalidSourceType(String),

    #[error("activity not found")]
    NotFound,

    #[error("activity graph cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of work node in the activity graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Project,
    Task,
    WorkGoal,
    AutomationContract,
    TriggerEvent,
    IntentProposal,
    DecisionItem,
    PermissionApproval,
    PlanningContinuation,
    BlockedAction,
    ExecutionOutcome,
    AuditSignal,
}

impl ActivityType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Task => "task",
            Self::WorkGoal => "work_goal",
            Self::AutomationContract => "automation_contract",
            Self::TriggerEvent => "trigger_event",
            Self::IntentProposal => "intent_proposal",
            Self::DecisionItem => "decision_item",
            Self::PermissionApproval => "permission_approval",
            Self::PlanningContinuation => "planning_continuation",
            Self::BlockedAction => "blocked_action",
            Self::ExecutionOutcome => "execution_outcome",
            Self::AuditSignal => "audit_signal",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceActivityError> {
        match value {
            "project" => Ok(Self::Project),
            "task" => Ok(Self::Task),
            "work_goal" => Ok(Self::WorkGoal),
            "automation_contract" => Ok(Self::AutomationContract),
            "trigger_event" => Ok(Self::TriggerEvent),
            "intent_proposal" => Ok(Self::IntentProposal),
            "decision_item" => Ok(Self::DecisionItem),
            "permission_approval" => Ok(Self::PermissionApproval),
            "planning_continuation" => Ok(Self::PlanningContinuation),
            "blocked_action" => Ok(Self::BlockedAction),
            "execution_outcome" => Ok(Self::ExecutionOutcome),
            "audit_signal" => Ok(Self::AuditSignal),
            other => Err(WorkspaceActivityError::InvalidActivityType(other.into())),
        }
    }
}

/// Provenance system for an activity node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivitySourceType {
    WorkContext,
    AutomationContract,
    TriggerEvaluation,
    DecisionQueue,
    PermissionApproval,
    Planning,
    Execution,
    Audit,
}

impl ActivitySourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkContext => "work_context",
            Self::AutomationContract => "automation_contract",
            Self::TriggerEvaluation => "trigger_evaluation",
            Self::DecisionQueue => "decision_queue",
            Self::PermissionApproval => "permission_approval",
            Self::Planning => "planning",
            Self::Execution => "execution",
            Self::Audit => "audit",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceActivityError> {
        match value {
            "work_context" => Ok(Self::WorkContext),
            "automation_contract" => Ok(Self::AutomationContract),
            "trigger_evaluation" => Ok(Self::TriggerEvaluation),
            "decision_queue" => Ok(Self::DecisionQueue),
            "permission_approval" => Ok(Self::PermissionApproval),
            "planning" => Ok(Self::Planning),
            "execution" => Ok(Self::Execution),
            "audit" => Ok(Self::Audit),
            other => Err(WorkspaceActivityError::InvalidSourceType(other.into())),
        }
    }
}

/// One node in the Workspace Activity Graph (aggregated, not owned).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActivity {
    pub id: WorkspaceActivityId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub activity_type: ActivityType,
    pub source_type: ActivitySourceType,
    pub source_id: String,
    pub parent_activity_id: Option<String>,
    pub related_activity_ids: Vec<String>,
    pub summary: String,
    pub explanation: String,
    pub timestamp: String,
    pub actor_id: String,
    pub actor_type: String,
    pub unresolved: bool,
    pub authority_effect: String,
}

impl WorkspaceActivity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn synthetic_id(activity_type: ActivityType, source_id: &str) -> WorkspaceActivityId {
        WorkspaceActivityId::new(format!("activity:{}:{}", activity_type.as_str(), source_id))
            .expect("synthetic activity id is valid")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn aggregate(
        workspace_id: impl Into<String>,
        activity_type: ActivityType,
        source_type: ActivitySourceType,
        source_id: impl Into<String>,
        summary: impl Into<String>,
        explanation: impl Into<String>,
        timestamp: impl Into<String>,
        actor_id: impl Into<String>,
        actor_type: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
        parent_activity_id: Option<String>,
        related_activity_ids: Vec<String>,
        unresolved: bool,
    ) -> Result<Self, WorkspaceActivityError> {
        let source_id = source_id.into();
        Ok(Self {
            id: Self::synthetic_id(activity_type, &source_id),
            workspace_id: WorkspaceId::new(workspace_id)?,
            project_id,
            task_id,
            activity_type,
            source_type,
            source_id,
            parent_activity_id,
            related_activity_ids,
            summary: summary.into(),
            explanation: explanation.into(),
            timestamp: timestamp.into(),
            actor_id: actor_id.into(),
            actor_type: actor_type.into(),
            unresolved,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Full activity graph snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActivityGraph {
    pub workspace_id: String,
    pub generated_at: String,
    pub activities: Vec<WorkspaceActivity>,
    pub timeline: Vec<WorkspaceActivity>,
    pub relationship_count: usize,
    pub unresolved_count: usize,
    pub authority_effect: String,
}

impl WorkspaceActivityGraph {
    pub fn from_activities(
        workspace_id: impl Into<String>,
        mut activities: Vec<WorkspaceActivity>,
    ) -> Self {
        // Deterministic timeline: timestamp ASC, then id ASC.
        activities.sort_by(|a, b| {
            a.timestamp
                .cmp(&b.timestamp)
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        let timeline = activities.clone();
        let relationship_count = activities
            .iter()
            .map(|a| {
                a.related_activity_ids.len() + usize::from(a.parent_activity_id.is_some())
            })
            .sum();
        let unresolved_count = activities.iter().filter(|a| a.unresolved).count();
        Self {
            workspace_id: workspace_id.into(),
            generated_at: Utc::now().to_rfc3339(),
            activities,
            timeline,
            relationship_count,
            unresolved_count,
            authority_effect: WorkspaceActivity::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Newest-first timeline with evaluation telemetry excluded (Sprint 128).
    ///
    /// `AuditSignal` entries record what the system did *while* thinking — generating
    /// Attention, Purpose, Evolution, and so on. Every cognitive consumer already skipped
    /// them, but skipping inside a `take(n)` loop let a burst of telemetry consume the
    /// window and push real work out of view, so the next evaluation saw different facts.
    /// Window through this method instead: filter first, then take.
    pub fn cognitive_timeline(&self) -> impl Iterator<Item = &WorkspaceActivity> {
        self.timeline
            .iter()
            .rev()
            .filter(|a| a.activity_type != ActivityType::AuditSignal)
    }

    pub fn summary(&self, limit: usize) -> WorkspaceActivityGraphSummary {
        let recent: Vec<_> = self
            .timeline
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect();
        WorkspaceActivityGraphSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            activity_count: self.activities.len(),
            relationship_count: self.relationship_count,
            unresolved_count: self.unresolved_count,
            recent_timeline: recent,
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Workspace Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActivityGraphSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub activity_count: usize,
    pub relationship_count: usize,
    pub unresolved_count: usize,
    pub recent_timeline: Vec<WorkspaceActivity>,
    pub authority_effect: String,
}

impl Default for WorkspaceActivityGraphSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            activity_count: 0,
            relationship_count: 0,
            unresolved_count: 0,
            recent_timeline: Vec::new(),
            authority_effect: WorkspaceActivity::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn activity(activity_type: ActivityType, source_id: &str, timestamp: &str) -> WorkspaceActivity {
        WorkspaceActivity::aggregate(
            "ws",
            activity_type,
            ActivitySourceType::Audit,
            source_id,
            source_id,
            "explanation",
            timestamp,
            "system",
            "System",
            None,
            None,
            None,
            Vec::new(),
            false,
        )
        .unwrap()
    }

    #[test]
    fn cognitive_timeline_filters_telemetry_before_windowing() {
        let graph = WorkspaceActivityGraph::from_activities(
            "ws",
            vec![
                activity(ActivityType::Task, "real-work", "2026-01-01T00:00:00Z"),
                activity(ActivityType::AuditSignal, "eval-1", "2026-01-01T00:00:01Z"),
                activity(ActivityType::AuditSignal, "eval-2", "2026-01-01T00:00:02Z"),
                activity(ActivityType::AuditSignal, "eval-3", "2026-01-01T00:00:03Z"),
            ],
        );
        // A burst of newer telemetry must not push real work out of a small window.
        let windowed: Vec<_> = graph
            .cognitive_timeline()
            .take(2)
            .map(|a| a.source_id.clone())
            .collect();
        assert_eq!(windowed, vec!["real-work".to_string()]);
        assert_eq!(graph.timeline.len(), 4, "telemetry stays in the timeline");
    }
}
