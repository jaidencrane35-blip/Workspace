//! Workspace Assistant Explanation Intelligence - Programme IV Batch 14.
//!
//! Clarify recorded evidence. Never conclude what it means.
//! Ownership clarification beside Batches 11–13 — not a second Programme III
//! Explanation Layer / reasoning engine.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_assistant_context::WorkspaceAssistantContextProjection;
use crate::workspace_assistant_retrieval::WorkspaceAssistantRetrievalProjection;
use crate::workspace_assistant_surface::{AssistantSurfaceScope, WorkspaceAssistantSurfaceProjection};
use crate::workspace_evidence_consistency::WorkspaceEvidenceConsistencyProjection;
use crate::workspace_evidence_contract::{
    reject_forbidden_phrases_with, stable_digest, AUTHORITY_EFFECT_NONE,
};
use crate::workspace_evidence_navigation::WorkspaceEvidenceNavigationProjection;
use crate::workspace_evidence_trace::WorkspaceEvidenceTraceProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;

const FORBIDDEN_EXPLANATION_PHRASES: &[&str] = &[
    "therefore this is true",
    "this happened because",
    "the correct interpretation",
    "you should",
    "the best action",
    "i conclude",
    "definitely true",
    "approve",
    "execute now",
    "i recommend",
    "decision is",
    "trust me",
];

/// Assistant explanation section / item kinds (string constants).
pub mod section_kind {
    pub const EVIDENCE_EXISTS: &str = "evidence_exists";
    pub const ORIGIN: &str = "origin";
    pub const CHANGE: &str = "change";
    pub const MISSING: &str = "missing";
    pub const CONFLICT: &str = "conflict";
    pub const UNCERTAINTY: &str = "uncertainty";
    pub const GAP: &str = "gap";
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantExplanationError {
    #[error("invalid assistant explanation status: {0}")]
    InvalidStatus(String),

    #[error("assistant explanation artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant explanation artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant explanation text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant explanation snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantExplanationStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantExplanationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantExplanationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantExplanationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationRequest {
    pub request_id: String,
    pub human_ask: String,
    /// Reused from Batch 11 — do not redefine scope flags.
    pub scope: AssistantSurfaceScope,
    /// Factual notes naming which upstreams were consulted.
    pub pathway_notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationRequest {
    fn from_ask_and_scope(human_ask: &str, scope: &AssistantSurfaceScope) -> Self {
        let enabled = enabled_pathway_names(scope);
        let mut pathway_notes = vec![
            "Request packaging records which upstream explanation and evidence pathways were consulted".into(),
            "Pathway notes are factual consult translation — not conclusions or advisory directives".into(),
        ];
        if enabled.is_empty() {
            pathway_notes.push("No explanation pathways were selected by scope".into());
        } else {
            pathway_notes.push(format!(
                "Selected pathways for consult: {}.",
                enabled.join(", ")
            ));
        }
        let request_id = format!(
            "assistant_explanation_request:{}",
            stable_digest(&format!("{}|{}", human_ask, enabled.join(",")))
        );
        Self {
            request_id,
            human_ask: human_ask.to_string(),
            scope: scope.clone(),
            pathway_notes,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationSection {
    pub section_id: String,
    pub kind: String,
    pub title: String,
    pub body: String,
    pub citation_refs: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationSection {
    pub fn package(
        kind: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        citation_refs: Vec<String>,
    ) -> Self {
        let kind = kind.into();
        let title = title.into();
        let body = body.into();
        let mut citation_refs = citation_refs;
        citation_refs.sort();
        citation_refs.dedup();
        let section_id = format!(
            "assistant_explanation_section:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}",
                kind,
                title,
                body,
                citation_refs.join(",")
            ))
        );
        Self {
            section_id,
            kind,
            title,
            body,
            citation_refs,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationCitation {
    pub citation_id: String,
    pub origin_domain: String,
    pub artefact_ref: String,
    pub source_revision: Option<String>,
    pub excerpt: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationCitation {
    pub fn package(
        origin_domain: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        excerpt: impl Into<String>,
    ) -> Self {
        let origin_domain = origin_domain.into();
        let artefact_ref = artefact_ref.into();
        let excerpt = excerpt.into();
        let citation_id = format!(
            "assistant_explanation_citation:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}",
                origin_domain,
                artefact_ref,
                source_revision.as_deref().unwrap_or(""),
                excerpt
            ))
        );
        Self {
            citation_id,
            origin_domain,
            artefact_ref,
            source_revision,
            excerpt,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub source_refs: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationLineage {
    fn from_citations_and_refs(
        citations: &[AssistantExplanationCitation],
        source_refs: Vec<String>,
    ) -> Self {
        let mut contributing_artefacts = Vec::new();
        let mut revisions = Vec::new();
        for citation in citations {
            if !contributing_artefacts.contains(&citation.artefact_ref) {
                contributing_artefacts.push(citation.artefact_ref.clone());
            }
            if let Some(rev) = citation.source_revision.as_ref() {
                if !revisions.contains(rev) {
                    revisions.push(rev.clone());
                }
            }
        }
        contributing_artefacts.sort();
        revisions.sort();
        let mut source_refs = source_refs;
        source_refs.sort();
        source_refs.dedup();
        let lineage_id = format!(
            "assistant_explanation_lineage:{}",
            stable_digest(&format!(
                "{}|{}|{}",
                contributing_artefacts.join(","),
                revisions.join(","),
                source_refs.join(",")
            ))
        );
        Self {
            lineage_id,
            contributing_artefacts,
            revisions,
            source_refs,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationDiagnostics {
    pub consulted_pathways: Vec<String>,
    pub unavailable_pathways: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationDiagnostics {
    fn assemble(consulted_pathways: Vec<String>, unavailable_pathways: Vec<String>) -> Self {
        Self {
            consulted_pathways,
            unavailable_pathways,
            notes: vec![
                "Assistant explanation clarifies recorded evidence only".into(),
                "Clarify does not equal conclude — packages never determine truth or invent causality".into(),
                "No decision, advisory, or Explanation Layer replacement authority is created here".into(),
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
pub struct AssistantExplanationGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_pathways: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantExplanationGap {
    pub const KIND_UNAVAILABLE_UPSTREAM: &'static str = "unavailable_upstream";
    pub const KIND_EMPTY_ASK: &'static str = "empty_ask";
    pub const KIND_INCOMPLETE_PACKAGE: &'static str = "incomplete_package";
    pub const KIND_MISSING_EXPLANATION_LAYER: &'static str = "missing_explanation_layer";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_pathways: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_explanation_gap:{}",
                stable_digest(&format!(
                    "{}|{}|{}",
                    gap_kind,
                    description,
                    affected_pathways.join(",")
                ))
            ),
            gap_kind,
            description,
            affected_pathways,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantExplanationSnapshot {
    pub explanation_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantExplanationStatus,
    pub superseded_at: Option<String>,
    pub request: AssistantExplanationRequest,
    pub sections: Vec<AssistantExplanationSection>,
    pub citations: Vec<AssistantExplanationCitation>,
    pub lineage: AssistantExplanationLineage,
    pub gaps: Vec<AssistantExplanationGap>,
    pub diagnostics: AssistantExplanationDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantExplanationSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_explanation:";

    #[allow(clippy::too_many_arguments)]
    pub fn package(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
        retrieval: Option<&WorkspaceAssistantRetrievalProjection>,
        context: Option<&WorkspaceAssistantContextProjection>,
        surface: Option<&WorkspaceAssistantSurfaceProjection>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        evidence_trace: Option<&WorkspaceEvidenceTraceProjection>,
        evidence_navigation: Option<&WorkspaceEvidenceNavigationProjection>,
        evidence_consistency: Option<&WorkspaceEvidenceConsistencyProjection>,
    ) -> Result<Self, AssistantExplanationError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let human_ask = human_ask.into();

        let request = AssistantExplanationRequest::from_ask_and_scope(&human_ask, &scope);

        let mut sections = Vec::new();
        let mut citations = Vec::new();
        let mut gaps = Vec::new();
        let mut consulted = Vec::new();
        let mut unavailable = Vec::new();
        let mut source_refs = Vec::new();

        if human_ask.trim().is_empty() {
            gaps.push(AssistantExplanationGap::record(
                AssistantExplanationGap::KIND_EMPTY_ASK,
                "Human ask is blank; explanation package records an empty-ask gap without inventing clarity claims",
                vec!["request".into()],
            ));
            sections.push(AssistantExplanationSection::package(
                section_kind::GAP,
                "Empty ask gap",
                "No human ask text was provided; the package records the gap and does not invent recorded evidence claims",
                vec![],
            ));
        }

        // Programme III Explanation Layer — REQUIRED when scoped.
        if scope.include_explanation {
            consulted.push("workspace_explanation".to_string());
            match explanation.and_then(|e| e.current.as_ref()) {
                Some(package) => {
                    source_refs.push(package.explanation_id.clone());
                    citations.push(AssistantExplanationCitation::package(
                        "workspace_explanation",
                        package.explanation_id.clone(),
                        Some(package.explanation_id.clone()),
                        package.narrative.clone(),
                    ));

                    for section in &package.sections {
                        let mut refs: Vec<String> = section
                            .evidence_refs
                            .iter()
                            .map(|r| r.external_ref.clone())
                            .collect();
                        refs.push(section.section_id.clone());
                        sections.push(AssistantExplanationSection::package(
                            section_kind::EVIDENCE_EXISTS,
                            format!("Recorded explanation: {}", section.title),
                            format!(
                                "Recorded explanation package states: {}",
                                section.body
                            ),
                            refs.clone(),
                        ));
                        sections.push(AssistantExplanationSection::package(
                            section_kind::ORIGIN,
                            format!("Origin surface: {}", section.surface),
                            format!(
                                "Evidence Trace / Explanation Layer links this section to surface '{}' as recorded",
                                section.surface
                            ),
                            refs,
                        ));
                    }

                    for gap in &package.gaps {
                        let refs: Vec<String> = gap
                            .evidence_refs
                            .iter()
                            .map(|r| r.external_ref.clone())
                            .collect();
                        sections.push(AssistantExplanationSection::package(
                            section_kind::MISSING,
                            format!("Missing: {}", gap.surface),
                            format!(
                                "Recorded explanation gap: {}",
                                gap.description
                            ),
                            refs,
                        ));
                    }

                    for conflict in &package.conflicts {
                        let refs: Vec<String> = conflict
                            .evidence_refs
                            .iter()
                            .map(|r| r.external_ref.clone())
                            .collect();
                        sections.push(AssistantExplanationSection::package(
                            section_kind::CONFLICT,
                            format!("Conflict on {}", conflict.surface),
                            format!(
                                "Recorded explanation conflict remains unresolved: {}",
                                conflict.description
                            ),
                            refs,
                        ));
                    }

                    if let Some(expl) = explanation {
                        if !expl.history.is_empty() {
                            let hist_refs: Vec<String> = expl
                                .history
                                .iter()
                                .map(|h| h.explanation_id.clone())
                                .collect();
                            sections.push(AssistantExplanationSection::package(
                                section_kind::CHANGE,
                                "Recorded explanation history",
                                format!(
                                    "Upstream explanation history records {} superseded package(s) as recorded",
                                    expl.history.len()
                                ),
                                hist_refs,
                            ));
                        }
                    }
                }
                None => {
                    unavailable.push("workspace_explanation".to_string());
                    gaps.push(AssistantExplanationGap::record(
                        AssistantExplanationGap::KIND_MISSING_EXPLANATION_LAYER,
                        "Programme III Workspace Explanation Layer package is unavailable for this assistant explanation",
                        vec!["workspace_explanation".into()],
                    ));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::GAP,
                        "Missing explanation layer",
                        "Upstream pathway 'workspace_explanation' is unavailable; missing remains missing without invented conclusions",
                        vec!["gap:workspace_explanation".into()],
                    ));
                }
            }
        }

        // Batch 13 retrieval — input package citations.
        if scope.include_semantic_query
            || scope.include_evidence_coverage
            || scope.include_evidence_consistency
            || scope.include_evidence_navigation
            || scope.include_evidence_trace
        {
            consulted.push("workspace_assistant_retrieval".to_string());
            match retrieval.and_then(|r| r.current.as_ref()) {
                Some(package) => {
                    source_refs.push(package.retrieval_id.clone());
                    for item in &package.items {
                        if item.kind == "gap" {
                            sections.push(AssistantExplanationSection::package(
                                section_kind::MISSING,
                                format!("Retrieval gap: {}", item.origin_domain),
                                format!("Retrieval package records: {}", item.excerpt),
                                vec![item.artefact_ref.clone()],
                            ));
                            continue;
                        }
                        citations.push(AssistantExplanationCitation::package(
                            item.origin_domain.clone(),
                            item.artefact_ref.clone(),
                            item.source_revision.clone(),
                            item.excerpt.clone(),
                        ));
                        sections.push(AssistantExplanationSection::package(
                            section_kind::EVIDENCE_EXISTS,
                            format!("Retrieved evidence: {}", item.origin_domain),
                            format!(
                                "Recorded retrieval item presents: {}",
                                item.excerpt
                            ),
                            vec![item.artefact_ref.clone()],
                        ));
                        sections.push(AssistantExplanationSection::package(
                            section_kind::ORIGIN,
                            format!("Retrieval origin: {}", item.origin_domain),
                            format!(
                                "Retrieval cites artefact '{}' from domain '{}' as recorded",
                                item.artefact_ref, item.origin_domain
                            ),
                            vec![item.artefact_ref.clone()],
                        ));
                    }
                    for gap in &package.gaps {
                        sections.push(AssistantExplanationSection::package(
                            section_kind::MISSING,
                            format!("Retrieval gap kind: {}", gap.gap_kind),
                            format!("Retrieval package gap: {}", gap.description),
                            gap.affected_pathways.clone(),
                        ));
                    }
                }
                None => {
                    unavailable.push("workspace_assistant_retrieval".to_string());
                    gaps.push(unavailable_gap("workspace_assistant_retrieval"));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::GAP,
                        "Unavailable retrieval package",
                        "Upstream pathway 'workspace_assistant_retrieval' is unavailable for this explanation package",
                        vec!["gap:workspace_assistant_retrieval".into()],
                    ));
                }
            }
        }

        if scope.include_evidence_trace {
            consulted.push("workspace_evidence_trace".to_string());
            match evidence_trace.and_then(|t| t.current.as_ref()) {
                Some(current) => {
                    source_refs.push(current.trace_id.clone());
                    citations.push(AssistantExplanationCitation::package(
                        "workspace_evidence_trace",
                        current.trace_id.clone(),
                        Some(current.trace_id.clone()),
                        current.narrative_summary.clone(),
                    ));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::ORIGIN,
                        "Evidence Trace origin",
                        format!(
                            "Evidence Trace links recorded provenance as: {}",
                            current.narrative_summary
                        ),
                        vec![current.trace_id.clone()],
                    ));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::EVIDENCE_EXISTS,
                        "Evidence Trace exists",
                        format!(
                            "Recorded Evidence Trace package is present: {}",
                            current.trace_id
                        ),
                        vec![current.trace_id.clone()],
                    ));
                }
                None => {
                    unavailable.push("workspace_evidence_trace".to_string());
                    gaps.push(unavailable_gap("workspace_evidence_trace"));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::GAP,
                        "Unavailable evidence trace",
                        "Upstream pathway 'workspace_evidence_trace' is unavailable for this explanation package",
                        vec!["gap:workspace_evidence_trace".into()],
                    ));
                }
            }
        }

        if scope.include_evidence_navigation {
            consulted.push("workspace_evidence_navigation".to_string());
            match evidence_navigation.and_then(|n| n.current.as_ref()) {
                Some(current) => {
                    source_refs.push(current.navigation_id.clone());
                    citations.push(AssistantExplanationCitation::package(
                        "workspace_evidence_navigation",
                        current.navigation_id.clone(),
                        Some(current.navigation_id.clone()),
                        current.narrative_summary.clone(),
                    ));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::ORIGIN,
                        "Evidence Navigation path",
                        format!(
                            "Evidence Navigation records path evidence as: {}",
                            current.narrative_summary
                        ),
                        vec![current.navigation_id.clone()],
                    ));
                }
                None => {
                    unavailable.push("workspace_evidence_navigation".to_string());
                    gaps.push(unavailable_gap("workspace_evidence_navigation"));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::GAP,
                        "Unavailable evidence navigation",
                        "Upstream pathway 'workspace_evidence_navigation' is unavailable for this explanation package",
                        vec!["gap:workspace_evidence_navigation".into()],
                    ));
                }
            }
        }

        if scope.include_evidence_consistency {
            consulted.push("workspace_evidence_consistency".to_string());
            match evidence_consistency.and_then(|c| c.current.as_ref()) {
                Some(current) => {
                    source_refs.push(current.consistency_id.clone());
                    citations.push(AssistantExplanationCitation::package(
                        "workspace_evidence_consistency",
                        current.consistency_id.clone(),
                        Some(current.consistency_id.clone()),
                        current.narrative_summary.clone(),
                    ));
                    if current.conflicts.is_empty() {
                        sections.push(AssistantExplanationSection::package(
                            section_kind::EVIDENCE_EXISTS,
                            "Consistency observation",
                            format!(
                                "Consistency reports recorded observation: {}",
                                current.narrative_summary
                            ),
                            vec![current.consistency_id.clone()],
                        ));
                    } else {
                        for conflict in &current.conflicts {
                            sections.push(AssistantExplanationSection::package(
                                section_kind::CONFLICT,
                                "Consistency conflict",
                                format!(
                                    "Consistency reports disagreement as recorded: {}",
                                    conflict.conflict_description
                                ),
                                vec![conflict.conflict_id.clone(), current.consistency_id.clone()],
                            ));
                        }
                    }
                }
                None => {
                    unavailable.push("workspace_evidence_consistency".to_string());
                    gaps.push(unavailable_gap("workspace_evidence_consistency"));
                    sections.push(AssistantExplanationSection::package(
                        section_kind::GAP,
                        "Unavailable evidence consistency",
                        "Upstream pathway 'workspace_evidence_consistency' is unavailable for this explanation package",
                        vec!["gap:workspace_evidence_consistency".into()],
                    ));
                }
            }
        }

        // Optional Batch 12 / Batch 11 presentation lineage refs.
        if let Some(projection) = context {
            if let Some(current) = projection.current.as_ref() {
                source_refs.push(current.context_id.clone());
                citations.push(AssistantExplanationCitation::package(
                    "workspace_assistant_context",
                    current.context_id.clone(),
                    Some(current.context_id.clone()),
                    current.narrative_summary.clone(),
                ));
            }
        }
        if let Some(projection) = surface {
            if let Some(current) = projection.current.as_ref() {
                source_refs.push(current.surface_id.clone());
                citations.push(AssistantExplanationCitation::package(
                    "workspace_assistant_surface",
                    current.surface_id.clone(),
                    Some(current.surface_id.clone()),
                    current.narrative_summary.clone(),
                ));
            }
        }

        // Uncertainty sections from diagnostics of unavailable pathways.
        if !unavailable.is_empty() {
            sections.push(AssistantExplanationSection::package(
                section_kind::UNCERTAINTY,
                "Upstream uncertainty",
                format!(
                    "Unavailable pathways remain unavailable: {}",
                    unavailable.join(", ")
                ),
                unavailable
                    .iter()
                    .map(|p| format!("gap:{}", p))
                    .collect(),
            ));
        }

        if !consulted.is_empty()
            && !unavailable.is_empty()
            && unavailable.len() == consulted.len()
        {
            gaps.push(AssistantExplanationGap::record(
                AssistantExplanationGap::KIND_INCOMPLETE_PACKAGE,
                "All consulted pathways were unavailable; explanation package is incomplete",
                consulted.clone(),
            ));
        }

        // Stable order by section_id — packaging only.
        sections.sort_by(|a, b| a.section_id.cmp(&b.section_id));
        citations.sort_by(|a, b| a.citation_id.cmp(&b.citation_id));

        let diagnostics =
            AssistantExplanationDiagnostics::assemble(consulted.clone(), unavailable);
        let lineage =
            AssistantExplanationLineage::from_citations_and_refs(&citations, source_refs);

        let narrative_summary = format!(
            "Assistant explanation: {} section(s), {} citation(s), {} unavailable pathway(s), {} gap(s)",
            sections.len(),
            citations.len(),
            diagnostics.unavailable_pathways.len(),
            gaps.len()
        );
        let narrative = "The assistant explanation package clarifies recorded Programme III explanation and Programme IV evidence for humans. It packages clarity only; it never concludes what the evidence means, invents causality, decides outcomes, advises actions, grants permission, or replaces the Workspace Explanation Layer.".to_string();
        let limitations = vec![
            "Charter confirmation: Clarify recorded evidence. Never conclude what it means.".into(),
            "Explanation packages are non-actionable evidence and cannot replace upstream owners".into(),
            "Unavailable upstream pathways remain unavailable in the package".into(),
            "Conflicts and gaps remain conflicts and gaps — never resolved here".into(),
        ];

        let explanation_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                request.request_id,
                lineage.lineage_id,
                sections
                    .iter()
                    .map(|s| s.section_id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                citations
                    .iter()
                    .map(|c| c.citation_id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                gaps.iter()
                    .map(|g| g.gap_id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        );

        let snap = Self {
            explanation_id,
            workspace_id,
            generated_at,
            status: AssistantExplanationStatus::Current,
            superseded_at: None,
            request,
            sections,
            citations,
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
        self.status = AssistantExplanationStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.request.is_non_actionable()
            && self.sections.iter().all(|s| s.is_non_actionable())
            && self.citations.iter().all(|c| c.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantExplanationError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantExplanationError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || !self.request.is_non_actionable()
            || self.sections.iter().any(|s| !s.is_non_actionable())
            || self.citations.iter().any(|c| !c.is_non_actionable())
            || !self.lineage.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantExplanationError::MustNotBeActionable);
        }
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for section in &self.sections {
            reject_forbidden(&section.title)?;
            reject_forbidden(&section.body)?;
        }
        for citation in &self.citations {
            reject_forbidden(&citation.excerpt)?;
        }
        for gap in &self.gaps {
            reject_forbidden(&gap.description)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden(note)?;
        }
        for note in &self.request.pathway_notes {
            reject_forbidden(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantExplanationHistoryEntry {
    pub explanation_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub section_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantExplanationHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantExplanationSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            explanation_id: snap.explanation_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            section_count: snap.sections.len(),
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
pub struct WorkspaceAssistantExplanationProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantExplanationSnapshot>,
    pub history: Vec<AssistantExplanationHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantExplanationProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantExplanationSnapshot>,
        history: Vec<AssistantExplanationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantExplanationSummary {
        WorkspaceAssistantExplanationSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            section_count: self.current.as_ref().map(|c| c.sections.len()).unwrap_or(0),
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
pub struct WorkspaceAssistantExplanationSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub section_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantExplanationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantExplanationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub section_summaries: Vec<String>,
    pub citation_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantExplanationExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantExplanationSnapshot) -> Self {
        Self {
            explanation_id: format!(
                "assistant_explanation_meta:{}",
                snap.explanation_id
            ),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            section_summaries: snap
                .sections
                .iter()
                .map(|s| format!("{} -> {} [{}]", s.kind, s.title, s.section_id))
                .collect(),
            citation_summaries: snap
                .citations
                .iter()
                .map(|c| {
                    format!(
                        "{} ({}) [{}]",
                        c.artefact_ref, c.origin_domain, c.citation_id
                    )
                })
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.gap_kind, g.description))
                .collect(),
            lineage_summaries: {
                let mut summaries = snap.lineage.contributing_artefacts.clone();
                summaries.extend(snap.lineage.source_refs.iter().cloned());
                summaries
            },
            uncertainty: snap.diagnostics.unavailable_pathways.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn unavailable_gap(pathway: &str) -> AssistantExplanationGap {
    AssistantExplanationGap::record(
        AssistantExplanationGap::KIND_UNAVAILABLE_UPSTREAM,
        format!(
            "Upstream pathway '{}' is unavailable for this explanation package",
            pathway
        ),
        vec![pathway.into()],
    )
}

fn enabled_pathway_names(scope: &AssistantSurfaceScope) -> Vec<String> {
    let mut names = Vec::new();
    if scope.include_explanation {
        names.push("workspace_explanation".into());
    }
    if scope.include_semantic_query
        || scope.include_evidence_coverage
        || scope.include_evidence_consistency
        || scope.include_evidence_navigation
        || scope.include_evidence_trace
    {
        names.push("workspace_assistant_retrieval".into());
    }
    if scope.include_evidence_trace {
        names.push("workspace_evidence_trace".into());
    }
    if scope.include_evidence_navigation {
        names.push("workspace_evidence_navigation".into());
    }
    if scope.include_evidence_consistency {
        names.push("workspace_evidence_consistency".into());
    }
    names
}

fn reject_forbidden(text: &str) -> Result<(), AssistantExplanationError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_EXPLANATION_PHRASES)
        .map_err(|phrase| AssistantExplanationError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_none(
        human_ask: &str,
        scope: AssistantSurfaceScope,
    ) -> WorkspaceAssistantExplanationSnapshot {
        WorkspaceAssistantExplanationSnapshot::package(
            "ws",
            "2026-07-29T00:00:00Z",
            human_ask,
            scope,
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
    fn package_is_non_actionable() {
        let snap = package_none(
            "what does recorded evidence say?",
            AssistantSurfaceScope::explanation_default(),
        );
        assert!(snap.is_non_executing());
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
        assert!(snap.validate().is_ok());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let left = package_none("same ask", AssistantSurfaceScope::explanation_default());
        let right = package_none("same ask", AssistantSurfaceScope::explanation_default());
        assert_eq!(left.explanation_id, right.explanation_id);
        assert_eq!(left.sections, right.sections);
        assert_eq!(left.request, right.request);
        assert_eq!(left.citations, right.citations);
        assert_eq!(left.lineage, right.lineage);
    }

    #[test]
    fn unavailable_upstreams_are_preserved() {
        let snap = package_none("ask", AssistantSurfaceScope::explanation_default());
        assert!(snap.gaps.iter().any(|g| {
            g.gap_kind == AssistantExplanationGap::KIND_UNAVAILABLE_UPSTREAM
                || g.gap_kind == AssistantExplanationGap::KIND_MISSING_EXPLANATION_LAYER
        }));
        assert!(!snap.diagnostics.unavailable_pathways.is_empty());
        assert!(snap
            .sections
            .iter()
            .any(|s| s.kind == section_kind::GAP || s.kind == section_kind::MISSING));
    }

    #[test]
    fn empty_ask_records_gap() {
        let snap = package_none("  ", AssistantSurfaceScope::explanation_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantExplanationGap::KIND_EMPTY_ASK));
        assert_eq!(snap.request.human_ask, "  ");
    }

    #[test]
    fn forbidden_conclusion_language_is_rejected() {
        assert!(reject_forbidden("therefore this is true").is_err());
        assert!(reject_forbidden("this happened because").is_err());
        assert!(reject_forbidden("the correct interpretation").is_err());
        assert!(reject_forbidden("you should").is_err());
        assert!(reject_forbidden("the best action").is_err());
        assert!(reject_forbidden("i conclude").is_err());
        assert!(reject_forbidden("definitely true").is_err());
        assert!(reject_forbidden("i recommend").is_err());
        assert!(reject_forbidden("decision is").is_err());
        assert!(reject_forbidden("trust me").is_err());
        assert!(reject_forbidden("recorded explanation package states").is_ok());
        assert!(reject_forbidden("clarify recorded evidence only").is_ok());
    }

    #[test]
    fn no_causal_invention_when_upstream_silent() {
        let snap = package_none("ask", AssistantSurfaceScope::explanation_default());
        // Without upstream packages there must be no invented causal "because" sections.
        for section in &snap.sections {
            let lower = section.body.to_lowercase();
            assert!(
                !lower.contains("this happened because"),
                "section invented causal language: {}",
                section.body
            );
            assert_ne!(
                section.kind, "cause",
                "must not invent causal section kinds"
            );
        }
        assert!(!snap.narrative.to_lowercase().contains("this happened because"));
        assert!(snap.narrative.contains("never concludes"));
        // When all upstreams silent, sections must be gaps/uncertainty/missing only —
        // never fabricated evidence_exists from thin air without citations that exist.
        let invented_exists = snap.sections.iter().any(|s| {
            s.kind == section_kind::EVIDENCE_EXISTS && s.citation_refs.is_empty()
        });
        assert!(
            !invented_exists,
            "must not invent evidence_exists sections without citation refs"
        );
    }

    #[test]
    fn projection_is_non_commandable() {
        let projection = WorkspaceAssistantExplanationProjection::assemble(
            "ws",
            Some(package_none(
                "ask",
                AssistantSurfaceScope::explanation_default(),
            )),
            vec![],
            0,
            "2026-07-29T00:00:00Z",
        );
        assert!(projection.is_non_commandable());
        let summary = projection.summary(5);
        assert!(!summary.actionable);
        assert_eq!(summary.authority_effect, AUTHORITY_EFFECT_NONE);
    }

    #[test]
    fn sections_cite_refs_or_gaps() {
        let snap = package_none("ask", AssistantSurfaceScope::explanation_default());
        for section in &snap.sections {
            let is_gap_kind = matches!(
                section.kind.as_str(),
                section_kind::GAP | section_kind::MISSING | section_kind::UNCERTAINTY
            );
            assert!(
                !section.citation_refs.is_empty() || is_gap_kind || !snap.gaps.is_empty(),
                "section {} must cite refs or relate to gaps",
                section.section_id
            );
        }
        assert!(snap.gaps.iter().any(|g| {
            g.gap_kind == AssistantExplanationGap::KIND_MISSING_EXPLANATION_LAYER
        }));
    }
}
