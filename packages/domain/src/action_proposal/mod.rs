//! Action Proposal + Recommendation identity/lifecycle (Sprint 137–138).
//!
//! Governance shapes between recommendations and future Permission Gateway work.
//! **Never executes.** Native family IDs are preserved — never collapsed into one namespace.
//! Reasoning provenance is immutable; lifecycle metadata mutates separately.

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
                evaluated_at: evaluation.map(|_| approved_at.clone().unwrap_or_else(|| "evaluated".into())),
                publish_requested_at: None,
                published_at: None,
            },
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
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
}
