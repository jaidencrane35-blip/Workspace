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
}

/// Current interpreted state of a single execution request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    /// No related outcomes in the provided history.
    Unknown,
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
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionReconciliationError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(ExecutionReconciliationError::InvalidState(value.to_string())),
        }
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
}
