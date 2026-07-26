//! Workspace Interaction Model — unified interaction opportunities (Phase 6).
//!
//! Answers: what meaningful things can the user interact with right now?
//! Projection only — not a planner, executor, recommendation engine, or automation system.
//! Owns nothing. Selecting an interaction creates an Intent handoff only — never executes.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Interaction-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceInteractionError {
    #[error("interactions require a workspace id")]
    MissingWorkspace,

    #[error("interactions cannot execute, automate, or authorize")]
    CannotExecute,

    #[error("interaction not found: {0}")]
    NotFound(String),

    #[error("interaction validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of user-facing interaction opportunity (informational).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionKind {
    ContinueWork,
    ReviewDecision,
    InspectRecommendation,
    ReviewAdaptation,
    ResolveBlocker,
    OpenContext,
    ReviewProgress,
    UnderstandChange,
}

impl InteractionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContinueWork => "continue_work",
            Self::ReviewDecision => "review_decision",
            Self::InspectRecommendation => "inspect_recommendation",
            Self::ReviewAdaptation => "review_adaptation",
            Self::ResolveBlocker => "resolve_blocker",
            Self::OpenContext => "open_context",
            Self::ReviewProgress => "review_progress",
            Self::UnderstandChange => "understand_change",
        }
    }
}

/// Lifecycle of an interaction candidate (never execution state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionItemState {
    Available,
    Selected,
    HandedOff,
}

impl InteractionItemState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Selected => "selected",
            Self::HandedOff => "handed_off",
        }
    }
}

/// Display-only priority band (not an opaque score).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionPriority {
    High,
    Medium,
    Low,
}

impl InteractionPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Evidence grounding an interaction in an existing projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionEvidence {
    pub label: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// One user-facing interaction opportunity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionItem {
    pub id: String,
    pub kind: InteractionKind,
    pub source_projection: String,
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub available_action: String,
    pub required_intent: String,
    pub state: InteractionItemState,
    pub priority: InteractionPriority,
    pub evidence: Vec<InteractionEvidence>,
    pub why: String,
    pub authority_effect: String,
}

impl InteractionItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Human-facing interaction summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionSummary {
    pub headline: String,
    pub continue_line: String,
    pub decision_line: String,
    pub recommendation_line: String,
    pub adaptation_line: String,
    pub blocker_line: String,
    pub progress_line: String,
    pub narrative: String,
}

/// Full Interaction Model snapshot — opportunities only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceInteractionState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub interaction_summary: InteractionSummary,
    pub items: Vec<InteractionItem>,
    pub item_count: usize,
    pub continue_count: usize,
    pub decision_count: usize,
    pub recommendation_count: usize,
    pub adaptation_count: usize,
    pub blocker_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub transition_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceInteractionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn find_item(&self, id: &str) -> Option<&InteractionItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn summary_projection(&self, limit: usize) -> WorkspaceInteractionSummary {
        WorkspaceInteractionSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            interaction_summary: self.interaction_summary.clone(),
            item_count: self.item_count,
            continue_count: self.continue_count,
            decision_count: self.decision_count,
            recommendation_count: self.recommendation_count,
            adaptation_count: self.adaptation_count,
            blocker_count: self.blocker_count,
            top_items: self.items.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceInteractionSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub interaction_summary: InteractionSummary,
    pub item_count: usize,
    pub continue_count: usize,
    pub decision_count: usize,
    pub recommendation_count: usize,
    pub adaptation_count: usize,
    pub blocker_count: usize,
    pub top_items: Vec<InteractionItem>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceInteractionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            interaction_summary: InteractionSummary {
                headline: String::new(),
                continue_line: String::new(),
                decision_line: String::new(),
                recommendation_line: String::new(),
                adaptation_line: String::new(),
                blocker_line: String::new(),
                progress_line: String::new(),
                narrative: String::new(),
            },
            item_count: 0,
            continue_count: 0,
            decision_count: 0,
            recommendation_count: 0,
            adaptation_count: 0,
            blocker_count: 0,
            top_items: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceInteractionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Intent handoff after selecting an interaction — caller must invoke Intent/Gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionHandoff {
    pub interaction_id: String,
    pub kind: InteractionKind,
    pub next_command: String,
    pub intent_statement: String,
    pub workspace_id: String,
    pub note: String,
    pub authority_effect: String,
}

impl InteractionHandoff {
    pub const NEXT_SUBMIT_ASSISTANT_GOAL: &'static str = "submit_assistant_goal";
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Result of selecting an interaction (handoff only — never executes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionSelectResult {
    pub item: Option<InteractionItem>,
    pub handoff: Option<InteractionHandoff>,
    pub authority_effect: String,
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceInteractionComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceInteractionComparison {
    pub fn compare(left: &WorkspaceInteractionState, right: &WorkspaceInteractionState) -> Self {
        let mut differences = Vec::new();
        if left.item_count != right.item_count {
            differences.push(format!(
                "item_count: {} → {}",
                left.item_count, right.item_count
            ));
        }
        if left.decision_count != right.decision_count {
            differences.push(format!(
                "decision_count: {} → {}",
                left.decision_count, right.decision_count
            ));
        }
        if left.recommendation_count != right.recommendation_count {
            differences.push(format!(
                "recommendation_count: {} → {}",
                left.recommendation_count, right.recommendation_count
            ));
        }
        if left.blocker_count != right.blocker_count {
            differences.push(format!(
                "blocker_count: {} → {}",
                left.blocker_count, right.blocker_count
            ));
        }
        if left.interaction_summary.continue_line != right.interaction_summary.continue_line {
            differences.push("continue_line differs".into());
        }
        if left.summary != right.summary {
            differences.push("summary differs".into());
        }
        if differences.is_empty() {
            differences.push("no material interaction differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceInteractionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceInteractionValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceInteractionValidation {
    pub fn validate(state: &WorkspaceInteractionState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceInteractionState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        for item in &state.items {
            if item.why.is_empty() {
                messages.push(format!("item {} missing why", item.id));
            }
            if item.explanation.is_empty() {
                messages.push(format!("item {} missing explanation", item.id));
            }
            if item.evidence.is_empty() {
                messages.push(format!("item {} missing evidence", item.id));
            }
            if item.required_intent.is_empty() {
                messages.push(format!("item {} missing required_intent", item.id));
            }
            if item.source_projection.is_empty() {
                messages.push(format!("item {} missing source_projection", item.id));
            }
            if item.authority_effect != InteractionItem::AUTHORITY_EFFECT_NONE {
                messages.push(format!("item {} has authority", item.id));
            }
            for ev in &item.evidence {
                if ev.why.is_empty() || ev.source_projection.is_empty() {
                    messages.push(format!("item {} incomplete evidence", item.id));
                }
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} interaction(s) · decisions {} · recommendations {} — \
                 opportunities only, never executes",
                state.item_count, state.decision_count, state.recommendation_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceInteractionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// RFC3339 timestamp for Interaction snapshots.
pub fn interaction_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Validate workspace id for Interaction projection.
pub fn validate_interaction_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceInteractionError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceInteractionError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceInteractionError::Domain)
}

/// Compact one-line summary for audits / Intelligence.
pub fn build_interaction_summary(label: &str, count: usize) -> String {
    format!(
        "Interactions for \"{label}\": {count} opportunity(ies). \
         Unified surface over existing understanding — never executes."
    )
}
