//! Workspace Assistant Interaction Intelligence - Programme IV Batch 15.
//!
//! Coordinate interaction flow. Never act for the user.
//! Ownership clarification beside Batches 11–14 — not a cognitive engine,
//! agent, memory system, or decision authority.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_assistant_context::WorkspaceAssistantContextProjection;
use crate::workspace_assistant_explanation::WorkspaceAssistantExplanationProjection;
use crate::workspace_assistant_retrieval::WorkspaceAssistantRetrievalProjection;
use crate::workspace_assistant_surface::{AssistantSurfaceScope, WorkspaceAssistantSurfaceProjection};
use crate::workspace_evidence_contract::{
    reject_forbidden_phrases_with, stable_digest, AUTHORITY_EFFECT_NONE,
};

const FORBIDDEN_INTERACTION_PHRASES: &[&str] = &[
    "i will handle",
    "let me handle",
    "i will run",
    "acting on your behalf",
    "i commit",
    "you should",
    "approve",
    "execute now",
    "i decided",
    "your goal is",
    "trust me",
    "decision is",
    "i remember that you",
    "autonomous",
];

/// Assistant interaction flow step kinds (string constants).
/// Sequence is presentation packaging order only — NOT a plan of action.
pub mod step_kind {
    pub const SURFACE: &str = "surface";
    pub const CONTEXT: &str = "context";
    pub const RETRIEVAL: &str = "retrieval";
    pub const EXPLANATION: &str = "explanation";
    pub const GAP: &str = "gap";
}

const PACKAGE_CONTEXT: &str = "workspace_assistant_context";
const PACKAGE_RETRIEVAL: &str = "workspace_assistant_retrieval";
const PACKAGE_EXPLANATION: &str = "workspace_assistant_explanation";
const PACKAGE_SURFACE: &str = "workspace_assistant_surface";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantInteractionError {
    #[error("invalid assistant interaction status: {0}")]
    InvalidStatus(String),

    #[error("assistant interaction artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant interaction artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant interaction text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant interaction snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantInteractionStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantInteractionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantInteractionError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantInteractionError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantInteractionRequest {
    pub request_id: String,
    pub human_ask: String,
    /// Reused from Batch 11 — do not redefine scope flags.
    pub scope: AssistantSurfaceScope,
    /// Factual notes naming which packages were routed for presentation.
    pub pathway_notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantInteractionRequest {
    fn from_ask_and_scope(human_ask: &str, scope: &AssistantSurfaceScope) -> Self {
        let pathway_notes = vec![
            "Request packaging records which assistant packages are routed for presentation".into(),
            "Pathway notes are factual flow coordination — coordinate does not equal act".into(),
            format!(
                "Scope flags retained from AssistantSurfaceScope (interaction_default pathways selected={})",
                scope.include_explanation
                    || scope.include_semantic_query
                    || scope.include_contextual
                    || scope.include_state
            ),
        ];
        let request_id = format!(
            "assistant_interaction_request:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                human_ask,
                scope.include_explanation,
                scope.include_semantic_query,
                scope.include_contextual,
                scope.include_state
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
pub struct AssistantInteractionStep {
    pub step_id: String,
    pub kind: String,
    /// Presentation order only — NOT a plan / commitment / execution sequence.
    pub sequence: usize,
    pub package_ref: Option<String>,
    pub origin_domain: String,
    pub excerpt: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantInteractionStep {
    fn package(
        kind: impl Into<String>,
        sequence: usize,
        package_ref: Option<String>,
        origin_domain: impl Into<String>,
        excerpt: impl Into<String>,
    ) -> Self {
        let kind = kind.into();
        let origin_domain = origin_domain.into();
        let excerpt = excerpt.into();
        let step_id = format!(
            "assistant_interaction_step:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                kind,
                sequence,
                package_ref.as_deref().unwrap_or(""),
                origin_domain,
                excerpt
            ))
        );
        Self {
            step_id,
            kind,
            sequence,
            package_ref,
            origin_domain,
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
pub struct AssistantInteractionRoute {
    pub route_id: String,
    /// Package ids in presentation flow order.
    pub routed_packages: Vec<String>,
    pub missing_packages: Vec<String>,
    /// Must state that coordinate ≠ act.
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantInteractionRoute {
    fn assemble(routed_packages: Vec<String>, missing_packages: Vec<String>) -> Self {
        let notes = vec![
            "Route packaging coordinates presentation among Batches 11–14 only".into(),
            "Coordinate does not equal act — routing never executes, decides, or commits".into(),
            "Flow order is presentation sequencing metadata, not a plan of action".into(),
        ];
        let route_id = format!(
            "assistant_interaction_route:{}",
            stable_digest(&format!(
                "{}|{}",
                routed_packages.join(","),
                missing_packages.join(",")
            ))
        );
        Self {
            route_id,
            routed_packages,
            missing_packages,
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
pub struct AssistantInteractionDiagnostics {
    pub consulted_packages: Vec<String>,
    pub unavailable_packages: Vec<String>,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantInteractionDiagnostics {
    fn assemble(consulted_packages: Vec<String>, unavailable_packages: Vec<String>) -> Self {
        Self {
            consulted_packages,
            unavailable_packages,
            notes: vec![
                "Assistant interaction coordinates conversation flow packaging only".into(),
                "Coordinate does not equal act — packages never act for the user".into(),
                "No memory, Intent, decision, advisory, or execution authority is created here".into(),
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
pub struct AssistantInteractionGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_packages: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantInteractionGap {
    pub const KIND_UNAVAILABLE_PACKAGE: &'static str = "unavailable_package";
    pub const KIND_EMPTY_ASK: &'static str = "empty_ask";
    pub const KIND_INCOMPLETE_FLOW: &'static str = "incomplete_flow";
    pub const KIND_MISSING_SURFACE: &'static str = "missing_surface";
    pub const KIND_MISSING_CONTEXT: &'static str = "missing_context";
    pub const KIND_MISSING_RETRIEVAL: &'static str = "missing_retrieval";
    pub const KIND_MISSING_EXPLANATION: &'static str = "missing_explanation";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_packages: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_interaction_gap:{}",
                stable_digest(&format!(
                    "{}|{}|{}",
                    gap_kind,
                    description,
                    affected_packages.join(",")
                ))
            ),
            gap_kind,
            description,
            affected_packages,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantInteractionSnapshot {
    pub interaction_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantInteractionStatus,
    pub superseded_at: Option<String>,
    pub request: AssistantInteractionRequest,
    pub steps: Vec<AssistantInteractionStep>,
    pub route: AssistantInteractionRoute,
    pub gaps: Vec<AssistantInteractionGap>,
    pub diagnostics: AssistantInteractionDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantInteractionSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_interaction:";

    /// Package an interaction flow from Batches 11–14 projections.
    ///
    /// Presentation sequencing (NOT a plan): context → retrieval → explanation → surface.
    /// Surface is last as utterance presentation. Sequence numbers are packaging
    /// metadata only — never agency, commitment, or execution order.
    pub fn package(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
        surface: Option<&WorkspaceAssistantSurfaceProjection>,
        context: Option<&WorkspaceAssistantContextProjection>,
        retrieval: Option<&WorkspaceAssistantRetrievalProjection>,
        explanation: Option<&WorkspaceAssistantExplanationProjection>,
    ) -> Result<Self, AssistantInteractionError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let human_ask = human_ask.into();

        let request = AssistantInteractionRequest::from_ask_and_scope(&human_ask, &scope);

        let mut steps = Vec::new();
        let mut gaps = Vec::new();
        let mut consulted = Vec::new();
        let mut unavailable = Vec::new();
        let mut routed_packages = Vec::new();
        let mut missing_packages = Vec::new();

        if human_ask.trim().is_empty() {
            gaps.push(AssistantInteractionGap::record(
                AssistantInteractionGap::KIND_EMPTY_ASK,
                "Human ask is blank; interaction package records an empty-ask gap without inventing flow claims",
                vec!["request".into()],
            ));
            steps.push(AssistantInteractionStep::package(
                step_kind::GAP,
                0,
                None,
                "request",
                "No human ask text was provided; the package records the gap and does not invent interaction commitments",
            ));
        }

        // Presentation sequencing only (NOT a plan):
        // 1 context → 2 retrieval → 3 explanation → 4 surface (utterance presentation last).
        let mut sequence: usize = 1;

        // Batch 12 context — REQUIRED composition input.
        consulted.push(PACKAGE_CONTEXT.to_string());
        match context.and_then(|c| c.current.as_ref()) {
            Some(package) => {
                routed_packages.push(package.context_id.clone());
                steps.push(AssistantInteractionStep::package(
                    step_kind::CONTEXT,
                    sequence,
                    Some(package.context_id.clone()),
                    PACKAGE_CONTEXT,
                    format!(
                        "Context package '{}' is routed for presentation continuity packaging",
                        package.context_id
                    ),
                ));
            }
            None => {
                unavailable.push(PACKAGE_CONTEXT.to_string());
                missing_packages.push(PACKAGE_CONTEXT.to_string());
                gaps.push(AssistantInteractionGap::record(
                    AssistantInteractionGap::KIND_MISSING_CONTEXT,
                    "Assistant context package is unavailable for this interaction flow",
                    vec![PACKAGE_CONTEXT.into()],
                ));
                gaps.push(unavailable_gap(PACKAGE_CONTEXT));
                steps.push(AssistantInteractionStep::package(
                    step_kind::GAP,
                    sequence,
                    None,
                    PACKAGE_CONTEXT,
                    "Upstream package 'workspace_assistant_context' is unavailable; missing remains missing without invented continuity",
                ));
            }
        }
        sequence += 1;

        // Batch 13 retrieval — REQUIRED composition input.
        consulted.push(PACKAGE_RETRIEVAL.to_string());
        match retrieval.and_then(|r| r.current.as_ref()) {
            Some(package) => {
                routed_packages.push(package.retrieval_id.clone());
                steps.push(AssistantInteractionStep::package(
                    step_kind::RETRIEVAL,
                    sequence,
                    Some(package.retrieval_id.clone()),
                    PACKAGE_RETRIEVAL,
                    format!(
                        "Retrieval package '{}' is routed for presentation of recorded evidence items",
                        package.retrieval_id
                    ),
                ));
            }
            None => {
                unavailable.push(PACKAGE_RETRIEVAL.to_string());
                missing_packages.push(PACKAGE_RETRIEVAL.to_string());
                gaps.push(AssistantInteractionGap::record(
                    AssistantInteractionGap::KIND_MISSING_RETRIEVAL,
                    "Assistant retrieval package is unavailable for this interaction flow",
                    vec![PACKAGE_RETRIEVAL.into()],
                ));
                gaps.push(unavailable_gap(PACKAGE_RETRIEVAL));
                steps.push(AssistantInteractionStep::package(
                    step_kind::GAP,
                    sequence,
                    None,
                    PACKAGE_RETRIEVAL,
                    "Upstream package 'workspace_assistant_retrieval' is unavailable; missing remains missing without invented retrieval",
                ));
            }
        }
        sequence += 1;

        // Batch 14 explanation — REQUIRED composition input.
        consulted.push(PACKAGE_EXPLANATION.to_string());
        match explanation.and_then(|e| e.current.as_ref()) {
            Some(package) => {
                routed_packages.push(package.explanation_id.clone());
                steps.push(AssistantInteractionStep::package(
                    step_kind::EXPLANATION,
                    sequence,
                    Some(package.explanation_id.clone()),
                    PACKAGE_EXPLANATION,
                    format!(
                        "Explanation package '{}' is routed for presentation clarity packaging",
                        package.explanation_id
                    ),
                ));
            }
            None => {
                unavailable.push(PACKAGE_EXPLANATION.to_string());
                missing_packages.push(PACKAGE_EXPLANATION.to_string());
                gaps.push(AssistantInteractionGap::record(
                    AssistantInteractionGap::KIND_MISSING_EXPLANATION,
                    "Assistant explanation package is unavailable for this interaction flow",
                    vec![PACKAGE_EXPLANATION.into()],
                ));
                gaps.push(unavailable_gap(PACKAGE_EXPLANATION));
                steps.push(AssistantInteractionStep::package(
                    step_kind::GAP,
                    sequence,
                    None,
                    PACKAGE_EXPLANATION,
                    "Upstream package 'workspace_assistant_explanation' is unavailable; missing remains missing without invented clarity",
                ));
            }
        }
        sequence += 1;

        // Batch 11 surface — REQUIRED; last as utterance presentation.
        consulted.push(PACKAGE_SURFACE.to_string());
        match surface.and_then(|s| s.current.as_ref()) {
            Some(package) => {
                routed_packages.push(package.surface_id.clone());
                steps.push(AssistantInteractionStep::package(
                    step_kind::SURFACE,
                    sequence,
                    Some(package.surface_id.clone()),
                    PACKAGE_SURFACE,
                    format!(
                        "Surface package '{}' is routed last for utterance presentation packaging",
                        package.surface_id
                    ),
                ));
            }
            None => {
                unavailable.push(PACKAGE_SURFACE.to_string());
                missing_packages.push(PACKAGE_SURFACE.to_string());
                gaps.push(AssistantInteractionGap::record(
                    AssistantInteractionGap::KIND_MISSING_SURFACE,
                    "Assistant surface package is unavailable for this interaction flow",
                    vec![PACKAGE_SURFACE.into()],
                ));
                gaps.push(unavailable_gap(PACKAGE_SURFACE));
                steps.push(AssistantInteractionStep::package(
                    step_kind::GAP,
                    sequence,
                    None,
                    PACKAGE_SURFACE,
                    "Upstream package 'workspace_assistant_surface' is unavailable; missing remains missing without invented utterances",
                ));
            }
        }

        if !missing_packages.is_empty() {
            gaps.push(AssistantInteractionGap::record(
                AssistantInteractionGap::KIND_INCOMPLETE_FLOW,
                format!(
                    "Interaction flow is incomplete; missing required packages: {}",
                    missing_packages.join(", ")
                ),
                missing_packages.clone(),
            ));
        }

        // Stable secondary order by step_id within identical sequence (deterministic).
        steps.sort_by(|a, b| {
            a.sequence
                .cmp(&b.sequence)
                .then_with(|| a.step_id.cmp(&b.step_id))
        });

        let route = AssistantInteractionRoute::assemble(routed_packages, missing_packages);
        let diagnostics =
            AssistantInteractionDiagnostics::assemble(consulted.clone(), unavailable);

        let narrative_summary = format!(
            "Assistant interaction: {} step(s), {} routed package(s), {} unavailable package(s), {} gap(s)",
            steps.len(),
            route.routed_packages.len(),
            diagnostics.unavailable_packages.len(),
            gaps.len()
        );
        let narrative = "The assistant interaction package coordinates conversation flow packaging across Batches 11–14 for humans. It packages interaction flow only; it never acts for the user, never owns memory or identity, never infers Intent, never decides or advises actions, never plans or executes, never makes commitments, and never runs self-directed agent loops.".to_string();
        let limitations = vec![
            "Charter confirmation: Coordinate interaction flow. Never act for the user.".into(),
            "Interaction packages are non-actionable evidence and cannot replace Batches 11–14 owners".into(),
            "Unavailable capability packages remain unavailable in the package".into(),
            "Flow order is presentation sequencing only — not a plan, commitment, or execution path".into(),
        ];

        let interaction_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                request.request_id,
                route.route_id,
                steps
                    .iter()
                    .map(|s| s.step_id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                gaps.iter()
                    .map(|g| g.gap_id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        );

        let snap = Self {
            interaction_id,
            workspace_id,
            generated_at,
            status: AssistantInteractionStatus::Current,
            superseded_at: None,
            request,
            steps,
            route,
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
        self.status = AssistantInteractionStatus::Superseded;
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
            && self.steps.iter().all(|s| s.is_non_actionable())
            && self.route.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantInteractionError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantInteractionError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || !self.request.is_non_actionable()
            || self.steps.iter().any(|s| !s.is_non_actionable())
            || !self.route.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantInteractionError::MustNotBeActionable);
        }
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for step in &self.steps {
            reject_forbidden(&step.excerpt)?;
        }
        for gap in &self.gaps {
            reject_forbidden(&gap.description)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden(note)?;
        }
        for note in &self.route.notes {
            reject_forbidden(note)?;
        }
        for note in &self.request.pathway_notes {
            reject_forbidden(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantInteractionHistoryEntry {
    pub interaction_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub step_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantInteractionHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantInteractionSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            interaction_id: snap.interaction_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            step_count: snap.steps.len(),
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
pub struct WorkspaceAssistantInteractionProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantInteractionSnapshot>,
    pub history: Vec<AssistantInteractionHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantInteractionProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantInteractionSnapshot>,
        history: Vec<AssistantInteractionHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantInteractionSummary {
        WorkspaceAssistantInteractionSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            step_count: self.current.as_ref().map(|c| c.steps.len()).unwrap_or(0),
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
pub struct WorkspaceAssistantInteractionSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub step_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantInteractionHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantInteractionExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub step_summaries: Vec<String>,
    pub route_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantInteractionExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantInteractionSnapshot) -> Self {
        Self {
            explanation_id: format!("assistant_interaction_meta:{}", snap.interaction_id),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            step_summaries: snap
                .steps
                .iter()
                .map(|s| {
                    format!(
                        "{} seq={} -> {} [{}]",
                        s.kind,
                        s.sequence,
                        s.package_ref.as_deref().unwrap_or("gap"),
                        s.step_id
                    )
                })
                .collect(),
            route_summaries: {
                let mut summaries = snap.route.routed_packages.clone();
                summaries.extend(
                    snap.route
                        .missing_packages
                        .iter()
                        .map(|p| format!("missing:{}", p)),
                );
                summaries
            },
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.gap_kind, g.description))
                .collect(),
            uncertainty: snap.diagnostics.unavailable_packages.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn unavailable_gap(package: &str) -> AssistantInteractionGap {
    AssistantInteractionGap::record(
        AssistantInteractionGap::KIND_UNAVAILABLE_PACKAGE,
        format!(
            "Capability package '{}' is unavailable for this interaction flow",
            package
        ),
        vec![package.into()],
    )
}

fn reject_forbidden(text: &str) -> Result<(), AssistantInteractionError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_INTERACTION_PHRASES)
        .map_err(|phrase| AssistantInteractionError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_none(
        human_ask: &str,
        scope: AssistantSurfaceScope,
    ) -> WorkspaceAssistantInteractionSnapshot {
        WorkspaceAssistantInteractionSnapshot::package(
            "ws",
            "2026-07-29T00:00:00Z",
            human_ask,
            scope,
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
            "how is interaction flow packaged?",
            AssistantSurfaceScope::interaction_default(),
        );
        assert!(snap.is_non_executing());
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
        assert!(snap.validate().is_ok());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let left = package_none("same ask", AssistantSurfaceScope::interaction_default());
        let right = package_none("same ask", AssistantSurfaceScope::interaction_default());
        assert_eq!(left.interaction_id, right.interaction_id);
        assert_eq!(left.steps, right.steps);
        assert_eq!(left.request, right.request);
        assert_eq!(left.route, right.route);
    }

    #[test]
    fn unavailable_packages_are_preserved() {
        let snap = package_none("ask", AssistantSurfaceScope::interaction_default());
        assert!(snap.gaps.iter().any(|g| {
            g.gap_kind == AssistantInteractionGap::KIND_UNAVAILABLE_PACKAGE
                || g.gap_kind == AssistantInteractionGap::KIND_MISSING_CONTEXT
                || g.gap_kind == AssistantInteractionGap::KIND_MISSING_SURFACE
                || g.gap_kind == AssistantInteractionGap::KIND_MISSING_RETRIEVAL
                || g.gap_kind == AssistantInteractionGap::KIND_MISSING_EXPLANATION
        }));
        assert!(!snap.diagnostics.unavailable_packages.is_empty());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantInteractionGap::KIND_INCOMPLETE_FLOW));
    }

    #[test]
    fn empty_ask_records_gap() {
        let snap = package_none("  ", AssistantSurfaceScope::interaction_default());
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantInteractionGap::KIND_EMPTY_ASK));
        assert_eq!(snap.request.human_ask, "  ");
    }

    #[test]
    fn forbidden_agency_language_is_rejected() {
        assert!(reject_forbidden("i will handle").is_err());
        assert!(reject_forbidden("let me handle").is_err());
        assert!(reject_forbidden("i will run").is_err());
        assert!(reject_forbidden("acting on your behalf").is_err());
        assert!(reject_forbidden("i commit").is_err());
        assert!(reject_forbidden("you should").is_err());
        assert!(reject_forbidden("approve").is_err());
        assert!(reject_forbidden("execute now").is_err());
        assert!(reject_forbidden("i decided").is_err());
        assert!(reject_forbidden("your goal is").is_err());
        assert!(reject_forbidden("trust me").is_err());
        assert!(reject_forbidden("decision is").is_err());
        assert!(reject_forbidden("i remember that you").is_err());
        assert!(reject_forbidden("autonomous").is_err());
        assert!(reject_forbidden("coordinates conversation flow packaging").is_ok());
        assert!(reject_forbidden("never act for the user").is_ok());
    }

    #[test]
    fn no_invented_package_refs_when_upstreams_missing() {
        let snap = package_none("ask", AssistantSurfaceScope::interaction_default());
        assert!(snap.route.routed_packages.is_empty());
        for step in &snap.steps {
            if step.kind != step_kind::GAP {
                panic!("must not invent non-gap steps without package refs");
            }
            assert!(
                step.package_ref.is_none(),
                "must not invent package refs: {:?}",
                step.package_ref
            );
        }
        assert!(snap.narrative.contains("never acts for the user"));
    }

    #[test]
    fn projection_is_non_commandable() {
        let projection = WorkspaceAssistantInteractionProjection::assemble(
            "ws",
            Some(package_none(
                "ask",
                AssistantSurfaceScope::interaction_default(),
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
    fn steps_sequence_is_stable_and_deterministic() {
        let snap = package_none("ask", AssistantSurfaceScope::interaction_default());
        // Empty-ask not present; required packages missing → gap steps at seq 1..4
        let gap_steps: Vec<_> = snap
            .steps
            .iter()
            .filter(|s| s.kind == step_kind::GAP && s.origin_domain.starts_with("workspace_"))
            .collect();
        assert_eq!(gap_steps.len(), 4);
        assert_eq!(gap_steps[0].origin_domain, PACKAGE_CONTEXT);
        assert_eq!(gap_steps[0].sequence, 1);
        assert_eq!(gap_steps[1].origin_domain, PACKAGE_RETRIEVAL);
        assert_eq!(gap_steps[1].sequence, 2);
        assert_eq!(gap_steps[2].origin_domain, PACKAGE_EXPLANATION);
        assert_eq!(gap_steps[2].sequence, 3);
        assert_eq!(gap_steps[3].origin_domain, PACKAGE_SURFACE);
        assert_eq!(gap_steps[3].sequence, 4);
        // Sequences are non-decreasing.
        for window in snap.steps.windows(2) {
            assert!(window[0].sequence <= window[1].sequence);
        }
    }
}
