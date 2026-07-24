//! Workspace Pattern Model — recurring structures (Phase 5).
//!
//! Aggregates Activity Graph, Evolution, Operating State, Composition, Task Graph,
//! Environment, Purpose, Continuity, and Decision Queue into explainable patterns.
//! Informational only — not prediction, surveillance, profiling, or automation.
//! Does not duplicate Activity Graph history or Memory.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Pattern Model validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspacePatternError {
    #[error("pattern model requires a workspace id")]
    MissingWorkspace,

    #[error("pattern model cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Informational pattern categories (never executed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternKind {
    ApplicationPattern,
    WorkflowPattern,
    TaskPattern,
    DecisionPattern,
    EnvironmentPattern,
}

impl PatternKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplicationPattern => "application_pattern",
            Self::WorkflowPattern => "workflow_pattern",
            Self::TaskPattern => "task_pattern",
            Self::DecisionPattern => "decision_pattern",
            Self::EnvironmentPattern => "environment_pattern",
        }
    }
}

/// Display-only consistency hint (not an opaque AI score).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternConfidence {
    High,
    Medium,
    Low,
}

impl PatternConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Evidence grounding a pattern in an existing Workspace signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternEvidence {
    pub id: String,
    pub source_model: String,
    pub source_ref: String,
    pub summary: String,
}

/// Explainable link between patterns / source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternRelationship {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// One recurring structure observation (not a prediction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePattern {
    pub id: String,
    pub kind: PatternKind,
    pub title: String,
    /// What repeated?
    pub observation: String,
    /// Which existing workspace events support this?
    pub evidence: Vec<PatternEvidence>,
    /// How consistent is the pattern?
    pub confidence: PatternConfidence,
    /// Why might this matter?
    pub impact: String,
    pub authority_effect: String,
}

impl WorkspacePattern {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Human-readable pattern summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternSummary {
    pub headline: String,
    pub recurring_line: String,
    pub workflow_line: String,
    pub environment_line: String,
    pub narrative: String,
}

/// Full Pattern Model snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePatternState {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub patterns: Vec<WorkspacePattern>,
    pub relationships: Vec<PatternRelationship>,
    pub pattern_summary: PatternSummary,
    pub pattern_count: usize,
    pub relationship_count: usize,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspacePatternState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspacePatternSummary {
        WorkspacePatternSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            pattern_count: self.pattern_count,
            relationship_count: self.relationship_count,
            top_patterns: self.patterns.iter().take(limit).cloned().collect(),
            pattern_summary: self.pattern_summary.clone(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePatternSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub pattern_count: usize,
    pub relationship_count: usize,
    pub top_patterns: Vec<WorkspacePattern>,
    pub pattern_summary: PatternSummary,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspacePatternSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            pattern_count: 0,
            relationship_count: 0,
            top_patterns: Vec::new(),
            pattern_summary: PatternSummary {
                headline: String::new(),
                recurring_line: String::new(),
                workflow_line: String::new(),
                environment_line: String::new(),
                narrative: String::new(),
            },
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspacePatternState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_pattern_model_summary(label: &str, count: usize) -> String {
    format!(
        "Patterns for \"{label}\" — {count} recurring structure(s). \
         Observations only; never predict, profile, execute, or grant authority."
    )
}

pub fn pattern_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_pattern_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspacePatternError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
