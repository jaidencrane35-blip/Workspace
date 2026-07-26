//! Workspace Attention Engine foundation (Phase 5 Batch 2 / Sprints 125–126).
//!
//! A governed prioritization layer over Workspace context.
//!
//! # Facts vs inference
//!
//! **Fact inputs** (owned elsewhere): Environment (← WorkspaceState), Decision Queue,
//! Continuity, Activity Graph, Task Graph, Composition, Purpose, Evolution.
//!
//! **Inference outputs** (this model): `score`, `priority`, `urgency`, `category`,
//! ranked `top_items`, and structured `reasons`. Attention never observes the desktop,
//! never executes, and never persists user decisions.
//!
//! # Score vs explanation
//!
//! - `score` / `score_factors` — how the priority value was computed
//! - `reasons` — why the item received attention (structured, UI-ready keys)
//! - `explanation` — short narrative from the source (not a second scoring path)
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

/// Structured signal explaining why an item received attention.
///
/// Keys are stable contracts for UI/i18n — not hardcoded display strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSignal {
    OutstandingDecision,
    BlockedAction,
    BlockedTask,
    WaitingTask,
    InProgressTask,
    InterruptedWork,
    UnfinishedContinuity,
    ResumableWork,
    CurrentFocus,
    DormantWork,
    CommitmentPending,
    EnvironmentDisconnect,
    MissingApplication,
    CompositionGap,
    HighPriorityIntent,
    PurposeObstacle,
    PurposeOutcome,
    EvolutionInsight,
    ActivityProgress,
    RecommendationCandidate,
    PatternObservation,
}

impl AttentionSignal {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OutstandingDecision => "outstanding_decision",
            Self::BlockedAction => "blocked_action",
            Self::BlockedTask => "blocked_task",
            Self::WaitingTask => "waiting_task",
            Self::InProgressTask => "in_progress_task",
            Self::InterruptedWork => "interrupted_work",
            Self::UnfinishedContinuity => "unfinished_continuity",
            Self::ResumableWork => "resumable_work",
            Self::CurrentFocus => "current_focus",
            Self::DormantWork => "dormant_work",
            Self::CommitmentPending => "commitment_pending",
            Self::EnvironmentDisconnect => "environment_disconnect",
            Self::MissingApplication => "missing_application",
            Self::CompositionGap => "composition_gap",
            Self::HighPriorityIntent => "high_priority_intent",
            Self::PurposeObstacle => "purpose_obstacle",
            Self::PurposeOutcome => "purpose_outcome",
            Self::EvolutionInsight => "evolution_insight",
            Self::ActivityProgress => "activity_progress",
            Self::RecommendationCandidate => "recommendation_candidate",
            Self::PatternObservation => "pattern_observation",
        }
    }
}

/// One structured reason an Attention Item was ranked.
///
/// Independent of score math: UI may render `explanation_key` without recomputing scores.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionReason {
    pub source: AttentionSourceType,
    pub signal: AttentionSignal,
    /// Contribution toward the item score (may be negative). Not a second score.
    pub weight: i32,
    /// Stable key for localization / UI (e.g. `decision.priority.critical`).
    pub explanation_key: String,
}

impl AttentionReason {
    pub fn new(
        source: AttentionSourceType,
        signal: AttentionSignal,
        weight: i32,
        explanation_key: impl Into<String>,
    ) -> Self {
        Self {
            source,
            signal,
            weight,
            explanation_key: explanation_key.into(),
        }
    }
}

/// Deduplicate by `explanation_key` (keep highest weight) and order
/// weight DESC, then explanation_key ASC.
pub fn normalize_attention_reasons(reasons: Vec<AttentionReason>) -> Vec<AttentionReason> {
    let mut by_key: std::collections::BTreeMap<String, AttentionReason> =
        std::collections::BTreeMap::new();
    for reason in reasons {
        match by_key.get(&reason.explanation_key) {
            Some(existing) if existing.weight >= reason.weight => {}
            _ => {
                by_key.insert(reason.explanation_key.clone(), reason);
            }
        }
    }
    let mut out: Vec<_> = by_key.into_values().collect();
    out.sort_by(|a, b| {
        b.weight
            .cmp(&a.weight)
            .then(a.explanation_key.cmp(&b.explanation_key))
    });
    out
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
/// `source_type` / `source_id` point at facts owned elsewhere. `score` /
/// `score_factors` are scoring inference; `reasons` explain why attention was
/// granted — independent of score math.
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
    /// Deterministic score (higher = more attention). Math via `score_factors`.
    pub score: u32,
    /// Human-readable factor list used to compute `score` (not UI copy).
    pub score_factors: Vec<String>,
    /// Structured reasons why this item received attention (not score recomputation).
    pub reasons: Vec<AttentionReason>,
    pub title: String,
    /// Short narrative from the source fact — not a duplicate of scoring.
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
        reasons: Vec<AttentionReason>,
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
            reasons: normalize_attention_reasons(reasons),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_reasons_dedupes_and_orders() {
        let normalized = normalize_attention_reasons(vec![
            AttentionReason::new(
                AttentionSourceType::DecisionQueue,
                AttentionSignal::OutstandingDecision,
                70,
                "decision.base.outstanding",
            ),
            AttentionReason::new(
                AttentionSourceType::DecisionQueue,
                AttentionSignal::HighPriorityIntent,
                12,
                "decision.priority.high",
            ),
            AttentionReason::new(
                AttentionSourceType::DecisionQueue,
                AttentionSignal::OutstandingDecision,
                50,
                "decision.base.outstanding",
            ),
            AttentionReason::new(
                AttentionSourceType::DecisionQueue,
                AttentionSignal::HighPriorityIntent,
                20,
                "decision.priority.critical",
            ),
        ]);
        assert_eq!(normalized.len(), 3);
        assert_eq!(normalized[0].explanation_key, "decision.base.outstanding");
        assert_eq!(normalized[0].weight, 70);
        assert_eq!(normalized[1].explanation_key, "decision.priority.critical");
        assert_eq!(normalized[2].explanation_key, "decision.priority.high");
    }

    #[test]
    fn empty_reasons_normalize_to_empty() {
        assert!(normalize_attention_reasons(vec![]).is_empty());
    }

    #[test]
    fn project_keeps_score_independent_of_reason_order() {
        let a = AttentionItem::project(
            "ws",
            AttentionSourceType::TaskGraph,
            "t1",
            AttentionCategory::Blocker,
            AttentionPriority::Critical,
            AttentionUrgency::Immediate,
            AttentionConfidence::High,
            88,
            vec!["base 88".into()],
            vec![
                AttentionReason::new(
                    AttentionSourceType::TaskGraph,
                    AttentionSignal::BlockedTask,
                    88,
                    "task.blocked",
                ),
                AttentionReason::new(
                    AttentionSourceType::TaskGraph,
                    AttentionSignal::HighPriorityIntent,
                    8,
                    "task.priority.high",
                ),
            ],
            "Blocked",
            "Task is blocked.",
            "now",
            AttentionState::Visible,
        )
        .unwrap();
        let b = AttentionItem::project(
            "ws",
            AttentionSourceType::TaskGraph,
            "t1",
            AttentionCategory::Blocker,
            AttentionPriority::Critical,
            AttentionUrgency::Immediate,
            AttentionConfidence::High,
            88,
            vec!["base 88".into()],
            vec![
                AttentionReason::new(
                    AttentionSourceType::TaskGraph,
                    AttentionSignal::HighPriorityIntent,
                    8,
                    "task.priority.high",
                ),
                AttentionReason::new(
                    AttentionSourceType::TaskGraph,
                    AttentionSignal::BlockedTask,
                    88,
                    "task.blocked",
                ),
            ],
            "Blocked",
            "Task is blocked.",
            "now",
            AttentionState::Visible,
        )
        .unwrap();
        assert_eq!(a.score, b.score);
        assert_eq!(a.reasons, b.reasons);
    }
}
