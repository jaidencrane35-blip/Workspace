//! Action Proposal + Recommendation identity/lifecycle (Sprint 137–138).
//!
//! Governance shapes between recommendations and future Permission Gateway work.
//! **Never executes.** Native family IDs are preserved — never collapsed into one namespace.
//! Reasoning provenance is immutable; lifecycle metadata mutates separately.
//!
//! Consolidated governance aggregates live in [`governance`] (Sprints 165–169):
//! preconditions (`contracts`), review-ops (`workflow`), observability (`ops`).
//! Ledger / policy / publication-prep types in this module remain the inward
//! dependency targets. See `docs/05-AI/WORKSPACE-GOVERNANCE.md`.

mod governance;
pub use governance::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_attention::AttentionReason;
use crate::workspace_recommendation::{RecommendationEvidence, RecommendationItem};

/// Action-proposal / recommendation governance errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ActionProposalError {
    #[error("action proposal cannot execute or authorize")]
    CannotExecute,

    #[error("invalid recommendation lifecycle transition from {from} to {to}")]
    InvalidLifecycleTransition { from: String, to: String },

    #[error("recommendation outcome requires a resolved lifecycle state")]
    OutcomeRequiresResolution,

    #[error("recommendation outcome cannot execute or authorize")]
    OutcomeCannotExecute,

    #[error("adaptation proposal cannot self-approve")]
    AdaptationSelfApprovalForbidden,

    #[error("adaptation approval requires a local_user reviewer")]
    AdaptationReviewerRequired,

    #[error("rejected adaptation proposal cannot apply")]
    RejectedAdaptationCannotApply,

    #[error("expired adaptation proposal cannot be approved or applied")]
    ExpiredAdaptationCannotProceed,

    #[error("adaptation apply is future-only; controlled change surface not active")]
    AdaptationApplyNotImplemented,

    #[error("unapproved adaptation cannot enter controlled change surface")]
    UnapprovedControlledChangeForbidden,

    #[error("change evaluation cannot rewrite history or provenance")]
    ChangeEvaluationCannotRewriteHistory,

    #[error("behaviour version publish / runtime apply is future-only")]
    BehaviourVersionPublishNotImplemented,

    #[error("publication requires prior approval and evaluation")]
    PublicationRequiresApproval,

    #[error("governance ledger cannot grant execution authority")]
    GovernanceCannotGrantAuthority,

    #[error("publication activation is future-only; runtime unchanged")]
    PublicationActivationNotImplemented,

    #[error("governance policy approval requirements not met")]
    GovernancePolicyRequirementsNotMet,

    #[error("governance review rationale is required")]
    GovernanceReviewRationaleRequired,

    #[error("governance policy expiry blocks further approval")]
    GovernancePolicyExpired,

    #[error("reviewer cannot bypass or rewrite provenance")]
    ReviewerCannotBypassProvenance,

    #[error("governance risk requirements not met for review routing")]
    GovernanceRiskRequirementsNotMet,

    #[error("governance risk metadata cannot mutate cognition or scoring")]
    GovernanceRiskCannotMutateCognition,

    #[error("governance decision evidence is required for approval")]
    GovernanceDecisionEvidenceRequired,

    #[error("publication readiness cannot activate runtime changes")]
    PublicationReadinessCannotActivate,

    #[error("governance workspace UI cannot approve execution")]
    GovernanceWorkspaceCannotApproveExecution,

    #[error("publication environment cannot activate runtime changes")]
    PublicationEnvironmentCannotActivate,

    #[error("publication safety validation failed: {0}")]
    PublicationSafetyValidationFailed(String),

    #[error("publication safety contract blocks progression: {0}")]
    PublicationSafetyBlocked(String),

    #[error("governance failure recovery cannot execute or activate: {0}")]
    GovernanceFailureRecoveryBlocked(String),

    #[error("governance failure handling blocks provenance deletion")]
    GovernanceFailureCannotDeleteProvenance,

    #[error("governance lifecycle integrity validation failed: {0}")]
    GovernanceLifecycleIntegrityFailed(String),

    #[error("governance timeline events are immutable")]
    GovernanceTimelineImmutable,

    #[error("governance obligation/condition cannot execute or grant authority")]
    GovernanceConditionCannotExecute,

    #[error("governance condition contract blocked: {0}")]
    GovernanceConditionBlocked(String),

    #[error("governance compatibility contract blocked: {0}")]
    GovernanceCompatibilityBlocked(String),

    #[error("governance integrity verification cannot repair or mutate")]
    GovernanceIntegrityCannotMutate,

    #[error("governance archive is append-only; deletion forbidden")]
    GovernanceArchiveImmutable,

    #[error("governance review workflow cannot execute or publish")]
    GovernanceReviewWorkflowBlocked(String),

    #[error("governance conflict resolution cannot publish, execute, or rewrite history")]
    GovernanceConflictResolutionBlocked(String),

    #[error("governance decision package is immutable after seal")]
    GovernanceDecisionPackageImmutable,

    #[error("governance compliance verification cannot repair or grant authority")]
    GovernanceComplianceCannotMutate,

    #[error("governance readiness dashboard is a projection only; cannot execute")]
    GovernanceDashboardCannotExecute,

    #[error("governance notification is informational only; cannot execute or grant authority")]
    GovernanceNotificationCannotExecute,

    #[error("governance delegation blocked: {0}")]
    GovernanceDelegationBlocked(String),

    #[error("governance metrics are observational only; cannot optimise or adapt")]
    GovernanceMetricsCannotMutate,

    #[error("governance reporting is a projection only; cannot execute")]
    GovernanceReportCannotExecute,

    #[error("governance export is read-only; import/sync/publish forbidden")]
    GovernanceExportForbidden(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Which recommendation family produced a proposal reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationFamily {
    RecommendationEngine,
    DecisionEngine,
    Intelligence,
    Adaptation,
    DecisionQueue,
}

impl RecommendationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RecommendationEngine => "recommendation_engine",
            Self::DecisionEngine => "decision_engine",
            Self::Intelligence => "intelligence",
            Self::Adaptation => "adaptation",
            Self::DecisionQueue => "decision_queue",
        }
    }

    /// Native ID prefix convention for this family (documentation + validation aid).
    pub fn native_id_prefix(self) -> &'static str {
        match self {
            Self::RecommendationEngine => "recommendation:",
            Self::DecisionEngine => "engine_decision:",
            Self::Intelligence => "rec-attention-",
            Self::Adaptation => "adaptation:",
            Self::DecisionQueue => "decision:",
        }
    }
}

/// Unified Recommendation Identity contract (Sprint 138).
///
/// Does **not** replace native family IDs. It records cross-references so governance
/// can follow Evidence → Recommendation → Decision → ActionProposal without merging
/// namespaces (`recommendation:*`, `rec-attention-*`, `engine_decision:*`, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationIdentity {
    /// Native id in the source family (unchanged).
    pub native_id: String,
    pub family: RecommendationFamily,
    /// Source domain label (e.g. `attention`, `task_graph`, `continuity`).
    pub source_domain: String,
    /// Originating reasoning reference (Attention item id, explanation_key, …).
    pub originating_reasoning_ref: Option<String>,
    /// Decision Engine / Queue reference when one exists.
    pub decision_ref: Option<String>,
    /// Optional ActionProposal id once prepared (architecture only).
    pub action_proposal_ref: Option<String>,
}

impl RecommendationIdentity {
    pub fn from_recommendation_item(item: &RecommendationItem) -> Self {
        Self {
            native_id: item.id.clone(),
            family: RecommendationFamily::RecommendationEngine,
            source_domain: item
                .evidence
                .first()
                .map(|e| e.source_model.clone())
                .unwrap_or_else(|| "recommendation_engine".into()),
            originating_reasoning_ref: item
                .related_attention_id
                .clone()
                .or_else(|| {
                    item.attention_reasons
                        .first()
                        .map(|r| r.explanation_key.clone())
                }),
            decision_ref: item.related_decision_id.clone(),
            action_proposal_ref: None,
        }
    }

    pub fn with_action_proposal_ref(mut self, proposal_id: impl Into<String>) -> Self {
        self.action_proposal_ref = Some(proposal_id.into());
        self
    }

    pub fn with_decision_ref(mut self, decision_id: impl Into<String>) -> Self {
        self.decision_ref = Some(decision_id.into());
        self
    }
}

/// Recommendation lifecycle states (Sprint 138).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationLifecycleState {
    Created,
    Available,
    Presented,
    Accepted,
    Rejected,
    Expired,
    Superseded,
}

impl RecommendationLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Available => "available",
            Self::Presented => "presented",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ActionProposalError> {
        match value {
            "created" => Ok(Self::Created),
            "available" => Ok(Self::Available),
            "presented" => Ok(Self::Presented),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            "superseded" => Ok(Self::Superseded),
            other => Err(ActionProposalError::InvalidLifecycleTransition {
                from: other.into(),
                to: "parse".into(),
            }),
        }
    }

    /// Open (reviewable) states — eligible for expire / supersede continuity.
    pub fn is_open(self) -> bool {
        matches!(
            self,
            Self::Created | Self::Available | Self::Presented
        )
    }

    /// Terminal resolution states — excluded from active suggestion surfaces.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted | Self::Rejected | Self::Expired | Self::Superseded
        )
    }

    /// Valid transitions for governance continuity (architecture contract).
    pub fn allows_transition(self, to: Self) -> bool {
        use RecommendationLifecycleState::*;
        matches!(
            (self, to),
            (Created, Available)
                | (Created, Expired)
                | (Created, Superseded)
                | (Available, Presented)
                | (Available, Expired)
                | (Available, Superseded)
                | (Available, Rejected)
                | (Presented, Accepted)
                | (Presented, Rejected)
                | (Presented, Expired)
                | (Presented, Superseded)
                | (Presented, Available) // may leave view without resolution
        )
    }
}

/// How a recommendation was resolved (lifecycle metadata only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationResolutionType {
    Accepted,
    Rejected,
    Expired,
    Superseded,
}

impl RecommendationResolutionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ActionProposalError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            "superseded" => Ok(Self::Superseded),
            other => Err(ActionProposalError::InvalidLifecycleTransition {
                from: other.into(),
                to: "parse_resolution".into(),
            }),
        }
    }
}

/// Mutable lifecycle metadata — never mutates reasoning provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationLifecycle {
    pub state: RecommendationLifecycleState,
    pub created_at: String,
    pub presented_at: Option<String>,
    pub resolved_at: Option<String>,
    pub resolution_type: Option<RecommendationResolutionType>,
    /// Who drove the transition (actor id) — not a capability grant.
    pub transition_actor_id: Option<String>,
}

impl RecommendationLifecycle {
    pub fn created(now: impl Into<String>) -> Self {
        Self {
            state: RecommendationLifecycleState::Created,
            created_at: now.into(),
            presented_at: None,
            resolved_at: None,
            resolution_type: None,
            transition_actor_id: None,
        }
    }

    pub fn transition(
        &mut self,
        to: RecommendationLifecycleState,
        at: impl Into<String>,
        actor_id: Option<String>,
    ) -> Result<(), ActionProposalError> {
        if !self.state.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.state.as_str().into(),
                to: to.as_str().into(),
            });
        }
        let at = at.into();
        match to {
            RecommendationLifecycleState::Presented => {
                self.presented_at = Some(at);
            }
            RecommendationLifecycleState::Accepted => {
                self.resolved_at = Some(at);
                self.resolution_type = Some(RecommendationResolutionType::Accepted);
            }
            RecommendationLifecycleState::Rejected => {
                self.resolved_at = Some(at);
                self.resolution_type = Some(RecommendationResolutionType::Rejected);
            }
            RecommendationLifecycleState::Expired => {
                self.resolved_at = Some(at);
                self.resolution_type = Some(RecommendationResolutionType::Expired);
            }
            RecommendationLifecycleState::Superseded => {
                self.resolved_at = Some(at);
                self.resolution_type = Some(RecommendationResolutionType::Superseded);
            }
            RecommendationLifecycleState::Available | RecommendationLifecycleState::Created => {}
        }
        self.state = to;
        self.transition_actor_id = actor_id;
        Ok(())
    }
}

/// Required provenance for answering: "Why was this recommendation created?"
///
/// **Immutable after construction** for reasoning fields — clone and attach traces
/// via builders that return new values; never rewrite `reasoning_origins`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationProvenance {
    pub recommendation_id: String,
    pub family: RecommendationFamily,
    /// Grounding evidence from source models.
    pub source_evidence: Vec<RecommendationEvidence>,
    /// Structured Attention reasons when the recommendation derives from Attention.
    pub reasoning_origins: Vec<AttentionReason>,
    /// Stable explanation keys for Experience translation / traces.
    pub explanation_keys: Vec<String>,
    /// Optional Experience resolver match keys (from translation traces).
    pub experience_trace_match_keys: Vec<String>,
    pub confidence: Option<String>,
    pub priority_or_impact: Option<String>,
    pub related_attention_id: Option<String>,
    /// Optional future capability target (architecture only — not a grant).
    pub future_capability_target: Option<String>,
}

impl RecommendationProvenance {
    /// Build provenance from a Recommendation Engine candidate.
    /// Does not invent meaning — copies structured fields only.
    pub fn from_recommendation_item(item: &RecommendationItem) -> Self {
        let explanation_keys = item
            .attention_reasons
            .iter()
            .map(|r| r.explanation_key.clone())
            .collect();
        Self {
            recommendation_id: item.id.clone(),
            family: RecommendationFamily::RecommendationEngine,
            source_evidence: item.evidence.clone(),
            reasoning_origins: item.attention_reasons.clone(),
            explanation_keys,
            experience_trace_match_keys: Vec::new(),
            confidence: Some(item.confidence.as_str().to_string()),
            priority_or_impact: Some(item.impact.clone()),
            related_attention_id: item.related_attention_id.clone(),
            future_capability_target: None,
        }
    }

    /// Attach Experience translation match keys (developer / future governance).
    pub fn with_experience_trace_match_keys(mut self, keys: Vec<String>) -> Self {
        self.experience_trace_match_keys = keys;
        self
    }
}

/// Governance record: identity + immutable provenance + mutable lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationGovernanceRecord {
    pub identity: RecommendationIdentity,
    pub provenance: RecommendationProvenance,
    pub lifecycle: RecommendationLifecycle,
    pub authority_effect: String,
}

impl RecommendationGovernanceRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_recommendation_item(
        item: &RecommendationItem,
        created_at: impl Into<String>,
    ) -> Self {
        let provenance = RecommendationProvenance::from_recommendation_item(item);
        let identity = RecommendationIdentity::from_recommendation_item(item);
        Self {
            identity,
            provenance,
            lifecycle: RecommendationLifecycle::created(created_at),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn transition(
        &mut self,
        to: RecommendationLifecycleState,
        at: impl Into<String>,
        actor_id: Option<String>,
    ) -> Result<(), ActionProposalError> {
        self.lifecycle.transition(to, at, actor_id)
    }

    /// Acceptance updates lifecycle only — never grants authority or executes.
    pub fn accept(
        &mut self,
        at: impl Into<String>,
        actor_id: Option<String>,
    ) -> Result<(), ActionProposalError> {
        self.transition(RecommendationLifecycleState::Accepted, at, actor_id)?;
        assert_eq!(self.authority_effect, Self::AUTHORITY_EFFECT_NONE);
        Ok(())
    }

    /// Record a governance outcome after resolution. Preserves provenance by clone.
    pub fn record_outcome(
        &self,
        recorded_at: impl Into<String>,
    ) -> Result<RecommendationOutcome, ActionProposalError> {
        RecommendationOutcome::from_governance_record(self, recorded_at)
    }
}

/// Human decision captured at outcome time (distinct from system success/failure).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationUserDecision {
    Accepted,
    Rejected,
    /// Returned to available / postponed without final resolution.
    Deferred,
    Expired,
    Superseded,
}

impl RecommendationUserDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }
}

/// Resulting outcome kind — rejection/expiry are **not** system failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationResultKind {
    /// Human accepted; follow-through / handoff may occur later under Gateway.
    AcceptedFollowThrough,
    /// Human rejected — valid outcome, not a failure.
    RejectedByUser,
    /// Timed out / source gone — valid outcome, not a failure.
    ExpiredWithoutAction,
    /// Replaced by a newer recommendation — valid outcome, not a failure.
    Superseded,
    /// Architecture: later link to a Gateway-gated execution outcome (optional).
    DownstreamExecutionLinked,
}

impl RecommendationResultKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedFollowThrough => "accepted_follow_through",
            Self::RejectedByUser => "rejected_by_user",
            Self::ExpiredWithoutAction => "expired_without_action",
            Self::Superseded => "superseded",
            Self::DownstreamExecutionLinked => "downstream_execution_linked",
        }
    }

    /// System failure semantics for feedback — user rejection/expiry never count as failure.
    pub fn is_system_failure(self) -> bool {
        false
    }
}

/// Quality / confidence metadata for feedback (never mutates scoring automatically).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationOutcomeQuality {
    pub confidence_at_outcome: Option<String>,
    pub useful_to_user: Option<bool>,
    pub notes: Option<String>,
}

impl RecommendationOutcomeQuality {
    pub fn from_provenance(provenance: &RecommendationProvenance) -> Self {
        Self {
            confidence_at_outcome: provenance.confidence.clone(),
            useful_to_user: None,
            notes: None,
        }
    }
}

/// True recommendation outcome record (Sprint 139).
///
/// Completes the loop: Evidence → … → Decision → Outcome.
/// May inform future Adaptation proposals; must not mutate historical reasoning,
/// silently rescore Attention/Decision, or execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationOutcome {
    pub id: String,
    pub identity: RecommendationIdentity,
    /// Frozen provenance snapshot at outcome time (immutable copy).
    pub provenance: RecommendationProvenance,
    pub lifecycle_resolution: Option<RecommendationResolutionType>,
    pub user_decision: RecommendationUserDecision,
    pub result_kind: RecommendationResultKind,
    pub recorded_at: String,
    pub quality: RecommendationOutcomeQuality,
    /// Experience translation match keys for the full debug chain.
    pub experience_trace_match_keys: Vec<String>,
    pub authority_effect: String,
}

/// Durable lifecycle overlay for Recommendation Engine candidates (Sprint 192).
///
/// Stores lifecycle + optional outcome snapshot only — never full candidate payloads.
/// Distinct from Decision Engine / Decision Queue overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationLifecycleOverlay {
    pub workspace_id: String,
    pub native_id: String,
    pub lifecycle_state: RecommendationLifecycleState,
    pub created_at: String,
    pub presented_at: Option<String>,
    pub resolved_at: Option<String>,
    pub resolution_type: Option<RecommendationResolutionType>,
    pub actor_id: Option<String>,
    pub outcome: Option<RecommendationOutcome>,
    /// Prior outcomes retained across supersede / generation reopen (Sprint 207).
    #[serde(default)]
    pub prior_outcomes: Vec<RecommendationOutcome>,
    /// Continuity fingerprint of the regenerable candidate payload (Sprint 197).
    #[serde(default)]
    pub content_fingerprint: Option<String>,
    /// Explicit confirmation beyond accept-as-agreement (Sprint 227).
    #[serde(default)]
    pub decision_confirmation:
        Option<crate::workspace_recommendation::RecommendationDecisionConfirmation>,
    pub updated_at: String,
    pub authority_effect: String,
}

impl RecommendationLifecycleOverlay {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_governance_record(
        workspace_id: impl Into<String>,
        record: &RecommendationGovernanceRecord,
        outcome: Option<RecommendationOutcome>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            native_id: record.identity.native_id.clone(),
            lifecycle_state: record.lifecycle.state,
            created_at: record.lifecycle.created_at.clone(),
            presented_at: record.lifecycle.presented_at.clone(),
            resolved_at: record.lifecycle.resolved_at.clone(),
            resolution_type: record.lifecycle.resolution_type,
            actor_id: record.lifecycle.transition_actor_id.clone(),
            outcome,
            prior_outcomes: Vec::new(),
            content_fingerprint: None,
            decision_confirmation: None,
            updated_at: updated_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_content_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.content_fingerprint = Some(fingerprint.into());
        self
    }

    pub fn with_prior_outcomes(mut self, prior: Vec<RecommendationOutcome>) -> Self {
        self.prior_outcomes = prior;
        self
    }

    pub fn with_decision_confirmation(
        mut self,
        confirmation: crate::workspace_recommendation::RecommendationDecisionConfirmation,
    ) -> Self {
        self.decision_confirmation = Some(confirmation);
        self
    }

    /// Accumulate current + prior outcomes when opening a new generation.
    pub fn carried_outcomes(&self) -> Vec<RecommendationOutcome> {
        let mut carried = self.prior_outcomes.clone();
        if let Some(outcome) = &self.outcome {
            carried.push(outcome.clone());
        }
        carried
    }

    pub fn apply_to_item(&self, item: &mut crate::workspace_recommendation::RecommendationItem) {
        item.lifecycle_state = Some(self.lifecycle_state.as_str().into());
        item.lifecycle_presented_at = self.presented_at.clone();
        item.lifecycle_resolved_at = self.resolved_at.clone();
        item.lifecycle_resolution_type = self.resolution_type.map(|r| r.as_str().into());
    }

    /// Rebuild a governance record for continuity transitions when the live item is gone.
    pub fn to_governance_record_for_continuity(&self) -> RecommendationGovernanceRecord {
        RecommendationGovernanceRecord {
            identity: RecommendationIdentity {
                native_id: self.native_id.clone(),
                family: RecommendationFamily::RecommendationEngine,
                source_domain: "recommendation_engine".into(),
                originating_reasoning_ref: None,
                decision_ref: None,
                action_proposal_ref: None,
            },
            provenance: RecommendationProvenance {
                recommendation_id: self.native_id.clone(),
                family: RecommendationFamily::RecommendationEngine,
                source_evidence: Vec::new(),
                reasoning_origins: Vec::new(),
                explanation_keys: Vec::new(),
                experience_trace_match_keys: Vec::new(),
                confidence: None,
                priority_or_impact: None,
                related_attention_id: None,
                future_capability_target: None,
            },
            lifecycle: RecommendationLifecycle {
                state: self.lifecycle_state,
                created_at: self.created_at.clone(),
                presented_at: self.presented_at.clone(),
                resolved_at: self.resolved_at.clone(),
                resolution_type: self.resolution_type,
                transition_actor_id: self.actor_id.clone(),
            },
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// IPC/service result for present / accept / reject — never executes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationReviewActionResult {
    pub workspace_id: String,
    pub recommendation_id: String,
    pub lifecycle_state: String,
    pub outcome: Option<RecommendationOutcome>,
    /// Structured future-DE intake context — observational only.
    #[serde(default)]
    pub decision_context: Option<crate::workspace_recommendation::RecommendationDecisionContext>,
    /// Read-only RE→DE readiness — never a handoff payload, never commands.
    #[serde(default)]
    pub decision_readiness: Option<crate::workspace_recommendation::RecommendationDecisionReadiness>,
    /// Explicit RE↔DE ownership / intent boundary — never executes.
    #[serde(default)]
    pub decision_boundary: Option<crate::workspace_recommendation::RecommendationDecisionBoundary>,
    /// Explicit confirmation beyond accept — never creates DE/intent/execution.
    #[serde(default)]
    pub decision_confirmation:
        Option<crate::workspace_recommendation::RecommendationDecisionConfirmation>,
    /// Typed future-DE intake package after confirmation — never creates DE objects.
    #[serde(default)]
    pub decision_intake:
        Option<crate::workspace_recommendation::RecommendationDecisionIntakeRequest>,
    /// Integrity inspection of intake — inspect ≠ handoff / DE ownership.
    #[serde(default)]
    pub decision_intake_inspection:
        Option<crate::workspace_recommendation::RecommendationDecisionIntakeInspection>,
    /// Versioned intake package identity — compatible ≠ transfer / handoff.
    #[serde(default)]
    pub decision_intake_compatibility:
        Option<crate::workspace_recommendation::RecommendationDecisionIntakeCompatibility>,
    /// Compatible ≠ proceed / consume / adapter permission.
    #[serde(default)]
    pub decision_intake_proceed_denial:
        Option<crate::workspace_recommendation::RecommendationDecisionIntakeProceedDenial>,
    pub explanation: String,
    pub authority_effect: String,
}

impl RecommendationReviewActionResult {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

impl RecommendationOutcome {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_governance_record(
        record: &RecommendationGovernanceRecord,
        recorded_at: impl Into<String>,
    ) -> Result<Self, ActionProposalError> {
        let resolution = record
            .lifecycle
            .resolution_type
            .ok_or(ActionProposalError::OutcomeRequiresResolution)?;
        let (user_decision, result_kind) = match resolution {
            RecommendationResolutionType::Accepted => (
                RecommendationUserDecision::Accepted,
                RecommendationResultKind::AcceptedFollowThrough,
            ),
            RecommendationResolutionType::Rejected => (
                RecommendationUserDecision::Rejected,
                RecommendationResultKind::RejectedByUser,
            ),
            RecommendationResolutionType::Expired => (
                RecommendationUserDecision::Expired,
                RecommendationResultKind::ExpiredWithoutAction,
            ),
            RecommendationResolutionType::Superseded => (
                RecommendationUserDecision::Superseded,
                RecommendationResultKind::Superseded,
            ),
        };
        Ok(Self {
            id: format!("recommendation_outcome:{}", record.identity.native_id),
            identity: record.identity.clone(),
            provenance: record.provenance.clone(),
            lifecycle_resolution: Some(resolution),
            user_decision,
            result_kind,
            recorded_at: recorded_at.into(),
            quality: RecommendationOutcomeQuality::from_provenance(&record.provenance),
            experience_trace_match_keys: record.provenance.experience_trace_match_keys.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn with_experience_trace_match_keys(mut self, keys: Vec<String>) -> Self {
        self.experience_trace_match_keys = keys;
        self
    }

    pub fn with_useful_flag(mut self, useful: bool) -> Self {
        self.quality.useful_to_user = Some(useful);
        self
    }

    /// Rejection and expiry are valid outcomes — never treated as system failure here.
    pub fn is_system_failure(&self) -> bool {
        self.result_kind.is_system_failure()
    }

    /// Hard-fail — outcomes never execute or authorize.
    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::OutcomeCannotExecute)
    }

    /// Feedback boundary: may be read by future Adaptation as evidence — never auto-applied.
    pub fn may_inform_future_adaptation(&self) -> bool {
        true
    }

    pub fn may_mutate_historical_reasoning(&self) -> bool {
        false
    }

    pub fn may_silently_change_scoring(&self) -> bool {
        false
    }

    /// Build a governed adaptation proposal from this outcome — never auto-applies.
    pub fn to_adaptation_proposal(
        &self,
        affected_area: impl Into<String>,
        proposed_change: impl Into<String>,
        expected_effect: impl Into<String>,
    ) -> OutcomeAdaptationProposal {
        OutcomeAdaptationProposal::from_outcome(
            self,
            affected_area,
            proposed_change,
            expected_effect,
        )
    }
}

/// Review / approval lifecycle for outcome-backed adaptation (Sprint 140–141).
///
/// Canonical chain (Sprint 141):
/// `Proposed → PendingReview(AwaitingReview) → Approved|Rejected → Applied(future) → Evaluated`
///
/// Sprint 140 names `AwaitingReview` / `ApprovedForHandoff` are kept as the Pending /
/// Approved wire values. `Reviewed` remains an optional audit acknowledgement step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeAdaptationReviewStatus {
    Proposed,
    /// Pending Review (Sprint 141 name).
    AwaitingReview,
    /// Optional human acknowledgement before Approved.
    Reviewed,
    /// Approved for controlled change surface / handoff — **not** Applied.
    ApprovedForHandoff,
    Rejected,
    Expired,
    /// Future only — must not be reached by auto-apply.
    Applied,
    /// After Applied evaluation (future only).
    Evaluated,
}

impl OutcomeAdaptationReviewStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::AwaitingReview => "awaiting_review",
            Self::Reviewed => "reviewed",
            Self::ApprovedForHandoff => "approved_for_handoff",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Applied => "applied",
            Self::Evaluated => "evaluated",
        }
    }

    /// Sprint 141 lifecycle label (human docs).
    pub fn lifecycle_label(self) -> &'static str {
        match self {
            Self::Proposed => "Proposed",
            Self::AwaitingReview => "Pending Review",
            Self::Reviewed => "Reviewed",
            Self::ApprovedForHandoff => "Approved",
            Self::Rejected => "Rejected",
            Self::Expired => "Expired",
            Self::Applied => "Applied",
            Self::Evaluated => "Evaluated",
        }
    }

    pub fn is_pending_review(self) -> bool {
        matches!(self, Self::AwaitingReview | Self::Reviewed)
    }

    pub fn is_approved(self) -> bool {
        matches!(self, Self::ApprovedForHandoff)
    }

    pub fn is_terminal_without_apply(self) -> bool {
        matches!(self, Self::Rejected | Self::Expired)
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Proposed, Self::AwaitingReview)
                | (Self::Proposed, Self::Rejected)
                | (Self::Proposed, Self::Expired)
                | (Self::AwaitingReview, Self::Reviewed)
                | (Self::AwaitingReview, Self::Rejected)
                | (Self::AwaitingReview, Self::Expired)
                | (Self::Reviewed, Self::ApprovedForHandoff)
                | (Self::Reviewed, Self::Rejected)
                | (Self::Reviewed, Self::Expired)
                // Applied / Evaluated are architecture-only; transitions gated in methods.
                | (Self::ApprovedForHandoff, Self::Applied)
                | (Self::Applied, Self::Evaluated)
        )
    }
}

/// Who may approve adaptation — distinct from Permission Gateway actors/grants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationReviewerIdentity {
    pub actor_id: String,
    /// Must be `local_user` for approve/reject decisions in Sprint 141.
    pub actor_type: String,
}

impl AdaptationReviewerIdentity {
    pub const LOCAL_USER_TYPE: &'static str = "local_user";

    pub fn local_user(actor_id: impl Into<String>) -> Self {
        Self {
            actor_id: actor_id.into(),
            actor_type: Self::LOCAL_USER_TYPE.into(),
        }
    }

    pub fn is_local_user(&self) -> bool {
        self.actor_type == Self::LOCAL_USER_TYPE
    }
}

/// Audit event for adaptation review (architecture record; not a capability grant).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationReviewAuditEvent {
    pub at: String,
    pub action: String,
    pub actor_id: String,
    pub note: Option<String>,
}

/// Rollback metadata for a controlled change (architecture — no auto-rollback).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRollbackMetadata {
    pub rollback_to_version_id: String,
    pub rollback_reason_template: String,
    /// Always false — rollback is explicit and governed.
    pub may_auto_rollback: bool,
}

impl ChangeRollbackMetadata {
    pub fn for_previous(previous_version_id: impl Into<String>) -> Self {
        let rollback_to_version_id = previous_version_id.into();
        Self {
            rollback_reason_template: format!(
                "Roll back controlled change to {rollback_to_version_id}"
            ),
            rollback_to_version_id,
            may_auto_rollback: false,
        }
    }
}

/// Audit metadata attached to a ControlledChangeSurface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlledChangeAuditMetadata {
    pub created_at: String,
    pub approval_reference: String,
    pub review_audit_actions: Vec<String>,
}

/// Lifecycle for versioned behaviour definitions (architecture only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviourVersionLifecycle {
    /// Draft prepared from an approved ControlledChangeSurface — not runtime-active.
    Draft,
    /// Future only — publishing is blocked in Sprint 142.
    Published,
    Superseded,
    RolledBack,
}

impl BehaviourVersionLifecycle {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Superseded => "superseded",
            Self::RolledBack => "rolled_back",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Draft, Self::Published)
                | (Self::Published, Self::Superseded)
                | (Self::Published, Self::RolledBack)
                | (Self::Superseded, Self::RolledBack)
        )
    }
}

/// Versioned behaviour identity — does **not** mutate runtime cognition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviourVersion {
    pub id: String,
    pub creation_source: String,
    pub change_reason: String,
    pub approval_reference: String,
    pub previous_version_id: Option<String>,
    pub lifecycle: BehaviourVersionLifecycle,
    /// Frozen provenance from the originating recommendation chain.
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl BehaviourVersion {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const CREATION_SOURCE_OUTCOME_ADAPTATION: &'static str = "outcome_adaptation";
    pub const BASELINE_ID: &'static str = "behaviour:v0_baseline";

    pub fn draft_from_surface(
        surface: &ControlledChangeSurface,
    ) -> Result<Self, ActionProposalError> {
        if surface.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(ActionProposalError::CannotExecute);
        }
        Ok(Self {
            id: surface.target_version_id.clone(),
            creation_source: Self::CREATION_SOURCE_OUTCOME_ADAPTATION.into(),
            change_reason: surface.approved_change.clone(),
            approval_reference: surface.originating_proposal_id.clone(),
            previous_version_id: Some(surface.previous_version_id.clone()),
            lifecycle: BehaviourVersionLifecycle::Draft,
            provenance: surface.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Unapproved proposals must not create behaviour versions.
    pub fn from_unapproved_proposal(
        _proposal: &OutcomeAdaptationProposal,
    ) -> Result<Self, ActionProposalError> {
        Err(ActionProposalError::UnapprovedControlledChangeForbidden)
    }

    /// Prepare a rollback draft — preserves provenance; does not mutate history.
    pub fn prepare_rollback_draft(&self, at_note: impl Into<String>) -> Result<Self, ActionProposalError> {
        let previous = self
            .previous_version_id
            .clone()
            .unwrap_or_else(|| Self::BASELINE_ID.into());
        let _ = at_note;
        Ok(Self {
            id: format!("behaviour:rollback:{}", self.id),
            creation_source: Self::CREATION_SOURCE_OUTCOME_ADAPTATION.into(),
            change_reason: format!("rollback to {previous}"),
            approval_reference: self.approval_reference.clone(),
            previous_version_id: Some(previous),
            lifecycle: BehaviourVersionLifecycle::Draft,
            provenance: self.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn may_mutate_runtime(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_publish(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::BehaviourVersionPublishNotImplemented)
    }

    pub fn attempt_mutate_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Evaluation of a governed change — observational; cannot rewrite history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeEvaluation {
    pub id: String,
    pub change_reference: String,
    pub observed_effects: Vec<String>,
    pub success_criteria: Vec<String>,
    pub rollback_recommendation: Option<String>,
    /// Immutable snapshot of provenance at evaluation time.
    pub provenance_snapshot: RecommendationProvenance,
    pub authority_effect: String,
}

impl ChangeEvaluation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_behaviour_version(
        version: &BehaviourVersion,
        observed_effects: Vec<String>,
        success_criteria: Vec<String>,
        rollback_recommendation: Option<String>,
    ) -> Self {
        Self {
            id: format!("change_eval:{}", version.id),
            change_reference: version.id.clone(),
            observed_effects,
            success_criteria,
            rollback_recommendation,
            provenance_snapshot: version.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_rewrite_history(&self) -> bool {
        false
    }

    pub fn may_mutate_runtime(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_rewrite_provenance(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::ChangeEvaluationCannotRewriteHistory)
    }

    pub fn attempt_mutate_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Actor references on a governance ledger entry (not capability grants).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceActorRefs {
    pub proposer_actor_id: String,
    pub reviewer_actor_id: Option<String>,
    pub publisher_actor_id: Option<String>,
}

/// Risk classification for adaptation governance policy (Sprint 144–145).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptationRiskClass {
    Low,
    Medium,
    High,
}

impl AdaptationRiskClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub fn requires_stronger_review(self) -> bool {
        matches!(self, Self::High)
    }
}

/// Who may review under a GovernancePolicy / GovernanceRisk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewerRequirements {
    pub required_actor_type: String,
    pub min_reviewers: u32,
    pub allow_self_approval: bool,
    pub require_rationale: bool,
}

/// Expiry rules for adaptation governance (architecture).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceExpiryRules {
    /// Descriptive TTL for proposals (e.g. `P7D`).
    pub proposal_ttl: Option<String>,
    pub review_ttl: Option<String>,
    pub publish_request_ttl: Option<String>,
    pub expired_blocks_approval: bool,
}

/// Impact classification for a governed change (Sprint 145).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceImpactClass {
    PresentationOnly,
    WorkflowHint,
    BehaviourVersionDraft,
    /// Classified only to reject — never a valid publish path.
    CognitionScoringMutation,
    /// Classified only to reject — execution stays on Permission Gateway.
    ExecutionAuthority,
}

impl GovernanceImpactClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PresentationOnly => "presentation_only",
            Self::WorkflowHint => "workflow_hint",
            Self::BehaviourVersionDraft => "behaviour_version_draft",
            Self::CognitionScoringMutation => "cognition_scoring_mutation",
            Self::ExecutionAuthority => "execution_authority",
        }
    }

    pub fn is_forbidden_adaptation_path(self) -> bool {
        matches!(
            self,
            Self::CognitionScoringMutation | Self::ExecutionAuthority
        )
    }
}

/// Change risk profile that drives review routing (Sprint 145).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRisk {
    pub id: String,
    pub risk_level: AdaptationRiskClass,
    pub affected_domain: String,
    pub impact_classification: GovernanceImpactClass,
    pub review_requirements: GovernanceReviewerRequirements,
    pub approval_threshold: u32,
    /// High-risk approvals must carry explicit conditions.
    pub requires_conditions: bool,
    pub authority_effect: String,
}

impl GovernanceRisk {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn classify_from_proposal(proposal: &OutcomeAdaptationProposal) -> Self {
        let area = proposal.affected_area.to_lowercase();
        let (level, impact) = if area.contains("scoring")
            || area.contains("attention_weight")
            || area.contains("decision_weight")
            || area.contains("cognition")
        {
            (
                AdaptationRiskClass::High,
                GovernanceImpactClass::CognitionScoringMutation,
            )
        } else if area.contains("permission")
            || area.contains("capability")
            || area.contains("gateway")
            || area.contains("execution")
        {
            (
                AdaptationRiskClass::High,
                GovernanceImpactClass::ExecutionAuthority,
            )
        } else if area.contains("behaviour") || area.contains("workflow") {
            (
                AdaptationRiskClass::High,
                GovernanceImpactClass::BehaviourVersionDraft,
            )
        } else if area.contains("presentation") || area.contains("experience") {
            (
                AdaptationRiskClass::Low,
                GovernanceImpactClass::PresentationOnly,
            )
        } else {
            (
                AdaptationRiskClass::Medium,
                GovernanceImpactClass::WorkflowHint,
            )
        };
        Self::from_level(level, proposal.affected_area.clone(), impact)
    }

    pub fn from_level(
        risk_level: AdaptationRiskClass,
        affected_domain: impl Into<String>,
        impact_classification: GovernanceImpactClass,
    ) -> Self {
        let affected_domain = affected_domain.into();
        let (min_reviewers, approval_threshold, requires_conditions) = match risk_level {
            AdaptationRiskClass::Low => (1, 1, false),
            AdaptationRiskClass::Medium => (1, 1, false),
            AdaptationRiskClass::High => (2, 2, true),
        };
        Self {
            id: format!(
                "governance_risk:{}:{}:{}",
                risk_level.as_str(),
                impact_classification.as_str(),
                affected_domain
            ),
            risk_level,
            affected_domain,
            impact_classification,
            review_requirements: GovernanceReviewerRequirements {
                required_actor_type: AdaptationReviewerIdentity::LOCAL_USER_TYPE.into(),
                min_reviewers,
                allow_self_approval: false,
                require_rationale: true,
            },
            approval_threshold,
            requires_conditions,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn required_reviewer_count(&self) -> u32 {
        self.review_requirements
            .min_reviewers
            .max(self.approval_threshold)
    }

    pub fn enforce_not_cognition_mutation(&self) -> Result<(), ActionProposalError> {
        if self.impact_classification.is_forbidden_adaptation_path() {
            return Err(ActionProposalError::GovernanceRiskCannotMutateCognition);
        }
        Ok(())
    }

    pub fn may_mutate_cognition(&self) -> bool {
        false
    }

    pub fn may_silently_change_scoring(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }

    pub fn attempt_mutate_cognition() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceRiskCannotMutateCognition)
    }
}

/// Review routing: Change → Risk → Policy → Required Reviewers → Decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewRouting {
    pub risk: GovernanceRisk,
    pub policy: GovernancePolicy,
    pub required_reviewer_count: u32,
    pub required_reviewer_actor_type: String,
    pub change_reference: String,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceReviewRouting {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn route_change(
        proposal: &OutcomeAdaptationProposal,
        risk: Option<GovernanceRisk>,
    ) -> Result<Self, ActionProposalError> {
        let risk = risk.unwrap_or_else(|| GovernanceRisk::classify_from_proposal(proposal));
        risk.enforce_not_cognition_mutation()?;
        let policy = GovernancePolicy::from_risk(&risk);
        Ok(Self {
            required_reviewer_count: risk.required_reviewer_count(),
            required_reviewer_actor_type: risk.review_requirements.required_actor_type.clone(),
            change_reference: proposal.id.clone(),
            provenance: proposal.provenance.clone(),
            policy,
            risk,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn enforce_decisions(
        &self,
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
    ) -> Result<(), ActionProposalError> {
        self.risk.enforce_not_cognition_mutation()?;
        self.policy
            .enforce_approval_requirements(proposal, decisions)?;
        if self.risk.requires_conditions
            && decisions
                .iter()
                .filter(|d| d.decision == GovernanceReviewDecisionKind::Approve)
                .any(|d| d.conditions.is_empty())
        {
            return Err(ActionProposalError::GovernanceRiskRequirementsNotMet);
        }
        for decision in decisions {
            if !decision.retains_provenance(&self.provenance) {
                return Err(ActionProposalError::ReviewerCannotBypassProvenance);
            }
        }
        Ok(())
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_mutate_cognition(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Policy layer governing how adaptation changes are reviewed (Sprint 144).
///
/// Distinct from Permission / CapabilityBound policies — never grants execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePolicy {
    pub id: String,
    pub governed_domain: String,
    pub risk_classification: AdaptationRiskClass,
    pub reviewer_requirements: GovernanceReviewerRequirements,
    /// Number of approving LocalUser decisions required.
    pub approval_threshold: u32,
    pub expiry_rules: GovernanceExpiryRules,
    pub authority_effect: String,
}

impl GovernancePolicy {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const DOMAIN_OUTCOME_ADAPTATION: &'static str = "outcome_adaptation";

    pub fn for_outcome_adaptation() -> Self {
        Self::from_risk(&GovernanceRisk::from_level(
            AdaptationRiskClass::Medium,
            Self::DOMAIN_OUTCOME_ADAPTATION,
            GovernanceImpactClass::WorkflowHint,
        ))
    }

    /// Build policy requirements from a classified GovernanceRisk (Sprint 145 routing).
    pub fn from_risk(risk: &GovernanceRisk) -> Self {
        Self {
            id: format!(
                "governance_policy:{}:{}",
                risk.risk_level.as_str(),
                risk.affected_domain
            ),
            governed_domain: risk.affected_domain.clone(),
            risk_classification: risk.risk_level,
            reviewer_requirements: risk.review_requirements.clone(),
            approval_threshold: risk.approval_threshold,
            expiry_rules: GovernanceExpiryRules {
                proposal_ttl: Some("P7D".into()),
                review_ttl: Some("P3D".into()),
                publish_request_ttl: Some("P3D".into()),
                expired_blocks_approval: true,
            },
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn reviewer_satisfies(&self, reviewer: &AdaptationReviewerIdentity) -> bool {
        reviewer.actor_type == self.reviewer_requirements.required_actor_type
            && reviewer.is_local_user()
    }

    pub fn evaluate_expiry(
        &self,
        proposal: &OutcomeAdaptationProposal,
    ) -> Result<(), ActionProposalError> {
        if self.expiry_rules.expired_blocks_approval
            && proposal.review_status == OutcomeAdaptationReviewStatus::Expired
        {
            return Err(ActionProposalError::GovernancePolicyExpired);
        }
        Ok(())
    }

    /// Enforce reviewer + approval threshold before publish / ledger advancement.
    pub fn enforce_approval_requirements(
        &self,
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
    ) -> Result<(), ActionProposalError> {
        self.evaluate_expiry(proposal)?;
        if self.reviewer_requirements.require_rationale
            && decisions.iter().any(|d| d.rationale.trim().is_empty())
        {
            return Err(ActionProposalError::GovernanceReviewRationaleRequired);
        }
        for decision in decisions {
            if !self.reviewer_satisfies(&decision.reviewer) {
                return Err(ActionProposalError::AdaptationReviewerRequired);
            }
            if !self.reviewer_requirements.allow_self_approval
                && (decision.reviewer.actor_id == proposal.proposed_by_actor_id
                    || decision.reviewer.actor_type == OutcomeAdaptationProposal::PROPOSER_ACTOR_TYPE)
            {
                return Err(ActionProposalError::AdaptationSelfApprovalForbidden);
            }
            if decision.may_bypass_provenance() {
                return Err(ActionProposalError::ReviewerCannotBypassProvenance);
            }
        }
        let approving: Vec<_> = decisions
            .iter()
            .filter(|d| d.decision == GovernanceReviewDecisionKind::Approve)
            .collect();
        let unique_reviewers: std::collections::BTreeSet<_> =
            approving.iter().map(|d| d.reviewer.actor_id.as_str()).collect();
        if unique_reviewers.len() < self.reviewer_requirements.min_reviewers as usize
            || approving.len() < self.approval_threshold as usize
        {
            return Err(ActionProposalError::GovernancePolicyRequirementsNotMet);
        }
        Ok(())
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }

    pub fn attempt_grant_execution_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }
}

/// Human review decision under a GovernancePolicy (Sprint 144).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReviewDecisionKind {
    Approve,
    Reject,
    RequestChanges,
}

impl GovernanceReviewDecisionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
            Self::RequestChanges => "request_changes",
        }
    }
}

/// Dissent / concern recorded during review — append-only; never erases approvals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDissentRecord {
    pub id: String,
    pub reviewer_actor_id: String,
    pub concern: String,
    pub timestamp: String,
}

/// Evidence package required for governance decisions (Sprint 146).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDecisionEvidence {
    pub id: String,
    pub supporting_evidence_references: Vec<String>,
    pub reviewer_concerns: Vec<String>,
    pub required_conditions: Vec<String>,
    pub dissent_records: Vec<GovernanceDissentRecord>,
    pub final_rationale: String,
    pub provenance_snapshot: RecommendationProvenance,
    pub risk_reference: Option<String>,
    pub authority_effect: String,
}

impl GovernanceDecisionEvidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        risk: &GovernanceRisk,
        supporting_evidence_references: Vec<String>,
        reviewer_concerns: Vec<String>,
        required_conditions: Vec<String>,
        final_rationale: impl Into<String>,
        provenance: &RecommendationProvenance,
    ) -> Result<Self, ActionProposalError> {
        let final_rationale = final_rationale.into();
        if supporting_evidence_references.is_empty() || final_rationale.trim().is_empty() {
            return Err(ActionProposalError::GovernanceDecisionEvidenceRequired);
        }
        Ok(Self {
            id: format!("gov_evidence:{}", risk.id),
            supporting_evidence_references,
            reviewer_concerns,
            required_conditions,
            dissent_records: Vec::new(),
            final_rationale,
            provenance_snapshot: provenance.clone(),
            risk_reference: Some(risk.id.clone()),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate_for_approval(&self) -> Result<(), ActionProposalError> {
        if self.supporting_evidence_references.is_empty() || self.final_rationale.trim().is_empty()
        {
            return Err(ActionProposalError::GovernanceDecisionEvidenceRequired);
        }
        Ok(())
    }

    /// Append dissent — never removes prior approvals or evidence history.
    pub fn record_dissent(
        &mut self,
        reviewer_actor_id: impl Into<String>,
        concern: impl Into<String>,
        timestamp: impl Into<String>,
    ) {
        let reviewer_actor_id = reviewer_actor_id.into();
        let concern = concern.into();
        let timestamp = timestamp.into();
        self.dissent_records.push(GovernanceDissentRecord {
            id: format!("dissent:{}:{}", reviewer_actor_id, self.dissent_records.len()),
            reviewer_actor_id: reviewer_actor_id.clone(),
            concern: concern.clone(),
            timestamp,
        });
        self.reviewer_concerns.push(concern);
    }

    pub fn dissent_count(&self) -> usize {
        self.dissent_records.len()
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn may_erase_approval_history(&self) -> bool {
        false
    }

    pub fn attempt_grant_execution_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Publication readiness lifecycle (architecture only — Sprint 146).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationReadinessState {
    Draft,
    RiskReviewed,
    Approved,
    ReadyForPublication,
    /// Future only — activation hard-fails.
    Published,
}

impl PublicationReadinessState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::RiskReviewed => "risk_reviewed",
            Self::Approved => "approved",
            Self::ReadyForPublication => "ready_for_publication",
            Self::Published => "published",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Draft, Self::RiskReviewed)
                | (Self::RiskReviewed, Self::Approved)
                | (Self::Approved, Self::ReadyForPublication)
                | (Self::ReadyForPublication, Self::Published)
        )
    }
}

/// Tracks readiness before future publication — never activates runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationReadiness {
    pub id: String,
    pub state: PublicationReadinessState,
    pub evidence_reference: String,
    pub governance_record_id: Option<String>,
    pub risk_reference: Option<String>,
    /// Append-only state history (dissent / later states never erase prior entries).
    pub history: Vec<PublicationReadinessState>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl PublicationReadiness {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn draft_from_evidence(evidence: &GovernanceDecisionEvidence) -> Self {
        Self {
            id: format!("pub_readiness:{}", evidence.id),
            state: PublicationReadinessState::Draft,
            evidence_reference: evidence.id.clone(),
            governance_record_id: None,
            risk_reference: evidence.risk_reference.clone(),
            history: vec![PublicationReadinessState::Draft],
            provenance: evidence.provenance_snapshot.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn transition(&mut self, to: PublicationReadinessState) -> Result<(), ActionProposalError> {
        if to == PublicationReadinessState::Published {
            return Err(ActionProposalError::PublicationReadinessCannotActivate);
        }
        if !self.state.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.state.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.state = to;
        self.history.push(to);
        Ok(())
    }

    pub fn mark_risk_reviewed(&mut self) -> Result<(), ActionProposalError> {
        self.transition(PublicationReadinessState::RiskReviewed)
    }

    pub fn mark_approved(
        &mut self,
        evidence: &GovernanceDecisionEvidence,
    ) -> Result<(), ActionProposalError> {
        evidence.validate_for_approval()?;
        self.transition(PublicationReadinessState::Approved)
    }

    pub fn mark_ready_for_publication(
        &mut self,
        evidence: &GovernanceDecisionEvidence,
    ) -> Result<(), ActionProposalError> {
        evidence.validate_for_approval()?;
        self.transition(PublicationReadinessState::ReadyForPublication)
    }

    pub fn with_governance_record(mut self, record: &GovernanceRecord) -> Self {
        self.governance_record_id = Some(record.id.clone());
        self
    }

    /// Dissent never removes prior history entries (including Approved / Ready).
    pub fn history_preserves_approvals(&self) -> bool {
        let approved_positions: Vec<_> = self
            .history
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == PublicationReadinessState::Approved)
            .map(|(i, _)| i)
            .collect();
        if approved_positions.is_empty() {
            return true;
        }
        // Every Approved entry remains present; history only grows.
        approved_positions
            .iter()
            .all(|&i| self.history.get(i) == Some(&PublicationReadinessState::Approved))
            && self.history.contains(&PublicationReadinessState::Approved)
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_activate_published(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationReadinessCannotActivate)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Architecture-only proposal panel for GovernanceWorkspace (Sprint 147).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceProposalView {
    pub proposal_reference: String,
    pub affected_area: String,
    pub proposed_change: String,
    pub expected_effect: String,
}

/// Architecture-only evidence panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceEvidenceView {
    pub evidence_reference: Option<String>,
    pub supporting_evidence_references: Vec<String>,
    pub dissent_count: usize,
    pub final_rationale: Option<String>,
}

/// Architecture-only risk panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRiskView {
    pub risk_reference: Option<String>,
    pub risk_level: Option<String>,
    pub impact_classification: Option<String>,
    pub required_reviewer_count: Option<u32>,
}

/// Architecture-only reviewer panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewerView {
    pub reviewer_actor_ids: Vec<String>,
    pub required_actor_type: String,
}

/// Decision history row for governance visibility (append-only display).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDecisionHistoryEntry {
    pub decision_reference: String,
    pub decision: String,
    pub reviewer_actor_id: String,
    pub timestamp: String,
    pub evidence_reference: Option<String>,
}

/// Human governance operating surface — visibility only; never executes (Sprint 147).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceWorkspace {
    pub id: String,
    pub governance_record_id: String,
    pub proposal_view: GovernanceProposalView,
    pub evidence_view: GovernanceEvidenceView,
    pub risk_view: GovernanceRiskView,
    pub reviewer_view: GovernanceReviewerView,
    pub decision_history: Vec<GovernanceDecisionHistoryEntry>,
    pub readiness_state: Option<PublicationReadinessState>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceWorkspace {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_governance_bundle(
        proposal: &OutcomeAdaptationProposal,
        record: &GovernanceRecord,
        evidence: Option<&GovernanceDecisionEvidence>,
        risk: Option<&GovernanceRisk>,
        decisions: &[GovernanceReviewDecision],
        readiness: Option<&PublicationReadiness>,
    ) -> Self {
        let reviewer_actor_ids: Vec<_> = decisions
            .iter()
            .map(|d| d.reviewer.actor_id.clone())
            .collect();
        let decision_history = decisions
            .iter()
            .map(|d| GovernanceDecisionHistoryEntry {
                decision_reference: d.id.clone(),
                decision: d.decision.as_str().into(),
                reviewer_actor_id: d.reviewer.actor_id.clone(),
                timestamp: d.timestamp.clone(),
                evidence_reference: d.evidence_reference.clone(),
            })
            .collect();
        Self {
            id: format!("governance_workspace:{}", record.id),
            governance_record_id: record.id.clone(),
            proposal_view: GovernanceProposalView {
                proposal_reference: proposal.id.clone(),
                affected_area: proposal.affected_area.clone(),
                proposed_change: proposal.proposed_change.clone(),
                expected_effect: proposal.expected_effect.clone(),
            },
            evidence_view: GovernanceEvidenceView {
                evidence_reference: evidence.map(|e| e.id.clone()),
                supporting_evidence_references: evidence
                    .map(|e| e.supporting_evidence_references.clone())
                    .unwrap_or_default(),
                dissent_count: evidence.map(|e| e.dissent_count()).unwrap_or(0),
                final_rationale: evidence.map(|e| e.final_rationale.clone()),
            },
            risk_view: GovernanceRiskView {
                risk_reference: risk.map(|r| r.id.clone()),
                risk_level: risk.map(|r| r.risk_level.as_str().into()),
                impact_classification: risk.map(|r| r.impact_classification.as_str().into()),
                required_reviewer_count: risk.map(|r| r.required_reviewer_count()),
            },
            reviewer_view: GovernanceReviewerView {
                reviewer_actor_ids,
                required_actor_type: AdaptationReviewerIdentity::LOCAL_USER_TYPE.into(),
            },
            decision_history,
            readiness_state: readiness
                .map(|r| r.state)
                .or(record.publication_readiness),
            provenance: record.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn preserves_provenance(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance == expected
    }

    pub fn may_approve_execution(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_approve_execution() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceWorkspaceCannotApproveExecution)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Staged rollout concept for publication environment (architecture only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationRolloutStage {
    None,
    StagedCanary,
    StagedPercent,
    Full,
}

impl PublicationRolloutStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::StagedCanary => "staged_canary",
            Self::StagedPercent => "staged_percent",
            Self::Full => "full",
        }
    }
}

/// Publication environment boundary — never activates runtime (Sprint 147).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationEnvironment {
    pub id: String,
    pub publication_target: String,
    pub workspace_scope: String,
    pub compatibility_requirements: Vec<String>,
    pub rollback_scope: String,
    pub staged_rollout: PublicationRolloutStage,
    pub governance_workspace_id: String,
    pub governance_record_id: String,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl PublicationEnvironment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const TARGET_FUTURE_BEHAVIOUR: &'static str = "future_behaviour_version";

    pub fn from_governance_workspace(
        workspace: &GovernanceWorkspace,
        workspace_scope: impl Into<String>,
        compatibility_requirements: Vec<String>,
        rollback_scope: impl Into<String>,
        staged_rollout: PublicationRolloutStage,
    ) -> Self {
        Self {
            id: format!("publication_env:{}", workspace.id),
            publication_target: Self::TARGET_FUTURE_BEHAVIOUR.into(),
            workspace_scope: workspace_scope.into(),
            compatibility_requirements,
            rollback_scope: rollback_scope.into(),
            staged_rollout,
            governance_workspace_id: workspace.id.clone(),
            governance_record_id: workspace.governance_record_id.clone(),
            provenance: workspace.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn preserves_provenance(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance == expected
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    /// Visibility / routing helper — does not create or activate a published version.
    pub fn bind_publish_request(
        &self,
        request: &PublishRequest,
    ) -> Result<(), ActionProposalError> {
        if request.governance_record_id != self.governance_record_id {
            return Err(ActionProposalError::PublicationRequiresApproval);
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(ActionProposalError::CannotExecute);
        }
        Ok(())
    }

    pub fn attempt_activate() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationEnvironmentCannotActivate)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Publication safety lifecycle (architecture only — Sprint 149).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationSafetyLifecycleState {
    ReadyForPublication,
    Validation,
    MigrationPrepared,
    RollbackPrepared,
    ReleaseApproved,
    /// Future only — activation hard-fails.
    Published,
    ValidationFailed,
}

impl PublicationSafetyLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForPublication => "ready_for_publication",
            Self::Validation => "validation",
            Self::MigrationPrepared => "migration_prepared",
            Self::RollbackPrepared => "rollback_prepared",
            Self::ReleaseApproved => "release_approved",
            Self::Published => "published",
            Self::ValidationFailed => "validation_failed",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::ReadyForPublication, Self::Validation)
                | (Self::Validation, Self::MigrationPrepared)
                | (Self::Validation, Self::ValidationFailed)
                | (Self::MigrationPrepared, Self::RollbackPrepared)
                | (Self::RollbackPrepared, Self::ReleaseApproved)
                | (Self::ReleaseApproved, Self::Published)
        )
    }
}

/// Compatibility check gate for future publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationCompatibilityCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

/// Migration requirement (architecture — does not apply SQL).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationMigrationRequirement {
    pub migration_id: String,
    pub description: String,
    pub prepared: bool,
}

/// Rollback requirement — history must remain preserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRollbackRequirement {
    pub rollback_to: String,
    pub history_preserved: bool,
    pub prepared: bool,
}

/// Named validation gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationValidationGate {
    pub name: String,
    pub required: bool,
    pub passed: bool,
}

/// Failure handling policy for publication safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationFailureHandling {
    pub on_validation_failure: String,
    pub on_activation_attempt: String,
    pub preserves_history: bool,
}

impl PublicationFailureHandling {
    pub fn default_safe() -> Self {
        Self {
            on_validation_failure: "block_publication".into(),
            on_activation_attempt: "hard_fail".into(),
            preserves_history: true,
        }
    }
}

/// Safety requirements a future publication system must satisfy before activation (Sprint 149).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSafetyContract {
    pub id: String,
    pub readiness_reference: String,
    pub environment_reference: String,
    pub compatibility_checks: Vec<PublicationCompatibilityCheck>,
    pub migration_requirements: Vec<PublicationMigrationRequirement>,
    pub rollback_requirements: Vec<PublicationRollbackRequirement>,
    pub validation_gates: Vec<PublicationValidationGate>,
    pub failure_handling: PublicationFailureHandling,
    pub lifecycle_state: PublicationSafetyLifecycleState,
    pub lifecycle_history: Vec<PublicationSafetyLifecycleState>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl PublicationSafetyContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_ready(
        readiness: &PublicationReadiness,
        environment: &PublicationEnvironment,
    ) -> Result<Self, ActionProposalError> {
        if readiness.state != PublicationReadinessState::ReadyForPublication {
            return Err(ActionProposalError::PublicationSafetyBlocked(
                "readiness must be ReadyForPublication".into(),
            ));
        }
        let compatibility_checks: Vec<_> = environment
            .compatibility_requirements
            .iter()
            .map(|req| PublicationCompatibilityCheck {
                name: req.clone(),
                passed: true,
                detail: format!("compatibility requirement declared: {req}"),
            })
            .collect();
        let mut gates = vec![
            PublicationValidationGate {
                name: "readiness_ready".into(),
                required: true,
                passed: true,
            },
            PublicationValidationGate {
                name: "environment_bound".into(),
                required: true,
                passed: !environment.governance_record_id.is_empty(),
            },
            PublicationValidationGate {
                name: "authority_effect_none".into(),
                required: true,
                passed: environment.authority_effect
                    == PublicationEnvironment::AUTHORITY_EFFECT_NONE
                    && readiness.authority_effect == PublicationReadiness::AUTHORITY_EFFECT_NONE,
            },
        ];
        for check in &compatibility_checks {
            gates.push(PublicationValidationGate {
                name: format!("compat:{}", check.name),
                required: true,
                passed: check.passed,
            });
        }
        Ok(Self {
            id: format!("publication_safety:{}", readiness.id),
            readiness_reference: readiness.id.clone(),
            environment_reference: environment.id.clone(),
            compatibility_checks,
            migration_requirements: vec![PublicationMigrationRequirement {
                migration_id: "behaviour:future_migration".into(),
                description: "Prepare ordered behaviour migration (architecture only)".into(),
                prepared: false,
            }],
            rollback_requirements: vec![PublicationRollbackRequirement {
                rollback_to: environment.rollback_scope.clone(),
                history_preserved: true,
                prepared: false,
            }],
            validation_gates: gates,
            failure_handling: PublicationFailureHandling::default_safe(),
            lifecycle_state: PublicationSafetyLifecycleState::ReadyForPublication,
            lifecycle_history: vec![PublicationSafetyLifecycleState::ReadyForPublication],
            provenance: readiness.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn transition(
        &mut self,
        to: PublicationSafetyLifecycleState,
    ) -> Result<(), ActionProposalError> {
        if to == PublicationSafetyLifecycleState::Published {
            return Err(ActionProposalError::PublicationActivationNotImplemented);
        }
        if !self.lifecycle_state.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.lifecycle_state.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.lifecycle_state = to;
        self.lifecycle_history.push(to);
        Ok(())
    }

    /// Run validation gates — failure blocks publication progression.
    pub fn run_validation(&mut self) -> Result<(), ActionProposalError> {
        self.transition(PublicationSafetyLifecycleState::Validation)?;
        let failed: Vec<_> = self
            .validation_gates
            .iter()
            .filter(|g| g.required && !g.passed)
            .map(|g| g.name.clone())
            .collect();
        if !failed.is_empty() {
            self.lifecycle_state = PublicationSafetyLifecycleState::ValidationFailed;
            self.lifecycle_history
                .push(PublicationSafetyLifecycleState::ValidationFailed);
            return Err(ActionProposalError::PublicationSafetyValidationFailed(
                failed.join(","),
            ));
        }
        Ok(())
    }

    pub fn with_failed_gate(mut self, gate_name: impl Into<String>) -> Self {
        let name = gate_name.into();
        if let Some(gate) = self.validation_gates.iter_mut().find(|g| g.name == name) {
            gate.passed = false;
        } else {
            self.validation_gates.push(PublicationValidationGate {
                name,
                required: true,
                passed: false,
            });
        }
        self
    }

    pub fn prepare_migration(&mut self) -> Result<(), ActionProposalError> {
        if self.lifecycle_state == PublicationSafetyLifecycleState::ValidationFailed {
            return Err(ActionProposalError::PublicationSafetyBlocked(
                "validation failed".into(),
            ));
        }
        if self.lifecycle_state == PublicationSafetyLifecycleState::ReadyForPublication {
            self.run_validation()?;
        }
        if self.lifecycle_state != PublicationSafetyLifecycleState::Validation {
            return Err(ActionProposalError::PublicationSafetyBlocked(
                "must validate before migration prepare".into(),
            ));
        }
        for req in &mut self.migration_requirements {
            req.prepared = true;
        }
        self.transition(PublicationSafetyLifecycleState::MigrationPrepared)
    }

    pub fn prepare_rollback(&mut self) -> Result<(), ActionProposalError> {
        self.transition(PublicationSafetyLifecycleState::RollbackPrepared)?;
        for req in &mut self.rollback_requirements {
            req.prepared = true;
            req.history_preserved = true;
        }
        Ok(())
    }

    /// Release approval for a future publish — still does not activate runtime.
    pub fn approve_release(&mut self) -> Result<(), ActionProposalError> {
        self.transition(PublicationSafetyLifecycleState::ReleaseApproved)?;
        assert_eq!(self.authority_effect, Self::AUTHORITY_EFFECT_NONE);
        Ok(())
    }

    pub fn rollback_preserves_history(&self) -> bool {
        self.failure_handling.preserves_history
            && self
                .rollback_requirements
                .iter()
                .all(|r| r.history_preserved)
            && !self.lifecycle_history.is_empty()
    }

    pub fn may_execute_commands(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn may_mutate_cognition(&self) -> bool {
        false
    }

    pub fn may_rewrite_provenance(&self) -> bool {
        false
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn attempt_publish_activate(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }

    pub fn attempt_bypass_gateway() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_mutate_cognition() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceRiskCannotMutateCognition)
    }

    pub fn attempt_rewrite_provenance() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::ChangeEvaluationCannotRewriteHistory)
    }
}

/// Failure category across governance / publication preparation (Sprint 150).
/// Pattern sources are audited separately — categories do not merge authority domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceFailureCategory {
    /// Pattern from MigrationRunner apply failures — block progression, keep prior state.
    MigrationApply,
    /// Pattern from command validation precondition failures.
    CommandValidation,
    /// Pattern from Permission Gateway Deny — never reinterpreted as Allow.
    PermissionDenial,
    /// Pattern from audit write/trail failures — never erase prior trail.
    AuditTrail,
    /// Pattern from review / policy expiry blocking further approval.
    ReviewExpiry,
    /// Pattern from publication safety validation gates.
    PublicationValidation,
    /// Pattern from explicit reject decisions — remain historically visible.
    ReviewRejection,
}

impl GovernanceFailureCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MigrationApply => "migration_apply",
            Self::CommandValidation => "command_validation",
            Self::PermissionDenial => "permission_denial",
            Self::AuditTrail => "audit_trail",
            Self::ReviewExpiry => "review_expiry",
            Self::PublicationValidation => "publication_validation",
            Self::ReviewRejection => "review_rejection",
        }
    }

    pub fn pattern_source(self) -> &'static str {
        match self {
            Self::MigrationApply => "MigrationRunner.apply_all failure",
            Self::CommandValidation => "Command validation preconditions",
            Self::PermissionDenial => "Permission Gateway Deny",
            Self::AuditTrail => "AuditService failure / append-only trail",
            Self::ReviewExpiry => "GovernancePolicyExpired / proposal expire",
            Self::PublicationValidation => "PublicationSafetyContract validation gates",
            Self::ReviewRejection => "GovernanceReviewDecision Reject",
        }
    }
}

/// Recovery lifecycle for governance failures (architecture only — Sprint 150).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceFailureRecoveryState {
    FailureDetected,
    Recorded,
    RecoveryPlanned,
    Recovered,
    Abandoned,
}

impl GovernanceFailureRecoveryState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailureDetected => "failure_detected",
            Self::Recorded => "recorded",
            Self::RecoveryPlanned => "recovery_planned",
            Self::Recovered => "recovered",
            Self::Abandoned => "abandoned",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::FailureDetected, Self::Recorded)
                | (Self::Recorded, Self::RecoveryPlanned)
                | (Self::RecoveryPlanned, Self::Recovered)
                | (Self::RecoveryPlanned, Self::Abandoned)
        )
    }

    /// Abandoned and Recovered remain historically visible; never erased.
    pub fn remains_visible(self) -> bool {
        matches!(
            self,
            Self::FailureDetected
                | Self::Recorded
                | Self::RecoveryPlanned
                | Self::Recovered
                | Self::Abandoned
        )
    }
}

/// Recovery requirements attached to a failure record (architecture — no execution).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceFailureRecoveryRequirement {
    pub description: String,
    pub preserves_provenance: bool,
    pub preserves_evidence: bool,
    pub grants_authority: bool,
    pub activates_runtime: bool,
    pub bypasses_gateway: bool,
}

impl GovernanceFailureRecoveryRequirement {
    pub fn default_safe(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            preserves_provenance: true,
            preserves_evidence: true,
            grants_authority: false,
            activates_runtime: false,
            bypasses_gateway: false,
        }
    }

    pub fn is_safe(&self) -> bool {
        self.preserves_provenance
            && self.preserves_evidence
            && !self.grants_authority
            && !self.activates_runtime
            && !self.bypasses_gateway
    }
}

/// Failure handling across governance and publication preparation (Sprint 150).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceFailureState {
    pub id: String,
    pub category: GovernanceFailureCategory,
    pub lifecycle_stage: GovernanceLifecycleStage,
    pub actors: GovernanceActorRefs,
    pub recovery_requirements: Vec<GovernanceFailureRecoveryRequirement>,
    pub preserved_evidence_references: Vec<String>,
    pub recovery_state: GovernanceFailureRecoveryState,
    pub recovery_history: Vec<GovernanceFailureRecoveryState>,
    pub provenance: RecommendationProvenance,
    /// Abandoned / rejected / failed paths remain visible in history.
    pub historically_visible: bool,
    pub detail: String,
    pub authority_effect: String,
}

impl GovernanceFailureState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn detect(
        category: GovernanceFailureCategory,
        lifecycle_stage: GovernanceLifecycleStage,
        actors: GovernanceActorRefs,
        preserved_evidence_references: Vec<String>,
        provenance: RecommendationProvenance,
        detail: impl Into<String>,
    ) -> Self {
        let detail = detail.into();
        Self {
            id: format!(
                "governance_failure:{}:{}",
                category.as_str(),
                lifecycle_stage.as_str()
            ),
            category,
            lifecycle_stage,
            actors,
            recovery_requirements: vec![GovernanceFailureRecoveryRequirement::default_safe(
                format!("recover from {}", category.as_str()),
            )],
            preserved_evidence_references,
            recovery_state: GovernanceFailureRecoveryState::FailureDetected,
            recovery_history: vec![GovernanceFailureRecoveryState::FailureDetected],
            provenance,
            historically_visible: true,
            detail,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Failed review reject — history and evidence retained.
    pub fn from_failed_review(
        reject_decision: &GovernanceReviewDecision,
        evidence: &GovernanceDecisionEvidence,
        actors: GovernanceActorRefs,
    ) -> Result<Self, ActionProposalError> {
        if reject_decision.decision != GovernanceReviewDecisionKind::Reject {
            return Err(ActionProposalError::GovernanceFailureRecoveryBlocked(
                "expected reject decision for failed review failure state".into(),
            ));
        }
        let mut refs = vec![evidence.id.clone()];
        if let Some(eref) = &reject_decision.evidence_reference {
            if !refs.contains(eref) {
                refs.push(eref.clone());
            }
        }
        Ok(Self::detect(
            GovernanceFailureCategory::ReviewRejection,
            GovernanceLifecycleStage::RejectedVisible,
            actors,
            refs,
            reject_decision.provenance_snapshot.clone(),
            format!("review rejected: {}", reject_decision.id),
        ))
    }

    /// Failed publication validation — evidence and readiness history retained.
    pub fn from_publication_validation_failure(
        safety: &PublicationSafetyContract,
        evidence_reference: impl Into<String>,
        actors: GovernanceActorRefs,
    ) -> Self {
        Self::detect(
            GovernanceFailureCategory::PublicationValidation,
            GovernanceLifecycleStage::PublicationReadinessReached,
            actors,
            vec![evidence_reference.into(), safety.readiness_reference.clone()],
            safety.provenance.clone(),
            format!(
                "publication validation failed at {:?}",
                safety.lifecycle_state
            ),
        )
    }

    /// Review / policy expiry — blocks approval; never grants authority.
    pub fn from_review_expiry(
        proposal: &OutcomeAdaptationProposal,
        evidence_references: Vec<String>,
        actors: GovernanceActorRefs,
    ) -> Self {
        Self::detect(
            GovernanceFailureCategory::ReviewExpiry,
            GovernanceLifecycleStage::Review,
            actors,
            evidence_references,
            proposal.provenance.clone(),
            format!("review expired for proposal {}", proposal.id),
        )
    }

    fn transition(
        &mut self,
        to: GovernanceFailureRecoveryState,
    ) -> Result<(), ActionProposalError> {
        if !self.recovery_state.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.recovery_state.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.recovery_state = to;
        self.recovery_history.push(to);
        self.historically_visible = true;
        Ok(())
    }

    pub fn record(&mut self) -> Result<(), ActionProposalError> {
        self.transition(GovernanceFailureRecoveryState::Recorded)?;
        assert!(!self.preserved_evidence_references.is_empty() || self.category
            == GovernanceFailureCategory::PermissionDenial);
        Ok(())
    }

    pub fn plan_recovery(
        &mut self,
        requirement: GovernanceFailureRecoveryRequirement,
    ) -> Result<(), ActionProposalError> {
        if self.recovery_state == GovernanceFailureRecoveryState::FailureDetected {
            self.record()?;
        }
        if !requirement.is_safe() {
            return Err(ActionProposalError::GovernanceFailureRecoveryBlocked(
                "recovery requirement would violate safety invariants".into(),
            ));
        }
        self.recovery_requirements.push(requirement);
        self.transition(GovernanceFailureRecoveryState::RecoveryPlanned)
    }

    pub fn mark_recovered(&mut self) -> Result<(), ActionProposalError> {
        self.transition(GovernanceFailureRecoveryState::Recovered)?;
        assert_eq!(self.authority_effect, Self::AUTHORITY_EFFECT_NONE);
        Ok(())
    }

    /// Abandoned changes remain historically visible — never deleted.
    pub fn abandon(&mut self) -> Result<(), ActionProposalError> {
        self.transition(GovernanceFailureRecoveryState::Abandoned)?;
        self.historically_visible = true;
        Ok(())
    }

    pub fn preserves_provenance(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance == expected
            && self
                .recovery_requirements
                .iter()
                .all(|r| r.preserves_provenance)
    }

    pub fn preserves_evidence(&self) -> bool {
        !self.preserved_evidence_references.is_empty()
            && self
                .recovery_requirements
                .iter()
                .all(|r| r.preserves_evidence)
    }

    pub fn abandoned_remains_visible(&self) -> bool {
        self.historically_visible
            && self
                .recovery_history
                .iter()
                .any(|s| *s == GovernanceFailureRecoveryState::Abandoned)
            && self.recovery_state.remains_visible()
    }

    pub fn may_delete_provenance(&self) -> bool {
        false
    }

    pub fn may_grant_authority(&self) -> bool {
        false
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_delete_provenance(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceFailureCannotDeleteProvenance)
    }

    pub fn attempt_grant_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_activate_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_bypass_gateway() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    /// Recovery planning never executes commands or activates runtime.
    pub fn attempt_recovery_execute(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceFailureRecoveryBlocked(
            "recovery cannot execute".into(),
        ))
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Stages of the end-to-end governance lifecycle (Sprint 148).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceLifecycleStage {
    ChangeProposal,
    RiskClassification,
    EvidenceCollection,
    Review,
    Decision,
    GovernanceRecordPersisted,
    WorkspacePresentation,
    PublicationReadinessReached,
    RejectedVisible,
}

impl GovernanceLifecycleStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChangeProposal => "change_proposal",
            Self::RiskClassification => "risk_classification",
            Self::EvidenceCollection => "evidence_collection",
            Self::Review => "review",
            Self::Decision => "decision",
            Self::GovernanceRecordPersisted => "governance_record",
            Self::WorkspacePresentation => "workspace_presentation",
            Self::PublicationReadinessReached => "publication_readiness",
            Self::RejectedVisible => "rejected_visible",
        }
    }
}

/// Immutable governance timeline event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceTimelineEvent {
    pub id: String,
    pub stage: GovernanceLifecycleStage,
    pub at: String,
    pub actor_reference: Option<String>,
    pub evidence_reference: Option<String>,
    pub decision_reference: Option<String>,
    pub note: Option<String>,
}

/// End-to-end governance timeline — append-only events; never grants authority (Sprint 148).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceTimeline {
    pub id: String,
    pub events: Vec<GovernanceTimelineEvent>,
    pub provenance: RecommendationProvenance,
    /// Rejected changes remain historically visible on the timeline.
    pub rejected_visible: bool,
    /// Dissent ids captured immutably (never erased).
    pub dissent_immutable_refs: Vec<String>,
    pub authority_effect: String,
}

impl GovernanceTimeline {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn begin(proposal: &OutcomeAdaptationProposal, at: impl Into<String>) -> Self {
        let at = at.into();
        let mut timeline = Self {
            id: format!("governance_timeline:{}", proposal.id),
            events: Vec::new(),
            provenance: proposal.provenance.clone(),
            rejected_visible: false,
            dissent_immutable_refs: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        };
        timeline.append_event(
            GovernanceLifecycleStage::ChangeProposal,
            at,
            Some(proposal.proposed_by_actor_id.clone()),
            None,
            None,
            Some(format!("proposal:{}", proposal.id)),
        );
        timeline
    }

    fn append_event(
        &mut self,
        stage: GovernanceLifecycleStage,
        at: impl Into<String>,
        actor_reference: Option<String>,
        evidence_reference: Option<String>,
        decision_reference: Option<String>,
        note: Option<String>,
    ) {
        let idx = self.events.len();
        self.events.push(GovernanceTimelineEvent {
            id: format!("{}:{}:{}", self.id, stage.as_str(), idx),
            stage,
            at: at.into(),
            actor_reference,
            evidence_reference,
            decision_reference,
            note,
        });
    }

    /// Build and validate a complete lifecycle timeline from governance artifacts.
    pub fn from_full_lifecycle(
        proposal: &OutcomeAdaptationProposal,
        risk: &GovernanceRisk,
        evidence: &GovernanceDecisionEvidence,
        decisions: &[GovernanceReviewDecision],
        record: &GovernanceRecord,
        workspace: &GovernanceWorkspace,
        readiness: &PublicationReadiness,
        at_prefix: &str,
    ) -> Result<Self, ActionProposalError> {
        let mut timeline = Self::begin(proposal, format!("{at_prefix}:proposal"));
        timeline.append_event(
            GovernanceLifecycleStage::RiskClassification,
            format!("{at_prefix}:risk"),
            None,
            None,
            None,
            Some(risk.id.clone()),
        );
        timeline.append_event(
            GovernanceLifecycleStage::EvidenceCollection,
            format!("{at_prefix}:evidence"),
            None,
            Some(evidence.id.clone()),
            None,
            Some(format!("refs:{}", evidence.supporting_evidence_references.len())),
        );
        timeline.append_event(
            GovernanceLifecycleStage::Review,
            format!("{at_prefix}:review"),
            record.actors.reviewer_actor_id.clone(),
            Some(evidence.id.clone()),
            None,
            Some("review_submitted".into()),
        );
        for (i, decision) in decisions.iter().enumerate() {
            timeline.append_event(
                GovernanceLifecycleStage::Decision,
                format!("{at_prefix}:decision:{i}"),
                Some(decision.reviewer.actor_id.clone()),
                decision.evidence_reference.clone(),
                Some(decision.id.clone()),
                Some(decision.decision.as_str().into()),
            );
            if decision.decision == GovernanceReviewDecisionKind::Reject {
                timeline.rejected_visible = true;
                timeline.append_event(
                    GovernanceLifecycleStage::RejectedVisible,
                    format!("{at_prefix}:rejected:{i}"),
                    Some(decision.reviewer.actor_id.clone()),
                    decision.evidence_reference.clone(),
                    Some(decision.id.clone()),
                    Some("rejected_remains_visible".into()),
                );
            }
        }
        timeline.dissent_immutable_refs = evidence
            .dissent_records
            .iter()
            .map(|d| d.id.clone())
            .collect();
        timeline.append_event(
            GovernanceLifecycleStage::GovernanceRecordPersisted,
            format!("{at_prefix}:record"),
            record.actors.reviewer_actor_id.clone(),
            record.evidence_reference.clone(),
            record.review_decision_references.first().cloned(),
            Some(record.id.clone()),
        );
        timeline.append_event(
            GovernanceLifecycleStage::WorkspacePresentation,
            format!("{at_prefix}:workspace"),
            None,
            workspace.evidence_view.evidence_reference.clone(),
            workspace
                .decision_history
                .first()
                .map(|d| d.decision_reference.clone()),
            Some(workspace.id.clone()),
        );
        timeline.append_event(
            GovernanceLifecycleStage::PublicationReadinessReached,
            format!("{at_prefix}:readiness"),
            None,
            Some(readiness.evidence_reference.clone()),
            None,
            Some(readiness.state.as_str().into()),
        );
        timeline.validate_lifecycle_integrity()?;
        if !timeline.provenance_survives(&proposal.provenance)
            || !workspace.preserves_provenance(&proposal.provenance)
            || record.provenance != proposal.provenance
        {
            return Err(ActionProposalError::GovernanceLifecycleIntegrityFailed(
                "provenance broken across lifecycle".into(),
            ));
        }
        Ok(timeline)
    }

    /// Rejected-only path: proposal → risk → evidence → reject decision → visible history.
    pub fn from_rejected_lifecycle(
        proposal: &OutcomeAdaptationProposal,
        risk: &GovernanceRisk,
        evidence: &GovernanceDecisionEvidence,
        reject_decision: &GovernanceReviewDecision,
        at_prefix: &str,
    ) -> Result<Self, ActionProposalError> {
        if reject_decision.decision != GovernanceReviewDecisionKind::Reject {
            return Err(ActionProposalError::GovernanceLifecycleIntegrityFailed(
                "expected reject decision".into(),
            ));
        }
        let mut timeline = Self::begin(proposal, format!("{at_prefix}:proposal"));
        timeline.append_event(
            GovernanceLifecycleStage::RiskClassification,
            format!("{at_prefix}:risk"),
            None,
            None,
            None,
            Some(risk.id.clone()),
        );
        timeline.append_event(
            GovernanceLifecycleStage::EvidenceCollection,
            format!("{at_prefix}:evidence"),
            None,
            Some(evidence.id.clone()),
            None,
            None,
        );
        timeline.append_event(
            GovernanceLifecycleStage::Review,
            format!("{at_prefix}:review"),
            Some(reject_decision.reviewer.actor_id.clone()),
            Some(evidence.id.clone()),
            None,
            None,
        );
        timeline.append_event(
            GovernanceLifecycleStage::Decision,
            format!("{at_prefix}:decision"),
            Some(reject_decision.reviewer.actor_id.clone()),
            reject_decision.evidence_reference.clone(),
            Some(reject_decision.id.clone()),
            Some("reject".into()),
        );
        timeline.rejected_visible = true;
        timeline.append_event(
            GovernanceLifecycleStage::RejectedVisible,
            format!("{at_prefix}:rejected_visible"),
            Some(reject_decision.reviewer.actor_id.clone()),
            reject_decision.evidence_reference.clone(),
            Some(reject_decision.id.clone()),
            Some("historically_visible".into()),
        );
        timeline.dissent_immutable_refs = evidence
            .dissent_records
            .iter()
            .map(|d| d.id.clone())
            .collect();
        if !timeline.provenance_survives(&proposal.provenance) {
            return Err(ActionProposalError::GovernanceLifecycleIntegrityFailed(
                "provenance broken on rejected path".into(),
            ));
        }
        Ok(timeline)
    }

    pub fn validate_lifecycle_integrity(&self) -> Result<(), ActionProposalError> {
        let required = [
            GovernanceLifecycleStage::ChangeProposal,
            GovernanceLifecycleStage::RiskClassification,
            GovernanceLifecycleStage::EvidenceCollection,
            GovernanceLifecycleStage::Review,
            GovernanceLifecycleStage::Decision,
            GovernanceLifecycleStage::GovernanceRecordPersisted,
            GovernanceLifecycleStage::WorkspacePresentation,
            GovernanceLifecycleStage::PublicationReadinessReached,
        ];
        for stage in required {
            if !self.events.iter().any(|e| e.stage == stage) {
                return Err(ActionProposalError::GovernanceLifecycleIntegrityFailed(
                    format!("missing stage {}", stage.as_str()),
                ));
            }
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(ActionProposalError::GovernanceLifecycleIntegrityFailed(
                "authority_effect must be none".into(),
            ));
        }
        Ok(())
    }

    pub fn provenance_survives(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance == expected
    }

    pub fn stages(&self) -> Vec<GovernanceLifecycleStage> {
        self.events.iter().map(|e| e.stage).collect()
    }

    pub fn may_rewrite_events(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn attempt_rewrite_event(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceTimelineImmutable)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }

    pub fn attempt_activate_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationReadinessCannotActivate)
    }
}

/// Explicit human review decision — cannot bypass provenance or execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewDecision {
    pub id: String,
    pub policy_id: String,
    pub reviewer: AdaptationReviewerIdentity,
    pub decision: GovernanceReviewDecisionKind,
    pub rationale: String,
    pub timestamp: String,
    pub conditions: Vec<String>,
    /// Sprint 146 — optional link to assembled decision evidence.
    pub evidence_reference: Option<String>,
    pub provenance_snapshot: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceReviewDecision {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(
        policy: &GovernancePolicy,
        reviewer: AdaptationReviewerIdentity,
        decision: GovernanceReviewDecisionKind,
        rationale: impl Into<String>,
        timestamp: impl Into<String>,
        conditions: Vec<String>,
        provenance: &RecommendationProvenance,
    ) -> Result<Self, ActionProposalError> {
        let rationale = rationale.into();
        if policy.reviewer_requirements.require_rationale && rationale.trim().is_empty() {
            return Err(ActionProposalError::GovernanceReviewRationaleRequired);
        }
        if !policy.reviewer_satisfies(&reviewer) {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        Ok(Self {
            id: format!(
                "gov_review:{}:{}:{}",
                policy.id,
                reviewer.actor_id,
                decision.as_str()
            ),
            policy_id: policy.id.clone(),
            reviewer,
            decision,
            rationale,
            timestamp: timestamp.into(),
            conditions,
            evidence_reference: None,
            provenance_snapshot: provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn with_evidence(mut self, evidence: &GovernanceDecisionEvidence) -> Self {
        self.evidence_reference = Some(evidence.id.clone());
        self
    }

    pub fn may_bypass_provenance(&self) -> bool {
        false
    }

    pub fn may_rewrite_provenance(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn retains_provenance(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance_snapshot == expected
    }

    pub fn attempt_bypass_provenance() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::ReviewerCannotBypassProvenance)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Timestamps for the governance chain (architecture ledger).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceTimestamps {
    pub proposed_at: Option<String>,
    pub reviewed_at: Option<String>,
    pub approved_at: Option<String>,
    pub changed_at: Option<String>,
    pub evaluated_at: Option<String>,
    pub publish_requested_at: Option<String>,
    /// Always `None` until a future activation sprint — never set by Sprint 143.
    pub published_at: Option<String>,
}

/// Complete historical governance traceability record (Sprint 143).
///
/// Links proposal → review → approval → change → version → evaluation → rollback.
/// Never grants execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRecord {
    pub id: String,
    pub proposal_reference: String,
    pub review_reference: Option<String>,
    pub approval_reference: Option<String>,
    pub change_reference: Option<String>,
    pub version_reference: Option<String>,
    pub evaluation_reference: Option<String>,
    pub rollback_reference: Option<String>,
    /// Sprint 144 — policy that governed this ledger entry.
    pub policy_reference: Option<String>,
    /// Sprint 144 — review decision ids applied under policy.
    pub review_decision_references: Vec<String>,
    /// Sprint 146 — decision evidence package.
    pub evidence_reference: Option<String>,
    /// Sprint 146 — dissent ids (append-only; never erase approvals).
    pub dissent_references: Vec<String>,
    /// Sprint 146 — publication readiness state label.
    pub publication_readiness: Option<PublicationReadinessState>,
    pub actors: GovernanceActorRefs,
    pub timestamps: GovernanceTimestamps,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_adaptation_chain(
        proposal: &OutcomeAdaptationProposal,
        surface: Option<&ControlledChangeSurface>,
        version: Option<&BehaviourVersion>,
        evaluation: Option<&ChangeEvaluation>,
        rollback_version: Option<&BehaviourVersion>,
    ) -> Self {
        let approved_at = proposal.decided_at.clone();
        let review_reference = proposal
            .audit_events
            .iter()
            .find(|e| e.action == "submitted_for_review")
            .map(|e| format!("review:{}:{}", proposal.id, e.at));
        let approval_reference = if proposal.review_status.is_approved() {
            Some(format!("approval:{}", proposal.id))
        } else {
            None
        };
        Self {
            id: format!("governance:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            review_reference,
            approval_reference,
            change_reference: surface.map(|s| s.originating_proposal_id.clone()),
            version_reference: version.map(|v| v.id.clone()),
            evaluation_reference: evaluation.map(|e| e.id.clone()),
            rollback_reference: rollback_version.map(|v| v.id.clone()),
            policy_reference: None,
            review_decision_references: Vec::new(),
            evidence_reference: None,
            dissent_references: Vec::new(),
            publication_readiness: None,
            actors: GovernanceActorRefs {
                proposer_actor_id: proposal.proposed_by_actor_id.clone(),
                reviewer_actor_id: proposal.reviewer.as_ref().map(|r| r.actor_id.clone()),
                publisher_actor_id: None,
            },
            timestamps: GovernanceTimestamps {
                proposed_at: proposal
                    .audit_events
                    .first()
                    .map(|e| e.at.clone())
                    .or_else(|| Some("proposed".into())),
                reviewed_at: proposal
                    .audit_events
                    .iter()
                    .find(|e| e.action == "submitted_for_review")
                    .map(|e| e.at.clone()),
                approved_at: approved_at.clone(),
                changed_at: surface.and_then(|s| s.approved_at.clone()),
                evaluated_at: evaluation.map(|_| {
                    approved_at
                        .clone()
                        .unwrap_or_else(|| "evaluated".into())
                }),
                publish_requested_at: None,
                published_at: None,
            },
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Attach policy + review decisions: Policy → Review Decision → Ledger → PublishRequest.
    pub fn with_policy_and_decisions(
        mut self,
        policy: &GovernancePolicy,
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
    ) -> Result<Self, ActionProposalError> {
        policy.enforce_approval_requirements(proposal, decisions)?;
        for decision in decisions {
            if !decision.retains_provenance(&self.provenance) {
                return Err(ActionProposalError::ReviewerCannotBypassProvenance);
            }
        }
        self.policy_reference = Some(policy.id.clone());
        self.review_decision_references = decisions.iter().map(|d| d.id.clone()).collect();
        if let Some(last) = decisions.last() {
            self.actors.reviewer_actor_id = Some(last.reviewer.actor_id.clone());
            self.timestamps.reviewed_at = Some(last.timestamp.clone());
            if decisions
                .iter()
                .any(|d| d.decision == GovernanceReviewDecisionKind::Approve)
            {
                self.timestamps.approved_at = Some(last.timestamp.clone());
                if self.approval_reference.is_none() {
                    self.approval_reference = Some(format!("approval:policy:{}", policy.id));
                }
            }
        }
        Ok(self)
    }

    /// Risk → Evidence → Review Decision → Ledger (Sprint 146).
    pub fn with_evidence_and_readiness(
        mut self,
        evidence: &GovernanceDecisionEvidence,
        readiness: &PublicationReadiness,
        decisions: &[GovernanceReviewDecision],
    ) -> Result<Self, ActionProposalError> {
        evidence.validate_for_approval()?;
        if readiness.evidence_reference != evidence.id {
            return Err(ActionProposalError::GovernanceDecisionEvidenceRequired);
        }
        for decision in decisions {
            if decision.decision == GovernanceReviewDecisionKind::Approve
                && decision.evidence_reference.as_deref() != Some(evidence.id.as_str())
            {
                return Err(ActionProposalError::GovernanceDecisionEvidenceRequired);
            }
            if !decision.retains_provenance(&self.provenance) {
                return Err(ActionProposalError::ReviewerCannotBypassProvenance);
            }
        }
        let prior_approval = self.approval_reference.clone();
        self.evidence_reference = Some(evidence.id.clone());
        self.dissent_references = evidence.dissent_records.iter().map(|d| d.id.clone()).collect();
        self.publication_readiness = Some(readiness.state);
        // Dissent must not erase prior approval history on the ledger.
        if prior_approval.is_some() {
            self.approval_reference = prior_approval;
        }
        Ok(self)
    }

    pub fn with_publish_request(mut self, request: &PublishRequest, at: impl Into<String>) -> Self {
        self.timestamps.publish_requested_at = Some(at.into());
        self.actors.publisher_actor_id = Some(request.requested_by.actor_id.clone());
        self
    }

    pub fn retains_provenance(&self, expected: &RecommendationProvenance) -> bool {
        &self.provenance == expected
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_grant_execution_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Status of an architecture-only publish request (Sprint 143).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishRequestStatus {
    Proposed,
    AwaitingGovernanceReview,
    ApprovedForPublish,
    Rejected,
    /// Label only — does not activate runtime behaviour.
    PublishedRecorded,
}

impl PublishRequestStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::AwaitingGovernanceReview => "awaiting_governance_review",
            Self::ApprovedForPublish => "approved_for_publish",
            Self::Rejected => "rejected",
            Self::PublishedRecorded => "published_recorded",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Proposed, Self::AwaitingGovernanceReview)
                | (Self::Proposed, Self::Rejected)
                | (Self::AwaitingGovernanceReview, Self::ApprovedForPublish)
                | (Self::AwaitingGovernanceReview, Self::Rejected)
        )
    }
}

/// Architecture: Evaluated BehaviourVersion → PublishRequest → Governance Review → Published.
///
/// Never activates runtime cognition or grants Gateway authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishRequest {
    pub id: String,
    pub behaviour_version_id: String,
    pub evaluation_reference: String,
    pub governance_record_id: String,
    pub status: PublishRequestStatus,
    pub requested_by: AdaptationReviewerIdentity,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl PublishRequest {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Build a publish request only when governance shows approval and an evaluation exists.
    pub fn from_evaluated_version(
        version: &BehaviourVersion,
        evaluation: &ChangeEvaluation,
        governance: &GovernanceRecord,
        requested_by: AdaptationReviewerIdentity,
    ) -> Result<Self, ActionProposalError> {
        if governance.approval_reference.is_none() {
            return Err(ActionProposalError::PublicationRequiresApproval);
        }
        if governance.evaluation_reference.as_deref() != Some(evaluation.id.as_str()) {
            return Err(ActionProposalError::PublicationRequiresApproval);
        }
        if evaluation.change_reference != version.id {
            return Err(ActionProposalError::PublicationRequiresApproval);
        }
        if !requested_by.is_local_user() {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        if version.lifecycle != BehaviourVersionLifecycle::Draft {
            return Err(ActionProposalError::BehaviourVersionPublishNotImplemented);
        }
        Ok(Self {
            id: format!("publish_request:{}", version.id),
            behaviour_version_id: version.id.clone(),
            evaluation_reference: evaluation.id.clone(),
            governance_record_id: governance.id.clone(),
            status: PublishRequestStatus::Proposed,
            requested_by,
            provenance: version.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Policy → Review Decision → GovernanceRecord → PublishRequest.
    pub fn from_evaluated_version_under_policy(
        version: &BehaviourVersion,
        evaluation: &ChangeEvaluation,
        governance: &GovernanceRecord,
        policy: &GovernancePolicy,
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
        requested_by: AdaptationReviewerIdentity,
    ) -> Result<Self, ActionProposalError> {
        if governance.policy_reference.as_deref() != Some(policy.id.as_str()) {
            return Err(ActionProposalError::GovernancePolicyRequirementsNotMet);
        }
        policy.enforce_approval_requirements(proposal, decisions)?;
        Self::from_evaluated_version(version, evaluation, governance, requested_by)
    }

    pub fn submit_for_governance_review(&mut self) -> Result<(), ActionProposalError> {
        self.transition(PublishRequestStatus::AwaitingGovernanceReview)
    }

    pub fn approve_for_publish(
        &mut self,
        reviewer: &AdaptationReviewerIdentity,
    ) -> Result<(), ActionProposalError> {
        if !reviewer.is_local_user() {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        self.transition(PublishRequestStatus::ApprovedForPublish)?;
        assert_eq!(self.authority_effect, Self::AUTHORITY_EFFECT_NONE);
        Ok(())
    }

    pub fn approve_for_publish_under_policy(
        &mut self,
        policy: &GovernancePolicy,
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
        reviewer: &AdaptationReviewerIdentity,
    ) -> Result<(), ActionProposalError> {
        policy.enforce_approval_requirements(proposal, decisions)?;
        self.approve_for_publish(reviewer)
    }

    pub fn reject_publish(
        &mut self,
        reviewer: &AdaptationReviewerIdentity,
    ) -> Result<(), ActionProposalError> {
        if !reviewer.is_local_user() {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        self.transition(PublishRequestStatus::Rejected)
    }

    fn transition(&mut self, to: PublishRequestStatus) -> Result<(), ActionProposalError> {
        if !self.status.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.status.as_str().into(),
                to: to.as_str().into(),
            });
        }
        // PublishedRecorded / activation is never reachable via transition.
        if to == PublishRequestStatus::PublishedRecorded {
            return Err(ActionProposalError::PublicationActivationNotImplemented);
        }
        self.status = to;
        Ok(())
    }

    /// Hard-fail — publication does not activate runtime behaviour in Sprint 143.
    pub fn attempt_activate_published_version(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_record_published_without_approval() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationRequiresApproval)
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_grant_execution_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Architecture-only published version marker — cannot be created in Sprint 143.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedVersionRecord {
    pub id: String,
    pub behaviour_version_id: String,
    pub publish_request_id: String,
    pub governance_record_id: String,
    pub authority_effect: String,
}

impl PublishedVersionRecord {
    pub fn attempt_from_publish_request(
        _request: &PublishRequest,
    ) -> Result<Self, ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn may_grant_execution_authority(&self) -> bool {
        false
    }
}

/// Future boundary: Approved Adaptation → Controlled Change Surface → Versioned Behaviour.
///
/// Architecture only — never mutates runtime cognition. Sprint 142 expands required fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlledChangeSurface {
    /// Originating `OutcomeAdaptationProposal` id.
    pub originating_proposal_id: String,
    /// Sprint 141 alias field (same as originating_proposal_id).
    pub adaptation_proposal_id: String,
    pub approved_change: String,
    pub reviewer: AdaptationReviewerIdentity,
    pub previous_version_id: String,
    pub target_version_id: String,
    pub rollback: ChangeRollbackMetadata,
    pub audit: ControlledChangeAuditMetadata,
    /// Frozen provenance — never rewritten by evaluation or rollback drafts.
    pub provenance: RecommendationProvenance,
    pub approved_at: Option<String>,
    /// Legacy Sprint 141 label (equals target_version_id).
    pub version_target: String,
    pub authority_effect: String,
}

impl ControlledChangeSurface {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const VERSION_TARGET_FUTURE: &'static str = "future_behaviour_version";

    pub fn from_approved_proposal(
        proposal: &OutcomeAdaptationProposal,
    ) -> Result<Self, ActionProposalError> {
        if !proposal.review_status.is_approved() {
            return Err(ActionProposalError::UnapprovedControlledChangeForbidden);
        }
        if proposal.review_status.is_terminal_without_apply() {
            return Err(ActionProposalError::RejectedAdaptationCannotApply);
        }
        let reviewer = proposal
            .reviewer
            .clone()
            .ok_or(ActionProposalError::AdaptationReviewerRequired)?;
        let previous_version_id = BehaviourVersion::BASELINE_ID.to_string();
        let target_version_id = format!("behaviour:from:{}", proposal.id);
        let approved_at = proposal.decided_at.clone().unwrap_or_else(|| "approved".into());
        Ok(Self {
            originating_proposal_id: proposal.id.clone(),
            adaptation_proposal_id: proposal.id.clone(),
            approved_change: proposal.proposed_change.clone(),
            reviewer,
            previous_version_id: previous_version_id.clone(),
            target_version_id: target_version_id.clone(),
            rollback: ChangeRollbackMetadata::for_previous(previous_version_id),
            audit: ControlledChangeAuditMetadata {
                created_at: approved_at.clone(),
                approval_reference: proposal.id.clone(),
                review_audit_actions: proposal
                    .audit_events
                    .iter()
                    .map(|e| e.action.clone())
                    .collect(),
            },
            provenance: proposal.provenance.clone(),
            approved_at: Some(approved_at),
            version_target: target_version_id,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Explicit guard — unapproved proposals never yield a surface.
    pub fn from_unapproved_proposal(
        proposal: &OutcomeAdaptationProposal,
    ) -> Result<Self, ActionProposalError> {
        if proposal.review_status.is_approved() {
            return Self::from_approved_proposal(proposal);
        }
        Err(ActionProposalError::UnapprovedControlledChangeForbidden)
    }

    pub fn to_behaviour_version_draft(&self) -> Result<BehaviourVersion, ActionProposalError> {
        BehaviourVersion::draft_from_surface(self)
    }

    pub fn may_mutate_runtime_cognition(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_apply() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::AdaptationApplyNotImplemented)
    }

    pub fn attempt_mutate_runtime_cognition() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

/// Architecture: Adaptation Proposal driven by `RecommendationOutcome`.
///
/// Distinct from workspace `AdaptationProposal` (Pattern/OS aggregation).
/// Requires explicit review. Never auto-applies, never mutates scoring, never executes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeAdaptationProposal {
    pub id: String,
    pub source_outcome_id: String,
    pub source_recommendation_id: String,
    /// Frozen provenance from the source outcome.
    pub provenance: RecommendationProvenance,
    pub affected_area: String,
    pub proposed_change: String,
    pub expected_effect: String,
    pub confidence: Option<String>,
    /// Always true for outcome-backed proposals.
    pub review_required: bool,
    pub review_status: OutcomeAdaptationReviewStatus,
    /// Proposer identity — cannot approve its own proposal.
    pub proposed_by_actor_id: String,
    pub proposed_by_actor_type: String,
    pub reviewer: Option<AdaptationReviewerIdentity>,
    pub decided_at: Option<String>,
    pub expires_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub audit_events: Vec<AdaptationReviewAuditEvent>,
    pub experience_trace_match_keys: Vec<String>,
    pub authority_effect: String,
}

impl OutcomeAdaptationProposal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const PROPOSER_ACTOR_ID: &'static str = "system:outcome_adaptation";
    pub const PROPOSER_ACTOR_TYPE: &'static str = "system";

    pub fn from_outcome(
        outcome: &RecommendationOutcome,
        affected_area: impl Into<String>,
        proposed_change: impl Into<String>,
        expected_effect: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("adaptation_from_outcome:{}", outcome.id),
            source_outcome_id: outcome.id.clone(),
            source_recommendation_id: outcome.identity.native_id.clone(),
            provenance: outcome.provenance.clone(),
            affected_area: affected_area.into(),
            proposed_change: proposed_change.into(),
            expected_effect: expected_effect.into(),
            confidence: outcome.quality.confidence_at_outcome.clone(),
            review_required: true,
            review_status: OutcomeAdaptationReviewStatus::Proposed,
            proposed_by_actor_id: Self::PROPOSER_ACTOR_ID.into(),
            proposed_by_actor_type: Self::PROPOSER_ACTOR_TYPE.into(),
            reviewer: None,
            decided_at: None,
            expires_at: None,
            rejection_reason: None,
            audit_events: Vec::new(),
            experience_trace_match_keys: outcome.experience_trace_match_keys.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_expiry(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    pub fn require_review(&mut self) -> Result<(), ActionProposalError> {
        self.submit_for_review("review_requested", None)
    }

    pub fn submit_for_review(
        &mut self,
        at: impl Into<String>,
        note: Option<String>,
    ) -> Result<(), ActionProposalError> {
        if !self.review_required {
            self.review_required = true;
        }
        self.transition_review(OutcomeAdaptationReviewStatus::AwaitingReview)?;
        self.push_audit(at, "submitted_for_review", Self::PROPOSER_ACTOR_ID, note);
        Ok(())
    }

    pub fn transition_review(
        &mut self,
        to: OutcomeAdaptationReviewStatus,
    ) -> Result<(), ActionProposalError> {
        if matches!(
            to,
            OutcomeAdaptationReviewStatus::Applied | OutcomeAdaptationReviewStatus::Evaluated
        ) {
            return Err(ActionProposalError::AdaptationApplyNotImplemented);
        }
        if !self.review_status.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.review_status.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.review_status = to;
        Ok(())
    }

    /// Approve only with an explicit local_user reviewer — not self, not Gateway grant.
    pub fn approve(
        &mut self,
        reviewer: AdaptationReviewerIdentity,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let at = at.into();
        self.ensure_not_expired()?;
        if !reviewer.is_local_user() {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        if self.would_be_self_approval(&reviewer) {
            return Err(ActionProposalError::AdaptationSelfApprovalForbidden);
        }
        if self.review_status == OutcomeAdaptationReviewStatus::Rejected {
            return Err(ActionProposalError::RejectedAdaptationCannotApply);
        }
        // Allow Approved from Pending Review directly or via Reviewed.
        if self.review_status == OutcomeAdaptationReviewStatus::AwaitingReview {
            self.transition_review(OutcomeAdaptationReviewStatus::Reviewed)?;
        }
        self.transition_review(OutcomeAdaptationReviewStatus::ApprovedForHandoff)?;
        self.reviewer = Some(reviewer.clone());
        self.decided_at = Some(at.clone());
        self.push_audit(at, "approved", &reviewer.actor_id, None);
        assert_eq!(self.authority_effect, Self::AUTHORITY_EFFECT_NONE);
        Ok(())
    }

    /// Sprint 140 helper — still requires a local_user reviewer identity.
    pub fn approve_for_handoff(&mut self) -> Result<(), ActionProposalError> {
        self.approve(AdaptationReviewerIdentity::local_user("local_user"), "approved")
    }

    pub fn reject(
        &mut self,
        reviewer: AdaptationReviewerIdentity,
        at: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let at = at.into();
        let reason = reason.into();
        self.ensure_not_expired()?;
        if !reviewer.is_local_user() {
            return Err(ActionProposalError::AdaptationReviewerRequired);
        }
        if self.would_be_self_approval(&reviewer) {
            return Err(ActionProposalError::AdaptationSelfApprovalForbidden);
        }
        self.transition_review(OutcomeAdaptationReviewStatus::Rejected)?;
        self.reviewer = Some(reviewer.clone());
        self.decided_at = Some(at.clone());
        self.rejection_reason = Some(reason.clone());
        self.push_audit(at, "rejected", &reviewer.actor_id, Some(reason));
        Ok(())
    }

    pub fn expire(&mut self, at: impl Into<String>) -> Result<(), ActionProposalError> {
        let at = at.into();
        self.transition_review(OutcomeAdaptationReviewStatus::Expired)?;
        self.decided_at = Some(at.clone());
        self.push_audit(at, "expired", Self::PROPOSER_ACTOR_ID, None);
        Ok(())
    }

    fn ensure_not_expired(&self) -> Result<(), ActionProposalError> {
        if self.review_status == OutcomeAdaptationReviewStatus::Expired {
            return Err(ActionProposalError::ExpiredAdaptationCannotProceed);
        }
        Ok(())
    }

    fn would_be_self_approval(&self, reviewer: &AdaptationReviewerIdentity) -> bool {
        reviewer.actor_id == self.proposed_by_actor_id
            || reviewer.actor_id == self.id
            || reviewer.actor_type == Self::PROPOSER_ACTOR_TYPE
    }

    fn push_audit(
        &mut self,
        at: impl Into<String>,
        action: impl Into<String>,
        actor_id: impl Into<String>,
        note: Option<String>,
    ) {
        self.audit_events.push(AdaptationReviewAuditEvent {
            at: at.into(),
            action: action.into(),
            actor_id: actor_id.into(),
            note,
        });
    }

    pub fn may_auto_apply(&self) -> bool {
        false
    }

    pub fn may_self_approve(&self) -> bool {
        false
    }

    pub fn may_execute_from_approval(&self) -> bool {
        false
    }

    pub fn may_mutate_cognition(&self) -> bool {
        false
    }

    pub fn may_silently_change_scoring(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn requires_explicit_handling(&self) -> bool {
        self.review_required
    }

    pub fn controlled_change_surface_ready(&self) -> bool {
        self.review_status.is_approved() && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    /// Hard-fail — adaptation proposals never apply or execute from this type.
    pub fn attempt_apply() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::AdaptationApplyNotImplemented)
    }

    pub fn attempt_apply_after_rejection(&self) -> Result<(), ActionProposalError> {
        if self.review_status == OutcomeAdaptationReviewStatus::Rejected {
            return Err(ActionProposalError::RejectedAdaptationCannotApply);
        }
        Err(ActionProposalError::AdaptationApplyNotImplemented)
    }

    pub fn attempt_mark_applied(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::AdaptationApplyNotImplemented)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }

    pub fn attempt_self_approve(&mut self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::AdaptationSelfApprovalForbidden)
    }
}

/// Risk metadata for a future ActionProposal (architecture only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionProposalRisk {
    pub level: String,
    pub summary: String,
    pub reversible: bool,
}

impl ActionProposalRisk {
    pub fn informational() -> Self {
        Self {
            level: "informational".into(),
            summary: "Proposal only — no execution authority.".into(),
            reversible: true,
        }
    }
}

/// Future bridge: recommendation + provenance → permission-controlled action request.
///
/// Creating an `ActionProposal` never launches, grants capabilities, or bypasses Gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionProposal {
    pub id: String,
    pub workspace_id: String,
    pub recommendation_ref: String,
    pub identity: RecommendationIdentity,
    pub provenance: RecommendationProvenance,
    /// Capability that would be requested in a future privileged command (not granted).
    pub requested_capability: Option<String>,
    /// Capability / approval requirements for future Gateway evaluation.
    pub permission_requirements: Vec<String>,
    pub risk: ActionProposalRisk,
    /// Always `"none"` until a future Allow — proposals are not authority.
    pub authority_effect: String,
}

impl ActionProposal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Architecture constructor — never authorizes execution.
    pub fn from_recommendation_item(
        workspace_id: impl Into<String>,
        item: &RecommendationItem,
    ) -> Self {
        let provenance = RecommendationProvenance::from_recommendation_item(item);
        let id = format!("action_proposal:{}", item.id);
        let identity =
            RecommendationIdentity::from_recommendation_item(item).with_action_proposal_ref(&id);
        Self {
            id,
            workspace_id: workspace_id.into(),
            recommendation_ref: item.id.clone(),
            identity,
            requested_capability: provenance.future_capability_target.clone(),
            permission_requirements: Vec::new(),
            risk: ActionProposalRisk::informational(),
            provenance,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_requested_capability(mut self, capability: impl Into<String>) -> Self {
        let capability = capability.into();
        self.requested_capability = Some(capability.clone());
        self.provenance.future_capability_target = Some(capability.clone());
        if !self.permission_requirements.contains(&capability) {
            self.permission_requirements.push(capability);
        }
        self
    }

    pub fn with_experience_trace_match_keys(mut self, keys: Vec<String>) -> Self {
        self.provenance = self.provenance.with_experience_trace_match_keys(keys);
        self
    }

    /// Hard-fail — ActionProposal is never an execution path.
    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_attention::{AttentionSignal, AttentionSourceType};
    use crate::workspace_recommendation::{RecommendationConfidence, RecommendationKind};

    fn sample_item() -> RecommendationItem {
        RecommendationItem {
            id: "recommendation:attention:blocked".into(),
            kind: RecommendationKind::ResolveBlocker,
            title: "Resolve blocker".into(),
            reason: "Attention flags blocked work".into(),
            evidence: vec![RecommendationEvidence {
                id: "ev1".into(),
                source_model: "attention".into(),
                source_ref: "attention:task:1".into(),
                summary: "Blocked task scored high".into(),
            }],
            impact: "Unblocks progress".into(),
            confidence: RecommendationConfidence::High,
            related_attention_id: Some("attention:task:1".into()),
            attention_reasons: vec![AttentionReason::new(
                AttentionSourceType::TaskGraph,
                AttentionSignal::BlockedTask,
                40,
                "task.base.blocked",
            )],
            related_task_id: Some("task:1".into()),
            related_purpose_label: None,
            related_decision_id: None,
            lifecycle_state: None,
            lifecycle_presented_at: None,
            lifecycle_resolved_at: None,
            lifecycle_resolution_type: None,
            explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn provenance_preserves_attention_reasons_and_keys() {
        let item = sample_item();
        let provenance = RecommendationProvenance::from_recommendation_item(&item);
        assert_eq!(provenance.recommendation_id, item.id);
        assert_eq!(provenance.reasoning_origins, item.attention_reasons);
        assert_eq!(provenance.explanation_keys, vec!["task.base.blocked"]);
        assert_eq!(provenance.source_evidence, item.evidence);
        assert_eq!(
            provenance.family,
            RecommendationFamily::RecommendationEngine
        );
    }

    #[test]
    fn action_proposal_has_no_authority_and_cannot_execute() {
        let proposal = ActionProposal::from_recommendation_item("ws-1", &sample_item())
            .with_requested_capability("application.launch")
            .with_experience_trace_match_keys(vec![
                "prefix_suffix:task.base.blocked".into(),
            ]);
        assert_eq!(proposal.authority_effect, "none");
        assert_eq!(
            proposal.requested_capability.as_deref(),
            Some("application.launch")
        );
        assert_eq!(
            proposal.provenance.experience_trace_match_keys,
            vec!["prefix_suffix:task.base.blocked"]
        );
        assert_eq!(
            proposal.identity.action_proposal_ref.as_deref(),
            Some(proposal.id.as_str())
        );
        assert!(ActionProposal::attempt_execute().is_err());
    }

    #[test]
    fn recommendation_payload_fields_remain_cloned_not_consumed() {
        let item = sample_item();
        let before = item.clone();
        let _ = ActionProposal::from_recommendation_item("ws-1", &item);
        assert_eq!(item, before);
    }

    #[test]
    fn identity_preserves_native_id_namespace() {
        let item = sample_item();
        let identity = RecommendationIdentity::from_recommendation_item(&item);
        assert_eq!(identity.native_id, item.id);
        assert!(identity
            .native_id
            .starts_with(RecommendationFamily::RecommendationEngine.native_id_prefix()));
        assert_eq!(identity.source_domain, "attention");
        assert_eq!(
            identity.originating_reasoning_ref.as_deref(),
            Some("attention:task:1")
        );
    }

    #[test]
    fn lifecycle_valid_transitions_and_rejects_invalid() {
        let mut life = RecommendationLifecycle::created("t0");
        assert_eq!(life.state, RecommendationLifecycleState::Created);
        life.transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        life.transition(RecommendationLifecycleState::Presented, "t2", Some("user".into()))
            .unwrap();
        assert_eq!(life.presented_at.as_deref(), Some("t2"));
        life.transition(RecommendationLifecycleState::Accepted, "t3", Some("user".into()))
            .unwrap();
        assert_eq!(
            life.resolution_type,
            Some(RecommendationResolutionType::Accepted)
        );

        let err = life
            .transition(RecommendationLifecycleState::Available, "t4", None)
            .unwrap_err();
        match err {
            ActionProposalError::InvalidLifecycleTransition { from, to } => {
                assert_eq!(from, "accepted");
                assert_eq!(to, "available");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn accept_and_expire_do_not_mutate_provenance_or_execute() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        let provenance_before = record.provenance.clone();
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", Some("local_user".into())).unwrap();
        assert_eq!(record.provenance, provenance_before);
        assert_eq!(record.authority_effect, "none");
        assert!(ActionProposal::attempt_execute().is_err());

        let mut expired = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        let provenance_before = expired.provenance.clone();
        expired
            .transition(RecommendationLifecycleState::Expired, "t1", None)
            .unwrap();
        assert_eq!(expired.provenance, provenance_before);
        assert_eq!(
            expired.lifecycle.resolution_type,
            Some(RecommendationResolutionType::Expired)
        );
    }

    #[test]
    fn rejection_and_expiry_are_not_system_failures() {
        let item = sample_item();
        let mut rejected = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        rejected
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        rejected
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        rejected
            .transition(RecommendationLifecycleState::Rejected, "t3", None)
            .unwrap();
        let outcome = rejected.record_outcome("t4").unwrap();
        assert_eq!(
            outcome.result_kind,
            RecommendationResultKind::RejectedByUser
        );
        assert!(!outcome.is_system_failure());
        assert_eq!(outcome.provenance, rejected.provenance);
        assert!(RecommendationOutcome::attempt_execute().is_err());

        let mut expired = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        expired
            .transition(RecommendationLifecycleState::Expired, "t1", None)
            .unwrap();
        let outcome = expired.record_outcome("t2").unwrap();
        assert_eq!(
            outcome.result_kind,
            RecommendationResultKind::ExpiredWithoutAction
        );
        assert!(!outcome.is_system_failure());
        assert!(!outcome.may_mutate_historical_reasoning());
        assert!(!outcome.may_silently_change_scoring());
        assert!(outcome.may_inform_future_adaptation());
    }

    #[test]
    fn outcome_preserves_provenance_and_experience_keys() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record.provenance = record.provenance.with_experience_trace_match_keys(vec![
            "prefix_suffix:task.base.blocked".into(),
        ]);
        let provenance = record.provenance.clone();
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        assert_eq!(outcome.provenance, provenance);
        assert_eq!(
            outcome.experience_trace_match_keys,
            vec!["prefix_suffix:task.base.blocked"]
        );
        assert_eq!(outcome.authority_effect, "none");
    }

    #[test]
    fn outcome_adaptation_requires_review_and_cannot_apply() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "attention_presentation",
            "Prefer structured DisplayReason lists for similar blockers",
            "Clearer rationale on future blocker recommendations",
        );
        assert!(proposal.requires_explicit_handling());
        assert!(!proposal.may_auto_apply());
        assert!(!proposal.may_self_approve());
        assert!(!proposal.may_mutate_cognition());
        assert!(!proposal.may_silently_change_scoring());
        proposal.require_review().unwrap();
        proposal
            .transition_review(OutcomeAdaptationReviewStatus::Reviewed)
            .unwrap();
        proposal.approve_for_handoff().unwrap();
        assert_eq!(proposal.provenance, provenance);
        assert_eq!(proposal.authority_effect, "none");
        assert!(proposal.controlled_change_surface_ready());
        assert!(OutcomeAdaptationProposal::attempt_apply().is_err());
        assert!(OutcomeAdaptationProposal::attempt_execute().is_err());
        assert!(proposal.attempt_mark_applied().is_err());
    }

    #[test]
    fn adaptation_review_forbids_self_approve_and_rejected_apply() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let mut proposal = outcome.to_adaptation_proposal("area", "change", "effect");
        proposal.require_review().unwrap();
        assert!(proposal.attempt_self_approve().is_err());
        assert_eq!(
            proposal.approve(
                AdaptationReviewerIdentity {
                    actor_id: OutcomeAdaptationProposal::PROPOSER_ACTOR_ID.into(),
                    actor_type: "local_user".into(),
                },
                "t"
            ),
            Err(ActionProposalError::AdaptationSelfApprovalForbidden)
        );
        assert_eq!(
            proposal.approve(
                AdaptationReviewerIdentity {
                    actor_id: "bot".into(),
                    actor_type: "system".into(),
                },
                "t"
            ),
            Err(ActionProposalError::AdaptationReviewerRequired)
        );
        proposal
            .reject(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-reject",
                "not useful",
            )
            .unwrap();
        assert_eq!(
            proposal.attempt_apply_after_rejection(),
            Err(ActionProposalError::RejectedAdaptationCannotApply)
        );
        assert!(!proposal.may_execute_from_approval());
        assert!(!proposal.may_bypass_permission_gateway());
        let surface = ControlledChangeSurface::from_approved_proposal(&proposal);
        assert!(surface.is_err());
    }

    #[test]
    fn controlled_change_requires_approval_and_preserves_provenance() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "presentation",
            "Prefer structured DisplayReason",
            "Clearer rationale",
        );
        assert_eq!(
            ControlledChangeSurface::from_unapproved_proposal(&proposal),
            Err(ActionProposalError::UnapprovedControlledChangeForbidden)
        );
        assert_eq!(
            BehaviourVersion::from_unapproved_proposal(&proposal),
            Err(ActionProposalError::UnapprovedControlledChangeForbidden)
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
        assert_eq!(surface.originating_proposal_id, proposal.id);
        assert_eq!(surface.approved_change, "Prefer structured DisplayReason");
        assert_eq!(surface.reviewer.actor_id, "local_user");
        assert_eq!(surface.previous_version_id, BehaviourVersion::BASELINE_ID);
        assert!(!surface.rollback.may_auto_rollback);
        assert_eq!(surface.provenance, provenance);
        let version = surface.to_behaviour_version_draft().unwrap();
        assert_eq!(version.lifecycle, BehaviourVersionLifecycle::Draft);
        assert_eq!(version.provenance, provenance);
        assert!(version.attempt_publish().is_err());
        let rollback = version.prepare_rollback_draft("note").unwrap();
        assert_eq!(rollback.provenance, provenance);
        let mut evaluation = ChangeEvaluation::from_behaviour_version(
            &version,
            vec!["display clearer".into()],
            vec!["user understands reason".into()],
            Some("rollback if confusion increases".into()),
        );
        assert!(!evaluation.may_rewrite_history());
        assert!(evaluation.attempt_rewrite_provenance().is_err());
        assert_eq!(evaluation.provenance_snapshot, provenance);
        assert!(!surface.may_bypass_permission_gateway());
        assert!(!version.may_bypass_permission_gateway());
        assert!(!evaluation.may_bypass_permission_gateway());
    }

    #[test]
    fn governance_ledger_and_publish_require_approval_without_authority() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "presentation",
            "Prefer structured DisplayReason",
            "Clearer rationale",
        );
        proposal.require_review().unwrap();
        // Unapproved: no publish.
        let incomplete = GovernanceRecord::from_adaptation_chain(
            &proposal, None, None, None, None,
        );
        assert!(incomplete.approval_reference.is_none());
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
        let version = surface.to_behaviour_version_draft().unwrap();
        let rollback = version.prepare_rollback_draft("safer").unwrap();
        let evaluation = ChangeEvaluation::from_behaviour_version(
            &version,
            vec!["clearer display".into()],
            vec!["users understand".into()],
            Some("rollback if confusion".into()),
        );
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal,
            Some(&surface),
            Some(&version),
            Some(&evaluation),
            Some(&rollback),
        );
        assert!(governance.retains_provenance(&provenance));
        assert_eq!(governance.rollback_reference.as_deref(), Some(rollback.id.as_str()));
        assert!(!governance.may_grant_execution_authority());
        assert!(GovernanceRecord::attempt_grant_execution_authority().is_err());
        let mut publish = PublishRequest::from_evaluated_version(
            &version,
            &evaluation,
            &governance,
            AdaptationReviewerIdentity::local_user("local_user"),
        )
        .unwrap();
        publish.submit_for_governance_review().unwrap();
        publish
            .approve_for_publish(&AdaptationReviewerIdentity::local_user("local_user"))
            .unwrap();
        assert!(publish.attempt_activate_published_version().is_err());
        assert!(PublishedVersionRecord::attempt_from_publish_request(&publish).is_err());
        assert!(PublishRequest::attempt_record_published_without_approval().is_err());
        assert!(!publish.may_grant_execution_authority());
        assert_eq!(publish.provenance, provenance);
        assert_eq!(rollback.provenance, provenance);
    }

    #[test]
    fn governance_policy_enforces_review_without_execution() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "presentation",
            "Prefer structured DisplayReason",
            "Clearer rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let policy = GovernancePolicy::for_outcome_adaptation();
        assert!(!policy.may_execute());
        assert!(GovernancePolicy::attempt_execute().is_err());
        assert!(GovernanceReviewDecision::attempt_bypass_provenance().is_err());
        // Empty rationale rejected.
        assert_eq!(
            GovernanceReviewDecision::new(
                &policy,
                AdaptationReviewerIdentity::local_user("local_user"),
                GovernanceReviewDecisionKind::Approve,
                "   ",
                "t-dec",
                vec![],
                &provenance,
            ),
            Err(ActionProposalError::GovernanceReviewRationaleRequired)
        );
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Meets presentation standards",
            "t-dec",
            vec!["no scoring mutation".into()],
            &provenance,
        )
        .unwrap();
        assert!(decision.retains_provenance(&provenance));
        assert!(!decision.may_bypass_provenance());
        let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
        let version = surface.to_behaviour_version_draft().unwrap();
        let evaluation = ChangeEvaluation::from_behaviour_version(
            &version,
            vec!["clearer".into()],
            vec!["understood".into()],
            None,
        );
        // Without decisions, policy requirements fail.
        assert_eq!(
            policy.enforce_approval_requirements(&proposal, &[]),
            Err(ActionProposalError::GovernancePolicyRequirementsNotMet)
        );
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal,
            Some(&surface),
            Some(&version),
            Some(&evaluation),
            None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap();
        assert_eq!(governance.policy_reference.as_deref(), Some(policy.id.as_str()));
        let mut publish = PublishRequest::from_evaluated_version_under_policy(
            &version,
            &evaluation,
            &governance,
            &policy,
            &proposal,
            &[decision],
            AdaptationReviewerIdentity::local_user("local_user"),
        )
        .unwrap();
        publish.submit_for_governance_review().unwrap();
        publish
            .approve_for_publish_under_policy(
                &policy,
                &proposal,
                &[],
                &AdaptationReviewerIdentity::local_user("local_user"),
            )
            .expect_err("empty decisions must fail threshold");
        assert!(!policy.may_bypass_permission_gateway());
    }

    #[test]
    fn governance_risk_routing_requires_stronger_high_risk_review() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "behaviour_workflow",
            "Draft behaviour version hint",
            "Governed draft only",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        assert!(GovernanceRisk::attempt_execute().is_err());
        assert!(GovernanceRisk::attempt_mutate_cognition().is_err());
        let scoring = outcome.to_adaptation_proposal(
            "attention_weight_scoring",
            "Retune attention",
            "forbidden",
        );
        assert_eq!(
            GovernanceReviewRouting::route_change(&scoring, None),
            Err(ActionProposalError::GovernanceRiskCannotMutateCognition)
        );
        let routing = GovernanceReviewRouting::route_change(&proposal, None).unwrap();
        assert_eq!(routing.risk.risk_level, AdaptationRiskClass::High);
        assert!(routing.risk.risk_level.requires_stronger_review());
        assert_eq!(routing.required_reviewer_count, 2);
        assert!(!routing.may_mutate_cognition());
        let one = GovernanceReviewDecision::new(
            &routing.policy,
            AdaptationReviewerIdentity::local_user("reviewer_a"),
            GovernanceReviewDecisionKind::Approve,
            "First approval",
            "t1",
            vec!["no runtime activation".into()],
            &provenance,
        )
        .unwrap();
        assert_eq!(
            routing.enforce_decisions(&proposal, &[one.clone()]),
            Err(ActionProposalError::GovernancePolicyRequirementsNotMet)
        );
        let two = GovernanceReviewDecision::new(
            &routing.policy,
            AdaptationReviewerIdentity::local_user("reviewer_b"),
            GovernanceReviewDecisionKind::Approve,
            "Second approval",
            "t2",
            vec!["no scoring mutation".into()],
            &provenance,
        )
        .unwrap();
        routing
            .enforce_decisions(&proposal, &[one, two])
            .unwrap();
        assert!(!routing.may_bypass_permission_gateway());
    }

    #[test]
    fn governance_decision_evidence_and_readiness_without_activation() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        assert_eq!(
            GovernanceDecisionEvidence::assemble(
                &risk,
                vec![],
                vec![],
                vec![],
                "missing refs",
                &provenance,
            ),
            Err(ActionProposalError::GovernanceDecisionEvidenceRequired)
        );
        let mut evidence = GovernanceDecisionEvidence::assemble(
            &risk,
            vec![
                format!("outcome:{}", outcome.id),
                "exact:continuity.resumable".into(),
            ],
            vec![],
            vec!["no scoring mutation".into()],
            "Evidence supports presentation-only change",
            &provenance,
        )
        .unwrap();
        assert!(!evidence.may_grant_execution_authority());
        assert!(GovernanceDecisionEvidence::attempt_grant_execution_authority().is_err());
        let policy = GovernancePolicy::from_risk(&risk);
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve with evidence",
            "t-dec",
            vec!["no scoring mutation".into()],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
        readiness.mark_risk_reviewed().unwrap();
        readiness.mark_approved(&evidence).unwrap();
        readiness.mark_ready_for_publication(&evidence).unwrap();
        assert!(readiness.attempt_activate_published().is_err());
        assert!(!readiness.may_activate_runtime());
        evidence.record_dissent("reviewer_c", "Minor wording concern", "t-dissent");
        assert_eq!(evidence.dissent_count(), 1);
        assert!(!evidence.may_erase_approval_history());
        assert!(readiness.history_preserves_approvals());
        assert!(readiness
            .history
            .contains(&PublicationReadinessState::Approved));
        let surface = ControlledChangeSurface::from_approved_proposal(&proposal).unwrap();
        let version = surface.to_behaviour_version_draft().unwrap();
        let evaluation =
            ChangeEvaluation::from_behaviour_version(&version, vec![], vec![], None);
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal,
            Some(&surface),
            Some(&version),
            Some(&evaluation),
            None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[decision])
        .unwrap();
        assert_eq!(governance.evidence_reference.as_deref(), Some(evidence.id.as_str()));
        assert_eq!(governance.dissent_references.len(), 1);
        assert!(governance.approval_reference.is_some());
        assert_eq!(
            governance.publication_readiness,
            Some(PublicationReadinessState::ReadyForPublication)
        );
    }

    #[test]
    fn governance_workspace_and_publication_environment_are_non_executing() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        let evidence = GovernanceDecisionEvidence::assemble(
            &risk,
            vec!["outcome:x".into(), "exact:continuity.resumable".into()],
            vec![],
            vec!["no activation".into()],
            "Presentation change evidenced",
            &provenance,
        )
        .unwrap();
        let policy = GovernancePolicy::from_risk(&risk);
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve",
            "t-dec",
            vec!["no activation".into()],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
        readiness.mark_risk_reviewed().unwrap();
        readiness.mark_approved(&evidence).unwrap();
        readiness.mark_ready_for_publication(&evidence).unwrap();
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal, None, None, None, None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[decision.clone()])
        .unwrap();
        let workspace = GovernanceWorkspace::from_governance_bundle(
            &proposal,
            &governance,
            Some(&evidence),
            Some(&risk),
            &[decision],
            Some(&readiness),
        );
        assert!(workspace.preserves_provenance(&provenance));
        assert!(!workspace.may_approve_execution());
        assert!(GovernanceWorkspace::attempt_approve_execution().is_err());
        assert!(GovernanceWorkspace::attempt_execute().is_err());
        let env = PublicationEnvironment::from_governance_workspace(
            &workspace,
            "workspace:local",
            vec!["schema_compatible".into()],
            BehaviourVersion::BASELINE_ID,
            PublicationRolloutStage::StagedCanary,
        );
        assert!(env.preserves_provenance(&provenance));
        assert!(!env.may_activate_runtime());
        assert!(PublicationEnvironment::attempt_activate().is_err());
        assert!(!env.may_bypass_permission_gateway());
        assert_eq!(env.governance_workspace_id, workspace.id);
    }

    #[test]
    fn governance_timeline_preserves_provenance_and_cannot_execute() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        let mut evidence = GovernanceDecisionEvidence::assemble(
            &risk,
            vec!["outcome:x".into(), "exact:continuity.resumable".into()],
            vec![],
            vec!["no activation".into()],
            "Evidenced",
            &provenance,
        )
        .unwrap();
        evidence.record_dissent("reviewer_b", "Minor concern", "t-d");
        let policy = GovernancePolicy::from_risk(&risk);
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve",
            "t-dec",
            vec!["no activation".into()],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
        readiness.mark_risk_reviewed().unwrap();
        readiness.mark_approved(&evidence).unwrap();
        readiness.mark_ready_for_publication(&evidence).unwrap();
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal, None, None, None, None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[decision.clone()])
        .unwrap();
        let workspace = GovernanceWorkspace::from_governance_bundle(
            &proposal,
            &governance,
            Some(&evidence),
            Some(&risk),
            &[decision.clone()],
            Some(&readiness),
        );
        let mut timeline = GovernanceTimeline::from_full_lifecycle(
            &proposal,
            &risk,
            &evidence,
            &[decision],
            &governance,
            &workspace,
            &readiness,
            "t",
        )
        .unwrap();
        assert!(timeline.provenance_survives(&provenance));
        assert_eq!(timeline.dissent_immutable_refs.len(), 1);
        assert!(!timeline.may_rewrite_events());
        assert!(timeline.attempt_rewrite_event().is_err());
        assert!(GovernanceTimeline::attempt_execute().is_err());
        assert!(GovernanceTimeline::attempt_activate_runtime().is_err());
        assert!(!timeline.may_bypass_permission_gateway());

        let reject = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Reject,
            "Not a fit",
            "t-rej",
            vec![],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let rejected = GovernanceTimeline::from_rejected_lifecycle(
            &proposal, &risk, &evidence, &reject, "r",
        )
        .unwrap();
        assert!(rejected.rejected_visible);
        assert!(rejected.provenance_survives(&provenance));
        assert!(rejected
            .stages()
            .contains(&GovernanceLifecycleStage::RejectedVisible));
    }

    #[test]
    fn publication_safety_blocks_failed_validation_and_activation() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        let evidence = GovernanceDecisionEvidence::assemble(
            &risk,
            vec!["outcome:x".into()],
            vec![],
            vec![],
            "Evidenced",
            &provenance,
        )
        .unwrap();
        let policy = GovernancePolicy::from_risk(&risk);
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve",
            "t-dec",
            vec![],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
        readiness.mark_risk_reviewed().unwrap();
        readiness.mark_approved(&evidence).unwrap();
        readiness.mark_ready_for_publication(&evidence).unwrap();
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal, None, None, None, None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[decision.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[decision.clone()])
        .unwrap();
        let workspace = GovernanceWorkspace::from_governance_bundle(
            &proposal,
            &governance,
            Some(&evidence),
            Some(&risk),
            &[decision],
            Some(&readiness),
        );
        let env = PublicationEnvironment::from_governance_workspace(
            &workspace,
            "workspace:local",
            vec!["schema_compatible".into()],
            BehaviourVersion::BASELINE_ID,
            PublicationRolloutStage::StagedCanary,
        );
        let mut failed = PublicationSafetyContract::from_ready(&readiness, &env)
            .unwrap()
            .with_failed_gate("compat:schema_compatible");
        assert!(failed.run_validation().is_err());
        assert_eq!(
            failed.lifecycle_state,
            PublicationSafetyLifecycleState::ValidationFailed
        );
        assert!(failed.prepare_migration().is_err());
        assert!(failed.lifecycle_history.contains(
            &PublicationSafetyLifecycleState::ReadyForPublication
        ));

        let mut ok = PublicationSafetyContract::from_ready(&readiness, &env).unwrap();
        ok.prepare_migration().unwrap();
        ok.prepare_rollback().unwrap();
        assert!(ok.rollback_preserves_history());
        ok.approve_release().unwrap();
        assert!(!ok.may_execute_commands());
        assert!(!ok.may_bypass_permission_gateway());
        assert!(!ok.may_mutate_cognition());
        assert!(!ok.may_rewrite_provenance());
        assert!(ok.attempt_publish_activate().is_err());
        assert!(PublicationSafetyContract::attempt_execute().is_err());
        assert!(PublicationSafetyContract::attempt_bypass_gateway().is_err());
        assert_eq!(ok.provenance, provenance);
    }

    #[test]
    fn governance_failure_preserves_history_and_blocks_recovery_execution() {
        let item = sample_item();
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, "t0");
        record
            .transition(RecommendationLifecycleState::Available, "t1", None)
            .unwrap();
        record
            .transition(RecommendationLifecycleState::Presented, "t2", None)
            .unwrap();
        record.accept("t3", None).unwrap();
        let outcome = record.record_outcome("t4").unwrap();
        let provenance = outcome.provenance.clone();
        let mut proposal = outcome.to_adaptation_proposal(
            "experience_presentation",
            "Keep DisplayReason primary",
            "Consistent rationale",
        );
        proposal.require_review().unwrap();
        proposal
            .approve(
                AdaptationReviewerIdentity::local_user("local_user"),
                "t-ok",
            )
            .unwrap();
        let risk = GovernanceRisk::classify_from_proposal(&proposal);
        let evidence = GovernanceDecisionEvidence::assemble(
            &risk,
            vec!["outcome:x".into()],
            vec![],
            vec![],
            "Evidenced",
            &provenance,
        )
        .unwrap();
        let policy = GovernancePolicy::from_risk(&risk);
        let reject = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Reject,
            "Not a fit",
            "t-rej",
            vec![],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let actors = GovernanceActorRefs {
            proposer_actor_id: proposal.proposed_by_actor_id.clone(),
            reviewer_actor_id: Some("local_user".into()),
            publisher_actor_id: None,
        };
        let mut failure =
            GovernanceFailureState::from_failed_review(&reject, &evidence, actors.clone())
                .unwrap();
        failure.record().unwrap();
        failure
            .plan_recovery(GovernanceFailureRecoveryRequirement::default_safe(
                "document rejection; no re-apply",
            ))
            .unwrap();
        failure.abandon().unwrap();
        assert!(failure.preserves_provenance(&provenance));
        assert!(failure.preserves_evidence());
        assert!(failure.abandoned_remains_visible());
        assert!(!failure.may_delete_provenance());
        assert!(!failure.may_grant_authority());
        assert!(!failure.may_activate_runtime());
        assert!(!failure.may_bypass_permission_gateway());
        assert!(failure.attempt_delete_provenance().is_err());
        assert!(failure.attempt_recovery_execute().is_err());
        assert!(GovernanceFailureState::attempt_execute().is_err());
        assert!(GovernanceFailureState::attempt_bypass_gateway().is_err());
        assert!(GovernanceFailureState::attempt_activate_runtime().is_err());
        assert_eq!(
            failure.category.pattern_source(),
            "GovernanceReviewDecision Reject"
        );

        let approve = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve",
            "t-dec",
            vec![],
            &provenance,
        )
        .unwrap()
        .with_evidence(&evidence);
        let mut readiness = PublicationReadiness::draft_from_evidence(&evidence);
        readiness.mark_risk_reviewed().unwrap();
        readiness.mark_approved(&evidence).unwrap();
        readiness.mark_ready_for_publication(&evidence).unwrap();
        let governance = GovernanceRecord::from_adaptation_chain(
            &proposal, None, None, None, None,
        )
        .with_policy_and_decisions(&policy, &proposal, &[approve.clone()])
        .unwrap()
        .with_evidence_and_readiness(&evidence, &readiness, &[approve.clone()])
        .unwrap();
        let workspace = GovernanceWorkspace::from_governance_bundle(
            &proposal,
            &governance,
            Some(&evidence),
            Some(&risk),
            &[approve],
            Some(&readiness),
        );
        let env = PublicationEnvironment::from_governance_workspace(
            &workspace,
            "workspace:local",
            vec!["schema_compatible".into()],
            BehaviourVersion::BASELINE_ID,
            PublicationRolloutStage::StagedCanary,
        );
        let mut safety = PublicationSafetyContract::from_ready(&readiness, &env)
            .unwrap()
            .with_failed_gate("compat:schema_compatible");
        let _ = safety.run_validation();
        let validation_failure = GovernanceFailureState::from_publication_validation_failure(
            &safety,
            evidence.id.clone(),
            actors,
        );
        assert!(validation_failure.preserves_evidence());
        assert!(validation_failure.preserves_provenance(&provenance));
        assert_eq!(
            validation_failure.category,
            GovernanceFailureCategory::PublicationValidation
        );
    }
}
