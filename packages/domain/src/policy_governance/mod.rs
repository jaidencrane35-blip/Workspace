//! Policy & Governance Engine — Programme III Batch 2.
//!
//! Policy reasoning layer that evaluates workspace context and produces
//! governance decisions, constraints, and explanations.
//! Policy explains authority. It does not become authority.
//! PermissionGateway remains the final authorisation boundary.
//!
//! Note: Distinct from `action_proposal::GovernancePolicy` (outcome-adaptation
//! review requirements). This module owns workspace policy evaluation evidence.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_state_envelope::{
    CompletenessStatus, ConsistencyStatus, FreshnessStatus, WorkspaceStateEnvelope,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PolicyGovernanceError {
    #[error("invalid policy status: {0}")]
    InvalidPolicyStatus(String),

    #[error("invalid policy scope: {0}")]
    InvalidPolicyScope(String),

    #[error("invalid evaluation result: {0}")]
    InvalidEvaluationResult(String),

    #[error("invalid severity: {0}")]
    InvalidSeverity(String),

    #[error("confidence must be 0..=100 (got {0})")]
    InvalidConfidence(u8),

    #[error("policy artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("policy artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("policy outputs must not contain capability grants")]
    MustNotContainCapabilityGrant,

    #[error("evaluation snapshot not found")]
    SnapshotNotFound,

    #[error("policy unavailable")]
    PolicyUnavailable,

    #[error("evaluation incomplete — workspace state missing")]
    EvaluationIncomplete,

    #[error("evaluation failed — corrupt or unusable evidence")]
    EvaluationFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_confidence(value: u8) -> Result<u8, PolicyGovernanceError> {
    if value > 100 {
        Err(PolicyGovernanceError::InvalidConfidence(value))
    } else {
        Ok(value)
    }
}

/// Policy lifecycle metadata — not operational lifecycle ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDefinitionStatus {
    Draft,
    Active,
    Deprecated,
    Retired,
}

impl PolicyDefinitionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyGovernanceError> {
        match value {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "deprecated" => Ok(Self::Deprecated),
            "retired" => Ok(Self::Retired),
            other => Err(PolicyGovernanceError::InvalidPolicyStatus(other.into())),
        }
    }

    pub fn is_evaluable(self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyScope {
    Workspace,
    Project,
    Capability,
    Command,
    Resource,
    ActorContext,
}

impl PolicyScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Project => "project",
            Self::Capability => "capability",
            Self::Command => "command",
            Self::Resource => "resource",
            Self::ActorContext => "actor_context",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyGovernanceError> {
        match value {
            "workspace" => Ok(Self::Workspace),
            "project" => Ok(Self::Project),
            "capability" => Ok(Self::Capability),
            "command" => Ok(Self::Command),
            "resource" => Ok(Self::Resource),
            "actor_context" => Ok(Self::ActorContext),
            other => Err(PolicyGovernanceError::InvalidPolicyScope(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl PolicySeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyGovernanceError> {
        match value {
            "critical" => Ok(Self::Critical),
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            other => Err(PolicyGovernanceError::InvalidSeverity(other.into())),
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::Critical => 4,
            Self::High => 3,
            Self::Medium => 2,
            Self::Low => 1,
        }
    }
}

/// Evaluation outcome — unknown context must not become approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEvaluationResult {
    Compliant,
    Violation,
    RequiresReview,
    Unknown,
    NotApplicable,
}

impl PolicyEvaluationResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compliant => "compliant",
            Self::Violation => "violation",
            Self::RequiresReview => "requires_review",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyGovernanceError> {
        match value {
            "compliant" => Ok(Self::Compliant),
            "violation" => Ok(Self::Violation),
            "requires_review" => Ok(Self::RequiresReview),
            "unknown" => Ok(Self::Unknown),
            "not_applicable" => Ok(Self::NotApplicable),
            other => Err(PolicyGovernanceError::InvalidEvaluationResult(other.into())),
        }
    }

    /// Restrictiveness rank for conflict aggregation (higher = more restrictive).
    pub fn restrictiveness(self) -> u8 {
        match self {
            Self::Violation => 5,
            Self::RequiresReview => 4,
            Self::Unknown => 3,
            Self::NotApplicable => 2,
            Self::Compliant => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceEvaluationStatus {
    Current,
    Superseded,
    Archived,
}

impl GovernanceEvaluationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyGovernanceError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(PolicyGovernanceError::InvalidPolicyStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// A workspace policy rule (Programme III) — distinct from outcome-adaptation GovernancePolicy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDefinition {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub scope: PolicyScope,
    pub version: String,
    pub severity: PolicySeverity,
    pub status: PolicyDefinitionStatus,
    pub authority_effect: String,
    pub actionable: bool,
}

impl PolicyDefinition {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn define(
        policy_id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        scope: PolicyScope,
        version: impl Into<String>,
        severity: PolicySeverity,
        status: PolicyDefinitionStatus,
    ) -> Self {
        Self {
            policy_id: policy_id.into(),
            name: name.into(),
            description: description.into(),
            scope,
            version: version.into(),
            severity,
            status,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), PolicyGovernanceError> {
        if !self.is_non_actionable() {
            return Err(PolicyGovernanceError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Evidence result of evaluating one policy against a context revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvaluation {
    pub evaluation_id: String,
    pub policy_id: String,
    pub policy_version: String,
    pub context_revision: String,
    pub result: PolicyEvaluationResult,
    pub severity: PolicySeverity,
    pub reason: String,
    pub evidence_refs: Vec<String>,
    pub created_at: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl PolicyEvaluation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "policy_evaluation:";

    pub fn record(
        policy: &PolicyDefinition,
        context_revision: impl Into<String>,
        result: PolicyEvaluationResult,
        reason: impl Into<String>,
        evidence_refs: Vec<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let context_revision = context_revision.into();
        let reason = reason.into();
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}",
            policy.policy_id,
            policy.version,
            context_revision,
            result.as_str(),
            reason
        ));
        Self {
            evaluation_id: format!("{}{}", Self::ID_PREFIX, digest),
            policy_id: policy.policy_id.clone(),
            policy_version: policy.version.clone(),
            context_revision,
            result,
            severity: policy.severity,
            reason,
            evidence_refs,
            created_at: created_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), PolicyGovernanceError> {
        if !self.is_non_actionable() {
            return Err(PolicyGovernanceError::MustNotBeActionable);
        }
        if self.reason.to_lowercase().contains("capability_grant")
            || self.reason.to_lowercase().contains("permission_grant")
        {
            return Err(PolicyGovernanceError::MustNotContainCapabilityGrant);
        }
        Ok(())
    }
}

/// Output for Gateway / human surfaces — never authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRecommendation {
    pub recommendation_id: String,
    pub evaluation_refs: Vec<String>,
    pub suggested_action: String,
    pub confidence: u8,
    pub explanation: String,
    pub aggregate_result: PolicyEvaluationResult,
    pub authority_effect: String,
    pub actionable: bool,
}

impl GovernanceRecommendation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "governance_recommendation:";

    pub fn from_evaluations(
        evaluations: &[PolicyEvaluation],
        created_hint: &str,
    ) -> Result<Self, PolicyGovernanceError> {
        let aggregate = aggregate_results(evaluations);
        let refs: Vec<String> = evaluations.iter().map(|e| e.evaluation_id.clone()).collect();
        let explanation = match aggregate {
            PolicyEvaluationResult::Compliant => {
                "Active policies report compliance for the observed context revision.".into()
            }
            PolicyEvaluationResult::Violation => {
                "One or more policies report a violation — Gateway must deny or escalate.".into()
            }
            PolicyEvaluationResult::RequiresReview => {
                "Policy says approval/review required — PermissionGateway remains the authoriser."
                    .into()
            }
            PolicyEvaluationResult::Unknown => {
                "Policy evaluation is unknown due to incomplete or unavailable context — fail closed."
                    .into()
            }
            PolicyEvaluationResult::NotApplicable => {
                "No active policies applied to this context.".into()
            }
        };
        let suggested_action = match aggregate {
            PolicyEvaluationResult::Compliant => "proceed_under_gateway".into(),
            PolicyEvaluationResult::Violation => "deny_or_escalate".into(),
            PolicyEvaluationResult::RequiresReview => "require_permission_approval".into(),
            PolicyEvaluationResult::Unknown => "fail_closed_unknown".into(),
            PolicyEvaluationResult::NotApplicable => "no_policy_signal".into(),
        };
        let confidence = match aggregate {
            PolicyEvaluationResult::Compliant => 80,
            PolicyEvaluationResult::Violation => 90,
            PolicyEvaluationResult::RequiresReview => 75,
            PolicyEvaluationResult::Unknown => 20,
            PolicyEvaluationResult::NotApplicable => 40,
        };
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}",
            created_hint,
            aggregate.as_str(),
            refs.join(","),
            suggested_action
        ));
        Ok(Self {
            recommendation_id: format!("{}{}", Self::ID_PREFIX, digest),
            evaluation_refs: refs,
            suggested_action,
            confidence: clamp_confidence(confidence)?,
            explanation,
            aggregate_result: aggregate,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), PolicyGovernanceError> {
        clamp_confidence(self.confidence)?;
        if !self.is_non_actionable() {
            return Err(PolicyGovernanceError::MustNotBeActionable);
        }
        let forbidden = ["capability_grant", "permission_grant", "execute", "dispatch"];
        let blob = format!(
            "{} {} {}",
            self.suggested_action, self.explanation, self.recommendation_id
        )
        .to_lowercase();
        if forbidden.iter().any(|f| blob.contains(f)) {
            return Err(PolicyGovernanceError::MustNotContainCapabilityGrant);
        }
        Ok(())
    }
}

/// Explanation surface for ExplainGovernanceDecision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub context_revision: Option<String>,
    pub policies_involved: Vec<String>,
    pub evidence: Vec<String>,
    pub reasoning: Vec<String>,
    pub uncertainty: Vec<String>,
    pub aggregate_result: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl GovernanceExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &PolicyGovernanceView) -> Self {
        let policies_involved: Vec<_> = view
            .evaluations
            .iter()
            .map(|e| format!("{}@{}", e.policy_id, e.policy_version))
            .collect();
        let evidence: Vec<_> = view
            .evaluations
            .iter()
            .flat_map(|e| e.evidence_refs.clone())
            .collect();
        let reasoning: Vec<_> = view
            .evaluations
            .iter()
            .map(|e| format!("{}: {} ({})", e.policy_id, e.reason, e.result.as_str()))
            .collect();
        let mut uncertainty = view.unknowns.clone();
        if view.context_revision.is_none() {
            uncertainty.push("workspace state envelope missing — EvaluationIncomplete".into());
        }
        Self {
            explanation_id: format!("governance_explanation:{}", view.meta.evaluation_set_id),
            workspace_id: view.meta.workspace_id.clone(),
            context_revision: view.context_revision.clone(),
            policies_involved,
            evidence,
            reasoning,
            uncertainty,
            aggregate_result: view
                .recommendation
                .as_ref()
                .map(|r| r.aggregate_result.as_str().into()),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceMeta {
    pub evaluation_set_id: String,
    pub workspace_id: String,
    pub status: GovernanceEvaluationStatus,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub context_revision: Option<String>,
    pub policy_catalog_revision: String,
    pub evaluation_count: usize,
    pub aggregate_result: String,
    pub authority_effect: String,
    pub terminal: bool,
    pub actionable: bool,
}

impl PolicyGovernanceMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "policy_governance:";

    pub fn new(
        workspace_id: impl Into<String>,
        created_at: impl Into<String>,
        context_revision: Option<String>,
        policy_catalog_revision: impl Into<String>,
        evaluation_count: usize,
        aggregate_result: impl Into<String>,
    ) -> Self {
        Self {
            evaluation_set_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: GovernanceEvaluationStatus::Current,
            created_at: created_at.into(),
            superseded_at: None,
            context_revision,
            policy_catalog_revision: policy_catalog_revision.into(),
            evaluation_count,
            aggregate_result: aggregate_result.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            terminal: false,
            actionable: false,
        }
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = GovernanceEvaluationStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn validate(&self) -> Result<(), PolicyGovernanceError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(PolicyGovernanceError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceView {
    pub meta: PolicyGovernanceMeta,
    pub policies: Vec<PolicyDefinition>,
    pub evaluations: Vec<PolicyEvaluation>,
    pub recommendation: Option<GovernanceRecommendation>,
    pub unknowns: Vec<String>,
    pub context_revision: Option<String>,
    pub authority_effect: String,
}

impl PolicyGovernanceView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == PolicyGovernanceMeta::AUTHORITY_EFFECT_NONE
            && !self.meta.actionable
            && !self.meta.terminal
            && self.policies.iter().all(|p| p.is_non_actionable())
            && self.evaluations.iter().all(|e| e.is_non_actionable())
            && self
                .recommendation
                .as_ref()
                .map(|r| r.is_non_actionable())
                .unwrap_or(true)
    }

    pub fn validate(&self) -> Result<(), PolicyGovernanceError> {
        self.meta.validate()?;
        for p in &self.policies {
            p.validate()?;
        }
        for e in &self.evaluations {
            e.validate()?;
        }
        if let Some(r) = &self.recommendation {
            r.validate()?;
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(PolicyGovernanceError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceHistoryEntry {
    pub evaluation_set_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub context_revision: Option<String>,
    pub policy_catalog_revision: String,
    pub evaluation_count: usize,
    pub aggregate_result: String,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl PolicyGovernanceHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_meta(meta: &PolicyGovernanceMeta) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        Some(Self {
            evaluation_set_id: meta.evaluation_set_id.clone(),
            status: meta.status.as_str().into(),
            created_at: meta.created_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            context_revision: meta.context_revision.clone(),
            policy_catalog_revision: meta.policy_catalog_revision.clone(),
            evaluation_count: meta.evaluation_count,
            aggregate_result: meta.aggregate_result.clone(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<PolicyGovernanceView>,
    pub history: Vec<PolicyGovernanceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl PolicyGovernanceSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<PolicyGovernanceView>,
        history: Vec<PolicyGovernanceHistoryEntry>,
        history_count: usize,
        generated_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            current,
            history,
            history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.history.iter().all(|h| h.is_non_actionable())
            && self
                .current
                .as_ref()
                .map(|c| c.is_non_executing())
                .unwrap_or(true)
    }

    pub fn summary(&self, history_limit: usize) -> PolicyGovernanceSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        PolicyGovernanceSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            evaluation_set_id: self
                .current
                .as_ref()
                .map(|c| c.meta.evaluation_set_id.clone()),
            context_revision: self
                .current
                .as_ref()
                .and_then(|c| c.context_revision.clone()),
            aggregate_result: self
                .current
                .as_ref()
                .map(|c| c.meta.aggregate_result.clone()),
            evaluation_count: self
                .current
                .as_ref()
                .map(|c| c.evaluations.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub evaluation_set_id: Option<String>,
    pub context_revision: Option<String>,
    pub aggregate_result: Option<String>,
    pub evaluation_count: usize,
    pub history: Vec<PolicyGovernanceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Built-in active policy catalog (version-isolated).
pub fn default_policy_catalog() -> Vec<PolicyDefinition> {
    vec![
        PolicyDefinition::define(
            "policy:workspace_state_completeness",
            "Workspace state completeness",
            "Incomplete or unknown workspace state must not be treated as safe.",
            PolicyScope::Workspace,
            "1",
            PolicySeverity::High,
            PolicyDefinitionStatus::Active,
        ),
        PolicyDefinition::define(
            "policy:workspace_state_consistency",
            "Workspace state consistency",
            "Contradictory source evidence requires review before privileged action.",
            PolicyScope::Workspace,
            "1",
            PolicySeverity::Critical,
            PolicyDefinitionStatus::Active,
        ),
        PolicyDefinition::define(
            "policy:unavailable_sources_fail_closed",
            "Unavailable sources fail closed",
            "Unavailable authoritative sources yield Unknown — never Compliant.",
            PolicyScope::Workspace,
            "1",
            PolicySeverity::High,
            PolicyDefinitionStatus::Active,
        ),
        PolicyDefinition::define(
            "policy:stale_context_requires_review",
            "Stale context requires review",
            "Stale envelope freshness requires review before treating context as current.",
            PolicyScope::Workspace,
            "1",
            PolicySeverity::Medium,
            PolicyDefinitionStatus::Active,
        ),
    ]
}

pub fn policy_catalog_revision(policies: &[PolicyDefinition]) -> String {
    let mut parts: Vec<_> = policies
        .iter()
        .map(|p| format!("{}@{}:{}", p.policy_id, p.version, p.status.as_str()))
        .collect();
    parts.sort();
    format!("policies:{}", stable_digest(&parts.join("|")))
}

/// Deterministic evaluation of active policies against an optional envelope.
pub fn evaluate_policies(
    policies: &[PolicyDefinition],
    envelope: Option<&WorkspaceStateEnvelope>,
    created_at: &str,
) -> Result<(Vec<PolicyEvaluation>, Vec<String>), PolicyGovernanceError> {
    let mut evaluations = Vec::new();
    let mut unknowns = Vec::new();
    let context_revision = envelope
        .map(|e| e.revision.clone())
        .unwrap_or_else(|| "missing".into());

    let mut active: Vec<_> = policies
        .iter()
        .filter(|p| p.status.is_evaluable())
        .cloned()
        .collect();
    active.sort_by(|a, b| {
        b.severity
            .rank()
            .cmp(&a.severity.rank())
            .then_with(|| a.policy_id.cmp(&b.policy_id))
            .then_with(|| a.version.cmp(&b.version))
    });

    if envelope.is_none() {
        unknowns.push("workspace state envelope missing — EvaluationIncomplete".into());
    }

    for policy in &active {
        let (result, reason, evidence) = match (policy.policy_id.as_str(), envelope) {
            (_, None) => (
                PolicyEvaluationResult::Unknown,
                "EvaluationIncomplete: workspace state envelope unavailable".into(),
                vec!["context:missing".into()],
            ),
            ("policy:workspace_state_completeness", Some(env)) => match env.completeness {
                CompletenessStatus::Complete => (
                    PolicyEvaluationResult::Compliant,
                    "Envelope completeness is Complete".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                CompletenessStatus::Partial => (
                    PolicyEvaluationResult::RequiresReview,
                    "Envelope completeness is Partial — privileged action needs review".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                CompletenessStatus::Unknown => (
                    PolicyEvaluationResult::Unknown,
                    "Envelope completeness is Unknown — fail closed".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                CompletenessStatus::Contradictory => (
                    PolicyEvaluationResult::RequiresReview,
                    "Envelope completeness is Contradictory".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
            },
            ("policy:workspace_state_consistency", Some(env)) => match env.consistency {
                ConsistencyStatus::Consistent => (
                    PolicyEvaluationResult::Compliant,
                    "Envelope consistency is Consistent".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                ConsistencyStatus::Contradictory => (
                    PolicyEvaluationResult::RequiresReview,
                    "Envelope reports contradictions — review required".into(),
                    env.contradictions
                        .iter()
                        .map(|c| c.conflict_id.clone())
                        .collect(),
                ),
                ConsistencyStatus::Unknown => (
                    PolicyEvaluationResult::Unknown,
                    "Envelope consistency is Unknown — fail closed".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
            },
            ("policy:unavailable_sources_fail_closed", Some(env)) => {
                let unavailable: Vec<_> = env
                    .sources
                    .iter()
                    .filter(|s| s.freshness_status == FreshnessStatus::Unavailable)
                    .map(|s| s.source_type.clone())
                    .collect();
                if unavailable.is_empty() {
                    (
                        PolicyEvaluationResult::Compliant,
                        "No unavailable sources observed".into(),
                        vec![format!("envelope:{}", env.state_id)],
                    )
                } else {
                    (
                        PolicyEvaluationResult::Unknown,
                        format!(
                            "Unavailable sources present ({}) — never Compliant",
                            unavailable.join(", ")
                        ),
                        unavailable
                            .into_iter()
                            .map(|s| format!("source:{s}:unavailable"))
                            .collect(),
                    )
                }
            }
            ("policy:stale_context_requires_review", Some(env)) => match env.freshness {
                FreshnessStatus::Fresh => (
                    PolicyEvaluationResult::Compliant,
                    "Envelope freshness is Fresh".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                FreshnessStatus::Stale => (
                    PolicyEvaluationResult::RequiresReview,
                    "Envelope freshness is Stale — review before treating as current".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                FreshnessStatus::Unavailable => (
                    PolicyEvaluationResult::Unknown,
                    "Envelope freshness is Unavailable — fail closed".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
                FreshnessStatus::Unknown => (
                    PolicyEvaluationResult::Unknown,
                    "Envelope freshness is Unknown — fail closed".into(),
                    vec![format!("envelope:{}", env.state_id)],
                ),
            },
            (_, Some(_)) => (
                PolicyEvaluationResult::NotApplicable,
                "Policy has no registered evaluator for this catalog version".into(),
                vec![],
            ),
        };

        if matches!(
            result,
            PolicyEvaluationResult::Unknown | PolicyEvaluationResult::RequiresReview
        ) {
            unknowns.push(format!("{} → {}", policy.policy_id, result.as_str()));
        }

        evaluations.push(PolicyEvaluation::record(
            policy,
            &context_revision,
            result,
            reason,
            evidence,
            created_at,
        ));
    }

    evaluations.sort_by(|a, b| {
        a.policy_id
            .cmp(&b.policy_id)
            .then_with(|| a.policy_version.cmp(&b.policy_version))
    });
    Ok((evaluations, unknowns))
}

pub fn aggregate_results(evaluations: &[PolicyEvaluation]) -> PolicyEvaluationResult {
    if evaluations.is_empty() {
        return PolicyEvaluationResult::NotApplicable;
    }
    evaluations
        .iter()
        .map(|e| e.result)
        .max_by_key(|r| r.restrictiveness())
        .unwrap_or(PolicyEvaluationResult::NotApplicable)
}

fn stable_digest(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_state_envelope::{
        AvailabilityStatus, CompletenessStatus, FreshnessStatus,
        WorkspaceStateConflict, WorkspaceStateSource,
    };

    fn fresh_complete_envelope() -> WorkspaceStateEnvelope {
        WorkspaceStateEnvelope::compose_deterministic(
            "ws",
            "t0",
            vec![WorkspaceStateSource::observe(
                "planning",
                "planning:1",
                "p1",
                "t0",
                FreshnessStatus::Fresh,
                AvailabilityStatus::Available,
                CompletenessStatus::Complete,
                None,
            )],
            vec![],
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn deterministic_evaluation_same_inputs_same_ids() {
        let policies = default_policy_catalog();
        let env = fresh_complete_envelope();
        let (a, _) = evaluate_policies(&policies, Some(&env), "t0").unwrap();
        let (b, _) = evaluate_policies(&policies, Some(&env), "t0").unwrap();
        assert_eq!(a, b);
        let rec_a = GovernanceRecommendation::from_evaluations(&a, "t0").unwrap();
        let rec_b = GovernanceRecommendation::from_evaluations(&b, "t0").unwrap();
        assert_eq!(rec_a.recommendation_id, rec_b.recommendation_id);
        assert_eq!(rec_a.aggregate_result, rec_b.aggregate_result);
    }

    #[test]
    fn unknown_handling_missing_state_fails_closed() {
        let policies = default_policy_catalog();
        let (evals, unknowns) = evaluate_policies(&policies, None, "t0").unwrap();
        assert!(!unknowns.is_empty());
        assert!(evals
            .iter()
            .all(|e| e.result == PolicyEvaluationResult::Unknown));
        let agg = aggregate_results(&evals);
        assert_eq!(agg, PolicyEvaluationResult::Unknown);
        assert_ne!(agg, PolicyEvaluationResult::Compliant);
    }

    #[test]
    fn conflicting_policy_handling_prefers_restrictive() {
        let mut policies = default_policy_catalog();
        // Force a draft policy that would be Compliant if active — ignored.
        policies.push(PolicyDefinition::define(
            "policy:draft_only",
            "Draft",
            "draft",
            PolicyScope::Workspace,
            "1",
            PolicySeverity::Low,
            PolicyDefinitionStatus::Draft,
        ));
        let env = WorkspaceStateEnvelope::compose_deterministic(
            "ws",
            "t0",
            vec![WorkspaceStateSource::unavailable(
                "execution",
                "t0",
                "Execution unavailable",
            )],
            vec![WorkspaceStateConflict::record_deterministic(
                "planning",
                "reasoning",
                "conflict",
                "high",
                vec!["a".into()],
            )],
            vec!["u".into()],
        )
        .unwrap();
        let (evals, _) = evaluate_policies(&policies, Some(&env), "t0").unwrap();
        assert!(!evals.iter().any(|e| e.policy_id == "policy:draft_only"));
        let agg = aggregate_results(&evals);
        assert!(
            agg.restrictiveness() >= PolicyEvaluationResult::RequiresReview.restrictiveness()
        );
    }

    #[test]
    fn severity_ordering_evaluates_critical_first_in_catalog_sort() {
        let policies = default_policy_catalog();
        let mut active: Vec<_> = policies
            .iter()
            .filter(|p| p.status.is_evaluable())
            .cloned()
            .collect();
        active.sort_by(|a, b| b.severity.rank().cmp(&a.severity.rank()));
        assert_eq!(active[0].severity, PolicySeverity::Critical);
    }

    #[test]
    fn version_isolation_changes_catalog_revision() {
        let v1 = default_policy_catalog();
        let mut v2 = default_policy_catalog();
        v2[0].version = "2".into();
        assert_ne!(policy_catalog_revision(&v1), policy_catalog_revision(&v2));
    }

    #[test]
    fn recommendation_never_grants_authority() {
        let policies = default_policy_catalog();
        let (evals, _) = evaluate_policies(&policies, None, "t0").unwrap();
        let rec = GovernanceRecommendation::from_evaluations(&evals, "t0").unwrap();
        assert!(rec.is_non_actionable());
        assert_eq!(rec.authority_effect, "none");
        assert_eq!(rec.aggregate_result, PolicyEvaluationResult::Unknown);
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut meta = PolicyGovernanceMeta::new(
            "ws",
            "t0",
            None,
            "policies:1",
            0,
            "unknown",
        );
        meta.mark_superseded("t1");
        let hist = PolicyGovernanceHistoryEntry::from_meta(&meta).unwrap();
        assert!(hist.is_non_actionable());
        let snap = PolicyGovernanceSnapshot::assemble("ws", None, vec![hist], 1, "t1");
        assert!(snap.is_non_commandable());
    }
}
