//! Workspace Assistant Context Intelligence - Programme IV Batch 12.
//!
//! Select and package context. Never invent or own it.
//! Ownership clarification beside Batch 11 surface — not a clone of turn composition.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_assistant_surface::{AssistantSurfaceScope, WorkspaceAssistantSurfaceProjection};
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

const FORBIDDEN_CONTEXT_PHRASES: &[&str] = &[
    "approve",
    "execute now",
    "i will run",
    "trust me",
    "decision is",
    "you should",
    "let me handle",
    "i remember that you",
    "your goal is",
    "i decided",
];

/// Assistant context item kinds (string constants).
pub mod item_kind {
    pub const CONVERSATION_SELECTION: &str = "conversation_selection";
    pub const RETRIEVED_EVIDENCE: &str = "retrieved_evidence";
    pub const PRIOR_ASSISTANT_EVIDENCE: &str = "prior_assistant_evidence";
    pub const AWARENESS_READONLY: &str = "awareness_readonly";
    pub const GAP: &str = "gap";
}

/// Assistant context item layers (string constants).
pub mod item_layer {
    pub const CONVERSATION_CONTEXT: &str = "conversation_context";
    pub const RETRIEVED_EVIDENCE: &str = "retrieved_evidence";
    pub const PRIOR_ASSISTANT_EVIDENCE: &str = "prior_assistant_evidence";
    pub const AWARENESS_READONLY: &str = "awareness_readonly";
    pub const GAP: &str = "gap";
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantContextError {
    #[error("invalid assistant context status: {0}")]
    InvalidStatus(String),

    #[error("assistant context artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant context artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant context text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant context snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantContextStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantContextStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantContextError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantContextError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantContextItem {
    pub item_id: String,
    pub kind: String,
    pub origin_domain: String,
    pub artefact_ref: String,
    pub source_revision: Option<String>,
    pub excerpt: String,
    pub layer: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantContextItem {
    pub fn package(
        kind: impl Into<String>,
        origin_domain: impl Into<String>,
        artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        excerpt: impl Into<String>,
        layer: impl Into<String>,
    ) -> Self {
        let kind = kind.into();
        let origin_domain = origin_domain.into();
        let artefact_ref = artefact_ref.into();
        let excerpt = excerpt.into();
        let layer = layer.into();
        let item_id = format!(
            "assistant_context_item:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                kind,
                origin_domain,
                artefact_ref,
                source_revision.as_deref().unwrap_or(""),
                layer
            ))
        );
        Self {
            item_id,
            kind,
            origin_domain,
            artefact_ref,
            source_revision,
            excerpt,
            layer,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantContextContinuity {
    pub continuity_id: String,
    pub prior_context_ids: Vec<String>,
    pub prior_surface_ids: Vec<String>,
    /// Surface history / current refs only — never invented turns.
    pub prior_turn_refs: Vec<String>,
    /// Factual packaging notes only.
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantContextContinuity {
    fn from_surface(
        surface: Option<&WorkspaceAssistantSurfaceProjection>,
        prior_context_ids: Vec<String>,
    ) -> Self {
        let mut prior_surface_ids = Vec::new();
        let mut prior_turn_refs = Vec::new();
        let mut notes = vec![
            "Continuity packages recorded assistant surface evidence only".into(),
            "No turns, goals, or intentions are invented here".into(),
        ];

        if let Some(projection) = surface {
            if let Some(current) = projection.current.as_ref() {
                prior_surface_ids.push(current.surface_id.clone());
                prior_turn_refs.push(current.surface_id.clone());
            }
            for entry in &projection.history {
                if !prior_surface_ids.contains(&entry.surface_id) {
                    prior_surface_ids.push(entry.surface_id.clone());
                }
                if !prior_turn_refs.contains(&entry.surface_id) {
                    prior_turn_refs.push(entry.surface_id.clone());
                }
            }
            if prior_surface_ids.is_empty() {
                notes.push("No recorded assistant surface turns are available for continuity".into());
            } else {
                notes.push(format!(
                    "Referenced {} recorded surface turn(s) for continuity packaging",
                    prior_surface_ids.len()
                ));
            }
        } else {
            notes.push("Assistant surface projection was not supplied for continuity".into());
        }

        if prior_context_ids.is_empty() {
            notes.push("No prior context packages were referenced".into());
        }

        let continuity_id = format!(
            "assistant_context_continuity:{}",
            stable_digest(&format!(
                "{}|{}|{}",
                prior_context_ids.join(","),
                prior_surface_ids.join(","),
                prior_turn_refs.join(",")
            ))
        );

        Self {
            continuity_id,
            prior_context_ids,
            prior_surface_ids,
            prior_turn_refs,
            notes,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantContextDiagnostics {
    pub selected_surfaces: Vec<String>,
    pub unavailable_surfaces: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantContextDiagnostics {
    fn assemble(selected_surfaces: Vec<String>, unavailable_surfaces: Vec<String>) -> Self {
        Self {
            selected_surfaces,
            unavailable_surfaces,
            notes: vec![
                "Assistant context packages selected recorded evidence only".into(),
                "No memory, permission, mutation, or lifecycle authority is created here".into(),
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
pub struct AssistantContextGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_surfaces: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantContextGap {
    pub const KIND_UNAVAILABLE_UPSTREAM: &'static str = "unavailable_upstream";
    pub const KIND_MISSING_SELECTION: &'static str = "missing_selection";
    pub const KIND_EMPTY_SCOPE: &'static str = "empty_scope";
    pub const KIND_INCOMPLETE_PACKAGE: &'static str = "incomplete_package";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_surfaces: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_context_gap:{}",
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
pub struct WorkspaceAssistantContextSnapshot {
    pub context_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantContextStatus,
    pub superseded_at: Option<String>,
    /// Reused from Batch 11 — do not redefine scope flags.
    pub scope: AssistantSurfaceScope,
    pub items: Vec<AssistantContextItem>,
    pub continuity: AssistantContextContinuity,
    pub gaps: Vec<AssistantContextGap>,
    pub diagnostics: AssistantContextDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantContextSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_context:";

    #[allow(clippy::too_many_arguments)]
    pub fn package(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: AssistantSurfaceScope,
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
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
        state: Option<&WorkspaceStateSnapshot>,
    ) -> Result<Self, AssistantContextError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut items = Vec::new();
        let mut gaps = Vec::new();
        let mut selected = Vec::new();
        let mut unavailable = Vec::new();

        let scope_empty = scope_all_false(&scope);
        if scope_empty {
            gaps.push(AssistantContextGap::record(
                AssistantContextGap::KIND_EMPTY_SCOPE,
                "Retrieval scope has no surfaces selected; context package contains no upstream selection",
                vec!["scope".into()],
            ));
        } else {
            let enabled = enabled_scope_names(&scope);
            items.push(AssistantContextItem::package(
                item_kind::CONVERSATION_SELECTION,
                "assistant_context",
                format!("scope:{}", stable_digest(&enabled.join(","))),
                None,
                format!("Selected retrieval scope includes: {}.", enabled.join(", ")),
                item_layer::CONVERSATION_CONTEXT,
            ));
        }

        macro_rules! include_upstream {
            ($flag:expr, $name:expr, $projection:expr, $kind:expr, $layer:expr, $extract:expr) => {
                if $flag {
                    selected.push($name.to_string());
                    match $projection {
                        Some(projection) => {
                            if let Some(signal) = $extract(projection) {
                                items.push(AssistantContextItem::package(
                                    $kind,
                                    $name,
                                    signal.artefact_ref,
                                    signal.source_revision,
                                    signal.excerpt,
                                    $layer,
                                ));
                            } else {
                                unavailable.push($name.to_string());
                                gaps.push(unavailable_gap($name));
                                items.push(AssistantContextItem::package(
                                    item_kind::GAP,
                                    $name,
                                    format!("gap:{}", $name),
                                    None,
                                    format!(
                                        "Upstream surface '{}' is unavailable for this context package",
                                        $name
                                    ),
                                    item_layer::GAP,
                                ));
                            }
                        }
                        None => {
                            unavailable.push($name.to_string());
                            gaps.push(unavailable_gap($name));
                            items.push(AssistantContextItem::package(
                                item_kind::GAP,
                                $name,
                                format!("gap:{}", $name),
                                None,
                                format!(
                                    "Upstream surface '{}' is unavailable for this context package",
                                    $name
                                ),
                                item_layer::GAP,
                            ));
                        }
                    }
                }
            };
        }

        include_upstream!(
            scope.include_semantic_query,
            "workspace_semantic_query",
            semantic_query,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceSemanticQueryProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.query_id.clone(),
                    source_revision: Some(c.query_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_navigation,
            "workspace_evidence_navigation",
            evidence_navigation,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceNavigationProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.navigation_id.clone(),
                    source_revision: Some(c.navigation_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_trace,
            "workspace_evidence_trace",
            evidence_trace,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceTraceProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.trace_id.clone(),
                    source_revision: Some(c.trace_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_coverage,
            "workspace_evidence_coverage",
            evidence_coverage,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceCoverageProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.coverage_id.clone(),
                    source_revision: Some(c.coverage_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_consistency,
            "workspace_evidence_consistency",
            evidence_consistency,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceConsistencyProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.consistency_id.clone(),
                    source_revision: Some(c.consistency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_dependency,
            "workspace_evidence_dependency",
            evidence_dependency,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceDependencyProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.dependency_id.clone(),
                    source_revision: Some(c.dependency_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_freshness,
            "workspace_evidence_freshness",
            evidence_freshness,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceFreshnessProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.freshness_id.clone(),
                    source_revision: Some(c.freshness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_completeness,
            "workspace_evidence_completeness",
            evidence_completeness,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceCompletenessProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.completeness_id.clone(),
                    source_revision: Some(c.completeness_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_evidence_reliability,
            "workspace_evidence_reliability",
            evidence_reliability,
            item_kind::RETRIEVED_EVIDENCE,
            item_layer::RETRIEVED_EVIDENCE,
            |p: &WorkspaceEvidenceReliabilityProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.reliability_id.clone(),
                    source_revision: Some(c.reliability_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_explanation,
            "workspace_explanation",
            explanation,
            item_kind::AWARENESS_READONLY,
            item_layer::AWARENESS_READONLY,
            |p: &WorkspaceExplanationSnapshot| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.explanation_id.clone(),
                    source_revision: Some(c.explanation_id.clone()),
                    excerpt: c.narrative.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_contextual,
            "contextual_understanding",
            contextual,
            item_kind::AWARENESS_READONLY,
            item_layer::AWARENESS_READONLY,
            |p: &ContextualUnderstandingProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.understanding_id.clone(),
                    source_revision: Some(c.understanding_id.clone()),
                    excerpt: c.situation_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_knowledge_integration,
            "knowledge_integration",
            knowledge_integration,
            item_kind::AWARENESS_READONLY,
            item_layer::AWARENESS_READONLY,
            |p: &KnowledgeIntegrationProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.integration_id.clone(),
                    source_revision: Some(c.integration_id.clone()),
                    excerpt: c.summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_intelligence_hub,
            "workspace_intelligence_hub",
            intelligence_hub,
            item_kind::AWARENESS_READONLY,
            item_layer::AWARENESS_READONLY,
            |p: &WorkspaceIntelligenceHubProjection| {
                p.current.as_ref().map(|c| ContextSignal {
                    artefact_ref: c.hub_id.clone(),
                    source_revision: Some(c.hub_id.clone()),
                    excerpt: c.narrative_summary.clone(),
                })
            }
        );
        include_upstream!(
            scope.include_state,
            "workspace_state_envelope",
            state,
            item_kind::AWARENESS_READONLY,
            item_layer::AWARENESS_READONLY,
            |p: &WorkspaceStateSnapshot| {
                p.current.as_ref().map(|c| ContextSignal {
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

        // Continuity from recorded surface evidence only — never invent turns.
        let continuity = AssistantContextContinuity::from_surface(surface, vec![]);
        for surface_id in &continuity.prior_surface_ids {
            items.push(AssistantContextItem::package(
                item_kind::PRIOR_ASSISTANT_EVIDENCE,
                "workspace_assistant_surface",
                surface_id.clone(),
                Some(surface_id.clone()),
                format!("Prior recorded assistant surface evidence: {surface_id}"),
                item_layer::PRIOR_ASSISTANT_EVIDENCE,
            ));
        }

        if surface.is_some()
            && continuity.prior_surface_ids.is_empty()
            && continuity.prior_context_ids.is_empty()
        {
            gaps.push(AssistantContextGap::record(
                AssistantContextGap::KIND_MISSING_SELECTION,
                "No prior assistant surface turns or context packages were available for continuity selection",
                vec!["workspace_assistant_surface".into()],
            ));
        }

        let retrieved_or_awareness = items.iter().any(|i| {
            i.kind == item_kind::RETRIEVED_EVIDENCE || i.kind == item_kind::AWARENESS_READONLY
        });
        if !scope_empty && !selected.is_empty() && !retrieved_or_awareness {
            gaps.push(AssistantContextGap::record(
                AssistantContextGap::KIND_INCOMPLETE_PACKAGE,
                "No retrieved or awareness artefacts are available for the selected retrieval scope",
                selected.clone(),
            ));
        }

        let diagnostics = AssistantContextDiagnostics::assemble(selected.clone(), unavailable);
        let narrative_summary = format!(
            "Assistant context: {} item(s), {} unavailable surface(s), {} gap(s), {} continuity ref(s)",
            items.len(),
            diagnostics.unavailable_surfaces.len(),
            gaps.len(),
            continuity.prior_turn_refs.len()
        );
        let narrative = "The assistant context package selects and packages recorded evidence for session display. It does not invent continuity, own memory, decide, grant permission, mutate, or run work.".to_string();
        let limitations = vec![
            "Charter confirmation: Select and package context. Never invent or own it.".into(),
            "Context packages are non-actionable evidence and cannot replace upstream owners".into(),
            "Unavailable upstream surfaces remain unavailable in the package".into(),
            "Continuity references recorded assistant surface ids only".into(),
        ];
        let context_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                continuity.continuity_id,
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
            context_id,
            workspace_id,
            generated_at,
            status: AssistantContextStatus::Current,
            superseded_at: None,
            scope,
            items,
            continuity,
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
        self.status = AssistantContextStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.items.iter().all(|i| i.is_non_actionable())
            && self.continuity.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantContextError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantContextError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || self.items.iter().any(|i| !i.is_non_actionable())
            || !self.continuity.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantContextError::MustNotBeActionable);
        }
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for item in &self.items {
            reject_forbidden(&item.excerpt)?;
        }
        for gap in &self.gaps {
            reject_forbidden(&gap.description)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden(note)?;
        }
        for note in &self.continuity.notes {
            reject_forbidden(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantContextHistoryEntry {
    pub context_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub item_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantContextHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantContextSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            context_id: snap.context_id.clone(),
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
pub struct WorkspaceAssistantContextProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantContextSnapshot>,
    pub history: Vec<AssistantContextHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantContextProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantContextSnapshot>,
        history: Vec<AssistantContextHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantContextSummary {
        WorkspaceAssistantContextSummary {
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
pub struct WorkspaceAssistantContextSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub item_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantContextHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantContextExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub item_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub continuity_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantContextExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantContextSnapshot) -> Self {
        Self {
            explanation_id: format!("assistant_context_explanation:{}", snap.context_id),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            item_summaries: snap
                .items
                .iter()
                .map(|i| format!("{} -> {} ({})", i.kind, i.artefact_ref, i.origin_domain))
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.gap_kind, g.description))
                .collect(),
            continuity_summaries: {
                let mut summaries = snap.continuity.prior_turn_refs.clone();
                summaries.extend(snap.continuity.notes.iter().cloned());
                summaries
            },
            uncertainty: snap.diagnostics.unavailable_surfaces.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct ContextSignal {
    artefact_ref: String,
    source_revision: Option<String>,
    excerpt: String,
}

fn unavailable_gap(surface: &str) -> AssistantContextGap {
    AssistantContextGap::record(
        AssistantContextGap::KIND_UNAVAILABLE_UPSTREAM,
        format!(
            "Upstream surface '{}' is unavailable for this context package",
            surface
        ),
        vec![surface.into()],
    )
}

fn scope_all_false(scope: &AssistantSurfaceScope) -> bool {
    !scope.include_semantic_query
        && !scope.include_evidence_navigation
        && !scope.include_evidence_trace
        && !scope.include_evidence_coverage
        && !scope.include_evidence_consistency
        && !scope.include_evidence_dependency
        && !scope.include_evidence_freshness
        && !scope.include_evidence_completeness
        && !scope.include_evidence_reliability
        && !scope.include_explanation
        && !scope.include_contextual
        && !scope.include_knowledge_integration
        && !scope.include_intelligence_hub
        && !scope.include_state
}

fn enabled_scope_names(scope: &AssistantSurfaceScope) -> Vec<String> {
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
    if scope.include_explanation {
        names.push("workspace_explanation".into());
    }
    if scope.include_contextual {
        names.push("contextual_understanding".into());
    }
    if scope.include_knowledge_integration {
        names.push("knowledge_integration".into());
    }
    if scope.include_intelligence_hub {
        names.push("workspace_intelligence_hub".into());
    }
    if scope.include_state {
        names.push("workspace_state_envelope".into());
    }
    names
}

fn reject_forbidden(text: &str) -> Result<(), AssistantContextError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_CONTEXT_PHRASES)
        .map_err(|phrase| AssistantContextError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_assistant_surface::{AssistantSurfaceHistoryEntry, AssistantSurfaceStatus};

    fn empty_scope() -> AssistantSurfaceScope {
        AssistantSurfaceScope {
            include_semantic_query: false,
            include_evidence_navigation: false,
            include_evidence_trace: false,
            include_evidence_coverage: false,
            include_evidence_consistency: false,
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

    fn package_none(scope: AssistantSurfaceScope) -> WorkspaceAssistantContextSnapshot {
        WorkspaceAssistantContextSnapshot::package(
            "ws",
            "2026-07-29T00:00:00Z",
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
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn package_is_non_actionable() {
        let snap = package_none(AssistantSurfaceScope::presentation_default());
        assert!(snap.is_non_executing());
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
        assert!(snap.validate().is_ok());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let left = package_none(AssistantSurfaceScope::presentation_default());
        let right = package_none(AssistantSurfaceScope::presentation_default());
        assert_eq!(left.context_id, right.context_id);
        assert_eq!(left.items, right.items);
        assert_eq!(left.continuity, right.continuity);
    }

    #[test]
    fn unavailable_upstreams_are_preserved() {
        let snap = package_none(AssistantSurfaceScope::presentation_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantContextGap::KIND_UNAVAILABLE_UPSTREAM));
        assert!(!snap.diagnostics.unavailable_surfaces.is_empty());
        assert!(snap.items.iter().any(|i| i.kind == item_kind::GAP));
    }

    #[test]
    fn empty_scope_records_gap() {
        let snap = package_none(empty_scope());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantContextGap::KIND_EMPTY_SCOPE));
        assert!(!snap
            .items
            .iter()
            .any(|i| i.kind == item_kind::CONVERSATION_SELECTION));
    }

    #[test]
    fn continuity_never_invents_turns() {
        let snap = package_none(AssistantSurfaceScope::presentation_default());
        assert!(snap.continuity.prior_turn_refs.is_empty());
        assert!(snap.continuity.prior_surface_ids.is_empty());
        assert!(!snap.items.iter().any(|i| {
            i.kind == item_kind::PRIOR_ASSISTANT_EVIDENCE
                && i.artefact_ref.starts_with("invented")
        }));

        let surface = WorkspaceAssistantSurfaceProjection::assemble(
            "ws",
            None,
            vec![AssistantSurfaceHistoryEntry {
                surface_id: "assistant_surface:recorded".into(),
                status: AssistantSurfaceStatus::Superseded.as_str().into(),
                created_at: "t0".into(),
                superseded_at: Some("t1".into()),
                citation_count: 0,
                gap_count: 0,
                terminal: true,
                actionable: false,
                authority_effect: AUTHORITY_EFFECT_NONE.into(),
            }],
            1,
            "2026-07-29T00:00:00Z",
        );
        let with_surface = WorkspaceAssistantContextSnapshot::package(
            "ws",
            "2026-07-29T00:00:00Z",
            empty_scope(),
            Some(&surface),
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
        assert_eq!(
            with_surface.continuity.prior_turn_refs,
            vec!["assistant_surface:recorded".to_string()]
        );
        assert!(with_surface
            .items
            .iter()
            .any(|i| i.kind == item_kind::PRIOR_ASSISTANT_EVIDENCE
                && i.artefact_ref == "assistant_surface:recorded"));
    }

    #[test]
    fn forbidden_authority_language_is_rejected() {
        assert!(reject_forbidden("you should approve").is_err());
        assert!(reject_forbidden("your goal is").is_err());
        assert!(reject_forbidden("recorded evidence only").is_ok());
    }

    #[test]
    fn projection_is_non_commandable() {
        let projection = WorkspaceAssistantContextProjection::assemble(
            "ws",
            Some(package_none(AssistantSurfaceScope::presentation_default())),
            vec![],
            0,
            "2026-07-29T00:00:00Z",
        );
        assert!(projection.is_non_commandable());
        let summary = projection.summary(5);
        assert!(!summary.actionable);
        assert_eq!(summary.authority_effect, AUTHORITY_EFFECT_NONE);
    }
}
