//! Workspace Environment Model — live desktop read model (Phase 5).
//!
//! Aggregates Windows Integration observations with work context.
//! Informational only — never executes, moves windows, or grants authority.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Environment-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceEnvironmentError {
    #[error("environment model requires a workspace id")]
    MissingWorkspace,

    #[error("environment model cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Observed OS window lifecycle (best-effort from enumerator).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentWindowState {
    Open,
    Minimized,
    Focused,
    Unknown,
}

impl EnvironmentWindowState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Minimized => "minimized",
            Self::Focused => "focused",
            Self::Unknown => "unknown",
        }
    }
}

/// One observed desktop window, optionally matched to a Workspace application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentWindow {
    pub id: String,
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    pub state: EnvironmentWindowState,
    pub matched_application_id: Option<String>,
    pub matched_application_name: Option<String>,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub layout_id: Option<String>,
    pub display_label: String,
    pub explanation: String,
    pub authority_effect: String,
}

impl EnvironmentWindow {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Registered application presence on the live desktop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentApplication {
    pub application_id: String,
    pub name: String,
    pub identifier: Option<String>,
    pub appears_running: bool,
    pub window_count: usize,
    pub focused: bool,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub explanation: String,
}

/// Windows that belong together (same process or matched application).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentWindowGroup {
    pub id: String,
    pub label: String,
    pub application_id: Option<String>,
    pub process_id: Option<u32>,
    pub window_ids: Vec<String>,
    pub project_id: Option<String>,
    pub explanation: String,
}

/// Soft association between environment and Workspace layout (canvas — not OS monitors).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentLayoutAssociation {
    pub layout_id: String,
    pub layout_name: String,
    pub explanation: String,
}

/// Missing registered app or active work without matching windows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentGap {
    pub kind: String,
    pub title: String,
    pub explanation: String,
    pub application_id: Option<String>,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

/// Full Environment Model snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEnvironmentState {
    pub workspace_id: String,
    pub generated_at: String,
    pub active_project_id: Option<String>,
    pub active_task_id: Option<String>,
    pub windows: Vec<EnvironmentWindow>,
    pub applications: Vec<EnvironmentApplication>,
    pub window_groups: Vec<EnvironmentWindowGroup>,
    pub layout_associations: Vec<EnvironmentLayoutAssociation>,
    pub gaps: Vec<EnvironmentGap>,
    pub focused_window_id: Option<String>,
    pub running_application_count: usize,
    pub missing_application_count: usize,
    pub disconnected_work: bool,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceEnvironmentState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceEnvironmentSummary {
        WorkspaceEnvironmentSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            window_count: self.windows.len(),
            running_application_count: self.running_application_count,
            missing_application_count: self.missing_application_count,
            group_count: self.window_groups.len(),
            disconnected_work: self.disconnected_work,
            focused_window_title: self
                .focused_window_id
                .as_ref()
                .and_then(|id| self.windows.iter().find(|w| &w.id == id))
                .map(|w| w.title.clone()),
            top_applications: self.applications.iter().take(limit).cloned().collect(),
            top_gaps: self.gaps.iter().take(limit).cloned().collect(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEnvironmentSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub window_count: usize,
    pub running_application_count: usize,
    pub missing_application_count: usize,
    pub group_count: usize,
    pub disconnected_work: bool,
    pub focused_window_title: Option<String>,
    pub top_applications: Vec<EnvironmentApplication>,
    pub top_gaps: Vec<EnvironmentGap>,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceEnvironmentSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            window_count: 0,
            running_application_count: 0,
            missing_application_count: 0,
            group_count: 0,
            disconnected_work: false,
            focused_window_title: None,
            top_applications: Vec::new(),
            top_gaps: Vec::new(),
            summary: String::new(),
            authority_effect: WorkspaceEnvironmentState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Build a deterministic summary string.
pub fn build_environment_summary(
    workspace_id: &str,
    window_count: usize,
    running: usize,
    missing: usize,
    disconnected: bool,
) -> String {
    let _ = workspace_id;
    let disconnect = if disconnected {
        " Active work appears disconnected from open windows."
    } else {
        ""
    };
    format!(
        "Environment — {window_count} window(s), {running} running app(s), {missing} missing registered app(s).{disconnect} Informational only; no window control."
    )
}

pub fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_workspace_id(workspace_id: impl Into<String>) -> Result<WorkspaceId, WorkspaceEnvironmentError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
