//! Workspace Readiness Model — preparedness for current work (Phase 5).
//!
//! Aggregates Operating State, Environment, Composition, Task Graph, Purpose,
//! Continuity, Evolution, Patterns, and Decision Queue into explainable
//! readiness assessments. Informational only — never prepares, fixes, or
//! executes. Distinct from runtime WorkspaceHealth and from Recommendation /
//! Adaptation layers (which may consume readiness gaps).

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Readiness-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceReadinessError {
    #[error("readiness requires a workspace id")]
    MissingWorkspace,

    #[error("readiness cannot execute, prepare, or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Readiness assessment categories (never executed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessKind {
    EnvironmentReadiness,
    ContextReadiness,
    TaskReadiness,
    DecisionReadiness,
    PurposeReadiness,
}

impl ReadinessKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentReadiness => "environment_readiness",
            Self::ContextReadiness => "context_readiness",
            Self::TaskReadiness => "task_readiness",
            Self::DecisionReadiness => "decision_readiness",
            Self::PurposeReadiness => "purpose_readiness",
        }
    }
}

/// Overall / per-assessment readiness status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStatus {
    Ready,
    PartiallyReady,
    Blocked,
}

impl ReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PartiallyReady => "partially_ready",
            Self::Blocked => "blocked",
        }
    }

    /// Combine statuses: Blocked wins, then PartiallyReady, else Ready.
    pub fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Blocked, _) | (_, Self::Blocked) => Self::Blocked,
            (Self::PartiallyReady, _) | (_, Self::PartiallyReady) => Self::PartiallyReady,
            _ => Self::Ready,
        }
    }
}

/// Evidence grounding a readiness assessment in an existing Workspace signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessSignal {
    pub id: String,
    pub source_model: String,
    pub source_ref: String,
    pub summary: String,
}

/// What prevents full readiness (explain-only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessGap {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub explanation: String,
    pub impact: String,
    pub source_model: String,
    pub source_ref: String,
}

/// One typed readiness assessment (not a fix instruction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessAssessment {
    pub id: String,
    pub kind: ReadinessKind,
    pub title: String,
    pub status: ReadinessStatus,
    /// Why this status was chosen.
    pub reason: String,
    pub signals: Vec<ReadinessSignal>,
    pub gaps: Vec<ReadinessGap>,
    /// Why readiness matters for continuing work.
    pub impact: String,
    pub authority_effect: String,
}

impl ReadinessAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Human-readable readiness summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessSummary {
    pub headline: String,
    pub status_line: String,
    pub gap_line: String,
    pub narrative: String,
}

/// Full Readiness Model snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceReadinessState {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub overall_status: ReadinessStatus,
    pub assessments: Vec<ReadinessAssessment>,
    pub assessment_count: usize,
    pub gap_count: usize,
    pub ready_count: usize,
    pub partially_ready_count: usize,
    pub blocked_count: usize,
    pub readiness_summary: ReadinessSummary,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceReadinessState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceReadinessSummary {
        WorkspaceReadinessSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            overall_status: self.overall_status,
            assessment_count: self.assessment_count,
            gap_count: self.gap_count,
            ready_count: self.ready_count,
            partially_ready_count: self.partially_ready_count,
            blocked_count: self.blocked_count,
            top_assessments: self.assessments.iter().take(limit).cloned().collect(),
            top_gaps: self
                .assessments
                .iter()
                .flat_map(|a| a.gaps.iter().cloned())
                .take(limit)
                .collect(),
            readiness_summary: self.readiness_summary.clone(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceReadinessSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub overall_status: ReadinessStatus,
    pub assessment_count: usize,
    pub gap_count: usize,
    pub ready_count: usize,
    pub partially_ready_count: usize,
    pub blocked_count: usize,
    pub top_assessments: Vec<ReadinessAssessment>,
    pub top_gaps: Vec<ReadinessGap>,
    pub readiness_summary: ReadinessSummary,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceReadinessSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            overall_status: ReadinessStatus::Ready,
            assessment_count: 0,
            gap_count: 0,
            ready_count: 0,
            partially_ready_count: 0,
            blocked_count: 0,
            top_assessments: Vec::new(),
            top_gaps: Vec::new(),
            readiness_summary: ReadinessSummary {
                headline: String::new(),
                status_line: String::new(),
                gap_line: String::new(),
                narrative: String::new(),
            },
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceReadinessState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_readiness_summary(label: &str, status: ReadinessStatus, gap_count: usize) -> String {
    match status {
        ReadinessStatus::Ready => format!(
            "Readiness for \"{label}\" — ready to continue. \
             Informational only; never prepares or executes. Distinct from WorkspaceHealth."
        ),
        ReadinessStatus::PartiallyReady => format!(
            "Readiness for \"{label}\" — partially ready ({gap_count} gap(s)). \
             Informational only; never prepares or executes. Distinct from WorkspaceHealth."
        ),
        ReadinessStatus::Blocked => format!(
            "Readiness for \"{label}\" — blocked ({gap_count} gap(s)). \
             Informational only; never prepares or executes. Distinct from WorkspaceHealth."
        ),
    }
}

pub fn readiness_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_readiness_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceReadinessError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
