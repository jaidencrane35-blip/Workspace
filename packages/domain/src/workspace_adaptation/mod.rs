//! Workspace Adaptation Proposal — possible improvements (Phase 5).
//!
//! Aggregates Pattern, Recommendation Engine, Operating State, Composition,
//! Environment, Continuity, and Purpose into explainable improvement proposals.
//! Informational only — never executes, mutates layout, or grants authority.
//! Distinct from Recommendation Engine (next-step suggestions) and Decision Engine.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Adaptation-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceAdaptationError {
    #[error("adaptation requires a workspace id")]
    MissingWorkspace,

    #[error("adaptation proposal not found")]
    NotFound,

    #[error("invalid adaptation transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("adaptation cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Informational adaptation categories (never executed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptationKind {
    LayoutImprovement,
    ApplicationGrouping,
    WorkspaceOrganization,
    WorkflowShortcut,
    ContextRestoration,
    TaskOrganization,
}

impl AdaptationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LayoutImprovement => "layout_improvement",
            Self::ApplicationGrouping => "application_grouping",
            Self::WorkspaceOrganization => "workspace_organization",
            Self::WorkflowShortcut => "workflow_shortcut",
            Self::ContextRestoration => "context_restoration",
            Self::TaskOrganization => "task_organization",
        }
    }
}

/// What a proposal would target (informational only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptationTargetKind {
    Layout,
    ApplicationGroup,
    Composition,
    Continuity,
    TaskGraph,
    Purpose,
    Environment,
}

impl AdaptationTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Layout => "layout",
            Self::ApplicationGroup => "application_group",
            Self::Composition => "composition",
            Self::Continuity => "continuity",
            Self::TaskGraph => "task_graph",
            Self::Purpose => "purpose",
            Self::Environment => "environment",
        }
    }
}

/// Lifecycle of a proposal (informational — never grants authority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptationStatus {
    Proposed,
    Reviewed,
    Accepted,
    Rejected,
}

impl AdaptationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Reviewed => "reviewed",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Proposed, Self::Reviewed)
                | (Self::Proposed, Self::Rejected)
                | (Self::Reviewed, Self::Accepted)
                | (Self::Reviewed, Self::Rejected)
        )
    }
}

/// Evidence grounding an adaptation in an existing Workspace signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationEvidence {
    pub id: String,
    pub source_model: String,
    pub source_ref: String,
    pub summary: String,
}

/// What would be affected if the human later pursues this via Intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationTarget {
    pub kind: AdaptationTargetKind,
    pub ref_id: String,
    pub label: String,
}

/// Benefit and risk of pursuing the proposal (explain-only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationImpact {
    /// What benefit could occur?
    pub benefit: String,
    /// What could change / go wrong?
    pub risk: String,
}

/// One possible workspace improvement (not an instruction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationProposal {
    pub id: String,
    pub kind: AdaptationKind,
    pub title: String,
    /// Why is this suggested?
    pub reason: String,
    pub evidence: Vec<AdaptationEvidence>,
    pub impact: AdaptationImpact,
    pub target: AdaptationTarget,
    pub status: AdaptationStatus,
    pub related_pattern_id: Option<String>,
    pub related_recommendation_id: Option<String>,
    pub authority_effect: String,
}

impl AdaptationProposal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    /// Accept hands off to Intent — never executes from Adaptation.
    pub const HANDOFF_SUBMIT_ASSISTANT_GOAL: &'static str = "submit_assistant_goal";
}

/// Human-readable adaptation summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationSummary {
    pub headline: String,
    pub top_proposal_line: String,
    pub review_line: String,
    pub narrative: String,
}

/// Full Adaptation Proposal snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAdaptationState {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub proposals: Vec<AdaptationProposal>,
    pub proposal_count: usize,
    pub open_count: usize,
    pub adaptation_summary: AdaptationSummary,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceAdaptationState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceAdaptationSummary {
        WorkspaceAdaptationSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            proposal_count: self.proposal_count,
            open_count: self.open_count,
            top_proposals: self.proposals.iter().take(limit).cloned().collect(),
            adaptation_summary: self.adaptation_summary.clone(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAdaptationSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub proposal_count: usize,
    pub open_count: usize,
    pub top_proposals: Vec<AdaptationProposal>,
    pub adaptation_summary: AdaptationSummary,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceAdaptationSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            proposal_count: 0,
            open_count: 0,
            top_proposals: Vec::new(),
            adaptation_summary: AdaptationSummary {
                headline: String::new(),
                top_proposal_line: String::new(),
                review_line: String::new(),
                narrative: String::new(),
            },
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceAdaptationState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Intent handoff after accept — caller must invoke Intent/Gateway explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationHandoff {
    pub proposal_id: String,
    pub next_command: String,
    pub goal_statement: String,
    pub workspace_id: String,
    pub note: String,
    pub authority_effect: String,
}

/// Result of review / accept / reject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationActionResult {
    pub proposal: Option<AdaptationProposal>,
    pub handoff: Option<AdaptationHandoff>,
    pub authority_effect: String,
}

pub fn build_adaptation_summary(label: &str, count: usize) -> String {
    format!(
        "Adaptation proposals for \"{label}\" — {count} possible improvement(s). \
         Proposals only; never execute, mutate layout, or grant authority. Human decides."
    )
}

pub fn adaptation_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_adaptation_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceAdaptationError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
