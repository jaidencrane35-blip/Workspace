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
    /// Observational assessments of intake receipts — never candidates.
    #[serde(default)]
    pub intake_assessments: Vec<DecisionEngineIntakeAssessment>,
    /// Observational eligibility for future candidate consideration — never candidates.
    #[serde(default)]
    pub intake_eligibilities: Vec<DecisionEngineIntakeEligibility>,
    /// DE-owned intake lifecycle acknowledgements — never DecisionCandidates.
    #[serde(default)]
    pub intake_candidates: Vec<DecisionEngineIntakeCandidate>,
    /// DE-owned intake evaluations — examination records, never planning authority.
    #[serde(default)]
    pub intake_evaluations: Vec<DecisionEngineIntakeEvaluation>,
    /// DE-owned intake dispositions — lifecycle decisions, never planning authority.
    #[serde(default)]
    pub intake_dispositions: Vec<DecisionEngineIntakeDisposition>,
    /// DE-owned promotion boundary — readiness for *future* DecisionCandidate promotion only.
    #[serde(default)]
    pub intake_promotion_boundaries: Vec<DecisionEngineIntakePromotionBoundary>,
    /// DE-owned candidate creation requests — request only, never DecisionCandidate creation.
    #[serde(default)]
    pub candidate_creation_requests: Vec<DecisionEngineCandidateCreationRequest>,
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
            intake_assessments: Vec::new(),
            intake_eligibilities: Vec::new(),
            intake_candidates: Vec::new(),
            intake_evaluations: Vec::new(),
            intake_dispositions: Vec::new(),
            intake_promotion_boundaries: Vec::new(),
            candidate_creation_requests: Vec::new(),
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

    /// Attach DE-owned observational intake assessments without changing scoring or candidates.
    pub fn with_intake_assessments(mut self, assessments: Vec<DecisionEngineIntakeAssessment>) -> Self {
        let eligible = assessments
            .iter()
            .filter(|a| a.eligible_for_future_candidate)
            .count();
        self.summary = format!(
            "{} Intake assessments: {} ({} eligible for future candidate).",
            self.summary,
            assessments.len(),
            eligible
        );
        self.intake_assessments = assessments;
        self
    }

    /// Attach DE-owned observational intake eligibility without changing scoring or candidates.
    pub fn with_intake_eligibilities(
        mut self,
        eligibilities: Vec<DecisionEngineIntakeEligibility>,
    ) -> Self {
        let eligible = eligibilities.iter().filter(|e| e.is_eligible).count();
        self.summary = format!(
            "{} Intake eligibilities: {} ({} eligible).",
            self.summary,
            eligibilities.len(),
            eligible
        );
        self.intake_eligibilities = eligibilities;
        self
    }

    /// Attach DE-owned intake candidates without changing scoring or DecisionCandidates.
    pub fn with_intake_candidates(mut self, candidates: Vec<DecisionEngineIntakeCandidate>) -> Self {
        let active = candidates.iter().filter(|c| c.lifecycle.is_active()).count();
        self.summary = format!(
            "{} Intake candidates: {} ({} active).",
            self.summary,
            candidates.len(),
            active
        );
        self.intake_candidates = candidates;
        self
    }

    /// Attach DE-owned intake evaluations without changing scoring or DecisionCandidates.
    pub fn with_intake_evaluations(
        mut self,
        evaluations: Vec<DecisionEngineIntakeEvaluation>,
    ) -> Self {
        self.summary = format!(
            "{} Intake evaluations: {}.",
            self.summary,
            evaluations.len()
        );
        self.intake_evaluations = evaluations;
        self
    }

    /// Attach DE-owned intake dispositions without changing scoring or DecisionCandidates.
    pub fn with_intake_dispositions(
        mut self,
        dispositions: Vec<DecisionEngineIntakeDisposition>,
    ) -> Self {
        self.summary = format!(
            "{} Intake dispositions: {}.",
            self.summary,
            dispositions.len()
        );
        self.intake_dispositions = dispositions;
        self
    }

    /// Attach DE-owned promotion boundaries without changing scoring or DecisionCandidates.
    pub fn with_intake_promotion_boundaries(
        mut self,
        boundaries: Vec<DecisionEngineIntakePromotionBoundary>,
    ) -> Self {
        let allowed = boundaries
            .iter()
            .filter(|b| b.is_promotion_allowed())
            .count();
        self.summary = format!(
            "{} Intake promotion boundaries: {} ({} promotion_allowed).",
            self.summary,
            boundaries.len(),
            allowed
        );
        self.intake_promotion_boundaries = boundaries;
        self
    }

    /// Attach DE-owned candidate creation requests without creating DecisionCandidates.
    pub fn with_candidate_creation_requests(
        mut self,
        requests: Vec<DecisionEngineCandidateCreationRequest>,
    ) -> Self {
        let requested = requests.iter().filter(|r| r.is_requested()).count();
        self.summary = format!(
            "{} Candidate creation requests: {} ({} requested).",
            self.summary,
            requests.len(),
            requested
        );
        self.candidate_creation_requests = requests;
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

/// Grounded inputs for recomputing an intake assessment (not persisted).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionEngineIntakeAssessmentInput {
    pub receipt: DecisionEngineIntakeReceipt,
    /// Acceptance still `accepted` and not revoked.
    pub acceptance_active: bool,
    /// RE lifecycle resolution is superseded.
    pub lifecycle_superseded: bool,
    /// Handoff request + adapter preparation still active for this package.
    pub receipt_current: bool,
}

/// DE-owned observational assessment of an intake receipt.
///
/// Evaluates suitability for *future* DecisionCandidate consideration only.
/// Fully derivable from receipt + RE overlay facts — never persisted.
/// Never creates candidates, goals, intents, planner handoffs, or transfers ownership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeAssessment {
    pub workspace_id: String,
    pub recommendation_id: String,
    pub sealed_intake_package_digest: String,
    /// `blocked` | `superseded` | `duplicate` | `stale` | `valid` | `eligible_for_future_candidate`
    pub assessment_state: String,
    pub receipt_observed: bool,
    pub receipt_current: bool,
    pub seal_valid: bool,
    pub acceptance_active: bool,
    pub is_duplicate: bool,
    pub is_superseded: bool,
    pub is_stale: bool,
    /// Informational only — never means create candidate.
    pub eligible_for_future_candidate: bool,
    pub evidence: Vec<String>,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub decision_engine_object_id: Option<String>,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_SUPERSEDED: &'static str = "superseded";
    pub const STATE_DUPLICATE: &'static str = "duplicate";
    pub const STATE_STALE: &'static str = "stale";
    pub const STATE_VALID: &'static str = "valid";
    pub const STATE_ELIGIBLE: &'static str = "eligible_for_future_candidate";

    /// Recompute assessments for a workspace batch (duplicate detection needs peers).
    pub fn assess_batch(inputs: &[DecisionEngineIntakeAssessmentInput]) -> Vec<Self> {
        use std::collections::HashMap;
        let mut primary_by_digest: HashMap<&str, &str> = HashMap::new();
        for input in inputs {
            let digest = input.receipt.sealed_intake_package_digest.as_str();
            let id = input.receipt.recommendation_id.as_str();
            primary_by_digest
                .entry(digest)
                .and_modify(|primary| {
                    if id < *primary {
                        *primary = id;
                    }
                })
                .or_insert(id);
        }
        let mut assessments: Vec<Self> = inputs
            .iter()
            .map(|input| {
                let digest = input.receipt.sealed_intake_package_digest.as_str();
                let is_duplicate = primary_by_digest
                    .get(digest)
                    .map(|primary| *primary != input.receipt.recommendation_id.as_str())
                    .unwrap_or(false)
                    && inputs
                        .iter()
                        .filter(|i| {
                            i.receipt.sealed_intake_package_digest
                                == input.receipt.sealed_intake_package_digest
                        })
                        .count()
                        > 1;
                Self::assess_one(input, is_duplicate)
            })
            .collect();
        assessments.sort_by(|a, b| a.recommendation_id.cmp(&b.recommendation_id));
        assessments
    }

    fn assess_one(input: &DecisionEngineIntakeAssessmentInput, is_duplicate: bool) -> Self {
        let receipt = &input.receipt;
        let receipt_observed = receipt.is_observed();
        let seal_valid = receipt.seal_aligned
            && receipt.receipt_state == DecisionEngineIntakeReceipt::STATE_OBSERVED;
        let acceptance_active = input.acceptance_active
            && receipt.acceptance_state
                == crate::workspace_recommendation::RecommendationDecisionEngineAcceptance::STATE_ACCEPTED;
        let is_superseded = input.lifecycle_superseded;
        let is_stale = acceptance_active && !input.receipt_current && !is_superseded;
        let mut evidence = Vec::new();
        if receipt_observed {
            evidence.push("receipt_observed".into());
        } else {
            evidence.push("receipt_not_observed".into());
        }
        if seal_valid {
            evidence.push("seal_valid".into());
        } else {
            evidence.push("seal_invalid".into());
        }
        if acceptance_active {
            evidence.push("acceptance_active".into());
        } else {
            evidence.push("acceptance_inactive".into());
        }
        if input.receipt_current {
            evidence.push("receipt_current".into());
        } else {
            evidence.push("receipt_not_current".into());
        }
        if is_superseded {
            evidence.push("lifecycle_superseded".into());
        }
        if is_duplicate {
            evidence.push("duplicate_seal_digest".into());
        }

        let (assessment_state, eligible) = if !receipt_observed || !seal_valid {
            (Self::STATE_BLOCKED, false)
        } else if is_superseded {
            (Self::STATE_SUPERSEDED, false)
        } else if is_duplicate {
            (Self::STATE_DUPLICATE, false)
        } else if is_stale || !acceptance_active || !input.receipt_current {
            (Self::STATE_STALE, false)
        } else if receipt_observed && seal_valid && acceptance_active && input.receipt_current {
            // Structurally valid and clear for future consideration — informational only.
            (Self::STATE_ELIGIBLE, true)
        } else {
            (Self::STATE_VALID, false)
        };
        if eligible {
            evidence.push("eligible_for_future_candidate".into());
        }

        Self {
            workspace_id: receipt.workspace_id.clone(),
            recommendation_id: receipt.recommendation_id.clone(),
            sealed_intake_package_digest: receipt.sealed_intake_package_digest.clone(),
            assessment_state: assessment_state.into(),
            receipt_observed,
            receipt_current: input.receipt_current,
            seal_valid,
            acceptance_active,
            is_duplicate,
            is_superseded,
            is_stale,
            eligible_for_future_candidate: eligible,
            evidence,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            decision_engine_object_id: None,
            handoff_command: None,
            note: if eligible {
                "Intake assessment: eligible for future DecisionCandidate consideration. \
                 Eligibility is informational only — no candidate, goal, intent, planner \
                 handoff, or ownership transfer was created."
                    .into()
            } else {
                format!(
                    "Intake assessment: {assessment_state}. No DecisionCandidate, goal, intent, \
                     planner handoff, or ownership transfer was created."
                )
            },
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
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
            || (self.eligible_for_future_candidate
                && self.assessment_state != Self::STATE_ELIGIBLE)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned observational eligibility for *future* DecisionCandidate consideration.
///
/// Answers: "May this assessment ever become a DecisionCandidate?" without creating one.
/// Fully derivable from receipt + assessment — never persisted.
/// Never creates candidates, goals, intents, planner handoffs, or transfers ownership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeEligibility {
    pub workspace_id: String,
    pub recommendation_id: String,
    pub sealed_intake_package_digest: String,
    /// `not_eligible` | `blocked` | `duplicate` | `superseded` | `stale` | `eligible`
    pub eligibility_state: String,
    /// Informational only — never means create candidate.
    pub is_eligible: bool,
    pub receipt_observed: bool,
    pub seal_aligned: bool,
    pub acceptance_active: bool,
    pub assessment_valid: bool,
    pub is_duplicate: bool,
    pub is_superseded: bool,
    pub is_stale: bool,
    pub evidence: Vec<String>,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub decision_engine_object_id: Option<String>,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeEligibility {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_NOT_ELIGIBLE: &'static str = "not_eligible";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_DUPLICATE: &'static str = "duplicate";
    pub const STATE_SUPERSEDED: &'static str = "superseded";
    pub const STATE_STALE: &'static str = "stale";
    pub const STATE_ELIGIBLE: &'static str = "eligible";

    /// Recompute eligibility projections from paired receipts and assessments.
    pub fn derive_batch(
        receipts: &[DecisionEngineIntakeReceipt],
        assessments: &[DecisionEngineIntakeAssessment],
    ) -> Vec<Self> {
        use std::collections::HashMap;
        let receipt_by_id: HashMap<&str, &DecisionEngineIntakeReceipt> = receipts
            .iter()
            .map(|r| (r.recommendation_id.as_str(), r))
            .collect();
        let mut eligibilities: Vec<Self> = assessments
            .iter()
            .filter_map(|assessment| {
                receipt_by_id
                    .get(assessment.recommendation_id.as_str())
                    .map(|receipt| Self::derive(receipt, assessment))
            })
            .collect();
        eligibilities.sort_by(|a, b| a.recommendation_id.cmp(&b.recommendation_id));
        eligibilities
    }

    /// Derive a single eligibility gate from receipt + assessment (recomputable).
    pub fn derive(
        receipt: &DecisionEngineIntakeReceipt,
        assessment: &DecisionEngineIntakeAssessment,
    ) -> Self {
        let identity_aligned = receipt.recommendation_id == assessment.recommendation_id
            && receipt.sealed_intake_package_digest == assessment.sealed_intake_package_digest
            && receipt.workspace_id == assessment.workspace_id;
        let receipt_observed = identity_aligned && receipt.is_observed() && assessment.receipt_observed;
        let seal_aligned = identity_aligned
            && receipt.seal_aligned
            && assessment.seal_valid
            && receipt.receipt_state == DecisionEngineIntakeReceipt::STATE_OBSERVED;
        let acceptance_active = assessment.acceptance_active;
        let assessment_valid = assessment.assessment_state
            == DecisionEngineIntakeAssessment::STATE_ELIGIBLE
            || assessment.assessment_state == DecisionEngineIntakeAssessment::STATE_VALID;
        let is_duplicate = assessment.is_duplicate;
        let is_superseded = assessment.is_superseded;
        let is_stale = assessment.is_stale || (acceptance_active && !assessment.receipt_current);

        let mut evidence = Vec::new();
        if receipt_observed {
            evidence.push("receipt_observed".into());
        } else {
            evidence.push("receipt_missing_or_misaligned".into());
        }
        if seal_aligned {
            evidence.push("seal_aligned".into());
        } else {
            evidence.push("seal_not_aligned".into());
        }
        if acceptance_active {
            evidence.push("acceptance_active".into());
        } else {
            evidence.push("acceptance_inactive".into());
        }
        if assessment_valid {
            evidence.push("assessment_valid".into());
        } else {
            evidence.push("assessment_not_valid".into());
        }
        if is_superseded {
            evidence.push("superseded".into());
        }
        if is_duplicate {
            evidence.push("duplicate".into());
        }
        if is_stale {
            evidence.push("stale".into());
        }

        let (eligibility_state, is_eligible) = if !receipt_observed || !seal_aligned {
            (Self::STATE_BLOCKED, false)
        } else if is_superseded {
            (Self::STATE_SUPERSEDED, false)
        } else if is_duplicate {
            (Self::STATE_DUPLICATE, false)
        } else if is_stale || !acceptance_active {
            (Self::STATE_STALE, false)
        } else if assessment.eligible_for_future_candidate
            && assessment_valid
            && receipt_observed
            && seal_aligned
            && acceptance_active
            && !is_duplicate
            && !is_superseded
            && !is_stale
        {
            (Self::STATE_ELIGIBLE, true)
        } else {
            (Self::STATE_NOT_ELIGIBLE, false)
        };
        if is_eligible {
            evidence.push("eligible".into());
        } else {
            evidence.push("not_eligible".into());
        }

        Self {
            workspace_id: assessment.workspace_id.clone(),
            recommendation_id: assessment.recommendation_id.clone(),
            sealed_intake_package_digest: assessment.sealed_intake_package_digest.clone(),
            eligibility_state: eligibility_state.into(),
            is_eligible,
            receipt_observed,
            seal_aligned,
            acceptance_active,
            assessment_valid,
            is_duplicate,
            is_superseded,
            is_stale,
            evidence,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            decision_engine_object_id: None,
            handoff_command: None,
            note: if is_eligible {
                "Intake eligibility: eligible for future DecisionCandidate consideration. \
                 Eligibility is informational only — no candidate, goal, intent, planner \
                 handoff, scoring, ranking, or ownership transfer was created."
                    .into()
            } else {
                format!(
                    "Intake eligibility: {eligibility_state}. No DecisionCandidate, goal, intent, \
                     planner handoff, scoring, ranking, or ownership transfer was created."
                )
            },
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
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
            || (self.is_eligible && self.eligibility_state != Self::STATE_ELIGIBLE)
            || (!self.is_eligible && self.eligibility_state == Self::STATE_ELIGIBLE)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned lifecycle for an intake candidate (not DecisionCandidate / planner / execution).
///
/// Mutates only Decision Engine state. Never touches Recommendation Engine overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeCandidateLifecycle {
    /// `active` | `withdrawn` | `invalidated`
    pub lifecycle_state: String,
    pub reason: Option<String>,
    pub updated_at: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeCandidateLifecycle {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_ACTIVE: &'static str = "active";
    pub const STATE_WITHDRAWN: &'static str = "withdrawn";
    pub const STATE_INVALIDATED: &'static str = "invalidated";

    pub const REASON_SEAL_MISMATCH: &'static str = "package_seal_mismatch";
    pub const REASON_ACCEPTANCE_REVOKED: &'static str = "acceptance_revoked";
    pub const REASON_SUPERSEDED: &'static str = "recommendation_superseded";
    pub const REASON_DE_WITHDRAWAL: &'static str = "decision_engine_withdrawal";

    pub fn start_active(updated_at: impl Into<String>) -> Self {
        Self {
            lifecycle_state: Self::STATE_ACTIVE.into(),
            reason: None,
            updated_at: updated_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.lifecycle_state == Self::STATE_ACTIVE
    }

    pub fn is_withdrawn(&self) -> bool {
        self.lifecycle_state == Self::STATE_WITHDRAWN
    }

    pub fn is_invalidated(&self) -> bool {
        self.lifecycle_state == Self::STATE_INVALIDATED
    }

    pub fn allows_transition(&self, to: &str) -> bool {
        match (self.lifecycle_state.as_str(), to) {
            (from, to) if from == to => true,
            (Self::STATE_ACTIVE, Self::STATE_WITHDRAWN)
            | (Self::STATE_ACTIVE, Self::STATE_INVALIDATED)
            | (Self::STATE_WITHDRAWN, Self::STATE_INVALIDATED) => true,
            // invalidated → active is never allowed; withdrawn → active is DE-explicit only (not here).
            (Self::STATE_INVALIDATED, _) => false,
            _ => false,
        }
    }

    /// DE-owned withdrawal — does not mutate Recommendation Engine records.
    pub fn withdraw(&mut self, at: impl Into<String>) -> Result<(), DecisionEngineError> {
        if !self.allows_transition(Self::STATE_WITHDRAWN) {
            return Err(DecisionEngineError::InvalidTransition {
                from: self.lifecycle_state.clone(),
                to: Self::STATE_WITHDRAWN.into(),
            });
        }
        self.lifecycle_state = Self::STATE_WITHDRAWN.into();
        self.reason = Some(Self::REASON_DE_WITHDRAWAL.into());
        self.updated_at = at.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        Ok(())
    }

    /// Invalidate from invalid source package facts. Terminal against reactivation.
    pub fn invalidate(
        &mut self,
        reason: impl Into<String>,
        at: impl Into<String>,
    ) -> Result<(), DecisionEngineError> {
        if self.is_invalidated() {
            self.reason = Some(reason.into());
            self.updated_at = at.into();
            return Ok(());
        }
        if !self.allows_transition(Self::STATE_INVALIDATED) {
            return Err(DecisionEngineError::InvalidTransition {
                from: self.lifecycle_state.clone(),
                to: Self::STATE_INVALIDATED.into(),
            });
        }
        self.lifecycle_state = Self::STATE_INVALIDATED.into();
        self.reason = Some(reason.into());
        self.updated_at = at.into();
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        Ok(())
    }

    /// Reactivation is never allowed from invalidated.
    pub fn attempt_reactivate(&mut self) -> Result<(), DecisionEngineError> {
        if self.is_invalidated() {
            return Err(DecisionEngineError::InvalidTransition {
                from: Self::STATE_INVALIDATED.into(),
                to: Self::STATE_ACTIVE.into(),
            });
        }
        Err(DecisionEngineError::CannotExecute)
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }
}

/// DE-owned intake lifecycle acknowledgement for an eligible RE sealed package.
///
/// Represents that Decision Engine has accepted the package for *future* evaluation.
/// This is not a `DecisionCandidate`, planner input, goal, intent, or executable path.
/// Recommendation Engine retains recommendation ownership — no ownership transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeCandidate {
    pub intake_candidate_id: String,
    pub workspace_id: String,
    pub intake_receipt_reference: String,
    pub recommendation_reference: String,
    pub package_seal_digest: String,
    pub acceptance_reference: String,
    pub compatibility_version: String,
    pub created_at: String,
    /// `observed` | `ready_for_future_evaluation` | `blocked` | `withdrawn`
    pub state: String,
    /// DE-owned lifecycle — distinct from DecisionCandidate outcome lifecycle.
    pub lifecycle: DecisionEngineIntakeCandidateLifecycle,
    /// Always false — IntakeCandidate ≠ DecisionCandidate.
    pub is_decision_candidate: bool,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeCandidate {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_OBSERVED: &'static str = "observed";
    pub const STATE_READY: &'static str = "ready_for_future_evaluation";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_WITHDRAWN: &'static str = "withdrawn";
    pub const ID_PREFIX: &'static str = "engine_decision_intake:";
    pub const RECEIPT_REF_PREFIX: &'static str = "decision_engine_intake_receipt:";
    pub const ACCEPTANCE_REF_PREFIX: &'static str = "recommendation_decision_engine_acceptance:";

    pub fn synthetic_id(recommendation_id: &str) -> String {
        format!("{}{recommendation_id}", Self::ID_PREFIX)
    }

    /// Create only when receipt, assessment, and eligibility all clear the gate.
    /// New eligible intake candidates start lifecycle `active`.
    /// Never creates a DecisionCandidate / goal / intent / planner handoff.
    pub fn try_create(
        receipt: &DecisionEngineIntakeReceipt,
        assessment: &DecisionEngineIntakeAssessment,
        eligibility: &DecisionEngineIntakeEligibility,
        created_at: impl Into<String>,
    ) -> Option<Self> {
        if !Self::creation_allowed(receipt, assessment, eligibility) {
            return None;
        }
        let created_at = created_at.into();
        let recommendation_id = receipt.recommendation_id.clone();
        Some(Self {
            intake_candidate_id: Self::synthetic_id(&recommendation_id),
            workspace_id: receipt.workspace_id.clone(),
            intake_receipt_reference: format!("{}{recommendation_id}", Self::RECEIPT_REF_PREFIX),
            recommendation_reference: recommendation_id.clone(),
            package_seal_digest: receipt.sealed_intake_package_digest.clone(),
            acceptance_reference: format!("{}{recommendation_id}", Self::ACCEPTANCE_REF_PREFIX),
            compatibility_version: receipt.contract_version.clone(),
            created_at: created_at.clone(),
            state: Self::STATE_READY.into(),
            lifecycle: DecisionEngineIntakeCandidateLifecycle::start_active(created_at),
            is_decision_candidate: false,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: "Decision Engine intake candidate acknowledged eligible Recommendation Engine \
                   sealed package for future evaluation. Not a DecisionCandidate, goal, intent, \
                   planner handoff, Gateway grant, or ownership transfer."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn creation_allowed(
        receipt: &DecisionEngineIntakeReceipt,
        assessment: &DecisionEngineIntakeAssessment,
        eligibility: &DecisionEngineIntakeEligibility,
    ) -> bool {
        receipt.is_observed()
            && receipt.seal_aligned
            && assessment.seal_valid
            && assessment.acceptance_active
            && assessment.receipt_current
            && !assessment.is_duplicate
            && !assessment.is_superseded
            && !assessment.is_stale
            && (assessment.assessment_state == DecisionEngineIntakeAssessment::STATE_ELIGIBLE
                || assessment.eligible_for_future_candidate)
            && eligibility.is_eligible
            && eligibility.eligibility_state == DecisionEngineIntakeEligibility::STATE_ELIGIBLE
            && eligibility.seal_aligned
            && eligibility.acceptance_active
            && !eligibility.is_duplicate
            && !eligibility.is_superseded
            && !eligibility.is_stale
            && receipt.recommendation_id == assessment.recommendation_id
            && receipt.recommendation_id == eligibility.recommendation_id
            && receipt.sealed_intake_package_digest == assessment.sealed_intake_package_digest
            && receipt.sealed_intake_package_digest == eligibility.sealed_intake_package_digest
    }

    /// Create intake candidates for eligible intakes only (deterministic order).
    pub fn create_batch(
        receipts: &[DecisionEngineIntakeReceipt],
        assessments: &[DecisionEngineIntakeAssessment],
        eligibilities: &[DecisionEngineIntakeEligibility],
        created_at: &str,
    ) -> Vec<Self> {
        use std::collections::HashMap;
        let receipt_by_id: HashMap<&str, &DecisionEngineIntakeReceipt> = receipts
            .iter()
            .map(|r| (r.recommendation_id.as_str(), r))
            .collect();
        let assessment_by_id: HashMap<&str, &DecisionEngineIntakeAssessment> = assessments
            .iter()
            .map(|a| (a.recommendation_id.as_str(), a))
            .collect();
        let mut created: Vec<Self> = eligibilities
            .iter()
            .filter(|e| e.is_eligible)
            .filter_map(|eligibility| {
                let id = eligibility.recommendation_id.as_str();
                let receipt = receipt_by_id.get(id)?;
                let assessment = assessment_by_id.get(id)?;
                Self::try_create(receipt, assessment, eligibility, created_at)
            })
            .collect();
        created.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        created
    }

    /// DE-owned withdrawal — only mutates Decision Engine lifecycle state.
    pub fn withdraw(&mut self, at: impl Into<String>) -> Result<(), DecisionEngineError> {
        self.lifecycle.withdraw(at)?;
        self.state = Self::STATE_WITHDRAWN.into();
        self.note = "Decision Engine withdrew intake candidate. Recommendation Engine overlays \
                     were not mutated. No DecisionCandidate, goal, intent, planner handoff, \
                     or ownership transfer."
            .into();
        Ok(())
    }

    /// Apply source-driven invalidation (seal mismatch / revoked / superseded).
    /// Never reactivates an invalidated lifecycle.
    pub fn apply_source_reevaluation(
        &mut self,
        eligibility: Option<&DecisionEngineIntakeEligibility>,
        acceptance_state: Option<&str>,
        at: impl Into<String>,
    ) {
        let at = at.into();
        if self.lifecycle.is_invalidated() {
            return;
        }
        if eligibility.is_some_and(|e| {
            e.is_eligible && e.sealed_intake_package_digest == self.package_seal_digest
        }) {
            // Eligible again: keep withdrawn as DE decision; never revive invalidated.
            if self.lifecycle.is_active() {
                self.state = Self::STATE_READY.into();
            }
            return;
        }

        let reason = if eligibility.is_some_and(|e| e.is_superseded) {
            DecisionEngineIntakeCandidateLifecycle::REASON_SUPERSEDED
        } else if matches!(
            acceptance_state,
            Some(
                crate::workspace_recommendation::RecommendationDecisionEngineAcceptance::STATE_REVOKED
                | crate::workspace_recommendation::RecommendationDecisionEngineAcceptance::STATE_DECLINED,
            )
        ) {
            DecisionEngineIntakeCandidateLifecycle::REASON_ACCEPTANCE_REVOKED
        } else {
            DecisionEngineIntakeCandidateLifecycle::REASON_SEAL_MISMATCH
        };

        let _ = self.lifecycle.invalidate(reason, at);
        self.state = Self::STATE_BLOCKED.into();
        self.note = format!(
            "Decision Engine invalidated intake candidate ({reason}). Recommendation Engine \
             overlays were not mutated. No DecisionCandidate, goal, intent, planner handoff, \
             or ownership transfer."
        );
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_planner_handoff(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_planner_handoff(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_intake_only(&self) -> Result<(), DecisionEngineError> {
        if self.is_decision_candidate
            || self.creates_decision_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.lifecycle.authority_effect
                != DecisionEngineIntakeCandidateLifecycle::AUTHORITY_EFFECT_NONE
            || !self.intake_candidate_id.starts_with(Self::ID_PREFIX)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned evaluation record for an active intake candidate.
///
/// Answers whether Decision Engine examined the intake candidate and the outcome.
/// Never grants planning authority, creates DecisionCandidates, goals, intents,
/// or mutates Recommendation Engine records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeEvaluation {
    pub evaluation_id: String,
    pub workspace_id: String,
    pub intake_candidate_id: String,
    pub evaluated_at: String,
    /// `evaluated` | `rejected` | `deferred`
    pub evaluation_state: String,
    pub evaluation_reason: String,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeEvaluation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_EVALUATED: &'static str = "evaluated";
    pub const STATE_REJECTED: &'static str = "rejected";
    pub const STATE_DEFERRED: &'static str = "deferred";
    pub const ID_PREFIX: &'static str = "engine_decision_intake_eval:";

    pub fn synthetic_id(intake_candidate_id: &str) -> String {
        format!("{}{intake_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_valid_state(state: &str) -> bool {
        matches!(
            state,
            Self::STATE_EVALUATED | Self::STATE_REJECTED | Self::STATE_DEFERRED
        )
    }

    /// Evaluate only an active intake candidate. Withdrawn/invalidated are rejected.
    pub fn try_evaluate(
        candidate: &DecisionEngineIntakeCandidate,
        evaluation_state: impl Into<String>,
        evaluation_reason: impl Into<String>,
        evaluated_at: impl Into<String>,
    ) -> Result<Self, DecisionEngineError> {
        if !candidate.lifecycle.is_active() {
            return Err(DecisionEngineError::InvalidTransition {
                from: candidate.lifecycle.lifecycle_state.clone(),
                to: "evaluate".into(),
            });
        }
        let evaluation_state = evaluation_state.into();
        if !Self::is_valid_state(&evaluation_state) {
            return Err(DecisionEngineError::InvalidOutcome(evaluation_state));
        }
        let evaluated_at = evaluated_at.into();
        let evaluation_reason = evaluation_reason.into();
        Ok(Self {
            evaluation_id: Self::synthetic_id(&candidate.intake_candidate_id),
            workspace_id: candidate.workspace_id.clone(),
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            evaluated_at,
            evaluation_state: evaluation_state.clone(),
            evaluation_reason,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: format!(
                "Decision Engine intake evaluation ({evaluation_state}). Examination record only — \
                 not DecisionCandidate creation, goal/intent creation, planner handoff, Gateway \
                 grant, adapter invocation, or ownership transfer."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_evaluation_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !Self::is_valid_state(&self.evaluation_state)
            || !self.evaluation_id.starts_with(Self::ID_PREFIX)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned disposition for an evaluated intake candidate.
///
/// Records what Decision Engine does with an examined intake (retain / dismiss / defer).
/// Never creates DecisionCandidates, goals, intents, planner work, or execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakeDisposition {
    pub disposition_id: String,
    pub workspace_id: String,
    pub intake_candidate_id: String,
    pub evaluation_id: String,
    pub disposed_at: String,
    /// `retained` | `dismissed` | `deferred`
    pub disposition_state: String,
    pub disposition_reason: String,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakeDisposition {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_RETAINED: &'static str = "retained";
    pub const STATE_DISMISSED: &'static str = "dismissed";
    pub const STATE_DEFERRED: &'static str = "deferred";
    pub const ID_PREFIX: &'static str = "engine_decision_intake_disp:";

    pub fn synthetic_id(intake_candidate_id: &str) -> String {
        format!("{}{intake_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_valid_state(state: &str) -> bool {
        matches!(
            state,
            Self::STATE_RETAINED | Self::STATE_DISMISSED | Self::STATE_DEFERRED
        )
    }

    /// Dispose only an active, evaluated intake candidate.
    /// Withdrawn, invalidated, and unevaluated intakes are rejected.
    pub fn try_dispose(
        candidate: &DecisionEngineIntakeCandidate,
        evaluation: &DecisionEngineIntakeEvaluation,
        disposition_state: impl Into<String>,
        disposition_reason: impl Into<String>,
        disposed_at: impl Into<String>,
    ) -> Result<Self, DecisionEngineError> {
        if !candidate.lifecycle.is_active() {
            return Err(DecisionEngineError::InvalidTransition {
                from: candidate.lifecycle.lifecycle_state.clone(),
                to: "dispose".into(),
            });
        }
        if evaluation.intake_candidate_id != candidate.intake_candidate_id
            || evaluation.workspace_id != candidate.workspace_id
            || !DecisionEngineIntakeEvaluation::is_valid_state(&evaluation.evaluation_state)
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: "unevaluated".into(),
                to: "dispose".into(),
            });
        }
        let disposition_state = disposition_state.into();
        if !Self::is_valid_state(&disposition_state) {
            return Err(DecisionEngineError::InvalidOutcome(disposition_state));
        }
        let disposed_at = disposed_at.into();
        let disposition_reason = disposition_reason.into();
        Ok(Self {
            disposition_id: Self::synthetic_id(&candidate.intake_candidate_id),
            workspace_id: candidate.workspace_id.clone(),
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            evaluation_id: evaluation.evaluation_id.clone(),
            disposed_at,
            disposition_state: disposition_state.clone(),
            disposition_reason,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: format!(
                "Decision Engine intake disposition ({disposition_state}). Lifecycle decision only — \
                 not DecisionCandidate creation, goal/intent creation, planner handoff, Gateway \
                 grant, adapter invocation, or ownership transfer."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_disposition_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !Self::is_valid_state(&self.disposition_state)
            || !self.disposition_id.starts_with(Self::ID_PREFIX)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Grounded inputs for recomputing a promotion boundary (not persisted).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionEngineIntakePromotionBoundaryInput {
    pub candidate: DecisionEngineIntakeCandidate,
    pub evaluation: Option<DecisionEngineIntakeEvaluation>,
    pub disposition: Option<DecisionEngineIntakeDisposition>,
    pub acceptance_active: bool,
    pub seal_aligned: bool,
    /// Future promotion event marker — never set by this boundary alone.
    pub previously_promoted: bool,
}

/// DE-owned boundary: when an intake artifact MAY become eligible for promotion
/// into the native DecisionCandidate domain.
///
/// This is not promotion, DecisionCandidate creation, scoring, or planning.
/// Fully derivable from intake candidate + evaluation + disposition + seal/acceptance facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineIntakePromotionBoundary {
    pub workspace_id: String,
    pub intake_candidate_id: String,
    pub recommendation_reference: String,
    /// `not_ready` | `promotion_allowed` | `promotion_blocked` | `promoted`
    pub boundary_state: String,
    pub evaluation_complete: bool,
    pub disposition_retained: bool,
    pub intake_active: bool,
    pub acceptance_active: bool,
    pub seal_aligned: bool,
    pub evidence: Vec<String>,
    pub creates_decision_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineIntakePromotionBoundary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_NOT_READY: &'static str = "not_ready";
    pub const STATE_PROMOTION_ALLOWED: &'static str = "promotion_allowed";
    pub const STATE_PROMOTION_BLOCKED: &'static str = "promotion_blocked";
    pub const STATE_PROMOTED: &'static str = "promoted";

    pub fn is_promotion_allowed(&self) -> bool {
        self.boundary_state == Self::STATE_PROMOTION_ALLOWED
    }

    pub fn derive_batch(
        inputs: &[DecisionEngineIntakePromotionBoundaryInput],
    ) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        out
    }

    pub fn derive(input: &DecisionEngineIntakePromotionBoundaryInput) -> Self {
        let candidate = &input.candidate;
        let intake_active = candidate.lifecycle.is_active();
        let evaluation_complete = input
            .evaluation
            .as_ref()
            .is_some_and(|e| {
                e.intake_candidate_id == candidate.intake_candidate_id
                    && e.evaluation_state == DecisionEngineIntakeEvaluation::STATE_EVALUATED
            });
        let disposition_retained = input.disposition.as_ref().is_some_and(|d| {
            d.intake_candidate_id == candidate.intake_candidate_id
                && d.disposition_state == DecisionEngineIntakeDisposition::STATE_RETAINED
        });
        let disposition_declined = input.disposition.as_ref().is_some_and(|d| {
            d.intake_candidate_id == candidate.intake_candidate_id
                && (d.disposition_state == DecisionEngineIntakeDisposition::STATE_DISMISSED
                    || d.disposition_state == DecisionEngineIntakeDisposition::STATE_DEFERRED)
        });

        let mut evidence = Vec::new();
        if intake_active {
            evidence.push("intake_active".into());
        } else if candidate.lifecycle.is_withdrawn() {
            evidence.push("intake_withdrawn".into());
        } else if candidate.lifecycle.is_invalidated() {
            evidence.push("intake_invalidated".into());
        }
        if evaluation_complete {
            evidence.push("evaluation_complete".into());
        } else {
            evidence.push("evaluation_incomplete".into());
        }
        if disposition_retained {
            evidence.push("disposition_retained".into());
        } else if disposition_declined {
            evidence.push("disposition_not_retained".into());
        } else {
            evidence.push("disposition_missing".into());
        }
        if input.acceptance_active {
            evidence.push("acceptance_active".into());
        } else {
            evidence.push("acceptance_inactive".into());
        }
        if input.seal_aligned {
            evidence.push("seal_aligned".into());
        } else {
            evidence.push("seal_mismatch".into());
        }

        let boundary_state = if input.previously_promoted {
            evidence.push("previously_promoted".into());
            Self::STATE_PROMOTED
        } else if candidate.lifecycle.is_withdrawn()
            || candidate.lifecycle.is_invalidated()
            || !input.acceptance_active
            || !input.seal_aligned
            || disposition_declined
        {
            Self::STATE_PROMOTION_BLOCKED
        } else if !evaluation_complete || !disposition_retained || !intake_active {
            Self::STATE_NOT_READY
        } else {
            evidence.push("promotion_allowed".into());
            Self::STATE_PROMOTION_ALLOWED
        };

        Self {
            workspace_id: candidate.workspace_id.clone(),
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            recommendation_reference: candidate.recommendation_reference.clone(),
            boundary_state: boundary_state.into(),
            evaluation_complete,
            disposition_retained,
            intake_active,
            acceptance_active: input.acceptance_active,
            seal_aligned: input.seal_aligned,
            evidence,
            creates_decision_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: format!(
                "Decision Engine intake promotion boundary ({boundary_state}). Readiness only — \
                 not DecisionCandidate creation, scoring, planner handoff, Gateway grant, \
                 goal/intent creation, or ownership transfer."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
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

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
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

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_boundary_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || (self.is_promotion_allowed()
                && self.boundary_state != Self::STATE_PROMOTION_ALLOWED)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned request that an eligible intake artifact seek conversion into
/// the native DecisionCandidate domain.
///
/// This is not DecisionCandidate creation, scoring, planner handoff, or execution.
/// Fully derivable from the promotion boundary — never persisted in this increment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineCandidateCreationRequest {
    pub request_id: String,
    pub workspace_id: String,
    pub intake_candidate_id: String,
    pub recommendation_reference: String,
    /// `not_requested` | `requested` | `rejected` | `created`
    pub request_state: String,
    pub promotion_boundary_state: String,
    pub disposition_retained: bool,
    pub acceptance_active: bool,
    pub seal_aligned: bool,
    pub evidence: Vec<String>,
    pub creates_decision_candidate: bool,
    pub creates_decision_score: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionEngineCandidateCreationRequest {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_NOT_REQUESTED: &'static str = "not_requested";
    pub const STATE_REQUESTED: &'static str = "requested";
    pub const STATE_REJECTED: &'static str = "rejected";
    pub const STATE_CREATED: &'static str = "created";
    pub const ID_PREFIX: &'static str = "engine_decision_intake_creation_request:";

    pub fn synthetic_id(intake_candidate_id: &str) -> String {
        format!("{}{intake_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_requested(&self) -> bool {
        self.request_state == Self::STATE_REQUESTED
    }

    pub fn derive_batch(
        boundaries: &[DecisionEngineIntakePromotionBoundary],
    ) -> Vec<Self> {
        let mut out: Vec<Self> = boundaries.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        out
    }

    /// Derive creation-request state from promotion boundary facts only.
    /// Never sets `created` — reserved for a future DecisionCandidate creation event.
    pub fn derive(boundary: &DecisionEngineIntakePromotionBoundary) -> Self {
        let request_state = match boundary.boundary_state.as_str() {
            DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_ALLOWED
                if boundary.disposition_retained
                    && boundary.acceptance_active
                    && boundary.seal_aligned
                    && boundary.intake_active =>
            {
                Self::STATE_REQUESTED
            }
            DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED => Self::STATE_REJECTED,
            _ => Self::STATE_NOT_REQUESTED,
        };

        let mut evidence = boundary.evidence.clone();
        match request_state {
            Self::STATE_REQUESTED => evidence.push("creation_requested".into()),
            Self::STATE_REJECTED => evidence.push("creation_rejected".into()),
            _ => evidence.push("creation_not_requested".into()),
        }

        Self {
            request_id: Self::synthetic_id(&boundary.intake_candidate_id),
            workspace_id: boundary.workspace_id.clone(),
            intake_candidate_id: boundary.intake_candidate_id.clone(),
            recommendation_reference: boundary.recommendation_reference.clone(),
            request_state: request_state.into(),
            promotion_boundary_state: boundary.boundary_state.clone(),
            disposition_retained: boundary.disposition_retained,
            acceptance_active: boundary.acceptance_active,
            seal_aligned: boundary.seal_aligned,
            evidence,
            creates_decision_candidate: false,
            creates_decision_score: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: format!(
                "Decision Engine candidate creation request ({request_state}). Request only — \
                 not DecisionCandidate creation, scoring, planner handoff, Gateway grant, \
                 goal/intent creation, or ownership transfer."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_create_decision_candidate(&self) -> bool {
        false
    }

    pub fn may_create_decision_score(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_transfer_ownership(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_candidate(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_decision_score(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_transfer_ownership(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_request_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_candidate
            || self.creates_decision_score
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || self.request_state == Self::STATE_CREATED
            || !self.request_id.starts_with(Self::ID_PREFIX)
            || (self.is_requested() && self.request_state != Self::STATE_REQUESTED)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}
