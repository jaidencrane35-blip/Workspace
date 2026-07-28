//! Workspace Decision Support — Programme III Batch 11.
//!
//! Support decisions. Never become the decision-maker.
//! support ≠ decision; trade-off ≠ recommendation; comparison ≠ ranking-as-authority.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_cross_intelligence::CrossWorkspaceIntelligenceProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_insight_coordination::InsightCoordinationProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecisionSupportError {
    #[error("invalid decision support status: {0}")]
    InvalidStatus(String),

    #[error("invalid decision support completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid decision dependency kind: {0}")]
    InvalidDependencyKind(String),

    #[error("decision support artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("decision support artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("decision support bundles and dependencies require evidence references")]
    RequiresEvidence,

    #[error("decision support snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionSupportStatus {
    Current,
    Superseded,
    Archived,
}

impl DecisionSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionSupportError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(DecisionSupportError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionSupportCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl DecisionSupportCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionSupportError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(DecisionSupportError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame constraining which upstream surfaces participate in decision support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportFrame {
    pub focus: Option<String>,
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub include_knowledge_synthesis: bool,
    pub include_knowledge_integration: bool,
    pub include_insight: bool,
    pub include_cross_workspace: bool,
}

impl DecisionSupportFrame {
    pub fn all_surfaces() -> Self {
        Self {
            focus: None,
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_synthesis: true,
            include_knowledge_integration: true,
            include_insight: true,
            include_cross_workspace: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DecisionSupportEvidenceRef {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn link(
        origin_domain: impl Into<String>,
        external_ref: impl Into<String>,
        source_revision: Option<String>,
    ) -> Self {
        Self {
            external_ref: external_ref.into(),
            origin_domain: origin_domain.into(),
            source_revision,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Decision-ready context package — descriptive evidence only; no decision outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportContext {
    pub context_id: String,
    pub decision_identifier: String,
    pub scope: String,
    pub participating_evidence: Vec<DecisionSupportEvidenceRef>,
    pub lineage: Vec<String>,
    pub completeness: String,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DecisionSupportContext {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "decision_context:";

    pub fn assemble(
        workspace_id: impl Into<String>,
        participating_evidence: Vec<DecisionSupportEvidenceRef>,
        completeness: impl Into<String>,
        uncertainty: Vec<String>,
        lineage: Vec<String>,
    ) -> Result<Self, DecisionSupportError> {
        if participating_evidence.is_empty() {
            return Err(DecisionSupportError::RequiresEvidence);
        }
        let workspace_id = workspace_id.into();
        let digest = stable_digest(&format!(
            "{workspace_id}|{}",
            participating_evidence
                .iter()
                .map(|r| r.external_ref.clone())
                .collect::<Vec<_>>()
                .join(",")
        ));
        Ok(Self {
            context_id: format!("{}{}", Self::ID_PREFIX, digest),
            decision_identifier: format!("workspace:{workspace_id}:support_context"),
            scope: format!("workspace:{workspace_id}"),
            participating_evidence,
            lineage,
            completeness: completeness.into(),
            uncertainty,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self
                .participating_evidence
                .iter()
                .all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: String,
    pub supporting: Vec<DecisionSupportEvidenceRef>,
    pub conflicting: Vec<DecisionSupportEvidenceRef>,
    pub unavailable: Vec<DecisionSupportEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceBundle {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_bundle:";

    pub fn collect(
        supporting: Vec<DecisionSupportEvidenceRef>,
        conflicting: Vec<DecisionSupportEvidenceRef>,
        unavailable: Vec<DecisionSupportEvidenceRef>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "support={}|conflict={}|unavail={}",
            supporting.len(),
            conflicting.len(),
            unavailable.len()
        ));
        Self {
            bundle_id: format!("{}{}", Self::ID_PREFIX, digest),
            supporting,
            conflicting,
            unavailable,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.supporting.iter().all(|r| r.is_non_actionable())
            && self.conflicting.iter().all(|r| r.is_non_actionable())
            && self.unavailable.iter().all(|r| r.is_non_actionable())
    }
}

/// Descriptive trade-off summary — never recommendation or ranking-as-authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeoffSummary {
    pub tradeoff_id: String,
    pub title: String,
    pub description: String,
    pub competing_constraints: Vec<String>,
    pub competing_objectives: Vec<String>,
    pub competing_policies: Vec<String>,
    pub evidence_quality_notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl TradeoffSummary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "tradeoff_summary:";

    pub fn describe(
        title: impl Into<String>,
        description: impl Into<String>,
        competing_constraints: Vec<String>,
        competing_objectives: Vec<String>,
        competing_policies: Vec<String>,
        evidence_quality_notes: Vec<String>,
    ) -> Self {
        let title = title.into();
        let description = description.into();
        let digest = stable_digest(&format!("{title}|{description}"));
        Self {
            tradeoff_id: format!("{}{}", Self::ID_PREFIX, digest),
            title,
            description,
            competing_constraints,
            competing_objectives,
            competing_policies,
            evidence_quality_notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Meaning-only dependency between evidence artefacts — never execution linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionDependency {
    pub dependency_id: String,
    pub from_ref: String,
    pub to_ref: String,
    pub kind: String,
    pub evidence_refs: Vec<DecisionSupportEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DecisionDependency {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "decision_dependency:";

    pub const ALLOWED_KINDS: &'static [&'static str] = &[
        "depends_on_evidence",
        "relates_to",
        "supported_by",
        "references",
    ];

    pub const FORBIDDEN_KINDS: &'static [&'static str] = &[
        "should_execute",
        "requires_action",
        "causes",
        "triggers",
        "authorises",
    ];

    pub fn link(
        from_ref: impl Into<String>,
        to_ref: impl Into<String>,
        kind: impl Into<String>,
        evidence_refs: Vec<DecisionSupportEvidenceRef>,
    ) -> Result<Self, DecisionSupportError> {
        if evidence_refs.is_empty() {
            return Err(DecisionSupportError::RequiresEvidence);
        }
        let kind = kind.into();
        validate_dependency_kind(&kind)?;
        let from_ref = from_ref.into();
        let to_ref = to_ref.into();
        let digest = stable_digest(&format!("{from_ref}|{to_ref}|{kind}"));
        Ok(Self {
            dependency_id: format!("{}{}", Self::ID_PREFIX, digest),
            from_ref,
            to_ref,
            kind,
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportGap {
    pub gap_id: String,
    pub missing_evidence: String,
    pub affected_surfaces: Vec<String>,
    pub uncertainty_explanation: String,
    pub severity: String,
    pub evidence_refs: Vec<DecisionSupportEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DecisionSupportGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "decision_support_gap:";

    pub fn record(
        missing_evidence: impl Into<String>,
        affected_surfaces: Vec<String>,
        uncertainty_explanation: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<DecisionSupportEvidenceRef>,
    ) -> Self {
        let missing_evidence = missing_evidence.into();
        let uncertainty_explanation = uncertainty_explanation.into();
        let digest = stable_digest(&format!(
            "{missing_evidence}|{}",
            affected_surfaces.join(",")
        ));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            missing_evidence,
            affected_surfaces,
            uncertainty_explanation,
            severity: severity.into(),
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic coverage rollup — not a decision scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportAssessment {
    pub assessment_id: String,
    pub coverage: u8,
    pub supporting_count: usize,
    pub conflicting_count: usize,
    pub unavailable_count: usize,
    pub gap_count: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DecisionSupportAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "decision_support_assessment:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Derived decision support artefact — understanding only; never a decision outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDecisionSupportSnapshot {
    pub support_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: DecisionSupportStatus,
    pub superseded_at: Option<String>,
    pub frame: DecisionSupportFrame,
    pub source_revisions: Vec<String>,
    pub contexts: Vec<DecisionSupportContext>,
    pub evidence_bundles: Vec<EvidenceBundle>,
    pub tradeoffs: Vec<TradeoffSummary>,
    pub dependencies: Vec<DecisionDependency>,
    pub gaps: Vec<DecisionSupportGap>,
    pub assessment: DecisionSupportAssessment,
    pub completeness: DecisionSupportCompleteness,
    pub provenance_links: Vec<DecisionSupportEvidenceRef>,
    pub summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceDecisionSupportSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "decision_support:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: DecisionSupportFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        insight: Option<&InsightCoordinationProjection>,
        cross_workspace: Option<&CrossWorkspaceIntelligenceProjection>,
    ) -> Result<Self, DecisionSupportError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut supporting_refs = Vec::new();
        let mut conflicting_refs = Vec::new();
        let mut unavailable_refs = Vec::new();
        let mut available_surfaces: Vec<(String, String, DecisionSupportEvidenceRef)> = Vec::new();
        let mut tradeoffs = Vec::new();
        let mut dependencies = Vec::new();
        let mut requested = 0usize;
        let mut conflict_count = 0usize;
        let mut policy_present = false;
        let mut contextual_present = false;
        let mut knowledge_present = false;
        let mut insight_present = false;

        let mut push_available = |domain: &str,
                                  artefact_id: String,
                                  revision: Option<String>,
                                  is_conflicting: bool| {
            let rev = revision.clone().unwrap_or_else(|| artefact_id.clone());
            source_revisions.push(format!("{domain}:{rev}"));
            let evidence = DecisionSupportEvidenceRef::link(domain, artefact_id.clone(), Some(rev));
            provenance_links.push(evidence.clone());
            available_surfaces.push((domain.into(), artefact_id, evidence.clone()));
            if is_conflicting {
                conflicting_refs.push(evidence.clone());
            } else {
                supporting_refs.push(evidence);
            }
            Ok::<(), DecisionSupportError>(())
        };

        if frame.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    let conflicting = envelope.consistency.as_str() == "contradictory";
                    if conflicting {
                        conflict_count += envelope.contradictions.len().max(1);
                    }
                    push_available(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                        conflicting,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable workspace state envelope available",
                        vec!["workspace_state_envelope".into()],
                        "Missing source remains Missing — never invent state structure",
                        "high",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "workspace_state_envelope",
                        "missing:workspace_state_envelope",
                        None,
                    ));
                }
            }
        }

        if frame.include_policy {
            requested += 1;
            match policy.and_then(|p| p.current.as_ref()) {
                Some(view) => {
                    policy_present = true;
                    let rev = view
                        .context_revision
                        .clone()
                        .unwrap_or_else(|| view.meta.evaluation_set_id.clone());
                    push_available(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable policy evaluation available",
                        vec!["policy_governance".into()],
                        "Unavailable policy evidence remains Unavailable",
                        "high",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "policy_governance",
                        "missing:policy_governance",
                        None,
                    ));
                }
            }
        }

        if frame.include_reconstruction {
            requested += 1;
            match reconstruction.and_then(|r| r.current.as_ref()) {
                Some(view) => {
                    let conflicting = view.completeness.as_str() == "contradictory";
                    if conflicting {
                        conflict_count += 1;
                    }
                    let rev = view
                        .to_revision
                        .clone()
                        .unwrap_or_else(|| view.reconstruction_id.clone());
                    push_available(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev),
                        conflicting,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable historical reconstruction available",
                        vec!["historical_reconstruction".into()],
                        "Missing reconstruction remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "historical_reconstruction",
                        "missing:historical_reconstruction",
                        None,
                    ));
                }
            }
        }

        if frame.include_temporal {
            requested += 1;
            match temporal.and_then(|t| t.current.as_ref()) {
                Some(view) => {
                    push_available(
                        "temporal_intelligence",
                        view.analysis_id.clone(),
                        Some(view.analysis_id.clone()),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable temporal analysis available",
                        vec!["temporal_intelligence".into()],
                        "Missing temporal evidence remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "temporal_intelligence",
                        "missing:temporal_intelligence",
                        None,
                    ));
                }
            }
        }

        if frame.include_explanation {
            requested += 1;
            match explanation.and_then(|e| e.current.as_ref()) {
                Some(pkg) => {
                    push_available(
                        "workspace_explanation",
                        pkg.explanation_id.clone(),
                        Some(pkg.explanation_id.clone()),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable explanation package available",
                        vec!["workspace_explanation".into()],
                        "Missing explanation remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "workspace_explanation",
                        "missing:workspace_explanation",
                        None,
                    ));
                }
            }
        }

        if frame.include_contextual {
            requested += 1;
            match contextual.and_then(|c| c.current.as_ref()) {
                Some(snap) => {
                    contextual_present = true;
                    push_available(
                        "contextual_understanding",
                        snap.understanding_id.clone(),
                        Some(snap.understanding_id.clone()),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable contextual understanding available",
                        vec!["contextual_understanding".into()],
                        "Missing contextual evidence remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "contextual_understanding",
                        "missing:contextual_understanding",
                        None,
                    ));
                }
            }
        }

        if frame.include_knowledge_synthesis {
            requested += 1;
            match knowledge_synthesis.and_then(|k| k.current.as_ref()) {
                Some(synth) => {
                    knowledge_present = true;
                    push_available(
                        "knowledge_synthesis",
                        synth.synthesis_id.clone(),
                        Some(synth.synthesis_id.clone()),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable knowledge synthesis available",
                        vec!["knowledge_synthesis".into()],
                        "Missing synthesis remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "knowledge_synthesis",
                        "missing:knowledge_synthesis",
                        None,
                    ));
                }
            }
        }

        if frame.include_knowledge_integration {
            requested += 1;
            match knowledge_integration.and_then(|k| k.current.as_ref()) {
                Some(integ) => {
                    knowledge_present = true;
                    push_available(
                        "knowledge_integration",
                        integ.integration_id.clone(),
                        Some(integ.integration_id.clone()),
                        false,
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable knowledge integration available",
                        vec!["knowledge_integration".into()],
                        "Missing integration remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "knowledge_integration",
                        "missing:knowledge_integration",
                        None,
                    ));
                }
            }
        }

        if frame.include_insight {
            requested += 1;
            match insight.and_then(|i| i.current.as_ref()) {
                Some(coord) => {
                    insight_present = true;
                    push_available(
                        "insight_coordination",
                        coord.coordination_id.clone(),
                        Some(coord.coordination_id.clone()),
                        coord.completeness.as_str() == "contradictory",
                    )?;
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable insight coordination available",
                        vec!["insight_coordination".into()],
                        "Missing insight coordination remains Missing",
                        "medium",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "insight_coordination",
                        "missing:insight_coordination",
                        None,
                    ));
                }
            }
        }

        if frame.include_cross_workspace {
            requested += 1;
            match cross_workspace.and_then(|c| c.current.as_ref()) {
                Some(intel) => {
                    let evidence = DecisionSupportEvidenceRef::link(
                        "cross_workspace_intelligence",
                        intel.intelligence_id.clone(),
                        Some(intel.intelligence_id.clone()),
                    );
                    source_revisions.push(format!(
                        "cross_workspace_intelligence:{}",
                        intel.intelligence_id
                    ));
                    provenance_links.push(evidence.clone());
                    supporting_refs.push(evidence);
                }
                None => {
                    gaps.push(DecisionSupportGap::record(
                        "No durable cross-workspace intelligence available",
                        vec!["cross_workspace_intelligence".into()],
                        "Missing cross-workspace evidence remains Missing — reference only when present",
                        "low",
                        vec![],
                    ));
                    unavailable_refs.push(DecisionSupportEvidenceRef::link(
                        "cross_workspace_intelligence",
                        "missing:cross_workspace_intelligence",
                        None,
                    ));
                }
            }
        }

        if policy_present && contextual_present {
            tradeoffs.push(TradeoffSummary::describe(
                "Policy and contextual constraint overlap",
                "Policy governance evaluation and contextual understanding are both present. \
                 Competing constraints may be described without selecting an action.",
                vec![
                    "Policy evaluation expresses authority constraints — support does not grant permission".into(),
                    "Contextual themes describe situational factors — not a decision outcome".into(),
                ],
                vec!["Understand policy posture alongside situational context".into()],
                vec!["Policy aggregate result vs contextual theme emphasis".into()],
                vec![],
            ));
        }

        if knowledge_present && insight_present {
            tradeoffs.push(TradeoffSummary::describe(
                "Knowledge and insight evidence quality",
                "Knowledge synthesis/integration and insight coordination are both present. \
                 Evidence quality notes are descriptive — never a recommendation.",
                vec![],
                vec![],
                vec![],
                vec![
                    "Derived knowledge concepts remain non-authoritative".into(),
                    "Insight coordination clusters describe overlap — not ranked choices".into(),
                    "trade-off ≠ recommendation".into(),
                ],
            ));
        }

        for i in 0..available_surfaces.len() {
            for j in (i + 1)..available_surfaces.len() {
                let (d1, id1, ref1) = &available_surfaces[i];
                let (d2, id2, ref2) = &available_surfaces[j];
                let kind = if (*d1 == "policy_governance" && *d2 == "contextual_understanding")
                    || (*d1 == "contextual_understanding" && *d2 == "policy_governance")
                {
                    "depends_on_evidence"
                } else if (*d1 == "knowledge_synthesis" && *d2 == "insight_coordination")
                    || (*d1 == "insight_coordination" && *d2 == "knowledge_synthesis")
                    || (*d1 == "knowledge_integration" && *d2 == "insight_coordination")
                    || (*d1 == "insight_coordination" && *d2 == "knowledge_integration")
                {
                    "supported_by"
                } else {
                    "relates_to"
                };
                dependencies.push(DecisionDependency::link(
                    id1.clone(),
                    id2.clone(),
                    kind,
                    vec![ref1.clone(), ref2.clone()],
                )?);
            }
        }

        let available = available_surfaces.len();
        let evidence_bundles = vec![EvidenceBundle::collect(
            supporting_refs.clone(),
            conflicting_refs.clone(),
            unavailable_refs.clone(),
        )];

        let mut contexts = Vec::new();
        if !supporting_refs.is_empty() {
            contexts.push(DecisionSupportContext::assemble(
                &workspace_id,
                supporting_refs.clone(),
                derive_completeness(requested, available, &gaps, conflict_count).as_str(),
                vec![
                    "Decision context organises evidence — it does not decide".into(),
                    "support ≠ decision".into(),
                ],
                source_revisions.clone(),
            )?);
        }

        let completeness = derive_completeness(requested, available, &gaps, conflict_count);
        let coverage = if requested == 0 {
            0
        } else {
            ((available * 100) / requested) as u8
        };
        let assessment = DecisionSupportAssessment {
            assessment_id: format!(
                "{}{}",
                DecisionSupportAssessment::ID_PREFIX,
                stable_digest(&format!("{workspace_id}|{available}|{requested}|{}", gaps.len()))
            ),
            coverage,
            supporting_count: supporting_refs.len(),
            conflicting_count: conflicting_refs.len(),
            unavailable_count: unavailable_refs.len(),
            gap_count: gaps.len(),
            uncertainty: vec![
                "Decision support assessment is diagnostic only".into(),
                "completeness ≠ permission".into(),
            ],
            limitations: vec![
                "support ≠ decision".into(),
                "trade-off ≠ recommendation".into(),
                "comparison ≠ ranking-as-authority".into(),
                "confidence ≠ approval".into(),
            ],
            authority_effect: DecisionSupportAssessment::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        };

        let summary = format!(
            "Decision support: {} context(s), {} bundle(s), {} tradeoff(s), {} gap(s), completeness={}",
            contexts.len(),
            evidence_bundles.len(),
            tradeoffs.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Organised decision-ready evidence for workspace {workspace_id}. \
             Available surfaces={available}/{requested}. Conflicts preserved={conflict_count}. \
             This layer supports understanding only — it never becomes the decision-maker."
        );
        let limitations = vec![
            "Workspace Decision Support organises evidence for human and system understanding only".into(),
            "It does not make decisions, recommendations, or execute actions".into(),
            "It does not replace Policy, Decision Engine, Recommendation Engine, or human judgement".into(),
            "Trade-off summaries are descriptive — not ranked choices".into(),
        ];

        let mut result = Self {
            support_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    source_revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: DecisionSupportStatus::Current,
            superseded_at: None,
            frame,
            source_revisions,
            contexts,
            evidence_bundles,
            tradeoffs,
            dependencies,
            gaps,
            assessment,
            completeness,
            provenance_links,
            summary,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        result.validate()?;
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: DecisionSupportFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        insight: Option<&InsightCoordinationProjection>,
        cross_workspace: Option<&CrossWorkspaceIntelligenceProjection>,
    ) -> Result<Self, DecisionSupportError> {
        let mut snap = Self::compose(
            workspace_id,
            generated_at,
            frame,
            state,
            policy,
            reconstruction,
            temporal,
            explanation,
            contextual,
            knowledge_synthesis,
            knowledge_integration,
            insight,
            cross_workspace,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.contexts
                .iter()
                .map(|c| c.context_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.evidence_bundles
                .iter()
                .map(|b| b.bundle_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.completeness.as_str()
        ));
        snap.support_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = DecisionSupportStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.contexts.iter().all(|c| c.is_non_actionable())
            && self
                .evidence_bundles
                .iter()
                .all(|b| b.is_non_actionable())
            && self.tradeoffs.iter().all(|t| t.is_non_actionable())
            && self.dependencies.iter().all(|d| d.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.assessment.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), DecisionSupportError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(DecisionSupportError::AuthorityEffectMustBeNone);
        }
        for ctx in &self.contexts {
            if ctx.participating_evidence.is_empty() || ctx.actionable {
                return Err(DecisionSupportError::RequiresEvidence);
            }
        }
        for dep in &self.dependencies {
            validate_dependency_kind(&dep.kind)?;
            if dep.evidence_refs.is_empty() || dep.actionable {
                return Err(DecisionSupportError::RequiresEvidence);
            }
        }
        if self.gaps.iter().any(|g| g.actionable) || self.assessment.actionable {
            return Err(DecisionSupportError::MustNotBeActionable);
        }
        if self.tradeoffs.iter().any(|t| t.actionable) {
            return Err(DecisionSupportError::MustNotBeActionable);
        }
        let forbidden_phrases = [
            "best choice",
            "recommended option",
            "optimal path",
            "do this",
            "approve this",
            "recommend that you",
        ];
        let blob = format!(
            "{} {} {} {}",
            self.summary,
            self.narrative,
            self.tradeoffs
                .iter()
                .map(|t| format!("{} {}", t.title, t.description))
                .collect::<Vec<_>>()
                .join(" "),
            self.contexts
                .iter()
                .map(|c| c.decision_identifier.clone())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .to_lowercase();
        for phrase in forbidden_phrases {
            if blob.contains(phrase) {
                return Err(DecisionSupportError::MustNotBeActionable);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionSupportHistoryEntry {
    pub support_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub context_count: usize,
    pub bundle_count: usize,
    pub tradeoff_count: usize,
    pub dependency_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl DecisionSupportHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceDecisionSupportSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            support_id: snap.support_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            context_count: snap.contexts.len(),
            bundle_count: snap.evidence_bundles.len(),
            tradeoff_count: snap.tradeoffs.len(),
            dependency_count: snap.dependencies.len(),
            gap_count: snap.gaps.len(),
            source_revision_count: snap.source_revisions.len(),
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
pub struct WorkspaceDecisionSupportProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceDecisionSupportSnapshot>,
    pub history: Vec<DecisionSupportHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl WorkspaceDecisionSupportProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceDecisionSupportSnapshot>,
        history: Vec<DecisionSupportHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceDecisionSupportSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        WorkspaceDecisionSupportSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            support_id: self.current.as_ref().map(|c| c.support_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            context_count: self.current.as_ref().map(|c| c.contexts.len()).unwrap_or(0),
            bundle_count: self
                .current
                .as_ref()
                .map(|c| c.evidence_bundles.len())
                .unwrap_or(0),
            tradeoff_count: self.current.as_ref().map(|c| c.tradeoffs.len()).unwrap_or(0),
            dependency_count: self
                .current
                .as_ref()
                .map(|c| c.dependencies.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            source_revision_count: self
                .current
                .as_ref()
                .map(|c| c.source_revisions.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDecisionSupportSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub support_id: Option<String>,
    pub completeness: Option<String>,
    pub context_count: usize,
    pub bundle_count: usize,
    pub tradeoff_count: usize,
    pub dependency_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<DecisionSupportHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDecisionSupportExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub summary: Option<String>,
    pub context_summaries: Vec<String>,
    pub bundle_summaries: Vec<String>,
    pub tradeoff_summaries: Vec<String>,
    pub dependency_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceDecisionSupportExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceDecisionSupportSnapshot) -> Self {
        Self {
            explanation_id: format!("decision_support_explanation:{}", snap.support_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            summary: Some(snap.summary.clone()),
            context_summaries: snap
                .contexts
                .iter()
                .map(|c| {
                    format!(
                        "{} [{}] evidence={}",
                        c.decision_identifier,
                        c.context_id,
                        c.participating_evidence.len()
                    )
                })
                .collect(),
            bundle_summaries: snap
                .evidence_bundles
                .iter()
                .map(|b| {
                    format!(
                        "supporting={} conflicting={} unavailable={} [{}]",
                        b.supporting.len(),
                        b.conflicting.len(),
                        b.unavailable.len(),
                        b.bundle_id
                    )
                })
                .collect(),
            tradeoff_summaries: snap
                .tradeoffs
                .iter()
                .map(|t| format!("{} [{}]", t.title, t.tradeoff_id))
                .collect(),
            dependency_summaries: snap
                .dependencies
                .iter()
                .map(|d| format!("{}→{} kind={}", d.from_ref, d.to_ref, d.kind))
                .collect(),
            gaps: snap
                .gaps
                .iter()
                .map(|g| g.missing_evidence.clone())
                .collect(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: snap.assessment.uncertainty.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[DecisionSupportGap],
    conflict_count: usize,
) -> DecisionSupportCompleteness {
    if requested == 0 || available == 0 {
        return DecisionSupportCompleteness::Unavailable;
    }
    if conflict_count > 0 {
        return DecisionSupportCompleteness::Contradictory;
    }
    if available == requested && gaps.is_empty() {
        return DecisionSupportCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() {
        return DecisionSupportCompleteness::Partial;
    }
    DecisionSupportCompleteness::Unknown
}

pub fn validate_dependency_kind(kind: &str) -> Result<(), DecisionSupportError> {
    let lower = kind.to_lowercase();
    if DecisionDependency::FORBIDDEN_KINDS
        .iter()
        .any(|f| *f == lower)
    {
        return Err(DecisionSupportError::InvalidDependencyKind(kind.into()));
    }
    if DecisionDependency::ALLOWED_KINDS.iter().any(|a| *a == lower) {
        return Ok(());
    }
    Err(DecisionSupportError::InvalidDependencyKind(kind.into()))
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

    #[test]
    fn missing_upstreams_unavailable() {
        let snap = WorkspaceDecisionSupportSnapshot::compose(
            "ws",
            "t0",
            DecisionSupportFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            snap.completeness,
            DecisionSupportCompleteness::Unavailable
        );
        assert!(snap.contexts.is_empty());
        assert_eq!(snap.gaps.len(), 10);
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = WorkspaceDecisionSupportSnapshot::compose_deterministic(
            "ws",
            "t1",
            DecisionSupportFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let right = WorkspaceDecisionSupportSnapshot::compose_deterministic(
            "ws",
            "t1",
            DecisionSupportFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.support_id, right.support_id);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.completeness, right.completeness);
    }

    #[test]
    fn forbidden_dependency_kinds_rejected() {
        for kind in DecisionDependency::FORBIDDEN_KINDS {
            let err = DecisionDependency::link(
                "a",
                "b",
                *kind,
                vec![DecisionSupportEvidenceRef::link("test", "ref", None)],
            )
            .unwrap_err();
            assert_eq!(
                err,
                DecisionSupportError::InvalidDependencyKind((*kind).into())
            );
        }
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = WorkspaceDecisionSupportSnapshot::compose(
            "ws",
            "t0",
            DecisionSupportFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        snap.mark_superseded("t1");
        let entry = DecisionSupportHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj = WorkspaceDecisionSupportProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn evidence_bundle_ok() {
        let snap = WorkspaceDecisionSupportSnapshot::compose(
            "ws",
            "t0",
            DecisionSupportFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(snap.evidence_bundles.len(), 1);
        let bundle = &snap.evidence_bundles[0];
        assert!(bundle.supporting.is_empty());
        assert!(!bundle.unavailable.is_empty());
        assert!(bundle.is_non_actionable());
    }

    #[test]
    fn tradeoff_descriptive_not_recommend() {
        let mut snap = WorkspaceDecisionSupportSnapshot::compose(
            "ws",
            "t0",
            DecisionSupportFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        snap.narrative = "The recommended option is clearly superior.".into();
        assert!(snap.validate().is_err());
    }

    #[test]
    fn assessment_non_actionable() {
        let snap = WorkspaceDecisionSupportSnapshot::compose(
            "ws",
            "t0",
            DecisionSupportFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(snap.assessment.is_non_actionable());
        assert!(!snap.assessment.actionable);
        assert_eq!(snap.assessment.authority_effect, "none");
    }
}
