//! Workspace Working Style Model — observable operating patterns (Phase 6).
//!
//! Answers: how does work usually happen in this Workspace?
//! Projection only — not profiling, surveillance, prediction, or autonomous learning.
//! Separates Observed Behaviour from Explicit Preference. Never treats observation as intent.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Working Style validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceWorkingStyleError {
    #[error("working style requires a workspace id")]
    MissingWorkspace,

    #[error("working style cannot execute, plan, learn autonomously, or authorize")]
    CannotExecute,

    #[error("working style validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Category of working-style observation (informational only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkingStyleKind {
    Rhythm,
    Organization,
    Workflow,
    InteractionPreference,
    ObservedUsage,
    ContextSwitching,
}

impl WorkingStyleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rhythm => "rhythm",
            Self::Organization => "organization",
            Self::Workflow => "workflow",
            Self::InteractionPreference => "interaction_preference",
            Self::ObservedUsage => "observed_usage",
            Self::ContextSwitching => "context_switching",
        }
    }
}

/// Critical distinction: observed behaviour vs explicit preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkingStyleOrigin {
    ObservedBehaviour,
    ExplicitPreference,
}

impl WorkingStyleOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObservedBehaviour => "observed_behaviour",
            Self::ExplicitPreference => "explicit_preference",
        }
    }
}

/// Display-only consistency hint (not an opaque score or prediction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkingStyleConfidence {
    High,
    Medium,
    Low,
}

impl WorkingStyleConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Evidence grounding an observation in an existing projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingStyleEvidence {
    pub label: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// One explainable working-style observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingStyleObservation {
    pub id: String,
    pub kind: WorkingStyleKind,
    pub origin: WorkingStyleOrigin,
    pub title: String,
    pub summary: String,
    pub confidence: WorkingStyleConfidence,
    pub evidence: Vec<WorkingStyleEvidence>,
    pub affected_context: String,
    pub explanation: String,
    pub why: String,
    pub authority_effect: String,
}

impl WorkingStyleObservation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Human-facing working style summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingStyleSummary {
    pub headline: String,
    pub rhythm_line: String,
    pub organization_line: String,
    pub workflow_line: String,
    pub context_switching_line: String,
    pub preference_line: String,
    pub observed_vs_preferred_line: String,
    pub narrative: String,
}

/// Full Working Style snapshot — aggregation only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkingStyleState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub style_summary: WorkingStyleSummary,
    pub observations: Vec<WorkingStyleObservation>,
    pub observation_count: usize,
    pub observed_count: usize,
    pub preference_count: usize,
    pub rhythm_count: usize,
    pub organization_count: usize,
    pub workflow_count: usize,
    pub context_switching_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub work_context_generated_at: String,
    pub navigation_generated_at: String,
    pub milestones_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceWorkingStyleState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceWorkingStyleSummary {
        WorkspaceWorkingStyleSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            style_summary: self.style_summary.clone(),
            observation_count: self.observation_count,
            observed_count: self.observed_count,
            preference_count: self.preference_count,
            top_observations: self.observations.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkingStyleSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub style_summary: WorkingStyleSummary,
    pub observation_count: usize,
    pub observed_count: usize,
    pub preference_count: usize,
    pub top_observations: Vec<WorkingStyleObservation>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceWorkingStyleSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            style_summary: WorkingStyleSummary {
                headline: String::new(),
                rhythm_line: String::new(),
                organization_line: String::new(),
                workflow_line: String::new(),
                context_switching_line: String::new(),
                preference_line: String::new(),
                observed_vs_preferred_line: String::new(),
                narrative: String::new(),
            },
            observation_count: 0,
            observed_count: 0,
            preference_count: 0,
            top_observations: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkingStyleComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceWorkingStyleComparison {
    pub fn compare(left: &WorkspaceWorkingStyleState, right: &WorkspaceWorkingStyleState) -> Self {
        let mut differences = Vec::new();
        if left.observation_count != right.observation_count {
            differences.push(format!(
                "observation_count: {} → {}",
                left.observation_count, right.observation_count
            ));
        }
        if left.observed_count != right.observed_count {
            differences.push(format!(
                "observed_count: {} → {}",
                left.observed_count, right.observed_count
            ));
        }
        if left.preference_count != right.preference_count {
            differences.push(format!(
                "preference_count: {} → {}",
                left.preference_count, right.preference_count
            ));
        }
        if left.style_summary.rhythm_line != right.style_summary.rhythm_line {
            differences.push("rhythm_line differs".into());
        }
        if left.style_summary.workflow_line != right.style_summary.workflow_line {
            differences.push("workflow_line differs".into());
        }
        if left.style_summary.observed_vs_preferred_line
            != right.style_summary.observed_vs_preferred_line
        {
            differences.push("observed_vs_preferred_line differs".into());
        }
        if left.summary != right.summary {
            differences.push("summary differs".into());
        }
        if differences.is_empty() {
            differences.push("no material working style differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkingStyleValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceWorkingStyleValidation {
    pub fn validate(state: &WorkspaceWorkingStyleState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        for obs in &state.observations {
            if obs.why.is_empty() {
                messages.push(format!("observation {} missing why", obs.id));
            }
            if obs.explanation.is_empty() {
                messages.push(format!("observation {} missing explanation", obs.id));
            }
            if obs.evidence.is_empty() {
                messages.push(format!("observation {} missing evidence", obs.id));
            }
            if obs.affected_context.is_empty() {
                messages.push(format!("observation {} missing affected_context", obs.id));
            }
            if obs.authority_effect != WorkingStyleObservation::AUTHORITY_EFFECT_NONE {
                messages.push(format!("observation {} has authority", obs.id));
            }
            for ev in &obs.evidence {
                if ev.why.is_empty() || ev.source_projection.is_empty() {
                    messages.push(format!("observation {} incomplete evidence", obs.id));
                }
            }
            // Explicit preferences must be labelled separately from observed behaviour.
            if obs.kind == WorkingStyleKind::InteractionPreference
                && obs.origin != WorkingStyleOrigin::ExplicitPreference
            {
                messages.push(format!(
                    "observation {} interaction preference must be explicit_preference",
                    obs.id
                ));
            }
            if obs.origin == WorkingStyleOrigin::ObservedBehaviour
                && obs.kind == WorkingStyleKind::InteractionPreference
            {
                messages.push(format!(
                    "observation {} cannot mix observed origin with preference kind",
                    obs.id
                ));
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} observation(s) · {} observed · {} explicit preference(s)",
                state.observation_count, state.observed_count, state.preference_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// RFC3339 timestamp for Working Style snapshots.
pub fn working_style_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Validate workspace id for Working Style projection.
pub fn validate_working_style_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceWorkingStyleError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceWorkingStyleError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceWorkingStyleError::Domain)
}

/// Compact one-line summary for audits / Intelligence.
pub fn build_working_style_summary(
    label: &str,
    observation_count: usize,
    preference_count: usize,
) -> String {
    format!(
        "Working style for \"{label}\": {observation_count} observation(s), \
         {preference_count} explicit preference(s). Observational only — never profiles or executes."
    )
}
