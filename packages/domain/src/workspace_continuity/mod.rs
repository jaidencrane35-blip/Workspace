//! Workspace Continuity Engine foundation (Phase 5 Batch 1).
//!
//! Pure read model — answers where work left off, what changed, and what
//! naturally resumes. Aggregates existing systems only; no authority.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{ContinuityFacetId, WorkspaceId};

/// Continuity-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceContinuityError {
    #[error("invalid continuity facet: {0}")]
    InvalidFacet(String),

    #[error("continuity requires a workspace id")]
    MissingWorkspace,

    #[error("continuity cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of continuity facet (projection label, not ownership).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityFacetKind {
    CurrentFocus,
    InterruptedWork,
    ResumableWork,
    OutstandingDecision,
    DormantProject,
    ActiveCommitment,
    RecentProgress,
    RecentOutcome,
    Blocker,
    SuggestedNextStep,
}

impl ContinuityFacetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentFocus => "current_focus",
            Self::InterruptedWork => "interrupted_work",
            Self::ResumableWork => "resumable_work",
            Self::OutstandingDecision => "outstanding_decision",
            Self::DormantProject => "dormant_project",
            Self::ActiveCommitment => "active_commitment",
            Self::RecentProgress => "recent_progress",
            Self::RecentOutcome => "recent_outcome",
            Self::Blocker => "blocker",
            Self::SuggestedNextStep => "suggested_next_step",
        }
    }
}

/// One explainable continuity facet projected from existing sources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFacet {
    pub id: ContinuityFacetId,
    pub workspace_id: WorkspaceId,
    pub kind: ContinuityFacetKind,
    pub title: String,
    pub summary: String,
    /// Why this facet appears.
    pub why: String,
    /// Supporting synthetic ids (decision:… / activity:… / project:…).
    pub evidence_refs: Vec<String>,
    /// What changed relative to the session continuity anchor.
    pub what_changed: String,
    pub source_type: String,
    pub source_id: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub authority_effect: String,
}

impl ContinuityFacet {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn synthetic_id(kind: ContinuityFacetKind, source_key: &str) -> ContinuityFacetId {
        ContinuityFacetId::new(format!("continuity:{}:{}", kind.as_str(), source_key))
            .expect("synthetic continuity id is valid")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn project(
        workspace_id: impl Into<String>,
        kind: ContinuityFacetKind,
        source_type: impl Into<String>,
        source_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        why: impl Into<String>,
        evidence_refs: Vec<String>,
        what_changed: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<Self, WorkspaceContinuityError> {
        let source_id = source_id.into();
        Ok(Self {
            id: Self::synthetic_id(kind, &source_id),
            workspace_id: WorkspaceId::new(workspace_id)?,
            kind,
            title: title.into(),
            summary: summary.into(),
            why: why.into(),
            evidence_refs,
            what_changed: what_changed.into(),
            source_type: source_type.into(),
            source_id,
            project_id,
            task_id,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Full continuity snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContinuityState {
    pub workspace_id: String,
    pub generated_at: String,
    /// Deterministic session anchor used for "what changed".
    pub session_anchor: String,
    pub current_focus: Option<ContinuityFacet>,
    pub interrupted_work: Vec<ContinuityFacet>,
    pub resumable_work: Vec<ContinuityFacet>,
    pub outstanding_decisions: Vec<ContinuityFacet>,
    pub dormant_projects: Vec<ContinuityFacet>,
    pub active_commitments: Vec<ContinuityFacet>,
    pub recent_progress: Vec<ContinuityFacet>,
    pub recent_outcomes: Vec<ContinuityFacet>,
    pub blockers: Vec<ContinuityFacet>,
    pub suggested_next_step: Option<ContinuityFacet>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceContinuityState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, progress_limit: usize) -> WorkspaceContinuitySummary {
        WorkspaceContinuitySummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            session_anchor: self.session_anchor.clone(),
            current_focus: self.current_focus.clone(),
            interrupted_count: self.interrupted_work.len(),
            resumable_count: self.resumable_work.len(),
            outstanding_decision_count: self.outstanding_decisions.len(),
            blocker_count: self.blockers.len(),
            dormant_project_count: self.dormant_projects.len(),
            active_commitment_count: self.active_commitments.len(),
            suggested_next_step: self.suggested_next_step.clone(),
            recent_progress: self
                .recent_progress
                .iter()
                .take(progress_limit)
                .cloned()
                .collect(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContinuitySummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub session_anchor: String,
    pub current_focus: Option<ContinuityFacet>,
    pub interrupted_count: usize,
    pub resumable_count: usize,
    pub outstanding_decision_count: usize,
    pub blocker_count: usize,
    pub dormant_project_count: usize,
    pub active_commitment_count: usize,
    pub suggested_next_step: Option<ContinuityFacet>,
    pub recent_progress: Vec<ContinuityFacet>,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceContinuitySummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            session_anchor: String::new(),
            current_focus: None,
            interrupted_count: 0,
            resumable_count: 0,
            outstanding_decision_count: 0,
            blocker_count: 0,
            dormant_project_count: 0,
            active_commitment_count: 0,
            suggested_next_step: None,
            recent_progress: Vec::new(),
            summary: String::new(),
            authority_effect: ContinuityFacet::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Helper for generated_at timestamps in tests and services.
pub fn continuity_now() -> String {
    Utc::now().to_rfc3339()
}
