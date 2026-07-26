//! Workspace Transition Engine — semantic movement between work states (Phase 6).
//!
//! Answers: where was I, what am I entering, what changed, what transition is occurring?
//! Projection only — not restoration, automation, scheduling, or execution.
//! Compares and explains state changes; never performs them.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Transition-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceTransitionError {
    #[error("transitions require a workspace id")]
    MissingWorkspace,

    #[error("transitions cannot execute, restore, automate, schedule, or authorize")]
    CannotExecute,

    #[error("transition validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of explained transition (informational only — never performed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionKind {
    EnteringContext,
    LeavingContext,
    ReturningToWork,
    SwitchingFocus,
    ContinuingInterruptedWork,
    CompletingWorkState,
    StartingNewWorkState,
}

impl TransitionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnteringContext => "entering_context",
            Self::LeavingContext => "leaving_context",
            Self::ReturningToWork => "returning_to_work",
            Self::SwitchingFocus => "switching_focus",
            Self::ContinuingInterruptedWork => "continuing_interrupted_work",
            Self::CompletingWorkState => "completing_work_state",
            Self::StartingNewWorkState => "starting_new_work_state",
        }
    }
}

/// Informational relationship between transition endpoints / supporting work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionRelationKind {
    Previous,
    Current,
    ReturnedFrom,
    InterruptedBy,
    ContinuedInto,
    RelatedTo,
    BlockedBy,
}

impl TransitionRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Previous => "previous",
            Self::Current => "current",
            Self::ReturnedFrom => "returned_from",
            Self::InterruptedBy => "interrupted_by",
            Self::ContinuedInto => "continued_into",
            Self::RelatedTo => "related_to",
            Self::BlockedBy => "blocked_by",
        }
    }
}

/// Display-only consistency hint (not a forecast).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionConfidence {
    High,
    Medium,
    Low,
}

impl TransitionConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Evidence grounding a transition in an existing projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionEvidence {
    pub label: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// Association pointer into existing projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionAssociation {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// One explainable transition between work states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTransition {
    pub id: String,
    pub kind: TransitionKind,
    pub title: String,
    pub previous_state: String,
    pub current_state: String,
    pub changed_elements: Vec<String>,
    pub evidence: Vec<TransitionEvidence>,
    pub related_context: Vec<TransitionAssociation>,
    pub related_milestones: Vec<TransitionAssociation>,
    pub open_decisions: Vec<TransitionAssociation>,
    pub interrupted_work: Vec<TransitionAssociation>,
    pub confidence: TransitionConfidence,
    pub explanation: String,
    pub why: String,
    pub authority_effect: String,
}

impl WorkspaceTransition {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Relationship between transition endpoints (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionRelationship {
    pub id: String,
    pub from_transition_id: String,
    pub to_ref: String,
    pub kind: TransitionRelationKind,
    pub why: String,
    pub authority_effect: String,
}

/// Human-facing transition summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionSummary {
    pub headline: String,
    pub left_off_line: String,
    pub current_transition_line: String,
    pub changed_line: String,
    pub returned_line: String,
    pub context_switch_line: String,
    pub interrupted_line: String,
    pub narrative: String,
}

/// Full Transition Engine snapshot — explanation only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTransitionState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub transition_summary: TransitionSummary,
    pub transitions: Vec<WorkspaceTransition>,
    pub relationships: Vec<TransitionRelationship>,
    pub current_transition_id: Option<String>,
    pub transition_count: usize,
    pub returning_count: usize,
    pub switching_count: usize,
    pub interrupted_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub work_context_generated_at: String,
    pub navigation_generated_at: String,
    pub milestones_generated_at: String,
    pub working_style_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceTransitionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn current_transition(&self) -> Option<&WorkspaceTransition> {
        let id = self.current_transition_id.as_ref()?;
        self.transitions.iter().find(|t| &t.id == id)
    }

    pub fn summary_projection(&self, limit: usize) -> WorkspaceTransitionSummary {
        WorkspaceTransitionSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            transition_summary: self.transition_summary.clone(),
            transition_count: self.transition_count,
            returning_count: self.returning_count,
            switching_count: self.switching_count,
            interrupted_count: self.interrupted_count,
            current_transition_title: self.current_transition().map(|t| t.title.clone()),
            top_transitions: self.transitions.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTransitionSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub transition_summary: TransitionSummary,
    pub transition_count: usize,
    pub returning_count: usize,
    pub switching_count: usize,
    pub interrupted_count: usize,
    pub current_transition_title: Option<String>,
    pub top_transitions: Vec<WorkspaceTransition>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceTransitionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            transition_summary: TransitionSummary {
                headline: String::new(),
                left_off_line: String::new(),
                current_transition_line: String::new(),
                changed_line: String::new(),
                returned_line: String::new(),
                context_switch_line: String::new(),
                interrupted_line: String::new(),
                narrative: String::new(),
            },
            transition_count: 0,
            returning_count: 0,
            switching_count: 0,
            interrupted_count: 0,
            current_transition_title: None,
            top_transitions: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceTransitionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTransitionComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceTransitionComparison {
    pub fn compare(left: &WorkspaceTransitionState, right: &WorkspaceTransitionState) -> Self {
        let mut differences = Vec::new();
        if left.current_transition_id != right.current_transition_id {
            differences.push(format!(
                "current_transition: {:?} → {:?}",
                left.current_transition().map(|t| &t.title),
                right.current_transition().map(|t| &t.title)
            ));
        }
        if left.transition_count != right.transition_count {
            differences.push(format!(
                "transition_count: {} → {}",
                left.transition_count, right.transition_count
            ));
        }
        if left.returning_count != right.returning_count {
            differences.push(format!(
                "returning_count: {} → {}",
                left.returning_count, right.returning_count
            ));
        }
        if left.switching_count != right.switching_count {
            differences.push(format!(
                "switching_count: {} → {}",
                left.switching_count, right.switching_count
            ));
        }
        if left.transition_summary.left_off_line != right.transition_summary.left_off_line {
            differences.push("left_off_line differs".into());
        }
        if left.transition_summary.changed_line != right.transition_summary.changed_line {
            differences.push("changed_line differs".into());
        }
        if left.summary != right.summary {
            differences.push("summary differs".into());
        }
        if differences.is_empty() {
            differences.push("no material transition differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceTransitionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTransitionValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceTransitionValidation {
    pub fn validate(state: &WorkspaceTransitionState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceTransitionState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        for transition in &state.transitions {
            if transition.why.is_empty() {
                messages.push(format!("transition {} missing why", transition.id));
            }
            if transition.explanation.is_empty() {
                messages.push(format!("transition {} missing explanation", transition.id));
            }
            if transition.previous_state.is_empty() || transition.current_state.is_empty() {
                messages.push(format!(
                    "transition {} missing previous/current state",
                    transition.id
                ));
            }
            if transition.evidence.is_empty() {
                messages.push(format!("transition {} missing evidence", transition.id));
            }
            if transition.authority_effect != WorkspaceTransition::AUTHORITY_EFFECT_NONE {
                messages.push(format!("transition {} has authority", transition.id));
            }
            for ev in &transition.evidence {
                if ev.why.is_empty() || ev.source_projection.is_empty() {
                    messages.push(format!("transition {} incomplete evidence", transition.id));
                }
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} transition(s) · returning {} · switching {} — \
                 explanation only, never restores or executes",
                state.transition_count, state.returning_count, state.switching_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceTransitionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// RFC3339 timestamp for Transition snapshots.
pub fn transition_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Validate workspace id for Transition projection.
pub fn validate_transition_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceTransitionError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceTransitionError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceTransitionError::Domain)
}

/// Compact one-line summary for audits / Intelligence.
pub fn build_transition_summary(label: &str, count: usize, current: Option<&str>) -> String {
    match current {
        Some(title) => {
            format!(
                "Transitions for \"{label}\": {count} explained movement(s); current is {title}."
            )
        }
        None => format!(
            "Transitions for \"{label}\": {count} explained movement(s); no current transition."
        ),
    }
}
