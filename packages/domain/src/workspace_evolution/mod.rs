//! Workspace Evolution Model — how work changes over time (Phase 5).
//!
//! Aggregates Activity Graph, Task Graph, Purpose, Composition, Continuity, and
//! Decision Queue into an explainable change narrative. Informational only —
//! never executes, persists evolution rows, or predicts the future.
//! Activity Graph remains the history SoT.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Evolution-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceEvolutionError {
    #[error("evolution model requires a workspace id")]
    MissingWorkspace,

    #[error("evolution model cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Deterministic insight kinds (rules — not ML or prediction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionInsightKind {
    TaskProgression,
    PurposeProgression,
    CompositionShift,
    InterruptedWork,
    DecisionOutcome,
    FocusChange,
}

impl EvolutionInsightKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaskProgression => "task_progression",
            Self::PurposeProgression => "purpose_progression",
            Self::CompositionShift => "composition_shift",
            Self::InterruptedWork => "interrupted_work",
            Self::DecisionOutcome => "decision_outcome",
            Self::FocusChange => "focus_change",
        }
    }
}

/// Source model an evolution event was projected from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionSourceModel {
    Activity,
    TaskGraph,
    Purpose,
    Composition,
    Continuity,
    DecisionQueue,
}

impl EvolutionSourceModel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Activity => "activity",
            Self::TaskGraph => "task_graph",
            Self::Purpose => "purpose",
            Self::Composition => "composition",
            Self::Continuity => "continuity",
            Self::DecisionQueue => "decision_queue",
        }
    }
}

/// One projected change event (not a durable history row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionEvent {
    pub id: String,
    pub kind: String,
    pub ref_id: String,
    pub source_model: EvolutionSourceModel,
    pub title: String,
    /// What changed?
    pub change: String,
    /// Which existing events support this?
    pub evidence: Vec<String>,
    /// What does this mean now?
    pub impact: String,
    pub timestamp: String,
    pub authority_effect: String,
}

impl EvolutionEvent {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Explainable link between evolution events / source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionRelationship {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub explanation: String,
    pub evidence: Vec<String>,
}

/// Deterministic narrative insight about how work evolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionInsight {
    pub id: String,
    pub kind: EvolutionInsightKind,
    pub title: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub related_event_ids: Vec<String>,
}

/// Full Evolution Model snapshot for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvolutionState {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub events: Vec<EvolutionEvent>,
    pub insights: Vec<EvolutionInsight>,
    pub relationships: Vec<EvolutionRelationship>,
    pub event_count: usize,
    pub insight_count: usize,
    pub relationship_count: usize,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceEvolutionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceEvolutionSummary {
        WorkspaceEvolutionSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            event_count: self.event_count,
            insight_count: self.insight_count,
            relationship_count: self.relationship_count,
            top_events: self.events.iter().rev().take(limit).cloned().collect(),
            top_insights: self.insights.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEvolutionSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub label: String,
    pub event_count: usize,
    pub insight_count: usize,
    pub relationship_count: usize,
    pub top_events: Vec<EvolutionEvent>,
    pub top_insights: Vec<EvolutionInsight>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceEvolutionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            label: String::new(),
            event_count: 0,
            insight_count: 0,
            relationship_count: 0,
            top_events: Vec::new(),
            top_insights: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceEvolutionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Deterministic product summary for evolution.
pub fn build_evolution_summary(label: &str, events: usize, insights: usize) -> String {
    format!(
        "Evolution of \"{label}\" — {events} change event(s), {insights} insight(s). \
         Projected from existing history; no prediction or new storage."
    )
}

pub fn evolution_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_evolution_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceEvolutionError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
