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
}
