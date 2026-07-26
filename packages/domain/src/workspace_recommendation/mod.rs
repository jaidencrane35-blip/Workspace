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

    #[error("recommendation candidate not found")]
    NotFound,

    #[error("recommendation lifecycle transition not allowed from {from} to {to}")]
    InvalidLifecycleTransition { from: String, to: String },

    #[error("recommendation outcome requires a resolved lifecycle state")]
    OutcomeRequiresResolution,

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

/// Structured, non-authoritative explanation for Operator / Work surfaces (Sprint 202).
///
/// Grounds "why this recommendation is shown" in evidence refs, catalog explanation
/// keys, lifecycle continuity, and optional Experience match keys.
/// Never chain-of-thought, never a capability grant, never an execution path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationExplanationView {
    pub why_suggested: String,
    pub impact: String,
    pub confidence: String,
    pub lifecycle_state: String,
    pub lifecycle_note: String,
    pub source_domains: Vec<String>,
    pub evidence_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    /// Catalog keys for Experience translation — not internal reasoning text.
    pub explanation_keys: Vec<String>,
    /// Optional Experience resolver match keys (evidence/provenance only).
    pub experience_trace_match_keys: Vec<String>,
    pub related_attention_id: Option<String>,
    pub related_task_id: Option<String>,
    pub related_decision_id: Option<String>,
    pub continuity_fingerprint: String,
    pub authority_effect: String,
}

impl RecommendationExplanationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_item(item: &RecommendationItem) -> Self {
        let lifecycle_state = item
            .lifecycle_state
            .clone()
            .unwrap_or_else(|| "available".into());
        let mut source_domains: Vec<String> = item
            .evidence
            .iter()
            .map(|e| e.source_model.clone())
            .collect();
        source_domains.sort();
        source_domains.dedup();
        let explanation_keys: Vec<String> = item
            .attention_reasons
            .iter()
            .map(|r| r.explanation_key.clone())
            .collect();
        Self {
            why_suggested: item.reason.clone(),
            impact: item.impact.clone(),
            confidence: item.confidence.as_str().into(),
            lifecycle_note: lifecycle_surface_note(
                &lifecycle_state,
                item.lifecycle_resolution_type.as_deref(),
            ),
            lifecycle_state,
            source_domains,
            evidence_summaries: item.evidence.iter().map(|e| e.summary.clone()).collect(),
            evidence_refs: item.evidence.iter().map(|e| e.source_ref.clone()).collect(),
            explanation_keys,
            experience_trace_match_keys: Vec::new(),
            related_attention_id: item.related_attention_id.clone(),
            related_task_id: item.related_task_id.clone(),
            related_decision_id: item.related_decision_id.clone(),
            continuity_fingerprint: item.continuity_fingerprint(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_experience_trace_match_keys(mut self, keys: Vec<String>) -> Self {
        self.experience_trace_match_keys = keys;
        self
    }
}

fn lifecycle_surface_note(state: &str, resolution: Option<&str>) -> String {
    match state {
        "created" => "Created — not yet available for review.".into(),
        "available" => "Available for review — proposal only; does not execute.".into(),
        "presented" => "Presented for human review — still a proposal.".into(),
        "accepted" => {
            "Accepted as a human decision record only — does not execute or grant authority."
                .into()
        }
        "rejected" => "Rejected by human — valid outcome, not a system failure.".into(),
        "expired" => "Expired — source no longer present or continuity ended.".into(),
        "superseded" => "Superseded — replaced by newer content for the same identity.".into(),
        other => format!(
            "Lifecycle '{other}'{}",
            resolution
                .map(|r| format!(" ({r})"))
                .unwrap_or_default()
        ),
    }
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
    /// Lifecycle overlay projection (Sprint 192+) — None until durable overlay applied.
    #[serde(default)]
    pub lifecycle_state: Option<String>,
    #[serde(default)]
    pub lifecycle_presented_at: Option<String>,
    #[serde(default)]
    pub lifecycle_resolved_at: Option<String>,
    #[serde(default)]
    pub lifecycle_resolution_type: Option<String>,
    /// Structured surface explanation (Sprint 202+) — None until projected.
    #[serde(default)]
    pub explanation: Option<RecommendationExplanationView>,
    pub authority_effect: String,
}

impl RecommendationItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Active for suggestion surfaces — terminal lifecycle states are history only.
    pub fn is_active_lifecycle(&self) -> bool {
        match self.lifecycle_state.as_deref() {
            None | Some("created") | Some("available") | Some("presented") => true,
            Some("accepted")
            | Some("rejected")
            | Some("expired")
            | Some("superseded") => false,
            Some(_) => true,
        }
    }

    /// Deterministic continuity fingerprint for expire/supersede (not a score).
    pub fn continuity_fingerprint(&self) -> String {
        let evidence: Vec<&str> = self.evidence.iter().map(|e| e.summary.as_str()).collect();
        format!(
            "{}|{}|{}|{}|{}",
            self.kind.as_str(),
            self.title,
            self.reason,
            self.impact,
            evidence.join(";")
        )
    }
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
        let active: Vec<RecommendationItem> = self
            .candidates
            .iter()
            .filter(|c| c.is_active_lifecycle())
            .cloned()
            .collect();
        WorkspaceRecommendationEngineSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            // Surface count reflects still-actionable suggestions.
            candidate_count: active.len(),
            relationship_count: self.relationship_count,
            top_candidates: active.into_iter().take(limit).collect(),
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
