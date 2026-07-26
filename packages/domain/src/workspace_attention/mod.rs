//! Workspace Attention Engine foundation (Phase 5 Batch 2 / Sprint 125).
//!
//! A governed prioritization layer over Workspace context.
//!
//! # Facts vs inference
//!
//! **Fact inputs** (owned elsewhere): Environment (← WorkspaceState), Decision Queue,
//! Continuity, Activity Graph, Task Graph, Composition, Purpose, Evolution.
//!
//! **Inference outputs** (this model): `score`, `priority`, `urgency`, `category`,
//! ranked `top_items`. Attention never observes the desktop, never executes, and
//! never persists user decisions.
//!
//! No authority, no execution, no duplicate persistence.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{AttentionItemId, WorkspaceId};

/// Attention-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceAttentionError {
    #[error("invalid attention category: {0}")]
    InvalidCategory(String),

    #[error("invalid attention priority: {0}")]
    InvalidPriority(String),

    #[error("invalid attention state: {0}")]
    InvalidState(String),

    #[error("attention requires a workspace id")]
    MissingWorkspace,

    #[error("attention cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Where an Attention Item was projected from (not ownership).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSourceType {
    DecisionQueue,
    Continuity,
    ActivityGraph,
    WorkflowContext,
    AutomationContract,
    TaskGraph,
    Environment,
    Composition,
    Purpose,
    Evolution,
    RecommendationEngine,
    Pattern,
}

impl AttentionSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DecisionQueue => "decision_queue",
            Self::Continuity => "continuity",
            Self::ActivityGraph => "activity_graph",
            Self::WorkflowContext => "workflow_context",
            Self::AutomationContract => "automation_contract",
            Self::TaskGraph => "task_graph",
            Self::Environment => "environment",
            Self::Composition => "composition",
            Self::Purpose => "purpose",
            Self::Evolution => "evolution",
            Self::RecommendationEngine => "recommendation_engine",
            Self::Pattern => "pattern",
        }
    }
}

/// Product category for attention grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionCategory {
    RequiresDecision,
    Blocker,
    Interrupted,
    Resumable,
    Informative,
    Commitment,
    CanWait,
}

impl AttentionCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresDecision => "requires_decision",
            Self::Blocker => "blocker",
            Self::Interrupted => "interrupted",
            Self::Resumable => "resumable",
            Self::Informative => "informative",
            Self::Commitment => "commitment",
            Self::CanWait => "can_wait",
        }
    }
}

/// Deterministic priority band (not permission).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionPriority {
    Critical,
    High,
    Normal,
    Low,
}

impl AttentionPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Normal => "normal",
            Self::Low => "low",
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::Critical => 0,
            Self::High => 1,
            Self::Normal => 2,
            Self::Low => 3,
        }
    }
}

/// How soon the user should look (informational).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionUrgency {
    Immediate,
    Soon,
    Whenever,
}

impl AttentionUrgency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Soon => "soon",
            Self::Whenever => "whenever",
        }
    }
}

/// Confidence in the projection (source strength).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionConfidence {
    High,
    Medium,
    Low,
}

impl AttentionConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Presentation state for an Attention Item.
///
/// Foundation derives Visible/New from live sources. Absent sources resolve
/// naturally (item omitted). Acknowledge/Defer overlays are reserved for later
/// surfaces and are not persisted in this batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionState {
    New,
    Visible,
    Acknowledged,
    Deferred,
    Resolved,
    Expired,
}

impl AttentionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Visible => "visible",
            Self::Acknowledged => "acknowledged",
            Self::Deferred => "deferred",
            Self::Resolved => "resolved",
            Self::Expired => "expired",
        }
    }
}

/// One scored, explainable attention **inference** over an upstream fact source.
///
/// `source_type` / `source_id` point at facts owned elsewhere. `score`, `priority`,
/// `urgency`, and `category` are Attention inference — not observation facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionItem {
    pub id: AttentionItemId,
    pub workspace_id: WorkspaceId,
    pub source_type: AttentionSourceType,
    pub source_id: String,
    pub category: AttentionCategory,
    pub priority: AttentionPriority,
    pub urgency: AttentionUrgency,
    pub confidence: AttentionConfidence,
    /// Deterministic score (higher = more attention). Explainable via `score_factors`.
    pub score: u32,
    /// Human-readable factor list used to compute `score`.
    pub score_factors: Vec<String>,
    pub title: String,
    pub explanation: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub attention_state: AttentionState,
    pub authority_effect: String,
}

impl AttentionItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn synthetic_id(source_type: AttentionSourceType, source_id: &str) -> AttentionItemId {
        AttentionItemId::new(format!(
            "attention:{}:{}",
            source_type.as_str(),
            source_id
        ))
        .expect("synthetic attention id is valid")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn project(
        workspace_id: impl Into<String>,
        source_type: AttentionSourceType,
        source_id: impl Into<String>,
        category: AttentionCategory,
        priority: AttentionPriority,
        urgency: AttentionUrgency,
        confidence: AttentionConfidence,
        score: u32,
        score_factors: Vec<String>,
        title: impl Into<String>,
        explanation: impl Into<String>,
        created_at: impl Into<String>,
        attention_state: AttentionState,
    ) -> Result<Self, WorkspaceAttentionError> {
        let source_id = source_id.into();
        Ok(Self {
            id: Self::synthetic_id(source_type, &source_id),
            workspace_id: WorkspaceId::new(workspace_id)?,
            source_type,
            source_id,
            category,
            priority,
            urgency,
            confidence,
            score,
            score_factors,
            title: title.into(),
            explanation: explanation.into(),
            created_at: created_at.into(),
            expires_at: None,
            attention_state,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Full attention snapshot — ranked inference over Workspace context.
///
/// Ordering is deterministic: score DESC, priority rank ASC, id ASC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAttentionState {
    pub workspace_id: String,
    pub generated_at: String,
    pub items: Vec<AttentionItem>,
    pub top_items: Vec<AttentionItem>,
    pub requires_decision_count: usize,
    pub blocker_count: usize,
    pub informative_count: usize,
    pub can_wait_count: usize,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceAttentionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_items(workspace_id: impl Into<String>, mut items: Vec<AttentionItem>) -> Self {
        // Deterministic order: score DESC, priority rank ASC, id ASC.
        items.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then(a.priority.rank().cmp(&b.priority.rank()))
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        let requires_decision_count = items
            .iter()
            .filter(|i| i.category == AttentionCategory::RequiresDecision)
            .count();
        let blocker_count = items
            .iter()
            .filter(|i| i.category == AttentionCategory::Blocker)
            .count();
        let informative_count = items
            .iter()
            .filter(|i| i.category == AttentionCategory::Informative)
            .count();
        let can_wait_count = items
            .iter()
            .filter(|i| i.category == AttentionCategory::CanWait)
            .count();
        let top_items: Vec<_> = items.iter().take(8).cloned().collect();
        let summary = format!(
            "Attention — {} item(s): {} require decision, {} blocker(s), {} informative, {} can wait. \
             Attention is informational only.",
            items.len(),
            requires_decision_count,
            blocker_count,
            informative_count,
            can_wait_count
        );
        Self {
            workspace_id: workspace_id.into(),
            generated_at: Utc::now().to_rfc3339(),
            items,
            top_items,
            requires_decision_count,
            blocker_count,
            informative_count,
            can_wait_count,
            summary,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn summary_projection(&self, limit: usize) -> WorkspaceAttentionSummary {
        WorkspaceAttentionSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            item_count: self.items.len(),
            requires_decision_count: self.requires_decision_count,
            blocker_count: self.blocker_count,
            informative_count: self.informative_count,
            can_wait_count: self.can_wait_count,
            top_items: self.items.iter().take(limit).cloned().collect(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAttentionSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub item_count: usize,
    pub requires_decision_count: usize,
    pub blocker_count: usize,
    pub informative_count: usize,
    pub can_wait_count: usize,
    pub top_items: Vec<AttentionItem>,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceAttentionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            item_count: 0,
            requires_decision_count: 0,
            blocker_count: 0,
            informative_count: 0,
            can_wait_count: 0,
            top_items: Vec::new(),
            summary: String::new(),
            authority_effect: AttentionItem::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}
