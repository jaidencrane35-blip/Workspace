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

    /// Actionable outcomes for Intelligence / compact summary candidates.
    pub fn is_actionable(self) -> bool {
        matches!(self, Self::Open | Self::Postponed)
    }

    /// Terminal decision evidence — projects into history, never into actionable candidates.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Selected | Self::Dismissed | Self::Expired)
    }
}

/// Compact non-actionable terminal Decision Engine evidence for projection consumers.
///
/// History is read-only continuity — never executable, never merged into
/// `top_candidates`, and never a second lifecycle authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionArtifactHistoryEntry {
    /// Artifact identity (DecisionCandidate id / synthetic engine id).
    pub artifact_id: String,
    pub candidate_id: String,
    /// Source key used by the lifecycle overlay (`attention:…`, `intake:…`, …).
    pub candidate_key: String,
    /// Terminal outcome (`selected` / `dismissed` / `expired`).
    pub decision_state: String,
    pub created_at: String,
    pub updated_at: String,
    /// Resolution identity — same vocabulary as `decision_state` for DE outcomes.
    pub resolution_type: String,
    pub origin: String,
    pub recommendation_id: Option<String>,
    pub intake_candidate_id: Option<String>,
    pub package_seal_digest: Option<String>,
    /// Always `true` for projected history entries.
    pub terminal: bool,
    /// Always `false` — terminal history never joins actionable candidates.
    pub actionable: bool,
    pub authority_effect: String,
}

impl DecisionArtifactHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_candidate(
        candidate: &DecisionCandidate,
        updated_at: Option<&str>,
    ) -> Option<Self> {
        if !candidate.outcome.is_terminal() {
            return None;
        }
        let artifact_id = candidate.id.to_string();
        let candidate_key = artifact_id
            .strip_prefix("engine_decision:")
            .unwrap_or(artifact_id.as_str())
            .to_string();
        let updated = updated_at
            .filter(|s| !s.is_empty())
            .unwrap_or(candidate.created_at.as_str())
            .to_string();
        Some(Self {
            artifact_id: artifact_id.clone(),
            candidate_id: artifact_id,
            candidate_key,
            decision_state: candidate.outcome.as_str().into(),
            created_at: candidate.created_at.clone(),
            updated_at: updated,
            resolution_type: candidate.outcome.as_str().into(),
            origin: candidate.origin.clone(),
            recommendation_id: candidate.recommendation_id.clone(),
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Project retained terminal overlay evidence when no live candidate payload remains.
    pub fn from_overlay(overlay: &DecisionEngineOverlay) -> Option<Self> {
        if !overlay.outcome.is_terminal() {
            return None;
        }
        let artifact_id = DecisionCandidate::synthetic_id(&overlay.candidate_key).to_string();
        Some(Self {
            artifact_id: artifact_id.clone(),
            candidate_id: artifact_id,
            candidate_key: overlay.candidate_key.clone(),
            decision_state: overlay.outcome.as_str().into(),
            created_at: overlay.updated_at.clone(),
            updated_at: overlay.updated_at.clone(),
            resolution_type: overlay.outcome.as_str().into(),
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            recommendation_id: None,
            intake_candidate_id: None,
            package_seal_digest: None,
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && matches!(
                self.decision_state.as_str(),
                "selected" | "dismissed" | "expired"
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

fn default_decision_candidate_origin() -> String {
    DecisionCandidate::ORIGIN_NATIVE.into()
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
    /// Provenance to DE intake candidate when created from intake (else None).
    #[serde(default)]
    pub intake_candidate_id: Option<String>,
    /// Provenance to DE candidate creation request when created from intake (else None).
    #[serde(default)]
    pub creation_request_id: Option<String>,
    /// Recommendation package seal digest when created from intake (else None).
    #[serde(default)]
    pub package_seal_digest: Option<String>,
    /// `native` | `recommendation_intake` — origin remains visible; never merged.
    #[serde(default = "default_decision_candidate_origin")]
    pub origin: String,
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
    /// Empty handoff — intake-created candidates never invoke planner in this increment.
    pub const HANDOFF_NONE: &'static str = "";
    pub const ORIGIN_NATIVE: &'static str = "native";
    pub const ORIGIN_RECOMMENDATION_INTAKE: &'static str = "recommendation_intake";

    pub fn synthetic_id(source_key: &str) -> DecisionCandidateId {
        DecisionCandidateId::new(format!("engine_decision:{source_key}"))
            .expect("synthetic decision candidate id is valid")
    }

    /// Unscored placeholder — intake creation must not invent ranking behaviour.
    pub fn unscored() -> DecisionScore {
        DecisionScore {
            total: 0,
            attention_contribution: 0,
            memory_contribution: 0,
            personalization_contribution: 0,
            goal_contribution: 0,
            factors: Vec::new(),
        }
    }

    pub fn is_recommendation_intake(&self) -> bool {
        self.origin == Self::ORIGIN_RECOMMENDATION_INTAKE
            || self.id.as_str().starts_with("engine_decision:intake:")
    }

    pub fn is_native_origin(&self) -> bool {
        !self.is_recommendation_intake()
    }

    pub fn has_complete_intake_provenance(&self) -> bool {
        self.intake_candidate_id
            .as_ref()
            .is_some_and(|v| !v.is_empty())
            && self
                .creation_request_id
                .as_ref()
                .is_some_and(|v| !v.is_empty())
            && self
                .package_seal_digest
                .as_ref()
                .is_some_and(|v| !v.is_empty())
            && self
                .recommendation_id
                .as_ref()
                .is_some_and(|v| !v.is_empty())
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
    /// Terminal decision evidence (selected/dismissed/expired) — never actionable.
    #[serde(default)]
    pub history: Vec<DecisionArtifactHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
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
    /// DE-owned candidate creation boundary — may create DecisionCandidate without scoring.
    #[serde(default)]
    pub candidate_creations: Vec<DecisionEngineCandidateCreation>,
    /// DE-owned lifecycle integration — origin-aware DE lifecycle without merging sources.
    #[serde(default)]
    pub lifecycle_integrations: Vec<DecisionCandidateLifecycleIntegration>,
    /// DE-owned evaluation-origin contracts — origin rules only, never scoring.
    #[serde(default)]
    pub evaluation_origin_contracts: Vec<DecisionCandidateEvaluationOriginContract>,
    /// DE-owned evaluation resolutions — scoring-path admission only, never scores.
    #[serde(default)]
    pub evaluation_resolutions: Vec<DecisionCandidateEvaluationResolution>,
    /// DE-owned DecisionScore results for accepted candidates — never ranking/selection.
    #[serde(default)]
    pub candidate_scores: Vec<DecisionCandidateScore>,
    /// DE-owned comparative ranking of scored candidates — never selection/planner.
    #[serde(default)]
    pub candidate_ranking: Option<DecisionCandidateRanking>,
    /// DE-owned selection decisions after ranking — never execution/planner.
    #[serde(default)]
    pub candidate_selections: Vec<DecisionCandidateSelection>,
    /// DE-owned progression requests after selection — never planner/execution.
    #[serde(default)]
    pub progression_requests: Vec<DecisionCandidateProgressionRequest>,
    /// DE-owned progression acknowledgements — never planner/execution.
    #[serde(default)]
    pub progression_acknowledgements: Vec<DecisionCandidateProgressionAcknowledgement>,
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
            .filter(|c| c.outcome.is_actionable())
            .cloned()
            .collect();
        let top_candidates: Vec<_> = open.iter().take(5).cloned().collect();
        let history = Self::project_history_from_candidates(&candidates, &[]);
        let history_count = history.len();
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
            history,
            history_count,
            intake_receipts: Vec::new(),
            intake_assessments: Vec::new(),
            intake_eligibilities: Vec::new(),
            intake_candidates: Vec::new(),
            intake_evaluations: Vec::new(),
            intake_dispositions: Vec::new(),
            intake_promotion_boundaries: Vec::new(),
            candidate_creation_requests: Vec::new(),
            candidate_creations: Vec::new(),
            lifecycle_integrations: Vec::new(),
            evaluation_origin_contracts: Vec::new(),
            evaluation_resolutions: Vec::new(),
            candidate_scores: Vec::new(),
            candidate_ranking: None,
            candidate_selections: Vec::new(),
            progression_requests: Vec::new(),
            progression_acknowledgements: Vec::new(),
            summary,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Project terminal evidence from live candidates plus retained orphan overlays.
    pub fn project_history_from_candidates(
        candidates: &[DecisionCandidate],
        overlays: &[DecisionEngineOverlay],
    ) -> Vec<DecisionArtifactHistoryEntry> {
        let overlay_by_key: std::collections::HashMap<&str, &DecisionEngineOverlay> = overlays
            .iter()
            .map(|o| (o.candidate_key.as_str(), o))
            .collect();
        let mut seen_keys = std::collections::HashSet::new();
        let mut history = Vec::new();

        for candidate in candidates {
            let key = candidate
                .id
                .as_str()
                .strip_prefix("engine_decision:")
                .unwrap_or(candidate.id.as_str());
            let updated = overlay_by_key.get(key).map(|o| o.updated_at.as_str());
            if let Some(entry) = DecisionArtifactHistoryEntry::from_candidate(candidate, updated) {
                seen_keys.insert(entry.candidate_key.clone());
                history.push(entry);
            }
        }

        for overlay in overlays {
            if seen_keys.contains(&overlay.candidate_key) {
                continue;
            }
            if let Some(entry) = DecisionArtifactHistoryEntry::from_overlay(overlay) {
                seen_keys.insert(entry.candidate_key.clone());
                history.push(entry);
            }
        }

        history.sort_by(|a, b| {
            b.updated_at
                .cmp(&a.updated_at)
                .then(a.candidate_key.cmp(&b.candidate_key))
        });
        history
    }

    /// Attach terminal history without altering actionable `top_candidates`.
    pub fn with_terminal_history(mut self, history: Vec<DecisionArtifactHistoryEntry>) -> Self {
        self.history_count = history.len();
        self.history = history;
        self
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

    /// Attach DE-owned candidate creation records (may reference created DecisionCandidates).
    pub fn with_candidate_creations(
        mut self,
        creations: Vec<DecisionEngineCandidateCreation>,
    ) -> Self {
        let created = creations.iter().filter(|c| c.is_created()).count();
        self.summary = format!(
            "{} Candidate creations: {} ({} created).",
            self.summary,
            creations.len(),
            created
        );
        self.candidate_creations = creations;
        self
    }

    /// Attach DE-owned lifecycle integrations without scoring or planner handoff.
    pub fn with_lifecycle_integrations(
        mut self,
        integrations: Vec<DecisionCandidateLifecycleIntegration>,
    ) -> Self {
        let integrated = integrations.iter().filter(|i| i.is_integrated()).count();
        self.summary = format!(
            "{} Lifecycle integrations: {} ({} integrated).",
            self.summary,
            integrations.len(),
            integrated
        );
        self.lifecycle_integrations = integrations;
        self
    }

    /// Attach DE-owned evaluation-origin contracts without scoring or ranking.
    pub fn with_evaluation_origin_contracts(
        mut self,
        contracts: Vec<DecisionCandidateEvaluationOriginContract>,
    ) -> Self {
        let eligible = contracts.iter().filter(|c| c.is_eligible_for_evaluation()).count();
        let evaluated = contracts.iter().filter(|c| c.is_evaluated()).count();
        self.summary = format!(
            "{} Evaluation origin contracts: {} ({} eligible, {} evaluated).",
            self.summary,
            contracts.len(),
            eligible,
            evaluated
        );
        self.evaluation_origin_contracts = contracts;
        self
    }

    /// Attach DE-owned evaluation resolutions without creating scores or ranks.
    pub fn with_evaluation_resolutions(
        mut self,
        resolutions: Vec<DecisionCandidateEvaluationResolution>,
    ) -> Self {
        let accepted = resolutions.iter().filter(|r| r.is_accepted_for_scoring()).count();
        self.summary = format!(
            "{} Evaluation resolutions: {} ({} accepted_for_scoring).",
            self.summary,
            resolutions.len(),
            accepted
        );
        self.evaluation_resolutions = resolutions;
        self
    }

    /// Attach DE-owned DecisionScore results without ranking, selecting, or planner handoff.
    pub fn with_candidate_scores(mut self, scores: Vec<DecisionCandidateScore>) -> Self {
        self.summary = format!("{} DecisionScores: {}.", self.summary, scores.len());
        self.candidate_scores = scores;
        self
    }

    /// Attach DE-owned comparative ranking without selecting or planner handoff.
    pub fn with_candidate_ranking(mut self, ranking: DecisionCandidateRanking) -> Self {
        self.summary = format!(
            "{} Candidate ranking: {} entr{}.",
            self.summary,
            ranking.entries.len(),
            if ranking.entries.len() == 1 { "y" } else { "ies" }
        );
        self.candidate_ranking = Some(ranking);
        self
    }

    /// Attach DE-owned selection decisions without execution or planner handoff.
    pub fn with_candidate_selections(mut self, selections: Vec<DecisionCandidateSelection>) -> Self {
        let selected = selections.iter().filter(|s| s.is_selected()).count();
        self.summary = format!(
            "{} Candidate selections: {} ({} selected).",
            self.summary,
            selections.len(),
            selected
        );
        self.candidate_selections = selections;
        self
    }

    /// Attach DE-owned progression requests without planner handoff or execution.
    pub fn with_progression_requests(
        mut self,
        requests: Vec<DecisionCandidateProgressionRequest>,
    ) -> Self {
        let requested = requests.iter().filter(|r| r.is_requested()).count();
        self.summary = format!(
            "{} Progression requests: {} ({} requested).",
            self.summary,
            requests.len(),
            requested
        );
        self.progression_requests = requests;
        self
    }

    /// Attach DE-owned progression acknowledgements without planner or execution.
    pub fn with_progression_acknowledgements(
        mut self,
        acknowledgements: Vec<DecisionCandidateProgressionAcknowledgement>,
    ) -> Self {
        let acknowledged = acknowledgements.iter().filter(|a| a.is_acknowledged()).count();
        self.summary = format!(
            "{} Progression acknowledgements: {} ({} acknowledged).",
            self.summary,
            acknowledgements.len(),
            acknowledged
        );
        self.progression_acknowledgements = acknowledgements;
        self
    }

    pub fn summary_projection(&self, limit: usize) -> DecisionEngineSummary {
        let actionable: Vec<DecisionCandidate> = self
            .candidates
            .iter()
            .filter(|c| c.outcome.is_actionable())
            .cloned()
            .collect();
        // Actionable surface excludes terminals; history remains visible so Intelligence
        // cannot treat "no top_candidates" as "no decision evidence".
        let history: Vec<DecisionArtifactHistoryEntry> =
            self.history.iter().take(limit).cloned().collect();
        DecisionEngineSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            // Surface count reflects still-actionable recommendations.
            candidate_count: actionable.len(),
            open_count: actionable.len(),
            top_candidates: actionable.into_iter().take(limit).collect(),
            history,
            history_count: self.history_count,
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
    /// Actionable outcomes only (`open` / `postponed`).
    pub top_candidates: Vec<DecisionCandidate>,
    /// Truncated terminal decision evidence (never actionable).
    #[serde(default)]
    pub history: Vec<DecisionArtifactHistoryEntry>,
    /// Authoritative terminal evidence count (may exceed `history.length`).
    #[serde(default)]
    pub history_count: usize,
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
            history: Vec::new(),
            history_count: 0,
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

    pub fn ensure_not_recorded(
        existing: Option<&Self>,
        requested_state: &str,
    ) -> Result<(), DecisionEngineError> {
        if let Some(existing) = existing {
            return Err(DecisionEngineError::InvalidTransition {
                from: existing.evaluation_state.clone(),
                to: requested_state.into(),
            });
        }
        Ok(())
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
        Self::derive_batch_with_created(boundaries, &[])
    }

    /// Derive requests; mark `created` when a DE candidate creation already exists.
    pub fn derive_batch_with_created(
        boundaries: &[DecisionEngineIntakePromotionBoundary],
        created_intake_candidate_ids: &[String],
    ) -> Vec<Self> {
        let mut out: Vec<Self> = boundaries
            .iter()
            .map(|b| {
                let already_created = created_intake_candidate_ids
                    .iter()
                    .any(|id| id == &b.intake_candidate_id);
                Self::derive_with_created(b, already_created)
            })
            .collect();
        out.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        out
    }

    /// Derive creation-request state from promotion boundary facts.
    /// Sets `created` only when a DecisionCandidate creation event already occurred.
    pub fn derive(boundary: &DecisionEngineIntakePromotionBoundary) -> Self {
        Self::derive_with_created(boundary, false)
    }

    pub fn derive_with_created(
        boundary: &DecisionEngineIntakePromotionBoundary,
        already_created: bool,
    ) -> Self {
        let request_state = if already_created {
            Self::STATE_CREATED
        } else {
            match boundary.boundary_state.as_str() {
                DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_ALLOWED
                    if boundary.disposition_retained
                        && boundary.acceptance_active
                        && boundary.seal_aligned
                        && boundary.intake_active =>
                {
                    Self::STATE_REQUESTED
                }
                DecisionEngineIntakePromotionBoundary::STATE_PROMOTION_BLOCKED => {
                    Self::STATE_REJECTED
                }
                DecisionEngineIntakePromotionBoundary::STATE_PROMOTED => Self::STATE_CREATED,
                _ => Self::STATE_NOT_REQUESTED,
            }
        };

        let mut evidence = boundary.evidence.clone();
        match request_state {
            Self::STATE_REQUESTED => evidence.push("creation_requested".into()),
            Self::STATE_REJECTED => evidence.push("creation_rejected".into()),
            Self::STATE_CREATED => evidence.push("creation_completed".into()),
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
            || !self.request_id.starts_with(Self::ID_PREFIX)
            || (self.is_requested() && self.request_state != Self::STATE_REQUESTED)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or performing DE-owned DecisionCandidate creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionEngineCandidateCreationInput {
    pub creation_request: DecisionEngineCandidateCreationRequest,
    pub intake_candidate: DecisionEngineIntakeCandidate,
    pub promotion_boundary: DecisionEngineIntakePromotionBoundary,
    pub package_seal_digest: String,
    /// When present, creation already persisted — project as `created`.
    pub existing_creation: Option<DecisionEngineCandidateCreation>,
}

/// DE-owned boundary for controlled creation of a native DecisionCandidate
/// from an approved intake creation request.
///
/// May create a DecisionCandidate. Must not score, plan, grant, or execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionEngineCandidateCreation {
    pub creation_id: String,
    pub workspace_id: String,
    pub intake_candidate_id: String,
    pub creation_request_id: String,
    pub recommendation_reference: String,
    pub package_seal_digest: String,
    /// `blocked` | `eligible_for_creation` | `created`
    pub creation_state: String,
    pub decision_candidate_id: Option<String>,
    pub title: Option<String>,
    pub goal_statement: Option<String>,
    pub created_at: Option<String>,
    pub evidence: Vec<String>,
    /// True only after a successful create in the `created` state.
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

impl DecisionEngineCandidateCreation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_ELIGIBLE: &'static str = "eligible_for_creation";
    pub const STATE_CREATED: &'static str = "created";
    pub const ID_PREFIX: &'static str = "engine_decision_creation:";
    pub const CANDIDATE_SOURCE_PREFIX: &'static str = "intake:";

    pub fn synthetic_id(intake_candidate_id: &str) -> String {
        format!("{}{intake_candidate_id}", Self::ID_PREFIX)
    }

    pub fn decision_candidate_source_key(recommendation_reference: &str) -> String {
        format!("{}{recommendation_reference}", Self::CANDIDATE_SOURCE_PREFIX)
    }

    pub fn is_created(&self) -> bool {
        self.creation_state == Self::STATE_CREATED
    }

    pub fn is_eligible_for_creation(&self) -> bool {
        self.creation_state == Self::STATE_ELIGIBLE
    }

    pub fn is_blocked(&self) -> bool {
        self.creation_state == Self::STATE_BLOCKED
    }

    pub fn derive_batch(inputs: &[DecisionEngineCandidateCreationInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        out
    }

    pub fn derive(input: &DecisionEngineCandidateCreationInput) -> Self {
        if let Some(existing) = &input.existing_creation {
            return existing.clone();
        }

        let request = &input.creation_request;
        let intake = &input.intake_candidate;
        let boundary = &input.promotion_boundary;
        let intake_active = intake.lifecycle.is_active();
        let provenance_intact = !input.package_seal_digest.is_empty()
            && input.package_seal_digest == intake.package_seal_digest
            && request.recommendation_reference == intake.recommendation_reference
            && request.intake_candidate_id == intake.intake_candidate_id
            && boundary.intake_candidate_id == intake.intake_candidate_id;

        let mut evidence = Vec::new();
        if request.is_requested() {
            evidence.push("creation_request_requested".into());
        } else if request.request_state == DecisionEngineCandidateCreationRequest::STATE_REJECTED {
            evidence.push("creation_request_rejected".into());
        } else if request.request_state == DecisionEngineCandidateCreationRequest::STATE_CREATED {
            evidence.push("creation_request_already_created".into());
        } else {
            evidence.push("creation_request_not_requested".into());
        }
        if intake_active {
            evidence.push("intake_active".into());
        } else if intake.lifecycle.is_withdrawn() {
            evidence.push("intake_withdrawn".into());
        } else if intake.lifecycle.is_invalidated() {
            evidence.push("intake_invalidated".into());
        } else {
            evidence.push("intake_inactive".into());
        }
        if request.acceptance_active && boundary.acceptance_active {
            evidence.push("acceptance_active".into());
        } else {
            evidence.push("acceptance_revoked".into());
        }
        if request.seal_aligned
            && boundary.seal_aligned
            && input.package_seal_digest == intake.package_seal_digest
        {
            evidence.push("seal_aligned".into());
        } else {
            evidence.push("seal_mismatch".into());
        }
        if boundary.is_promotion_allowed() {
            evidence.push("promotion_allowed".into());
        }
        if provenance_intact {
            evidence.push("provenance_intact".into());
        } else {
            evidence.push("provenance_broken".into());
        }

        let blocked = request.request_state
            == DecisionEngineCandidateCreationRequest::STATE_REJECTED
            || !intake_active
            || intake.lifecycle.is_invalidated()
            || !request.acceptance_active
            || !boundary.acceptance_active
            || !request.seal_aligned
            || !boundary.seal_aligned
            || input.package_seal_digest != intake.package_seal_digest
            || !provenance_intact;

        let eligible = !blocked
            && request.is_requested()
            && boundary.is_promotion_allowed()
            && intake_active
            && provenance_intact;

        let creation_state = if eligible {
            Self::STATE_ELIGIBLE
        } else {
            Self::STATE_BLOCKED
        };

        Self {
            creation_id: Self::synthetic_id(&intake.intake_candidate_id),
            workspace_id: intake.workspace_id.clone(),
            intake_candidate_id: intake.intake_candidate_id.clone(),
            creation_request_id: request.request_id.clone(),
            recommendation_reference: intake.recommendation_reference.clone(),
            package_seal_digest: input.package_seal_digest.clone(),
            creation_state: creation_state.into(),
            decision_candidate_id: None,
            title: None,
            goal_statement: None,
            created_at: None,
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
                "Decision Engine candidate creation ({creation_state}). Creation boundary only — \
                 scoring, planner handoff, Gateway grant, goal/intent creation, and execution \
                 remain forbidden."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Create a DE-owned DecisionCandidate from an eligible creation projection.
    /// Never scores, plans, grants, or mutates Recommendation Engine records.
    pub fn try_create(
        input: &DecisionEngineCandidateCreationInput,
        created_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if !projected.is_eligible_for_creation() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.creation_state,
                to: Self::STATE_CREATED.into(),
            });
        }
        if input.existing_creation.as_ref().is_some_and(|c| c.is_created()) {
            return Err(DecisionEngineError::InvalidTransition {
                from: Self::STATE_CREATED.into(),
                to: Self::STATE_CREATED.into(),
            });
        }

        let created_at = created_at.into();
        let source_key =
            Self::decision_candidate_source_key(&projected.recommendation_reference);
        let candidate_id = DecisionCandidate::synthetic_id(&source_key);
        let title = format!(
            "Intake recommendation: {}",
            projected.recommendation_reference
        );
        let goal_statement = format!(
            "Consider accepted recommendation package {} (seal {}).",
            projected.recommendation_reference, projected.package_seal_digest
        );

        let mut creation = projected;
        creation.creation_state = Self::STATE_CREATED.into();
        creation.decision_candidate_id = Some(candidate_id.as_str().to_string());
        creation.title = Some(title.clone());
        creation.goal_statement = Some(goal_statement.clone());
        creation.created_at = Some(created_at.clone());
        creation.creates_decision_candidate = true;
        creation.evidence.push("decision_candidate_created".into());
        creation.note = "Decision Engine created native DecisionCandidate from approved intake \
                         creation request. No scoring, planner handoff, Gateway grant, goal, \
                         intent, or Recommendation Engine mutation."
            .into();

        let workspace_id = WorkspaceId::new(&creation.workspace_id).map_err(|err| {
            DecisionEngineError::Domain(err)
        })?;

        let candidate = DecisionCandidate {
            id: candidate_id,
            workspace_id,
            title,
            goal_statement,
            originating_goal: None,
            attention_item_id: None,
            recommendation_id: Some(creation.recommendation_reference.clone()),
            intake_candidate_id: Some(creation.intake_candidate_id.clone()),
            creation_request_id: Some(creation.creation_request_id.clone()),
            package_seal_digest: Some(creation.package_seal_digest.clone()),
            origin: DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE.into(),
            score: DecisionCandidate::unscored(),
            explanation: DecisionExplanation {
                headline: "Created from approved Decision Engine intake".into(),
                reasons: vec![DecisionReason {
                    kind: "intake_creation".into(),
                    summary: format!(
                        "Promoted from intake {} via creation request {}",
                        creation.intake_candidate_id, creation.creation_request_id
                    ),
                    evidence_ref: Some(creation.intake_candidate_id.clone()),
                    attention_reason: None,
                }],
                confidence: "unscored".into(),
            },
            related_goal_ids: Vec::new(),
            pending_approval_ids: Vec::new(),
            outcome: DecisionOutcome::Open,
            created_at,
            handoff_command: DecisionCandidate::HANDOFF_NONE.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        };

        creation.assert_creation_boundary()?;
        Ok((creation, candidate))
    }

    /// Rebuild DecisionCandidate from a persisted created record (still unscored).
    pub fn to_decision_candidate(
        &self,
        outcome: DecisionOutcome,
    ) -> Result<DecisionCandidate, DecisionEngineError> {
        if !self.is_created() {
            return Err(DecisionEngineError::InvalidTransition {
                from: self.creation_state.clone(),
                to: "materialize_decision_candidate".into(),
            });
        }
        let candidate_id = self
            .decision_candidate_id
            .as_ref()
            .ok_or(DecisionEngineError::NotFound)?;
        let title = self.title.clone().ok_or(DecisionEngineError::NotFound)?;
        let goal_statement = self
            .goal_statement
            .clone()
            .ok_or(DecisionEngineError::NotFound)?;
        let created_at = self
            .created_at
            .clone()
            .ok_or(DecisionEngineError::NotFound)?;
        let workspace_id =
            WorkspaceId::new(&self.workspace_id).map_err(DecisionEngineError::Domain)?;
        let id = DecisionCandidateId::new(candidate_id.clone())
            .map_err(DecisionEngineError::Domain)?;

        Ok(DecisionCandidate {
            id,
            workspace_id,
            title,
            goal_statement,
            originating_goal: None,
            attention_item_id: None,
            recommendation_id: Some(self.recommendation_reference.clone()),
            intake_candidate_id: Some(self.intake_candidate_id.clone()),
            creation_request_id: Some(self.creation_request_id.clone()),
            package_seal_digest: Some(self.package_seal_digest.clone()),
            origin: DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE.into(),
            score: DecisionCandidate::unscored(),
            explanation: DecisionExplanation {
                headline: "Created from approved Decision Engine intake".into(),
                reasons: vec![DecisionReason {
                    kind: "intake_creation".into(),
                    summary: format!(
                        "Promoted from intake {} via creation request {}",
                        self.intake_candidate_id, self.creation_request_id
                    ),
                    evidence_ref: Some(self.intake_candidate_id.clone()),
                    attention_reason: None,
                }],
                confidence: "unscored".into(),
            },
            related_goal_ids: Vec::new(),
            pending_approval_ids: Vec::new(),
            outcome,
            created_at,
            handoff_command: DecisionCandidate::HANDOFF_NONE.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn may_create_decision_score(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_score(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_creation_boundary(&self) -> Result<(), DecisionEngineError> {
        if self.creates_decision_score
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.creation_id.starts_with(Self::ID_PREFIX)
            || (self.is_created() && !self.creates_decision_candidate)
            || (self.is_created() && self.decision_candidate_id.is_none())
            || (self.is_created()
                && !self
                    .decision_candidate_id
                    .as_deref()
                    .unwrap_or("")
                    .starts_with("engine_decision:"))
            || (!self.is_created() && self.creates_decision_candidate)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// DE-owned integration of DecisionCandidates into the normal DE lifecycle
/// without merging native and recommendation_intake origins.
///
/// Projected only — outcomes continue to persist via `decision_engine_lifecycle`.
/// Never scores, ranks, plans, grants, or mutates Recommendation Engine records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateLifecycleIntegration {
    pub integration_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    pub outcome: String,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    /// `integrated` | `blocked` | `provenance_invalid`
    pub integration_state: String,
    pub provenance_immutable: bool,
    pub provenance_complete: bool,
    pub scoring_applied: bool,
    pub ranking_applied: bool,
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

impl DecisionCandidateLifecycleIntegration {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_INTEGRATED: &'static str = "integrated";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_PROVENANCE_INVALID: &'static str = "provenance_invalid";
    pub const ID_PREFIX: &'static str = "engine_decision_lifecycle_integration:";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_integrated(&self) -> bool {
        self.integration_state == Self::STATE_INTEGRATED
    }

    pub fn classify_origin(candidate: &DecisionCandidate) -> &'static str {
        if candidate.origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
            || candidate.id.as_str().starts_with("engine_decision:intake:")
            || candidate.intake_candidate_id.is_some()
            || candidate.creation_request_id.is_some()
            || candidate.package_seal_digest.is_some()
        {
            DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
        } else {
            DecisionCandidate::ORIGIN_NATIVE
        }
    }

    pub fn provenance_valid_for_origin(
        origin: &str,
        candidate: &DecisionCandidate,
    ) -> bool {
        if origin == DecisionCandidate::ORIGIN_NATIVE {
            // Native candidates must not carry intake provenance (origin separation).
            candidate.intake_candidate_id.is_none()
                && candidate.creation_request_id.is_none()
                && candidate.package_seal_digest.is_none()
                && !candidate.id.as_str().starts_with("engine_decision:intake:")
        } else if origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE {
            candidate.has_complete_intake_provenance()
                && candidate
                    .intake_candidate_id
                    .as_deref()
                    .is_some_and(|id| id.starts_with("engine_decision_intake:"))
                && candidate
                    .creation_request_id
                    .as_deref()
                    .is_some_and(|id| {
                        id.starts_with(DecisionEngineCandidateCreationRequest::ID_PREFIX)
                    })
                && candidate.id.as_str().starts_with("engine_decision:intake:")
        } else {
            false
        }
    }

    pub fn derive_batch(candidates: &[DecisionCandidate]) -> Vec<Self> {
        let mut out: Vec<Self> = candidates.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(candidate: &DecisionCandidate) -> Self {
        let origin = Self::classify_origin(candidate);
        let provenance_complete = candidate.has_complete_intake_provenance();
        let provenance_ok = Self::provenance_valid_for_origin(origin, candidate);

        let (integration_state, evidence_note) = if !provenance_ok {
            (Self::STATE_PROVENANCE_INVALID, "provenance_invalid")
        } else if !candidate.id.as_str().starts_with("engine_decision:") {
            (Self::STATE_BLOCKED, "non_decision_namespace")
        } else {
            (Self::STATE_INTEGRATED, "lifecycle_integrated")
        };

        Self {
            integration_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            outcome: candidate.outcome.as_str().into(),
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            integration_state: integration_state.into(),
            provenance_immutable: true,
            provenance_complete,
            scoring_applied: false,
            ranking_applied: false,
            creates_decision_score: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            handoff_command: None,
            note: format!(
                "Decision Engine lifecycle integration ({integration_state}; origin={origin}; \
                 {evidence_note}). Origin preserved; provenance immutable; no scoring, ranking, \
                 planner handoff, Gateway, goals, intents, or execution."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Apply a DE lifecycle outcome while preserving origin and provenance.
    /// Never scores, plans, grants, or clears recommendation provenance.
    pub fn try_apply_outcome(
        candidate: &DecisionCandidate,
        to: DecisionOutcome,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let integration = Self::derive(candidate);
        if !integration.is_integrated() {
            return Err(DecisionEngineError::InvalidTransition {
                from: integration.integration_state,
                to: to.as_str().into(),
            });
        }
        if !candidate.outcome.allows_transition(to) {
            return Err(DecisionEngineError::InvalidTransition {
                from: candidate.outcome.as_str().into(),
                to: to.as_str().into(),
            });
        }

        let before = candidate.clone();
        let mut updated = candidate.clone();
        updated.outcome = to;
        // Re-assert origin classification and never strip provenance.
        updated.origin = Self::classify_origin(&before).into();
        updated.intake_candidate_id = before.intake_candidate_id.clone();
        updated.creation_request_id = before.creation_request_id.clone();
        updated.package_seal_digest = before.package_seal_digest.clone();
        updated.recommendation_id = before.recommendation_id.clone();
        // Recommendation-intake candidates remain non-planner-connected.
        if updated.is_recommendation_intake() {
            updated.handoff_command = DecisionCandidate::HANDOFF_NONE.into();
        }

        Self::assert_provenance_retained(&before, &updated)?;
        let mut next = Self::derive(&updated);
        next.note = format!(
            "Decision Engine lifecycle integration applied outcome {} for origin {}. \
             Provenance retained; no scoring, planner handoff, Gateway, or execution.",
            to.as_str(),
            next.origin
        );
        next.assert_integration_only()?;
        Ok((next, updated))
    }

    pub fn assert_provenance_retained(
        before: &DecisionCandidate,
        after: &DecisionCandidate,
    ) -> Result<(), DecisionEngineError> {
        if before.intake_candidate_id != after.intake_candidate_id
            || before.creation_request_id != after.creation_request_id
            || before.package_seal_digest != after.package_seal_digest
            || before.recommendation_id != after.recommendation_id
            || Self::classify_origin(before) != Self::classify_origin(after)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        if after.is_recommendation_intake() && !after.has_complete_intake_provenance() {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }

    /// Reject any attempt to clear intake provenance from a recommendation-derived candidate.
    pub fn attempt_strip_provenance(
        candidate: &DecisionCandidate,
    ) -> Result<DecisionCandidate, DecisionEngineError> {
        let _ = candidate;
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn may_create_decision_score(&self) -> bool {
        false
    }

    pub fn may_rank(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_score(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_integration_only(&self) -> Result<(), DecisionEngineError> {
        if !self.provenance_immutable
            || self.scoring_applied
            || self.ranking_applied
            || self.creates_decision_score
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.integration_id.starts_with(Self::ID_PREFIX)
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or applying origin-aware DecisionCandidate evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateEvaluationOriginInput {
    pub candidate: DecisionCandidate,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    /// Persisted evaluated contract, when present.
    pub existing_evaluated: Option<DecisionCandidateEvaluationOriginContract>,
}

/// DE-owned contract defining how DecisionCandidate evaluation differs by origin.
///
/// This is not scoring, ranking, planner handoff, or execution.
/// `evaluated` acknowledges the origin evaluation contract only — no DecisionScore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateEvaluationOriginContract {
    pub evaluation_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    /// `unevaluated` | `eligible_for_evaluation` | `blocked` | `evaluated`
    pub evaluation_state: String,
    pub lifecycle_valid: bool,
    pub provenance_valid: bool,
    pub origin_supported: bool,
    pub recommendation_visible: bool,
    pub package_identity_traceable: bool,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub evaluated_at: Option<String>,
    pub scoring_applied: bool,
    pub ranking_applied: bool,
    pub creates_decision_score: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateEvaluationOriginContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_UNEVALUATED: &'static str = "unevaluated";
    pub const STATE_ELIGIBLE: &'static str = "eligible_for_evaluation";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const STATE_EVALUATED: &'static str = "evaluated";
    pub const ID_PREFIX: &'static str = "engine_decision_evaluation_origin:";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_eligible_for_evaluation(&self) -> bool {
        self.evaluation_state == Self::STATE_ELIGIBLE
    }

    pub fn is_blocked(&self) -> bool {
        self.evaluation_state == Self::STATE_BLOCKED
    }

    pub fn is_evaluated(&self) -> bool {
        self.evaluation_state == Self::STATE_EVALUATED
    }

    pub fn origin_supported(origin: &str) -> bool {
        origin == DecisionCandidate::ORIGIN_NATIVE
            || origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
    }

    pub fn derive_batch(inputs: &[DecisionCandidateEvaluationOriginInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(input: &DecisionCandidateEvaluationOriginInput) -> Self {
        if let Some(existing) = &input.existing_evaluated {
            if existing.is_evaluated() {
                return existing.clone();
            }
        }

        let candidate = &input.candidate;
        let integration = &input.lifecycle_integration;
        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let origin_field = candidate.origin.trim();
        let origin_supported = if origin_field.is_empty() {
            // Empty field: allow classified origin from id/provenance.
            Self::origin_supported(origin)
        } else {
            Self::origin_supported(origin_field)
        };
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = integration.is_integrated()
            && integration.decision_candidate_id == candidate.id.as_str();
        let recommendation_visible = origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
            && candidate
                .recommendation_id
                .as_ref()
                .is_some_and(|v| !v.is_empty());
        let package_identity_traceable = origin == DecisionCandidate::ORIGIN_NATIVE
            || (candidate
                .package_seal_digest
                .as_ref()
                .is_some_and(|v| !v.is_empty())
                && recommendation_visible);

        let evaluation_state = if !origin_supported {
            Self::STATE_BLOCKED
        } else if !lifecycle_valid {
            Self::STATE_BLOCKED
        } else if origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE && !provenance_valid {
            Self::STATE_BLOCKED
        } else if !provenance_valid {
            // Native with leaked intake provenance, etc.
            Self::STATE_BLOCKED
        } else if lifecycle_valid && provenance_valid && origin_supported {
            Self::STATE_ELIGIBLE
        } else {
            Self::STATE_UNEVALUATED
        };

        Self {
            evaluation_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            evaluation_state: evaluation_state.into(),
            lifecycle_valid,
            provenance_valid,
            origin_supported,
            recommendation_visible,
            package_identity_traceable,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            evaluated_at: None,
            scoring_applied: false,
            ranking_applied: false,
            creates_decision_score: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            handoff_command: None,
            note: format!(
                "Decision Engine evaluation origin contract ({evaluation_state}; origin={origin}). \
                 Origin rules only — not scoring, ranking, planner handoff, Gateway, goals, \
                 intents, execution, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Acknowledge origin evaluation contract without scoring or ranking.
    /// Never mutates Recommendation Engine records or DecisionCandidate scores.
    pub fn try_evaluate(
        input: &DecisionCandidateEvaluationOriginInput,
        evaluated_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if !projected.is_eligible_for_evaluation() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.evaluation_state,
                to: Self::STATE_EVALUATED.into(),
            });
        }
        if input
            .existing_evaluated
            .as_ref()
            .is_some_and(|e| e.is_evaluated())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: Self::STATE_EVALUATED.into(),
                to: Self::STATE_EVALUATED.into(),
            });
        }

        let evaluated_at = evaluated_at.into();
        let before = input.candidate.clone();
        let mut evaluated = projected;
        evaluated.evaluation_state = Self::STATE_EVALUATED.into();
        evaluated.evaluated_at = Some(evaluated_at);
        evaluated.note = format!(
            "Decision Engine evaluation origin contract evaluated for origin {}. \
             Contract acknowledgment only — no DecisionScore, ranking, planner, Gateway, \
             or Recommendation Engine mutation.",
            evaluated.origin
        );

        // Candidate identity/provenance/score unchanged.
        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score {
            return Err(DecisionEngineError::CannotExecute);
        }
        evaluated.assert_evaluation_contract_only()?;
        Ok((evaluated, after))
    }

    pub fn may_create_decision_score(&self) -> bool {
        false
    }

    pub fn may_rank(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_score(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_rank(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_evaluation_contract_only(&self) -> Result<(), DecisionEngineError> {
        if self.scoring_applied
            || self.ranking_applied
            || self.creates_decision_score
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.evaluation_id.starts_with(Self::ID_PREFIX)
            || (self.is_evaluated() && self.evaluated_at.is_none())
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or applying evaluation resolution toward scoring admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateEvaluationResolutionInput {
    pub candidate: DecisionCandidate,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub evaluation_origin: DecisionCandidateEvaluationOriginContract,
    /// When recommendation_intake, optional intake lifecycle for withdrawn/invalidated checks.
    pub intake_withdrawn_or_invalidated: bool,
    /// Persisted accepted/rejected resolution, when present.
    pub existing_resolution: Option<DecisionCandidateEvaluationResolution>,
}

/// DE-owned resolution of evaluation toward scoring-path admission.
///
/// Mirrors intake disposition after evaluation: decides whether a candidate may
/// enter scoring — without creating DecisionScore, ranking, or planner handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateEvaluationResolution {
    pub resolution_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    /// `awaiting_resolution` | `accepted_for_scoring` | `rejected_for_scoring` | `blocked`
    pub resolution_state: String,
    pub evaluation_complete: bool,
    pub lifecycle_valid: bool,
    pub provenance_valid: bool,
    pub candidate_active: bool,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub resolved_at: Option<String>,
    pub resolution_reason: Option<String>,
    pub scoring_applied: bool,
    pub ranking_applied: bool,
    pub creates_decision_score: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateEvaluationResolution {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_AWAITING: &'static str = "awaiting_resolution";
    pub const STATE_ACCEPTED: &'static str = "accepted_for_scoring";
    pub const STATE_REJECTED: &'static str = "rejected_for_scoring";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const ID_PREFIX: &'static str = "engine_decision_evaluation_resolution:";
    pub const RESOLVE_ACCEPT: &'static str = "accept_for_scoring";
    pub const RESOLVE_REJECT: &'static str = "reject_for_scoring";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_accepted_for_scoring(&self) -> bool {
        self.resolution_state == Self::STATE_ACCEPTED
    }

    pub fn is_rejected_for_scoring(&self) -> bool {
        self.resolution_state == Self::STATE_REJECTED
    }

    pub fn is_blocked(&self) -> bool {
        self.resolution_state == Self::STATE_BLOCKED
    }

    pub fn is_awaiting(&self) -> bool {
        self.resolution_state == Self::STATE_AWAITING
    }

    pub fn derive_batch(inputs: &[DecisionCandidateEvaluationResolutionInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(input: &DecisionCandidateEvaluationResolutionInput) -> Self {
        if let Some(existing) = &input.existing_resolution {
            if existing.is_accepted_for_scoring() || existing.is_rejected_for_scoring() {
                return existing.clone();
            }
        }

        let candidate = &input.candidate;
        let integration = &input.lifecycle_integration;
        let evaluation = &input.evaluation_origin;
        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = integration.is_integrated()
            && integration.decision_candidate_id == candidate.id.as_str();
        let evaluation_complete = evaluation.is_evaluated()
            && evaluation.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;

        let resolution_state = if !provenance_valid
            || !lifecycle_valid
            || input.intake_withdrawn_or_invalidated
            || !candidate.id.as_str().starts_with("engine_decision:")
        {
            Self::STATE_BLOCKED
        } else if evaluation_complete && candidate_active && provenance_valid && lifecycle_valid {
            Self::STATE_AWAITING
        } else if !evaluation_complete {
            // Not yet evaluated — not awaiting resolution of evaluation.
            Self::STATE_BLOCKED
        } else {
            // Evaluated but inactive (dismissed/expired/selected) — blocked for scoring path.
            Self::STATE_BLOCKED
        };

        Self {
            resolution_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            resolution_state: resolution_state.into(),
            evaluation_complete,
            lifecycle_valid,
            provenance_valid,
            candidate_active,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            resolved_at: None,
            resolution_reason: None,
            scoring_applied: false,
            ranking_applied: false,
            creates_decision_score: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            handoff_command: None,
            note: format!(
                "Decision Engine evaluation resolution ({resolution_state}; origin={origin}). \
                 Scoring-path admission only — not DecisionScore, ranking, planner, Gateway, \
                 goals, intents, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Resolve an awaiting evaluation toward scoring admission or rejection.
    /// Never creates DecisionScore, ranks, plans, or mutates Recommendation Engine.
    pub fn try_resolve(
        input: &DecisionCandidateEvaluationResolutionInput,
        resolution: impl Into<String>,
        reason: impl Into<String>,
        resolved_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if !projected.is_awaiting() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.resolution_state,
                to: "resolve".into(),
            });
        }
        if input
            .existing_resolution
            .as_ref()
            .is_some_and(|r| r.is_accepted_for_scoring() || r.is_rejected_for_scoring())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.resolution_state,
                to: "already_resolved".into(),
            });
        }

        let resolution = resolution.into();
        let next_state = match resolution.as_str() {
            Self::RESOLVE_ACCEPT | Self::STATE_ACCEPTED => Self::STATE_ACCEPTED,
            Self::RESOLVE_REJECT | Self::STATE_REJECTED => Self::STATE_REJECTED,
            other => {
                return Err(DecisionEngineError::InvalidTransition {
                    from: projected.resolution_state,
                    to: other.into(),
                });
            }
        };

        // Accept requires all gates still hold.
        if next_state == Self::STATE_ACCEPTED
            && (!projected.evaluation_complete
                || !projected.lifecycle_valid
                || !projected.provenance_valid
                || !projected.candidate_active)
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.resolution_state,
                to: Self::STATE_ACCEPTED.into(),
            });
        }

        let resolved_at = resolved_at.into();
        let reason = reason.into();
        let before = input.candidate.clone();
        let mut resolved = projected;
        resolved.resolution_state = next_state.into();
        resolved.resolved_at = Some(resolved_at);
        resolved.resolution_reason = Some(reason);
        resolved.note = format!(
            "Decision Engine evaluation resolution ({}) for origin {}. \
             Scoring-path admission only — no DecisionScore, ranking, planner, Gateway, \
             or Recommendation Engine mutation.",
            resolved.resolution_state, resolved.origin
        );

        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score {
            return Err(DecisionEngineError::CannotExecute);
        }
        resolved.assert_resolution_only()?;
        Ok((resolved, after))
    }

    pub fn may_create_decision_score(&self) -> bool {
        false
    }

    pub fn may_rank(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_create_decision_score(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_rank(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_resolution_only(&self) -> Result<(), DecisionEngineError> {
        if self.scoring_applied
            || self.ranking_applied
            || self.creates_decision_score
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.resolution_id.starts_with(Self::ID_PREFIX)
            || ((self.is_accepted_for_scoring() || self.is_rejected_for_scoring())
                && self.resolved_at.is_none())
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for creating a DE-owned DecisionScore result for an accepted candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateScoreInput {
    pub candidate: DecisionCandidate,
    pub resolution: DecisionCandidateEvaluationResolution,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub intake_withdrawn_or_invalidated: bool,
    pub existing_score: Option<DecisionCandidateScore>,
}

/// DE-owned DecisionScore artifact — scoring result only.
///
/// Created only after `EvaluationResolution.accepted_for_scoring`. Never ranks,
/// selects, plans, executes, or mutates Recommendation Engine.
///
/// Distinct from the embedded [`DecisionScore`] breakdown value type used on
/// candidates for synthesis explainability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateScore {
    pub score_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    pub resolution_id: String,
    /// Score value / breakdown (identity of the scoring result payload).
    pub score: DecisionScore,
    /// Explicit scoring factors/reasons for explainability.
    pub scoring_factors: Vec<String>,
    pub scored_at: String,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub ranking_applied: bool,
    pub selects_candidate: bool,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateScore {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "engine_decision_score:";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    /// Deterministic origin-aware score value — not a peer ranking.
    pub fn compute_score(candidate: &DecisionCandidate, origin: &str) -> DecisionScore {
        if origin == DecisionCandidate::ORIGIN_NATIVE {
            let mut score = candidate.score.clone();
            let mut factors = vec!["decision_score:native".into()];
            factors.extend(score.factors.iter().cloned());
            if factors.len() == 1 {
                factors.push("native_synthesis_score".into());
            }
            score.factors = factors;
            score
        } else if origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE {
            DecisionScore {
                total: 40,
                attention_contribution: 0,
                memory_contribution: 0,
                personalization_contribution: 0,
                goal_contribution: 0,
                factors: vec![
                    "decision_score:recommendation_intake".into(),
                    "intake_baseline".into(),
                    "provenance_aligned".into(),
                ],
            }
        } else {
            DecisionCandidate::unscored()
        }
    }

    /// Create a DecisionScore result when resolution admits scoring.
    /// Never ranks, selects, plans, or mutates Recommendation Engine / candidate outcome.
    pub fn try_create(
        input: &DecisionCandidateScoreInput,
        scored_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        if let Some(existing) = &input.existing_score {
            let after = input.candidate.clone();
            DecisionCandidateLifecycleIntegration::assert_provenance_retained(
                &input.candidate,
                &after,
            )?;
            if after.score != input.candidate.score || after.outcome != input.candidate.outcome {
                return Err(DecisionEngineError::CannotExecute);
            }
            existing.assert_score_only()?;
            return Ok((existing.clone(), after));
        }

        let candidate = &input.candidate;
        let resolution = &input.resolution;
        let integration = &input.lifecycle_integration;

        if resolution.is_rejected_for_scoring() {
            return Err(DecisionEngineError::InvalidTransition {
                from: resolution.resolution_state.clone(),
                to: "score".into(),
            });
        }
        if resolution.is_blocked() || resolution.is_awaiting() {
            return Err(DecisionEngineError::InvalidTransition {
                from: resolution.resolution_state.clone(),
                to: "score".into(),
            });
        }
        if !resolution.is_accepted_for_scoring() {
            return Err(DecisionEngineError::InvalidTransition {
                from: resolution.resolution_state.clone(),
                to: "score".into(),
            });
        }

        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = integration.is_integrated()
            && integration.decision_candidate_id == candidate.id.as_str()
            && resolution.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;

        if !provenance_valid
            || !lifecycle_valid
            || input.intake_withdrawn_or_invalidated
            || !candidate_active
            || !candidate.id.as_str().starts_with("engine_decision:")
            || resolution.package_seal_digest != candidate.package_seal_digest
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: "accepted_for_scoring".into(),
                to: "score_blocked".into(),
            });
        }

        // Invalid source package for intake: empty seal when intake-originated.
        if origin == DecisionCandidate::ORIGIN_RECOMMENDATION_INTAKE
            && candidate
                .package_seal_digest
                .as_deref()
                .is_none_or(|d| d.is_empty())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: "accepted_for_scoring".into(),
                to: "invalid_source_package".into(),
            });
        }

        let score_value = Self::compute_score(candidate, origin);
        let scored_at = scored_at.into();
        let before = candidate.clone();
        let score = Self {
            score_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            resolution_id: resolution.resolution_id.clone(),
            scoring_factors: score_value.factors.clone(),
            score: score_value,
            scored_at,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            ranking_applied: false,
            selects_candidate: false,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            handoff_command: None,
            note: format!(
                "Decision Engine DecisionScore for origin {origin}. \
                 Scoring result only — not ranking, selection, planner, Gateway, \
                 goals, intents, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        };

        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score || after.outcome != before.outcome {
            return Err(DecisionEngineError::CannotExecute);
        }
        score.assert_score_only()?;
        Ok((score, after))
    }

    pub fn may_rank(&self) -> bool {
        false
    }

    pub fn may_select(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_rank(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_select(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_gateway(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_score_only(&self) -> Result<(), DecisionEngineError> {
        if self.ranking_applied
            || self.selects_candidate
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.score_id.starts_with(Self::ID_PREFIX)
            || self.scored_at.is_empty()
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting one candidate into a DE-owned comparative ranking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateRankingMemberInput {
    pub candidate: DecisionCandidate,
    pub score: Option<DecisionCandidateScore>,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub intake_withdrawn_or_invalidated: bool,
}

/// One ordered position in a DecisionCandidateRanking — not a selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateRankingEntry {
    /// 1-based comparative position (not a winner/selection).
    pub rank: u32,
    pub decision_candidate_id: String,
    pub score_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    pub score_total: u32,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
}

/// DE-owned comparative ordering of scored candidates.
///
/// Projected from valid [`DecisionCandidateScore`] artifacts. Never selects,
/// plans, executes, or mutates Recommendation Engine / candidate lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateRanking {
    pub ranking_id: String,
    pub workspace_id: String,
    pub ranked_at: String,
    pub entries: Vec<DecisionCandidateRankingEntry>,
    pub ranking_factors: Vec<String>,
    pub selects_candidate: bool,
    pub selected_candidate_id: Option<String>,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateRanking {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "engine_decision_ranking:";

    pub fn synthetic_id(workspace_id: &str) -> String {
        format!("{}{workspace_id}", Self::ID_PREFIX)
    }

    /// Eligibility for a ranking entry — does not select or mutate.
    pub fn try_rank_member(
        input: &DecisionCandidateRankingMemberInput,
    ) -> Result<DecisionCandidateRankingEntry, DecisionEngineError> {
        let candidate = &input.candidate;
        let Some(score) = &input.score else {
            return Err(DecisionEngineError::InvalidTransition {
                from: "unscored".into(),
                to: "rank".into(),
            });
        };

        if score.decision_candidate_id != candidate.id.as_str() {
            return Err(DecisionEngineError::InvalidTransition {
                from: "score_mismatch".into(),
                to: "rank".into(),
            });
        }

        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = input.lifecycle_integration.is_integrated()
            && input.lifecycle_integration.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;

        if input.intake_withdrawn_or_invalidated {
            return Err(DecisionEngineError::InvalidTransition {
                from: "withdrawn".into(),
                to: "rank".into(),
            });
        }
        if !provenance_valid {
            return Err(DecisionEngineError::InvalidTransition {
                from: "invalid_provenance".into(),
                to: "rank".into(),
            });
        }
        if !lifecycle_valid
            || !candidate_active
            || !candidate.id.as_str().starts_with("engine_decision:")
            || score.assert_score_only().is_err()
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: "ineligible".into(),
                to: "rank".into(),
            });
        }

        Ok(DecisionCandidateRankingEntry {
            // Assigned by derive after sort.
            rank: 0,
            decision_candidate_id: candidate.id.as_str().to_string(),
            score_id: score.score_id.clone(),
            origin: origin.into(),
            score_total: score.score.total,
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
        })
    }

    /// Project comparative ordering from scored active candidates.
    /// Excludes missing scores, withdrawn, and invalid-provenance members.
    /// Never selects a winner or mutates candidate lifecycle.
    pub fn derive(
        workspace_id: impl Into<String>,
        ranked_at: impl Into<String>,
        inputs: &[DecisionCandidateRankingMemberInput],
    ) -> Self {
        let workspace_id = workspace_id.into();
        let ranked_at = ranked_at.into();
        let mut entries: Vec<DecisionCandidateRankingEntry> = inputs
            .iter()
            .filter_map(|input| Self::try_rank_member(input).ok())
            .collect();
        entries.sort_by(|a, b| {
            b.score_total
                .cmp(&a.score_total)
                .then(a.decision_candidate_id.cmp(&b.decision_candidate_id))
        });
        for (idx, entry) in entries.iter_mut().enumerate() {
            entry.rank = (idx as u32).saturating_add(1);
        }

        Self {
            ranking_id: Self::synthetic_id(&workspace_id),
            workspace_id,
            ranked_at,
            entries,
            ranking_factors: vec![
                "decision_ranking:score_total_desc".into(),
                "decision_ranking:candidate_id_asc_tiebreak".into(),
                "decision_ranking:scored_active_only".into(),
            ],
            selects_candidate: false,
            selected_candidate_id: None,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            handoff_command: None,
            note: "Decision Engine candidate ranking — comparative ordering of scored candidates only. \
                 Not selection, planner handoff, Gateway, goals, intents, or Recommendation Engine mutation."
                .into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_select(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_select(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_gateway(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_ranking_only(&self) -> Result<(), DecisionEngineError> {
        if self.selects_candidate
            || self.selected_candidate_id.is_some()
            || self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.ranking_id.starts_with(Self::ID_PREFIX)
            || self.ranked_at.is_empty()
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or applying DE-owned selection after ranking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateSelectionInput {
    pub candidate: DecisionCandidate,
    pub score: Option<DecisionCandidateScore>,
    pub ranking: Option<DecisionCandidateRanking>,
    pub ranking_entry: Option<DecisionCandidateRankingEntry>,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub intake_withdrawn_or_invalidated: bool,
    pub existing_selection: Option<DecisionCandidateSelection>,
}

/// DE-owned selection decision after ranking — progression intent only.
///
/// Distinct from [`DecisionOutcome::Selected`] lifecycle / planner handoff.
/// Never executes, plans, creates goals/intents, or mutates Recommendation Engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateSelection {
    pub selection_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    /// `awaiting_selection` | `selected` | `rejected` | `withdrawn`
    pub selection_state: String,
    pub ranking_id: Option<String>,
    pub ranking_position: Option<u32>,
    pub score_id: Option<String>,
    pub has_ranking_entry: bool,
    pub has_score: bool,
    pub provenance_valid: bool,
    pub lifecycle_valid: bool,
    pub candidate_active: bool,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub selected_at: Option<String>,
    pub selection_reason: Option<String>,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub mutates_candidate_outcome: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateSelection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_AWAITING: &'static str = "awaiting_selection";
    pub const STATE_SELECTED: &'static str = "selected";
    pub const STATE_REJECTED: &'static str = "rejected";
    pub const STATE_WITHDRAWN: &'static str = "withdrawn";
    pub const ID_PREFIX: &'static str = "engine_decision_selection:";
    pub const ACTION_SELECT: &'static str = "select";
    pub const ACTION_REJECT: &'static str = "reject";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_awaiting(&self) -> bool {
        self.selection_state == Self::STATE_AWAITING
    }

    pub fn is_selected(&self) -> bool {
        self.selection_state == Self::STATE_SELECTED
    }

    pub fn is_rejected(&self) -> bool {
        self.selection_state == Self::STATE_REJECTED
    }

    pub fn is_withdrawn(&self) -> bool {
        self.selection_state == Self::STATE_WITHDRAWN
    }

    pub fn derive_batch(inputs: &[DecisionCandidateSelectionInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(input: &DecisionCandidateSelectionInput) -> Self {
        if let Some(existing) = &input.existing_selection {
            if existing.is_selected() || existing.is_rejected() {
                return existing.clone();
            }
        }

        let candidate = &input.candidate;
        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = input.lifecycle_integration.is_integrated()
            && input.lifecycle_integration.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;
        let has_score = input
            .score
            .as_ref()
            .is_some_and(|s| s.decision_candidate_id == candidate.id.as_str());
        let has_ranking_entry = input
            .ranking_entry
            .as_ref()
            .is_some_and(|e| e.decision_candidate_id == candidate.id.as_str());

        let selection_state = if input.intake_withdrawn_or_invalidated
            || !candidate_active
            || !provenance_valid
            || !lifecycle_valid
            || !candidate.id.as_str().starts_with("engine_decision:")
        {
            Self::STATE_WITHDRAWN
        } else if has_ranking_entry && has_score && provenance_valid && lifecycle_valid {
            Self::STATE_AWAITING
        } else {
            // Missing ranking/score — not selectable; projected withdrawn from selection path.
            Self::STATE_WITHDRAWN
        };

        Self {
            selection_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            selection_state: selection_state.into(),
            ranking_id: input.ranking.as_ref().map(|r| r.ranking_id.clone()),
            ranking_position: input.ranking_entry.as_ref().map(|e| e.rank),
            score_id: input.score.as_ref().map(|s| s.score_id.clone()),
            has_ranking_entry,
            has_score,
            provenance_valid,
            lifecycle_valid,
            candidate_active,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            selected_at: None,
            selection_reason: None,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            mutates_candidate_outcome: false,
            handoff_command: None,
            note: format!(
                "Decision Engine candidate selection ({selection_state}; origin={origin}). \
                 Progression decision only — not execution, planner, Gateway, goals, intents, \
                 candidate outcome mutation, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Apply select/reject on an awaiting selection. Never executes or mutates outcome/RE.
    pub fn try_select(
        input: &DecisionCandidateSelectionInput,
        action: impl Into<String>,
        reason: impl Into<String>,
        selected_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if projected.is_withdrawn() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.selection_state,
                to: "select".into(),
            });
        }
        if !projected.is_awaiting() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.selection_state,
                to: "select".into(),
            });
        }
        if input
            .existing_selection
            .as_ref()
            .is_some_and(|s| s.is_selected() || s.is_rejected())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.selection_state,
                to: "already_decided".into(),
            });
        }

        // Explicit gate failures for contract clarity.
        if !projected.has_ranking_entry {
            return Err(DecisionEngineError::InvalidTransition {
                from: "missing_ranking".into(),
                to: "select".into(),
            });
        }
        if !projected.has_score {
            return Err(DecisionEngineError::InvalidTransition {
                from: "missing_score".into(),
                to: "select".into(),
            });
        }
        if !projected.provenance_valid {
            return Err(DecisionEngineError::InvalidTransition {
                from: "invalid_provenance".into(),
                to: "select".into(),
            });
        }
        if !projected.candidate_active || input.intake_withdrawn_or_invalidated {
            return Err(DecisionEngineError::InvalidTransition {
                from: "withdrawn".into(),
                to: "select".into(),
            });
        }

        let action = action.into();
        let next_state = match action.as_str() {
            Self::ACTION_SELECT | Self::STATE_SELECTED => Self::STATE_SELECTED,
            Self::ACTION_REJECT | Self::STATE_REJECTED => Self::STATE_REJECTED,
            other => {
                return Err(DecisionEngineError::InvalidTransition {
                    from: projected.selection_state,
                    to: other.into(),
                });
            }
        };

        let selected_at = selected_at.into();
        let reason = reason.into();
        let before = input.candidate.clone();
        let mut decided = projected;
        decided.selection_state = next_state.into();
        decided.selected_at = Some(selected_at);
        decided.selection_reason = Some(reason);
        decided.note = format!(
            "Decision Engine candidate selection ({}) for origin {}. \
             Progression decision only — no execution, planner, Gateway, goals, intents, \
             candidate outcome mutation, or Recommendation Engine mutation.",
            decided.selection_state, decided.origin
        );

        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score || after.outcome != before.outcome {
            return Err(DecisionEngineError::CannotExecute);
        }
        decided.assert_selection_only()?;
        Ok((decided, after))
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_gateway(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_selection_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.mutates_candidate_outcome
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.selection_id.starts_with(Self::ID_PREFIX)
            || ((self.is_selected() || self.is_rejected()) && self.selected_at.is_none())
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or applying a DE-owned progression request after selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateProgressionRequestInput {
    pub candidate: DecisionCandidate,
    pub selection: DecisionCandidateSelection,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub intake_withdrawn_or_invalidated: bool,
    pub existing_request: Option<DecisionCandidateProgressionRequest>,
}

/// DE-owned non-executing request that a selected candidate be considered for
/// downstream progression.
///
/// Distinct from planner handoff / Command Pipeline / Gateway. Never creates
/// goals, intents, or mutates Recommendation Engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateProgressionRequest {
    pub request_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    /// `pending` | `requested` | `cancelled` | `blocked`
    pub request_state: String,
    pub selection_id: String,
    pub selection_state: String,
    pub ranking_id: Option<String>,
    pub ranking_position: Option<u32>,
    pub score_id: Option<String>,
    pub selection_valid: bool,
    pub provenance_valid: bool,
    pub lifecycle_valid: bool,
    pub candidate_active: bool,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub requested_at: Option<String>,
    pub request_reason: Option<String>,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub mutates_candidate_outcome: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateProgressionRequest {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_PENDING: &'static str = "pending";
    pub const STATE_REQUESTED: &'static str = "requested";
    pub const STATE_CANCELLED: &'static str = "cancelled";
    pub const STATE_BLOCKED: &'static str = "blocked";
    pub const ID_PREFIX: &'static str = "engine_decision_progression_request:";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_pending(&self) -> bool {
        self.request_state == Self::STATE_PENDING
    }

    pub fn is_requested(&self) -> bool {
        self.request_state == Self::STATE_REQUESTED
    }

    pub fn is_cancelled(&self) -> bool {
        self.request_state == Self::STATE_CANCELLED
    }

    pub fn is_blocked(&self) -> bool {
        self.request_state == Self::STATE_BLOCKED
    }

    pub fn derive_batch(inputs: &[DecisionCandidateProgressionRequestInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(input: &DecisionCandidateProgressionRequestInput) -> Self {
        if let Some(existing) = &input.existing_request {
            if existing.is_requested() || existing.is_cancelled() {
                return existing.clone();
            }
        }

        let candidate = &input.candidate;
        let selection = &input.selection;
        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = input.lifecycle_integration.is_integrated()
            && input.lifecycle_integration.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;
        let selection_valid = selection.is_selected()
            && selection.decision_candidate_id == candidate.id.as_str()
            && !selection.is_withdrawn();

        let request_state = if !selection_valid
            || selection.is_withdrawn()
            || input.intake_withdrawn_or_invalidated
            || !provenance_valid
            || !lifecycle_valid
            || !candidate_active
            || !candidate.id.as_str().starts_with("engine_decision:")
        {
            Self::STATE_BLOCKED
        } else {
            Self::STATE_PENDING
        };

        Self {
            request_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            request_state: request_state.into(),
            selection_id: selection.selection_id.clone(),
            selection_state: selection.selection_state.clone(),
            ranking_id: selection.ranking_id.clone(),
            ranking_position: selection.ranking_position,
            score_id: selection.score_id.clone(),
            selection_valid,
            provenance_valid,
            lifecycle_valid,
            candidate_active,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            requested_at: None,
            request_reason: None,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            mutates_candidate_outcome: false,
            handoff_command: None,
            note: format!(
                "Decision Engine progression request ({request_state}; origin={origin}). \
                 Downstream consideration request only — not planner, Gateway, goals, intents, \
                 execution, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Issue a progression request for a pending selected candidate.
    /// Never invokes planner, Gateway, or mutates candidate/RE.
    pub fn try_request(
        input: &DecisionCandidateProgressionRequestInput,
        reason: impl Into<String>,
        requested_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if projected.is_blocked() {
            let from = if !projected.selection_valid {
                "not_selected"
            } else if !projected.provenance_valid {
                "invalid_provenance"
            } else if input.intake_withdrawn_or_invalidated || input.selection.is_withdrawn() {
                "withdrawn"
            } else {
                "blocked"
            };
            return Err(DecisionEngineError::InvalidTransition {
                from: from.into(),
                to: "request".into(),
            });
        }
        if !projected.is_pending() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.request_state,
                to: "request".into(),
            });
        }
        if input
            .existing_request
            .as_ref()
            .is_some_and(|r| r.is_requested() || r.is_cancelled())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.request_state,
                to: "already_decided".into(),
            });
        }
        if !input.selection.is_selected() {
            return Err(DecisionEngineError::InvalidTransition {
                from: "not_selected".into(),
                to: "request".into(),
            });
        }
        if !projected.provenance_valid {
            return Err(DecisionEngineError::InvalidTransition {
                from: "invalid_provenance".into(),
                to: "request".into(),
            });
        }
        if input.intake_withdrawn_or_invalidated || input.selection.is_withdrawn() {
            return Err(DecisionEngineError::InvalidTransition {
                from: "withdrawn".into(),
                to: "request".into(),
            });
        }

        let requested_at = requested_at.into();
        let reason = reason.into();
        let before = input.candidate.clone();
        let mut requested = projected;
        requested.request_state = Self::STATE_REQUESTED.into();
        requested.requested_at = Some(requested_at);
        requested.request_reason = Some(reason);
        requested.note = format!(
            "Decision Engine progression request (requested; origin={}). \
             Downstream consideration request only — not planner, Gateway, goals, intents, \
             execution, or Recommendation Engine mutation.",
            requested.origin
        );

        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score || after.outcome != before.outcome {
            return Err(DecisionEngineError::CannotExecute);
        }
        requested.assert_request_only()?;
        Ok((requested, after))
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_gateway(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_request_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.mutates_candidate_outcome
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.request_id.starts_with(Self::ID_PREFIX)
            || (self.is_requested() && self.requested_at.is_none())
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

/// Input for projecting or applying DE-owned progression acknowledgement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionCandidateProgressionAcknowledgementInput {
    pub candidate: DecisionCandidate,
    pub progression_request: DecisionCandidateProgressionRequest,
    pub lifecycle_integration: DecisionCandidateLifecycleIntegration,
    pub intake_withdrawn_or_invalidated: bool,
    pub existing_acknowledgement: Option<DecisionCandidateProgressionAcknowledgement>,
}

/// DE-owned acknowledgement that a progression request was received and is
/// eligible for a future downstream workflow — without executing anything.
///
/// Distinct from planner handoff, goals, intents, Gateway, and Command Pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionCandidateProgressionAcknowledgement {
    pub acknowledgement_id: String,
    pub workspace_id: String,
    pub decision_candidate_id: String,
    /// `native` | `recommendation_intake`
    pub origin: String,
    /// `awaiting_acknowledgement` | `acknowledged` | `rejected` | `expired`
    pub acknowledgement_state: String,
    pub request_id: String,
    pub request_state: String,
    pub selection_id: String,
    pub ranking_id: Option<String>,
    pub score_id: Option<String>,
    pub request_valid: bool,
    pub provenance_valid: bool,
    pub lifecycle_valid: bool,
    pub candidate_active: bool,
    pub intake_candidate_id: Option<String>,
    pub creation_request_id: Option<String>,
    pub package_seal_digest: Option<String>,
    pub recommendation_reference: Option<String>,
    pub acknowledged_at: Option<String>,
    pub acknowledgement_reason: Option<String>,
    pub creates_goal: bool,
    pub creates_intent: bool,
    pub adapter_invoked: bool,
    pub planner_invoked: bool,
    pub ownership_transferred: bool,
    pub mutates_recommendation_engine: bool,
    pub mutates_candidate_outcome: bool,
    pub handoff_command: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionCandidateProgressionAcknowledgement {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const STATE_AWAITING: &'static str = "awaiting_acknowledgement";
    pub const STATE_ACKNOWLEDGED: &'static str = "acknowledged";
    pub const STATE_REJECTED: &'static str = "rejected";
    pub const STATE_EXPIRED: &'static str = "expired";
    pub const ID_PREFIX: &'static str = "engine_decision_progression_ack:";
    pub const ACTION_ACKNOWLEDGE: &'static str = "acknowledge";
    pub const ACTION_REJECT: &'static str = "reject";

    pub fn synthetic_id(decision_candidate_id: &str) -> String {
        format!("{}{decision_candidate_id}", Self::ID_PREFIX)
    }

    pub fn is_awaiting(&self) -> bool {
        self.acknowledgement_state == Self::STATE_AWAITING
    }

    pub fn is_acknowledged(&self) -> bool {
        self.acknowledgement_state == Self::STATE_ACKNOWLEDGED
    }

    pub fn is_rejected(&self) -> bool {
        self.acknowledgement_state == Self::STATE_REJECTED
    }

    pub fn is_expired(&self) -> bool {
        self.acknowledgement_state == Self::STATE_EXPIRED
    }

    pub fn derive_batch(inputs: &[DecisionCandidateProgressionAcknowledgementInput]) -> Vec<Self> {
        let mut out: Vec<Self> = inputs.iter().map(Self::derive).collect();
        out.sort_by(|a, b| a.decision_candidate_id.cmp(&b.decision_candidate_id));
        out
    }

    pub fn derive(input: &DecisionCandidateProgressionAcknowledgementInput) -> Self {
        if let Some(existing) = &input.existing_acknowledgement {
            if existing.is_acknowledged() || existing.is_rejected() {
                return existing.clone();
            }
        }

        let candidate = &input.candidate;
        let request = &input.progression_request;
        let origin = DecisionCandidateLifecycleIntegration::classify_origin(candidate);
        let provenance_valid =
            DecisionCandidateLifecycleIntegration::provenance_valid_for_origin(origin, candidate);
        let lifecycle_valid = input.lifecycle_integration.is_integrated()
            && input.lifecycle_integration.decision_candidate_id == candidate.id.as_str();
        let candidate_active = matches!(
            candidate.outcome,
            DecisionOutcome::Open | DecisionOutcome::Postponed
        ) && !input.intake_withdrawn_or_invalidated;
        let request_valid = request.is_requested()
            && request.decision_candidate_id == candidate.id.as_str()
            && !request.is_cancelled()
            && !request.is_blocked();

        let acknowledgement_state = if input.intake_withdrawn_or_invalidated
            || !candidate_active
            || !provenance_valid
            || !lifecycle_valid
            || !candidate.id.as_str().starts_with("engine_decision:")
        {
            // Request path no longer valid for acknowledgement.
            if request.is_requested() {
                Self::STATE_EXPIRED
            } else {
                Self::STATE_REJECTED
            }
        } else if request_valid && provenance_valid && lifecycle_valid && candidate_active {
            Self::STATE_AWAITING
        } else {
            Self::STATE_REJECTED
        };

        Self {
            acknowledgement_id: Self::synthetic_id(candidate.id.as_str()),
            workspace_id: candidate.workspace_id.as_str().to_string(),
            decision_candidate_id: candidate.id.as_str().to_string(),
            origin: origin.into(),
            acknowledgement_state: acknowledgement_state.into(),
            request_id: request.request_id.clone(),
            request_state: request.request_state.clone(),
            selection_id: request.selection_id.clone(),
            ranking_id: request.ranking_id.clone(),
            score_id: request.score_id.clone(),
            request_valid,
            provenance_valid,
            lifecycle_valid,
            candidate_active,
            intake_candidate_id: candidate.intake_candidate_id.clone(),
            creation_request_id: candidate.creation_request_id.clone(),
            package_seal_digest: candidate.package_seal_digest.clone(),
            recommendation_reference: candidate.recommendation_id.clone(),
            acknowledged_at: None,
            acknowledgement_reason: None,
            creates_goal: false,
            creates_intent: false,
            adapter_invoked: false,
            planner_invoked: false,
            ownership_transferred: false,
            mutates_recommendation_engine: false,
            mutates_candidate_outcome: false,
            handoff_command: None,
            note: format!(
                "Decision Engine progression acknowledgement ({acknowledgement_state}; origin={origin}). \
                 Receipt only — not planner, Gateway, goals, intents, execution, or Recommendation Engine mutation."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Acknowledge or reject a pending progression request receipt.
    /// Never invokes planner, Gateway, or mutates candidate/RE.
    pub fn try_acknowledge(
        input: &DecisionCandidateProgressionAcknowledgementInput,
        action: impl Into<String>,
        reason: impl Into<String>,
        acknowledged_at: impl Into<String>,
    ) -> Result<(Self, DecisionCandidate), DecisionEngineError> {
        let projected = Self::derive(input);
        if projected.is_expired() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.acknowledgement_state,
                to: "acknowledge".into(),
            });
        }
        if projected.is_rejected() && !projected.is_awaiting() {
            let from = if !projected.request_valid {
                "invalid_request"
            } else if !projected.provenance_valid {
                "invalid_provenance"
            } else if input.intake_withdrawn_or_invalidated {
                "withdrawn"
            } else {
                "rejected"
            };
            return Err(DecisionEngineError::InvalidTransition {
                from: from.into(),
                to: "acknowledge".into(),
            });
        }
        if !projected.is_awaiting() {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.acknowledgement_state,
                to: "acknowledge".into(),
            });
        }
        if input
            .existing_acknowledgement
            .as_ref()
            .is_some_and(|a| a.is_acknowledged() || a.is_rejected())
        {
            return Err(DecisionEngineError::InvalidTransition {
                from: projected.acknowledgement_state,
                to: "already_decided".into(),
            });
        }
        if !input.progression_request.is_requested() || !projected.request_valid {
            return Err(DecisionEngineError::InvalidTransition {
                from: "invalid_request".into(),
                to: "acknowledge".into(),
            });
        }
        if !projected.provenance_valid {
            return Err(DecisionEngineError::InvalidTransition {
                from: "invalid_provenance".into(),
                to: "acknowledge".into(),
            });
        }
        if input.intake_withdrawn_or_invalidated || !projected.candidate_active {
            return Err(DecisionEngineError::InvalidTransition {
                from: "withdrawn".into(),
                to: "acknowledge".into(),
            });
        }

        let action = action.into();
        let next_state = match action.as_str() {
            Self::ACTION_ACKNOWLEDGE | Self::STATE_ACKNOWLEDGED => Self::STATE_ACKNOWLEDGED,
            Self::ACTION_REJECT | Self::STATE_REJECTED => Self::STATE_REJECTED,
            other => {
                return Err(DecisionEngineError::InvalidTransition {
                    from: projected.acknowledgement_state,
                    to: other.into(),
                });
            }
        };

        let acknowledged_at = acknowledged_at.into();
        let reason = reason.into();
        let before = input.candidate.clone();
        let mut decided = projected;
        decided.acknowledgement_state = next_state.into();
        decided.acknowledged_at = Some(acknowledged_at);
        decided.acknowledgement_reason = Some(reason);
        decided.note = format!(
            "Decision Engine progression acknowledgement ({}; origin={}). \
             Receipt only — not planner, Gateway, goals, intents, execution, or Recommendation Engine mutation.",
            decided.acknowledgement_state, decided.origin
        );

        let after = before.clone();
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(&before, &after)?;
        if after.score != before.score || after.outcome != before.outcome {
            return Err(DecisionEngineError::CannotExecute);
        }
        decided.assert_acknowledgement_only()?;
        Ok((decided, after))
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_invoke_planner(&self) -> bool {
        false
    }

    pub fn may_invoke_gateway(&self) -> bool {
        false
    }

    pub fn may_create_goal(&self) -> bool {
        false
    }

    pub fn may_create_intent(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_planner(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_invoke_gateway(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_goal(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn attempt_create_intent(&self) -> Result<(), DecisionEngineError> {
        Err(DecisionEngineError::CannotExecute)
    }

    pub fn assert_acknowledgement_only(&self) -> Result<(), DecisionEngineError> {
        if self.creates_goal
            || self.creates_intent
            || self.adapter_invoked
            || self.planner_invoked
            || self.ownership_transferred
            || self.mutates_recommendation_engine
            || self.mutates_candidate_outcome
            || self.handoff_command.is_some()
            || self.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || !self.acknowledgement_id.starts_with(Self::ID_PREFIX)
            || (self.is_acknowledged() && self.acknowledged_at.is_none())
            || (self.is_rejected()
                && self.acknowledgement_reason.is_some()
                && self.acknowledged_at.is_none())
        {
            return Err(DecisionEngineError::CannotExecute);
        }
        Ok(())
    }
}

#[cfg(test)]
mod terminal_evidence_projection_tests {
    use super::*;

    fn context() -> DecisionContext {
        DecisionContext {
            workspace_id: "ws-1".into(),
            active_project_id: None,
            active_task_id: None,
            attention_item_count: 0,
            memory_highlight_count: 0,
            preference_highlight_count: 0,
            pending_approval_count: 0,
            pending_plan_count: 0,
            task_graph_open_count: 0,
            task_graph_blocked_count: 0,
        }
    }

    fn candidate(key: &str, outcome: DecisionOutcome) -> DecisionCandidate {
        DecisionCandidate {
            id: DecisionCandidate::synthetic_id(key),
            workspace_id: WorkspaceId::new("ws-1").unwrap(),
            title: key.into(),
            goal_statement: "goal".into(),
            originating_goal: None,
            attention_item_id: None,
            recommendation_id: Some("rec-1".into()),
            intake_candidate_id: None,
            creation_request_id: None,
            package_seal_digest: None,
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            score: DecisionCandidate::unscored(),
            explanation: DecisionExplanation {
                headline: "h".into(),
                reasons: vec![],
                confidence: "low".into(),
            },
            related_goal_ids: vec![],
            pending_approval_ids: vec![],
            outcome,
            created_at: "t0".into(),
            handoff_command: DecisionCandidate::HANDOFF_NONE.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn terminal_artifacts_excluded_from_actionable_and_appear_in_history() {
        let state = DecisionEngineState::from_candidates(
            "ws-1",
            context(),
            vec![
                candidate("open-1", DecisionOutcome::Open),
                candidate("dismissed-1", DecisionOutcome::Dismissed),
                candidate("selected-1", DecisionOutcome::Selected),
                candidate("expired-1", DecisionOutcome::Expired),
                candidate("postponed-1", DecisionOutcome::Postponed),
            ],
        );
        assert!(state
            .top_candidates
            .iter()
            .all(|c| c.outcome.is_actionable()));
        assert!(!state
            .top_candidates
            .iter()
            .any(|c| c.outcome.is_terminal()));
        assert_eq!(state.history_count, 3);
        assert!(state.history.iter().all(|h| h.is_non_actionable()));
        assert!(state
            .history
            .iter()
            .any(|h| h.candidate_key == "dismissed-1" && h.decision_state == "dismissed"));
        assert!(state
            .history
            .iter()
            .any(|h| h.candidate_key == "selected-1" && h.resolution_type == "selected"));
        assert!(state
            .history
            .iter()
            .any(|h| h.candidate_key == "expired-1" && h.terminal));
    }

    #[test]
    fn empty_actionable_summary_still_carries_history_count() {
        let state = DecisionEngineState::from_candidates(
            "ws-1",
            context(),
            vec![
                candidate("d1", DecisionOutcome::Dismissed),
                candidate("e1", DecisionOutcome::Expired),
                candidate("s1", DecisionOutcome::Selected),
            ],
        );
        let summary = state.summary_projection(2);
        assert!(summary.top_candidates.is_empty());
        assert_eq!(summary.candidate_count, 0);
        assert_eq!(summary.open_count, 0);
        assert_eq!(summary.history.len(), 2, "window truncated");
        assert_eq!(summary.history_count, 3, "full evidence count retained");
        assert!(summary.history.iter().all(|h| !h.actionable && h.terminal));
    }

    #[test]
    fn orphan_overlay_projects_into_history_without_live_candidate() {
        let overlay = DecisionEngineOverlay {
            workspace_id: "ws-1".into(),
            candidate_key: "attention:gone".into(),
            outcome: DecisionOutcome::Dismissed,
            updated_at: "t9".into(),
            actor_id: "local-user".into(),
        };
        let history = DecisionEngineState::project_history_from_candidates(&[], &[overlay]);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].candidate_key, "attention:gone");
        assert!(history[0].is_non_actionable());
        assert!(DecisionArtifactHistoryEntry::from_overlay(
            &DecisionEngineOverlay {
                workspace_id: "ws-1".into(),
                candidate_key: "open-key".into(),
                outcome: DecisionOutcome::Open,
                updated_at: "t0".into(),
                actor_id: "local-user".into(),
            }
        )
        .is_none());
    }

    #[test]
    fn history_entry_cannot_become_executable_candidate() {
        let entry = DecisionArtifactHistoryEntry::from_candidate(
            &candidate("dismissed-1", DecisionOutcome::Dismissed),
            Some("t1"),
        )
        .unwrap();
        assert!(!entry.actionable);
        assert_eq!(entry.authority_effect, "none");
        assert!(entry.terminal);
        // History DTO is distinct from DecisionCandidate — no handoff/execution fields.
        let encoded = serde_json::to_value(&entry).unwrap();
        assert!(encoded.get("handoff_command").is_none());
        assert!(encoded.get("goal_statement").is_none());
    }
}
