//! Governed Decision Queue foundation (Phase 4 Batch 8).
//!
//! Aggregates pending human decisions from existing sources.
//! Never executes, never grants authority, never becomes a second source of truth.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{DecisionItemId, WorkspaceId};

/// Decision-queue validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecisionQueueError {
    #[error("invalid decision source type: {0}")]
    InvalidSourceType(String),

    #[error("invalid decision state: {0}")]
    InvalidState(String),

    #[error("invalid decision priority: {0}")]
    InvalidPriority(String),

    #[error("invalid decision category: {0}")]
    InvalidCategory(String),

    #[error("decision item not found")]
    NotFound,

    #[error("decision transition not allowed from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("decision queue cannot authorize permissions")]
    CannotAuthorizePermissions,

    #[error("decision queue cannot execute or grant authority")]
    CannotExecute,

    #[error("decision accept requires source subsystem handoff")]
    RequiresSourceHandoff,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Origin system for a DecisionItem (extensible).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionSourceType {
    IntentProposal,
    PendingApproval,
    BlockedAction,
    PlanningContinuation,
}

impl DecisionSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IntentProposal => "intent_proposal",
            Self::PendingApproval => "pending_approval",
            Self::BlockedAction => "blocked_action",
            Self::PlanningContinuation => "planning_continuation",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionQueueError> {
        match value {
            "intent_proposal" => Ok(Self::IntentProposal),
            "pending_approval" => Ok(Self::PendingApproval),
            "blocked_action" => Ok(Self::BlockedAction),
            "planning_continuation" => Ok(Self::PlanningContinuation),
            other => Err(DecisionQueueError::InvalidSourceType(other.into())),
        }
    }
}

/// Product category for display / filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionCategory {
    Permission,
    Automation,
    Blocked,
    Planning,
    ContractDefinition,
}

impl DecisionCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Permission => "permission",
            Self::Automation => "automation",
            Self::Blocked => "blocked",
            Self::Planning => "planning",
            Self::ContractDefinition => "contract_definition",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionQueueError> {
        match value {
            "permission" => Ok(Self::Permission),
            "automation" => Ok(Self::Automation),
            "blocked" => Ok(Self::Blocked),
            "planning" => Ok(Self::Planning),
            "contract_definition" => Ok(Self::ContractDefinition),
            other => Err(DecisionQueueError::InvalidCategory(other.into())),
        }
    }
}

/// Lifecycle state for a DecisionItem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    Pending,
    Viewed,
    Deferred,
    Dismissed,
    Accepted,
    Rejected,
    Expired,
}

impl DecisionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Viewed => "viewed",
            Self::Deferred => "deferred",
            Self::Dismissed => "dismissed",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionQueueError> {
        match value {
            "pending" => Ok(Self::Pending),
            "viewed" => Ok(Self::Viewed),
            "deferred" => Ok(Self::Deferred),
            "dismissed" => Ok(Self::Dismissed),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            other => Err(DecisionQueueError::InvalidState(other.into())),
        }
    }

    /// Overlay-persisted states (never store source payloads).
    /// `Expired` is overlay-owned orphan retention when the live source disappears.
    pub fn is_overlay_state(self) -> bool {
        matches!(
            self,
            Self::Pending | Self::Viewed | Self::Deferred | Self::Dismissed | Self::Expired
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Dismissed | Self::Accepted | Self::Rejected | Self::Expired
        )
    }

    /// Actionable presentation states for Intelligence / compact summaries.
    pub fn is_actionable_overlay(self) -> bool {
        matches!(
            self,
            Self::Pending | Self::Viewed | Self::Deferred
        )
    }

    pub fn allows_overlay_transition(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Viewed)
                | (Self::Pending, Self::Deferred)
                | (Self::Pending, Self::Dismissed)
                | (Self::Pending, Self::Expired)
                | (Self::Viewed, Self::Deferred)
                | (Self::Viewed, Self::Dismissed)
                | (Self::Viewed, Self::Expired)
                | (Self::Deferred, Self::Viewed)
                | (Self::Deferred, Self::Dismissed)
                | (Self::Deferred, Self::Expired)
        )
    }
}

/// Display / ordering priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionPriority {
    Critical,
    High,
    Normal,
    Low,
}

impl DecisionPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Normal => "normal",
            Self::Low => "low",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DecisionQueueError> {
        match value {
            "critical" => Ok(Self::Critical),
            "high" => Ok(Self::High),
            "normal" => Ok(Self::Normal),
            "low" => Ok(Self::Low),
            other => Err(DecisionQueueError::InvalidPriority(other.into())),
        }
    }

    pub fn weight(self) -> u8 {
        match self {
            Self::Critical => 0,
            Self::High => 1,
            Self::Normal => 2,
            Self::Low => 3,
        }
    }
}

/// Canonical Workspace inbox item for a governed human decision.
///
/// Aggregated — not a duplicate of the source system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionItem {
    pub id: DecisionItemId,
    pub workspace_id: WorkspaceId,
    pub source_type: DecisionSourceType,
    pub source_id: String,
    pub category: DecisionCategory,
    pub title: String,
    pub summary: String,
    pub explanation: String,
    pub recommended_action: String,
    pub decision_state: DecisionState,
    pub priority: DecisionPriority,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub actor_id: String,
    pub actor_type: String,
    pub project_id: Option<String>,
    pub required_capabilities: Vec<String>,
    pub handoff_command: Option<String>,
    pub authority_effect: String,
}

impl DecisionItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn synthetic_id(source_type: DecisionSourceType, source_id: &str) -> DecisionItemId {
        DecisionItemId::new(format!("decision:{}:{}", source_type.as_str(), source_id))
            .expect("synthetic decision id is valid")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn aggregate(
        workspace_id: impl Into<String>,
        source_type: DecisionSourceType,
        source_id: impl Into<String>,
        category: DecisionCategory,
        title: impl Into<String>,
        summary: impl Into<String>,
        explanation: impl Into<String>,
        recommended_action: impl Into<String>,
        priority: DecisionPriority,
        created_at: impl Into<String>,
        actor_id: impl Into<String>,
        actor_type: impl Into<String>,
        project_id: Option<String>,
        required_capabilities: Vec<String>,
        handoff_command: Option<String>,
    ) -> Result<Self, DecisionQueueError> {
        let source_id = source_id.into();
        Ok(Self {
            id: Self::synthetic_id(source_type, &source_id),
            workspace_id: WorkspaceId::new(workspace_id)?,
            source_type,
            source_id,
            category,
            title: title.into(),
            summary: summary.into(),
            explanation: explanation.into(),
            recommended_action: recommended_action.into(),
            decision_state: DecisionState::Pending,
            priority,
            created_at: created_at.into(),
            expires_at: None,
            actor_id: actor_id.into(),
            actor_type: actor_type.into(),
            project_id,
            required_capabilities,
            handoff_command,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn apply_overlay(&mut self, state: DecisionState) {
        if state.is_overlay_state() {
            self.decision_state = state;
        }
    }

    pub fn dependency_boost(&self) -> u8 {
        match self.source_type {
            DecisionSourceType::BlockedAction => 0,
            DecisionSourceType::PendingApproval => 1,
            DecisionSourceType::IntentProposal => 2,
            DecisionSourceType::PlanningContinuation => 3,
        }
    }
}

/// Handoff when the queue cannot (or must not) mutate the source itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionHandoff {
    pub decision_item_id: String,
    pub source_type: DecisionSourceType,
    pub source_id: String,
    pub next_command: String,
    pub note: String,
    pub authority_effect: String,
}

impl DecisionHandoff {
    pub fn new(
        item: &DecisionItem,
        next_command: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            decision_item_id: item.id.to_string(),
            source_type: item.source_type,
            source_id: item.source_id.clone(),
            next_command: next_command.into(),
            note: note.into(),
            authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Result of an accept/reject attempt on a DecisionItem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionActionResult {
    pub item: Option<DecisionItem>,
    pub handoff: Option<DecisionHandoff>,
    pub delegated: bool,
    pub authority_effect: String,
}

/// Compact non-actionable terminal overlay evidence for projection consumers.
///
/// History is read-only continuity — never executable, never merged into actionable
/// `items`, and never a second lifecycle authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionOverlayHistoryEntry {
    pub decision_item_id: String,
    pub source_type: DecisionSourceType,
    pub source_id: String,
    /// Terminal overlay state (`dismissed` or `expired`).
    pub decision_state: DecisionState,
    /// Live aggregation source was absent when this entry was projected.
    pub orphaned: bool,
    pub updated_at: String,
    pub actor_id: String,
    /// Always `false` — terminal history never joins actionable queues.
    pub actionable: bool,
    pub authority_effect: String,
}

impl DecisionOverlayHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Project retained terminal overlay evidence. Open overlays are excluded.
    pub fn from_overlay(overlay: &DecisionLifecycleOverlay, orphaned: bool) -> Option<Self> {
        if !matches!(
            overlay.decision_state,
            DecisionState::Dismissed | DecisionState::Expired
        ) {
            return None;
        }
        Some(Self {
            decision_item_id: DecisionItem::synthetic_id(overlay.source_type, &overlay.source_id)
                .to_string(),
            source_type: overlay.source_type,
            source_id: overlay.source_id.clone(),
            decision_state: overlay.decision_state,
            orphaned,
            updated_at: overlay.updated_at.clone(),
            actor_id: overlay.actor_id.clone(),
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && matches!(
                self.decision_state,
                DecisionState::Dismissed | DecisionState::Expired
            )
    }
}

/// Ordered Workspace Decision Queue snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionQueue {
    pub workspace_id: String,
    pub generated_at: String,
    pub items: Vec<DecisionItem>,
    pub pending_count: usize,
    pub high_priority_count: usize,
    /// Terminal / orphan overlay evidence (not actionable).
    #[serde(default)]
    pub history: Vec<DecisionOverlayHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
    pub authority_effect: String,
}

impl DecisionQueue {
    pub fn from_items(workspace_id: impl Into<String>, mut items: Vec<DecisionItem>) -> Self {
        items.sort_by(|a, b| {
            a.priority
                .weight()
                .cmp(&b.priority.weight())
                .then(a.dependency_boost().cmp(&b.dependency_boost()))
                .then(a.created_at.cmp(&b.created_at))
                .then(a.id.as_str().cmp(b.id.as_str()))
        });
        let pending_count = items
            .iter()
            .filter(|i| {
                matches!(
                    i.decision_state,
                    DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
                )
            })
            .count();
        let high_priority_count = items
            .iter()
            .filter(|i| {
                matches!(
                    i.priority,
                    DecisionPriority::Critical | DecisionPriority::High
                ) && !matches!(
                    i.decision_state,
                    DecisionState::Dismissed
                        | DecisionState::Accepted
                        | DecisionState::Rejected
                        | DecisionState::Expired
                )
            })
            .count();
        Self {
            workspace_id: workspace_id.into(),
            generated_at: Utc::now().to_rfc3339(),
            items,
            pending_count,
            high_priority_count,
            history: Vec::new(),
            history_count: 0,
            authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Attach terminal overlay history without altering actionable `items`.
    pub fn with_overlay_history(mut self, history: Vec<DecisionOverlayHistoryEntry>) -> Self {
        self.history_count = history.len();
        self.history = history;
        self
    }

    pub fn summary(&self, limit: usize) -> DecisionQueueSummary {
        let actionable: Vec<DecisionItem> = self
            .items
            .iter()
            .filter(|i| i.decision_state.is_actionable_overlay())
            .cloned()
            .collect();
        // Actionable surface excludes terminals; history remains visible so Intelligence
        // cannot treat "no pending items" as "no retained overlay evidence".
        let history: Vec<DecisionOverlayHistoryEntry> =
            self.history.iter().take(limit).cloned().collect();
        DecisionQueueSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            pending_count: self.pending_count,
            high_priority_count: self.high_priority_count,
            items: actionable.into_iter().take(limit).collect(),
            history,
            history_count: self.history_count,
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact queue projection for Workspace Intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionQueueSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub pending_count: usize,
    pub high_priority_count: usize,
    /// Actionable overlay states only (`pending` / `viewed` / `deferred`).
    pub items: Vec<DecisionItem>,
    /// Truncated terminal/orphan overlay evidence (never actionable).
    #[serde(default)]
    pub history: Vec<DecisionOverlayHistoryEntry>,
    #[serde(default)]
    pub history_count: usize,
    pub authority_effect: String,
}

impl Default for DecisionQueueSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            generated_at: String::new(),
            pending_count: 0,
            high_priority_count: 0,
            items: Vec::new(),
            history: Vec::new(),
            history_count: 0,
            authority_effect: DecisionItem::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Persisted lifecycle overlay row (no source payload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionLifecycleOverlay {
    pub workspace_id: String,
    pub source_type: DecisionSourceType,
    pub source_id: String,
    pub decision_state: DecisionState,
    pub updated_at: String,
    pub actor_id: String,
}

#[cfg(test)]
mod projection_integrity_tests {
    use super::*;

    fn item(state: DecisionState, source_id: &str) -> DecisionItem {
        let mut item = DecisionItem::aggregate(
            "ws-1",
            DecisionSourceType::IntentProposal,
            source_id,
            DecisionCategory::Planning,
            "title",
            "summary",
            "explanation",
            "action",
            DecisionPriority::Normal,
            "t0",
            "actor",
            "user",
            None,
            vec![],
            None,
        )
        .unwrap();
        item.decision_state = state;
        item
    }

    #[test]
    fn summary_excludes_terminal_presentation_states() {
        let queue = DecisionQueue::from_items(
            "ws-1",
            vec![
                item(DecisionState::Pending, "p"),
                item(DecisionState::Dismissed, "d"),
                item(DecisionState::Expired, "e"),
                item(DecisionState::Viewed, "v"),
            ],
        );
        let summary = queue.summary(10);
        assert_eq!(summary.items.len(), 2);
        assert!(summary
            .items
            .iter()
            .all(|i| i.decision_state.is_actionable_overlay()));
        assert!(!summary
            .items
            .iter()
            .any(|i| i.source_id == "d" || i.source_id == "e"));
    }

    fn overlay(state: DecisionState, source_id: &str) -> DecisionLifecycleOverlay {
        DecisionLifecycleOverlay {
            workspace_id: "ws-1".into(),
            source_type: DecisionSourceType::IntentProposal,
            source_id: source_id.into(),
            decision_state: state,
            updated_at: "t1".into(),
            actor_id: "actor".into(),
        }
    }

    #[test]
    fn history_projects_dismissed_expired_and_orphaned_without_actionability() {
        let dismissed = DecisionOverlayHistoryEntry::from_overlay(
            &overlay(DecisionState::Dismissed, "d"),
            false,
        )
        .expect("dismissed projects");
        let orphan_dismissed = DecisionOverlayHistoryEntry::from_overlay(
            &overlay(DecisionState::Dismissed, "od"),
            true,
        )
        .expect("orphan dismissed projects");
        let expired = DecisionOverlayHistoryEntry::from_overlay(
            &overlay(DecisionState::Expired, "e"),
            true,
        )
        .expect("expired orphan projects");

        assert!(!dismissed.orphaned);
        assert!(orphan_dismissed.orphaned);
        assert_eq!(expired.decision_state, DecisionState::Expired);
        assert!(expired.orphaned);
        for entry in [&dismissed, &orphan_dismissed, &expired] {
            assert!(entry.is_non_actionable());
            assert!(!entry.actionable);
            assert_eq!(
                entry.authority_effect,
                DecisionOverlayHistoryEntry::AUTHORITY_EFFECT_NONE
            );
            assert_eq!(
                entry.source_type,
                DecisionSourceType::IntentProposal
            );
        }
        assert!(DecisionOverlayHistoryEntry::from_overlay(
            &overlay(DecisionState::Pending, "open"),
            true
        )
        .is_none());
    }

    #[test]
    fn summary_carries_history_while_excluding_terminals_from_items() {
        let history = vec![
            DecisionOverlayHistoryEntry::from_overlay(
                &overlay(DecisionState::Dismissed, "d"),
                false,
            )
            .unwrap(),
            DecisionOverlayHistoryEntry::from_overlay(
                &overlay(DecisionState::Expired, "e"),
                true,
            )
            .unwrap(),
        ];
        let queue = DecisionQueue::from_items(
            "ws-1",
            vec![
                item(DecisionState::Pending, "p"),
                item(DecisionState::Dismissed, "d"),
            ],
        )
        .with_overlay_history(history);
        let summary = queue.summary(10);
        assert_eq!(summary.items.len(), 1);
        assert_eq!(summary.items[0].source_id, "p");
        assert_eq!(summary.history_count, 2);
        assert_eq!(summary.history.len(), 2);
        assert!(summary.history.iter().all(|h| h.is_non_actionable()));
        assert!(summary.history.iter().any(|h| h.orphaned && h.source_id == "e"));
    }
}
