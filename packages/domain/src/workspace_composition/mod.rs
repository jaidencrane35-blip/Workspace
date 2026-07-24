//! Workspace Composition Engine — logical working environments (Phase 5).
//!
//! Aggregates Environment, Task Graph, Continuity, Activity, and Workflow into
//! meaning: how applications, layouts, projects, and tasks belong together.
//! Informational only — never executes, launches, groups, or grants authority.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Composition-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceCompositionError {
    #[error("composition engine requires a workspace id")]
    MissingWorkspace,

    #[error("composition engine cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of resource participating in a composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompositionMemberKind {
    Application,
    Window,
    Layout,
    TaskNode,
    Project,
    ActiveWork,
    Environment,
    Activity,
    Continuity,
}

impl CompositionMemberKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Window => "window",
            Self::Layout => "layout",
            Self::TaskNode => "task_node",
            Self::Project => "project",
            Self::ActiveWork => "active_work",
            Self::Environment => "environment",
            Self::Activity => "activity",
            Self::Continuity => "continuity",
        }
    }
}

/// One resource that belongs (or should belong) in the working environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionMember {
    pub id: String,
    pub kind: CompositionMemberKind,
    pub ref_id: String,
    pub label: String,
    /// True when observed/present; false when expected but missing.
    pub present: bool,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub authority_effect: String,
}

impl CompositionMember {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Explainable link between composition members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionRelationship {
    pub id: String,
    pub from_member_id: String,
    pub to_member_id: String,
    pub kind: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// Something that appears incomplete in the composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionGap {
    pub kind: String,
    pub title: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub member_id: Option<String>,
}

/// Full Composition snapshot for a workspace (one primary working environment).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCompositionState {
    pub workspace_id: String,
    pub generated_at: String,
    /// Human label for the working environment (e.g. project name).
    pub label: String,
    pub active_project_id: Option<String>,
    pub active_project_name: Option<String>,
    pub active_task_id: Option<String>,
    pub focus_label: Option<String>,
    pub members: Vec<CompositionMember>,
    pub relationships: Vec<CompositionRelationship>,
    pub gaps: Vec<CompositionGap>,
    pub present_application_count: usize,
    pub missing_application_count: usize,
    pub task_node_count: usize,
    pub window_count: usize,
    pub outstanding_decision_count: usize,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceCompositionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceCompositionSummary {
        WorkspaceCompositionSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            active_project_name: self.active_project_name.clone(),
            focus_label: self.focus_label.clone(),
            present_application_count: self.present_application_count,
            missing_application_count: self.missing_application_count,
            task_node_count: self.task_node_count,
            window_count: self.window_count,
            outstanding_decision_count: self.outstanding_decision_count,
            member_count: self.members.len(),
            relationship_count: self.relationships.len(),
            gap_count: self.gaps.len(),
            top_members: self.members.iter().take(limit).cloned().collect(),
            top_gaps: self.gaps.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCompositionSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub active_project_name: Option<String>,
    pub focus_label: Option<String>,
    pub present_application_count: usize,
    pub missing_application_count: usize,
    pub task_node_count: usize,
    pub window_count: usize,
    pub outstanding_decision_count: usize,
    pub member_count: usize,
    pub relationship_count: usize,
    pub gap_count: usize,
    pub top_members: Vec<CompositionMember>,
    pub top_gaps: Vec<CompositionGap>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceCompositionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            active_project_name: None,
            focus_label: None,
            present_application_count: 0,
            missing_application_count: 0,
            task_node_count: 0,
            window_count: 0,
            outstanding_decision_count: 0,
            member_count: 0,
            relationship_count: 0,
            gap_count: 0,
            top_members: Vec::new(),
            top_gaps: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceCompositionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Deterministic product summary for a composition.
pub fn build_composition_summary(
    label: &str,
    present_apps: usize,
    missing_apps: usize,
    focus: Option<&str>,
    outstanding: usize,
) -> String {
    let focus_part = focus
        .map(|f| format!(" Current focus: {f}."))
        .unwrap_or_default();
    format!(
        "{label} — {present_apps} application(s) present, {missing_apps} missing, \
         {outstanding} outstanding decision(s).{focus_part} \
         Meaning only; no launch, grouping, or window control."
    )
}

pub fn composition_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_composition_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceCompositionError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
