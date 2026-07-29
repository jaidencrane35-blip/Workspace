//! Workspace Assistant Retrieval Intelligence - Programme IV Batch 13.
//!
//! Present retrieved evidence. Never rank truth or recommend action.
//! Ownership clarification beside Batches 11–12 — not a second Semantic Query / search engine.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_assistant_context::WorkspaceAssistantContextProjection;
use crate::workspace_assistant_surface::{AssistantSurfaceScope, WorkspaceAssistantSurfaceProjection};
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
use crate::workspace_semantic_query::WorkspaceSemanticQueryProjection;

const FORBIDDEN_RETRIEVAL_PHRASES: &[&str] = &[
    "approve",
    "execute now",
    "you should",
    "this is the correct answer",
    "best source",
    "most relevant",
    "this action should",
    "i recommend",
    "trust me",
    "decision is",
];

/// Assistant retrieval item kinds (string constants).
pub mod item_kind {
    pub const SEMANTIC_MATCH: &str = "semantic_match";
    pub const NAVIGATION_PATH: &str = "navigation_path";
    pub const TRACE_LINK: &str = "trace_link";
    pub const COVERAGE_SIGNAL: &str = "coverage_signal";
    pub const CONSISTENCY_SIGNAL: &str = "consistency_signal";
    pub const DEPENDENCY_SIGNAL: &str = "dependency_signal";
    pub const FRESHNESS_SIGNAL: &str = "freshness_signal";
    pub const COMPLETENESS_SIGNAL: &str = "completeness_signal";
    pub const RELIABILITY_SIGNAL: &str = "reliability_signal";
    pub const CONTEXT_REF: &str = "context_ref";
    pub const GAP: &str = "gap";
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantRetrievalError {
    #[error("invalid assistant retrieval status: {0}")]
    InvalidStatus(String),

    #[error("assistant retrieval artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant retrieval artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant retrieval text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant retrieval snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantRetrievalStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantRetrievalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantRetrievalError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantRetrievalError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantRetrievalRequest {
    pub request_id: String,
    pub human_ask: String,
    /// Reused from Batch 11 — do not redefine scope flags.
    pub scope: AssistantSurfaceScope,
    /// Factual translation notes only (which pathways selected).
    pub pathway_notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantRetrievalRequest {
    fn from_ask_and_scope(human_ask: &str, scope: &AssistantSurfaceScope) -> Self {
        let enabled = enabled_pathway_names(scope);
        let mut pathway_notes = vec![
            "Request packaging translates the human ask into existing pathway consults only".into(),
            "Pathway notes are factual selection translation — not ranking or truth authority".into(),
        ];
        if enabled.is_empty() {
            pathway_notes.push("No retrieval pathways were selected by scope".into());
        } else {
            pathway_notes.push(format!(
                "Selected pathways for consult: {}.",
                enabled.join(", ")
            ));
        }
        let request_id = format!(
            "assistant_retrieval_request:{}",
            stable_digest(&format!(
                "{}|{}",
                human_ask,
                enabled.join(",")
            ))
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
pub struct AssistantRetrievalItem {
    pub item_id: String,
    pub kind: String,
    pub origin_domain: String,
    pub artefact_ref: String,
    pub source_revision: Option<String>,
    /// Recorded upstream excerpt only — never invented.
    pub excerpt: String,
    pub completeness_note: Option<String>,
    pub uncertainty_note: Option<String>,
    /// Stable deterministic display order — NOT relevance rank.
    pub display_order: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantRetrievalItem {
    pub fn package(
        kind: impl Into<String>,
        origin_domain: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        excerpt: impl Into<String>,
        completeness_note: Option<String>,
        uncertainty_note: Option<String>,
        display_order: usize,
    ) -> Self {
        let kind = kind.into();
        let origin_domain = origin_domain.into();
        let artefact_ref = artefact_ref.into();
        let excerpt = excerpt.into();
        let item_id = format!(
            "assistant_retrieval_item:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                kind,
                origin_domain,
                artefact_ref,
                source_revision.as_deref().unwrap_or(""),
                display_order
            ))
        );
        Self {
            item_id,
            kind,
            origin_domain,
            artefact_ref,
            source_revision,
            excerpt,
            completeness_note,
            uncertainty_note,
            display_order,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantRetrievalLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub source_refs: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantRetrievalLineage {
    fn from_items(items: &[AssistantRetrievalItem], source_refs: Vec<String>) -> Self {
        let mut contributing_artefacts = Vec::new();
        let mut revisions = Vec::new();
        for item in items {
            if item.kind == item_kind::GAP {
                continue;
            }
            if !contributing_artefacts.contains(&item.artefact_ref) {
                contributing_artefacts.push(item.artefact_ref.clone());
            }
            if let Some(rev) = item.source_revision.as_ref() {
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
            "assistant_retrieval_lineage:{}",
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
pub struct AssistantRetrievalDiagnostics {
    pub consulted_pathways: Vec<String>,
    pub unavailable_pathways: Vec<String>,
    pub scoped_out_pathways: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantRetrievalDiagnostics {
    fn assemble(
        consulted_pathways: Vec<String>,
        unavailable_pathways: Vec<String>,
        scoped_out_pathways: Vec<String>,
    ) -> Self {
        Self {
            consulted_pathways,
            unavailable_pathways,
            scoped_out_pathways,
            notes: vec![
                "Assistant retrieval presents recorded evidence only".into(),
                "Presentation order is deterministic display packaging — not ranking or truth".into(),
                "No decision, execution, or search-engine authority is created here".into(),
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
pub struct AssistantRetrievalGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_pathways: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantRetrievalGap {
    pub const KIND_UNAVAILABLE_UPSTREAM: &'static str = "unavailable_upstream";
    pub const KIND_EMPTY_ASK: &'static str = "empty_ask";
    pub const KIND_EMPTY_RESULTS: &'static str = "empty_results";
    pub const KIND_INCOMPLETE_PACKAGE: &'static str = "incomplete_package";
    pub const KIND_SCOPED_OUT: &'static str = "scoped_out";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_pathways: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_retrieval_gap:{}",
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
pub struct WorkspaceAssistantRetrievalSnapshot {
    pub retrieval_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantRetrievalStatus,
    pub superseded_at: Option<String>,
    pub request: AssistantRetrievalRequest,
    pub items: Vec<AssistantRetrievalItem>,
    pub lineage: AssistantRetrievalLineage,
    pub gaps: Vec<AssistantRetrievalGap>,
    pub diagnostics: AssistantRetrievalDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantRetrievalSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_retrieval:";

    #[allow(clippy::too_many_arguments)]
    pub fn package(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
        context: Option<&WorkspaceAssistantContextProjection>,
        surface: Option<&WorkspaceAssistantSurfaceProjection>,
        semantic_query: Option<&WorkspaceSemanticQueryProjection>,
        evidence_navigation: Option<&WorkspaceEvidenceNavigationProjection>,
        evidence_trace: Option<&WorkspaceEvidenceTraceProjection>,
        evidence_coverage: Option<&WorkspaceEvidenceCoverageProjection>,
        evidence_consistency: Option<&WorkspaceEvidenceConsistencyProjection>,
        evidence_dependency: Option<&WorkspaceEvidenceDependencyProjection>,
        evidence_freshness: Option<&WorkspaceEvidenceFreshnessProjection>,
        evidence_completeness: Option<&WorkspaceEvidenceCompletenessProjection>,
        evidence_reliability: Option<&WorkspaceEvidenceReliabilityProjection>,
    ) -> Result<Self, AssistantRetrievalError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let human_ask = human_ask.into();

        let request = AssistantRetrievalRequest::from_ask_and_scope(&human_ask, &scope);

        let mut pending_items: Vec<PendingItem> = Vec::new();
        let mut gaps = Vec::new();
        let mut consulted = Vec::new();
        let mut unavailable = Vec::new();
        let mut source_refs = Vec::new();

        if human_ask.trim().is_empty() {
            gaps.push(AssistantRetrievalGap::record(
                AssistantRetrievalGap::KIND_EMPTY_ASK,
                "Human ask is blank; retrieval package records an empty-ask gap without inventing matches",
                vec!["request".into()],
            ));
        }

        let scoped_out = scoped_out_pathway_names(&scope);
        for pathway in &scoped_out {
            gaps.push(AssistantRetrievalGap::record(
                AssistantRetrievalGap::KIND_SCOPED_OUT,
                format!(
                    "Pathway '{}' was scoped out of this retrieval package",
                    pathway
                ),
                vec![pathway.clone()],
            ));
        }

        macro_rules! include_pathway {
            ($flag:expr, $name:expr, $kind:expr, $projection:expr, $extract:expr) => {
                if $flag {
                    consulted.push($name.to_string());
                    match $projection {
                        Some(projection) => {
                            if let Some(signal) = $extract(projection) {
                                source_refs.push(signal.artefact_ref.clone());
                                pending_items.push(PendingItem {
                                    kind: $kind.to_string(),
                                    origin_domain: $name.to_string(),
                                    artefact_ref: signal.artefact_ref,
                                    source_revision: signal.source_revision,
                                    excerpt: signal.excerpt,
                                    completeness_note: signal.completeness_note,
                                    uncertainty_note: signal.uncertainty_note,
                                });
                            } else {
                                unavailable.push($name.to_string());
                                gaps.push(unavailable_gap($name));
                                pending_items.push(PendingItem {
                                    kind: item_kind::GAP.to_string(),
                                    origin_domain: $name.to_string(),
                                    artefact_ref: format!("gap:{}", $name),
                                    source_revision: None,
                                    excerpt: format!(
                                        "Upstream pathway '{}' is unavailable for this retrieval package",
                                        $name
                                    ),
                                    completeness_note: None,
                                    uncertainty_note: Some(
                                        "Unavailable upstream remains unavailable".into(),
                                    ),
                                });
                            }
                        }
                        None => {
                            unavailable.push($name.to_string());
                            gaps.push(unavailable_gap($name));
                            pending_items.push(PendingItem {
                                kind: item_kind::GAP.to_string(),
                                origin_domain: $name.to_string(),
                                artefact_ref: format!("gap:{}", $name),
                                source_revision: None,
                                excerpt: format!(
                                    "Upstream pathway '{}' is unavailable for this retrieval package",
                                    $name
                                ),
                                completeness_note: None,
                                uncertainty_note: Some(
                                    "Unavailable upstream remains unavailable".into(),
                                ),
                            });
                        }
                    }
                }
            };
        }

        include_pathway!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            item_kind::SEMANTIC_MATCH,
            semantic_query,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.query_id.clone(),
                    source_revision: Some(c.query_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            item_kind::NAVIGATION_PATH,
            evidence_navigation,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.navigation_id.clone(),
                    source_revision: Some(c.navigation_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            item_kind::TRACE_LINK,
            evidence_trace,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.trace_id.clone(),
                    source_revision: Some(c.trace_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            item_kind::COVERAGE_SIGNAL,
            evidence_coverage,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.coverage_id.clone(),
                    source_revision: Some(c.coverage_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: Some(c.narrative_summary.clone()),
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            item_kind::CONSISTENCY_SIGNAL,
            evidence_consistency,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.consistency_id.clone(),
                    source_revision: Some(c.consistency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: Some(c.narrative_summary.clone()),
                })
            }
        );
        include_pathway!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            item_kind::DEPENDENCY_SIGNAL,
            evidence_dependency,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.dependency_id.clone(),
                    source_revision: Some(c.dependency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_freshness,
            "workspace_evidence_freshness",
            item_kind::FRESHNESS_SIGNAL,
            evidence_freshness,
            |p: &WorkspaceEvidenceFreshnessProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.freshness_id.clone(),
                    source_revision: Some(c.freshness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_completeness,
            "workspace_evidence_completeness",
            item_kind::COMPLETENESS_SIGNAL,
            evidence_completeness,
            |p: &WorkspaceEvidenceCompletenessProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.completeness_id.clone(),
                    source_revision: Some(c.completeness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: Some(c.narrative_summary.clone()),
                    uncertainty_note: None,
                })
            }
        );
        include_pathway!(
            scope.include_evidence_reliability,
            "workspace_evidence_reliability",
            item_kind::RELIABILITY_SIGNAL,
            evidence_reliability,
            |p: &WorkspaceEvidenceReliabilityProjection| {
                p.current.as_ref().map(|c| RetrievalSignal {
                    artefact_ref: c.reliability_id.clone(),
                    source_revision: Some(c.reliability_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                })
            }
        );

        // Optional Batch 12 / Batch 11 lineage refs — presentation only.
        if let Some(projection) = context {
            if let Some(current) = projection.current.as_ref() {
                pending_items.push(PendingItem {
                    kind: item_kind::CONTEXT_REF.to_string(),
                    origin_domain: "workspace_assistant_context".into(),
                    artefact_ref: current.context_id.clone(),
                    source_revision: Some(current.context_id.clone()),
                    excerpt: current.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                });
                source_refs.push(current.context_id.clone());
            }
        }
        if let Some(projection) = surface {
            if let Some(current) = projection.current.as_ref() {
                pending_items.push(PendingItem {
                    kind: item_kind::CONTEXT_REF.to_string(),
                    origin_domain: "workspace_assistant_surface".into(),
                    artefact_ref: current.surface_id.clone(),
                    source_revision: Some(current.surface_id.clone()),
                    excerpt: current.narrative_summary.clone(),
                    completeness_note: None,
                    uncertainty_note: None,
                });
                source_refs.push(current.surface_id.clone());
            }
        }

        // Stable deterministic ordering by artefact_ref — NEVER relevance rank.
        pending_items.sort_by(|a, b| a.artefact_ref.cmp(&b.artefact_ref));

        let items: Vec<AssistantRetrievalItem> = pending_items
            .into_iter()
            .enumerate()
            .map(|(display_order, pending)| {
                AssistantRetrievalItem::package(
                    pending.kind,
                    pending.origin_domain,
                    pending.artefact_ref,
                    pending.source_revision,
                    pending.excerpt,
                    pending.completeness_note,
                    pending.uncertainty_note,
                    display_order,
                )
            })
            .collect();

        let evidence_items: Vec<_> = items
            .iter()
            .filter(|i| i.kind != item_kind::GAP && i.kind != item_kind::CONTEXT_REF)
            .collect();
        if !consulted.is_empty() && evidence_items.is_empty() {
            gaps.push(AssistantRetrievalGap::record(
                AssistantRetrievalGap::KIND_EMPTY_RESULTS,
                "Consulted pathways returned no recorded evidence items for presentation",
                consulted.clone(),
            ));
        }

        if !consulted.is_empty()
            && evidence_items.is_empty()
            && !unavailable.is_empty()
            && unavailable.len() == consulted.len()
        {
            gaps.push(AssistantRetrievalGap::record(
                AssistantRetrievalGap::KIND_INCOMPLETE_PACKAGE,
                "All consulted pathways were unavailable; retrieval package is incomplete",
                consulted.clone(),
            ));
        }

        let diagnostics =
            AssistantRetrievalDiagnostics::assemble(consulted.clone(), unavailable, scoped_out);
        let lineage = AssistantRetrievalLineage::from_items(&items, source_refs);

        let narrative_summary = format!(
            "Assistant retrieval: {} item(s), {} unavailable pathway(s), {} gap(s), {} lineage ref(s)",
            items.len(),
            diagnostics.unavailable_pathways.len(),
            gaps.len(),
            lineage.contributing_artefacts.len()
        );
        let narrative = "The assistant retrieval package presents recorded Programme IV evidence for display. Presentation order is stable and deterministic; it does not rank truth, invent matches, decide outcomes, grant permission, or run work.".to_string();
        let limitations = vec![
            "Charter confirmation: Present retrieved evidence. Never rank truth or own relevance.".into(),
            "Retrieval packages are non-actionable evidence and cannot replace upstream owners".into(),
            "Unavailable upstream pathways remain unavailable in the package".into(),
            "Display order is packaging only — not relevance or truth ranking".into(),
        ];

        let retrieval_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                request.request_id,
                lineage.lineage_id,
                items
                    .iter()
                    .map(|i| i.item_id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                gaps.iter()
                    .map(|g| g.gap_id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        );

        let snap = Self {
            retrieval_id,
            workspace_id,
            generated_at,
            status: AssistantRetrievalStatus::Current,
            superseded_at: None,
            request,
            items,
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
        self.status = AssistantRetrievalStatus::Superseded;
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
            && self.items.iter().all(|i| i.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantRetrievalError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantRetrievalError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || !self.request.is_non_actionable()
            || self.items.iter().any(|i| !i.is_non_actionable())
            || !self.lineage.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantRetrievalError::MustNotBeActionable);
        }
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for item in &self.items {
            reject_forbidden(&item.excerpt)?;
            if let Some(note) = &item.completeness_note {
                reject_forbidden(note)?;
            }
            if let Some(note) = &item.uncertainty_note {
                reject_forbidden(note)?;
            }
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
pub struct AssistantRetrievalHistoryEntry {
    pub retrieval_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub item_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantRetrievalHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantRetrievalSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            retrieval_id: snap.retrieval_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            item_count: snap.items.len(),
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
pub struct WorkspaceAssistantRetrievalProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantRetrievalSnapshot>,
    pub history: Vec<AssistantRetrievalHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantRetrievalProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantRetrievalSnapshot>,
        history: Vec<AssistantRetrievalHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantRetrievalSummary {
        WorkspaceAssistantRetrievalSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            item_count: self.current.as_ref().map(|c| c.items.len()).unwrap_or(0),
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
pub struct WorkspaceAssistantRetrievalSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub item_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantRetrievalHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantRetrievalExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub item_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantRetrievalExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantRetrievalSnapshot) -> Self {
        Self {
            explanation_id: format!("assistant_retrieval_explanation:{}", snap.retrieval_id),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            item_summaries: snap
                .items
                .iter()
                .map(|i| {
                    format!(
                        "{} -> {} ({}) [display_order={}]",
                        i.kind, i.artefact_ref, i.origin_domain, i.display_order
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

struct RetrievalSignal {
    artefact_ref: String,
    source_revision: Option<String>,
    excerpt: String,
    completeness_note: Option<String>,
    uncertainty_note: Option<String>,
}

struct PendingItem {
    kind: String,
    origin_domain: String,
    artefact_ref: String,
    source_revision: Option<String>,
    excerpt: String,
    completeness_note: Option<String>,
    uncertainty_note: Option<String>,
}

fn unavailable_gap(pathway: &str) -> AssistantRetrievalGap {
    AssistantRetrievalGap::record(
        AssistantRetrievalGap::KIND_UNAVAILABLE_UPSTREAM,
        format!(
            "Upstream pathway '{}' is unavailable for this retrieval package",
            pathway
        ),
        vec![pathway.into()],
    )
}

fn enabled_pathway_names(scope: &AssistantSurfaceScope) -> Vec<String> {
    let mut names = Vec::new();
    if scope.include_semantic_query {
        names.push("workspace_semantic_query".into());
    }
    if scope.include_evidence_navigation {
        names.push("workspace_evidence_navigation".into());
    }
    if scope.include_evidence_trace {
        names.push("workspace_evidence_trace".into());
    }
    if scope.include_evidence_coverage {
        names.push("workspace_evidence_coverage".into());
    }
    if scope.include_evidence_consistency {
        names.push("workspace_evidence_consistency".into());
    }
    if scope.include_evidence_dependency {
        names.push("workspace_evidence_dependency".into());
    }
    if scope.include_evidence_freshness {
        names.push("workspace_evidence_freshness".into());
    }
    if scope.include_evidence_completeness {
        names.push("workspace_evidence_completeness".into());
    }
    if scope.include_evidence_reliability {
        names.push("workspace_evidence_reliability".into());
    }
    names
}

fn scoped_out_pathway_names(scope: &AssistantSurfaceScope) -> Vec<String> {
    let mut names = Vec::new();
    if !scope.include_semantic_query {
        names.push("workspace_semantic_query".into());
    }
    if !scope.include_evidence_navigation {
        names.push("workspace_evidence_navigation".into());
    }
    if !scope.include_evidence_trace {
        names.push("workspace_evidence_trace".into());
    }
    if !scope.include_evidence_coverage {
        names.push("workspace_evidence_coverage".into());
    }
    if !scope.include_evidence_consistency {
        names.push("workspace_evidence_consistency".into());
    }
    if !scope.include_evidence_dependency {
        names.push("workspace_evidence_dependency".into());
    }
    if !scope.include_evidence_freshness {
        names.push("workspace_evidence_freshness".into());
    }
    if !scope.include_evidence_completeness {
        names.push("workspace_evidence_completeness".into());
    }
    if !scope.include_evidence_reliability {
        names.push("workspace_evidence_reliability".into());
    }
    names
}

fn reject_forbidden(text: &str) -> Result<(), AssistantRetrievalError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_RETRIEVAL_PHRASES)
        .map_err(|phrase| AssistantRetrievalError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_none(
        human_ask: &str,
        scope: AssistantSurfaceScope,
    ) -> WorkspaceAssistantRetrievalSnapshot {
        WorkspaceAssistantRetrievalSnapshot::package(
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
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn package_is_non_actionable() {
        let snap = package_none("what evidence is recorded?", AssistantSurfaceScope::retrieval_default());
        assert!(snap.is_non_executing());
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
        assert!(snap.validate().is_ok());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let left = package_none("same ask", AssistantSurfaceScope::retrieval_default());
        let right = package_none("same ask", AssistantSurfaceScope::retrieval_default());
        assert_eq!(left.retrieval_id, right.retrieval_id);
        assert_eq!(left.items, right.items);
        assert_eq!(left.request, right.request);
        assert_eq!(left.lineage, right.lineage);
    }

    #[test]
    fn unavailable_upstreams_are_preserved() {
        let snap = package_none("ask", AssistantSurfaceScope::retrieval_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantRetrievalGap::KIND_UNAVAILABLE_UPSTREAM));
        assert!(!snap.diagnostics.unavailable_pathways.is_empty());
        assert!(snap.items.iter().any(|i| i.kind == item_kind::GAP));
    }

    #[test]
    fn empty_ask_records_gap() {
        let snap = package_none("  ", AssistantSurfaceScope::retrieval_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantRetrievalGap::KIND_EMPTY_ASK));
        assert_eq!(snap.request.human_ask, "  ");
    }

    #[test]
    fn empty_results_gap_when_consulted_but_no_evidence() {
        let snap = package_none("ask with no upstreams", AssistantSurfaceScope::retrieval_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantRetrievalGap::KIND_EMPTY_RESULTS));
        assert!(!snap.diagnostics.consulted_pathways.is_empty());
    }

    #[test]
    fn forbidden_ranking_language_is_rejected() {
        assert!(reject_forbidden("best source").is_err());
        assert!(reject_forbidden("most relevant").is_err());
        assert!(reject_forbidden("i recommend").is_err());
        assert!(reject_forbidden("you should").is_err());
        assert!(reject_forbidden("this is the correct answer").is_err());
        assert!(reject_forbidden("recorded evidence only").is_ok());
        assert!(reject_forbidden("presentation is not ranking").is_ok());
    }

    #[test]
    fn projection_is_non_commandable() {
        let projection = WorkspaceAssistantRetrievalProjection::assemble(
            "ws",
            Some(package_none(
                "ask",
                AssistantSurfaceScope::retrieval_default(),
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
    fn display_order_is_stable_by_artefact_ref() {
        let snap = package_none("ask", AssistantSurfaceScope::retrieval_default());
        let mut refs: Vec<_> = snap.items.iter().map(|i| i.artefact_ref.clone()).collect();
        let unsorted = refs.clone();
        refs.sort();
        assert_eq!(
            unsorted, refs,
            "items must already be sorted by artefact_ref"
        );
        for (idx, item) in snap.items.iter().enumerate() {
            assert_eq!(item.display_order, idx);
        }
        assert!(!snap.narrative.to_lowercase().contains("best"));
        assert!(!snap.narrative.to_lowercase().contains("most relevant"));
        assert!(snap.narrative.contains("does not rank truth"));
    }
}
