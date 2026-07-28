//! Audit-derived execution state reconciliation — pure domain (Sprint 29).
//!
//! [`reconcile_execution_state`] folds [`ExecutionOutcome`] history for one
//! `execution_request_id` into a current interpreted state plus dispatch /
//! cancellation flags. Pure only — no persistence, scoring, or automation.
//!
//! **Cancelled** is an outcome fact. Per Sprint 27 guard semantics it does
//! **not** block retry (`dispatch_allowed = true`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::execution_outcome::{ExecutionOutcome, ExecutionOutcomeStatus};

/// Reconciliation-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionReconciliationError {
    #[error("Execution reconciliation request id must not be empty")]
    EmptyExecutionRequestId,

    #[error("Invalid execution state: {0}")]
    InvalidState(String),

    #[error("Execution lifecycle field '{0}' must not be empty")]
    EmptyLifecycleField(&'static str),

    #[error("Completed execution lifecycle record requires completed_at")]
    MissingCompletedAt,

    #[error("Non-completed execution lifecycle record must not have completed_at")]
    UnexpectedCompletedAt,
}

/// Current interpreted state of a single execution request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    /// No related outcomes in the provided history.
    Unknown,
    /// Dispatch was durably claimed and has not reached a terminal update.
    InProgress,
    /// A successful completion exists — terminal for dispatch and cancel.
    Completed,
    /// Failed without a later completion — retryable and cancellable.
    Failed,
    /// Cancellation recorded without a completion — retry still allowed.
    Cancelled,
}

impl ExecutionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionReconciliationError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(ExecutionReconciliationError::InvalidState(value.to_string())),
        }
    }

    /// In-flight executions only — terminals project into history.
    pub fn is_actionable(self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Durable lifecycle outcomes — never treated as missing evidence.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled
        )
    }
}

/// Durable execution lifecycle fact keyed by the canonical execution request id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLifecycleRecord {
    pub execution_request_id: String,
    pub suggestion_id: String,
    pub intent_id: Option<String>,
    pub state: ExecutionState,
    pub retry_allowed: bool,
    pub failure_reason: Option<String>,
    pub claimed_at: String,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

impl ExecutionLifecycleRecord {
    pub fn validate(&self) -> Result<(), ExecutionReconciliationError> {
        for (name, value) in [
            ("execution_request_id", self.execution_request_id.as_str()),
            ("suggestion_id", self.suggestion_id.as_str()),
            ("claimed_at", self.claimed_at.as_str()),
            ("updated_at", self.updated_at.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ExecutionReconciliationError::EmptyLifecycleField(name));
            }
        }
        if !matches!(
            self.state,
            ExecutionState::InProgress
                | ExecutionState::Completed
                | ExecutionState::Failed
                | ExecutionState::Cancelled
        ) {
            return Err(ExecutionReconciliationError::InvalidState(
                self.state.as_str().into(),
            ));
        }
        if self.state == ExecutionState::Completed
            && self
                .completed_at
                .as_deref()
                .is_none_or(|value| value.trim().is_empty())
        {
            return Err(ExecutionReconciliationError::MissingCompletedAt);
        }
        if self.state != ExecutionState::Completed && self.completed_at.is_some() {
            return Err(ExecutionReconciliationError::UnexpectedCompletedAt);
        }
        if self.state == ExecutionState::Failed
            && self
                .failure_reason
                .as_deref()
                .is_none_or(|reason| reason.trim().is_empty())
        {
            return Err(ExecutionReconciliationError::InvalidState(
                "failed without failure_reason".into(),
            ));
        }
        Ok(())
    }
}

pub fn reconcile_execution_lifecycle(
    record: &ExecutionLifecycleRecord,
) -> ExecutionReconciliation {
    let (dispatch_allowed, cancellation_allowed) = match record.state {
        ExecutionState::Cancelled => (true, false),
        ExecutionState::Failed => (record.retry_allowed, record.retry_allowed),
        _ => (false, false),
    };
    ExecutionReconciliation {
        execution_request_id: record.execution_request_id.clone(),
        current_state: record.state,
        dispatch_allowed,
        cancellation_allowed,
    }
}

/// Deterministic reconciliation of outcome history for one execution id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReconciliation {
    pub execution_request_id: String,
    pub current_state: ExecutionState,
    /// Whether `ExecuteIntentRequest` may proceed (Sprint 27 semantics).
    pub dispatch_allowed: bool,
    /// Whether `RequestExecutionCancellation` may proceed (Sprint 28 semantics).
    pub cancellation_allowed: bool,
}

impl ExecutionReconciliation {
    pub fn validate(&self) -> Result<(), ExecutionReconciliationError> {
        if self.execution_request_id.trim().is_empty() {
            return Err(ExecutionReconciliationError::EmptyExecutionRequestId);
        }
        ExecutionState::parse(self.current_state.as_str())?;
        Ok(())
    }
}

/// Compact non-actionable terminal execution evidence for projection consumers.
///
/// Preserves retry eligibility, failure reason, and classification. Never
/// executable and never merged into the actionable channel. Unknown provenance
/// / state is never invented into a known terminal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleHistoryEntry {
    pub execution_request_id: String,
    pub suggestion_id: String,
    pub intent_id: Option<String>,
    pub state: ExecutionState,
    pub retry_allowed: bool,
    pub failure_reason: Option<String>,
    pub claimed_at: String,
    pub completed_at: Option<String>,
    pub updated_at: String,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl ExecutionLifecycleHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_record(record: &ExecutionLifecycleRecord) -> Option<Self> {
        if !record.state.is_terminal() {
            return None;
        }
        Some(Self {
            execution_request_id: record.execution_request_id.clone(),
            suggestion_id: record.suggestion_id.clone(),
            intent_id: record.intent_id.clone(),
            state: record.state,
            retry_allowed: record.retry_allowed,
            failure_reason: record.failure_reason.clone(),
            claimed_at: record.claimed_at.clone(),
            completed_at: record.completed_at.clone(),
            updated_at: record.updated_at.clone(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Outcome-only fallback when no durable lifecycle row remains.
    ///
    /// Does not invent Unknown into a terminal — only Completed/Failed/Cancelled.
    pub fn from_outcome(outcome: &ExecutionOutcome) -> Option<Self> {
        let state = match outcome.status {
            ExecutionOutcomeStatus::Completed => ExecutionState::Completed,
            ExecutionOutcomeStatus::Failed => ExecutionState::Failed,
            ExecutionOutcomeStatus::Cancelled => ExecutionState::Cancelled,
        };
        let retry_allowed = matches!(
            outcome.status,
            ExecutionOutcomeStatus::Failed | ExecutionOutcomeStatus::Cancelled
        );
        Some(Self {
            execution_request_id: outcome.execution_request_id.clone(),
            suggestion_id: outcome.suggestion_id.clone().unwrap_or_default(),
            intent_id: outcome.intent_id.clone(),
            state,
            retry_allowed,
            failure_reason: outcome.failure_reason.clone(),
            claimed_at: outcome.completed_at.clone(),
            completed_at: if state == ExecutionState::Completed {
                Some(outcome.completed_at.clone())
            } else {
                None
            },
            updated_at: outcome.completed_at.clone(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.state.is_terminal()
    }
}

/// In-flight execution projection — the only actionable execution channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleActionableEntry {
    pub execution_request_id: String,
    pub suggestion_id: String,
    pub intent_id: Option<String>,
    pub state: ExecutionState,
    pub claimed_at: String,
    pub updated_at: String,
    pub cancellation_allowed: bool,
    pub authority_effect: String,
}

impl ExecutionLifecycleActionableEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_record(record: &ExecutionLifecycleRecord) -> Option<Self> {
        if !record.state.is_actionable() {
            return None;
        }
        let reconciliation = reconcile_execution_lifecycle(record);
        Some(Self {
            execution_request_id: record.execution_request_id.clone(),
            suggestion_id: record.suggestion_id.clone(),
            intent_id: record.intent_id.clone(),
            state: record.state,
            claimed_at: record.claimed_at.clone(),
            updated_at: record.updated_at.clone(),
            cancellation_allowed: reconciliation.cancellation_allowed,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Dual-channel execution lifecycle projection.
///
/// `actionable` = in-progress only. `history` + `history_count` = durable
/// terminal outcomes. Unknown never defaults to a known terminal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLifecycleProjection {
    pub actionable: Vec<ExecutionLifecycleActionableEntry>,
    pub history: Vec<ExecutionLifecycleHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl ExecutionLifecycleProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_records(
        records: &[ExecutionLifecycleRecord],
        outcome_fallback: &[ExecutionOutcome],
        history_limit: usize,
    ) -> Self {
        let mut seen = std::collections::HashSet::new();
        let mut actionable = Vec::new();
        let mut history = Vec::new();

        for record in records {
            // Unknown must remain unknown — never invent actionable/terminal.
            if record.state == ExecutionState::Unknown {
                continue;
            }
            seen.insert(record.execution_request_id.clone());
            if let Some(entry) = ExecutionLifecycleActionableEntry::from_record(record) {
                actionable.push(entry);
            }
            if let Some(entry) = ExecutionLifecycleHistoryEntry::from_record(record) {
                history.push(entry);
            }
        }

        for state in reconcile_execution_states(outcome_fallback) {
            if !seen.insert(state.execution_request_id.clone()) {
                continue;
            }
            // Unknown stays unknown — skip both channels.
            if state.current_state == ExecutionState::Unknown {
                continue;
            }
            if state.current_state.is_actionable() {
                // Outcome-only in-progress is not represented without a lifecycle claim.
                continue;
            }
            if let Some(outcome) = outcome_fallback
                .iter()
                .find(|o| o.execution_request_id == state.execution_request_id)
            {
                if let Some(entry) = ExecutionLifecycleHistoryEntry::from_outcome(outcome) {
                    history.push(entry);
                }
            }
        }

        history.sort_by(|a, b| {
            b.updated_at
                .cmp(&a.updated_at)
                .then_with(|| a.execution_request_id.cmp(&b.execution_request_id))
        });
        let history_count = history.len();
        let history: Vec<_> = history.into_iter().take(history_limit).collect();

        Self {
            actionable,
            history,
            history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            actionable: Vec::new(),
            history: Vec::new(),
            history_count: 0,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Fold outcomes for `execution_request_id` into a reconciled state.
///
/// Precedence (aligned with existing guard/cancellation rules):
/// 1. No related outcomes → `Unknown` (dispatch false, cancel false)
/// 2. Any `Completed` → `Completed` (dispatch false, cancel false)
/// 3. Else any `Cancelled` → `Cancelled` (dispatch true, cancel false)
/// 4. Else any `Failed` → `Failed` (dispatch true, cancel true)
pub fn reconcile_execution_state(
    execution_request_id: &str,
    outcomes: &[ExecutionOutcome],
) -> ExecutionReconciliation {
    let execution_request_id = execution_request_id.trim().to_string();
    if execution_request_id.is_empty() {
        return ExecutionReconciliation {
            execution_request_id,
            current_state: ExecutionState::Unknown,
            dispatch_allowed: false,
            cancellation_allowed: false,
        };
    }

    let related: Vec<&ExecutionOutcome> = outcomes
        .iter()
        .filter(|outcome| outcome.execution_request_id == execution_request_id)
        .collect();

    if related.is_empty() {
        return ExecutionReconciliation {
            execution_request_id,
            current_state: ExecutionState::Unknown,
            dispatch_allowed: false,
            cancellation_allowed: false,
        };
    }

    if related
        .iter()
        .any(|outcome| outcome.status == ExecutionOutcomeStatus::Completed)
    {
        return ExecutionReconciliation {
            execution_request_id,
            current_state: ExecutionState::Completed,
            dispatch_allowed: false,
            cancellation_allowed: false,
        };
    }

    if related
        .iter()
        .any(|outcome| outcome.status == ExecutionOutcomeStatus::Cancelled)
    {
        return ExecutionReconciliation {
            execution_request_id,
            current_state: ExecutionState::Cancelled,
            dispatch_allowed: true,
            cancellation_allowed: false,
        };
    }

    // Related outcomes exist and are Failed-only (or other non-terminal).
    ExecutionReconciliation {
        execution_request_id,
        current_state: ExecutionState::Failed,
        dispatch_allowed: true,
        cancellation_allowed: true,
    }
}

/// Reconciles all unique execution ids present in `outcomes`.
///
/// Ids are ordered by most-recent first appearance in the outcome stream.
/// Each id is folded with the same precedence as [`reconcile_execution_state`].
pub fn reconcile_execution_states(outcomes: &[ExecutionOutcome]) -> Vec<ExecutionReconciliation> {
    let mut seen = std::collections::HashSet::new();
    let mut ids = Vec::new();
    for outcome in outcomes {
        if seen.insert(outcome.execution_request_id.clone()) {
            ids.push(outcome.execution_request_id.clone());
        }
    }

    ids.into_iter()
        .map(|id| reconcile_execution_state(&id, outcomes))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome(id: &str, status: ExecutionOutcomeStatus) -> ExecutionOutcome {
        ExecutionOutcome {
            execution_request_id: id.into(),
            status,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: "2026-07-24T12:00:00Z".into(),
            success: matches!(status, ExecutionOutcomeStatus::Completed),
            failure_reason: None,
            suggestion_id: Some("s-1".into()),
            intent_id: None,
        }
    }

    #[test]
    fn empty_history_is_unknown() {
        let result = reconcile_execution_state("execution:s-1", &[]);
        assert_eq!(result.current_state, ExecutionState::Unknown);
        assert!(!result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn durable_in_progress_record_is_valid_and_non_dispatchable() {
        let record = ExecutionLifecycleRecord {
            execution_request_id: "execution:s-1".into(),
            suggestion_id: "s-1".into(),
            intent_id: Some("intent:test".into()),
            state: ExecutionState::InProgress,
            retry_allowed: false,
            failure_reason: None,
            claimed_at: "2026-07-28T00:00:00Z".into(),
            completed_at: None,
            updated_at: "2026-07-28T00:00:00Z".into(),
        };
        assert!(record.validate().is_ok());
        assert_eq!(
            ExecutionState::parse("in_progress").unwrap(),
            ExecutionState::InProgress
        );
    }

    #[test]
    fn completed_is_terminal() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Completed)],
        );
        assert_eq!(result.current_state, ExecutionState::Completed);
        assert!(!result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn failed_is_retryable_and_cancellable() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Failed)],
        );
        assert_eq!(result.current_state, ExecutionState::Failed);
        assert!(result.dispatch_allowed);
        assert!(result.cancellation_allowed);
    }

    #[test]
    fn cancelled_state_allows_retry_not_cancel() {
        // Cancelled is an outcome fact; it does not block retry (Sprint 27).
        let result = reconcile_execution_state(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Cancelled)],
        );
        assert_eq!(result.current_state, ExecutionState::Cancelled);
        assert!(result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn mixed_failed_completed_prefers_completed() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[
                outcome("execution:s-1", ExecutionOutcomeStatus::Failed),
                outcome("execution:s-1", ExecutionOutcomeStatus::Completed),
            ],
        );
        assert_eq!(result.current_state, ExecutionState::Completed);
        assert!(!result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn failed_then_cancelled_is_cancelled() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[
                outcome("execution:s-1", ExecutionOutcomeStatus::Failed),
                outcome("execution:s-1", ExecutionOutcomeStatus::Cancelled),
            ],
        );
        assert_eq!(result.current_state, ExecutionState::Cancelled);
        assert!(result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn unrelated_outcomes_ignored() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[outcome("execution:other", ExecutionOutcomeStatus::Completed)],
        );
        assert_eq!(result.current_state, ExecutionState::Unknown);
    }

    #[test]
    fn serializes_round_trip() {
        let result = reconcile_execution_state(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Failed)],
        );
        let json = serde_json::to_string(&result).unwrap();
        let restored: ExecutionReconciliation = serde_json::from_str(&json).unwrap();
        assert_eq!(result, restored);
    }

    #[test]
    fn list_reconciles_multiple_ids_in_order() {
        let outcomes = vec![
            outcome("execution:b", ExecutionOutcomeStatus::Failed),
            outcome("execution:a", ExecutionOutcomeStatus::Completed),
            outcome("execution:c", ExecutionOutcomeStatus::Cancelled),
        ];
        let states = reconcile_execution_states(&outcomes);
        assert_eq!(states.len(), 3);
        assert_eq!(states[0].execution_request_id, "execution:b");
        assert_eq!(states[0].current_state, ExecutionState::Failed);
        assert_eq!(states[1].execution_request_id, "execution:a");
        assert_eq!(states[1].current_state, ExecutionState::Completed);
        assert_eq!(states[2].execution_request_id, "execution:c");
        assert_eq!(states[2].current_state, ExecutionState::Cancelled);
    }

    #[test]
    fn list_empty_outcomes() {
        assert!(reconcile_execution_states(&[]).is_empty());
    }

    #[test]
    fn list_dedupes_same_id() {
        let outcomes = vec![
            outcome("execution:s-1", ExecutionOutcomeStatus::Failed),
            outcome("execution:s-1", ExecutionOutcomeStatus::Completed),
        ];
        let states = reconcile_execution_states(&outcomes);
        assert_eq!(states.len(), 1);
        assert_eq!(states[0].current_state, ExecutionState::Completed);
    }

    #[test]
    fn projection_separates_actionable_from_terminal_history() {
        let records = vec![
            ExecutionLifecycleRecord {
                execution_request_id: "execution:active".into(),
                suggestion_id: "s-active".into(),
                intent_id: None,
                state: ExecutionState::InProgress,
                retry_allowed: false,
                failure_reason: None,
                claimed_at: "t0".into(),
                completed_at: None,
                updated_at: "t1".into(),
            },
            ExecutionLifecycleRecord {
                execution_request_id: "execution:failed".into(),
                suggestion_id: "s-fail".into(),
                intent_id: Some("intent:x".into()),
                state: ExecutionState::Failed,
                retry_allowed: true,
                failure_reason: Some("gateway_denied".into()),
                claimed_at: "t0".into(),
                completed_at: None,
                updated_at: "t2".into(),
            },
            ExecutionLifecycleRecord {
                execution_request_id: "execution:done".into(),
                suggestion_id: "s-done".into(),
                intent_id: None,
                state: ExecutionState::Completed,
                retry_allowed: false,
                failure_reason: None,
                claimed_at: "t0".into(),
                completed_at: Some("t3".into()),
                updated_at: "t3".into(),
            },
        ];
        let projection = ExecutionLifecycleProjection::from_records(&records, &[], 10);
        assert_eq!(projection.actionable.len(), 1);
        assert_eq!(
            projection.actionable[0].execution_request_id,
            "execution:active"
        );
        assert!(projection
            .actionable
            .iter()
            .all(|e| e.state.is_actionable()));
        assert_eq!(projection.history_count, 2);
        assert!(projection.history.iter().all(|h| h.is_non_actionable()));
        let failed = projection
            .history
            .iter()
            .find(|h| h.execution_request_id == "execution:failed")
            .expect("failed in history");
        assert!(failed.retry_allowed);
        assert_eq!(failed.failure_reason.as_deref(), Some("gateway_denied"));
        assert!(!projection
            .actionable
            .iter()
            .any(|e| e.execution_request_id == "execution:failed"));
        assert!(!projection
            .history
            .iter()
            .any(|h| h.execution_request_id == "execution:active"));
    }

    #[test]
    fn projection_history_count_authoritative_over_window() {
        let records: Vec<_> = (0..5)
            .map(|i| ExecutionLifecycleRecord {
                execution_request_id: format!("execution:{i}"),
                suggestion_id: format!("s-{i}"),
                intent_id: None,
                state: ExecutionState::Completed,
                retry_allowed: false,
                failure_reason: None,
                claimed_at: "t0".into(),
                completed_at: Some(format!("t{i}")),
                updated_at: format!("t{i}"),
            })
            .collect();
        let projection = ExecutionLifecycleProjection::from_records(&records, &[], 2);
        assert!(projection.actionable.is_empty());
        assert_eq!(projection.history.len(), 2);
        assert_eq!(projection.history_count, 5);
    }

    #[test]
    fn unknown_state_never_invented_into_channels() {
        let records = vec![ExecutionLifecycleRecord {
            execution_request_id: "execution:mystery".into(),
            suggestion_id: "s-x".into(),
            intent_id: None,
            state: ExecutionState::Unknown,
            retry_allowed: false,
            failure_reason: None,
            claimed_at: "t0".into(),
            completed_at: None,
            updated_at: "t0".into(),
        }];
        let projection = ExecutionLifecycleProjection::from_records(&records, &[], 10);
        assert!(projection.actionable.is_empty());
        assert_eq!(projection.history_count, 0);
        assert!(projection.history.is_empty());
    }

    #[test]
    fn history_entry_not_convertible_to_reconciliation_dispatch_surface() {
        let entry = ExecutionLifecycleHistoryEntry::from_record(&ExecutionLifecycleRecord {
            execution_request_id: "execution:fail".into(),
            suggestion_id: "s".into(),
            intent_id: None,
            state: ExecutionState::Failed,
            retry_allowed: true,
            failure_reason: Some("boom".into()),
            claimed_at: "t0".into(),
            completed_at: None,
            updated_at: "t1".into(),
        })
        .unwrap();
        assert!(entry.is_non_actionable());
        assert!(!entry.actionable);
        // History DTO must not expose cancel/dispatch command handles.
        let json = serde_json::to_value(&entry).unwrap();
        assert!(json.get("dispatch_allowed").is_none());
        assert!(json.get("cancellation_allowed").is_none());
    }
}
