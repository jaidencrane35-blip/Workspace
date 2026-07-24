//! Workspace Purpose Model — why work exists (Phase 5).
//!
//! Aggregates WorkGoals, Projects, Task Graph, Composition, Continuity, Activity,
//! and Decision Queue into durable meaning. Informational only — never executes,
//! creates goals, or grants authority. WorkGoal remains the durable SoT.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Purpose-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspacePurposeError {
    #[error("purpose model requires a workspace id")]
    MissingWorkspace,

    #[error("purpose model cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of evidence supporting a purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurposeEvidenceKind {
    WorkGoal,
    Project,
    TaskGraph,
    Composition,
    Continuity,
    Activity,
    DecisionQueue,
    Attention,
}

impl PurposeEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkGoal => "work_goal",
            Self::Project => "project",
            Self::TaskGraph => "task_graph",
            Self::Composition => "composition",
            Self::Continuity => "continuity",
            Self::Activity => "activity",
            Self::DecisionQueue => "decision_queue",
            Self::Attention => "attention",
        }
    }
}

/// Structured evidence that a purpose relationship is grounded in existing models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurposeEvidence {
    pub id: String,
    pub kind: PurposeEvidenceKind,
    pub ref_id: String,
    pub label: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// Explainable link between purpose and supporting work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurposeRelationship {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub progress_note: Option<String>,
    pub incomplete_note: Option<String>,
}

/// Obstacle standing between the user and the purpose outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurposeObstacle {
    pub kind: String,
    pub title: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// Full Purpose Model snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePurposeState {
    pub workspace_id: String,
    pub generated_at: String,
    /// Human-facing outcome label (e.g. WorkGoal description or project name).
    pub label: String,
    pub primary_work_goal_id: Option<String>,
    pub primary_work_goal_description: Option<String>,
    pub active_project_id: Option<String>,
    pub active_project_name: Option<String>,
    pub active_task_id: Option<String>,
    pub composition_label: Option<String>,
    pub focus_label: Option<String>,
    pub progress_percent: u8,
    pub open_task_count: usize,
    pub completed_task_count: usize,
    pub blocked_task_count: usize,
    pub outstanding_decision_count: usize,
    pub interrupted_count: usize,
    pub evidence_items: Vec<PurposeEvidence>,
    pub relationships: Vec<PurposeRelationship>,
    pub obstacles: Vec<PurposeObstacle>,
    pub recent_progress: Vec<String>,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspacePurposeState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspacePurposeSummary {
        WorkspacePurposeSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            primary_work_goal_description: self.primary_work_goal_description.clone(),
            active_project_name: self.active_project_name.clone(),
            composition_label: self.composition_label.clone(),
            focus_label: self.focus_label.clone(),
            progress_percent: self.progress_percent,
            open_task_count: self.open_task_count,
            completed_task_count: self.completed_task_count,
            blocked_task_count: self.blocked_task_count,
            outstanding_decision_count: self.outstanding_decision_count,
            interrupted_count: self.interrupted_count,
            obstacle_count: self.obstacles.len(),
            relationship_count: self.relationships.len(),
            top_evidence: self.evidence_items.iter().take(limit).cloned().collect(),
            top_obstacles: self.obstacles.iter().take(limit).cloned().collect(),
            recent_progress: self.recent_progress.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePurposeSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub primary_work_goal_description: Option<String>,
    pub active_project_name: Option<String>,
    pub composition_label: Option<String>,
    pub focus_label: Option<String>,
    pub progress_percent: u8,
    pub open_task_count: usize,
    pub completed_task_count: usize,
    pub blocked_task_count: usize,
    pub outstanding_decision_count: usize,
    pub interrupted_count: usize,
    pub obstacle_count: usize,
    pub relationship_count: usize,
    pub top_evidence: Vec<PurposeEvidence>,
    pub top_obstacles: Vec<PurposeObstacle>,
    pub recent_progress: Vec<String>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspacePurposeSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            primary_work_goal_description: None,
            active_project_name: None,
            composition_label: None,
            focus_label: None,
            progress_percent: 0,
            open_task_count: 0,
            completed_task_count: 0,
            blocked_task_count: 0,
            outstanding_decision_count: 0,
            interrupted_count: 0,
            obstacle_count: 0,
            relationship_count: 0,
            top_evidence: Vec::new(),
            top_obstacles: Vec::new(),
            recent_progress: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspacePurposeState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Deterministic product summary for a purpose.
pub fn build_purpose_summary(
    label: &str,
    progress: u8,
    open_tasks: usize,
    obstacles: usize,
    outstanding: usize,
) -> String {
    format!(
        "Purpose: {label} — {progress}% progress, {open_tasks} open task(s), \
         {obstacles} obstacle(s), {outstanding} outstanding decision(s). \
         Meaning only; no execution or authority."
    )
}

pub fn purpose_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_purpose_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspacePurposeError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
