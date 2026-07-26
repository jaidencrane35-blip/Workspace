//! Governed Decision Engine foundation (Phase 5).
//!
//! Synthesizes Attention + Intelligence into ranked, explainable recommendations.
//! Distinct from Decision Queue (human inbox). Never executes or grants authority.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{DecisionCandidateId, WorkspaceId};
use crate::workspace_attention::AttentionReason;

/// Decision Engine validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecisionEngineError {
    #[error("invalid decision outcome: {0}")]
    InvalidOutcome(String),

    #[error("decision candidate not found")]
    NotFound,

    #[error("decision transition not allowed from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("decision engine cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Structured reason supporting a recommendation (never chain-of-thought).
///
/// When the rationale originates in Attention, `attention_reason` carries the upstream
/// `AttentionReason` verbatim so the chain facts → signals → reasons → decisions stays
/// traceable. `summary` remains a factual restatement for surfaces that render text;
/// translation belongs to Experience, keyed off `attention_reason.explanation_key`.
/// Reasons Decision Engine derives itself (goal, memory, plan, approval) leave it `None`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionReason {
    pub kind: String,
    pub summary: String,
    pub evidence_ref: Option<String>,
    pub attention_reason: Option<AttentionReason>,
}

/// Deterministic score breakdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionScore {
    pub total: u32,
    pub attention_contribution: u32,
    pub memory_contribution: u32,
    pub personalization_contribution: u32,
    pub goal_contribution: u32,
    pub factors: Vec<String>,
}

/// Human-facing explanation of a candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionExplanation {
    pub headline: String,
    pub reasons: Vec<DecisionReason>,
    pub confidence: String,
}

/// Lifecycle of a Decision Engine candidate (overlay + projection).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionOutcome {
    Open,
    Selected,
    Dismissed,
    Postponed,
    Expired,
}

impl DecisionOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Selected => "selected",
            Self::Dismissed => "dismissed",
            Self::Postponed => "postponed",
            Self::Expired => "expired",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionEngineError> {
        match value {
            "open" => Ok(Self::Open),
            "selected" => Ok(Self::Selected),
            "dismissed" => Ok(Self::Dismissed),
            "postponed" => Ok(Self::Postponed),
            "expired" => Ok(Self::Expired),
            other => Err(DecisionEngineError::InvalidOutcome(other.into())),
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Open, Self::Selected)
                | (Self::Open, Self::Dismissed)
                | (Self::Open, Self::Postponed)
                | (Self::Postponed, Self::Open)
                | (Self::Postponed, Self::Selected)
                | (Self::Postponed, Self::Dismissed)
                | (Self::Open, Self::Expired)
                | (Self::Postponed, Self::Expired)
        )
    }
}

/// Context aggregated for synthesis (informational snapshot).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionContext {
    pub workspace_id: String,
    pub active_project_id: Option<String>,
    pub active_task_id: Option<String>,
    pub attention_item_count: usize,
    pub memory_highlight_count: usize,
    pub preference_highlight_count: usize,
    pub pending_approval_count: usize,
    pub pending_plan_count: usize,
    pub task_graph_open_count: usize,
    pub task_graph_blocked_count: usize,
}

/// One ranked, explainable recommendation candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidate {
    pub id: DecisionCandidateId,
    pub workspace_id: WorkspaceId,
    pub title: String,
    pub goal_statement: String,
    pub originating_goal: Option<String>,
    pub attention_item_id: Option<String>,
    pub recommendation_id: Option<String>,
    pub score: DecisionScore,
    pub explanation: DecisionExplanation,
    pub related_goal_ids: Vec<String>,
    pub pending_approval_ids: Vec<String>,
    pub outcome: DecisionOutcome,
    pub created_at: String,
    pub handoff_command: String,
    pub authority_effect: String,
}

impl DecisionCandidate {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const HANDOFF_SUBMIT_ASSISTANT_GOAL: &'static str = "submit_assistant_goal";

    pub fn synthetic_id(source_key: &str) -> DecisionCandidateId {
        DecisionCandidateId::new(format!("engine_decision:{source_key}"))
            .expect("synthetic decision candidate id is valid")
    }
}

/// Thin lifecycle overlay — never stores recommendation payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineOverlay {
    pub workspace_id: String,
    pub candidate_key: String,
    pub outcome: DecisionOutcome,
    pub updated_at: String,
    pub actor_id: String,
}

/// Full Decision Engine snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineState {
    pub workspace_id: String,
    pub generated_at: String,
    pub context: DecisionContext,
    pub candidates: Vec<DecisionCandidate>,
    pub top_candidates: Vec<DecisionCandidate>,
    /// Observational receipts of accepted RE sealed packages — never candidates.
    #[serde(default)]
    pub intake_receipts: Vec<DecisionEngineIntakeReceipt>,
    pub summary: String,
    pub authority_effect: String,
}

impl DecisionEngineState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_candidates(
        workspace_id: impl Into<String>,
        context: DecisionContext,
        mut candidates: Vec<DecisionCandidate>,
    ) -> Self {
        candidates.sort_by(|a, b| {
            b.score
                .total
                .cmp(&a.score.total)
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        let open: Vec<_> = candidates
            .iter()
            .filter(|c| matches!(c.outcome, DecisionOutcome::Open | DecisionOutcome::Postponed))
            .cloned()
            .collect();
        let top_candidates: Vec<_> = open.iter().take(5).cloned().collect();
        let summary = format!(
            "Decision Engine — {} candidate(s), {} open. Recommendations only; planner plans; gateway authorizes.",
            candidates.len(),
            open.len()
        );
        Self {
            workspace_id: workspace_id.into(),
            generated_at: Utc::now().to_rfc3339(),
            context,
            candidates,
            top_candidates,
            intake_receipts: Vec::new(),
            summary,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Attach DE-owned observational RE intake receipts without changing scoring or candidates.
    pub fn with_intake_receipts(mut self, receipts: Vec<DecisionEngineIntakeReceipt>) -> Self {
        let observed = receipts.iter().filter(|r| r.is_observed()).count();
        self.summary = format!(
            "{} Intake receipts: {} ({} seal-aligned).",
            self.summary,
            receipts.len(),
            observed
        );
        self.intake_receipts = receipts;
        self
    }

    pub fn summary_projection(&self, limit: usize) -> DecisionEngineSummary {
        DecisionEngineSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            candidate_count: self.candidates.len(),
            open_count: self
                .candidates
                .iter()
                .filter(|c| matches!(c.outcome, DecisionOutcome::Open | DecisionOutcome::Postponed))
                .count(),
            top_candidates: self.top_candidates.iter().take(limit).cloned().collect(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub candidate_count: usize,
    pub open_count: usize,
    pub top_candidates: Vec<DecisionCandidate>,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for DecisionEngineSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            candidate_count: 0,
            open_count: 0,
            top_candidates: Vec::new(),
            summary: String::new(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Result of select / dismiss / postpone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineActionResult {
    pub candidate: Option<DecisionCandidate>,
    pub handoff: Option<DecisionEngineHandoff>,
    pub authority_effect: String,
}

/// Planner handoff — caller must invoke `submit_assistant_goal` explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineHandoff {
    pub candidate_id: String,
    pub next_command: String,
    pub goal_statement: String,
    pub workspace_id: String,
    pub note: String,
    pub authority_effect: String,
}

/// DE-owned observational receipt of an accepted RE sealed handoff package.
///
/// Reads Recommendation Engine acceptance + package seal alignment only.
/// Does not create `DecisionCandidate`, goals, intents, planner handoffs,
/// adapter invocations, ownership transfer, or Gateway grants.
/// Does not mutate Recommendation Engine overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeReceipt {
    pub workspace_id: String,
    pub recommendation_id: String,
    /// `observed` | `seal_mismatch`
    pub receipt_state: String,
    pub acceptance_state: String,
    pub ownership_state: String,
    /// Always false — observation ≠ ownership transfer.
    pub ownership_transferred: bool,
    /// Remains Recommendation Engine; DE only observes.
    pub current_owner: String,
    pub sealed_intake_package_digest: String,
    pub seal_aligned: bool,
    pub contract_version: String,
    pub contract_family: String,
    /// Always `None` — receipt is not a Decision Engine object.
    pub decision_engine_object_id: Option<String>,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    /// Always `None` — distinct from `DecisionCandidate.handoff_command`.
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeReceipt {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_OBSERVED: &'static str = "observed";
    pub const STATE_SEAL_MISMATCH: &'static str = "seal_mismatch";
    pub const OWNER_RECOMMENDATION: &'static str = "recommendation_engine";

    /// Observe an accepted RE package when seal identity is present.
    /// Returns `None` unless acceptance is `accepted` for future DE ownership.
    pub fn try_observe(
        acceptance: &crate::workspace_recommendation::RecommendationDecisionEngineAcceptance,
        seal: &crate::workspace_recommendation::RecommendationDecisionIntakePackageSeal,
    ) -> Option<Self> {
        use crate::workspace_recommendation::RecommendationDecisionEngineAcceptance as Acc;
        if acceptance.acceptance_state != Acc::STATE_ACCEPTED {
            return None;
        }
        if acceptance.ownership_transferred
            || acceptance.decision_engine_object_id.is_some()
            || acceptance.current_owner != Acc::OWNER_RECOMMENDATION
        {
            return None;
        }
        let seal_aligned = seal.sealed
            && seal.package_matches_seal
            && seal.seal_state
                == crate::workspace_recommendation::RecommendationDecisionIntakePackageSeal::STATE_SEALED
            && seal.intake_package_digest == acceptance.sealed_intake_package_digest;
        let receipt_state = if seal_aligned {
            Self::STATE_OBSERVED
        } else {
            Self::STATE_SEAL_MISMATCH
        };
        Some(Self {
            workspace_id: acceptance.workspace_id.clone(),
            recommendation_id: acceptance.recommendation_id.clone(),
            receipt_state: receipt_state.into(),
            acceptance_state: acceptance.acceptance_state.clone(),
            ownership_state: acceptance.ownership_state.clone(),
            ownership_transferred: false,
            current_owner: Self::OWNER_RECOMMENDATION.into(),
            sealed_intake_package_digest: acceptance.sealed_intake_package_digest.clone(),
            seal_aligned,
            contract_version: acceptance.contract_version.clone(),
            contract_family: acceptance.contract_family.clone(),
            decision_engine_object_id: None,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            handoff_command: None,
            note: if seal_aligned {
                "Decision Engine observed accepted Recommendation Engine sealed package. \
                 Observation is not DecisionCandidate creation, goal/intent creation, \
                 ownership transfer, adapter invocation, planner handoff, or execution."
                    .into()
            } else {
                "Decision Engine saw accepted Recommendation Engine package but seal is not aligned. \
                 Progression blocked; no DecisionCandidate or ownership transfer."
                    .into()
            },
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_observed(&self) -> bool {
        self.receipt_state == Self::STATE_OBSERVED && self.seal_aligned
    }

    pub fn may_create_decision_candidate(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
        false
    }

    pub fn may_invoke_adapter(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_candidate(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_adapter(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_observational_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.decision_engine_object_id.is_some()
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.current_owner != Self::OWNER_RECOMMENDATION
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}
