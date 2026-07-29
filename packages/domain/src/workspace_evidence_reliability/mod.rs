//! Workspace Evidence Reliability Engine - Programme IV Batch 9.
//!
//! Observe reliability. Never establish truth.
//! reliability != truth; observation != trust; limited != reject.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_evidence_completeness::WorkspaceEvidenceCompletenessProjection;
use crate::workspace_evidence_consistency::WorkspaceEvidenceConsistencyProjection;
use crate::workspace_evidence_coverage::WorkspaceEvidenceCoverageProjection;
use crate::workspace_evidence_dependency::WorkspaceEvidenceDependencyProjection;
use crate::workspace_evidence_freshness::WorkspaceEvidenceFreshnessProjection;
use crate::workspace_evidence_navigation::WorkspaceEvidenceNavigationProjection;
use crate::workspace_evidence_trace::WorkspaceEvidenceTraceProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_intelligence_hub::WorkspaceIntelligenceHubProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_semantic_query::WorkspaceSemanticQueryProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvidenceReliabilityError {
    #[error("invalid evidence reliability status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence reliability observation state: {0}")]
    InvalidObservationState(String),

    #[error("invalid evidence reliability completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence reliability artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence reliability artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("evidence reliability snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceReliabilityStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceReliabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceReliabilityError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceReliabilityError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Descriptive reliability state only; never a trust directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReliabilityObservationState {
    Reliable,
    Limited,
    Unknown,
    Unavailable,
}

impl ReliabilityObservationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reliable => "reliable",
            Self::Limited => "limited",
            Self::Unknown => "unknown",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceReliabilityError> {
        match value {
            "reliable" => Ok(Self::Reliable),
            "limited" => Ok(Self::Limited),
            "unknown" => Ok(Self::Unknown),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceReliabilityError::InvalidObservationState(
                other.into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceReliabilityCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceReliabilityCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceReliabilityError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceReliabilityError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream surfaces participate in reliability observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityScope {
    pub include_evidence_completeness: bool,
    pub include_evidence_freshness: bool,
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

impl ReliabilityScope {
    pub fn all_surfaces() -> Self {
        Self {
            include_evidence_completeness: true,
            include_evidence_freshness: true,
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
pub struct EvidenceReliabilityEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceReliabilityEvidenceRef {
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

/// Observable reliability assessment; no interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityAssessment {
    pub assessment_id: String,
    pub observed_scope: Vec<String>,
    pub participating_artefacts: Vec<String>,
    pub observable_reliability_state: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ReliabilityAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reliability_assessment:";

    pub fn assemble(
        observed_scope: Vec<String>,
        participating_artefacts: Vec<String>,
        observable_reliability_state: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "scope={}|part={}|state={}",
            observed_scope.join(","),
            participating_artefacts.join(","),
            observable_reliability_state.join(",")
        ));
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_scope,
            participating_artefacts,
            observable_reliability_state,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// One observed evidence reliability state; descriptive only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityObservation {
    pub observation_id: String,
    pub artefact_ref: String,
    pub origin_domain: String,
    pub state: ReliabilityObservationState,
    pub description: String,
    pub evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ReliabilityObservation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reliability_observation:";

    pub fn record(
        artefact_ref: impl Into<String>,
        origin_domain: impl Into<String>,
        state: ReliabilityObservationState,
        description: impl Into<String>,
        evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
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

/// Missing, unavailable, incomplete provenance, or insufficient characteristics; never repaired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ReliabilityGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reliability_gap:";
    pub const KIND_UNAVAILABLE_EVIDENCE: &'static str = "unavailable_evidence";
    pub const KIND_MISSING_PROVENANCE: &'static str = "missing_provenance";
    pub const KIND_INCOMPLETE_OBSERVATION: &'static str = "incomplete_observation";
    pub const KIND_INSUFFICIENT_CHARACTERISTICS: &'static str = "insufficient_characteristics";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
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

/// Exact upstream artefacts that contributed recorded reliability metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ReliabilityLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reliability_lineage:";

    pub fn from_contributors(
        contributing_artefacts: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceReliabilityEvidenceRef>,
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

/// Deterministic diagnostics; no directives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReliabilityDiagnostics {
    pub diagnostics_id: String,
    pub observable_reliability_coverage: String,
    pub unavailable_observations: Vec<String>,
    pub insufficient_evidence_characteristics: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ReliabilityDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reliability_diagnostics:";

    pub fn assemble(
        observable_reliability_coverage: impl Into<String>,
        unavailable_observations: Vec<String>,
        insufficient_evidence_characteristics: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let observable_reliability_coverage = observable_reliability_coverage.into();
        let digest = stable_digest(&format!(
            "{observable_reliability_coverage}|{}|{}",
            unavailable_observations.join(","),
            insufficient_evidence_characteristics.join(",")
        ));
        Self {
            diagnostics_id: format!("{}{}", Self::ID_PREFIX, digest),
            observable_reliability_coverage,
            unavailable_observations,
            insufficient_evidence_characteristics,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable evidence reliability artefact; observes recorded reliability only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceReliabilitySnapshot {
    pub reliability_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceReliabilityStatus,
    pub superseded_at: Option<String>,
    pub scope: ReliabilityScope,
    pub assessment: ReliabilityAssessment,
    pub observations: Vec<ReliabilityObservation>,
    pub gaps: Vec<ReliabilityGap>,
    pub lineage: ReliabilityLineage,
    pub diagnostics: ReliabilityDiagnostics,
    pub reliability: EvidenceReliabilityCompleteness,
    pub provenance_links: Vec<EvidenceReliabilityEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceReliabilitySnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_reliability:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: ReliabilityScope,
        evidence_completeness: Option<&WorkspaceEvidenceCompletenessProjection>,
        evidence_freshness: Option<&WorkspaceEvidenceFreshnessProjection>,
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
    ) -> Result<Self, EvidenceReliabilityError> {
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
        let mut unavailable_observations = Vec::new();
        let mut insufficient_characteristics = Vec::new();
        let mut surfaces_requested = 0usize;
        let mut surfaces_available = 0usize;
        let mut observable_states = Vec::new();
        let mut contradictory_seen = false;

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
                                let link = EvidenceReliabilityEvidenceRef::link(
                                    $name,
                                    sig.artefact_ref.clone(),
                                    sig.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link.clone());

                                if sig.completeness_level.as_deref() == Some("contradictory") {
                                    contradictory_seen = true;
                                }
                                let state = classify_reliability(&sig);
                                observable_states.push(state.as_str().to_string());
                                observations.push(ReliabilityObservation::record(
                                    sig.artefact_ref.clone(),
                                    $name,
                                    state,
                                    describe_state($name, state, &sig),
                                    vec![link.clone()],
                                ));

                                if state == ReliabilityObservationState::Unavailable {
                                    unavailable_observations.push($name.to_string());
                                    gaps.push(ReliabilityGap::record(
                                        ReliabilityGap::KIND_UNAVAILABLE_EVIDENCE,
                                        format!(
                                            "Surface '{}' records unavailable reliability characteristics",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link.clone()],
                                    ));
                                }
                                if sig.has_gaps && state == ReliabilityObservationState::Limited {
                                    insufficient_characteristics.push($name.to_string());
                                    gaps.push(ReliabilityGap::record(
                                        ReliabilityGap::KIND_INCOMPLETE_OBSERVATION,
                                        format!(
                                            "Surface '{}' records incomplete reliability observation; limited remains limited",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link.clone()],
                                    ));
                                }
                                if sig.provenance_empty && state != ReliabilityObservationState::Unavailable {
                                    insufficient_characteristics.push($name.to_string());
                                    gaps.push(ReliabilityGap::record(
                                        ReliabilityGap::KIND_MISSING_PROVENANCE,
                                        format!(
                                            "Surface '{}' has no recorded provenance for reliability observation",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link.clone()],
                                    ));
                                }
                                if sig.completeness_level.is_none()
                                    && sig.freshness_or_consistency_hint.is_none()
                                    && !sig.has_gaps
                                {
                                    insufficient_characteristics.push($name.to_string());
                                    gaps.push(ReliabilityGap::record(
                                        ReliabilityGap::KIND_INSUFFICIENT_CHARACTERISTICS,
                                        format!(
                                            "Surface '{}' has insufficient recorded reliability characteristics",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link],
                                    ));
                                }
                            } else {
                                unavailable_observations.push($name.to_string());
                                observable_states.push(
                                    ReliabilityObservationState::Unavailable.as_str().into(),
                                );
                                gaps.push(ReliabilityGap::record(
                                    ReliabilityGap::KIND_UNAVAILABLE_EVIDENCE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact; reliability remains unavailable",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                            }
                        }
                        None => {
                            unavailable_observations.push($name.to_string());
                            observable_states
                                .push(ReliabilityObservationState::Unavailable.as_str().into());
                            gaps.push(ReliabilityGap::record(
                                ReliabilityGap::KIND_UNAVAILABLE_EVIDENCE,
                                format!(
                                    "Upstream surface '{}' unavailable; reliability remains unavailable",
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
            scope.include_evidence_completeness,
            "workspace_evidence_completeness",
            evidence_completeness,
            |p: &WorkspaceEvidenceCompletenessProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.completeness_id.clone(),
                    revision: Some(c.completeness_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_freshness,
            "workspace_evidence_freshness",
            evidence_freshness,
            |p: &WorkspaceEvidenceFreshnessProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.freshness_id.clone(),
                    revision: Some(c.freshness_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: c
                        .observations
                        .iter()
                        .map(|o| o.state.as_str().to_string())
                        .find(|state| state == "stale")
                        .or_else(|| c.observations.first().map(|o| o.state.as_str().to_string())),
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            evidence_dependency,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.dependency_id.clone(),
                    revision: Some(c.dependency_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.consistency_id.clone(),
                    revision: Some(c.consistency_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: c
                        .observations
                        .iter()
                        .map(|o| o.state.as_str().to_string())
                        .find(|state| state == "inconsistent")
                        .or_else(|| c.observations.first().map(|o| o.state.as_str().to_string())),
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.coverage_id.clone(),
                    revision: Some(c.coverage_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.trace_id.clone(),
                    revision: Some(c.trace_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.participating_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.navigation_id.clone(),
                    revision: Some(c.navigation_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.participating_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.query_id.clone(),
                    revision: Some(c.query_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.contributing_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.hub_id.clone(),
                    revision: Some(c.hub_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.lineage.upstream_sources.is_empty(),
                })
            }
        );
        observe!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.integration_id.clone(),
                    revision: Some(c.integration_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.source_revisions.is_empty()
                        && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.understanding_id.clone(),
                    revision: Some(c.understanding_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.source_revisions.is_empty()
                        && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.explanation_id.clone(),
                    revision: Some(c.explanation_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.analysis_id.clone(),
                    revision: Some(c.analysis_id.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.chain_summary.ordered_refs.is_empty()
                        && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.reconstruction_id.clone(),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.gaps.is_empty(),
                    provenance_empty: c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceReliability {
                    artefact_ref: c.state_id.clone(),
                    revision: Some(c.revision.clone()),
                    completeness_level: Some(c.completeness.as_str().to_string()),
                    freshness_or_consistency_hint: None,
                    has_gaps: !c.unknowns.is_empty() || !c.contradictions.is_empty(),
                    provenance_empty: c.sources.is_empty(),
                })
            }
        );

        observable_states.sort();
        observable_states.dedup();
        unavailable_observations.sort();
        unavailable_observations.dedup();
        insufficient_characteristics.sort();
        insufficient_characteristics.dedup();

        let assessment = ReliabilityAssessment::assemble(
            observed_scope.clone(),
            participating.clone(),
            observable_states,
        );
        let reliability = derive_reliability(
            surfaces_requested,
            surfaces_available,
            &observations,
            &gaps,
            contradictory_seen,
        );
        let diagnostics = ReliabilityDiagnostics::assemble(
            reliability.as_str(),
            unavailable_observations,
            insufficient_characteristics,
            vec![
                "Reliability diagnostics are observational only".into(),
                "Reliability never implies truth".into(),
                "Observation never repairs or estimates evidence".into(),
            ],
        );
        let lineage = ReliabilityLineage::from_contributors(
            contributing,
            revisions.clone(),
            lineage_evidence,
        );

        let narrative_summary = format!(
            "Evidence reliability: {}/{} sources available, {} observation(s), {} gap(s), reliability={}",
            surfaces_available,
            surfaces_requested,
            observations.len(),
            gaps.len(),
            reliability.as_str()
        );
        let narrative = format!(
            "Observed recorded evidence reliability for workspace {workspace_id}. \
             Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer observes reliability only; it never repairs, fills, or estimates evidence."
        );
        let limitations = vec![
            "Workspace Evidence Reliability Engine observes recorded reliability only".into(),
            "It never completes evidence or changes upstream snapshots".into(),
            "Reliability never implies truth, validity, or authority".into(),
            "Limited remains limited; unavailable remains unavailable".into(),
        ];

        let snap = Self {
            reliability_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}|{}",
                    revisions.join(","),
                    reliability.as_str()
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceReliabilityStatus::Current,
            superseded_at: None,
            scope,
            assessment,
            observations,
            gaps,
            lineage,
            diagnostics,
            reliability,
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
        scope: ReliabilityScope,
        evidence_completeness: Option<&WorkspaceEvidenceCompletenessProjection>,
        evidence_freshness: Option<&WorkspaceEvidenceFreshnessProjection>,
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
    ) -> Result<Self, EvidenceReliabilityError> {
        Self::compose(
            workspace_id,
            generated_at,
            scope,
            evidence_completeness,
            evidence_freshness,
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
        self.status = EvidenceReliabilityStatus::Superseded;
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

    pub fn validate(&self) -> Result<(), EvidenceReliabilityError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceReliabilityError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceReliabilityError::MustNotBeActionable);
        }
        if !self.assessment.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.observations.iter().any(|o| !o.is_non_actionable())
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceReliabilityError::MustNotBeActionable);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for limitation in &self.limitations {
            reject_forbidden_phrases(limitation)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden_phrases(note)?;
        }
        for obs in &self.observations {
            reject_forbidden_phrases(&obs.description)?;
        }
        for gap in &self.gaps {
            reject_forbidden_phrases(&gap.description)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReliabilityHistoryEntry {
    pub reliability_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub reliability: String,
    pub observation_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl EvidenceReliabilityHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceReliabilitySnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            reliability_id: snap.reliability_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            reliability: snap.reliability.as_str().into(),
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
pub struct WorkspaceEvidenceReliabilityProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceReliabilitySnapshot>,
    pub history: Vec<EvidenceReliabilityHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceReliabilityProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceReliabilitySnapshot>,
        history: Vec<EvidenceReliabilityHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceReliabilitySummary {
        WorkspaceEvidenceReliabilitySummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            reliability: self
                .current
                .as_ref()
                .map(|c| c.reliability.as_str().to_string()),
            observation_count: self
                .current
                .as_ref()
                .map(|c| c.observations.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            narrative_summary: self.current.as_ref().map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceReliabilitySummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub reliability: Option<String>,
    pub observation_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceReliabilityHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceReliabilityExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub reliability: Option<String>,
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

impl WorkspaceEvidenceReliabilityExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceReliabilitySnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_reliability_explanation:{}", snap.reliability_id),
            workspace_id: snap.workspace_id.clone(),
            reliability: Some(snap.reliability.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            observation_summaries: snap
                .observations
                .iter()
                .map(|o| {
                    format!(
                        "{} ({}) => {} - {}",
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
                "Reliability explanation is observational only".into(),
                "Unknown remains unknown".into(),
                "Limited remains limited".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceReliability {
    artefact_ref: String,
    revision: Option<String>,
    completeness_level: Option<String>,
    freshness_or_consistency_hint: Option<String>,
    has_gaps: bool,
    provenance_empty: bool,
}

fn classify_reliability(sig: &SurfaceReliability) -> ReliabilityObservationState {
    match sig.freshness_or_consistency_hint.as_deref() {
        Some("unavailable") => return ReliabilityObservationState::Unavailable,
        Some("unknown") => return ReliabilityObservationState::Unknown,
        Some("stale" | "limited" | "partial" | "inconsistent") => {
            return ReliabilityObservationState::Limited;
        }
        Some("fresh" | "consistent") | None => {}
        Some(_) => {}
    }

    match sig.completeness_level.as_deref() {
        Some("unavailable") => return ReliabilityObservationState::Unavailable,
        Some("unknown") => return ReliabilityObservationState::Unknown,
        Some("partial" | "contradictory") => return ReliabilityObservationState::Limited,
        Some("complete") if !sig.has_gaps && !sig.provenance_empty => {
            return ReliabilityObservationState::Reliable;
        }
        Some("complete") => return ReliabilityObservationState::Limited,
        Some(_) | None => {}
    }
    if sig.has_gaps {
        return ReliabilityObservationState::Limited;
    }
    if sig.provenance_empty {
        return ReliabilityObservationState::Unknown;
    }
    if sig.completeness_level.is_none() && sig.freshness_or_consistency_hint.is_none() {
        return ReliabilityObservationState::Unknown;
    }
    ReliabilityObservationState::Reliable
}

fn describe_state(
    surface: &str,
    state: ReliabilityObservationState,
    sig: &SurfaceReliability,
) -> String {
    match state {
        ReliabilityObservationState::Reliable => format!(
            "Surface '{surface}' records reliable characteristics; observation makes no truth claim"
        ),
        ReliabilityObservationState::Limited => format!(
            "Surface '{surface}' records limited reliability characteristics; limited remains limited"
        ),
        ReliabilityObservationState::Unknown => format!(
            "Surface '{surface}' reliability is unknown; observation does not estimate"
        ),
        ReliabilityObservationState::Unavailable => format!(
            "Surface '{surface}' reliability unavailable; observation does not replace it (artefact={})",
            sig.artefact_ref
        ),
    }
}

fn derive_reliability(
    requested: usize,
    available: usize,
    observations: &[ReliabilityObservation],
    gaps: &[ReliabilityGap],
    contradictory_seen: bool,
) -> EvidenceReliabilityCompleteness {
    if requested == 0 {
        return EvidenceReliabilityCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceReliabilityCompleteness::Unavailable;
    }
    if contradictory_seen {
        return EvidenceReliabilityCompleteness::Contradictory;
    }
    if available == requested
        && gaps.is_empty()
        && observations
            .iter()
            .all(|o| o.state == ReliabilityObservationState::Reliable)
    {
        return EvidenceReliabilityCompleteness::Complete;
    }
    if available == requested
        && gaps.is_empty()
        && observations
            .iter()
            .any(|o| o.state == ReliabilityObservationState::Unknown)
    {
        return EvidenceReliabilityCompleteness::Unknown;
    }
    if available < requested
        || !gaps.is_empty()
        || observations.iter().any(|o| {
            matches!(
                o.state,
                ReliabilityObservationState::Limited | ReliabilityObservationState::Unavailable
            )
        })
    {
        return EvidenceReliabilityCompleteness::Partial;
    }
    EvidenceReliabilityCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceReliabilityError> {
    crate::workspace_evidence_contract::reject_forbidden_phrases_with(
        text,
        &[
            "determine truth",
            "resolve conflict",
            "trust this",
            "must trust",
            "please repair",
            "must repair",
            "fill the gap",
            "invent missing",
            "fabricate reliability",
        ],
    )
    .map_err(|_| EvidenceReliabilityError::MustNotBeActionable)
}

fn stable_digest(input: &str) -> String {
    crate::workspace_evidence_contract::stable_digest(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_compose() -> WorkspaceEvidenceReliabilitySnapshot {
        WorkspaceEvidenceReliabilitySnapshot::compose(
            "ws",
            "t0",
            ReliabilityScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn unavailable_preserved() {
        let snap = empty_compose();
        assert_eq!(
            snap.reliability,
            EvidenceReliabilityCompleteness::Unavailable
        );
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == ReliabilityGap::KIND_UNAVAILABLE_EVIDENCE));
        assert!(snap.observations.is_empty());
        assert!(snap.is_non_executing());
    }

    #[test]
    fn identical_inputs_produce_identical_outputs() {
        let left = WorkspaceEvidenceReliabilitySnapshot::compose_deterministic(
            "ws",
            "t1",
            ReliabilityScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap();
        let right = WorkspaceEvidenceReliabilitySnapshot::compose_deterministic(
            "ws",
            "t1",
            ReliabilityScope::all_surfaces(),
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
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.reliability_id, right.reliability_id);
        assert_eq!(left.observations, right.observations);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = empty_compose();
        snap.mark_superseded("t1");
        let entry = EvidenceReliabilityHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj =
            WorkspaceEvidenceReliabilityProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn partial_preserved() {
        let sig = SurfaceReliability {
            artefact_ref: "a".into(),
            revision: Some("r1".into()),
            completeness_level: Some("partial".into()),
            freshness_or_consistency_hint: None,
            has_gaps: false,
            provenance_empty: false,
        };
        assert_eq!(
            classify_reliability(&sig),
            ReliabilityObservationState::Limited
        );
    }

    #[test]
    fn unknown_preserved() {
        let sig = SurfaceReliability {
            artefact_ref: "a".into(),
            revision: None,
            completeness_level: Some("unknown".into()),
            freshness_or_consistency_hint: None,
            has_gaps: false,
            provenance_empty: false,
        };
        assert_eq!(
            classify_reliability(&sig),
            ReliabilityObservationState::Unknown
        );
    }

    #[test]
    fn unavailable_observation_state_preserved() {
        let sig = SurfaceReliability {
            artefact_ref: "a".into(),
            revision: None,
            completeness_level: Some("unavailable".into()),
            freshness_or_consistency_hint: None,
            has_gaps: false,
            provenance_empty: true,
        };
        assert_eq!(
            classify_reliability(&sig),
            ReliabilityObservationState::Unavailable
        );
    }

    #[test]
    fn reliability_never_implies_truth() {
        let snap = empty_compose();
        assert!(snap.limitations.iter().any(|l| l.contains("truth")));
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
    fn deterministic_reliability_assessment() {
        let snap = empty_compose();
        assert_eq!(snap.assessment.observed_scope.len(), 15);
        assert!(snap.assessment.participating_artefacts.is_empty());
        assert!(snap
            .assessment
            .observable_reliability_state
            .iter()
            .any(|s| s == "unavailable"));
    }
}
