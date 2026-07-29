//! Workspace Evidence Coverage Engine — Programme IV Batch 4.
//!
//! Measure evidence coverage. Never measure truth.
//! coverage ≠ correctness; completeness ≠ confidence; gap ≠ recommendation.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
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
pub enum EvidenceCoverageError {
    #[error("invalid evidence coverage status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence coverage completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence coverage artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence coverage artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("coverage metrics require non-negative observable quantities")]
    InvalidMetric,

    #[error("evidence coverage snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCoverageStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceCoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceCoverageError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceCoverageError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCoverageCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceCoverageCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceCoverageError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceCoverageError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream surfaces participate in coverage observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageScope {
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

impl CoverageScope {
    pub fn all_surfaces() -> Self {
        Self {
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
pub struct EvidenceCoverageEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceCoverageEvidenceRef {
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

/// Observable coverage assessment — no interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageAssessment {
    pub assessment_id: String,
    pub requested_scope: Vec<String>,
    pub observed_sources: Vec<String>,
    pub available_evidence: Vec<String>,
    pub unavailable_evidence: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoverageAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coverage_assessment:";

    pub fn assemble(
        requested_scope: Vec<String>,
        observed_sources: Vec<String>,
        available_evidence: Vec<String>,
        unavailable_evidence: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "req={}|obs={}|avail={}|unavail={}",
            requested_scope.len(),
            observed_sources.len(),
            available_evidence.len(),
            unavailable_evidence.len()
        ));
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, digest),
            requested_scope,
            observed_sources,
            available_evidence,
            unavailable_evidence,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Observable quantities only — no quality judgement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageMetric {
    pub metric_id: String,
    pub observed_sources: usize,
    pub reachable_artefacts: usize,
    pub missing_references: usize,
    pub unavailable_snapshots: usize,
    pub lineage_completeness_numerator: usize,
    pub lineage_completeness_denominator: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoverageMetric {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coverage_metric:";

    pub fn observe(
        observed_sources: usize,
        reachable_artefacts: usize,
        missing_references: usize,
        unavailable_snapshots: usize,
        lineage_completeness_numerator: usize,
        lineage_completeness_denominator: usize,
    ) -> Result<Self, EvidenceCoverageError> {
        if lineage_completeness_numerator > lineage_completeness_denominator {
            return Err(EvidenceCoverageError::InvalidMetric);
        }
        let digest = stable_digest(&format!(
            "{observed_sources}|{reachable_artefacts}|{missing_references}|{unavailable_snapshots}|{lineage_completeness_numerator}/{lineage_completeness_denominator}"
        ));
        Ok(Self {
            metric_id: format!("{}{}", Self::ID_PREFIX, digest),
            observed_sources,
            reachable_artefacts,
            missing_references,
            unavailable_snapshots,
            lineage_completeness_numerator,
            lineage_completeness_denominator,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Unavailable / broken / missing / incomplete — never automatically resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceCoverageEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoverageGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coverage_gap:";
    pub const KIND_UNAVAILABLE: &'static str = "unavailable_evidence";
    pub const KIND_BROKEN: &'static str = "broken_references";
    pub const KIND_MISSING_PROVENANCE: &'static str = "missing_provenance";
    pub const KIND_INCOMPLETE: &'static str = "incomplete_retrieval";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceCoverageEvidenceRef>,
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

/// Exact upstream artefacts that contributed — no inferred provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceCoverageEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoverageLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coverage_lineage:";

    pub fn from_contributors(
        contributing_artefacts: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceCoverageEvidenceRef>,
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

/// Deterministic diagnostics — no recommendations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageDiagnostics {
    pub diagnostics_id: String,
    pub completeness_state: String,
    pub observed_limitations: Vec<String>,
    pub unavailable_sources: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoverageDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coverage_diagnostics:";

    pub fn assemble(
        completeness_state: impl Into<String>,
        observed_limitations: Vec<String>,
        unavailable_sources: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let completeness_state = completeness_state.into();
        let digest = stable_digest(&format!(
            "{completeness_state}|{}|{}",
            observed_limitations.len(),
            unavailable_sources.len()
        ));
        Self {
            diagnostics_id: format!("{}{}", Self::ID_PREFIX, digest),
            completeness_state,
            observed_limitations,
            unavailable_sources,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable evidence coverage artefact — observable completeness only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceCoverageSnapshot {
    pub coverage_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceCoverageStatus,
    pub superseded_at: Option<String>,
    pub scope: CoverageScope,
    pub assessment: CoverageAssessment,
    pub metrics: CoverageMetric,
    pub gaps: Vec<CoverageGap>,
    pub lineage: CoverageLineage,
    pub diagnostics: CoverageDiagnostics,
    pub completeness: EvidenceCoverageCompleteness,
    pub provenance_links: Vec<EvidenceCoverageEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceCoverageSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_coverage:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: CoverageScope,
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
    ) -> Result<Self, EvidenceCoverageError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let mut requested = Vec::new();
        let mut observed = Vec::new();
        let mut available = Vec::new();
        let mut unavailable = Vec::new();
        let mut gaps = Vec::new();
        let mut contributing = Vec::new();
        let mut revisions = Vec::new();
        let mut lineage_evidence = Vec::new();
        let mut provenance_links = Vec::new();
        let mut reachable = 0usize;
        let mut missing_refs = 0usize;
        let mut unavailable_snapshots = 0usize;
        let mut lineage_num = 0usize;
        let mut lineage_den = 0usize;
        let mut surfaces_requested = 0usize;
        let mut surfaces_available = 0usize;

        macro_rules! observe {
            ($flag:expr, $name:expr, $opt:expr, $extract:expr) => {
                if $flag {
                    surfaces_requested += 1;
                    requested.push($name.to_string());
                    match $opt {
                        Some(proj) => {
                            observed.push($name.to_string());
                            if let Some(obs) = $extract(proj) {
                                surfaces_available += 1;
                                available.push(obs.artefact_ref.clone());
                                contributing.push(format!("{}:{}", $name, obs.artefact_ref));
                                if let Some(rev) = &obs.revision {
                                    revisions.push(rev.clone());
                                }
                                let link = EvidenceCoverageEvidenceRef::link(
                                    $name,
                                    obs.artefact_ref.clone(),
                                    obs.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link);
                                reachable += obs.reachable_count;
                                missing_refs += obs.missing_count;
                                lineage_num += obs.lineage_present;
                                lineage_den += 1;
                                if obs.incomplete {
                                    gaps.push(CoverageGap::record(
                                        CoverageGap::KIND_INCOMPLETE,
                                        format!(
                                            "Surface '{}' reports incomplete retrieval — never fabricate completeness",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![],
                                    ));
                                }
                                for broken in obs.broken_refs {
                                    gaps.push(CoverageGap::record(
                                        CoverageGap::KIND_BROKEN,
                                        format!(
                                            "Broken reference '{broken}' on surface '{}' — never repair lineage",
                                            $name
                                        ),
                                        vec![broken],
                                        vec![],
                                    ));
                                }
                                for miss in obs.missing_provenance {
                                    gaps.push(CoverageGap::record(
                                        CoverageGap::KIND_MISSING_PROVENANCE,
                                        format!(
                                            "Missing provenance on surface '{}' — never invent evidence",
                                            $name
                                        ),
                                        vec![miss],
                                        vec![],
                                    ));
                                }
                            } else {
                                unavailable_snapshots += 1;
                                unavailable.push($name.to_string());
                                gaps.push(CoverageGap::record(
                                    CoverageGap::KIND_UNAVAILABLE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact — never invent evidence",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                                lineage_den += 1;
                            }
                        }
                        None => {
                            unavailable_snapshots += 1;
                            unavailable.push($name.to_string());
                            gaps.push(CoverageGap::record(
                                CoverageGap::KIND_UNAVAILABLE,
                                format!(
                                    "Upstream surface '{}' unavailable — never invent evidence or completeness",
                                    $name
                                ),
                                vec![$name.into()],
                                vec![],
                            ));
                            lineage_den += 1;
                        }
                    }
                }
            };
        }

        observe!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.trace_id.clone(),
                    revision: Some(c.trace_id.clone()),
                    reachable_count: c.chain.as_ref().map(|ch| ch.segments.len()).unwrap_or(0),
                    missing_count: c.gaps.len(),
                    lineage_present: if c.lineage.participating_snapshots.is_empty() {
                        0
                    } else {
                        1
                    },
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                    broken_refs: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "broken_provenance")
                        .flat_map(|g| g.affected_references.clone())
                        .collect(),
                    missing_provenance: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "missing_lineage")
                        .flat_map(|g| g.affected_references.clone())
                        .collect(),
                })
            }
        );
        observe!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.navigation_id.clone(),
                    revision: Some(c.navigation_id.clone()),
                    reachable_count: c.paths.len(),
                    missing_count: c.gaps.len(),
                    lineage_present: if c.lineage.participating_snapshots.is_empty() {
                        0
                    } else {
                        1
                    },
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                    broken_refs: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "broken_lineage")
                        .flat_map(|g| g.affected_references.clone())
                        .collect(),
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.query_id.clone(),
                    revision: Some(c.query_id.clone()),
                    reachable_count: c.result.match_count,
                    missing_count: c.gaps.len(),
                    lineage_present: if c.lineage.contributing_snapshots.is_empty() {
                        0
                    } else {
                        1
                    },
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                    broken_refs: vec![],
                    missing_provenance: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "missing_lineage")
                        .map(|g| g.unavailable_source.clone())
                        .collect(),
                })
            }
        );
        observe!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.hub_id.clone(),
                    revision: Some(c.hub_id.clone()),
                    reachable_count: c.packages.len(),
                    missing_count: c.gaps.len(),
                    lineage_present: if c.lineage.upstream_sources.is_empty() {
                        0
                    } else {
                        1
                    },
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.integration_id.clone(),
                    revision: Some(c.integration_id.clone()),
                    reachable_count: c.provenance_links.len(),
                    missing_count: 0,
                    lineage_present: if c.provenance_links.is_empty() { 0 } else { 1 },
                    incomplete: false,
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.understanding_id.clone(),
                    revision: Some(c.understanding_id.clone()),
                    reachable_count: 1,
                    missing_count: 0,
                    lineage_present: 1,
                    incomplete: false,
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.explanation_id.clone(),
                    revision: Some(c.explanation_id.clone()),
                    reachable_count: 1,
                    missing_count: 0,
                    lineage_present: 1,
                    incomplete: false,
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.analysis_id.clone(),
                    revision: Some(c.analysis_id.clone()),
                    reachable_count: 1,
                    missing_count: 0,
                    lineage_present: 1,
                    incomplete: false,
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.reconstruction_id.clone(),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    reachable_count: 1,
                    missing_count: 0,
                    lineage_present: 1,
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );
        observe!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceObservation {
                    artefact_ref: c.state_id.clone(),
                    revision: Some(c.revision.clone()),
                    reachable_count: 1,
                    missing_count: 0,
                    lineage_present: 1,
                    incomplete: c.consistency.as_str() == "unknown",
                    broken_refs: vec![],
                    missing_provenance: vec![],
                })
            }
        );

        let assessment = CoverageAssessment::assemble(
            requested.clone(),
            observed.clone(),
            available.clone(),
            unavailable.clone(),
        );
        let metrics = CoverageMetric::observe(
            observed.len(),
            reachable,
            missing_refs,
            unavailable_snapshots,
            lineage_num,
            lineage_den,
        )?;
        let completeness =
            derive_completeness(surfaces_requested, surfaces_available, &gaps);
        let diagnostics = CoverageDiagnostics::assemble(
            completeness.as_str(),
            vec![
                "Coverage reports observable completeness only".into(),
                "coverage does not imply correctness".into(),
                "completeness does not imply confidence".into(),
            ],
            unavailable.clone(),
            vec![
                "Coverage diagnostics are observational only".into(),
                "gap is not an action directive".into(),
            ],
        );
        let lineage =
            CoverageLineage::from_contributors(contributing, revisions.clone(), lineage_evidence);

        let narrative_summary = format!(
            "Evidence coverage: {}/{} sources available, {} gap(s), completeness={}",
            surfaces_available,
            surfaces_requested,
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Observed evidence completeness for workspace {workspace_id}. \
             Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer measures observable coverage only — it never evaluates truth, \
             infers missing evidence, or implies confidence."
        );
        let limitations = vec![
            "Workspace Evidence Coverage Engine measures observable evidence completeness only".into(),
            "It never infers missing evidence or evaluates truth".into(),
            "It never becomes the authority for any upstream intelligence layer".into(),
            "Coverage never implies correctness, confidence, or action".into(),
        ];

        let mut snap = Self {
            coverage_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceCoverageStatus::Current,
            superseded_at: None,
            scope,
            assessment,
            metrics,
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
        scope: CoverageScope,
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
    ) -> Result<Self, EvidenceCoverageError> {
        Self::compose(
            workspace_id,
            generated_at,
            scope,
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
        self.status = EvidenceCoverageStatus::Superseded;
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
            && self.metrics.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.diagnostics.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), EvidenceCoverageError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceCoverageError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceCoverageError::MustNotBeActionable);
        }
        if !self.assessment.is_non_actionable()
            || !self.metrics.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceCoverageError::MustNotBeActionable);
        }
        if self.metrics.lineage_completeness_numerator
            > self.metrics.lineage_completeness_denominator
        {
            return Err(EvidenceCoverageError::InvalidMetric);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for note in &self.diagnostics.notes {
            reject_forbidden_phrases(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCoverageHistoryEntry {
    pub coverage_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub observed_sources: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl EvidenceCoverageHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceCoverageSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            coverage_id: snap.coverage_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            observed_sources: snap.metrics.observed_sources,
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
pub struct WorkspaceEvidenceCoverageProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceCoverageSnapshot>,
    pub history: Vec<EvidenceCoverageHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceCoverageProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceCoverageSnapshot>,
        history: Vec<EvidenceCoverageHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceCoverageSummary {
        WorkspaceEvidenceCoverageSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().to_string()),
            observed_sources: self
                .current
                .as_ref()
                .map(|c| c.metrics.observed_sources)
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
pub struct WorkspaceEvidenceCoverageSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub observed_sources: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceCoverageHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceCoverageExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub metric_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceEvidenceCoverageExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceCoverageSnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_coverage_explanation:{}", snap.coverage_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            metric_summaries: vec![format!(
                "observed={} reachable={} missing={} unavailable={} lineage={}/{}",
                snap.metrics.observed_sources,
                snap.metrics.reachable_artefacts,
                snap.metrics.missing_references,
                snap.metrics.unavailable_snapshots,
                snap.metrics.lineage_completeness_numerator,
                snap.metrics.lineage_completeness_denominator
            )],
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
                "Coverage explanation is observational only".into(),
                "Unknown remains unknown".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceObservation {
    artefact_ref: String,
    revision: Option<String>,
    reachable_count: usize,
    missing_count: usize,
    lineage_present: usize,
    incomplete: bool,
    broken_refs: Vec<String>,
    missing_provenance: Vec<String>,
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[CoverageGap],
) -> EvidenceCoverageCompleteness {
    if requested == 0 {
        return EvidenceCoverageCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceCoverageCompleteness::Unavailable;
    }
    if available == requested && gaps.is_empty() {
        return EvidenceCoverageCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() {
        return EvidenceCoverageCompleteness::Partial;
    }
    EvidenceCoverageCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceCoverageError> {
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
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(EvidenceCoverageError::MustNotBeActionable);
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

    #[test]
    fn unavailable_sources_preserved() {
        let snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        assert_eq!(snap.completeness, EvidenceCoverageCompleteness::Unavailable);
        assert_eq!(snap.gaps.len(), 10);
        assert_eq!(snap.metrics.unavailable_snapshots, 10);
        assert!(snap.is_non_executing());
        assert!(snap.lineage.contributing_artefacts.is_empty());
    }

    #[test]
    fn identical_snapshots_produce_identical_coverage() {
        let left = WorkspaceEvidenceCoverageSnapshot::compose_deterministic(
            "ws",
            "t1",
            CoverageScope::all_surfaces(),
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
        let right = WorkspaceEvidenceCoverageSnapshot::compose_deterministic(
            "ws",
            "t1",
            CoverageScope::all_surfaces(),
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
        assert_eq!(left.coverage_id, right.coverage_id);
        assert_eq!(left.metrics, right.metrics);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.lineage, right.lineage);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        let entry = EvidenceCoverageHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj =
            WorkspaceEvidenceCoverageProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn missing_evidence_preserved() {
        let state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope {
                include_evidence_trace: false,
                include_evidence_navigation: false,
                include_semantic_query: false,
                include_intelligence_hub: false,
                include_knowledge_integration: false,
                include_contextual: false,
                include_explanation: false,
                include_temporal: false,
                include_reconstruction: false,
                include_state: true,
            },
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(&state),
        )
        .unwrap();
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == CoverageGap::KIND_UNAVAILABLE));
        assert!(snap.lineage.contributing_artefacts.is_empty());
    }

    #[test]
    fn lineage_deterministic() {
        let a = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        let b = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        assert_eq!(a.lineage.lineage_id, b.lineage.lineage_id);
    }

    #[test]
    fn coverage_never_implies_truth() {
        let snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        assert!(snap.limitations.iter().any(|l| l.contains("never") && l.contains("truth")));
        assert!(!snap.narrative.to_ascii_lowercase().contains("is correct"));
        assert!(!snap.narrative.to_ascii_lowercase().contains("is true"));
    }

    #[test]
    fn non_executable_snapshots() {
        let snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        assert!(snap.is_non_executing());
        let mut bad = snap;
        bad.actionable = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn deterministic_coverage_metrics() {
        let snap = WorkspaceEvidenceCoverageSnapshot::compose(
            "ws",
            "t0",
            CoverageScope::all_surfaces(),
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
        assert_eq!(snap.metrics.observed_sources, 0);
        assert_eq!(snap.metrics.unavailable_snapshots, 10);
        assert_eq!(snap.metrics.lineage_completeness_numerator, 0);
        assert_eq!(snap.metrics.lineage_completeness_denominator, 10);
    }
}
