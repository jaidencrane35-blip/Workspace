//! Workspace Evidence Navigation Engine — Programme IV Batch 2.
//!
//! Navigate evidence. Never interpret evidence.
//! navigation ≠ interpretation; path ≠ inference; traversal ≠ reasoning.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_intelligence_hub::WorkspaceIntelligenceHubProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_semantic_query::WorkspaceSemanticQueryProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvidenceNavigationError {
    #[error("invalid evidence navigation status: {0}")]
    InvalidStatus(String),

    #[error("invalid evidence navigation completeness: {0}")]
    InvalidCompleteness(String),

    #[error("evidence navigation artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("evidence navigation artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("evidence paths require origin and destination references")]
    RequiresReferences,

    #[error("traversal depth must be at least 1")]
    InvalidDepth,

    #[error("evidence navigation snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceNavigationStatus {
    Current,
    Superseded,
    Archived,
}

impl EvidenceNavigationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceNavigationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(EvidenceNavigationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceNavigationCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl EvidenceNavigationCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EvidenceNavigationError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(EvidenceNavigationError::InvalidCompleteness(other.into())),
        }
    }
}

/// Which upstream evidence surfaces participate in traversal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraversalScope {
    pub include_semantic_query: bool,
    pub include_intelligence_hub: bool,
    pub include_knowledge_integration: bool,
    pub include_knowledge_synthesis: bool,
    pub include_contextual: bool,
    pub include_explanation: bool,
    pub include_temporal: bool,
    pub include_reconstruction: bool,
    pub include_state: bool,
}

impl TraversalScope {
    pub fn all_surfaces() -> Self {
        Self {
            include_semantic_query: true,
            include_intelligence_hub: true,
            include_knowledge_integration: true,
            include_knowledge_synthesis: true,
            include_contextual: true,
            include_explanation: true,
            include_temporal: true,
            include_reconstruction: true,
            include_state: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceNavigationEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceNavigationEvidenceRef {
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

/// Non-executable navigation request — no execution semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceNavigationSession {
    pub session_id: String,
    pub navigation_request: String,
    pub entry_references: Vec<String>,
    pub traversal_scope: TraversalScope,
    pub traversal_depth: usize,
    pub authority_effect: String,
    pub actionable: bool,
    pub executable: bool,
}

impl EvidenceNavigationSession {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_nav_session:";

    pub fn request(
        navigation_request: impl Into<String>,
        entry_references: Vec<String>,
        traversal_scope: TraversalScope,
        traversal_depth: usize,
    ) -> Result<Self, EvidenceNavigationError> {
        if traversal_depth < 1 {
            return Err(EvidenceNavigationError::InvalidDepth);
        }
        let navigation_request = navigation_request.into();
        let digest = stable_digest(&format!(
            "{navigation_request}|{}|{traversal_depth}",
            entry_references.join(",")
        ));
        Ok(Self {
            session_id: format!("{}{}", Self::ID_PREFIX, digest),
            navigation_request,
            entry_references,
            traversal_scope,
            traversal_depth,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            executable: false,
        })
    }

    pub fn is_non_executable(&self) -> bool {
        !self.actionable
            && !self.executable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.traversal_depth >= 1
    }
}

/// Existing provenance path only — never inferred edges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidencePath {
    pub path_id: String,
    pub origin: String,
    pub destination: String,
    pub intermediate_references: Vec<String>,
    pub evidence_lineage: Vec<EvidenceNavigationEvidenceRef>,
    pub hop_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidencePath {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_path:";

    pub fn from_existing(
        origin: impl Into<String>,
        destination: impl Into<String>,
        intermediate_references: Vec<String>,
        evidence_lineage: Vec<EvidenceNavigationEvidenceRef>,
    ) -> Result<Self, EvidenceNavigationError> {
        let origin = origin.into();
        let destination = destination.into();
        if origin.is_empty() || destination.is_empty() {
            return Err(EvidenceNavigationError::RequiresReferences);
        }
        if evidence_lineage.is_empty() {
            return Err(EvidenceNavigationError::RequiresReferences);
        }
        if evidence_lineage.iter().any(|r| !r.is_non_actionable()) {
            return Err(EvidenceNavigationError::MustNotBeActionable);
        }
        let hop_count = intermediate_references.len() + 1;
        let digest = stable_digest(&format!(
            "{origin}|{destination}|{}",
            intermediate_references.join(",")
        ));
        Ok(Self {
            path_id: format!("{}{}", Self::ID_PREFIX, digest),
            origin,
            destination,
            intermediate_references,
            evidence_lineage,
            hop_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_lineage.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic rollup — reachable / unreachable / completeness only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceNavigationDiagSummary {
    pub summary_id: String,
    pub reachable_evidence: Vec<String>,
    pub unreachable_evidence: Vec<String>,
    pub traversal_completeness: String,
    pub path_count: usize,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceNavigationDiagSummary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "navigation_summary:";

    pub fn assemble(
        reachable_evidence: Vec<String>,
        unreachable_evidence: Vec<String>,
        traversal_completeness: impl Into<String>,
        path_count: usize,
        notes: Vec<String>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "reach={}|unreach={}|paths={path_count}",
            reachable_evidence.len(),
            unreachable_evidence.len()
        ));
        Self {
            summary_id: format!("{}{}", Self::ID_PREFIX, digest),
            reachable_evidence,
            unreachable_evidence,
            traversal_completeness: traversal_completeness.into(),
            path_count,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Broken lineage / unavailable / incomplete — never repaired automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_references: Vec<String>,
    pub evidence_refs: Vec<EvidenceNavigationEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl NavigationGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "navigation_gap:";
    pub const KIND_BROKEN_LINEAGE: &'static str = "broken_lineage";
    pub const KIND_UNAVAILABLE: &'static str = "unavailable_evidence";
    pub const KIND_INCOMPLETE: &'static str = "incomplete_traversal";

    pub fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_references: Vec<String>,
        evidence_refs: Vec<EvidenceNavigationEvidenceRef>,
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

/// Records every upstream snapshot participating in traversal — no inferred provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationLineage {
    pub lineage_id: String,
    pub participating_snapshots: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<EvidenceNavigationEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl NavigationLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "navigation_lineage:";

    pub fn from_participants(
        participating_snapshots: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<EvidenceNavigationEvidenceRef>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "{}|{}",
            participating_snapshots.join(","),
            revisions.join(",")
        ));
        Self {
            lineage_id: format!("{}{}", Self::ID_PREFIX, digest),
            participating_snapshots,
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

/// Durable evidence navigation artefact — observational paths only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceNavigationSnapshot {
    pub navigation_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: EvidenceNavigationStatus,
    pub superseded_at: Option<String>,
    pub session: EvidenceNavigationSession,
    pub paths: Vec<EvidencePath>,
    pub summary: EvidenceNavigationDiagSummary,
    pub gaps: Vec<NavigationGap>,
    pub lineage: NavigationLineage,
    pub completeness: EvidenceNavigationCompleteness,
    pub provenance_links: Vec<EvidenceNavigationEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceEvidenceNavigationSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_navigation:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        session: EvidenceNavigationSession,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        contextual: Option<&ContextualUnderstandingProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, EvidenceNavigationError> {
        if !session.is_non_executable() {
            return Err(EvidenceNavigationError::MustNotBeActionable);
        }
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let mut graph = EvidenceGraph::default();
        let mut gaps = Vec::new();
        let mut participating = Vec::new();
        let mut revisions = Vec::new();
        let mut lineage_evidence = Vec::new();
        let mut provenance_links = Vec::new();
        let mut surfaces_requested = 0usize;
        let mut surfaces_available = 0usize;

        macro_rules! take_surface {
            ($flag:expr, $name:expr, $opt:expr, $extract:expr) => {
                if $flag {
                    surfaces_requested += 1;
                    match $opt {
                        Some(proj) => {
                            if let Some(node) = $extract(proj) {
                                surfaces_available += 1;
                                participating.push(format!("{}:{}", $name, node.artefact_ref));
                                if let Some(rev) = &node.revision {
                                    revisions.push(rev.clone());
                                }
                                let link = EvidenceNavigationEvidenceRef::link(
                                    $name,
                                    node.artefact_ref.clone(),
                                    node.revision.clone(),
                                );
                                lineage_evidence.push(link.clone());
                                provenance_links.push(link);
                                graph.ingest_surface($name, &node);
                            } else {
                                gaps.push(NavigationGap::record(
                                    NavigationGap::KIND_UNAVAILABLE,
                                    format!(
                                        "Upstream surface '{}' has no current artefact — never invent paths",
                                        $name
                                    ),
                                    vec![$name.into()],
                                    vec![],
                                ));
                            }
                        }
                        None => {
                            gaps.push(NavigationGap::record(
                                NavigationGap::KIND_UNAVAILABLE,
                                format!(
                                    "Upstream surface '{}' unavailable — never invent paths or continuity",
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

        take_surface!(
            session.traversal_scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.query_id.clone(),
                    revision: Some(c.query_id.clone()),
                    edges: c
                        .result
                        .matches
                        .iter()
                        .map(|m| {
                            (
                                c.query_id.clone(),
                                m.evidence_ref.external_ref.clone(),
                                EvidenceNavigationEvidenceRef::link(
                                    m.evidence_ref.origin_domain.clone(),
                                    m.evidence_ref.external_ref.clone(),
                                    m.evidence_ref.source_revision.clone(),
                                ),
                            )
                        })
                        .chain(c.provenance_links.iter().map(|r| {
                            (
                                c.query_id.clone(),
                                r.external_ref.clone(),
                                EvidenceNavigationEvidenceRef::link(
                                    r.origin_domain.clone(),
                                    r.external_ref.clone(),
                                    r.source_revision.clone(),
                                ),
                            )
                        }))
                        .collect(),
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.hub_id.clone(),
                    revision: Some(c.hub_id.clone()),
                    edges: c
                        .packages
                        .iter()
                        .flat_map(|pkg| {
                            pkg.evidence_refs.iter().map(|r| {
                                (
                                    pkg.artefact_ref.clone(),
                                    r.external_ref.clone(),
                                    EvidenceNavigationEvidenceRef::link(
                                        r.origin_domain.clone(),
                                        r.external_ref.clone(),
                                        r.source_revision.clone(),
                                    ),
                                )
                            })
                        })
                        .chain(c.provenance_links.iter().map(|r| {
                            (
                                c.hub_id.clone(),
                                r.external_ref.clone(),
                                EvidenceNavigationEvidenceRef::link(
                                    r.origin_domain.clone(),
                                    r.external_ref.clone(),
                                    r.source_revision.clone(),
                                ),
                            )
                        }))
                        .collect(),
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.integration_id.clone(),
                    revision: Some(c.integration_id.clone()),
                    edges: c
                        .provenance_links
                        .iter()
                        .map(|r| {
                            (
                                c.integration_id.clone(),
                                r.external_ref.clone(),
                                EvidenceNavigationEvidenceRef::link(
                                    r.origin_domain.clone(),
                                    r.external_ref.clone(),
                                    r.source_revision.clone(),
                                ),
                            )
                        })
                        .collect(),
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_knowledge_synthesis,
            "knowledge_synthesis",
            knowledge_synthesis,
            |p: &KnowledgeSynthesisProjection| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.synthesis_id.clone(),
                    revision: Some(c.synthesis_id.clone()),
                    edges: c
                        .provenance_links
                        .iter()
                        .map(|r| {
                            (
                                c.synthesis_id.clone(),
                                r.external_ref.clone(),
                                EvidenceNavigationEvidenceRef::link(
                                    r.origin_domain.clone(),
                                    r.external_ref.clone(),
                                    r.source_revision.clone(),
                                ),
                            )
                        })
                        .collect(),
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.understanding_id.clone(),
                    revision: Some(c.understanding_id.clone()),
                    edges: vec![(
                        c.understanding_id.clone(),
                        c.understanding_id.clone(),
                        EvidenceNavigationEvidenceRef::link(
                            "contextual_understanding",
                            c.understanding_id.clone(),
                            Some(c.understanding_id.clone()),
                        ),
                    )],
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.explanation_id.clone(),
                    revision: Some(c.explanation_id.clone()),
                    edges: vec![(
                        c.explanation_id.clone(),
                        c.explanation_id.clone(),
                        EvidenceNavigationEvidenceRef::link(
                            "workspace_explanation",
                            c.explanation_id.clone(),
                            Some(c.explanation_id.clone()),
                        ),
                    )],
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_temporal,
            "temporal_intelligence",
            temporal,
            |p: &TemporalIntelligenceSnapshot| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.analysis_id.clone(),
                    revision: Some(c.analysis_id.clone()),
                    edges: vec![(
                        c.analysis_id.clone(),
                        c.analysis_id.clone(),
                        EvidenceNavigationEvidenceRef::link(
                            "temporal_intelligence",
                            c.analysis_id.clone(),
                            Some(c.analysis_id.clone()),
                        ),
                    )],
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_reconstruction,
            "historical_reconstruction",
            reconstruction,
            |p: &HistoricalReconstructionSnapshot| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.reconstruction_id.clone(),
                    revision: c
                        .to_revision
                        .clone()
                        .or_else(|| Some(c.reconstruction_id.clone())),
                    edges: vec![(
                        c.reconstruction_id.clone(),
                        c.reconstruction_id.clone(),
                        EvidenceNavigationEvidenceRef::link(
                            "historical_reconstruction",
                            c.reconstruction_id.clone(),
                            c.to_revision.clone(),
                        ),
                    )],
                })
            }
        );
        take_surface!(
            session.traversal_scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceNode {
                    artefact_ref: c.state_id.clone(),
                    revision: Some(c.revision.clone()),
                    edges: vec![(
                        c.state_id.clone(),
                        c.state_id.clone(),
                        EvidenceNavigationEvidenceRef::link(
                            "workspace_state_envelope",
                            c.state_id.clone(),
                            Some(c.revision.clone()),
                        ),
                    )],
                })
            }
        );

        // Resolve entry references — missing entries become broken_lineage gaps (never bridged).
        let entries = if session.entry_references.is_empty() {
            graph.default_entries()
        } else {
            let mut resolved = Vec::new();
            for entry in &session.entry_references {
                if graph.has_node(entry) {
                    resolved.push(entry.clone());
                } else {
                    gaps.push(NavigationGap::record(
                        NavigationGap::KIND_BROKEN_LINEAGE,
                        format!(
                            "Entry reference '{entry}' not found in existing evidence graph — never bridge missing lineage"
                        ),
                        vec![entry.clone()],
                        vec![],
                    ));
                }
            }
            resolved
        };

        let (paths, truncated) = graph.traverse(&entries, session.traversal_depth)?;
        if truncated {
            gaps.push(NavigationGap::record(
                NavigationGap::KIND_INCOMPLETE,
                String::from(
                    "Traversal stopped at configured depth — incomplete traversal preserved, never fabricated continuity"
                ),
                entries.clone(),
                vec![],
            ));
        }

        let mut reachable: Vec<String> = paths
            .iter()
            .flat_map(|p| {
                std::iter::once(p.origin.clone())
                    .chain(p.intermediate_references.clone())
                    .chain(std::iter::once(p.destination.clone()))
            })
            .collect();
        reachable.sort();
        reachable.dedup();

        let mut unreachable = Vec::new();
        for entry in &session.entry_references {
            if !reachable.contains(entry) && !graph.has_node(entry) {
                unreachable.push(entry.clone());
            }
        }
        unreachable.sort();
        unreachable.dedup();

        let completeness =
            derive_completeness(surfaces_requested, surfaces_available, &gaps, paths.is_empty());
        let summary = EvidenceNavigationDiagSummary::assemble(
            reachable.clone(),
            unreachable,
            completeness.as_str(),
            paths.len(),
            vec![
                "Navigation summary is diagnostic only".into(),
                "path ≠ inference".into(),
                "traversal ≠ reasoning".into(),
            ],
        );
        let lineage =
            NavigationLineage::from_participants(participating, revisions.clone(), lineage_evidence);

        let narrative_summary = format!(
            "Evidence navigation: {} path(s), {} gap(s), completeness={}",
            paths.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Exposed existing evidence relationships for workspace {workspace_id} \
             (request '{}'). Available surfaces={surfaces_available}/{surfaces_requested}. \
             This layer navigates existing provenance only — it never interprets evidence \
             or invents connections.",
            session.navigation_request
        );
        let limitations = vec![
            "Workspace Evidence Navigation Engine exposes existing evidence relationships only".into(),
            "It never interprets evidence or invents connections".into(),
            "It never becomes the authority for any upstream intelligence layer".into(),
            "Broken chains remain broken — never fabricate traversal continuity".into(),
        ];

        let mut snap = Self {
            navigation_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}|{}",
                    session.session_id,
                    revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: EvidenceNavigationStatus::Current,
            superseded_at: None,
            session,
            paths,
            summary,
            gaps,
            lineage,
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
        session: EvidenceNavigationSession,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        contextual: Option<&ContextualUnderstandingProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, EvidenceNavigationError> {
        Self::compose(
            workspace_id,
            generated_at,
            session,
            semantic_query,
            intelligence_hub,
            knowledge_integration,
            knowledge_synthesis,
            contextual,
            explanation,
            temporal,
            reconstruction,
            state,
        )
    }

    pub fn mark_superseded(&mut self, superseded_at: impl Into<String>) {
        self.status = EvidenceNavigationStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.session.is_non_executable()
            && self.paths.iter().all(|p| p.is_non_actionable())
            && self.summary.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), EvidenceNavigationError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(EvidenceNavigationError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(EvidenceNavigationError::MustNotBeActionable);
        }
        if !self.session.is_non_executable() {
            return Err(EvidenceNavigationError::MustNotBeActionable);
        }
        for path in &self.paths {
            if !path.is_non_actionable() || path.evidence_lineage.is_empty() {
                return Err(EvidenceNavigationError::MustNotBeActionable);
            }
        }
        if !self.summary.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(EvidenceNavigationError::MustNotBeActionable);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for note in &self.summary.notes {
            reject_forbidden_phrases(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceNavigationHistoryEntry {
    pub navigation_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub path_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl EvidenceNavigationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceNavigationSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            navigation_id: snap.navigation_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            path_count: snap.paths.len(),
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
pub struct WorkspaceEvidenceNavigationProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceEvidenceNavigationSnapshot>,
    pub history: Vec<EvidenceNavigationHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceEvidenceNavigationProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceEvidenceNavigationSnapshot>,
        history: Vec<EvidenceNavigationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceEvidenceNavigationSummary {
        WorkspaceEvidenceNavigationSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().to_string()),
            path_count: self.current.as_ref().map(|c| c.paths.len()).unwrap_or(0),
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
pub struct WorkspaceEvidenceNavigationSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub path_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<EvidenceNavigationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvidenceNavigationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub path_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceEvidenceNavigationExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceEvidenceNavigationSnapshot) -> Self {
        Self {
            explanation_id: format!("evidence_navigation_explanation:{}", snap.navigation_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            path_summaries: snap
                .paths
                .iter()
                .map(|p| format!("{} -> {} (hops={})", p.origin, p.destination, p.hop_count))
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{} ({})", g.description, g.gap_kind))
                .collect(),
            lineage_summaries: snap.lineage.participating_snapshots.clone(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: vec![
                "Navigation explanation is observational only".into(),
                "Unknown remains unknown".into(),
            ],
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceNode {
    artefact_ref: String,
    revision: Option<String>,
    /// Existing edges only: (from, to, evidence_ref)
    edges: Vec<(String, String, EvidenceNavigationEvidenceRef)>,
}

#[derive(Default)]
struct EvidenceGraph {
    nodes: Vec<String>,
    /// adjacency: from -> Vec<(to, evidence_ref)>
    edges: Vec<(String, String, EvidenceNavigationEvidenceRef)>,
}

impl EvidenceGraph {
    fn ingest_surface(&mut self, _surface: &str, node: &SurfaceNode) {
        self.add_node(&node.artefact_ref);
        for (from, to, link) in &node.edges {
            self.add_node(from);
            self.add_node(to);
            // Skip self-loops as path edges; they only register presence.
            if from != to {
                self.edges.push((from.clone(), to.clone(), link.clone()));
            }
        }
        // Sort for determinism
        self.nodes.sort();
        self.nodes.dedup();
        self.edges.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        self.edges.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    }

    fn add_node(&mut self, id: &str) {
        if !self.nodes.iter().any(|n| n == id) {
            self.nodes.push(id.to_string());
        }
    }

    fn has_node(&self, id: &str) -> bool {
        self.nodes.iter().any(|n| n == id)
    }

    fn default_entries(&self) -> Vec<String> {
        // Deterministic: all nodes that are edge origins, else all nodes.
        let mut origins: Vec<String> = self.edges.iter().map(|e| e.0.clone()).collect();
        origins.sort();
        origins.dedup();
        if origins.is_empty() {
            let mut nodes = self.nodes.clone();
            nodes.sort();
            nodes
        } else {
            origins
        }
    }

    fn traverse(
        &self,
        entries: &[String],
        max_depth: usize,
    ) -> Result<(Vec<EvidencePath>, bool), EvidenceNavigationError> {
        let mut paths = Vec::new();
        let mut truncated = false;
        for entry in entries {
            let mut stack = vec![(entry.clone(), vec![entry.clone()], vec![])];
            while let Some((current, chain, lineage)) = stack.pop() {
                let _depth = chain.len().saturating_sub(1);
                let mut expanded = false;
                for (from, to, link) in &self.edges {
                    if from != &current {
                        continue;
                    }
                    if chain.contains(to) {
                        continue; // no cycles — preserve existing acyclic paths only
                    }
                    expanded = true;
                    let mut next_chain = chain.clone();
                    next_chain.push(to.clone());
                    let mut next_lineage = lineage.clone();
                    next_lineage.push(link.clone());
                    let next_depth = next_chain.len().saturating_sub(1);
                    if next_depth >= max_depth {
                        // Emit path at depth limit
                        let origin = next_chain.first().cloned().unwrap_or_default();
                        let destination = next_chain.last().cloned().unwrap_or_default();
                        let intermediates = if next_chain.len() > 2 {
                            next_chain[1..next_chain.len() - 1].to_vec()
                        } else {
                            vec![]
                        };
                        paths.push(EvidencePath::from_existing(
                            origin,
                            destination,
                            intermediates,
                            next_lineage,
                        )?);
                        // Check if further edges exist beyond depth
                        if self.edges.iter().any(|(f, t, _)| f == to && !next_chain.contains(t)) {
                            truncated = true;
                        }
                    } else {
                        stack.push((to.clone(), next_chain, next_lineage));
                    }
                }
                // If no expansion and chain has at least one hop, emit path
                if !expanded && chain.len() > 1 {
                    let origin = chain.first().cloned().unwrap_or_default();
                    let destination = chain.last().cloned().unwrap_or_default();
                    let intermediates = if chain.len() > 2 {
                        chain[1..chain.len() - 1].to_vec()
                    } else {
                        vec![]
                    };
                    paths.push(EvidencePath::from_existing(
                        origin,
                        destination,
                        intermediates,
                        lineage,
                    )?);
                }
            }
        }
        paths.sort_by(|a, b| a.path_id.cmp(&b.path_id));
        paths.dedup_by(|a, b| a.path_id == b.path_id);
        Ok((paths, truncated))
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[NavigationGap],
    paths_empty: bool,
) -> EvidenceNavigationCompleteness {
    if requested == 0 {
        return EvidenceNavigationCompleteness::Unknown;
    }
    if available == 0 {
        return EvidenceNavigationCompleteness::Unavailable;
    }
    if available == requested && gaps.is_empty() && !paths_empty {
        return EvidenceNavigationCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() || paths_empty {
        return EvidenceNavigationCompleteness::Partial;
    }
    EvidenceNavigationCompleteness::Unknown
}

fn reject_forbidden_phrases(text: &str) -> Result<(), EvidenceNavigationError> {
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
        "means that",
        "therefore decide",
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(EvidenceNavigationError::MustNotBeActionable);
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

    fn empty_session() -> EvidenceNavigationSession {
        EvidenceNavigationSession::request(
            "navigate evidence",
            vec![],
            TraversalScope::all_surfaces(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn unavailable_upstreams_produce_gaps_not_paths() {
        let snap = WorkspaceEvidenceNavigationSnapshot::compose(
            "ws",
            "t0",
            empty_session(),
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
            EvidenceNavigationCompleteness::Unavailable
        );
        assert_eq!(snap.gaps.len(), 9);
        assert!(snap.paths.is_empty());
        assert!(snap.is_non_executing());
        assert!(snap.lineage.participating_snapshots.is_empty());
    }

    #[test]
    fn identical_snapshots_produce_identical_paths() {
        let left = WorkspaceEvidenceNavigationSnapshot::compose_deterministic(
            "ws",
            "t1",
            empty_session(),
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
        let right = WorkspaceEvidenceNavigationSnapshot::compose_deterministic(
            "ws",
            "t1",
            empty_session(),
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
        assert_eq!(left.navigation_id, right.navigation_id);
        assert_eq!(left.paths, right.paths);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.lineage, right.lineage);
    }

        #[test]
    fn history_non_actionable() {
        let mut snap = WorkspaceEvidenceNavigationSnapshot::compose(
            "ws",
            "t0",
            empty_session(),
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
        let entry = EvidenceNavigationHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj =
            WorkspaceEvidenceNavigationProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn missing_lineage_preserved_as_broken_gap() {
        let snap = WorkspaceEvidenceNavigationSnapshot::compose(
            "ws",
            "t0",
            EvidenceNavigationSession::request(
                "navigate",
                vec!["missing_ref_xyz".into()],
                TraversalScope {
                    include_semantic_query: false,
                    include_intelligence_hub: false,
                    include_knowledge_integration: false,
                    include_knowledge_synthesis: false,
                    include_contextual: false,
                    include_explanation: false,
                    include_temporal: false,
                    include_reconstruction: false,
                    include_state: false,
                },
                2,
            )
            .unwrap(),
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
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == NavigationGap::KIND_BROKEN_LINEAGE));
        assert!(snap.paths.is_empty());
    }

    #[test]
    fn lineage_preserved_from_participating_snapshots_only() {
        let state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let snap = WorkspaceEvidenceNavigationSnapshot::compose(
            "ws",
            "t0",
            EvidenceNavigationSession::request(
                "state only",
                vec![],
                TraversalScope {
                    include_semantic_query: false,
                    include_intelligence_hub: false,
                    include_knowledge_integration: false,
                    include_knowledge_synthesis: false,
                    include_contextual: false,
                    include_explanation: false,
                    include_temporal: false,
                    include_reconstruction: false,
                    include_state: true,
                },
                2,
            )
            .unwrap(),
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
        // empty current → unavailable gap; no invented participants
        assert!(snap.lineage.participating_snapshots.is_empty());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == NavigationGap::KIND_UNAVAILABLE));
    }

    #[test]
    fn session_must_be_non_executable() {
        let mut session = empty_session();
        session.executable = true;
        let err = WorkspaceEvidenceNavigationSnapshot::compose(
            "ws",
            "t0",
            session,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(err.is_err());
    }

    #[test]
    fn invalid_depth_rejected() {
        let err = EvidenceNavigationSession::request(
            "x",
            vec![],
            TraversalScope::all_surfaces(),
            0,
        );
        assert_eq!(err, Err(EvidenceNavigationError::InvalidDepth));
    }
}
