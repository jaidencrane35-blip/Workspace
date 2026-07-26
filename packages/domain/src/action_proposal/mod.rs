//! Action Proposal architecture (Sprint 137).
//!
//! Governance bridge shape between recommendations and future Permission Gateway work.
//! **Never executes.** Not wired to CommandPipeline. Authority remains `"none"` until a
//! future privileged command obtains Gateway Allow.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_attention::AttentionReason;
use crate::workspace_recommendation::{RecommendationEvidence, RecommendationItem};

/// Action-proposal validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ActionProposalError {
    #[error("action proposal cannot execute or authorize")]
    CannotExecute,

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
}

/// Required provenance for answering: "Why was this recommendation created?"
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
/// Sprint 137 defines the shape only. Creating an `ActionProposal` never launches,
/// grants capabilities, or bypasses Permission Gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionProposal {
    pub id: String,
    pub workspace_id: String,
    pub recommendation_ref: String,
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
        Self {
            id: format!("action_proposal:{}", item.id),
            workspace_id: workspace_id.into(),
            recommendation_ref: item.id.clone(),
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
        assert!(ActionProposal::attempt_execute().is_err());
    }

    #[test]
    fn recommendation_payload_fields_remain_cloned_not_consumed() {
        let item = sample_item();
        let before = item.clone();
        let _ = ActionProposal::from_recommendation_item("ws-1", &item);
        assert_eq!(item, before);
    }
}
