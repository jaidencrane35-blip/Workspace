//! Workspace Recommendation Engine — what might be useful next (Phase 5).
//!
//! Aggregates Attention, Continuity, Evolution, Purpose, Task Graph, Composition,
//! Decision Queue, and Environment into typed next-step suggestions.
//! Informational only — never executes, accepts, or grants authority.
//! Distinct from Intelligence `WorkspaceRecommendation` (Attention projection)
//! and from Decision Engine (accept → planner handoff).

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;
use crate::workspace_attention::AttentionReason;

/// Recommendation Engine validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceRecommendationEngineError {
    #[error("recommendation engine requires a workspace id")]
    MissingWorkspace,

    #[error("recommendation engine cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Informational recommendation categories (never executed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationKind {
    ContinueWork,
    ResolveBlocker,
    ReviewDecision,
    CompleteTask,
    ReorganizeWorkspace,
    RestoreContext,
    ExploreOpportunity,
}

impl RecommendationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContinueWork => "continue_work",
            Self::ResolveBlocker => "resolve_blocker",
            Self::ReviewDecision => "review_decision",
            Self::CompleteTask => "complete_task",
            Self::ReorganizeWorkspace => "reorganize_workspace",
            Self::RestoreContext => "restore_context",
            Self::ExploreOpportunity => "explore_opportunity",
        }
    }
}

/// Display-only strength hint (not an opaque AI score).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationConfidence {
    High,
    Medium,
    Low,
}

impl RecommendationConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Evidence grounding a recommendation in an existing Workspace signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationEvidence {
    pub id: String,
    pub source_model: String,
    pub source_ref: String,
    pub summary: String,
}

/// Explainable link between recommendation candidates / source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationRelationship {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// One typed next-step suggestion (not an instruction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationItem {
    pub id: String,
    pub kind: RecommendationKind,
    pub title: String,
    /// Why is this suggested?
    pub reason: String,
    /// Which workspace signals support it?
    pub evidence: Vec<RecommendationEvidence>,
    /// What would this improve?
    pub impact: String,
    pub confidence: RecommendationConfidence,
    pub related_attention_id: Option<String>,
    /// Structured reasons carried verbatim from the Attention item named by
    /// `related_attention_id`, in Attention's order. Empty when the suggestion derives
    /// from another model (Continuity, Task Graph, Composition, Pattern, Readiness).
    pub attention_reasons: Vec<AttentionReason>,
    pub related_task_id: Option<String>,
    pub related_purpose_label: Option<String>,
    pub related_decision_id: Option<String>,
    pub authority_effect: String,
}

impl RecommendationItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Full Recommendation Engine snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRecommendationEngineState {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub candidates: Vec<RecommendationItem>,
    pub relationships: Vec<RecommendationRelationship>,
    pub candidate_count: usize,
    pub relationship_count: usize,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceRecommendationEngineState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceRecommendationEngineSummary {
        WorkspaceRecommendationEngineSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            candidate_count: self.candidate_count,
            relationship_count: self.relationship_count,
            top_candidates: self.candidates.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRecommendationEngineSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub candidate_count: usize,
    pub relationship_count: usize,
    pub top_candidates: Vec<RecommendationItem>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceRecommendationEngineSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            candidate_count: 0,
            relationship_count: 0,
            top_candidates: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceRecommendationEngineState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_recommendation_engine_summary(label: &str, count: usize) -> String {
    format!(
        "Recommendations for \"{label}\" — {count} suggested next step(s). \
         Suggestions only; never execute, approve, or grant authority."
    )
}

pub fn recommendation_engine_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_recommendation_engine_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceRecommendationEngineError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
