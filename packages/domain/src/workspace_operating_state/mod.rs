//! Workspace Operating State — what is happening right now (Phase 5).
//!
//! Aggregates Environment, Composition, Task Graph, Purpose, Evolution,
//! Continuity, Activity, Decision Queue, Attention, and Recommendation Engine
//! into a unified current-situation snapshot.
//! Informational only — never executes, owns, or grants authority.
//! Does not replace source models; does not persist snapshots.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Operating State validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceOperatingStateError {
    #[error("operating state requires a workspace id")]
    MissingWorkspace,

    #[error("operating state cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Which subsystem grounded an operating signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatingSignalKind {
    Purpose,
    Project,
    ActiveWork,
    Environment,
    Composition,
    Progress,
    Blocker,
    PendingDecision,
    Recommendation,
    Attention,
    Continuity,
    Evolution,
}

impl OperatingSignalKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Purpose => "purpose",
            Self::Project => "project",
            Self::ActiveWork => "active_work",
            Self::Environment => "environment",
            Self::Composition => "composition",
            Self::Progress => "progress",
            Self::Blocker => "blocker",
            Self::PendingDecision => "pending_decision",
            Self::Recommendation => "recommendation",
            Self::Attention => "attention",
            Self::Continuity => "continuity",
            Self::Evolution => "evolution",
        }
    }
}

/// One explainable facet of the current workspace situation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingSignal {
    pub id: String,
    pub kind: OperatingSignalKind,
    /// What is active / present?
    pub current_value: String,
    /// Which subsystem provided this?
    pub source_model: String,
    pub source_ref: String,
    pub evidence: Vec<String>,
    pub authority_effect: String,
}

impl OperatingSignal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Explainable link between operating signals / source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingRelationship {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    /// Why does this belong together?
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// Structured "what am I doing right now?" context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingContext {
    pub purpose_label: String,
    pub active_project_label: Option<String>,
    pub active_task_label: Option<String>,
    pub environment_summary: String,
    pub composition_label: String,
    pub recent_progress: Vec<String>,
    pub current_blockers: Vec<String>,
    pub pending_decisions: Vec<String>,
    pub top_recommendations: Vec<String>,
    pub attention_priorities: Vec<String>,
    pub continuity_focus: Option<String>,
}

/// Human-readable operating summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingSummary {
    pub headline: String,
    pub purpose_line: String,
    pub environment_line: String,
    pub progress_line: String,
    pub pending_line: String,
    pub suggested_line: String,
    pub narrative: String,
}

/// Full Operating State snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOperatingState {
    pub workspace_id: String,
    pub generated_at: String,
    pub context: OperatingContext,
    pub signals: Vec<OperatingSignal>,
    pub relationships: Vec<OperatingRelationship>,
    pub operating_summary: OperatingSummary,
    pub signal_count: usize,
    pub relationship_count: usize,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceOperatingState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceOperatingStateSummary {
        WorkspaceOperatingStateSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            context: self.context.clone(),
            operating_summary: self.operating_summary.clone(),
            signal_count: self.signal_count,
            relationship_count: self.relationship_count,
            top_signals: self.signals.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOperatingStateSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub context: OperatingContext,
    pub operating_summary: OperatingSummary,
    pub signal_count: usize,
    pub relationship_count: usize,
    pub top_signals: Vec<OperatingSignal>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceOperatingStateSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            context: OperatingContext {
                purpose_label: String::new(),
                active_project_label: None,
                active_task_label: None,
                environment_summary: String::new(),
                composition_label: String::new(),
                recent_progress: Vec::new(),
                current_blockers: Vec::new(),
                pending_decisions: Vec::new(),
                top_recommendations: Vec::new(),
                attention_priorities: Vec::new(),
                continuity_focus: None,
            },
            operating_summary: OperatingSummary {
                headline: String::new(),
                purpose_line: String::new(),
                environment_line: String::new(),
                progress_line: String::new(),
                pending_line: String::new(),
                suggested_line: String::new(),
                narrative: String::new(),
            },
            signal_count: 0,
            relationship_count: 0,
            top_signals: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceOperatingState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_operating_state_summary(purpose: &str, signal_count: usize) -> String {
    format!(
        "Operating State for \"{purpose}\" — {signal_count} signal(s). \
         Snapshot of understanding only; never executes, approves, or grants authority."
    )
}

pub fn operating_state_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_operating_state_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceOperatingStateError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
