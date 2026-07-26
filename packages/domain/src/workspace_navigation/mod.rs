//! Workspace Navigation Engine — interaction model over existing understanding (Phase 6).
//!
//! Guides users through existing work. Owns no data, planning, or authority.
//! Deterministic paths only — not AI routing, not execution.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Navigation-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceNavigationError {
    #[error("navigation requires a workspace id")]
    MissingWorkspace,

    #[error("navigation cannot execute, plan, route, restore, or authorize")]
    CannotExecute,

    #[error("navigation validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of navigable destination (presentation / inspection only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationPathKind {
    CurrentFocus,
    SuggestedDestination,
    RelatedWork,
    BlockingItem,
    ConnectedTask,
    ConnectedProject,
    ConnectedContext,
    RelevantDecision,
    RelevantRecommendation,
    RecentChange,
    PossibleNextInspection,
    DependencyChain,
    Breadcrumb,
}

impl NavigationPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentFocus => "current_focus",
            Self::SuggestedDestination => "suggested_destination",
            Self::RelatedWork => "related_work",
            Self::BlockingItem => "blocking_item",
            Self::ConnectedTask => "connected_task",
            Self::ConnectedProject => "connected_project",
            Self::ConnectedContext => "connected_context",
            Self::RelevantDecision => "relevant_decision",
            Self::RelevantRecommendation => "relevant_recommendation",
            Self::RecentChange => "recent_change",
            Self::PossibleNextInspection => "possible_next_inspection",
            Self::DependencyChain => "dependency_chain",
            Self::Breadcrumb => "breadcrumb",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::CurrentFocus => "Current Focus",
            Self::SuggestedDestination => "Suggested Destination",
            Self::RelatedWork => "Related Work",
            Self::BlockingItem => "Blocking Items",
            Self::ConnectedTask => "Connected Tasks",
            Self::ConnectedProject => "Connected Projects",
            Self::ConnectedContext => "Connected Contexts",
            Self::RelevantDecision => "Relevant Decisions",
            Self::RelevantRecommendation => "Relevant Recommendations",
            Self::RecentChange => "Recent Changes",
            Self::PossibleNextInspection => "Possible Next Inspection",
            Self::DependencyChain => "Dependency Chain",
            Self::Breadcrumb => "Navigation Breadcrumbs",
        }
    }
}

/// Informational navigation relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationRelationKind {
    Current,
    Related,
    DependsOn,
    BlockedBy,
    Supports,
    LeadsTo,
    RecentlyVisited,
    SuggestedNext,
    Dormant,
    Disconnected,
}

impl NavigationRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Related => "related",
            Self::DependsOn => "depends_on",
            Self::BlockedBy => "blocked_by",
            Self::Supports => "supports",
            Self::LeadsTo => "leads_to",
            Self::RecentlyVisited => "recently_visited",
            Self::SuggestedNext => "suggested_next",
            Self::Dormant => "dormant",
            Self::Disconnected => "disconnected",
        }
    }
}

/// One explainable navigation stop / destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationNode {
    pub id: String,
    pub label: String,
    pub kind: NavigationPathKind,
    pub summary: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
    pub authority_effect: String,
}

impl NavigationNode {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Relationship between two navigation nodes (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationEdge {
    pub id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub kind: NavigationRelationKind,
    pub why: String,
    pub authority_effect: String,
}

/// Grouped navigation path for UI sections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationPath {
    pub kind: NavigationPathKind,
    pub title: String,
    pub nodes: Vec<NavigationNode>,
    pub node_count: usize,
    pub why: String,
}

/// Human-facing navigation summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationSummary {
    pub headline: String,
    pub current_path_line: String,
    pub related_line: String,
    pub blocked_line: String,
    pub next_inspection_line: String,
    pub breadcrumb_line: String,
    pub narrative: String,
}

/// Full Navigation Engine snapshot — interaction projection only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceNavigationState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub navigation_summary: NavigationSummary,
    pub paths: Vec<NavigationPath>,
    pub nodes: Vec<NavigationNode>,
    pub edges: Vec<NavigationEdge>,
    pub breadcrumbs: Vec<NavigationNode>,
    pub path_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub blocked_count: usize,
    pub suggested_count: usize,
    pub session_generated_at: String,
    pub experience_generated_at: String,
    pub work_context_generated_at: String,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceNavigationState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceNavigationSummary {
        WorkspaceNavigationSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            navigation_summary: self.navigation_summary.clone(),
            path_count: self.path_count,
            node_count: self.node_count,
            edge_count: self.edge_count,
            blocked_count: self.blocked_count,
            suggested_count: self.suggested_count,
            top_paths: self.paths.iter().take(limit).cloned().collect(),
            breadcrumbs: self.breadcrumbs.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceNavigationSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub navigation_summary: NavigationSummary,
    pub path_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub blocked_count: usize,
    pub suggested_count: usize,
    pub top_paths: Vec<NavigationPath>,
    pub breadcrumbs: Vec<NavigationNode>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceNavigationSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            navigation_summary: NavigationSummary {
                headline: String::new(),
                current_path_line: String::new(),
                related_line: String::new(),
                blocked_line: String::new(),
                next_inspection_line: String::new(),
                breadcrumb_line: String::new(),
                narrative: String::new(),
            },
            path_count: 0,
            node_count: 0,
            edge_count: 0,
            blocked_count: 0,
            suggested_count: 0,
            top_paths: Vec::new(),
            breadcrumbs: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceNavigationState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceNavigationComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceNavigationComparison {
    pub fn compare(left: &WorkspaceNavigationState, right: &WorkspaceNavigationState) -> Self {
        let mut differences = Vec::new();
        if left.navigation_summary.current_path_line != right.navigation_summary.current_path_line {
            differences.push(format!(
                "current_path: {} → {}",
                left.navigation_summary.current_path_line, right.navigation_summary.current_path_line
            ));
        }
        if left.node_count != right.node_count {
            differences.push(format!("node_count: {} → {}", left.node_count, right.node_count));
        }
        if left.blocked_count != right.blocked_count {
            differences.push(format!(
                "blocked_count: {} → {}",
                left.blocked_count, right.blocked_count
            ));
        }
        if left.suggested_count != right.suggested_count {
            differences.push(format!(
                "suggested_count: {} → {}",
                left.suggested_count, right.suggested_count
            ));
        }
        if left.breadcrumb_line_key() != right.breadcrumb_line_key() {
            differences.push("breadcrumbs differ".into());
        }
        if differences.is_empty() {
            differences.push("no material navigation differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceNavigationState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

impl WorkspaceNavigationState {
    fn breadcrumb_line_key(&self) -> String {
        self.breadcrumbs
            .iter()
            .map(|b| b.label.as_str())
            .collect::<Vec<_>>()
            .join(" > ")
    }
}

/// Validate projection invariants (Operator / IPC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceNavigationValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceNavigationValidation {
    pub fn validate(state: &WorkspaceNavigationState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceNavigationState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        if state.paths.is_empty() {
            messages.push("at least one navigation path required".into());
        }
        for node in &state.nodes {
            if node.why.is_empty() || node.source_projection.is_empty() {
                messages.push(format!("node {} incomplete explainability", node.id));
            }
            if node.authority_effect != NavigationNode::AUTHORITY_EFFECT_NONE {
                messages.push(format!("node {} has authority", node.id));
            }
        }
        for edge in &state.edges {
            if edge.why.is_empty() {
                messages.push(format!("edge {} missing why", edge.id));
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} path(s) · {} node(s) · blocked {}",
                state.path_count, state.node_count, state.blocked_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceNavigationState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn navigation_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_navigation_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceNavigationError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceNavigationError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceNavigationError::Domain)
}

pub fn build_navigation_summary(label: &str, node_count: usize, blocked_count: usize) -> String {
    format!(
        "Navigation for \"{label}\": {node_count} stop(s); {blocked_count} blocked path signal(s)."
    )
}
