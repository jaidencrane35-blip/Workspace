//! Workspace Intelligence Hub — Programme III Batch 12.
//!
//! Aggregate intelligence. Never replace the intelligence that produced it.
//! aggregation ≠ reinterpretation; hub package ≠ new SoT; conflict record ≠ resolution.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_cross_intelligence::CrossWorkspaceIntelligenceProjection;
use crate::workspace_decision_support::WorkspaceDecisionSupportProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_insight_coordination::InsightCoordinationProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IntelligenceHubError {
    #[error("invalid intelligence hub status: {0}")]
    InvalidStatus(String),

    #[error("invalid intelligence hub completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid package availability: {0}")]
    InvalidAvailability(String),

    #[error("intelligence hub artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("intelligence hub artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("available intelligence packages require evidence references")]
    RequiresEvidence,

    #[error("intelligence hub snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelligenceHubStatus {
    Current,
    Superseded,
    Archived,
}

impl IntelligenceHubStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, IntelligenceHubError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(IntelligenceHubError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelligenceHubCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl IntelligenceHubCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, IntelligenceHubError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(IntelligenceHubError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame constraining which upstream Programme III surfaces participate in hub aggregation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceHubFrame {
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
    pub include_decision_support: bool,
}

impl IntelligenceHubFrame {
    pub fn all_surfaces() -> Self {
        Self {
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
            include_decision_support: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceHubEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceHubEvidenceRef {
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

/// Reference-only intelligence package — never copies upstream authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligencePackage {
    pub package_id: String,
    pub surface: String,
    pub artefact_ref: String,
    pub source_revision: Option<String>,
    pub availability: String,
    pub evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligencePackage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_package:";

    pub const AVAILABILITY_AVAILABLE: &'static str = "available";
    pub const AVAILABILITY_UNAVAILABLE: &'static str = "unavailable";
    pub const AVAILABILITY_STALE: &'static str = "stale";
    pub const AVAILABILITY_UNKNOWN: &'static str = "unknown";

    pub fn reference(
        surface: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        availability: impl Into<String>,
        evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    ) -> Result<Self, IntelligenceHubError> {
        let availability = availability.into();
        validate_availability(&availability)?;
        let surface = surface.into();
        let artefact_ref = artefact_ref.into();
        if availability == Self::AVAILABILITY_AVAILABLE && evidence_refs.is_empty() {
            return Err(IntelligenceHubError::RequiresEvidence);
        }
        let digest = stable_digest(&format!("{surface}|{artefact_ref}|{availability}"));
        Ok(Self {
            package_id: format!("{}{}", Self::ID_PREFIX, digest),
            surface,
            artefact_ref,
            source_revision,
            availability,
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

/// Diagnostic rollup of surface availability — never a recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceSummary {
    pub summary_id: String,
    pub available_surfaces: Vec<String>,
    pub unavailable_surfaces: Vec<String>,
    pub stale_surfaces: Vec<String>,
    pub conflicting_surfaces: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceSummary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_summary:";

    pub fn assemble(
        available_surfaces: Vec<String>,
        unavailable_surfaces: Vec<String>,
        stale_surfaces: Vec<String>,
        conflicting_surfaces: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "avail={}|unavail={}|stale={}|conflict={}",
            available_surfaces.len(),
            unavailable_surfaces.len(),
            stale_surfaces.len(),
            conflicting_surfaces.len()
        ));
        Self {
            summary_id: format!("{}{}", Self::ID_PREFIX, digest),
            available_surfaces,
            unavailable_surfaces,
            stale_surfaces,
            conflicting_surfaces,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Provenance rollup from upstream references only — no inferred lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceLineage {
    pub lineage_id: String,
    pub upstream_sources: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_lineage:";

    pub fn rollup(
        upstream_sources: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "{}|{}",
            upstream_sources.join(","),
            revisions.join(",")
        ));
        Self {
            lineage_id: format!("{}{}", Self::ID_PREFIX, digest),
            upstream_sources,
            revisions,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceGap {
    pub gap_id: String,
    pub missing_evidence: String,
    pub affected_surfaces: Vec<String>,
    pub uncertainty_explanation: String,
    pub severity: String,
    pub evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_hub_gap:";

    pub fn record(
        missing_evidence: impl Into<String>,
        affected_surfaces: Vec<String>,
        uncertainty_explanation: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<IntelligenceHubEvidenceRef>,
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

/// Conflict record — preserved, never resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceConflict {
    pub conflict_id: String,
    pub description: String,
    pub participating_surfaces: Vec<String>,
    pub evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceConflict {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_conflict:";

    pub fn record(
        description: impl Into<String>,
        participating_surfaces: Vec<String>,
        evidence_refs: Vec<IntelligenceHubEvidenceRef>,
    ) -> Self {
        let description = description.into();
        let digest = stable_digest(&format!(
            "{description}|{}",
            participating_surfaces.join(",")
        ));
        Self {
            conflict_id: format!("{}{}", Self::ID_PREFIX, digest),
            description,
            participating_surfaces,
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
pub struct IntelligenceHubAssessment {
    pub assessment_id: String,
    pub coverage: u8,
    pub package_count: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntelligenceHubAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_hub_assessment:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Aggregated intelligence hub artefact — observational only; never upstream authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIntelligenceHubSnapshot {
    pub hub_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: IntelligenceHubStatus,
    pub superseded_at: Option<String>,
    pub frame: IntelligenceHubFrame,
    pub source_revisions: Vec<String>,
    pub packages: Vec<IntelligencePackage>,
    pub summary: IntelligenceSummary,
    pub lineage: IntelligenceLineage,
    pub gaps: Vec<IntelligenceGap>,
    pub conflicts: Vec<IntelligenceConflict>,
    pub assessment: IntelligenceHubAssessment,
    pub completeness: IntelligenceHubCompleteness,
    pub provenance_links: Vec<IntelligenceHubEvidenceRef>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceIntelligenceHubSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "intelligence_hub:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: IntelligenceHubFrame,
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
        decision_support: Option<&WorkspaceDecisionSupportProjection>,
    ) -> Result<Self, IntelligenceHubError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut acc = HubComposeAccumulator::default();

        if frame.include_state {
            acc.requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    let conflicting = envelope.consistency.as_str() == "contradictory";
                    if conflicting {
                        acc.conflicts.push(IntelligenceConflict::record(
                            format!(
                                "Workspace state envelope reports contradictory consistency: {}",
                                envelope.state_id
                            ),
                            vec!["workspace_state_envelope".into()],
                            vec![IntelligenceHubEvidenceRef::link(
                                "workspace_state_envelope",
                                envelope.state_id.clone(),
                                Some(envelope.revision.clone()),
                            )],
                        ));
                    }
                    acc.push_available(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                        conflicting,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "workspace_state_envelope",
                    "No durable workspace state envelope available",
                    "high",
                ),
            }
        }

        if frame.include_policy {
            acc.requested += 1;
            match policy.and_then(|p| p.current.as_ref()) {
                Some(view) => {
                    let rev = view
                        .context_revision
                        .clone()
                        .unwrap_or_else(|| view.meta.evaluation_set_id.clone());
                    acc.push_available(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "policy_governance",
                    "No durable policy evaluation available",
                    "high",
                ),
            }
        }

        if frame.include_reconstruction {
            acc.requested += 1;
            match reconstruction.and_then(|r| r.current.as_ref()) {
                Some(view) => {
                    let conflicting = view.completeness.as_str() == "contradictory";
                    if conflicting {
                        acc.conflicts.push(IntelligenceConflict::record(
                            format!(
                                "Historical reconstruction reports contradictory completeness: {}",
                                view.reconstruction_id
                            ),
                            vec!["historical_reconstruction".into()],
                            vec![IntelligenceHubEvidenceRef::link(
                                "historical_reconstruction",
                                view.reconstruction_id.clone(),
                                view.to_revision.clone(),
                            )],
                        ));
                    }
                    let rev = view
                        .to_revision
                        .clone()
                        .unwrap_or_else(|| view.reconstruction_id.clone());
                    acc.push_available(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev),
                        conflicting,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "historical_reconstruction",
                    "No durable historical reconstruction available",
                    "medium",
                ),
            }
        }

        if frame.include_temporal {
            acc.requested += 1;
            match temporal.and_then(|t| t.current.as_ref()) {
                Some(view) => {
                    acc.push_available(
                        "temporal_intelligence",
                        view.analysis_id.clone(),
                        Some(view.analysis_id.clone()),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "temporal_intelligence",
                    "No durable temporal analysis available",
                    "medium",
                ),
            }
        }

        if frame.include_explanation {
            acc.requested += 1;
            match explanation.and_then(|e| e.current.as_ref()) {
                Some(pkg) => {
                    acc.push_available(
                        "workspace_explanation",
                        pkg.explanation_id.clone(),
                        Some(pkg.explanation_id.clone()),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "workspace_explanation",
                    "No durable explanation package available",
                    "medium",
                ),
            }
        }

        if frame.include_contextual {
            acc.requested += 1;
            match contextual.and_then(|c| c.current.as_ref()) {
                Some(snap) => {
                    acc.push_available(
                        "contextual_understanding",
                        snap.understanding_id.clone(),
                        Some(snap.understanding_id.clone()),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "contextual_understanding",
                    "No durable contextual understanding available",
                    "medium",
                ),
            }
        }

        if frame.include_knowledge_synthesis {
            acc.requested += 1;
            match knowledge_synthesis.and_then(|k| k.current.as_ref()) {
                Some(synth) => {
                    acc.push_available(
                        "knowledge_synthesis",
                        synth.synthesis_id.clone(),
                        Some(synth.synthesis_id.clone()),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "knowledge_synthesis",
                    "No durable knowledge synthesis available",
                    "medium",
                ),
            }
        }

        if frame.include_knowledge_integration {
            acc.requested += 1;
            match knowledge_integration.and_then(|k| k.current.as_ref()) {
                Some(integ) => {
                    acc.push_available(
                        "knowledge_integration",
                        integ.integration_id.clone(),
                        Some(integ.integration_id.clone()),
                        false,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "knowledge_integration",
                    "No durable knowledge integration available",
                    "medium",
                ),
            }
        }

        if frame.include_insight {
            acc.requested += 1;
            match insight.and_then(|i| i.current.as_ref()) {
                Some(coord) => {
                    let conflicting = coord.completeness.as_str() == "contradictory";
                    if conflicting {
                        acc.conflicts.push(IntelligenceConflict::record(
                            format!(
                                "Insight coordination reports contradictory completeness: {}",
                                coord.coordination_id
                            ),
                            vec!["insight_coordination".into()],
                            vec![IntelligenceHubEvidenceRef::link(
                                "insight_coordination",
                                coord.coordination_id.clone(),
                                Some(coord.coordination_id.clone()),
                            )],
                        ));
                    }
                    acc.push_available(
                        "insight_coordination",
                        coord.coordination_id.clone(),
                        Some(coord.coordination_id.clone()),
                        conflicting,
                        false,
                    )?;
                }
                None => acc.push_missing(
                    "insight_coordination",
                    "No durable insight coordination available",
                    "medium",
                ),
            }
        }

        if frame.include_cross_workspace {
            acc.requested += 1;
            match cross_workspace.and_then(|c| c.current.as_ref()) {
                Some(intel) => {
                    let stale = intel.completeness.as_str() == "partial"
                        || intel.completeness.as_str() == "unknown";
                    acc.push_available(
                        "cross_workspace_intelligence",
                        intel.intelligence_id.clone(),
                        Some(intel.intelligence_id.clone()),
                        false,
                        stale,
                    )?;
                }
                None => acc.push_missing(
                    "cross_workspace_intelligence",
                    "No durable cross-workspace intelligence available",
                    "low",
                ),
            }
        }

        if frame.include_decision_support {
            acc.requested += 1;
            match decision_support.and_then(|d| d.current.as_ref()) {
                Some(support) => {
                    let conflicting = support.completeness.as_str() == "contradictory";
                    if conflicting {
                        acc.conflicts.push(IntelligenceConflict::record(
                            format!(
                                "Decision support reports contradictory completeness: {}",
                                support.support_id
                            ),
                            vec!["workspace_decision_support".into()],
                            vec![IntelligenceHubEvidenceRef::link(
                                "workspace_decision_support",
                                support.support_id.clone(),
                                Some(support.support_id.clone()),
                            )],
                        ));
                    }
                    let stale = support.completeness.as_str() == "partial"
                        || support.completeness.as_str() == "unknown";
                    acc.push_available(
                        "workspace_decision_support",
                        support.support_id.clone(),
                        Some(support.support_id.clone()),
                        conflicting,
                        stale,
                    )?;
                }
                None => acc.push_missing(
                    "workspace_decision_support",
                    "No durable decision support available",
                    "medium",
                ),
            }
        }

        let lineage = IntelligenceLineage::rollup(
            acc.lineage_sources,
            acc.lineage_revisions,
            acc.lineage_evidence,
        );

        let summary_notes = vec![
            "Intelligence hub summary is diagnostic only — aggregation ≠ reinterpretation".into(),
            "conflict record ≠ resolution".into(),
            "lineage ≠ inferred provenance".into(),
        ];
        let summary = IntelligenceSummary::assemble(
            acc.available_surfaces.clone(),
            acc.unavailable_surfaces.clone(),
            acc.stale_surfaces.clone(),
            acc.conflicting_surfaces.clone(),
            summary_notes,
        );

        let completeness = derive_completeness(
            acc.requested,
            acc.available_surfaces.len(),
            &acc.gaps,
            acc.conflict_count,
        );
        let coverage = if acc.requested == 0 {
            0
        } else {
            ((acc.available_surfaces.len() * 100) / acc.requested) as u8
        };
        let assessment = IntelligenceHubAssessment {
            assessment_id: format!(
                "{}{}",
                IntelligenceHubAssessment::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{coverage}|{}|{}|{}",
                    acc.packages.len(),
                    acc.gaps.len(),
                    acc.conflicts.len()
                ))
            ),
            coverage,
            package_count: acc.packages.len(),
            gap_count: acc.gaps.len(),
            conflict_count: acc.conflicts.len(),
            uncertainty: vec![
                "Intelligence hub assessment is diagnostic only".into(),
                "confidence ≠ permission".into(),
            ],
            limitations: vec![
                "aggregation ≠ reinterpretation".into(),
                "hub package ≠ new SoT".into(),
                "summary ≠ recommendation".into(),
                "conflict record ≠ resolution".into(),
            ],
            authority_effect: IntelligenceHubAssessment::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        };

        let available_count = acc.available_surfaces.len();
        let narrative_summary = format!(
            "Intelligence hub: {} package(s), {} gap(s), {} conflict(s), completeness={}",
            acc.packages.len(),
            acc.gaps.len(),
            acc.conflicts.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Aggregated Programme III intelligence for workspace {workspace_id}. \
             Available surfaces={available_count}/{}. Conflicts preserved={}. \
             This layer aggregates observational packages only — it never replaces upstream authority.",
            acc.requested,
            acc.conflict_count
        );
        let limitations = vec![
            "Workspace Intelligence Hub aggregates intelligence for observational understanding only".into(),
            "It does not make decisions, recommendations, or execute actions".into(),
            "It does not replace any upstream Programme III intelligence layer".into(),
            "Conflict records are preserved — never resolved by the hub".into(),
        ];

        let mut result = Self {
            hub_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    acc.source_revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: IntelligenceHubStatus::Current,
            superseded_at: None,
            frame,
            source_revisions: acc.source_revisions,
            packages: acc.packages,
            summary,
            lineage,
            gaps: acc.gaps,
            conflicts: acc.conflicts,
            assessment,
            completeness,
            provenance_links: acc.provenance_links,
            narrative_summary,
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
        frame: IntelligenceHubFrame,
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
        decision_support: Option<&WorkspaceDecisionSupportProjection>,
    ) -> Result<Self, IntelligenceHubError> {
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
            decision_support,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.packages
                .iter()
                .map(|p| p.package_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.conflicts
                .iter()
                .map(|c| c.conflict_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.completeness.as_str()
        ));
        snap.hub_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = IntelligenceHubStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.packages.iter().all(|p| p.is_non_actionable())
            && self.summary.is_non_actionable()
            && self.lineage.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.conflicts.iter().all(|c| c.is_non_actionable())
            && self.assessment.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), IntelligenceHubError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(IntelligenceHubError::AuthorityEffectMustBeNone);
        }
        for pkg in &self.packages {
            if pkg.availability == IntelligencePackage::AVAILABILITY_AVAILABLE
                && pkg.evidence_refs.is_empty()
            {
                return Err(IntelligenceHubError::RequiresEvidence);
            }
            if pkg.actionable {
                return Err(IntelligenceHubError::MustNotBeActionable);
            }
        }
        if self.gaps.iter().any(|g| g.actionable)
            || self.conflicts.iter().any(|c| c.actionable)
            || self.assessment.actionable
            || self.summary.actionable
        {
            return Err(IntelligenceHubError::MustNotBeActionable);
        }
        let forbidden_phrases = [
            "recommend that you",
            "best choice",
            "execute this",
            "approve this",
            "do this",
        ];
        let blob = format!(
            "{} {} {} {}",
            self.narrative_summary,
            self.narrative,
            self.summary.notes.join(" "),
            self.limitations.join(" ")
        )
        .to_lowercase();
        for phrase in forbidden_phrases {
            if blob.contains(phrase) {
                return Err(IntelligenceHubError::MustNotBeActionable);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceHubHistoryEntry {
    pub hub_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub package_count: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl IntelligenceHubHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceIntelligenceHubSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            hub_id: snap.hub_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            package_count: snap.packages.len(),
            gap_count: snap.gaps.len(),
            conflict_count: snap.conflicts.len(),
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
pub struct WorkspaceIntelligenceHubProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceIntelligenceHubSnapshot>,
    pub history: Vec<IntelligenceHubHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl WorkspaceIntelligenceHubProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceIntelligenceHubSnapshot>,
        history: Vec<IntelligenceHubHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceIntelligenceHubSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        WorkspaceIntelligenceHubSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            hub_id: self.current.as_ref().map(|c| c.hub_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            package_count: self.current.as_ref().map(|c| c.packages.len()).unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            conflict_count: self
                .current
                .as_ref()
                .map(|c| c.conflicts.len())
                .unwrap_or(0),
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
pub struct WorkspaceIntelligenceHubSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub hub_id: Option<String>,
    pub completeness: Option<String>,
    pub package_count: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<IntelligenceHubHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIntelligenceHubExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub package_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub conflict_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceIntelligenceHubExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceIntelligenceHubSnapshot) -> Self {
        Self {
            explanation_id: format!("intelligence_hub_explanation:{}", snap.hub_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            package_summaries: snap
                .packages
                .iter()
                .map(|p| {
                    format!(
                        "{} surface={} availability={} [{}]",
                        p.artefact_ref, p.surface, p.availability, p.package_id
                    )
                })
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| g.missing_evidence.clone())
                .collect(),
            conflict_summaries: snap
                .conflicts
                .iter()
                .map(|c| c.description.clone())
                .collect(),
            lineage_summaries: vec![format!(
                "upstream={} revisions={} [{}]",
                snap.lineage.upstream_sources.join(","),
                snap.lineage.revisions.len(),
                snap.lineage.lineage_id
            )],
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

#[derive(Default)]
struct HubComposeAccumulator {
    gaps: Vec<IntelligenceGap>,
    conflicts: Vec<IntelligenceConflict>,
    packages: Vec<IntelligencePackage>,
    provenance_links: Vec<IntelligenceHubEvidenceRef>,
    source_revisions: Vec<String>,
    lineage_sources: Vec<String>,
    lineage_revisions: Vec<String>,
    lineage_evidence: Vec<IntelligenceHubEvidenceRef>,
    available_surfaces: Vec<String>,
    unavailable_surfaces: Vec<String>,
    stale_surfaces: Vec<String>,
    conflicting_surfaces: Vec<String>,
    requested: usize,
    conflict_count: usize,
}

impl HubComposeAccumulator {
    fn push_available(
        &mut self,
        surface: &str,
        artefact_id: String,
        revision: Option<String>,
        is_conflicting: bool,
        is_stale: bool,
    ) -> Result<(), IntelligenceHubError> {
        let rev = revision.clone().unwrap_or_else(|| artefact_id.clone());
        self.source_revisions.push(format!("{surface}:{rev}"));
        let evidence = IntelligenceHubEvidenceRef::link(surface, artefact_id.clone(), Some(rev));
        self.provenance_links.push(evidence.clone());
        self.lineage_sources.push(surface.into());
        self.lineage_revisions
            .push(self.source_revisions.last().cloned().unwrap_or_default());
        self.lineage_evidence.push(evidence.clone());

        let availability = if is_conflicting {
            IntelligencePackage::AVAILABILITY_UNKNOWN
        } else if is_stale {
            IntelligencePackage::AVAILABILITY_STALE
        } else {
            IntelligencePackage::AVAILABILITY_AVAILABLE
        };
        let pkg = IntelligencePackage::reference(
            surface,
            artefact_id,
            revision,
            availability,
            vec![evidence],
        )?;
        self.packages.push(pkg);

        if is_conflicting {
            self.conflicting_surfaces.push(surface.into());
            self.conflict_count += 1;
        } else if is_stale {
            self.stale_surfaces.push(surface.into());
        } else {
            self.available_surfaces.push(surface.into());
        }
        Ok(())
    }

    fn push_missing(&mut self, surface: &str, missing_msg: &str, severity: &str) {
        self.gaps.push(IntelligenceGap::record(
            missing_msg,
            vec![surface.into()],
            "Missing source remains Missing — never invent intelligence packages",
            severity,
            vec![],
        ));
        self.unavailable_surfaces.push(surface.into());
        let pkg = IntelligencePackage::reference(
            surface,
            format!("missing:{surface}"),
            None,
            IntelligencePackage::AVAILABILITY_UNAVAILABLE,
            vec![IntelligenceHubEvidenceRef::link(
                surface,
                format!("missing:{surface}"),
                None,
            )],
        )
        .expect("unavailable package with placeholder ref");
        self.packages.push(pkg);
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[IntelligenceGap],
    conflict_count: usize,
) -> IntelligenceHubCompleteness {
    if requested == 0 || available == 0 {
        return IntelligenceHubCompleteness::Unavailable;
    }
    if conflict_count > 0 {
        return IntelligenceHubCompleteness::Contradictory;
    }
    if available == requested && gaps.is_empty() {
        return IntelligenceHubCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() {
        return IntelligenceHubCompleteness::Partial;
    }
    IntelligenceHubCompleteness::Unknown
}

fn validate_availability(value: &str) -> Result<(), IntelligenceHubError> {
    match value {
        IntelligencePackage::AVAILABILITY_AVAILABLE
        | IntelligencePackage::AVAILABILITY_UNAVAILABLE
        | IntelligencePackage::AVAILABILITY_STALE
        | IntelligencePackage::AVAILABILITY_UNKNOWN => Ok(()),
        other => Err(IntelligenceHubError::InvalidAvailability(other.into())),
    }
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
        WorkspaceStateConflict, WorkspaceStateEnvelope, WorkspaceStateSource,
    };

    #[test]
    fn missing_upstreams_unavailable() {
        let snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        assert_eq!(snap.completeness, IntelligenceHubCompleteness::Unavailable);
        assert_eq!(snap.gaps.len(), 11);
        assert!(snap.is_non_executing());
        assert!(snap
            .packages
            .iter()
            .all(|p| p.availability == IntelligencePackage::AVAILABILITY_UNAVAILABLE));
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = WorkspaceIntelligenceHubSnapshot::compose_deterministic(
            "ws",
            "t1",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        let right = WorkspaceIntelligenceHubSnapshot::compose_deterministic(
            "ws",
            "t1",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        assert_eq!(left.hub_id, right.hub_id);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.completeness, right.completeness);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        snap.mark_superseded("t1");
        let entry = IntelligenceHubHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj = WorkspaceIntelligenceHubProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn conflicts_preserved_not_resolved() {
        let conflict = WorkspaceStateConflict::record_deterministic(
            "planning",
            "reasoning",
            "Planning present while reasoning absent",
            "medium",
            vec!["planning:available".into(), "reasoning:unavailable".into()],
        );
        let envelope = WorkspaceStateEnvelope::compose(
            "ws",
            "t0",
            vec![WorkspaceStateSource::unavailable(
                "reasoning",
                "t0",
                "Reasoning snapshot missing",
            )],
            vec![conflict],
            vec!["reasoning current unknown".into()],
        )
        .unwrap();
        let state = WorkspaceStateSnapshot::assemble("ws", Some(envelope), vec![], 0, "t0");
        let snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame {
                include_state: true,
                ..IntelligenceHubFrame::all_surfaces()
            },
            Some(&state),
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
        assert!(!snap.conflicts.is_empty());
        assert!(snap.conflicts.iter().all(|c| !c.actionable));
        assert!(snap
            .summary
            .conflicting_surfaces
            .contains(&"workspace_state_envelope".to_string()));
    }

    #[test]
    fn packages_require_refs_when_available() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame {
                include_state: true,
                include_policy: false,
                include_reconstruction: false,
                include_temporal: false,
                include_explanation: false,
                include_contextual: false,
                include_knowledge_synthesis: false,
                include_knowledge_integration: false,
                include_insight: false,
                include_cross_workspace: false,
                include_decision_support: false,
            },
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
            None,
        )
        .unwrap();
        for pkg in &snap.packages {
            if pkg.availability == IntelligencePackage::AVAILABILITY_AVAILABLE {
                assert!(!pkg.evidence_refs.is_empty());
            }
        }
        let mut bad = snap;
        if let Some(pkg) = bad.packages.first_mut() {
            pkg.availability = IntelligencePackage::AVAILABILITY_AVAILABLE.into();
            pkg.evidence_refs.clear();
        }
        assert!(bad.validate().is_err());
    }

    #[test]
    fn assessment_non_actionable() {
        let snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        assert!(snap.assessment.is_non_actionable());
        assert!(!snap.assessment.actionable);
        assert_eq!(snap.assessment.authority_effect, "none");
    }

    #[test]
    fn forbidden_phrases_rejected() {
        let mut snap = WorkspaceIntelligenceHubSnapshot::compose(
            "ws",
            "t0",
            IntelligenceHubFrame::all_surfaces(),
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
            None,
        )
        .unwrap();
        snap.narrative = "You should execute this immediately.".into();
        assert!(snap.validate().is_err());
    }
}
