//! Workspace Evidence Completeness Engine - Programme IV Batch 8.
//!
//! Observe completeness. Never complete evidence.
//! completeness != truth; observation != repair; partial != completion request.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
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
pub enum EvidenceCompletenessError {
    #[error("invalid evidence completeness status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence completeness observation state: {0}")]
    InvalidObservationState(String),

    #[error("invalid evidence completeness completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence completeness artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence completeness artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("evidence completeness snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCompletenessStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceCompletenessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceCompletenessError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceCompletenessError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Descriptive completeness state only; never a completion directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessObservationState {
    Complete,
    Partial,
    Unknown,
    Unavailable,
}

impl CompletenessObservationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceCompletenessError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceCompletenessError::InvalidObservationState(
                other.into(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCompletenessCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceCompletenessCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceCompletenessError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceCompletenessError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream surfaces participate in completeness observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessScope {
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

impl CompletenessScope {
    pub fn all_surfaces() -> Self {
        Self {
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
pub struct EvidenceCompletenessEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceCompletenessEvidenceRef {
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

/// Observable completeness assessment; no interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessAssessment {
    pub assessment_id: String,
    pub observed_scope: Vec<String>,
    pub participating_artefacts: Vec<String>,
    pub observable_completeness_state: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CompletenessAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "completeness_assessment:";

    pub fn assemble(
        observed_scope: Vec<String>,
        participating_artefacts: Vec<String>,
        observable_completeness_state: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "scope={}|part={}|state={}",
            observed_scope.join(","),
            participating_artefacts.join(","),
            observable_completeness_state.join(",")
        ));
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_scope,
            participating_artefacts,
            observable_completeness_state,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// One observed evidence completeness state; descriptive only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessObservation {
    pub observation_id: String,
    pub artefact_ref: String,
    pub origin_domain: String,
    pub state: CompletenessObservationState,
    pub description: String,
    pub evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CompletenessObservation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "completeness_observation:";

    pub fn record(
        artefact_ref: impl Into<String>,
        origin_domain: impl Into<String>,
        state: CompletenessObservationState,
        description: impl Into<String>,
        evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
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

/// Missing, unavailable, incomplete lineage, or incomplete observation; never repaired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CompletenessGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "completeness_gap:";
    pub const KIND_MISSING_EVIDENCE: &'static str = "missing_evidence";
    pub const KIND_UNAVAILABLE_ARTEFACT: &'static str = "unavailable_artefact";
    pub const KIND_INCOMPLETE_LINEAGE: &'static str = "incomplete_lineage";
    pub const KIND_INCOMPLETE_OBSERVATION: &'static str = "incomplete_observation";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
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

/// Exact upstream artefacts that contributed recorded completeness metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CompletenessLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "completeness_lineage:";

    pub fn from_contributors(
        contributing_artefacts: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceCompletenessEvidenceRef>,
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

/// Deterministic diagnostics; no directives or completion behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessDiagnostics {
    pub diagnostics_id: String,
    pub observed_completeness_coverage: String,
    pub missing_observations: Vec<String>,
    pub unavailable_artefacts: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CompletenessDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "completeness_diagnostics:";

    pub fn assemble(
        observed_completeness_coverage: impl Into<String>,
        missing_observations: Vec<String>,
        unavailable_artefacts: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let observed_completeness_coverage = observed_completeness_coverage.into();
        let digest = stable_digest(&format!(
            "{observed_completeness_coverage}|{}|{}",
            missing_observations.join(","),
            unavailable_artefacts.join(",")
        ));
        Self {
            diagnostics_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_completeness_coverage,
            missing_observations,
            unavailable_artefacts,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable evidence completeness artefact; observes recorded completeness only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceCompletenessSnapshot {
    pub completeness_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceCompletenessStatus,
    pub superseded_at: Option<String>,
    pub scope: CompletenessScope,
    pub assessment: CompletenessAssessment,
    pub observations: Vec<CompletenessObservation>,
    pub gaps: Vec<CompletenessGap>,
    pub lineage: CompletenessLineage,
    pub diagnostics: CompletenessDiagnostics,
    pub completeness: EvidenceCompletenessCompleteness,
    pub provenance_links: Vec<EvidenceCompletenessEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceCompletenessSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_completeness:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: CompletenessScope,
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
    ) -> Result<Self, EvidenceCompletenessError> {
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
        let mut missing_observations = Vec::new();
        let mut unavailable_artefacts = Vec::new();
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
                                let link = EvidenceCompletenessEvidenceRef::link(
                                    $name,
                                    sig.artefact_ref.clone(),
                                    sig.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link.clone());

                                if sig.recorded_level.as_deref() == Some("contradictory") {
                                    contradictory_seen = true;
                                }
                                let state = classify_completeness(&sig);
                                observable_states.push(state.as_str().to_string());
                                observations.push(CompletenessObservation::record(
                                    sig.artefact_ref.clone(),
                                    $name,
                                    state,
                                    describe_state($name, state, &sig),
                                    vec![link.clone()],
                                ));

                                if sig.has_gaps
                                    && matches!(
                                        state,
                                        CompletenessObservationState::Partial
                                            | CompletenessObservationState::Unknown
                                    )
                                {
                                    missing_observations.push($name.to_string());
                                    gaps.push(CompletenessGap::record(
                                        CompletenessGap::KIND_INCOMPLETE_OBSERVATION,
                                        format!(
                                            "Surface '{}' records incomplete completeness observation; partial remains partial",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link.clone()],
                                    ));
                                }
                                if sig.lineage_empty
                                    && state != CompletenessObservationState::Unavailable
                                {
                                    gaps.push(CompletenessGap::record(
                                        CompletenessGap::KIND_INCOMPLETE_LINEAGE,
                                        format!(
                                            "Surface '{}' has no recorded lineage for completeness observation",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![link],
                                    ));
                                }
                            } else {
                                missing_observations.push($name.to_string());
                                observable_states.push(
                                    CompletenessObservationState::Unavailable.as_str().into(),
                                );
                                gaps.push(CompletenessGap::record(
                                    CompletenessGap::KIND_MISSING_EVIDENCE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact; completeness remains unavailable",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                            }
                        }
                        None => {
                            unavailable_artefacts.push($name.to_string());
                            observable_states
                                .push(CompletenessObservationState::Unavailable.as_str().into());
                            gaps.push(CompletenessGap::record(
                                CompletenessGap::KIND_UNAVAILABLE_ARTEFACT,
                                format!(
                                    "Upstream surface '{}' unavailable; completeness remains unavailable",
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
            scope.include_evidence_freshness,
            "workspace_evidence_freshness",
            evidence_freshness,
            |p: &WorkspaceEvidenceFreshnessProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.freshness_id.clone(),
                    revision: Some(c.freshness_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            evidence_dependency,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.dependency_id.clone(),
                    revision: Some(c.dependency_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.consistency_id.clone(),
                    revision: Some(c.consistency_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.coverage_id.clone(),
                    revision: Some(c.coverage_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.contributing_artefacts.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.trace_id.clone(),
                    revision: Some(c.trace_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.participating_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.navigation_id.clone(),
                    revision: Some(c.navigation_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.participating_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.query_id.clone(),
                    revision: Some(c.query_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.contributing_snapshots.is_empty(),
                })
            }
        );
        observe!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.hub_id.clone(),
                    revision: Some(c.hub_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.lineage.upstream_sources.is_empty(),
                })
            }
        );
        observe!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.integration_id.clone(),
                    revision: Some(c.integration_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.source_revisions.is_empty() && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.understanding_id.clone(),
                    revision: Some(c.understanding_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.source_revisions.is_empty() && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.explanation_id.clone(),
                    revision: Some(c.explanation_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.analysis_id.clone(),
                    revision: Some(c.analysis_id.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.chain_summary.ordered_refs.is_empty()
                        && c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.reconstruction_id.clone(),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.gaps.is_empty(),
                    lineage_empty: c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceCompleteness {
                    artefact_ref: c.state_id.clone(),
                    revision: Some(c.revision.clone()),
                    recorded_level: Some(c.completeness.as_str().to_string()),
                    has_gaps: !c.unknowns.is_empty() || !c.contradictions.is_empty(),
                    lineage_empty: c.sources.is_empty(),
                })
            }
        );

        observable_states.sort();
        observable_states.dedup();
        missing_observations.sort();
        missing_observations.dedup();
        unavailable_artefacts.sort();
        unavailable_artefacts.dedup();

        let assessment = CompletenessAssessment::assemble(
            observed_scope.clone(),
            participating.clone(),
            observable_states,
        );
        let completeness = derive_completeness(
            surfaces_requested,
            surfaces_available,
            &observations,
            &gaps,
            contradictory_seen,
        );
        let diagnostics = CompletenessDiagnostics::assemble(
            completeness.as_str(),
            missing_observations,
            unavailable_artefacts,
            vec![
                "Completeness diagnostics are observational only".into(),
                "Completeness never implies truth".into(),
                "Observation never repairs, fills, or estimates evidence".into(),
            ],
        );
        let lineage = CompletenessLineage::from_contributors(
            contributing,
            revisions.clone(),
            lineage_evidence,
        );

        let narrative_summary = format!(
            "Evidence completeness: {}/{} sources available, {} observation(s), {} gap(s), completeness={}",
            surfaces_available,
            surfaces_requested,
            observations.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Observed recorded evidence completeness for workspace {workspace_id}. \
             Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer observes completeness only; it never repairs, fills, or estimates evidence."
        );
        let limitations = vec![
            "Workspace Evidence Completeness Engine observes recorded completeness only".into(),
            "It never completes evidence or changes upstream snapshots".into(),
            "Completeness never implies truth, validity, or authority".into(),
            "Partial remains partial; unavailable remains unavailable".into(),
        ];

        let snap = Self {
            completeness_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}|{}",
                    revisions.join(","),
                    completeness.as_str()
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceCompletenessStatus::Current,
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
        scope: CompletenessScope,
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
    ) -> Result<Self, EvidenceCompletenessError> {
        Self::compose(
            workspace_id,
            generated_at,
            scope,
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
        self.status = EvidenceCompletenessStatus::Superseded;
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

    pub fn validate(&self) -> Result<(), EvidenceCompletenessError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceCompletenessError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceCompletenessError::MustNotBeActionable);
        }
        if !self.assessment.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.observations.iter().any(|o| !o.is_non_actionable())
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceCompletenessError::MustNotBeActionable);
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
pub struct EvidenceCompletenessHistoryEntry {
    pub completeness_id: String,
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

impl EvidenceCompletenessHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceCompletenessSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            completeness_id: snap.completeness_id.clone(),
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
pub struct WorkspaceEvidenceCompletenessProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceCompletenessSnapshot>,
    pub history: Vec<EvidenceCompletenessHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceCompletenessProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceCompletenessSnapshot>,
        history: Vec<EvidenceCompletenessHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceCompletenessSummary {
        WorkspaceEvidenceCompletenessSummary {
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
            narrative_summary: self.current.as_ref().map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceCompletenessSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub observation_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceCompletenessHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceCompletenessExplanation {
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

impl WorkspaceEvidenceCompletenessExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceCompletenessSnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_completeness_explanation:{}", snap.completeness_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
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
                "Completeness explanation is observational only".into(),
                "Unknown remains unknown".into(),
                "Partial remains partial".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceCompleteness {
    artefact_ref: String,
    revision: Option<String>,
    recorded_level: Option<String>,
    has_gaps: bool,
    lineage_empty: bool,
}

fn classify_completeness(sig: &SurfaceCompleteness) -> CompletenessObservationState {
    if let Some(level) = sig.recorded_level.as_deref() {
        match level {
            "unavailable" => return CompletenessObservationState::Unavailable,
            "unknown" => return CompletenessObservationState::Unknown,
            "partial" | "contradictory" => return CompletenessObservationState::Partial,
            "complete" if !sig.has_gaps => return CompletenessObservationState::Complete,
            "complete" => return CompletenessObservationState::Partial,
            _ => {}
        }
    }
    if sig.has_gaps {
        return CompletenessObservationState::Partial;
    }
    if sig.lineage_empty {
        return CompletenessObservationState::Unknown;
    }
    CompletenessObservationState::Complete
}

fn describe_state(
    surface: &str,
    state: CompletenessObservationState,
    sig: &SurfaceCompleteness,
) -> String {
    match state {
        CompletenessObservationState::Complete => format!(
            "Surface '{surface}' records complete completeness metadata; observation makes no truth claim"
        ),
        CompletenessObservationState::Partial => format!(
            "Surface '{surface}' records partial completeness metadata; partial remains partial"
        ),
        CompletenessObservationState::Unknown => format!(
            "Surface '{surface}' completeness is unknown; observation does not estimate"
        ),
        CompletenessObservationState::Unavailable => format!(
            "Surface '{surface}' completeness unavailable; observation does not replace it (artefact={})",
            sig.artefact_ref
        ),
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    observations: &[CompletenessObservation],
    gaps: &[CompletenessGap],
    contradictory_seen: bool,
) -> EvidenceCompletenessCompleteness {
    if requested == 0 {
        return EvidenceCompletenessCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceCompletenessCompleteness::Unavailable;
    }
    if contradictory_seen {
        return EvidenceCompletenessCompleteness::Contradictory;
    }
    if available == requested
        && gaps.is_empty()
        && observations
            .iter()
            .all(|o| o.state == CompletenessObservationState::Complete)
    {
        return EvidenceCompletenessCompleteness::Complete;
    }
    if available == requested
        && gaps.is_empty()
        && observations
            .iter()
            .any(|o| o.state == CompletenessObservationState::Unknown)
    {
        return EvidenceCompletenessCompleteness::Unknown;
    }
    if available < requested
        || !gaps.is_empty()
        || observations.iter().any(|o| {
            matches!(
                o.state,
                CompletenessObservationState::Partial | CompletenessObservationState::Unavailable
            )
        })
    {
        return EvidenceCompletenessCompleteness::Partial;
    }
    EvidenceCompletenessCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceCompletenessError> {
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
        "please repair",
        "must repair",
        "fill the gap",
        "invent missing",
        "fabricate completeness",
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(EvidenceCompletenessError::MustNotBeActionable);
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

    fn empty_compose() -> WorkspaceEvidenceCompletenessSnapshot {
        WorkspaceEvidenceCompletenessSnapshot::compose(
            "ws",
            "t0",
            CompletenessScope::all_surfaces(),
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
            snap.completeness,
            EvidenceCompletenessCompleteness::Unavailable
        );
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == CompletenessGap::KIND_UNAVAILABLE_ARTEFACT));
        assert!(snap.observations.is_empty());
        assert!(snap.is_non_executing());
    }

    #[test]
    fn identical_inputs_produce_identical_outputs() {
        let left = WorkspaceEvidenceCompletenessSnapshot::compose_deterministic(
            "ws",
            "t1",
            CompletenessScope::all_surfaces(),
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
        let right = WorkspaceEvidenceCompletenessSnapshot::compose_deterministic(
            "ws",
            "t1",
            CompletenessScope::all_surfaces(),
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
        assert_eq!(left.completeness_id, right.completeness_id);
        assert_eq!(left.observations, right.observations);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = empty_compose();
        snap.mark_superseded("t1");
        let entry = EvidenceCompletenessHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj =
            WorkspaceEvidenceCompletenessProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn partial_preserved() {
        let sig = SurfaceCompleteness {
            artefact_ref: "a".into(),
            revision: Some("r1".into()),
            recorded_level: Some("partial".into()),
            has_gaps: false,
            lineage_empty: false,
        };
        assert_eq!(
            classify_completeness(&sig),
            CompletenessObservationState::Partial
        );
    }

    #[test]
    fn unknown_preserved() {
        let sig = SurfaceCompleteness {
            artefact_ref: "a".into(),
            revision: None,
            recorded_level: Some("unknown".into()),
            has_gaps: false,
            lineage_empty: false,
        };
        assert_eq!(
            classify_completeness(&sig),
            CompletenessObservationState::Unknown
        );
    }

    #[test]
    fn unavailable_observation_state_preserved() {
        let sig = SurfaceCompleteness {
            artefact_ref: "a".into(),
            revision: None,
            recorded_level: Some("unavailable".into()),
            has_gaps: false,
            lineage_empty: true,
        };
        assert_eq!(
            classify_completeness(&sig),
            CompletenessObservationState::Unavailable
        );
    }

    #[test]
    fn completeness_never_implies_truth() {
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
    fn deterministic_completeness_assessment() {
        let snap = empty_compose();
        assert_eq!(snap.assessment.observed_scope.len(), 14);
        assert!(snap.assessment.participating_artefacts.is_empty());
        assert!(snap
            .assessment
            .observable_completeness_state
            .iter()
            .any(|s| s == "unavailable"));
    }
}
