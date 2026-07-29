//! Workspace Evidence Freshness Engine — Programme IV Batch 7.
//!
//! Observe freshness. Never refresh evidence.
//! freshness ≠ validity; observation ≠ regeneration; stale ≠ refresh request.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_evidence_consistency::WorkspaceEvidenceConsistencyProjection;
use crate::workspace_evidence_coverage::WorkspaceEvidenceCoverageProjection;
use crate::workspace_evidence_dependency::WorkspaceEvidenceDependencyProjection;
use crate::workspace_evidence_navigation::WorkspaceEvidenceNavigationProjection;
use crate::workspace_evidence_trace::WorkspaceEvidenceTraceProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_intelligence_hub::WorkspaceIntelligenceHubProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_semantic_query::WorkspaceSemanticQueryProjection;
use crate::workspace_state_envelope::{FreshnessStatus, WorkspaceStateSnapshot};
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvidenceFreshnessError {
    #[error("invalid evidence freshness status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence freshness observation state: {0}")]
    InvalidObservationState(String),

    #[error("invalid evidence freshness completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence freshness artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence freshness artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("evidence freshness snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceFreshnessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceFreshnessError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceFreshnessError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Descriptive freshness state only — never a refresh directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessObservationState {
    Fresh,
    Stale,
    Unknown,
    Unavailable,
}

impl FreshnessObservationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceFreshnessError> {
        match value {
            "fresh" => Ok(Self::Fresh),
            "stale" => Ok(Self::Stale),
            "unknown" => Ok(Self::Unknown),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceFreshnessError::InvalidObservationState(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceFreshnessCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceFreshnessError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceFreshnessError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream surfaces participate in freshness observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessScope {
    pub include_evidence_dependency: bool,
    pub include_evidence_consistency: bool,
    pub include_evidence_coverage: bool,
    pub include_evidence_trace: bool,
    pub include_evidence_navigation: bool,
    pub include_semantic_query: bool,
    pub include_intelligence_hub: bool,
    pub include_knowledge_integration: bool,
    pub include_contextual: bool,
    pub include_explanation: bool,
    pub include_temporal: bool,
    pub include_reconstruction: bool,
    pub include_state: bool,
}

impl FreshnessScope {
    pub fn all_surfaces() -> Self {
        Self {
            include_evidence_dependency: true,
            include_evidence_consistency: true,
            include_evidence_coverage: true,
            include_evidence_trace: true,
            include_evidence_navigation: true,
            include_semantic_query: true,
            include_intelligence_hub: true,
            include_knowledge_integration: true,
            include_contextual: true,
            include_explanation: true,
            include_temporal: true,
            include_reconstruction: true,
            include_state: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceFreshnessEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceFreshnessEvidenceRef {
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

/// Observable freshness assessment — no interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessAssessment {
    pub assessment_id: String,
    pub observed_scope: Vec<String>,
    pub participating_artefacts: Vec<String>,
    pub observable_freshness_state: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl FreshnessAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "freshness_assessment:";

    pub fn assemble(
        observed_scope: Vec<String>,
        participating_artefacts: Vec<String>,
        observable_freshness_state: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "scope={}|part={}|state={}",
            observed_scope.len(),
            participating_artefacts.len(),
            observable_freshness_state.join(",")
        ));
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_scope,
            participating_artefacts,
            observable_freshness_state,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// One observed evidence freshness state — descriptive only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessObservation {
    pub observation_id: String,
    pub artefact_ref: String,
    pub origin_domain: String,
    pub state: FreshnessObservationState,
    pub recorded_timestamp: Option<String>,
    pub recorded_revision: Option<String>,
    pub description: String,
    pub evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl FreshnessObservation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "freshness_observation:";

    pub fn record(
        artefact_ref: impl Into<String>,
        origin_domain: impl Into<String>,
        state: FreshnessObservationState,
        recorded_timestamp: Option<String>,
        recorded_revision: Option<String>,
        description: impl Into<String>,
        evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    ) -> Self {
        let artefact_ref = artefact_ref.into();
        let origin_domain = origin_domain.into();
        let description = description.into();
        let digest = stable_digest(&format!(
            "{origin_domain}|{artefact_ref}|{}|{description}",
            state.as_str()
        ));
        Self {
            observation_id: format!("{}{}", Self::ID_PREFIX, digest),
            artefact_ref,
            origin_domain,
            state,
            recorded_timestamp,
            recorded_revision,
            description,
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

/// Unavailable timestamp / missing revision / unknown age / inaccessible — never repaired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl FreshnessGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "freshness_gap:";
    pub const KIND_UNAVAILABLE_TIMESTAMP: &'static str = "unavailable_timestamp";
    pub const KIND_MISSING_REVISION: &'static str = "missing_revision";
    pub const KIND_UNKNOWN_AGE: &'static str = "unknown_age";
    pub const KIND_INACCESSIBLE: &'static str = "inaccessible_evidence";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        let digest = stable_digest(&format!(
            "{gap_kind}|{description}|{}",
            affected_references.join(",")
        ));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            gap_kind,
            description,
            affected_references,
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

/// Exact upstream artefacts that contributed recorded freshness metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl FreshnessLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "freshness_lineage:";

    pub fn from_contributors(
        contributing_artefacts: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceFreshnessEvidenceRef>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "{}|{}",
            contributing_artefacts.join(","),
            revisions.join(",")
        ));
        Self {
            lineage_id: format!("{}{}", Self::ID_PREFIX, digest),
            contributing_artefacts,
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

/// Deterministic diagnostics — no recommendations or refresh directives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessDiagnostics {
    pub diagnostics_id: String,
    pub observed_freshness_coverage: String,
    pub unavailable_observations: Vec<String>,
    pub missing_timestamps: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl FreshnessDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "freshness_diagnostics:";

    pub fn assemble(
        observed_freshness_coverage: impl Into<String>,
        unavailable_observations: Vec<String>,
        missing_timestamps: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let observed_freshness_coverage = observed_freshness_coverage.into();
        let digest = stable_digest(&format!(
            "{observed_freshness_coverage}|{}|{}",
            unavailable_observations.len(),
            missing_timestamps.len()
        ));
        Self {
            diagnostics_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_freshness_coverage,
            unavailable_observations,
            missing_timestamps,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable evidence freshness artefact — observe recorded freshness only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceFreshnessSnapshot {
    pub freshness_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceFreshnessStatus,
    pub superseded_at: Option<String>,
    pub scope: FreshnessScope,
    pub assessment: FreshnessAssessment,
    pub observations: Vec<FreshnessObservation>,
    pub gaps: Vec<FreshnessGap>,
    pub lineage: FreshnessLineage,
    pub diagnostics: FreshnessDiagnostics,
    pub completeness: EvidenceFreshnessCompleteness,
    pub provenance_links: Vec<EvidenceFreshnessEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceFreshnessSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_freshness:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: FreshnessScope,
        evidence_dependency: Option<&WorkspaceEvidenceDependencyProjection>,
        evidence_consistency: Option<&WorkspaceEvidenceConsistencyProjection>,
        evidence_coverage: Option<&WorkspaceEvidenceCoverageProjection>,
        evidence_trace: Option<&WorkspaceEvidenceTraceProjection>,
        evidence_navigation: Option<&WorkspaceEvidenceNavigationProjection>,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        contextual: Option<&ContextualUnderstandingProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, EvidenceFreshnessError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut observed_scope = Vec::new();
        let mut participating = Vec::new();
        let mut observations = Vec::new();
        let mut gaps = Vec::new();
        let mut contributing = Vec::new();
        let mut revisions = Vec::new();
        let mut lineage_evidence = Vec::new();
        let mut provenance_links = Vec::new();
        let mut unavailable = Vec::new();
        let mut missing_timestamps = Vec::new();
        let mut surfaces_requested = 0usize;
        let mut surfaces_available = 0usize;
        let mut observable_states = Vec::new();

        macro_rules! observe {
            ($flag:expr, $name:expr, $opt:expr, $extract:expr) => {
                if $flag {
                    surfaces_requested += 1;
                    observed_scope.push($name.to_string());
                    match $opt {
                        Some(proj) => {
                            if let Some(sig) = $extract(proj) {
                                surfaces_available += 1;
                                participating.push(sig.artefact_ref.clone());
                                contributing.push(format!("{}:{}", $name, sig.artefact_ref));
                                if let Some(rev) = &sig.revision {
                                    revisions.push(rev.clone());
                                }
                                let link = EvidenceFreshnessEvidenceRef::link(
                                    $name,
                                    sig.artefact_ref.clone(),
                                    sig.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link.clone());

                                let state = classify_freshness(&sig);
                                observable_states.push(state.as_str().to_string());
                                observations.push(FreshnessObservation::record(
                                    sig.artefact_ref.clone(),
                                    $name,
                                    state,
                                    sig.timestamp.clone(),
                                    sig.revision.clone(),
                                    describe_state($name, state, &sig),
                                    vec![link],
                                ));

                                if sig.timestamp.as_ref().map(|t| t.trim().is_empty()).unwrap_or(true)
                                {
                                    missing_timestamps.push($name.to_string());
                                    gaps.push(FreshnessGap::record(
                                        FreshnessGap::KIND_UNAVAILABLE_TIMESTAMP,
                                        format!(
                                            "Surface '{}' has no recorded timestamp — never fabricate timestamps",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![],
                                    ));
                                }
                                if sig.revision.as_ref().map(|r| r.trim().is_empty()).unwrap_or(true)
                                {
                                    gaps.push(FreshnessGap::record(
                                        FreshnessGap::KIND_MISSING_REVISION,
                                        format!(
                                            "Surface '{}' has no recorded revision — never invent revisions",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![],
                                    ));
                                }
                                if state == FreshnessObservationState::Unknown {
                                    gaps.push(FreshnessGap::record(
                                        FreshnessGap::KIND_UNKNOWN_AGE,
                                        format!(
                                            "Surface '{}' freshness age is unknown — never estimate freshness",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![],
                                    ));
                                }
                            } else {
                                unavailable.push($name.to_string());
                                observable_states.push(
                                    FreshnessObservationState::Unavailable.as_str().into(),
                                );
                                gaps.push(FreshnessGap::record(
                                    FreshnessGap::KIND_INACCESSIBLE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact — never invent freshness",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                            }
                        }
                        None => {
                            unavailable.push($name.to_string());
                            observable_states
                                .push(FreshnessObservationState::Unavailable.as_str().into());
                            gaps.push(FreshnessGap::record(
                                FreshnessGap::KIND_INACCESSIBLE,
                                format!(
                                    "Upstream surface '{}' unavailable — never invent freshness or refresh",
                                    $name
                                ),
                                vec![$name.into()],
                                vec![],
                            ));
                        }
                    }
                }
            };
        }

        observe!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            evidence_dependency,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.dependency_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.dependency_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.consistency_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.consistency_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.coverage_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.coverage_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.trace_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.trace_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.navigation_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.navigation_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.query_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.query_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.hub_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.hub_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.integration_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.integration_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.understanding_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.understanding_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.explanation_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.explanation_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.analysis_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.analysis_id.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.reconstruction_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: None,
                })
            }
        );
        observe!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceFreshness {
                    artefact_ref: c.state_id.clone(),
                    timestamp: Some(c.generated_at.clone()),
                    revision: Some(c.revision.clone()),
                    superseded: c.superseded_at.is_some() || c.terminal,
                    explicit: Some(c.freshness),
                })
            }
        );

        observable_states.sort();
        observable_states.dedup();
        missing_timestamps.sort();
        missing_timestamps.dedup();

        let assessment = FreshnessAssessment::assemble(
            observed_scope.clone(),
            participating.clone(),
            observable_states,
        );
        let completeness =
            derive_completeness(surfaces_requested, surfaces_available, &observations, &gaps);
        let diagnostics = FreshnessDiagnostics::assemble(
            completeness.as_str(),
            unavailable.clone(),
            missing_timestamps,
            vec![
                "Freshness diagnostics are observational only".into(),
                "freshness does not imply validity".into(),
                "observation does not imply regeneration".into(),
            ],
        );
        let lineage =
            FreshnessLineage::from_contributors(contributing, revisions.clone(), lineage_evidence);

        let narrative_summary = format!(
            "Evidence freshness: {}/{} sources available, {} observation(s), {} gap(s), completeness={}",
            surfaces_available,
            surfaces_requested,
            observations.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Observed recorded evidence freshness for workspace {workspace_id}. \
             Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer observes freshness only — it never refreshes evidence, \
             regenerates snapshots, or estimates freshness."
        );
        let limitations = vec![
            "Workspace Evidence Freshness Engine observes recorded evidence freshness only".into(),
            "It never refreshes evidence or regenerates snapshots".into(),
            "It never becomes the authority for any upstream intelligence layer".into(),
            "Freshness never implies validity, confidence, or action".into(),
        ];

        let snap = Self {
            freshness_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceFreshnessStatus::Current,
            superseded_at: None,
            scope,
            assessment,
            observations,
            gaps,
            lineage,
            diagnostics,
            completeness,
            provenance_links,
            source_revisions: revisions,
            narrative_summary,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        snap.validate()?;
        Ok(snap)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: FreshnessScope,
        evidence_dependency: Option<&WorkspaceEvidenceDependencyProjection>,
        evidence_consistency: Option<&WorkspaceEvidenceConsistencyProjection>,
        evidence_coverage: Option<&WorkspaceEvidenceCoverageProjection>,
        evidence_trace: Option<&WorkspaceEvidenceTraceProjection>,
        evidence_navigation: Option<&WorkspaceEvidenceNavigationProjection>,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        contextual: Option<&ContextualUnderstandingProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, EvidenceFreshnessError> {
        Self::compose(
            workspace_id,
            generated_at,
            scope,
            evidence_dependency,
            evidence_consistency,
            evidence_coverage,
            evidence_trace,
            evidence_navigation,
            semantic_query,
            intelligence_hub,
            knowledge_integration,
            contextual,
            explanation,
            temporal,
            reconstruction,
            state,
        )
    }

    pub fn mark_superseded(&mut self, superseded_at: impl Into<String>) {
        self.status = EvidenceFreshnessStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.assessment.is_non_actionable()
            && self.observations.iter().all(|o| o.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.diagnostics.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), EvidenceFreshnessError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceFreshnessError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceFreshnessError::MustNotBeActionable);
        }
        if !self.assessment.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.observations.iter().any(|o| !o.is_non_actionable())
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceFreshnessError::MustNotBeActionable);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for note in &self.diagnostics.notes {
            reject_forbidden_phrases(note)?;
        }
        for obs in &self.observations {
            reject_forbidden_phrases(&obs.description)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceFreshnessHistoryEntry {
    pub freshness_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub observation_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl EvidenceFreshnessHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceFreshnessSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            freshness_id: snap.freshness_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            observation_count: snap.observations.len(),
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
            && (self.status == "superseded" || self.status == "archived")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceFreshnessProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceFreshnessSnapshot>,
    pub history: Vec<EvidenceFreshnessHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceFreshnessProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceFreshnessSnapshot>,
        history: Vec<EvidenceFreshnessHistoryEntry>,
        history_count: usize,
        projected_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            current,
            history,
            history_count,
            projected_at: projected_at.into(),
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceFreshnessSummary {
        WorkspaceEvidenceFreshnessSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().to_string()),
            observation_count: self
                .current
                .as_ref()
                .map(|c| c.observations.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            narrative_summary: self
                .current
                .as_ref()
                .map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceFreshnessSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub observation_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceFreshnessHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceFreshnessExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub observation_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceEvidenceFreshnessExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceFreshnessSnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_freshness_explanation:{}", snap.freshness_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            observation_summaries: snap
                .observations
                .iter()
                .map(|o| {
                    format!(
                        "{} ({}) => {} — {}",
                        o.artefact_ref,
                        o.origin_domain,
                        o.state.as_str(),
                        o.description
                    )
                })
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{} ({})", g.description, g.gap_kind))
                .collect(),
            lineage_summaries: snap.lineage.contributing_artefacts.clone(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: vec![
                "Freshness explanation is observational only".into(),
                "Unknown remains unknown".into(),
                "Stale remains stale — never a refresh directive".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceFreshness {
    artefact_ref: String,
    timestamp: Option<String>,
    revision: Option<String>,
    superseded: bool,
    explicit: Option<FreshnessStatus>,
}

fn classify_freshness(sig: &SurfaceFreshness) -> FreshnessObservationState {
    if let Some(explicit) = sig.explicit {
        return match explicit {
            FreshnessStatus::Fresh => FreshnessObservationState::Fresh,
            FreshnessStatus::Stale => FreshnessObservationState::Stale,
            FreshnessStatus::Unknown => FreshnessObservationState::Unknown,
            FreshnessStatus::Unavailable => FreshnessObservationState::Unavailable,
        };
    }
    let has_timestamp = sig
        .timestamp
        .as_ref()
        .map(|t| !t.trim().is_empty())
        .unwrap_or(false);
    let has_revision = sig
        .revision
        .as_ref()
        .map(|r| !r.trim().is_empty())
        .unwrap_or(false);
    if sig.superseded {
        return FreshnessObservationState::Stale;
    }
    if !has_timestamp && !has_revision {
        return FreshnessObservationState::Unknown;
    }
    if has_timestamp || has_revision {
        return FreshnessObservationState::Fresh;
    }
    FreshnessObservationState::Unknown
}

fn describe_state(
    surface: &str,
    state: FreshnessObservationState,
    sig: &SurfaceFreshness,
) -> String {
    match state {
        FreshnessObservationState::Fresh => format!(
            "Surface '{surface}' records current freshness metadata — not a validity claim"
        ),
        FreshnessObservationState::Stale => format!(
            "Surface '{surface}' records superseded or stale freshness metadata — never a refresh request"
        ),
        FreshnessObservationState::Unknown => format!(
            "Surface '{surface}' freshness is unknown — never estimate age or invent timestamps"
        ),
        FreshnessObservationState::Unavailable => format!(
            "Surface '{surface}' freshness unavailable — never invent freshness (artefact={})",
            sig.artefact_ref
        ),
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    observations: &[FreshnessObservation],
    gaps: &[FreshnessGap],
) -> EvidenceFreshnessCompleteness {
    if requested == 0 {
        return EvidenceFreshnessCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceFreshnessCompleteness::Unavailable;
    }
    if observations
        .iter()
        .any(|o| o.state == FreshnessObservationState::Stale)
        && observations
            .iter()
            .any(|o| o.state == FreshnessObservationState::Fresh)
    {
        return EvidenceFreshnessCompleteness::Contradictory;
    }
    if available == requested
        && gaps.is_empty()
        && observations
            .iter()
            .all(|o| o.state == FreshnessObservationState::Fresh)
    {
        return EvidenceFreshnessCompleteness::Complete;
    }
    if available < requested
        || !gaps.is_empty()
        || observations.iter().any(|o| {
            matches!(
                o.state,
                FreshnessObservationState::Unknown
                    | FreshnessObservationState::Unavailable
                    | FreshnessObservationState::Stale
            )
        })
    {
        return EvidenceFreshnessCompleteness::Partial;
    }
    EvidenceFreshnessCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceFreshnessError> {
    let lower = text.to_ascii_lowercase();
    const FORBIDDEN: &[&str] = &[
        "recommend",
        "should do",
        "best choice",
        "optimal",
        "execute now",
        "approve",
        "dispatch",
        "automate",
        "is correct",
        "is true",
        "confident that",
        "therefore decide",
        "please refresh",
        "must refresh",
        "regenerate now",
        "schedule update",
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(EvidenceFreshnessError::MustNotBeActionable);
        }
    }
    Ok(())
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

    fn empty_compose() -> WorkspaceEvidenceFreshnessSnapshot {
        WorkspaceEvidenceFreshnessSnapshot::compose(
            "ws",
            "t0",
            FreshnessScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn unavailable_preserved() {
        let snap = empty_compose();
        assert_eq!(
            snap.completeness,
            EvidenceFreshnessCompleteness::Unavailable
        );
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == FreshnessGap::KIND_INACCESSIBLE));
        assert!(snap.observations.is_empty());
        assert!(snap.is_non_executing());
    }

    #[test]
    fn identical_inputs_produce_identical_outputs() {
        let left = WorkspaceEvidenceFreshnessSnapshot::compose_deterministic(
            "ws",
            "t1",
            FreshnessScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap();
        let right = WorkspaceEvidenceFreshnessSnapshot::compose_deterministic(
            "ws",
            "t1",
            FreshnessScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.freshness_id, right.freshness_id);
        assert_eq!(left.observations, right.observations);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = empty_compose();
        snap.mark_superseded("t1");
        let entry = EvidenceFreshnessHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj = WorkspaceEvidenceFreshnessProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn stale_preserved() {
        let obs = FreshnessObservation::record(
            "artefact:1",
            "workspace_state_envelope",
            FreshnessObservationState::Stale,
            Some("t0".into()),
            Some("rev1".into()),
            "Surface 'workspace_state_envelope' records superseded or stale freshness metadata — never a refresh request",
            vec![],
        );
        assert_eq!(obs.state, FreshnessObservationState::Stale);
        assert!(obs.is_non_actionable());
        assert!(!obs.description.to_ascii_lowercase().contains("must refresh"));
    }

    #[test]
    fn unknown_preserved() {
        let sig = SurfaceFreshness {
            artefact_ref: "a".into(),
            timestamp: None,
            revision: None,
            superseded: false,
            explicit: None,
        };
        assert_eq!(
            classify_freshness(&sig),
            FreshnessObservationState::Unknown
        );
    }

    #[test]
    fn freshness_never_implies_validity() {
        let snap = empty_compose();
        assert!(snap
            .limitations
            .iter()
            .any(|l| l.contains("validity")));
        assert!(!snap.narrative.to_ascii_lowercase().contains("is correct"));
        assert!(!snap.narrative.to_ascii_lowercase().contains("is true"));
    }

    #[test]
    fn non_executable_snapshots() {
        let snap = empty_compose();
        assert!(snap.is_non_executing());
        let mut bad = snap;
        bad.actionable = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn deterministic_freshness_assessment() {
        let snap = empty_compose();
        assert_eq!(snap.assessment.observed_scope.len(), 13);
        assert!(snap.assessment.participating_artefacts.is_empty());
        assert!(snap
            .assessment
            .observable_freshness_state
            .iter()
            .any(|s| s == "unavailable"));
    }
}
