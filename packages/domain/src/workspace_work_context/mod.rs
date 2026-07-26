//! Workspace Work Context Engine — semantic projection of “what kind of work” (Phase 6).
//!
//! Consumes existing Session, Experience, and Intelligence pipeline outputs only.
//! Owns no data, planning, or authority. Classifications are deterministic — not AI inference.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Work Context validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceWorkContextError {
    #[error("work context requires a workspace id")]
    MissingWorkspace,

    #[error("work context cannot execute, plan, restore, or authorize")]
    CannotExecute,

    #[error("work context validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Classification only — never an execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkContextType {
    Development,
    Research,
    Administration,
    Communication,
    Creative,
    Learning,
    Operations,
    Planning,
    Custom,
}

impl WorkContextType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Research => "research",
            Self::Administration => "administration",
            Self::Communication => "communication",
            Self::Creative => "creative",
            Self::Learning => "learning",
            Self::Operations => "operations",
            Self::Planning => "planning",
            Self::Custom => "custom",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Research => "Research",
            Self::Administration => "Administration",
            Self::Communication => "Communication",
            Self::Creative => "Creative",
            Self::Learning => "Learning",
            Self::Operations => "Operations",
            Self::Planning => "Planning",
            Self::Custom => "Custom",
        }
    }
}

/// Informational relationship between contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkContextRelationKind {
    Primary,
    Supporting,
    Nested,
    RecentlyActive,
    Interrupted,
    Dormant,
    Candidate,
}

impl WorkContextRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Supporting => "supporting",
            Self::Nested => "nested",
            Self::RecentlyActive => "recently_active",
            Self::Interrupted => "interrupted",
            Self::Dormant => "dormant",
            Self::Candidate => "candidate",
        }
    }
}

/// Current status of a recognized context (informational).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkContextStatus {
    Active,
    Interrupted,
    Blocked,
    Dormant,
    Candidate,
}

impl WorkContextStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Interrupted => "interrupted",
            Self::Blocked => "blocked",
            Self::Dormant => "dormant",
            Self::Candidate => "candidate",
        }
    }
}

/// Deterministic confidence band from evidence count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkContextConfidence {
    High,
    Medium,
    Low,
}

impl WorkContextConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn from_evidence_count(count: usize) -> Self {
        match count {
            0 | 1 => Self::Low,
            2 => Self::Medium,
            _ => Self::High,
        }
    }
}

/// One explainable evidence pointer into an existing projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkContextEvidence {
    pub label: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// Association pointer (project/task/app/purpose/decision/activity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkContextAssociation {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub source_projection: String,
    pub source_ref: String,
    pub why: String,
}

/// One coherent semantic work context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkContext {
    pub id: String,
    pub name: String,
    pub context_type: WorkContextType,
    pub evidence: Vec<WorkContextEvidence>,
    pub confidence: WorkContextConfidence,
    pub associated_projects: Vec<WorkContextAssociation>,
    pub associated_tasks: Vec<WorkContextAssociation>,
    pub associated_applications: Vec<WorkContextAssociation>,
    pub associated_purpose: Option<WorkContextAssociation>,
    pub associated_decisions: Vec<WorkContextAssociation>,
    pub associated_activity: Vec<WorkContextAssociation>,
    pub current_status: WorkContextStatus,
    pub suggested_focus: String,
    pub blocked_reasons: Vec<String>,
    pub recent_progress: Vec<String>,
    pub why: String,
    pub authority_effect: String,
}

impl WorkContext {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Relationship between two contexts (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkContextRelationship {
    pub from_context_id: String,
    pub to_context_id: String,
    pub kind: WorkContextRelationKind,
    pub why: String,
    pub authority_effect: String,
}

/// Full Work Context Engine snapshot — projection only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkContextState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub contexts: Vec<WorkContext>,
    pub primary_context_id: Option<String>,
    pub relationships: Vec<WorkContextRelationship>,
    pub context_count: usize,
    pub active_count: usize,
    pub blocked_count: usize,
    pub dormant_count: usize,
    pub candidate_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceWorkContextState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn primary_context(&self) -> Option<&WorkContext> {
        let id = self.primary_context_id.as_ref()?;
        self.contexts.iter().find(|c| &c.id == id)
    }

    pub fn summary_projection(&self, limit: usize) -> WorkspaceWorkContextSummary {
        WorkspaceWorkContextSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            context_count: self.context_count,
            active_count: self.active_count,
            blocked_count: self.blocked_count,
            dormant_count: self.dormant_count,
            candidate_count: self.candidate_count,
            primary_context_name: self.primary_context().map(|c| c.name.clone()),
            primary_context_type: self
                .primary_context()
                .map(|c| c.context_type.as_str().to_string()),
            top_contexts: self.contexts.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkContextSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub context_count: usize,
    pub active_count: usize,
    pub blocked_count: usize,
    pub dormant_count: usize,
    pub candidate_count: usize,
    pub primary_context_name: Option<String>,
    pub primary_context_type: Option<String>,
    pub top_contexts: Vec<WorkContext>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceWorkContextSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            context_count: 0,
            active_count: 0,
            blocked_count: 0,
            dormant_count: 0,
            candidate_count: 0,
            primary_context_name: None,
            primary_context_type: None,
            top_contexts: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkContextComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceWorkContextComparison {
    pub fn compare(left: &WorkspaceWorkContextState, right: &WorkspaceWorkContextState) -> Self {
        let mut differences = Vec::new();
        if left.primary_context_id != right.primary_context_id {
            differences.push(format!(
                "primary_context: {:?} → {:?}",
                left.primary_context().map(|c| &c.name),
                right.primary_context().map(|c| &c.name)
            ));
        }
        if left.context_count != right.context_count {
            differences.push(format!(
                "context_count: {} → {}",
                left.context_count, right.context_count
            ));
        }
        if left.active_count != right.active_count {
            differences.push(format!(
                "active_count: {} → {}",
                left.active_count, right.active_count
            ));
        }
        if left.blocked_count != right.blocked_count {
            differences.push(format!(
                "blocked_count: {} → {}",
                left.blocked_count, right.blocked_count
            ));
        }
        let left_types: Vec<_> = left.contexts.iter().map(|c| c.context_type.as_str()).collect();
        let right_types: Vec<_> = right.contexts.iter().map(|c| c.context_type.as_str()).collect();
        if left_types != right_types {
            differences.push(format!("context_types: {left_types:?} → {right_types:?}"));
        }
        if differences.is_empty() {
            differences.push("no material work-context differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceWorkContextValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceWorkContextValidation {
    pub fn validate(state: &WorkspaceWorkContextState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        if state.contexts.is_empty() {
            messages.push("at least one context required".into());
        }
        for context in &state.contexts {
            if context.why.is_empty() {
                messages.push(format!("context {} missing why", context.id));
            }
            if context.authority_effect != WorkContext::AUTHORITY_EFFECT_NONE {
                messages.push(format!("context {} has authority", context.id));
            }
            if context.evidence.is_empty() {
                messages.push(format!("context {} missing evidence", context.id));
            }
            for ev in &context.evidence {
                if ev.why.is_empty() || ev.source_projection.is_empty() {
                    messages.push(format!("context {} has incomplete evidence", context.id));
                }
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} context(s) · primary {:?}",
                state.context_count,
                state.primary_context().map(|c| &c.name)
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn work_context_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_work_context_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceWorkContextError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceWorkContextError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(|e| WorkspaceWorkContextError::Domain(e))
}

pub fn build_work_context_summary(label: &str, context_count: usize, primary: Option<&str>) -> String {
    match primary {
        Some(name) => format!(
            "Work Context for \"{label}\": {context_count} context(s); primary is {name}."
        ),
        None => format!("Work Context for \"{label}\": {context_count} context(s); no primary."),
    }
}
