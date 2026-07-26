//! Workspace Milestone Engine — coordination projection of meaningful outcomes (Phase 6).
//!
//! Helps users understand progress toward outcomes. Owns no data, planning, or authority.
//! Deterministic projections only — not a planner, scheduler, or executor.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Milestone-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceMilestoneError {
    #[error("milestones require a workspace id")]
    MissingWorkspace,

    #[error("milestones cannot execute, plan, schedule, complete, or authorize")]
    CannotExecute,

    #[error("milestone validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Milestone status (informational coordination only — not a workflow engine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneStatus {
    Current,
    Upcoming,
    Blocked,
    Completed,
}

impl MilestoneStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Upcoming => "upcoming",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
        }
    }
}

/// Informational relationship between milestones / supporting work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneRelationKind {
    Current,
    Upcoming,
    Blocked,
    Completed,
    DependsOn,
    ContributesTo,
    Supersedes,
    Supports,
}

impl MilestoneRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Upcoming => "upcoming",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::DependsOn => "depends_on",
            Self::ContributesTo => "contributes_to",
            Self::Supersedes => "supersedes",
            Self::Supports => "supports",
        }
    }
}

/// Deterministic readiness band for milestone completion (not a forecast).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneReadinessBand {
    Ready,
    PartiallyReady,
    Blocked,
    Unknown,
}

impl MilestoneReadinessBand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PartiallyReady => "partially_ready",
            Self::Blocked => "blocked",
            Self::Unknown => "unknown",
        }
    }
}

/// Explainable progress evidence pointer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneEvidence {
    pub label: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// Association pointer into existing projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneAssociation {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// One meaningful outcome projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestone {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub status: MilestoneStatus,
    pub readiness: MilestoneReadinessBand,
    pub progress_percent: u8,
    pub evidence: Vec<MilestoneEvidence>,
    pub related_tasks: Vec<MilestoneAssociation>,
    pub related_projects: Vec<MilestoneAssociation>,
    pub supporting_contexts: Vec<MilestoneAssociation>,
    pub outstanding_decisions: Vec<MilestoneAssociation>,
    pub dependencies: Vec<MilestoneAssociation>,
    pub why: String,
    pub authority_effect: String,
}

impl WorkspaceMilestone {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Relationship between milestones (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneRelationship {
    pub id: String,
    pub from_milestone_id: String,
    pub to_milestone_id: String,
    pub kind: MilestoneRelationKind,
    pub why: String,
    pub authority_effect: String,
}

/// Human-facing milestone summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneSummary {
    pub headline: String,
    pub current_line: String,
    pub closest_line: String,
    pub blocked_line: String,
    pub completed_line: String,
    pub next_attention_line: String,
    pub narrative: String,
}

/// Full Milestone Engine snapshot — coordination projection only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestoneState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub milestone_summary: MilestoneSummary,
    pub milestones: Vec<WorkspaceMilestone>,
    pub relationships: Vec<MilestoneRelationship>,
    pub current_milestone_id: Option<String>,
    pub milestone_count: usize,
    pub current_count: usize,
    pub upcoming_count: usize,
    pub blocked_count: usize,
    pub completed_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub work_context_generated_at: String,
    pub navigation_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceMilestoneState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn current_milestone(&self) -> Option<&WorkspaceMilestone> {
        let id = self.current_milestone_id.as_ref()?;
        self.milestones.iter().find(|m| &m.id == id)
    }

    pub fn summary_projection(&self, limit: usize) -> WorkspaceMilestoneSummary {
        WorkspaceMilestoneSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            milestone_summary: self.milestone_summary.clone(),
            milestone_count: self.milestone_count,
            current_count: self.current_count,
            upcoming_count: self.upcoming_count,
            blocked_count: self.blocked_count,
            completed_count: self.completed_count,
            current_milestone_title: self.current_milestone().map(|m| m.title.clone()),
            top_milestones: self.milestones.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestoneSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub milestone_summary: MilestoneSummary,
    pub milestone_count: usize,
    pub current_count: usize,
    pub upcoming_count: usize,
    pub blocked_count: usize,
    pub completed_count: usize,
    pub current_milestone_title: Option<String>,
    pub top_milestones: Vec<WorkspaceMilestone>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceMilestoneSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            milestone_summary: MilestoneSummary {
                headline: String::new(),
                current_line: String::new(),
                closest_line: String::new(),
                blocked_line: String::new(),
                completed_line: String::new(),
                next_attention_line: String::new(),
                narrative: String::new(),
            },
            milestone_count: 0,
            current_count: 0,
            upcoming_count: 0,
            blocked_count: 0,
            completed_count: 0,
            current_milestone_title: None,
            top_milestones: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestoneComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceMilestoneComparison {
    pub fn compare(left: &WorkspaceMilestoneState, right: &WorkspaceMilestoneState) -> Self {
        let mut differences = Vec::new();
        if left.current_milestone_id != right.current_milestone_id {
            differences.push(format!(
                "current_milestone: {:?} → {:?}",
                left.current_milestone().map(|m| &m.title),
                right.current_milestone().map(|m| &m.title)
            ));
        }
        if left.milestone_count != right.milestone_count {
            differences.push(format!(
                "milestone_count: {} → {}",
                left.milestone_count, right.milestone_count
            ));
        }
        if left.blocked_count != right.blocked_count {
            differences.push(format!(
                "blocked_count: {} → {}",
                left.blocked_count, right.blocked_count
            ));
        }
        if left.completed_count != right.completed_count {
            differences.push(format!(
                "completed_count: {} → {}",
                left.completed_count, right.completed_count
            ));
        }
        if left.milestone_summary.current_line != right.milestone_summary.current_line {
            differences.push("current_line differs".into());
        }
        if differences.is_empty() {
            differences.push("no material milestone differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMilestoneValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceMilestoneValidation {
    pub fn validate(state: &WorkspaceMilestoneState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        if state.milestones.is_empty() {
            messages.push("at least one milestone required".into());
        }
        for milestone in &state.milestones {
            if milestone.why.is_empty() {
                messages.push(format!("milestone {} missing why", milestone.id));
            }
            if milestone.evidence.is_empty() {
                messages.push(format!("milestone {} missing evidence", milestone.id));
            }
            if milestone.authority_effect != WorkspaceMilestone::AUTHORITY_EFFECT_NONE {
                messages.push(format!("milestone {} has authority", milestone.id));
            }
            for ev in &milestone.evidence {
                if ev.why.is_empty() || ev.source_projection.is_empty() {
                    messages.push(format!("milestone {} incomplete evidence", milestone.id));
                }
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} milestone(s) · blocked {} · completed {}",
                state.milestone_count, state.blocked_count, state.completed_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceMilestoneState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn milestone_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_milestone_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceMilestoneError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceMilestoneError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceMilestoneError::Domain)
}

pub fn build_milestone_summary(label: &str, count: usize, current: Option<&str>) -> String {
    match current {
        Some(title) => {
            format!("Milestones for \"{label}\": {count} outcome(s); current is {title}.")
        }
        None => format!("Milestones for \"{label}\": {count} outcome(s); no current milestone."),
    }
}
