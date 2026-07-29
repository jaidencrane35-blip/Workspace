//! Workspace Evidence Dependency Engine — Programme IV Batch 6.
//!
//! Observe dependency. Never create dependency.
//! dependency ≠ causation; relationship ≠ execution; observation ≠ scheduling.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_evidence_consistency::WorkspaceEvidenceConsistencyProjection;
use crate::workspace_evidence_coverage::WorkspaceEvidenceCoverageProjection;
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
pub enum EvidenceDependencyError {
    #[error("invalid evidence dependency status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence dependency completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence dependency artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence dependency artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("evidence dependency snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDependencyStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceDependencyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceDependencyError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceDependencyError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDependencyCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceDependencyCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceDependencyError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceDependencyError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream surfaces participate in dependency observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyScope {
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

impl DependencyScope {
    pub fn all_surfaces() -> Self {
        Self {
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
pub struct EvidenceDependencyEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceDependencyEvidenceRef {
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

/// Observable dependency assessment — no interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyAssessment {
    pub assessment_id: String,
    pub requested_scope: Vec<String>,
    pub participating_artefacts: Vec<String>,
    pub observed_dependency_set: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_assessment:";

    pub fn assemble(
        requested_scope: Vec<String>,
        participating_artefacts: Vec<String>,
        observed_dependency_set: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "scope={}|part={}|deps={}",
            requested_scope.len(),
            participating_artefacts.len(),
            observed_dependency_set.len()
        ));
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, digest),
            requested_scope,
            participating_artefacts,
            observed_dependency_set,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// One recorded evidence artefact — reference only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyNode {
    pub node_id: String,
    pub artefact_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyNode {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_node:";

    pub fn reference(
        origin_domain: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
    ) -> Self {
        let origin_domain = origin_domain.into();
        let artefact_ref = artefact_ref.into();
        let digest = stable_digest(&format!("{origin_domain}|{artefact_ref}"));
        Self {
            node_id: format!("{}{}", Self::ID_PREFIX, digest),
            artefact_ref,
            origin_domain,
            source_revision,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// One recorded dependency — never inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyRelationship {
    pub relationship_id: String,
    pub source_reference: String,
    pub target_reference: String,
    pub relationship_type: String,
    pub recorded_lineage: String,
    pub evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyRelationship {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_relationship:";
    pub const TYPE_RECORDED_LINEAGE: &'static str = "recorded_lineage";
    pub const TYPE_RECORDED_HOP: &'static str = "recorded_hop";
    pub const TYPE_RECORDED_PROVENANCE: &'static str = "recorded_provenance";

    pub fn recorded(
        source_reference: impl Into<String>,
        target_reference: impl Into<String>,
        relationship_type: impl Into<String>,
        recorded_lineage: impl Into<String>,
        evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
    ) -> Self {
        let source_reference = source_reference.into();
        let target_reference = target_reference.into();
        let relationship_type = relationship_type.into();
        let recorded_lineage = recorded_lineage.into();
        let digest = stable_digest(&format!(
            "{source_reference}|{target_reference}|{relationship_type}|{recorded_lineage}"
        ));
        Self {
            relationship_id: format!("{}{}", Self::ID_PREFIX, digest),
            source_reference,
            target_reference,
            relationship_type,
            recorded_lineage,
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

/// Unavailable / broken / incomplete / missing — never repaired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_gap:";
    pub const KIND_UNAVAILABLE: &'static str = "unavailable_dependency";
    pub const KIND_BROKEN: &'static str = "broken_lineage";
    pub const KIND_INCOMPLETE: &'static str = "incomplete_dependency_chain";
    pub const KIND_MISSING_SOURCE: &'static str = "missing_source";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
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

/// Exact upstream artefacts that contributed recorded dependency structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_lineage:";

    pub fn from_contributors(
        contributing_artefacts: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceDependencyEvidenceRef>,
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

/// Deterministic diagnostics — no recommendations or repairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyDiagnostics {
    pub diagnostics_id: String,
    pub completeness: String,
    pub reachable_dependency_count: usize,
    pub unavailable_references: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl DependencyDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "dependency_diagnostics:";

    pub fn assemble(
        completeness: impl Into<String>,
        reachable_dependency_count: usize,
        unavailable_references: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        let completeness = completeness.into();
        let digest = stable_digest(&format!(
            "{completeness}|{reachable_dependency_count}|{}",
            unavailable_references.len()
        ));
        Self {
            diagnostics_id: format!("{}{}", Self::ID_PREFIX, digest),
            completeness,
            reachable_dependency_count,
            unavailable_references,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable evidence dependency artefact — observe recorded structure only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceDependencySnapshot {
    pub dependency_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceDependencyStatus,
    pub superseded_at: Option<String>,
    pub scope: DependencyScope,
    pub assessment: DependencyAssessment,
    pub nodes: Vec<DependencyNode>,
    pub relationships: Vec<DependencyRelationship>,
    pub gaps: Vec<DependencyGap>,
    pub lineage: DependencyLineage,
    pub diagnostics: DependencyDiagnostics,
    pub completeness: EvidenceDependencyCompleteness,
    pub provenance_links: Vec<EvidenceDependencyEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceDependencySnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_dependency:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: DependencyScope,
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
    ) -> Result<Self, EvidenceDependencyError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut requested_scope = Vec::new();
        let mut nodes = Vec::new();
        let mut relationships = Vec::new();
        let mut gaps = Vec::new();
        let mut contributing = Vec::new();
        let mut revisions = Vec::new();
        let mut lineage_evidence = Vec::new();
        let mut provenance_links = Vec::new();
        let mut unavailable = Vec::new();
        let mut participating = Vec::new();
        let mut surfaces_requested = 0usize;
        let mut surfaces_available = 0usize;

        macro_rules! observe {
            ($flag:expr, $name:expr, $opt:expr, $extract:expr) => {
                if $flag {
                    surfaces_requested += 1;
                    requested_scope.push($name.to_string());
                    match $opt {
                        Some(proj) => {
                            if let Some(extracted) = $extract(proj) {
                                surfaces_available += 1;
                                participating.push(extracted.artefact_ref.clone());
                                contributing.push(format!("{}:{}", $name, extracted.artefact_ref));
                                if let Some(rev) = &extracted.revision {
                                    revisions.push(rev.clone());
                                }
                                let link = EvidenceDependencyEvidenceRef::link(
                                    $name,
                                    extracted.artefact_ref.clone(),
                                    extracted.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link);
                                nodes.push(DependencyNode::reference(
                                    $name,
                                    extracted.artefact_ref.clone(),
                                    extracted.revision.clone(),
                                ));
                                for dep in extracted.recorded_deps {
                                    nodes.push(DependencyNode::reference(
                                        dep.target_domain.clone(),
                                        dep.target_ref.clone(),
                                        dep.target_revision.clone(),
                                    ));
                                    relationships.push(DependencyRelationship::recorded(
                                        extracted.artefact_ref.clone(),
                                        dep.target_ref.clone(),
                                        dep.relationship_type.clone(),
                                        dep.recorded_lineage.clone(),
                                        vec![EvidenceDependencyEvidenceRef::link(
                                            $name,
                                            dep.target_ref.clone(),
                                            dep.target_revision.clone(),
                                        )],
                                    ));
                                }
                                for broken in extracted.broken_refs {
                                    gaps.push(DependencyGap::record(
                                        DependencyGap::KIND_BROKEN,
                                        format!(
                                            "Broken lineage '{broken}' on surface '{}' — never invent or repair dependency",
                                            $name
                                        ),
                                        vec![broken],
                                        vec![],
                                    ));
                                }
                                for missing in extracted.missing_sources {
                                    gaps.push(DependencyGap::record(
                                        DependencyGap::KIND_MISSING_SOURCE,
                                        format!(
                                            "Missing source '{missing}' on surface '{}' — never invent dependency",
                                            $name
                                        ),
                                        vec![missing],
                                        vec![],
                                    ));
                                }
                                if extracted.incomplete {
                                    gaps.push(DependencyGap::record(
                                        DependencyGap::KIND_INCOMPLETE,
                                        format!(
                                            "Surface '{}' reports incomplete dependency chain — never bridge missing links",
                                            $name
                                        ),
                                        vec![$name.into()],
                                        vec![],
                                    ));
                                }
                            } else {
                                unavailable.push($name.to_string());
                                gaps.push(DependencyGap::record(
                                    DependencyGap::KIND_UNAVAILABLE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact — never invent dependency",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                            }
                        }
                        None => {
                            unavailable.push($name.to_string());
                            gaps.push(DependencyGap::record(
                                DependencyGap::KIND_UNAVAILABLE,
                                format!(
                                    "Upstream surface '{}' unavailable — never invent dependency structure",
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
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.consistency_id.clone(),
                    revision: Some(c.consistency_id.clone()),
                    recorded_deps: c
                        .lineage
                        .contributing_artefacts
                        .iter()
                        .map(|a| RecordedDep {
                            target_domain: "recorded_contributor".into(),
                            target_ref: a.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("consistency:{}", c.consistency_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                })
            }
        );
        observe!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.coverage_id.clone(),
                    revision: Some(c.coverage_id.clone()),
                    recorded_deps: c
                        .lineage
                        .contributing_artefacts
                        .iter()
                        .map(|a| RecordedDep {
                            target_domain: "recorded_contributor".into(),
                            target_ref: a.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("coverage:{}", c.coverage_id),
                        })
                        .collect(),
                    broken_refs: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "broken_references")
                        .flat_map(|g| g.affected_references.clone())
                        .collect(),
                    missing_sources: c.assessment.unavailable_evidence.clone(),
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                })
            }
        );
        observe!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| {
                    let mut recorded_deps = Vec::new();
                    if let Some(chain) = &c.chain {
                        for seg in &chain.segments {
                            recorded_deps.push(RecordedDep {
                                target_domain: "trace_segment".into(),
                                target_ref: seg.destination_reference.clone(),
                                target_revision: seg.originating_revision.clone(),
                                relationship_type: DependencyRelationship::TYPE_RECORDED_HOP.into(),
                                recorded_lineage: format!(
                                    "trace:{}:{}",
                                    c.trace_id, seg.segment_id
                                ),
                            });
                        }
                    }
                    for snap in &c.lineage.participating_snapshots {
                        recorded_deps.push(RecordedDep {
                            target_domain: "participating_snapshot".into(),
                            target_ref: snap.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("trace:{}", c.trace_id),
                        });
                    }
                    SurfaceDeps {
                        artefact_ref: c.trace_id.clone(),
                        revision: Some(c.trace_id.clone()),
                        recorded_deps,
                        broken_refs: c
                            .gaps
                            .iter()
                            .filter(|g| g.gap_kind == "broken_provenance")
                            .flat_map(|g| g.affected_references.clone())
                            .collect(),
                        missing_sources: c
                            .gaps
                            .iter()
                            .filter(|g| g.gap_kind == "missing_lineage")
                            .flat_map(|g| g.affected_references.clone())
                            .collect(),
                        incomplete: c.completeness.as_str() == "partial"
                            || c.completeness.as_str() == "unknown",
                    }
                })
            }
        );
        observe!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| {
                    let mut recorded_deps = Vec::new();
                    for path in &c.paths {
                        recorded_deps.push(RecordedDep {
                            target_domain: "navigation_destination".into(),
                            target_ref: path.destination.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_HOP.into(),
                            recorded_lineage: format!(
                                "navigation:{}:{}",
                                c.navigation_id, path.path_id
                            ),
                        });
                        for intermediate in &path.intermediate_references {
                            recorded_deps.push(RecordedDep {
                                target_domain: "navigation_intermediate".into(),
                                target_ref: intermediate.clone(),
                                target_revision: None,
                                relationship_type: DependencyRelationship::TYPE_RECORDED_HOP.into(),
                                recorded_lineage: format!(
                                    "navigation:{}:{}",
                                    c.navigation_id, path.path_id
                                ),
                            });
                        }
                        for link in &path.evidence_lineage {
                            recorded_deps.push(RecordedDep {
                                target_domain: link.origin_domain.clone(),
                                target_ref: link.external_ref.clone(),
                                target_revision: link.source_revision.clone(),
                                relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                    .into(),
                                recorded_lineage: format!(
                                    "navigation:{}:{}",
                                    c.navigation_id, path.path_id
                                ),
                            });
                        }
                    }
                    for snap in &c.lineage.participating_snapshots {
                        recorded_deps.push(RecordedDep {
                            target_domain: "participating_snapshot".into(),
                            target_ref: snap.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("navigation:{}", c.navigation_id),
                        });
                    }
                    SurfaceDeps {
                        artefact_ref: c.navigation_id.clone(),
                        revision: Some(c.navigation_id.clone()),
                        recorded_deps,
                        broken_refs: c
                            .gaps
                            .iter()
                            .filter(|g| g.gap_kind == "broken_lineage")
                            .flat_map(|g| g.affected_references.clone())
                            .collect(),
                        missing_sources: vec![],
                        incomplete: c.completeness.as_str() == "partial"
                            || c.completeness.as_str() == "unknown",
                    }
                })
            }
        );
        observe!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.query_id.clone(),
                    revision: Some(c.query_id.clone()),
                    recorded_deps: c
                        .lineage
                        .contributing_snapshots
                        .iter()
                        .map(|s| RecordedDep {
                            target_domain: "contributing_snapshot".into(),
                            target_ref: s.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("semantic_query:{}", c.query_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: c
                        .gaps
                        .iter()
                        .filter(|g| g.gap_kind == "missing_lineage")
                        .map(|g| g.unavailable_source.clone())
                        .collect(),
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                })
            }
        );
        observe!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.hub_id.clone(),
                    revision: Some(c.hub_id.clone()),
                    recorded_deps: c
                        .lineage
                        .upstream_sources
                        .iter()
                        .map(|s| RecordedDep {
                            target_domain: "upstream_source".into(),
                            target_ref: s.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_LINEAGE.into(),
                            recorded_lineage: format!("intelligence_hub:{}", c.hub_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                })
            }
        );
        observe!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.integration_id.clone(),
                    revision: Some(c.integration_id.clone()),
                    recorded_deps: c
                        .provenance_links
                        .iter()
                        .map(|link| RecordedDep {
                            target_domain: link.origin_domain.clone(),
                            target_ref: link.external_ref.clone(),
                            target_revision: link.source_revision.clone(),
                            relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                .into(),
                            recorded_lineage: format!("knowledge_integration:{}", c.integration_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: c.provenance_links.is_empty(),
                })
            }
        );
        observe!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.understanding_id.clone(),
                    revision: Some(c.understanding_id.clone()),
                    recorded_deps: c
                        .provenance_links
                        .iter()
                        .map(|link| RecordedDep {
                            target_domain: link.origin_domain.clone(),
                            target_ref: link.external_ref.clone(),
                            target_revision: link.source_revision.clone(),
                            relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                .into(),
                            recorded_lineage: format!("contextual:{}", c.understanding_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: false,
                })
            }
        );
        observe!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.explanation_id.clone(),
                    revision: Some(c.explanation_id.clone()),
                    recorded_deps: c
                        .provenance_links
                        .iter()
                        .map(|link| RecordedDep {
                            target_domain: link.kind.clone(),
                            target_ref: link.external_ref.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                .into(),
                            recorded_lineage: format!("explanation:{}", c.explanation_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: false,
                })
            }
        );
        observe!(
            scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.analysis_id.clone(),
                    revision: Some(c.analysis_id.clone()),
                    recorded_deps: c
                        .provenance_links
                        .iter()
                        .map(|link| RecordedDep {
                            target_domain: link.kind.clone(),
                            target_ref: link.external_ref.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                .into(),
                            recorded_lineage: format!("temporal:{}", c.analysis_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: false,
                })
            }
        );
        observe!(
            scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.reconstruction_id.clone(),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    recorded_deps: c
                        .provenance_links
                        .iter()
                        .map(|link| RecordedDep {
                            target_domain: link.kind.clone(),
                            target_ref: link.external_ref.clone(),
                            target_revision: None,
                            relationship_type: DependencyRelationship::TYPE_RECORDED_PROVENANCE
                                .into(),
                            recorded_lineage: format!("reconstruction:{}", c.reconstruction_id),
                        })
                        .collect(),
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: c.completeness.as_str() == "partial"
                        || c.completeness.as_str() == "unknown",
                })
            }
        );
        observe!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceDeps {
                    artefact_ref: c.state_id.clone(),
                    revision: Some(c.revision.clone()),
                    recorded_deps: vec![],
                    broken_refs: vec![],
                    missing_sources: vec![],
                    incomplete: c.consistency.as_str() == "unknown",
                })
            }
        );

        // Deduplicate nodes by artefact_ref+origin_domain
        nodes.sort_by(|a, b| {
            (&a.origin_domain, &a.artefact_ref).cmp(&(&b.origin_domain, &b.artefact_ref))
        });
        nodes.dedup_by(|a, b| {
            a.origin_domain == b.origin_domain && a.artefact_ref == b.artefact_ref
        });
        relationships.sort_by(|a, b| a.relationship_id.cmp(&b.relationship_id));
        relationships.dedup_by(|a, b| a.relationship_id == b.relationship_id);

        let observed_dependency_set: Vec<String> = relationships
            .iter()
            .map(|r| {
                format!(
                    "{}->{}:{}",
                    r.source_reference, r.target_reference, r.relationship_type
                )
            })
            .collect();

        let assessment = DependencyAssessment::assemble(
            requested_scope.clone(),
            participating.clone(),
            observed_dependency_set,
        );
        let completeness =
            derive_completeness(surfaces_requested, surfaces_available, &relationships, &gaps);
        let diagnostics = DependencyDiagnostics::assemble(
            completeness.as_str(),
            relationships.len(),
            unavailable.clone(),
            vec![
                "Dependency diagnostics are observational only".into(),
                "dependency does not imply causation".into(),
                "dependency does not imply execution".into(),
            ],
        );
        let lineage =
            DependencyLineage::from_contributors(contributing, revisions.clone(), lineage_evidence);

        let narrative_summary = format!(
            "Evidence dependency: {}/{} sources available, {} node(s), {} relationship(s), {} gap(s), completeness={}",
            surfaces_available,
            surfaces_requested,
            nodes.len(),
            relationships.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Observed recorded evidence dependencies for workspace {workspace_id}. \
             Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer observes recorded dependency structure only — it never invents \
             dependencies, establishes causation, or implies execution."
        );
        let limitations = vec![
            "Workspace Evidence Dependency Engine observes recorded evidence dependencies only".into(),
            "It never invents dependencies or establishes causation".into(),
            "It never becomes the authority for any upstream intelligence layer".into(),
            "Dependency never implies execution, causation, or action".into(),
        ];

        let snap = Self {
            dependency_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceDependencyStatus::Current,
            superseded_at: None,
            scope,
            assessment,
            nodes,
            relationships,
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
        scope: DependencyScope,
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
    ) -> Result<Self, EvidenceDependencyError> {
        Self::compose(
            workspace_id,
            generated_at,
            scope,
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
        self.status = EvidenceDependencyStatus::Superseded;
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
            && self.nodes.iter().all(|n| n.is_non_actionable())
            && self.relationships.iter().all(|r| r.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.diagnostics.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), EvidenceDependencyError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceDependencyError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceDependencyError::MustNotBeActionable);
        }
        if !self.assessment.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.nodes.iter().any(|n| !n.is_non_actionable())
            || self.relationships.iter().any(|r| !r.is_non_actionable())
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceDependencyError::MustNotBeActionable);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for note in &self.diagnostics.notes {
            reject_forbidden_phrases(note)?;
        }
        for rel in &self.relationships {
            reject_forbidden_phrases(&rel.recorded_lineage)?;
            reject_forbidden_phrases(&rel.relationship_type)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceDependencyHistoryEntry {
    pub dependency_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub node_count: usize,
    pub relationship_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl EvidenceDependencyHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceDependencySnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            dependency_id: snap.dependency_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            node_count: snap.nodes.len(),
            relationship_count: snap.relationships.len(),
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
pub struct WorkspaceEvidenceDependencyProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceDependencySnapshot>,
    pub history: Vec<EvidenceDependencyHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceDependencyProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceDependencySnapshot>,
        history: Vec<EvidenceDependencyHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceDependencySummary {
        WorkspaceEvidenceDependencySummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().to_string()),
            node_count: self.current.as_ref().map(|c| c.nodes.len()).unwrap_or(0),
            relationship_count: self
                .current
                .as_ref()
                .map(|c| c.relationships.len())
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
pub struct WorkspaceEvidenceDependencySummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub node_count: usize,
    pub relationship_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceDependencyHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceDependencyExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub node_summaries: Vec<String>,
    pub relationship_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceEvidenceDependencyExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceDependencySnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_dependency_explanation:{}", snap.dependency_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            node_summaries: snap
                .nodes
                .iter()
                .map(|n| format!("{} ({})", n.artefact_ref, n.origin_domain))
                .collect(),
            relationship_summaries: snap
                .relationships
                .iter()
                .map(|r| {
                    format!(
                        "{} -> {} [{}]",
                        r.source_reference, r.target_reference, r.relationship_type
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
                "Dependency explanation is observational only".into(),
                "Missing links remain missing".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct RecordedDep {
    target_domain: String,
    target_ref: String,
    target_revision: Option<String>,
    relationship_type: String,
    recorded_lineage: String,
}

struct SurfaceDeps {
    artefact_ref: String,
    revision: Option<String>,
    recorded_deps: Vec<RecordedDep>,
    broken_refs: Vec<String>,
    missing_sources: Vec<String>,
    incomplete: bool,
}

fn derive_completeness(
    requested: usize,
    available: usize,
    relationships: &[DependencyRelationship],
    gaps: &[DependencyGap],
) -> EvidenceDependencyCompleteness {
    if requested == 0 {
        return EvidenceDependencyCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceDependencyCompleteness::Unavailable;
    }
    if gaps.iter().any(|g| g.gap_kind == DependencyGap::KIND_BROKEN) {
        return EvidenceDependencyCompleteness::Contradictory;
    }
    if available == requested && gaps.is_empty() {
        return EvidenceDependencyCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() || relationships.is_empty() {
        return EvidenceDependencyCompleteness::Partial;
    }
    EvidenceDependencyCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceDependencyError> {
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
        "schedule work",
        "causes ",
        "workflow step",
        "execution order",
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(EvidenceDependencyError::MustNotBeActionable);
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

    fn empty_compose() -> WorkspaceEvidenceDependencySnapshot {
        WorkspaceEvidenceDependencySnapshot::compose(
            "ws",
            "t0",
            DependencyScope::all_surfaces(),
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
    fn unavailable_dependencies_preserved() {
        let snap = empty_compose();
        assert_eq!(
            snap.completeness,
            EvidenceDependencyCompleteness::Unavailable
        );
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == DependencyGap::KIND_UNAVAILABLE));
        assert!(snap.relationships.is_empty());
        assert!(snap.is_non_executing());
    }

    #[test]
    fn identical_inputs_produce_identical_outputs() {
        let left = WorkspaceEvidenceDependencySnapshot::compose_deterministic(
            "ws",
            "t1",
            DependencyScope::all_surfaces(),
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
        let right = WorkspaceEvidenceDependencySnapshot::compose_deterministic(
            "ws",
            "t1",
            DependencyScope::all_surfaces(),
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
        assert_eq!(left.dependency_id, right.dependency_id);
        assert_eq!(left.nodes, right.nodes);
        assert_eq!(left.relationships, right.relationships);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = empty_compose();
        snap.mark_superseded("t1");
        let entry = EvidenceDependencyHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj =
            WorkspaceEvidenceDependencyProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn broken_links_preserved() {
        let gap = DependencyGap::record(
            DependencyGap::KIND_BROKEN,
            "Broken lineage 'x' — never invent or repair dependency",
            vec!["x".into()],
            vec![],
        );
        assert!(gap.is_non_actionable());
        assert_eq!(gap.gap_kind, DependencyGap::KIND_BROKEN);
    }

    #[test]
    fn dependency_never_implies_causation() {
        let snap = empty_compose();
        assert!(snap
            .limitations
            .iter()
            .any(|l| l.contains("causation")));
        assert!(!snap.narrative.to_ascii_lowercase().contains("causes "));
        assert!(!snap
            .diagnostics
            .notes
            .iter()
            .any(|n| n.to_ascii_lowercase().contains("causes ")));
    }

    #[test]
    fn dependency_never_implies_execution() {
        let snap = empty_compose();
        assert!(snap
            .limitations
            .iter()
            .any(|l| l.contains("execution")));
        assert!(!snap.narrative.to_ascii_lowercase().contains("execute now"));
        assert!(!snap.actionable);
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
    fn deterministic_dependency_graph() {
        let snap = empty_compose();
        assert_eq!(snap.assessment.requested_scope.len(), 12);
        assert!(snap.assessment.participating_artefacts.is_empty());
        assert!(snap.assessment.observed_dependency_set.is_empty());
        assert_eq!(snap.diagnostics.reachable_dependency_count, 0);
    }
}
