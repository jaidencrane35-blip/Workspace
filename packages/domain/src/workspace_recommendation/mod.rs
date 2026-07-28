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

    #[error("recommendation decision engine acceptance transition not allowed from {from} to {to}")]
    InvalidAcceptanceTransition { from: String, to: String },

    #[error("recommendation decision engine acceptance cannot transfer ownership, create DE objects, or execute")]
    AcceptanceCannotCreateAuthority,

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
    /// Versioned intake package identity for future consumers (Sprint 242+) — never transfer.
    #[serde(default)]
    pub decision_intake_compatibility: Option<RecommendationDecisionIntakeCompatibility>,
    /// Explicit denial that compatibility is not proceed permission (Sprint 247+) — never handoff.
    #[serde(default)]
    pub decision_intake_proceed_denial: Option<RecommendationDecisionIntakeProceedDenial>,
    /// Frozen intake package digest after proceed denial (Sprint 252+) — never adapter/handoff.
    #[serde(default)]
    pub decision_intake_package_seal: Option<RecommendationDecisionIntakePackageSeal>,
    /// Prepared RE→future-DE adapter path (Sprint 257+) — never invokes adapter or creates DE objects.
    #[serde(default)]
    pub decision_intake_adapter_preparation: Option<RecommendationDecisionIntakeAdapterPreparation>,
    /// Non-executing handoff request to future DE (Sprint 262+) — request ≠ performed / DE object.
    #[serde(default)]
    pub decision_handoff_request: Option<RecommendationDecisionHandoffRequest>,
    /// Non-executing DE acceptance boundary (Sprint 267+) — accept ≠ ownership transfer / DE object.
    #[serde(default)]
    pub decision_engine_acceptance: Option<RecommendationDecisionEngineAcceptance>,
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

/// Versioned package identity for an inspectable intake (Sprint 242).
///
/// Pins `recommendation_decision_intake:v1` for a future consumer without authorizing
/// migrate, transfer, DE ownership, or handoff. Compatible ≠ handoff-ready.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakeCompatibility {
    pub recommendation_id: String,
    pub contract_family: String,
    pub contract_version: String,
    pub schema_version: i32,
    pub producer: String,
    /// Declared future reader — never current owner.
    pub declared_consumer: String,
    pub required_field_floor: Vec<String>,
    pub inspection_valid: bool,
    pub field_floor_satisfied: bool,
    pub version_current: bool,
    /// True only when inspection is valid, version current, field floor met, ownership still RE.
    pub compatible: bool,
    pub may_migrate: bool,
    pub transfer_authorized: bool,
    pub handoff_performed: bool,
    pub decision_engine_object_id: Option<String>,
    pub findings: Vec<String>,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakeCompatibility {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const CONTRACT_FAMILY: &'static str = "recommendation_decision_intake";
    pub const CONTRACT_VERSION: &'static str = "recommendation_decision_intake:v1";
    pub const SCHEMA_VERSION: i32 = 1;
    pub const PRODUCER: &'static str = "recommendation_engine";
    pub const DECLARED_CONSUMER: &'static str = "decision_engine";

    pub fn required_field_floor() -> Vec<String> {
        vec![
            "recommendation_id".into(),
            "workspace_id".into(),
            "confirmation_intent".into(),
            "confirmed_at".into(),
            "kind".into(),
            "title".into(),
            "suggested_goal_statement".into(),
            "continuity_fingerprint".into(),
            "intake_state".into(),
        ]
    }

    /// Derive compatibility from a verified inspection + intake package.
    /// Invalid/stale inspection never yields `compatible = true`.
    pub fn derive_from_inspection(
        intake: &RecommendationDecisionIntakeRequest,
        inspection: &RecommendationDecisionIntakeInspection,
    ) -> Self {
        Self::evaluate(
            intake,
            inspection,
            Self::CONTRACT_VERSION,
            Self::SCHEMA_VERSION,
        )
    }

    /// Evaluate against an expected contract/schema pin (for mismatch tests and future pins).
    pub fn evaluate(
        intake: &RecommendationDecisionIntakeRequest,
        inspection: &RecommendationDecisionIntakeInspection,
        expected_contract_version: &str,
        expected_schema_version: i32,
    ) -> Self {
        let mut findings = Vec::new();
        let inspection_valid = inspection.safe_to_inspect
            && inspection.inspection_state == RecommendationDecisionIntakeInspection::STATE_VALID;
        if !inspection_valid {
            findings.push(
                "Intake inspection is not valid — package must not progress beyond inspect."
                    .into(),
            );
        }

        let version_current = expected_contract_version == Self::CONTRACT_VERSION
            && expected_schema_version == Self::SCHEMA_VERSION;
        if !version_current {
            findings.push(format!(
                "Contract/schema mismatch: expected {expected_contract_version}#{expected_schema_version}, \
                 producer emits {}#{}",
                Self::CONTRACT_VERSION,
                Self::SCHEMA_VERSION
            ));
        }

        let field_floor_satisfied = !intake.recommendation_id.is_empty()
            && !intake.workspace_id.is_empty()
            && !intake.confirmation_intent.is_empty()
            && !intake.confirmed_at.is_empty()
            && !intake.kind.is_empty()
            && !intake.title.is_empty()
            && !intake.suggested_goal_statement.is_empty()
            && !intake.continuity_fingerprint.is_empty()
            && intake.intake_state == RecommendationDecisionIntakeRequest::STATE_REQUESTED;
        if !field_floor_satisfied {
            findings.push("Required intake field floor not satisfied for schema v1.".into());
        }

        let ownership_ok = intake.assert_non_authoritative().is_ok()
            && intake.decision_engine_object_id.is_none()
            && !intake.handoff_performed;
        if !ownership_ok {
            findings.push(
                "Ownership/non-authoritative violation — RE must retain package; DE must not own."
                    .into(),
            );
        }

        let compatible =
            inspection_valid && version_current && field_floor_satisfied && ownership_ok;

        let note = if compatible {
            "Intake package compatible with recommendation_decision_intake:v1. Future consumer \
             may pin this identity. Compatible is not transfer, migrate, handoff, or DE ownership."
                .into()
        } else {
            format!(
                "Intake package incompatible for future consumer pin. Findings: {}",
                findings.join(" ")
            )
        };

        Self {
            recommendation_id: intake.recommendation_id.clone(),
            contract_family: Self::CONTRACT_FAMILY.into(),
            contract_version: Self::CONTRACT_VERSION.into(),
            schema_version: Self::SCHEMA_VERSION,
            producer: Self::PRODUCER.into(),
            declared_consumer: Self::DECLARED_CONSUMER.into(),
            required_field_floor: Self::required_field_floor(),
            inspection_valid,
            field_floor_satisfied,
            version_current,
            compatible,
            may_migrate: false,
            transfer_authorized: false,
            handoff_performed: false,
            decision_engine_object_id: None,
            findings,
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

    pub fn attempt_migrate(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_transfer(&self) -> Result<(), WorkspaceRecommendationEngineError> {
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

    pub fn assert_non_transfer(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.transfer_authorized
            || self.handoff_performed
            || self.may_migrate
            || self.decision_engine_object_id.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Explicit denial that intake compatibility is not proceed/consume permission (Sprint 247).
///
/// Closes the misread that `compatible=true` + `declared_consumer=decision_engine`
/// authorizes adapter invocation, DE ownership, or handoff. Always denies proceed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakeProceedDenial {
    pub recommendation_id: String,
    pub compatibility_compatible: bool,
    pub contract_version: String,
    pub current_owner: String,
    /// Always pin-only — never current owner.
    pub declared_consumer_role: String,
    /// `identity_pin_only` | `incompatible_blocked`
    pub eligibility_state: String,
    pub proceed_authorized: bool,
    pub consume_authorized: bool,
    pub adapter_invokable: bool,
    pub permission_effect: String,
    pub denial_reasons: Vec<String>,
    pub handoff_performed: bool,
    pub decision_engine_object_id: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakeProceedDenial {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PERMISSION_EFFECT_NONE: &'static str = "none";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const CONSUMER_ROLE_PIN_ONLY: &'static str = "future_reader_pin_only";
    pub const STATE_IDENTITY_PIN_ONLY: &'static str = "identity_pin_only";
    pub const STATE_INCOMPATIBLE_BLOCKED: &'static str = "incompatible_blocked";

    /// Derive proceed denial from compatibility. Compatible never upgrades to proceed.
    pub fn derive_from_compatibility(
        compatibility: &RecommendationDecisionIntakeCompatibility,
    ) -> Self {
        let mut denial_reasons = vec![
            "compatibility_is_not_permission".into(),
            "proceed_requires_future_decision_engine_adapter".into(),
            "recommendation_engine_retains_ownership".into(),
        ];
        if !compatibility.compatible {
            denial_reasons.insert(
                0,
                "identity_mismatch_or_invalid_inspection_blocks_progression".into(),
            );
        }
        if !compatibility.version_current {
            denial_reasons.insert(0, "contract_version_mismatch_blocks_progression".into());
        }

        let eligibility_state = if compatibility.compatible {
            Self::STATE_IDENTITY_PIN_ONLY
        } else {
            Self::STATE_INCOMPATIBLE_BLOCKED
        };

        let note = if compatibility.compatible {
            "Intake identity pin is compatible. Compatible is not permission to proceed, \
             consume, invoke an adapter, transfer ownership, or create Decision Engine objects."
                .into()
        } else {
            "Intake identity is incompatible or blocked. Progression denied; no proceed, \
             consume, adapter, handoff, or DE ownership."
                .into()
        };

        Self {
            recommendation_id: compatibility.recommendation_id.clone(),
            compatibility_compatible: compatibility.compatible,
            contract_version: compatibility.contract_version.clone(),
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            declared_consumer_role: Self::CONSUMER_ROLE_PIN_ONLY.into(),
            eligibility_state: eligibility_state.into(),
            proceed_authorized: false,
            consume_authorized: false,
            adapter_invokable: false,
            permission_effect: Self::PERMISSION_EFFECT_NONE.into(),
            denial_reasons,
            handoff_performed: false,
            decision_engine_object_id: None,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_proceed(&self) -> bool {
        false
    }

    pub fn may_consume(&self) -> bool {
        false
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn attempt_authorize_proceed(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_consume(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
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

    /// Assert compatibility pin never grants proceed/consume/adapter permission.
    pub fn assert_compatible_is_not_permission(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.proceed_authorized
            || self.consume_authorized
            || self.adapter_invokable
            || self.handoff_performed
            || self.decision_engine_object_id.is_some()
            || self.permission_effect != Self::PERMISSION_EFFECT_NONE
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Frozen RE intake package snapshot after proceed denial (Sprint 252).
///
/// Digests the intake body so a future consumer cannot silently consume a drifted
/// package. Seal does not authorize proceed, adapter, DE ownership, or handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakePackageSeal {
    pub recommendation_id: String,
    pub intake_package_digest: String,
    pub continuity_fingerprint_at_seal: String,
    pub contract_version: String,
    pub confirmation_intent: String,
    pub confirmed_at: String,
    pub eligibility_state: String,
    pub sealed: bool,
    /// `sealed` | `seal_mismatch`
    pub seal_state: String,
    pub package_matches_seal: bool,
    pub sealed_at: String,
    pub current_owner: String,
    pub proceed_authorized: bool,
    pub consume_authorized: bool,
    pub adapter_invokable: bool,
    pub handoff_performed: bool,
    pub decision_engine_object_id: Option<String>,
    pub permission_effect: String,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakePackageSeal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PERMISSION_EFFECT_NONE: &'static str = "none";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const STATE_SEALED: &'static str = "sealed";
    pub const STATE_SEAL_MISMATCH: &'static str = "seal_mismatch";

    /// Deterministic digest of intake identity/provenance fields (not a score).
    pub fn package_digest(intake: &RecommendationDecisionIntakeRequest) -> String {
        let evidence = intake.evidence_refs.join("|");
        let keys = intake.explanation_keys.join("|");
        let payload = [
            intake.recommendation_id.as_str(),
            intake.workspace_id.as_str(),
            intake.confirmation_intent.as_str(),
            intake.confirmed_at.as_str(),
            intake.kind.as_str(),
            intake.title.as_str(),
            intake.suggested_goal_statement.as_str(),
            intake.continuity_fingerprint.as_str(),
            intake.explanation_ref.as_deref().unwrap_or(""),
            evidence.as_str(),
            keys.as_str(),
            intake.outcome_id.as_deref().unwrap_or(""),
            intake.related_task_id.as_deref().unwrap_or(""),
            intake.related_attention_id.as_deref().unwrap_or(""),
            intake.related_decision_id.as_deref().unwrap_or(""),
            intake.intake_state.as_str(),
            if intake.handoff_performed {
                "handoff"
            } else {
                "no_handoff"
            },
            intake
                .decision_engine_object_id
                .as_deref()
                .unwrap_or("none"),
        ]
        .join("\u{1f}");
        format!(
            "digest:{}:{}",
            RecommendationDecisionIntakeCompatibility::CONTRACT_VERSION,
            payload
        )
    }

    /// Seal intake after proceed denial. Never authorizes proceed/adapter.
    pub fn derive_from_proceed_denial(
        intake: &RecommendationDecisionIntakeRequest,
        compatibility: &RecommendationDecisionIntakeCompatibility,
        denial: &RecommendationDecisionIntakeProceedDenial,
        sealed_at: impl Into<String>,
    ) -> Self {
        let sealed_at = sealed_at.into();
        let digest = Self::package_digest(intake);
        Self {
            recommendation_id: intake.recommendation_id.clone(),
            intake_package_digest: digest,
            continuity_fingerprint_at_seal: intake.continuity_fingerprint.clone(),
            contract_version: compatibility.contract_version.clone(),
            confirmation_intent: intake.confirmation_intent.clone(),
            confirmed_at: intake.confirmed_at.clone(),
            eligibility_state: denial.eligibility_state.clone(),
            sealed: true,
            seal_state: Self::STATE_SEALED.into(),
            package_matches_seal: true,
            sealed_at,
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            proceed_authorized: false,
            consume_authorized: false,
            adapter_invokable: false,
            handoff_performed: false,
            decision_engine_object_id: None,
            permission_effect: Self::PERMISSION_EFFECT_NONE.into(),
            note: "Intake package sealed for future consideration. Seal freezes the RE-owned \
                   snapshot digest; it does not authorize proceed, adapter invocation, handoff, \
                   or Decision Engine ownership."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Re-check a stored seal against a live (possibly drifted) intake package.
    pub fn reverify_against(mut self, intake: &RecommendationDecisionIntakeRequest) -> Self {
        let matches = self.intake_package_digest == Self::package_digest(intake)
            && self.recommendation_id == intake.recommendation_id;
        self.package_matches_seal = matches;
        self.seal_state = if matches {
            Self::STATE_SEALED.into()
        } else {
            Self::STATE_SEAL_MISMATCH.into()
        };
        // Proceed denial remains absolute even when the package still matches.
        self.proceed_authorized = false;
        self.consume_authorized = false;
        self.adapter_invokable = false;
        self.handoff_performed = false;
        self.decision_engine_object_id = None;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        if !matches {
            self.note = "Sealed intake digest does not match live package. Stale/drifted artifact \
                         cannot progress; proceed/adapter/handoff remain denied."
                .into();
        }
        self
    }

    pub fn assert_matches_intake(
        &self,
        intake: &RecommendationDecisionIntakeRequest,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.intake_package_digest == Self::package_digest(intake)
            && self.package_matches_seal
            && self.seal_state == Self::STATE_SEALED
        {
            Ok(())
        } else {
            Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
        }
    }

    pub fn attempt_mutate_after_seal(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.sealed {
            Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
        } else {
            Ok(())
        }
    }

    pub fn may_proceed(&self) -> bool {
        false
    }

    pub fn may_consume(&self) -> bool {
        false
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn attempt_authorize_proceed(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_consume(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
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

    pub fn assert_seal_is_not_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        if !self.sealed
            || self.proceed_authorized
            || self.consume_authorized
            || self.adapter_invokable
            || self.handoff_performed
            || self.decision_engine_object_id.is_some()
            || self.permission_effect != Self::PERMISSION_EFFECT_NONE
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Controlled RE → future-DE adapter boundary preparation (Sprint 257).
///
/// Records that a user-confirmed, sealed intake may be prepared for a *future*
/// adapter path. Does not invoke the adapter, create Decision Engine objects,
/// intents, commands, or Gateway grants. Ownership remains Recommendation Engine.
/// Reversible via `revoke`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionIntakeAdapterPreparation {
    pub recommendation_id: String,
    pub workspace_id: String,
    /// `prepared` | `revoked`
    pub preparation_state: String,
    pub prepared_at: Option<String>,
    pub revoked_at: Option<String>,
    pub confirmation_intent: String,
    pub sealed_intake_package_digest: String,
    pub contract_version: String,
    pub continuity_fingerprint_at_prep: String,
    /// False when live seal no longer matches the prepared digest.
    pub seal_aligned: bool,
    pub current_owner: String,
    pub declared_consumer_role: String,
    pub adapter_invoked: bool,
    pub mapping_performed: bool,
    pub decision_engine_object_id: Option<String>,
    /// Documentation of intended future mapping only — never executed.
    pub suggested_mapping_notes: Vec<String>,
    pub proceed_authorized: bool,
    pub handoff_performed: bool,
    pub permission_effect: String,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionIntakeAdapterPreparation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PERMISSION_EFFECT_NONE: &'static str = "none";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const CONSUMER_ROLE_ADAPTER_READER: &'static str = "future_adapter_reader_only";
    pub const STATE_PREPARED: &'static str = "prepared";
    pub const STATE_REVOKED: &'static str = "revoked";

    pub fn suggested_mapping_notes() -> Vec<String> {
        vec![
            "future_only: intake.title → DecisionCandidate.title (DE-owned creation later)".into(),
            "future_only: intake.suggested_goal_statement → informational goal_statement string"
                .into(),
            "future_only: intake.recommendation_id → DecisionCandidate.recommendation_id".into(),
            "future_only: evidence/explanation refs → DecisionExplanation inputs".into(),
            "never: score synthesis, next_command, submit_assistant_goal, Gateway, ownership transfer"
                .into(),
        ]
    }

    /// Prepare adapter path only when confirmation + matching seal are present.
    /// Never invokes adapter or creates DE objects.
    pub fn try_prepare(
        intake: &RecommendationDecisionIntakeRequest,
        confirmation: &RecommendationDecisionConfirmation,
        seal: &RecommendationDecisionIntakePackageSeal,
        prepared_at: impl Into<String>,
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
        if !seal.sealed
            || !seal.package_matches_seal
            || seal.seal_state != RecommendationDecisionIntakePackageSeal::STATE_SEALED
        {
            return None;
        }
        if seal.assert_matches_intake(intake).is_err() {
            return None;
        }
        let prepared_at = prepared_at.into();
        Some(Self {
            recommendation_id: intake.recommendation_id.clone(),
            workspace_id: intake.workspace_id.clone(),
            preparation_state: Self::STATE_PREPARED.into(),
            prepared_at: Some(prepared_at),
            revoked_at: None,
            confirmation_intent: confirmation.confirmation_intent.clone(),
            sealed_intake_package_digest: seal.intake_package_digest.clone(),
            contract_version: seal.contract_version.clone(),
            continuity_fingerprint_at_prep: seal.continuity_fingerprint_at_seal.clone(),
            seal_aligned: true,
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            declared_consumer_role: Self::CONSUMER_ROLE_ADAPTER_READER.into(),
            adapter_invoked: false,
            mapping_performed: false,
            decision_engine_object_id: None,
            suggested_mapping_notes: Self::suggested_mapping_notes(),
            proceed_authorized: false,
            handoff_performed: false,
            permission_effect: Self::PERMISSION_EFFECT_NONE.into(),
            note: "Adapter path prepared against sealed intake digest. Preparation is not \
                   adapter invocation, Decision Engine object creation, intent creation, \
                   ownership transfer, or execution authority. Reversible via revoke."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Reversible withdrawal of adapter preparation — no DE/Gateway side effects.
    pub fn revoke(&mut self, revoked_at: impl Into<String>) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.adapter_invoked
            || self.mapping_performed
            || self.decision_engine_object_id.is_some()
            || self.handoff_performed
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        self.preparation_state = Self::STATE_REVOKED.into();
        self.revoked_at = Some(revoked_at.into());
        self.proceed_authorized = false;
        self.note = "Adapter preparation revoked. No Decision Engine objects, intents, or \
                     Gateway grants were created. Recommendation Engine retains ownership."
            .into();
        Ok(())
    }

    /// Re-check preparation against a live (possibly drifted) seal.
    pub fn rebind_to_seal(mut self, seal: &RecommendationDecisionIntakePackageSeal) -> Self {
        self.seal_aligned = self.preparation_state == Self::STATE_PREPARED
            && seal.package_matches_seal
            && seal.intake_package_digest == self.sealed_intake_package_digest
            && seal.seal_state == RecommendationDecisionIntakePackageSeal::STATE_SEALED;
        self.adapter_invoked = false;
        self.mapping_performed = false;
        self.decision_engine_object_id = None;
        self.proceed_authorized = false;
        self.handoff_performed = false;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        if self.preparation_state == Self::STATE_PREPARED && !self.seal_aligned {
            self.note = "Adapter preparation exists but sealed digest is no longer aligned. \
                         Progression blocked; invoke/create-DE remain denied. Revoke or re-confirm."
                .into();
        }
        self
    }

    /// Active only when prepared and seal still aligned — still not invoke/ownership.
    pub fn is_active_preparation(&self) -> bool {
        self.preparation_state == Self::STATE_PREPARED && self.seal_aligned
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn may_proceed(&self) -> bool {
        false
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_perform_mapping(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_authorize_proceed(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
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

    pub fn assert_preparation_is_not_invocation(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.adapter_invoked
            || self.mapping_performed
            || self.proceed_authorized
            || self.handoff_performed
            || self.decision_engine_object_id.is_some()
            || self.permission_effect != Self::PERMISSION_EFFECT_NONE
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Non-executing RE → future-DE handoff *request* artifact (Sprint 262).
///
/// Declares that Recommendation Engine is requesting future Decision Engine
/// consideration of a sealed, prepared intake. Does **not** perform handoff,
/// create DE objects, invoke the adapter, create intents, or grant Gateway
/// authority. Ownership remains Recommendation Engine until a future explicit
/// DE acceptance (separate increment). Reversible via `revoke`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionHandoffRequest {
    pub recommendation_id: String,
    pub workspace_id: String,
    /// `requested` | `revoked`
    pub request_state: String,
    /// True only while `requested` and preparation/seal remain valid.
    pub handoff_requested: bool,
    /// Always false — request ≠ performed handoff.
    pub handoff_performed: bool,
    pub requested_at: Option<String>,
    pub revoked_at: Option<String>,
    pub confirmation_intent: String,
    pub confirmed_at: String,
    pub sealed_intake_package_digest: String,
    pub contract_version: String,
    pub contract_family: String,
    pub continuity_fingerprint: String,
    pub preparation_state_at_request: String,
    pub preparation_prepared_at: Option<String>,
    /// False when preparation is revoked or seal no longer aligned.
    pub preparation_active: bool,
    pub seal_aligned: bool,
    pub current_owner: String,
    pub decision_engine_object_id: Option<String>,
    pub adapter_invoked: bool,
    pub permission_effect: String,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionHandoffRequest {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PERMISSION_EFFECT_NONE: &'static str = "none";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const STATE_REQUESTED: &'static str = "requested";
    pub const STATE_REVOKED: &'static str = "revoked";

    /// Emit a handoff *request* only from active preparation + confirmation + seal + identity.
    /// Never performs handoff or creates DE objects.
    pub fn try_request(
        preparation: &RecommendationDecisionIntakeAdapterPreparation,
        confirmation: &RecommendationDecisionConfirmation,
        seal: &RecommendationDecisionIntakePackageSeal,
        compatibility: &RecommendationDecisionIntakeCompatibility,
        requested_at: impl Into<String>,
    ) -> Option<Self> {
        if !preparation.is_active_preparation() {
            return None;
        }
        if confirmation.confirmation_state != RecommendationDecisionConfirmation::STATE_CONFIRMED {
            return None;
        }
        if confirmation.confirmed_at.is_none() {
            return None;
        }
        if !seal.sealed
            || !seal.package_matches_seal
            || seal.seal_state != RecommendationDecisionIntakePackageSeal::STATE_SEALED
        {
            return None;
        }
        if seal.intake_package_digest != preparation.sealed_intake_package_digest {
            return None;
        }
        if !compatibility.compatible {
            return None;
        }
        let requested_at = requested_at.into();
        Some(Self {
            recommendation_id: preparation.recommendation_id.clone(),
            workspace_id: preparation.workspace_id.clone(),
            request_state: Self::STATE_REQUESTED.into(),
            handoff_requested: true,
            handoff_performed: false,
            requested_at: Some(requested_at),
            revoked_at: None,
            confirmation_intent: confirmation.confirmation_intent.clone(),
            confirmed_at: confirmation.confirmed_at.clone().unwrap_or_default(),
            sealed_intake_package_digest: seal.intake_package_digest.clone(),
            contract_version: compatibility.contract_version.clone(),
            contract_family: compatibility.contract_family.clone(),
            continuity_fingerprint: preparation.continuity_fingerprint_at_prep.clone(),
            preparation_state_at_request: preparation.preparation_state.clone(),
            preparation_prepared_at: preparation.prepared_at.clone(),
            preparation_active: true,
            seal_aligned: true,
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            decision_engine_object_id: None,
            adapter_invoked: false,
            permission_effect: Self::PERMISSION_EFFECT_NONE.into(),
            note: "Handoff requested to future Decision Engine for sealed, prepared intake. \
                   Request is not handoff performance, DE object creation, adapter invocation, \
                   ownership transfer, or execution authority. Reversible via revoke."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Reversible withdrawal of the handoff request — no DE/Gateway side effects.
    pub fn revoke(
        &mut self,
        revoked_at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.handoff_performed
            || self.adapter_invoked
            || self.decision_engine_object_id.is_some()
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        self.request_state = Self::STATE_REVOKED.into();
        self.handoff_requested = false;
        self.revoked_at = Some(revoked_at.into());
        self.preparation_active = false;
        self.note = "Handoff request revoked. No Decision Engine objects, intents, or Gateway \
                     grants were created. Recommendation Engine retains ownership."
            .into();
        Ok(())
    }

    /// Re-check request against live preparation — revoked/misaligned prep blocks progression.
    pub fn rebind_to_preparation(
        mut self,
        preparation: &RecommendationDecisionIntakeAdapterPreparation,
    ) -> Self {
        let prep_active = preparation.is_active_preparation()
            && preparation.sealed_intake_package_digest == self.sealed_intake_package_digest;
        self.preparation_active = prep_active;
        self.seal_aligned = prep_active && preparation.seal_aligned;
        if self.request_state == Self::STATE_REQUESTED {
            self.handoff_requested = prep_active && self.seal_aligned;
            if !self.handoff_requested {
                self.note = "Handoff request exists but preparation is no longer active/aligned. \
                             Progression blocked; handoff_performed remains false. Revoke or re-confirm."
                    .into();
            }
        } else {
            self.handoff_requested = false;
        }
        self.handoff_performed = false;
        self.adapter_invoked = false;
        self.decision_engine_object_id = None;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        self
    }

    pub fn is_active_request(&self) -> bool {
        self.request_state == Self::STATE_REQUESTED
            && self.handoff_requested
            && self.preparation_active
            && self.seal_aligned
            && !self.handoff_performed
    }

    pub fn may_perform_handoff(&self) -> bool {
        false
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn attempt_perform_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
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

    pub fn assert_request_is_not_performed_handoff(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.handoff_performed
            || self.adapter_invoked
            || self.decision_engine_object_id.is_some()
            || self.permission_effect != Self::PERMISSION_EFFECT_NONE
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(WorkspaceRecommendationEngineError::CannotBecomeHandoff);
        }
        Ok(())
    }
}

/// Non-executing Decision Engine acceptance boundary (Sprint 267).
///
/// Records whether a future Decision Engine accepts or declines an active handoff
/// request. Acceptance records *future* ownership intent only —
/// `ownership_transferred` remains false, no DE object is created, adapter is not
/// invoked, and Recommendation Engine remains `current_owner`. Reversible via revoke.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationDecisionEngineAcceptance {
    pub recommendation_id: String,
    pub workspace_id: String,
    /// `awaiting_acceptance` | `accepted` | `declined` | `revoked`
    pub acceptance_state: String,
    /// `retained_by_recommendation` | `accepted_for_future_decision_engine` | `declined_by_decision_engine`
    pub ownership_state: String,
    /// Always false — acceptance ≠ performed ownership transfer.
    pub ownership_transferred: bool,
    pub current_owner: String,
    /// `decision_engine` when accepted for future ownership; otherwise none.
    pub declared_future_owner: Option<String>,
    pub handoff_request_state: String,
    pub handoff_requested: bool,
    pub sealed_intake_package_digest: String,
    pub contract_version: String,
    pub contract_family: String,
    pub confirmation_intent: String,
    pub accepted_at: Option<String>,
    pub declined_at: Option<String>,
    pub revoked_at: Option<String>,
    pub decision_engine_object_id: Option<String>,
    pub adapter_invoked: bool,
    pub handoff_performed: bool,
    pub permission_effect: String,
    pub note: String,
    pub authority_effect: String,
}

impl RecommendationDecisionEngineAcceptance {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PERMISSION_EFFECT_NONE: &'static str = "none";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";
    pub const OWNER_DECISION: &'static str = "decision_engine";

    pub const STATE_AWAITING: &'static str = "awaiting_acceptance";
    pub const STATE_ACCEPTED: &'static str = "accepted";
    pub const STATE_DECLINED: &'static str = "declined";
    pub const STATE_REVOKED: &'static str = "revoked";

    pub const OWNERSHIP_RETAINED: &'static str = "retained_by_recommendation";
    pub const OWNERSHIP_ACCEPTED_FUTURE: &'static str = "accepted_for_future_decision_engine";
    pub const OWNERSHIP_DECLINED: &'static str = "declined_by_decision_engine";

    /// Derive awaiting acceptance only from an active handoff request.
    pub fn derive_from_handoff_request(
        request: &RecommendationDecisionHandoffRequest,
    ) -> Option<Self> {
        if !request.is_active_request() {
            return None;
        }
        Some(Self {
            recommendation_id: request.recommendation_id.clone(),
            workspace_id: request.workspace_id.clone(),
            acceptance_state: Self::STATE_AWAITING.into(),
            ownership_state: Self::OWNERSHIP_RETAINED.into(),
            ownership_transferred: false,
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            declared_future_owner: None,
            handoff_request_state: request.request_state.clone(),
            handoff_requested: request.handoff_requested,
            sealed_intake_package_digest: request.sealed_intake_package_digest.clone(),
            contract_version: request.contract_version.clone(),
            contract_family: request.contract_family.clone(),
            confirmation_intent: request.confirmation_intent.clone(),
            accepted_at: None,
            declined_at: None,
            revoked_at: None,
            decision_engine_object_id: None,
            adapter_invoked: false,
            handoff_performed: false,
            permission_effect: Self::PERMISSION_EFFECT_NONE.into(),
            note: "Awaiting Decision Engine acceptance of sealed handoff request. \
                   Acceptance is not ownership transfer, DE object creation, adapter \
                   invocation, or execution authority. Recommendation Engine retains ownership."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn allows_transition(&self, to: &str) -> bool {
        match (self.acceptance_state.as_str(), to) {
            (Self::STATE_AWAITING, Self::STATE_ACCEPTED)
            | (Self::STATE_AWAITING, Self::STATE_DECLINED)
            | (Self::STATE_AWAITING, Self::STATE_REVOKED)
            | (Self::STATE_ACCEPTED, Self::STATE_REVOKED)
            | (Self::STATE_DECLINED, Self::STATE_REVOKED) => true,
            (from, to) if from == to => true,
            _ => false,
        }
    }

    /// Record DE acceptance of the handoff request for *future* ownership — never transfers now.
    pub fn accept(
        &mut self,
        at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if !self.handoff_requested {
            return Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority);
        }
        if !self.allows_transition(Self::STATE_ACCEPTED) {
            return Err(WorkspaceRecommendationEngineError::InvalidAcceptanceTransition {
                from: self.acceptance_state.clone(),
                to: Self::STATE_ACCEPTED.into(),
            });
        }
        let at = at.into();
        self.acceptance_state = Self::STATE_ACCEPTED.into();
        self.ownership_state = Self::OWNERSHIP_ACCEPTED_FUTURE.into();
        self.ownership_transferred = false;
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        self.declared_future_owner = Some(Self::OWNER_DECISION.into());
        self.accepted_at = Some(at);
        self.declined_at = None;
        self.decision_engine_object_id = None;
        self.adapter_invoked = false;
        self.handoff_performed = false;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.note = "Decision Engine acceptance recorded for future ownership of sealed intake. \
                     Ownership is not transferred yet; no Decision object, intent, adapter \
                     invocation, or Gateway grant was created. Recommendation Engine remains owner."
            .into();
        Ok(())
    }

    /// Decline DE acceptance — RE retains ownership; no DE object or transfer.
    pub fn decline(
        &mut self,
        at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if !self.allows_transition(Self::STATE_DECLINED) {
            return Err(WorkspaceRecommendationEngineError::InvalidAcceptanceTransition {
                from: self.acceptance_state.clone(),
                to: Self::STATE_DECLINED.into(),
            });
        }
        self.acceptance_state = Self::STATE_DECLINED.into();
        self.ownership_state = Self::OWNERSHIP_DECLINED.into();
        self.ownership_transferred = false;
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        self.declared_future_owner = None;
        self.declined_at = Some(at.into());
        self.accepted_at = None;
        self.decision_engine_object_id = None;
        self.adapter_invoked = false;
        self.handoff_performed = false;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.note = "Decision Engine declined the handoff request. Recommendation Engine retains \
                     ownership. No Decision object, intent, adapter invocation, or Gateway grant."
            .into();
        Ok(())
    }

    /// Reversible withdrawal of acceptance boundary — no DE/Gateway side effects.
    pub fn revoke(
        &mut self,
        revoked_at: impl Into<String>,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.ownership_transferred
            || self.adapter_invoked
            || self.handoff_performed
            || self.decision_engine_object_id.is_some()
        {
            return Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority);
        }
        if !self.allows_transition(Self::STATE_REVOKED)
            && self.acceptance_state != Self::STATE_REVOKED
        {
            return Err(WorkspaceRecommendationEngineError::InvalidAcceptanceTransition {
                from: self.acceptance_state.clone(),
                to: Self::STATE_REVOKED.into(),
            });
        }
        self.acceptance_state = Self::STATE_REVOKED.into();
        self.ownership_state = Self::OWNERSHIP_RETAINED.into();
        self.ownership_transferred = false;
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        self.declared_future_owner = None;
        self.revoked_at = Some(revoked_at.into());
        self.handoff_requested = false;
        self.note = "Decision Engine acceptance boundary revoked. No ownership transfer, DE \
                     objects, intents, or Gateway grants occurred. Recommendation Engine retains ownership."
            .into();
        Ok(())
    }

    /// Re-check against live handoff request — inactive request blocks progression.
    pub fn rebind_to_handoff_request(
        mut self,
        request: &RecommendationDecisionHandoffRequest,
    ) -> Self {
        self.handoff_request_state = request.request_state.clone();
        self.handoff_requested = request.is_active_request();
        self.sealed_intake_package_digest = request.sealed_intake_package_digest.clone();
        self.contract_version = request.contract_version.clone();
        self.contract_family = request.contract_family.clone();
        self.confirmation_intent = request.confirmation_intent.clone();
        self.ownership_transferred = false;
        self.adapter_invoked = false;
        self.handoff_performed = false;
        self.decision_engine_object_id = None;
        self.permission_effect = Self::PERMISSION_EFFECT_NONE.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        self.current_owner = Self::OWNER_RECOMMENDATION.into();
        if self.acceptance_state == Self::STATE_AWAITING && !self.handoff_requested {
            self.note = "Acceptance awaits handoff request, but request is no longer active. \
                         Progression blocked; ownership remains on Recommendation Engine."
                .into();
        }
        if self.acceptance_state == Self::STATE_ACCEPTED {
            self.declared_future_owner = Some(Self::OWNER_DECISION.into());
            self.ownership_state = Self::OWNERSHIP_ACCEPTED_FUTURE.into();
            if !self.handoff_requested {
                self.note = "Prior DE acceptance exists but handoff request is no longer active. \
                             Ownership was never transferred; Recommendation Engine remains owner."
                    .into();
            }
        }
        self
    }

    pub fn is_awaiting(&self) -> bool {
        self.acceptance_state == Self::STATE_AWAITING && self.handoff_requested
    }

    pub fn is_accepted_for_future(&self) -> bool {
        self.acceptance_state == Self::STATE_ACCEPTED
            && self.ownership_state == Self::OWNERSHIP_ACCEPTED_FUTURE
            && !self.ownership_transferred
            && self.current_owner == Self::OWNER_RECOMMENDATION
    }

    pub fn may_transfer_ownership(&self) -> bool {
        false
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority)
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::CannotExecute)
    }

    pub fn attempt_create_decision_engine_object(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority)
    }

    pub fn attempt_create_intent(&self) -> Result<(), WorkspaceRecommendationEngineError> {
        Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority)
    }

    pub fn attempt_perform_handoff(&self) -> Result<(), WorkspaceRecommendationEngineError> {
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

    pub fn assert_acceptance_is_not_ownership_transfer(
        &self,
    ) -> Result<(), WorkspaceRecommendationEngineError> {
        if self.ownership_transferred
            || self.adapter_invoked
            || self.handoff_performed
            || self.decision_engine_object_id.is_some()
            || self.permission_effect != Self::PERMISSION_EFFECT_NONE
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority);
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
            Some(_) => false,
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
        // Actionable surface excludes terminals; history remains visible on the summary so
        // Intelligence/Assistant cannot treat "no top_candidates" as "no evidence".
        let history: Vec<RecommendationHistoryEntry> =
            self.history.iter().take(limit).cloned().collect();
        WorkspaceRecommendationEngineSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            // Surface count reflects still-actionable suggestions.
            candidate_count: active.len(),
            relationship_count: self.relationship_count,
            top_candidates: active.into_iter().take(limit).collect(),
            history,
            history_count: self.history_count,
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
    /// Truncated terminal/orphan evidence for consumers that only receive the summary.
    #[serde(default)]
    pub history: Vec<RecommendationHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
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
            history: Vec::new(),
            history_count: 0,
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
