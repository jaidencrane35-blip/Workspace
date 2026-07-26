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

    #[error("recommendation decision context/readiness cannot become a handoff or intent")]
    CannotBecomeHandoff,

    #[error("recommendation decision confirmation transition not allowed from {from} to {to}")]
    InvalidConfirmationTransition { from: String, to: String },

    #[error("recommendation decision confirmation cannot create decisions, intents, or execute")]
    ConfirmationCannotCreateAuthority,

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
    /// Structured future-DE intake context (Sprint 217+) — observational only.
    #[serde(default)]
    pub decision_context: Option<RecommendationDecisionContext>,
    /// Read-only Decision Engine handoff readiness (Sprint 212+) — never creates commands.
    #[serde(default)]
    pub decision_readiness: Option<RecommendationDecisionReadiness>,
    /// Explicit RE↔DE ownership / intent boundary (Sprint 222+) — never executes.
    #[serde(default)]
    pub decision_boundary: Option<RecommendationDecisionBoundary>,
    /// Explicit confirmation beyond accept-as-agreement (Sprint 227+) — never creates DE/intent.
    #[serde(default)]
    pub decision_confirmation: Option<RecommendationDecisionConfirmation>,
    /// Typed future-DE intake package after confirmation (Sprint 232+) — never creates DE objects.
    #[serde(default)]
    pub decision_intake: Option<RecommendationDecisionIntakeRequest>,
    /// Integrity inspection of intake for future consumers (Sprint 237+) — never a handoff.
    #[serde(default)]
    pub decision_intake_inspection: Option<RecommendationDecisionIntakeInspection>,
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

/// Structured, non-executing intake snapshot for a *future* Decision Engine (Sprint 217).
///
/// Assembles recommendation identity, explanation/provenance refs, lifecycle, outcome
/// history refs, and user decision — observational only. Never becomes an intent,
/// Decision Engine object, or Gateway grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionContext {
    pub workspace_id: String,
    pub recommendation_id: String,
    pub kind: String,
    pub title: String,
    pub family: String,
    /// Explanation surface reference (recommendation id scoped) — not chain-of-thought.
    pub explanation_ref: Option<String>,
    pub explanation_keys: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub experience_trace_match_keys: Vec<String>,
    pub continuity_fingerprint: String,
    pub lifecycle_state: String,
    pub lifecycle_resolution: Option<String>,
    pub outcome_id: Option<String>,
    /// Outcome ids for this native recommendation (current + prior history).
    pub outcome_history_refs: Vec<String>,
    pub user_decision: Option<String>,
    pub result_kind: Option<String>,
    pub related_task_id: Option<String>,
    pub related_attention_id: Option<String>,
    pub related_decision_id: Option<String>,
    pub related_purpose_label: Option<String>,
    /// Prerequisite ids still missing for a complete future intake.
    pub missing: Vec<String>,
    pub complete: bool,
    /// Always `None` until an explicit future DE intake sprint wires linkage.
    pub decision_engine_object_id: Option<String>,
    /// Always `false` — context/readiness never perform handoff.
    pub handoff_performed: bool,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionContext {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const FAMILY: &'static str = "recommendation_engine";

    /// Assemble a read-only context from a projected recommendation (+ optional history refs).
    pub fn assemble(
        workspace_id: impl Into<String>,
        item: &RecommendationItem,
        outcome_history_refs: &[String],
    ) -> Self {
        let lifecycle_state = item
            .lifecycle_state
            .clone()
            .unwrap_or_else(|| "available".into());
        let explanation = item.explanation.as_ref();
        let explanation_keys = explanation
            .map(|e| e.explanation_keys.clone())
            .unwrap_or_else(|| {
                item.attention_reasons
                    .iter()
                    .map(|r| r.explanation_key.clone())
                    .collect()
            });
        let evidence_refs = explanation
            .map(|e| e.evidence_refs.clone())
            .unwrap_or_else(|| item.evidence.iter().map(|e| e.source_ref.clone()).collect());
        let experience_trace_match_keys = explanation
            .map(|e| e.experience_trace_match_keys.clone())
            .or_else(|| {
                item.outcome
                    .as_ref()
                    .map(|o| o.experience_trace_match_keys.clone())
            })
            .unwrap_or_default();
        let continuity_fingerprint = explanation
            .map(|e| e.continuity_fingerprint.clone())
            .unwrap_or_else(|| item.continuity_fingerprint());
        let explanation_ref = if explanation.is_some() || !item.reason.trim().is_empty() {
            Some(format!("recommendation_explanation:{}", item.id))
        } else {
            None
        };

        let mut outcome_history_refs: Vec<String> = outcome_history_refs.to_vec();
        if let Some(outcome) = &item.outcome {
            if !outcome_history_refs
                .iter()
                .any(|id| id == &outcome.outcome_id)
            {
                outcome_history_refs.push(outcome.outcome_id.clone());
            }
        }
        outcome_history_refs.sort();
        outcome_history_refs.dedup();

        let has_provenance = !evidence_refs.is_empty()
            || !item.attention_reasons.is_empty()
            || !explanation_keys.is_empty();
        let has_lifecycle_completion = lifecycle_state == "accepted";
        let has_outcome_history = !outcome_history_refs.is_empty();
        let has_explanation = explanation_ref.is_some();
        let has_required_decision_context = item.related_task_id.is_some()
            || item.related_attention_id.is_some()
            || item.related_decision_id.is_some()
            || item
                .related_purpose_label
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

        let mut missing = Vec::new();
        if !has_provenance {
            missing.push(RecommendationDecisionReadiness::PREREQ_PROVENANCE.into());
        }
        if !has_lifecycle_completion {
            missing.push(RecommendationDecisionReadiness::PREREQ_LIFECYCLE.into());
        }
        if !has_outcome_history {
            missing.push(RecommendationDecisionReadiness::PREREQ_OUTCOME.into());
        }
        if !has_explanation {
            missing.push(RecommendationDecisionReadiness::PREREQ_EXPLANATION.into());
        }
        if !has_required_decision_context {
            missing.push(RecommendationDecisionReadiness::PREREQ_DECISION_CONTEXT.into());
        }

        let complete = missing.is_empty();
        let note = if complete {
            "Decision context assembled for future Decision Engine intake. Observational only — \
             not a Decision Engine object, intent, or handoff."
                .into()
        } else {
            format!(
                "Decision context incomplete (missing: {}). Read-only assembly — never creates \
                 intents or Decision Engine objects.",
                missing.join(", ")
            )
        };

        Self {
            workspace_id: workspace_id.into(),
            recommendation_id: item.id.clone(),
            kind: item.kind.as_str().into(),
            title: item.title.clone(),
            family: Self::FAMILY.into(),
            explanation_ref,
            explanation_keys,
            evidence_refs,
            experience_trace_match_keys,
            continuity_fingerprint,
            lifecycle_state,
            lifecycle_resolution: item.lifecycle_resolution_type.clone(),
            outcome_id: item.outcome.as_ref().map(|o| o.outcome_id.clone()),
            outcome_history_refs,
            user_decision: item.outcome.as_ref().map(|o| o.user_decision.clone()),
            result_kind: item.outcome.as_ref().map(|o| o.result_kind.clone()),
            related_task_id: item.related_task_id.clone(),
            related_attention_id: item.related_attention_id.clone(),
            related_decision_id: item.related_decision_id.clone(),
            related_purpose_label: item.related_purpose_label.clone(),
            missing,
            complete,
            decision_engine_object_id: None,
            handoff_performed: false,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    /// Context never upgrades into a DE handoff — even when `complete` is true.
    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_become_decision_engine_object(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }
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
        let context = RecommendationDecisionContext::assemble("", item, &[]);
        Self::assess_from_context(&context)
    }

    /// Assess readiness from an assembled decision context (single source of missing fields).
    pub fn assess_from_context(context: &RecommendationDecisionContext) -> Self {
        let has_provenance = !context
            .missing
            .iter()
            .any(|m| m == Self::PREREQ_PROVENANCE);
        let has_lifecycle_completion = !context.missing.iter().any(|m| m == Self::PREREQ_LIFECYCLE);
        let has_outcome_history = !context.missing.iter().any(|m| m == Self::PREREQ_OUTCOME);
        let has_explanation = !context
            .missing
            .iter()
            .any(|m| m == Self::PREREQ_EXPLANATION);
        let has_required_decision_context = !context
            .missing
            .iter()
            .any(|m| m == Self::PREREQ_DECISION_CONTEXT);

        let prerequisites = vec![
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_PROVENANCE.into(),
                label: "Has provenance".into(),
                satisfied: has_provenance,
                detail: if has_provenance {
                    format!(
                        "Provenance refs present ({} evidence ref(s), {} explanation key(s)).",
                        context.evidence_refs.len(),
                        context.explanation_keys.len()
                    )
                } else {
                    "Missing evidence / provenance grounding in decision context.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_LIFECYCLE.into(),
                label: "Has lifecycle completion".into(),
                satisfied: has_lifecycle_completion,
                detail: if has_lifecycle_completion {
                    "Accepted as a human decision record.".into()
                } else {
                    format!(
                        "Lifecycle is '{}' — future DE handoff requires accepted.",
                        context.lifecycle_state
                    )
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_OUTCOME.into(),
                label: "Has outcome history".into(),
                satisfied: has_outcome_history,
                detail: if has_outcome_history {
                    format!(
                        "Outcome history refs: {}.",
                        context.outcome_history_refs.join(", ")
                    )
                } else {
                    "No recommendation outcome history refs in decision context.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_EXPLANATION.into(),
                label: "Has explanation".into(),
                satisfied: has_explanation,
                detail: if has_explanation {
                    format!(
                        "Explanation ref: {}.",
                        context.explanation_ref.as_deref().unwrap_or("present")
                    )
                } else {
                    "Missing explanation reference in decision context.".into()
                },
            },
            RecommendationDecisionPrerequisite {
                id: Self::PREREQ_DECISION_CONTEXT.into(),
                label: "Has required decision context".into(),
                satisfied: has_required_decision_context,
                detail: if has_required_decision_context {
                    "Related task/attention/decision/purpose fields present in context.".into()
                } else {
                    "Missing related task/attention/decision/purpose context — blocks readiness."
                        .into()
                },
            },
        ];
        let missing = context.missing.clone();
        let all_met = missing.is_empty() && context.complete;
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
                "Accepted outcome recorded, but incomplete decision context blocks future Decision \
                 Engine handoff eligibility. Informational only — no commands created."
                    .into(),
            )
        } else {
            (
                Self::STATE_HANDOFF_DEFERRED.into(),
                true,
                "Decision context complete for a future Decision Engine intake. Handoff is \
                 intentionally deferred — readiness cannot become a handoff; Decision Engine \
                 remains responsible for intents/goals."
                    .into(),
            )
        };

        Self {
            recommendation_id: context.recommendation_id.clone(),
            outcome_id: context.outcome_id.clone(),
            lifecycle_state: context.lifecycle_state.clone(),
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

    /// Readiness never upgrades into a DE handoff — even when `ready_for_future_handoff`.
    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
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

/// Explicit separation between recommendation acceptance and future Decision Engine intake
/// (Sprint 222).
///
/// Classifies the human decision and ownership so operators cannot confuse accept with
/// intent creation, Decision Engine objects, or execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionBoundary {
    pub recommendation_id: String,
    /// `recommendation_only` | `context_ready` | `awaiting_decision_engine_intake`
    pub transition_state: String,
    /// Always `handoff_not_performed` until an explicit future intake sprint.
    pub handoff_state: String,
    /// What the human decision means — never execution authorization.
    /// `none` | `recommendation_agreement` | `recommendation_rejection` | `recommendation_terminal`
    pub user_intent_kind: String,
    pub recommendation_owner: String,
    pub decision_owner: String,
    pub execution_owner: String,
    pub governance_owner: String,
    pub experience_owner: String,
    /// True when lifecycle is accepted — still a recommendation decision only.
    pub accepted_as_recommendation_decision: bool,
    pub creates_intent: bool,
    pub creates_decision_engine_object: bool,
    pub grants_execution_authority: bool,
    pub handoff_performed: bool,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionBoundary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub const STATE_RECOMMENDATION_ONLY: &'static str = "recommendation_only";
    pub const STATE_CONTEXT_READY: &'static str = "context_ready";
    pub const STATE_AWAITING_DECISION_ENGINE_INTAKE: &'static str =
        "awaiting_decision_engine_intake";
    pub const HANDOFF_NOT_PERFORMED: &'static str = "handoff_not_performed";

    pub const INTENT_NONE: &'static str = "none";
    pub const INTENT_AGREEMENT: &'static str = "recommendation_agreement";
    pub const INTENT_REJECTION: &'static str = "recommendation_rejection";
    pub const INTENT_TERMINAL: &'static str = "recommendation_terminal";

    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const OWNER_DECISION: &'static str = "decision_engine";
    pub const OWNER_GATEWAY: &'static str = "permission_gateway";
    pub const OWNER_GOVERNANCE: &'static str = "governance";
    pub const OWNER_EXPERIENCE: &'static str = "experience";

    /// Derive boundary from assembled context + readiness (observational only).
    pub fn from_context_and_readiness(
        context: &RecommendationDecisionContext,
        readiness: &RecommendationDecisionReadiness,
    ) -> Self {
        let user_decision = context.user_decision.as_deref();
        let accepted = context.lifecycle_state == "accepted"
            || user_decision == Some("accepted");
        let user_intent_kind = match user_decision {
            Some("accepted") => Self::INTENT_AGREEMENT,
            Some("rejected") => Self::INTENT_REJECTION,
            Some("expired") | Some("superseded") => Self::INTENT_TERMINAL,
            _ if accepted => Self::INTENT_AGREEMENT,
            _ => Self::INTENT_NONE,
        };

        let transition_state = if readiness.ready_for_future_handoff && context.complete {
            Self::STATE_AWAITING_DECISION_ENGINE_INTAKE
        } else if accepted && context.complete {
            Self::STATE_CONTEXT_READY
        } else if accepted && !context.complete {
            // Accepted agreement recorded, but intake context still incomplete.
            Self::STATE_RECOMMENDATION_ONLY
        } else {
            Self::STATE_RECOMMENDATION_ONLY
        };

        let note = match transition_state {
            s if s == Self::STATE_AWAITING_DECISION_ENGINE_INTAKE => {
                "Boundary: recommendation agreement recorded; context ready for *future* Decision \
                 Engine intake. Handoff not performed — Decision Engine owns goals/intents; \
                 Gateway owns execution."
                    .into()
            }
            s if s == Self::STATE_CONTEXT_READY => {
                "Boundary: decision context is complete, but readiness does not authorize intake. \
                 Handoff not performed; accepted recommendation is not an intent."
                    .into()
            }
            _ if accepted => {
                "Boundary: accepted as a recommendation decision only (agreement with a suggestion). \
                 Not a request for action, not an intent, not execution authority. Handoff not performed."
                    .into()
            }
            _ => {
                "Boundary: recommendation remains Recommendation Engine–owned. No Decision Engine \
                 object, intent, or execution authority."
                    .into()
            }
        };

        Self {
            recommendation_id: context.recommendation_id.clone(),
            transition_state: transition_state.into(),
            handoff_state: Self::HANDOFF_NOT_PERFORMED.into(),
            user_intent_kind: user_intent_kind.into(),
            recommendation_owner: Self::OWNER_RECOMMENDATION.into(),
            decision_owner: Self::OWNER_DECISION.into(),
            execution_owner: Self::OWNER_GATEWAY.into(),
            governance_owner: Self::OWNER_GOVERNANCE.into(),
            experience_owner: Self::OWNER_EXPERIENCE.into(),
            accepted_as_recommendation_decision: accepted,
            creates_intent: false,
            creates_decision_engine_object: false,
            grants_execution_authority: false,
            handoff_performed: false,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_intent(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_decision_engine_object(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_authorize_execution(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_create_decision_engine_object(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }

    /// Rejection guards: acceptance / completeness / readiness never cross ownership lines.
    pub fn assert_rejection_guards(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.creates_intent
            || self.creates_decision_engine_object
            || self.grants_execution_authority
            || self.handoff_performed
            || self.handoff_state != Self::HANDOFF_NOT_PERFORMED
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        if self.accepted_as_recommendation_decision && self.user_intent_kind == Self::INTENT_AGREEMENT
        {
            // Accepted ≠ intent / execution — already encoded by flags above.
        }
        Ok(())
    }
}

/// Explicit user confirmation between recommendation acceptance and future DE creation
/// (Sprint 227).
///
/// Accept remains agreement-only. Confirmation records whether the user wants a *future*
/// Decision Engine consideration — still never creates DE objects, intents, or Gateway grants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionConfirmation {
    pub recommendation_id: String,
    /// `not_required` | `required` | `confirmed` | `declined`
    pub confirmation_state: String,
    /// `agreement_only` | `create_future_decision` | `request_action_review`
    pub confirmation_intent: String,
    pub recommendation_owner: String,
    pub confirmation_owner: String,
    pub decision_owner: String,
    pub execution_owner: String,
    pub creates_decision_engine_object: bool,
    pub creates_intent: bool,
    pub grants_execution_authority: bool,
    pub handoff_performed: bool,
    pub confirmed_at: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionConfirmation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub const STATE_NOT_REQUIRED: &'static str = "not_required";
    pub const STATE_REQUIRED: &'static str = "required";
    pub const STATE_CONFIRMED: &'static str = "confirmed";
    pub const STATE_DECLINED: &'static str = "declined";

    pub const INTENT_AGREEMENT_ONLY: &'static str = "agreement_only";
    pub const INTENT_CREATE_FUTURE_DECISION: &'static str = "create_future_decision";
    pub const INTENT_REQUEST_ACTION_REVIEW: &'static str = "request_action_review";

    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const OWNER_USER: &'static str = "user";
    pub const OWNER_DECISION: &'static str = "decision_engine";
    pub const OWNER_GATEWAY: &'static str = "permission_gateway";

    /// Derive initial confirmation from boundary — accept never sets `confirmed`.
    pub fn derive_from_boundary(boundary: &RecommendationDecisionBoundary) -> Self {
        let (confirmation_state, note) = if boundary.transition_state
            == RecommendationDecisionBoundary::STATE_AWAITING_DECISION_ENGINE_INTAKE
        {
            (
                Self::STATE_REQUIRED,
                "Confirmation required for future Decision Engine consideration. Accept remains \
                 agreement only — confirming still does not create a Decision object, intent, or \
                 execution authority."
                    .into(),
            )
        } else {
            (
                Self::STATE_NOT_REQUIRED,
                "Confirmation not required. Recommendation acceptance (if any) is agreement only; \
                 no future Decision Engine intake requested."
                    .into(),
            )
        };
        Self {
            recommendation_id: boundary.recommendation_id.clone(),
            confirmation_state: confirmation_state.into(),
            confirmation_intent: Self::INTENT_AGREEMENT_ONLY.into(),
            recommendation_owner: Self::OWNER_RECOMMENDATION.into(),
            confirmation_owner: Self::OWNER_USER.into(),
            decision_owner: Self::OWNER_DECISION.into(),
            execution_owner: Self::OWNER_GATEWAY.into(),
            creates_decision_engine_object: false,
            creates_intent: false,
            grants_execution_authority: false,
            handoff_performed: false,
            confirmed_at: None,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn allows_transition(&self, to: &str) -> bool {
        match (self.confirmation_state.as_str(), to) {
            (Self::STATE_REQUIRED, Self::STATE_CONFIRMED)
            | (Self::STATE_REQUIRED, Self::STATE_DECLINED) => true,
            (from, to) if from == to => true,
            _ => false,
        }
    }

    /// Record user confirmation for *future* DE consideration — never creates DE/intent.
    pub fn confirm(
        &mut self,
        confirmation_intent: &str,
        at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if !matches!(
            confirmation_intent,
            Self::INTENT_CREATE_FUTURE_DECISION | Self::INTENT_REQUEST_ACTION_REVIEW
        ) {
            return Err(WorkspaceRecommendationEngineError::ConfirmationCannotCreateAuthority);
        }
        if !self.allows_transition(Self::STATE_CONFIRMED) {
            return Err(WorkspaceRecommendationEngineError::InvalidConfirmationTransition {
                from: self.confirmation_state.clone(),
                to: Self::STATE_CONFIRMED.into(),
            });
        }
        self.confirmation_state = Self::STATE_CONFIRMED.into();
        self.confirmation_intent = confirmation_intent.into();
        self.confirmed_at = Some(at.into());
        self.creates_decision_engine_object = false;
        self.creates_intent = false;
        self.grants_execution_authority = false;
        self.handoff_performed = false;
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.note = "User confirmed desire for future Decision Engine consideration. Confirmation \
                     is not Decision creation, not an intent, and not execution authorization. \
                     Handoff not performed."
            .into();
        Ok(())
    }

    /// Decline future DE consideration — accept/agreement history remains intact.
    pub fn decline(
        &mut self,
        at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if !self.allows_transition(Self::STATE_DECLINED) {
            return Err(WorkspaceRecommendationEngineError::InvalidConfirmationTransition {
                from: self.confirmation_state.clone(),
                to: Self::STATE_DECLINED.into(),
            });
        }
        self.confirmation_state = Self::STATE_DECLINED.into();
        self.confirmation_intent = Self::INTENT_AGREEMENT_ONLY.into();
        self.confirmed_at = Some(at.into());
        self.creates_decision_engine_object = false;
        self.creates_intent = false;
        self.grants_execution_authority = false;
        self.handoff_performed = false;
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.note = "User declined future Decision Engine consideration. Recommendation agreement \
                     (if any) remains; no Decision object, intent, or execution authority."
            .into();
        Ok(())
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::ConfirmationCannotCreateAuthority)
    }

    pub fn attempt_create_decision_engine_object(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::ConfirmationCannotCreateAuthority)
    }

    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_create_decision_engine_object(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }

    pub fn assert_non_authoritative(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.creates_intent
            || self.creates_decision_engine_object
            || self.grants_execution_authority
            || self.handoff_performed
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
        {
            return Err(WorkspaceRecommendationEngineError::ConfirmationCannotCreateAuthority);
        }
        Ok(())
    }
}

/// Typed RE → future-DE intake package after user confirmation (Sprint 232).
///
/// Assembled only when confirmation is `confirmed` with a future-decision intent.
/// Never creates `DecisionCandidate`, intents, commands, or Gateway grants.
/// `handoff_performed` remains false — package only, not DE ownership transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakeRequest {
    pub recommendation_id: String,
    pub workspace_id: String,
    pub confirmation_intent: String,
    pub confirmed_at: String,
    pub kind: String,
    pub title: String,
    /// Informational statement for a future DE — not a Goal/Intent object.
    pub suggested_goal_statement: String,
    pub continuity_fingerprint: String,
    pub explanation_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub explanation_keys: Vec<String>,
    pub outcome_id: Option<String>,
    pub related_task_id: Option<String>,
    pub related_attention_id: Option<String>,
    pub related_decision_id: Option<String>,
    /// Always `None` — intake does not create DE objects.
    pub decision_engine_object_id: Option<String>,
    /// `requested` — emitted for future adapter consumption; not owned by DE yet.
    pub intake_state: String,
    pub handoff_performed: bool,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakeRequest {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_REQUESTED: &'static str = "requested";

    /// Assemble intake only when confirmation + context/readiness gates pass.
    pub fn try_assemble(
        context: &RecommendationDecisionContext,
        readiness: &RecommendationDecisionReadiness,
        confirmation: &RecommendationDecisionConfirmation,
    ) -> Option<Self> {
        if confirmation.confirmation_state != RecommendationDecisionConfirmation::STATE_CONFIRMED {
            return None;
        }
        if !matches!(
            confirmation.confirmation_intent.as_str(),
            RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION
                | RecommendationDecisionConfirmation::INTENT_REQUEST_ACTION_REVIEW
        ) {
            return None;
        }
        if !context.complete || !readiness.ready_for_future_handoff {
            return None;
        }
        let confirmed_at = confirmation.confirmed_at.clone()?;
        let suggested_goal_statement = format!(
            "{} — {}",
            context.title,
            context
                .explanation_ref
                .as_deref()
                .unwrap_or("recommendation agreement recorded")
        );
        Some(Self {
            recommendation_id: context.recommendation_id.clone(),
            workspace_id: context.workspace_id.clone(),
            confirmation_intent: confirmation.confirmation_intent.clone(),
            confirmed_at,
            kind: context.kind.clone(),
            title: context.title.clone(),
            suggested_goal_statement,
            continuity_fingerprint: context.continuity_fingerprint.clone(),
            explanation_ref: context.explanation_ref.clone(),
            evidence_refs: context.evidence_refs.clone(),
            explanation_keys: context.explanation_keys.clone(),
            outcome_id: context.outcome_id.clone(),
            related_task_id: context.related_task_id.clone(),
            related_attention_id: context.related_attention_id.clone(),
            related_decision_id: context.related_decision_id.clone(),
            decision_engine_object_id: None,
            intake_state: Self::STATE_REQUESTED.into(),
            handoff_performed: false,
            note: "Intake request assembled for future Decision Engine consideration. \
                   Not a Decision Engine object, not an intent, not execution authority. \
                   Handoff not performed — Decision Engine remains owner of object creation."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_decision_engine_object(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_intent(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn may_create_decision_engine_object(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }

    pub fn assert_non_authoritative(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.handoff_performed
            || self.decision_engine_object_id.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.intake_state != Self::STATE_REQUESTED
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Read-only integrity inspection of an intake request for a future consumer (Sprint 237).
///
/// Proves a consumer may safely inspect intake without that inspection becoming a handoff
/// path or DE ownership transfer. Does not create Decision Engine objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakeInspection {
    pub recommendation_id: String,
    /// `valid` | `stale_context` | `binding_failed` | `fingerprint_mismatch`
    /// | `provenance_mismatch` | `non_authoritative_violation`
    pub inspection_state: String,
    /// True only when all integrity checks pass and intake remains non-authoritative.
    pub safe_to_inspect: bool,
    pub fingerprint_matches: bool,
    pub confirmation_bound: bool,
    pub context_compatible: bool,
    pub provenance_intact: bool,
    pub ownership_intact: bool,
    pub findings: Vec<String>,
    pub handoff_performed: bool,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakeInspection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_VALID: &'static str = "valid";
    pub const STATE_STALE_CONTEXT: &'static str = "stale_context";
    pub const STATE_BINDING_FAILED: &'static str = "binding_failed";
    pub const STATE_FINGERPRINT_MISMATCH: &'static str = "fingerprint_mismatch";
    pub const STATE_PROVENANCE_MISMATCH: &'static str = "provenance_mismatch";
    pub const STATE_NON_AUTHORITATIVE_VIOLATION: &'static str = "non_authoritative_violation";

    /// Re-bind intake to live context/confirmation/readiness without transferring ownership.
    pub fn verify(
        intake: &RecommendationDecisionIntakeRequest,
        context: &RecommendationDecisionContext,
        confirmation: &RecommendationDecisionConfirmation,
        readiness: &RecommendationDecisionReadiness,
    ) -> Self {
        let mut findings = Vec::new();

        let non_auth_ok = intake.assert_non_authoritative().is_ok();
        if !non_auth_ok {
            findings.push(
                "Intake violates non-authoritative contract (handoff, DE object, or state)."
                    .into(),
            );
        }

        let fingerprint_matches =
            intake.continuity_fingerprint == context.continuity_fingerprint;
        if !fingerprint_matches {
            findings.push("Continuity fingerprint does not match live decision context.".into());
        }

        let confirmation_bound = intake.recommendation_id == confirmation.recommendation_id
            && confirmation.confirmation_state
                == RecommendationDecisionConfirmation::STATE_CONFIRMED
            && confirmation.confirmation_intent == intake.confirmation_intent
            && confirmation.confirmed_at.as_deref() == Some(intake.confirmed_at.as_str());
        if !confirmation_bound {
            findings.push(
                "Confirmation binding failed — intake is not bound to a live confirmed decision."
                    .into(),
            );
        }

        let context_compatible = context.complete
            && readiness.ready_for_future_handoff
            && context.recommendation_id == intake.recommendation_id
            && context.workspace_id == intake.workspace_id
            && context.kind == intake.kind
            && context.title == intake.title;
        if !context_compatible {
            findings.push(
                "Context incompatible or stale — complete/ready gates or identity fields mismatch."
                    .into(),
            );
        }

        let provenance_intact = intake.explanation_ref == context.explanation_ref
            && intake.evidence_refs == context.evidence_refs
            && intake.explanation_keys == context.explanation_keys
            && intake.outcome_id == context.outcome_id
            && intake.related_task_id == context.related_task_id
            && intake.related_attention_id == context.related_attention_id
            && intake.related_decision_id == context.related_decision_id;
        if !provenance_intact {
            findings.push(
                "Provenance refs drifted from live decision context (evidence/explanation/related)."
                    .into(),
            );
        }

        let ownership_intact = confirmation.recommendation_owner
            == RecommendationDecisionConfirmation::OWNER_RECOMMENDATION
            && intake.intake_state == RecommendationDecisionIntakeRequest::STATE_REQUESTED
            && intake.decision_engine_object_id.is_none()
            && !intake.handoff_performed
            && confirmation.decision_owner == RecommendationDecisionConfirmation::OWNER_DECISION;
        if !ownership_intact {
            findings.push(
                "Ownership integrity failed — RE must own intake; DE must not own it yet.".into(),
            );
        }

        let safe_to_inspect = non_auth_ok
            && fingerprint_matches
            && confirmation_bound
            && context_compatible
            && provenance_intact
            && ownership_intact;

        let inspection_state = if !non_auth_ok || !ownership_intact {
            Self::STATE_NON_AUTHORITATIVE_VIOLATION
        } else if !confirmation_bound {
            Self::STATE_BINDING_FAILED
        } else if !fingerprint_matches {
            Self::STATE_FINGERPRINT_MISMATCH
        } else if !provenance_intact {
            Self::STATE_PROVENANCE_MISMATCH
        } else if !context_compatible {
            Self::STATE_STALE_CONTEXT
        } else {
            Self::STATE_VALID
        };

        let note = if safe_to_inspect {
            "Intake inspection valid — future consumer may inspect this package. Inspection is \
             not a handoff, not DE ownership, and not execution authority."
                .into()
        } else {
            format!(
                "Intake inspection failed ({inspection_state}). Package must not be treated as \
                 handoff-ready or Decision Engine–owned. Findings: {}",
                findings.join(" ")
            )
        };

        Self {
            recommendation_id: intake.recommendation_id.clone(),
            inspection_state: inspection_state.into(),
            safe_to_inspect,
            fingerprint_matches,
            confirmation_bound,
            context_compatible,
            provenance_intact,
            ownership_intact,
            findings,
            handoff_performed: false,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn attempt_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_decision_engine_object(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_create_intent(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn may_create_decision_engine_object(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_provenance(&self) -> bool {
        false
    }

    /// Assert this inspection record itself carries no handoff/execution authority.
    /// Does **not** mean a consumer may hand off — use `attempt_handoff()` for that (always fails).
    pub fn assert_inspection_is_not_handoff(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.handoff_performed || self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
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
