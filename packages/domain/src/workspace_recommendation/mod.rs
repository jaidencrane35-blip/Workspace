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
    /// Structured outcome projection when resolved (Sprint 207+) — immutable feedback.
    #[serde(default)]
    pub outcome: Option<RecommendationOutcomeView>,
    /// Read-only Decision Engine handoff readiness (Sprint 212+) — never creates commands.
    #[serde(default)]
    pub decision_readiness: Option<RecommendationDecisionReadiness>,
    pub authority_effect: String,
}

/// Surface projection of a recommendation outcome (Sprint 207).
///
/// Informational feedback only — never executes, never mutates reasoning, never scores.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationOutcomeView {
    pub outcome_id: String,
    pub recommendation_id: String,
    pub user_decision: String,
    pub result_kind: String,
    pub lifecycle_resolution: Option<String>,
    pub recorded_at: String,
    pub explanation_keys: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub experience_trace_match_keys: Vec<String>,
    /// Always false for recommendation outcomes (rejection/expiry are valid).
    pub is_system_failure: bool,
    pub authority_effect: String,
}

impl RecommendationOutcomeView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Historical continuity entry for Operator / Work (Sprint 207).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationHistoryEntry {
    pub native_id: String,
    pub lifecycle_state: String,
    pub outcome: RecommendationOutcomeView,
    pub resolved_at: Option<String>,
    pub authority_effect: String,
}

impl RecommendationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// One prerequisite for a future Recommendation → Decision Engine handoff (Sprint 212).
///
/// Informational only — satisfaction never creates intents, commands, or Gateway grants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionPrerequisite {
    pub id: String,
    pub label: String,
    pub satisfied: bool,
    pub detail: String,
}

/// Read-only assessment of whether an accepted recommendation has enough continuity
/// for a *future* Decision Engine handoff.
///
/// Never emits handoff, never creates goals/commands, never calls Permission Gateway.
/// Decision Engine remains responsible for intents/goals when that path is wired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionReadiness {
    pub recommendation_id: String,
    pub outcome_id: Option<String>,
    pub lifecycle_state: String,
    /// `incomplete` | `blocked` | `handoff_deferred`
    pub readiness_state: String,
    pub prerequisites: Vec<RecommendationDecisionPrerequisite>,
    pub missing: Vec<String>,
    /// True only when accepted + all prerequisites satisfied. Still does **not** hand off.
    pub ready_for_future_handoff: bool,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionReadiness {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_INCOMPLETE: &'static str = "incomplete";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_HANDOFF_DEFERRED: &'static str = "handoff_deferred";

    pub const PREREQ_PROVENANCE: &'static str = "has_provenance";
    pub const PREREQ_LIFECYCLE: &'static str = "has_lifecycle_completion";
    pub const PREREQ_OUTCOME: &'static str = "has_outcome_history";
    pub const PREREQ_EXPLANATION: &'static str = "has_explanation";
    pub const PREREQ_DECISION_CONTEXT: &'static str = "has_required_decision_context";

    /// Assess handoff prerequisites from a projected recommendation item.
    pub fn assess(item: &RecommendationItem) -> Self {
        let lifecycle_state = item
            .lifecycle_state
            .clone()
            .unwrap_or_else(|| "available".into());
        let outcome_id = item.outcome.as_ref().map(|o| o.outcome_id.clone());

        let has_provenance = !item.evidence.is_empty()
            || !item.attention_reasons.is_empty()
            || item
                .explanation
                .as_ref()
                .map(|e| !e.evidence_refs.is_empty() || !e.explanation_keys.is_empty())
                .unwrap_or(false);
        let has_lifecycle_completion = lifecycle_state == "accepted";
        let has_outcome_history = item.outcome.is_some();
        let has_explanation = item
            .explanation
            .as_ref()
            .map(|e| !e.why_suggested.trim().is_empty())
            .unwrap_or(false)
            || !item.reason.trim().is_empty();
        let has_required_decision_context = item.related_task_id.is_some()
            || item.related_attention_id.is_some()
            || item.related_decision_id.is_some()
            || item
                .related_purpose_label
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

        let prerequisites = vec![
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_PROVENANCE.into(),
                label: "Has provenance".into(),
                satisfied: has_provenance,
                detail: if has_provenance {
                    "Evidence, attention reasons, or explanation refs present.".into()
                } else {
                    "Missing evidence / provenance grounding.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_LIFECYCLE.into(),
                label: "Has lifecycle completion".into(),
                satisfied: has_lifecycle_completion,
                detail: if has_lifecycle_completion {
                    "Accepted as a human decision record.".into()
                } else {
                    format!("Lifecycle is '{lifecycle_state}' — future DE handoff requires accepted.")
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_OUTCOME.into(),
                label: "Has outcome history".into(),
                satisfied: has_outcome_history,
                detail: if has_outcome_history {
                    "Immutable outcome recorded.".into()
                } else {
                    "No recommendation outcome recorded yet.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_EXPLANATION.into(),
                label: "Has explanation".into(),
                satisfied: has_explanation,
                detail: if has_explanation {
                    "Structured why-suggested / reason available.".into()
                } else {
                    "Missing explanation surface.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_DECISION_CONTEXT.into(),
                label: "Has required decision context".into(),
                satisfied: has_required_decision_context,
                detail: if has_required_decision_context {
                    "Task, attention, decision, or purpose context available for a future DE handoff."
                        .into()
                } else {
                    "Missing related task/attention/decision/purpose context — blocks readiness."
                        .into()
                },
            },
        ];
        let missing: Vec<String> = prerequisites
            .iter()
            .filter(|p| !p.satisfied)
            .map(|p| p.id.clone())
            .collect();
        let all_met = missing.is_empty();
        let (readiness_state, ready_for_future_handoff, note) = if !has_lifecycle_completion {
            (
                Self::STATE_INCOMPLETE.into(),
                false,
                "Decision readiness incomplete until the recommendation is accepted with an outcome. \
                 Recommendation Engine never creates Decision Engine intents or commands."
                    .into(),
            )
        } else if !all_met {
            (
                Self::STATE_BLOCKED.into(),
                false,
                "Accepted outcome recorded, but missing prerequisites block future Decision Engine \
                 handoff eligibility. Informational only — no commands created."
                    .into(),
            )
        } else {
            (
                Self::STATE_HANDOFF_DEFERRED.into(),
                true,
                "Prerequisites satisfied for a future Decision Engine handoff contract. Handoff is \
                 intentionally deferred — Decision Engine remains responsible for intents/goals; \
                 Recommendation acceptance does not create commands or call Permission Gateway."
                    .into(),
            )
        };

        Self {
            recommendation_id: item.id.clone(),
            outcome_id,
            lifecycle_state,
            readiness_state,
            prerequisites,
            missing,
            ready_for_future_handoff,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn may_create_decision_commands(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }
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
    /// Terminal / orphan outcomes for historical continuity (not actionable).
    #[serde(default)]
    pub history: Vec<RecommendationHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
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
