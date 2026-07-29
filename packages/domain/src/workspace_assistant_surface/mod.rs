//! Workspace Assistant Surface - Programme IV Batch 11.
//!
//! Present intelligence. Never become authority.
//! This is a conversational composition layer over recorded projections only.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_evidence_completeness::WorkspaceEvidenceCompletenessProjection;
use crate::workspace_evidence_consistency::WorkspaceEvidenceConsistencyProjection;
use crate::workspace_evidence_contract::{
    reject_forbidden_phrases_with, stable_digest, AUTHORITY_EFFECT_NONE,
};
use crate::workspace_evidence_coverage::WorkspaceEvidenceCoverageProjection;
use crate::workspace_evidence_dependency::WorkspaceEvidenceDependencyProjection;
use crate::workspace_evidence_freshness::WorkspaceEvidenceFreshnessProjection;
use crate::workspace_evidence_navigation::WorkspaceEvidenceNavigationProjection;
use crate::workspace_evidence_reliability::WorkspaceEvidenceReliabilityProjection;
use crate::workspace_evidence_trace::WorkspaceEvidenceTraceProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_intelligence_hub::WorkspaceIntelligenceHubProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_semantic_query::WorkspaceSemanticQueryProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;

const FORBIDDEN_ASSISTANT_PHRASES: &[&str] = &[
    "approve",
    "execute now",
    "i will run",
    "trust me",
    "decision is",
    "you should",
    "let me handle",
];

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantSurfaceError {
    #[error("invalid assistant surface status: {0}")]
    InvalidStatus(String),

    #[error("assistant surface artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant surface artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant surface text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant surface snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantSurfaceStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantSurfaceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantSurfaceError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantSurfaceError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceScope {
    pub include_semantic_query: bool,
    pub include_evidence_navigation: bool,
    pub include_evidence_trace: bool,
    pub include_evidence_coverage: bool,
    pub include_evidence_consistency: bool,
    pub include_evidence_dependency: bool,
    pub include_evidence_freshness: bool,
    pub include_evidence_completeness: bool,
    pub include_evidence_reliability: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub include_knowledge_integration: bool,
    pub include_intelligence_hub: bool,
    pub include_state: bool,
}

impl AssistantSurfaceScope {
    pub fn all_surfaces() -> Self {
        Self::presentation_default()
    }

    pub fn presentation_default() -> Self {
        Self {
            include_semantic_query: true,
            include_evidence_navigation: true,
            include_evidence_trace: true,
            include_evidence_coverage: true,
            include_evidence_consistency: true,
            include_evidence_dependency: true,
            include_evidence_freshness: true,
            include_evidence_completeness: true,
            include_evidence_reliability: true,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_integration: true,
            include_intelligence_hub: true,
            include_state: true,
        }
    }

    /// Retrieval-focused default for Batch 13 — enables Semantic Query plus
    /// Evidence Navigation / Trace / Coverage / Consistency only.
    pub fn retrieval_default() -> Self {
        Self {
            include_semantic_query: true,
            include_evidence_navigation: true,
            include_evidence_trace: true,
            include_evidence_coverage: true,
            include_evidence_consistency: true,
            include_evidence_dependency: false,
            include_evidence_freshness: false,
            include_evidence_completeness: false,
            include_evidence_reliability: false,
            include_explanation: false,
            include_contextual: false,
            include_knowledge_integration: false,
            include_intelligence_hub: false,
            include_state: false,
        }
    }

    /// Explanation-focused default for Batch 14 — enables Explanation Layer,
    /// Evidence Trace / Navigation, Consistency (conflict visibility), and
    /// Semantic Query as a retrieval-related input pathway.
    pub fn explanation_default() -> Self {
        Self {
            include_semantic_query: true,
            include_evidence_navigation: true,
            include_evidence_trace: true,
            include_evidence_coverage: false,
            include_evidence_consistency: true,
            include_evidence_dependency: false,
            include_evidence_freshness: false,
            include_evidence_completeness: false,
            include_evidence_reliability: false,
            include_explanation: true,
            include_contextual: false,
            include_knowledge_integration: false,
            include_intelligence_hub: false,
            include_state: false,
        }
    }

    /// Interaction-focused default for Batch 15 — enables surface-relevant
    /// pathways used when coordinating Batches 11–14 (explanation, semantic
    /// query, evidence trace/navigation/consistency, contextual, state).
    /// Additive like `retrieval_default()` / `explanation_default()`.
    pub fn interaction_default() -> Self {
        Self {
            include_semantic_query: true,
            include_evidence_navigation: true,
            include_evidence_trace: true,
            include_evidence_coverage: false,
            include_evidence_consistency: true,
            include_evidence_dependency: false,
            include_evidence_freshness: false,
            include_evidence_completeness: false,
            include_evidence_reliability: false,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_integration: false,
            include_intelligence_hub: false,
            include_state: true,
        }
    }

    /// Personalisation-focused default for Batch 16 — enables presentation-
    /// relevant flags similar to `interaction_default()` / `presentation_default()`
    /// (explanation, semantic query, evidence navigation/trace/consistency,
    /// contextual, state). Additive; does not invent identity pathways.
    pub fn personalisation_default() -> Self {
        Self {
            include_semantic_query: true,
            include_evidence_navigation: true,
            include_evidence_trace: true,
            include_evidence_coverage: false,
            include_evidence_consistency: true,
            include_evidence_dependency: false,
            include_evidence_freshness: false,
            include_evidence_completeness: false,
            include_evidence_reliability: false,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_integration: false,
            include_intelligence_hub: false,
            include_state: true,
        }
    }

    /// American spelling alias for [`Self::personalisation_default`].
    pub fn personalization_default() -> Self {
        Self::personalisation_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantCitation {
    pub origin_domain: String,
    pub artefact_ref: String,
    pub source_revision: Option<String>,
    pub excerpt: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantCitation {
    pub fn present(
        origin_domain: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        excerpt: impl Into<String>,
    ) -> Self {
        Self {
            origin_domain: origin_domain.into(),
            artefact_ref: artefact_ref.into(),
            source_revision,
            excerpt: excerpt.into(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantUtterance {
    pub utterance_id: String,
    pub role: String,
    pub body: String,
    pub citations: Vec<AssistantCitation>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantUtterance {
    fn compose(body: String, citations: Vec<AssistantCitation>) -> Self {
        let digest = stable_digest(&format!(
            "{}|{}",
            body,
            citations
                .iter()
                .map(|c| format!("{}:{}", c.origin_domain, c.artefact_ref))
                .collect::<Vec<_>>()
                .join(",")
        ));
        Self {
            utterance_id: format!("assistant_utterance:{digest}"),
            role: "assistant".into(),
            body,
            citations,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        self.role == "assistant"
            && !self.actionable
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.citations.iter().all(|c| c.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub citations: Vec<AssistantCitation>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl PresentationLineage {
    fn from_citations(citations: Vec<AssistantCitation>) -> Self {
        let contributing_artefacts = citations
            .iter()
            .map(|c| format!("{}:{}", c.origin_domain, c.artefact_ref))
            .collect::<Vec<_>>();
        let revisions = citations
            .iter()
            .filter_map(|c| c.source_revision.clone())
            .collect::<Vec<_>>();
        let lineage_id = format!(
            "assistant_lineage:{}",
            stable_digest(&format!(
                "{}|{}",
                contributing_artefacts.join(","),
                revisions.join(",")
            ))
        );
        Self {
            lineage_id,
            contributing_artefacts,
            revisions,
            citations,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.citations.iter().all(|c| c.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_surfaces: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantSurfaceGap {
    pub const KIND_UNAVAILABLE_UPSTREAM: &'static str = "unavailable_upstream";
    pub const KIND_MISSING_CITATION: &'static str = "missing_citation";
    pub const KIND_EMPTY_ASK: &'static str = "empty_ask";
    pub const KIND_INCOMPLETE_COMPOSITION: &'static str = "incomplete_composition";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_surfaces: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_gap:{}",
                stable_digest(&format!(
                    "{}|{}|{}",
                    gap_kind,
                    description,
                    affected_surfaces.join(",")
                ))
            ),
            gap_kind,
            description,
            affected_surfaces,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceDiagnostics {
    pub consulted_surfaces: Vec<String>,
    pub unavailable_surfaces: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantSurfaceDiagnostics {
    fn assemble(consulted_surfaces: Vec<String>, unavailable_surfaces: Vec<String>) -> Self {
        Self {
            consulted_surfaces,
            unavailable_surfaces,
            notes: vec![
                "Assistant surface presents recorded evidence only".into(),
                "No permission, mutation, or lifecycle authority is created here".into(),
            ],
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantSurfaceSnapshot {
    pub surface_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantSurfaceStatus,
    pub superseded_at: Option<String>,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
    pub utterance: AssistantUtterance,
    pub lineage: PresentationLineage,
    pub gaps: Vec<AssistantSurfaceGap>,
    pub diagnostics: AssistantSurfaceDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantSurfaceSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_surface:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        evidence_navigation: Option<&WorkspaceEvidenceNavigationProjection>,
        evidence_trace: Option<&WorkspaceEvidenceTraceProjection>,
        evidence_coverage: Option<&WorkspaceEvidenceCoverageProjection>,
        evidence_consistency: Option<&WorkspaceEvidenceConsistencyProjection>,
        evidence_dependency: Option<&WorkspaceEvidenceDependencyProjection>,
        evidence_freshness: Option<&WorkspaceEvidenceFreshnessProjection>,
        evidence_completeness: Option<&WorkspaceEvidenceCompletenessProjection>,
        evidence_reliability: Option<&WorkspaceEvidenceReliabilityProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, AssistantSurfaceError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let human_ask = human_ask.into();

        let mut citations = Vec::new();
        let mut gaps = Vec::new();
        let mut consulted = Vec::new();
        let mut unavailable = Vec::new();

        macro_rules! include_surface {
            ($flag:expr, $name:expr, $projection:expr, $extract:expr) => {
                if $flag {
                    consulted.push($name.to_string());
                    match $projection {
                        Some(projection) => {
                            if let Some(signal) = $extract(projection) {
                                citations.push(AssistantCitation::present(
                                    $name,
                                    signal.artefact_ref,
                                    signal.source_revision,
                                    signal.excerpt,
                                ));
                            } else {
                                unavailable.push($name.to_string());
                                gaps.push(unavailable_gap($name));
                            }
                        }
                        None => {
                            unavailable.push($name.to_string());
                            gaps.push(unavailable_gap($name));
                        }
                    }
                }
            };
        }

        include_surface!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.query_id.clone(),
                    source_revision: Some(c.query_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.navigation_id.clone(),
                    source_revision: Some(c.navigation_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.trace_id.clone(),
                    source_revision: Some(c.trace_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.coverage_id.clone(),
                    source_revision: Some(c.coverage_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.consistency_id.clone(),
                    source_revision: Some(c.consistency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            evidence_dependency,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.dependency_id.clone(),
                    source_revision: Some(c.dependency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_freshness,
            "workspace_evidence_freshness",
            evidence_freshness,
            |p: &WorkspaceEvidenceFreshnessProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.freshness_id.clone(),
                    source_revision: Some(c.freshness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_completeness,
            "workspace_evidence_completeness",
            evidence_completeness,
            |p: &WorkspaceEvidenceCompletenessProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.completeness_id.clone(),
                    source_revision: Some(c.completeness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_evidence_reliability,
            "workspace_evidence_reliability",
            evidence_reliability,
            |p: &WorkspaceEvidenceReliabilityProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.reliability_id.clone(),
                    source_revision: Some(c.reliability_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.explanation_id.clone(),
                    source_revision: Some(c.explanation_id.clone()),
                    excerpt: c.narrative.clone(),
                })
            }
        );
        include_surface!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.understanding_id.clone(),
                    source_revision: Some(c.understanding_id.clone()),
                    excerpt: c.situation_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.integration_id.clone(),
                    source_revision: Some(c.integration_id.clone()),
                    excerpt: c.summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.hub_id.clone(),
                    source_revision: Some(c.hub_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_surface!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| SurfaceSignal {
                    artefact_ref: c.state_id.clone(),
                    source_revision: Some(c.revision.clone()),
                    excerpt: format!(
                        "Workspace state revision {} with {} source(s), {} unknown(s), {} conflict(s)",
                        c.revision,
                        c.sources.len(),
                        c.unknowns.len(),
                        c.contradictions.len()
                    ),
                })
            }
        );

        if human_ask.trim().is_empty() {
            gaps.push(AssistantSurfaceGap::record(
                AssistantSurfaceGap::KIND_EMPTY_ASK,
                "The human ask is empty; the surface can only present available recorded evidence",
                vec!["human_ask".into()],
            ));
        }
        if citations.is_empty() && !consulted.is_empty() {
            gaps.push(AssistantSurfaceGap::record(
                AssistantSurfaceGap::KIND_INCOMPLETE_COMPOSITION,
                "No cited upstream artefact is available for this assistant turn",
                consulted.clone(),
            ));
        }

        let body = assistant_body(&citations, &unavailable);
        let utterance = AssistantUtterance::compose(body, citations.clone());
        let lineage = PresentationLineage::from_citations(citations);
        let diagnostics = AssistantSurfaceDiagnostics::assemble(consulted.clone(), unavailable);
        let narrative_summary = format!(
            "Assistant surface: {} cited surface(s), {} unavailable surface(s), {} gap(s)",
            lineage.citations.len(),
            diagnostics.unavailable_surfaces.len(),
            gaps.len()
        );
        let narrative = "The assistant surface composes available recorded evidence for display. It does not decide, grant permission, mutate, or run work.".to_string();
        let limitations = vec![
            "Charter confirmation: Present intelligence. Never become authority.".into(),
            "Conversation evidence is non-actionable and cannot replace upstream evidence owners"
                .into(),
            "Unavailable upstream surfaces remain unavailable in the presentation".into(),
        ];
        let surface_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                human_ask,
                lineage.lineage_id,
                gaps.iter()
                    .map(|g| g.gap_id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        );

        let snap = Self {
            surface_id,
            workspace_id,
            generated_at,
            status: AssistantSurfaceStatus::Current,
            superseded_at: None,
            human_ask,
            scope,
            utterance,
            lineage,
            gaps,
            diagnostics,
            narrative_summary,
            narrative,
            limitations,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        snap.validate()?;
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, superseded_at: impl Into<String>) {
        self.status = AssistantSurfaceStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.utterance.is_non_actionable()
            && self.lineage.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantSurfaceError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantSurfaceError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || !self.utterance.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantSurfaceError::MustNotBeActionable);
        }
        reject_forbidden(&self.utterance.body)?;
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for citation in &self.utterance.citations {
            reject_forbidden(&citation.excerpt)?;
        }
        for gap in &self.gaps {
            reject_forbidden(&gap.description)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceHistoryEntry {
    pub surface_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub citation_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantSurfaceHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantSurfaceSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            surface_id: snap.surface_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            citation_count: snap.utterance.citations.len(),
            gap_count: snap.gaps.len(),
            terminal: true,
            actionable: false,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && matches!(self.status.as_str(), "superseded" | "archived")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantSurfaceProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantSurfaceSnapshot>,
    pub history: Vec<AssistantSurfaceHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantSurfaceProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantSurfaceSnapshot>,
        history: Vec<AssistantSurfaceHistoryEntry>,
        history_count: usize,
        projected_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            current,
            history,
            history_count,
            projected_at: projected_at.into(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.history.iter().all(|h| h.is_non_actionable())
            && self
                .current
                .as_ref()
                .map(|c| c.is_non_executing())
                .unwrap_or(true)
    }

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantSurfaceSummary {
        WorkspaceAssistantSurfaceSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            citation_count: self
                .current
                .as_ref()
                .map(|c| c.utterance.citations.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            narrative_summary: self.current.as_ref().map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantSurfaceSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub citation_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantSurfaceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantSurfaceExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub citation_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantSurfaceExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantSurfaceSnapshot) -> Self {
        Self {
            explanation_id: format!("assistant_surface_explanation:{}", snap.surface_id),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            citation_summaries: snap
                .utterance
                .citations
                .iter()
                .map(|c| format!("{} -> {}", c.origin_domain, c.artefact_ref))
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.gap_kind, g.description))
                .collect(),
            lineage_summaries: snap.lineage.contributing_artefacts.clone(),
            uncertainty: snap.diagnostics.unavailable_surfaces.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SurfaceSignal {
    artefact_ref: String,
    source_revision: Option<String>,
    excerpt: String,
}

fn unavailable_gap(surface: &str) -> AssistantSurfaceGap {
    AssistantSurfaceGap::record(
        AssistantSurfaceGap::KIND_UNAVAILABLE_UPSTREAM,
        format!(
            "Upstream surface '{}' is unavailable for this assistant turn",
            surface
        ),
        vec![surface.into()],
    )
}

fn assistant_body(citations: &[AssistantCitation], unavailable: &[String]) -> String {
    let mut parts = vec![
        "This is a presentation of recorded evidence only; it carries no authority and contains no action.".to_string(),
    ];
    if citations.is_empty() {
        parts.push("No upstream citations are available for this turn.".into());
    } else {
        parts.push(format!(
            "Available citations: {}.",
            citations
                .iter()
                .map(|c| format!("{} ({})", c.origin_domain, c.artefact_ref))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !unavailable.is_empty() {
        parts.push(format!("Unavailable surfaces: {}.", unavailable.join(", ")));
    }
    parts.push("The recorded evidence remains owned by its upstream surfaces.".into());
    parts.join(" ")
}

fn reject_forbidden(text: &str) -> Result<(), AssistantSurfaceError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_ASSISTANT_PHRASES)
        .map_err(|phrase| AssistantSurfaceError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compose_none(ask: &str) -> WorkspaceAssistantSurfaceSnapshot {
        WorkspaceAssistantSurfaceSnapshot::compose(
            "ws",
            "2026-07-29T00:00:00Z",
            ask,
            AssistantSurfaceScope::presentation_default(),
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
    fn empty_ask_records_gap() {
        let snap = compose_none("  ");
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantSurfaceGap::KIND_EMPTY_ASK));
        assert_eq!(snap.human_ask, "  ");
    }

    #[test]
    fn unavailable_upstreams_are_preserved() {
        let snap = compose_none("what evidence is recorded?");
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantSurfaceGap::KIND_UNAVAILABLE_UPSTREAM));
        assert!(!snap.diagnostics.unavailable_surfaces.is_empty());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let left = compose_none("summarize recorded evidence");
        let right = compose_none("summarize recorded evidence");
        assert_eq!(left.surface_id, right.surface_id);
        assert_eq!(left.utterance, right.utterance);
        assert_eq!(left.lineage, right.lineage);
    }

    #[test]
    fn history_is_non_actionable() {
        let mut snap = compose_none("summarize recorded evidence");
        snap.mark_superseded("2026-07-29T01:00:00Z");
        let entry = AssistantSurfaceHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
    }

    #[test]
    fn assistant_never_implies_authority() {
        let snap = compose_none("summarize recorded evidence");
        assert!(snap.validate().is_ok());
        assert!(!snap
            .utterance
            .body
            .to_ascii_lowercase()
            .contains("you should"));
        assert!(!snap
            .utterance
            .body
            .to_ascii_lowercase()
            .contains("decision is"));
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
    }

    #[test]
    fn projection_is_non_executable() {
        let projection = WorkspaceAssistantSurfaceProjection::assemble(
            "ws",
            Some(compose_none("summarize recorded evidence")),
            vec![],
            0,
            "2026-07-29T00:00:00Z",
        );
        assert!(projection.is_non_commandable());
    }

    #[test]
    fn citations_require_provenance_when_available() {
        let citation = AssistantCitation::present(
            "workspace_evidence_completeness",
            "evidence_completeness:abc",
            Some("evidence_completeness:abc".into()),
            "Evidence completeness: recorded summary",
        );
        assert!(citation.is_non_actionable());
        assert_eq!(
            citation.source_revision.as_deref(),
            Some("evidence_completeness:abc")
        );
        assert!(!citation.artefact_ref.is_empty());
    }

    #[test]
    fn forbidden_authority_language_is_rejected() {
        assert!(reject_forbidden("you should approve").is_err());
        assert!(reject_forbidden("recorded evidence only").is_ok());
    }
}
