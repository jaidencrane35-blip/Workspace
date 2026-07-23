//! Audit-derived execution idempotency guard — pure domain (Sprint 27).
//!
//! [`ExecutionGuardResult`] answers whether a governed execution request may
//! proceed, based on prior [`ExecutionOutcome`] history. Pure decision only —
//! no persistence, scoring, or retry policy engine.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::execution_outcome::{ExecutionOutcome, ExecutionOutcomeStatus};

/// Guard-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionGuardError {
    #[error("Invalid execution guard result: {0}")]
    InvalidResult(String),
}

/// Deterministic result of an execution idempotency check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionGuardResult {
    /// No prior successful completion — dispatch may proceed.
    Allowed,
    /// A successful completion already exists for this request id.
    AlreadyExecuted,
    /// The request identifier is unusable.
    InvalidRequest,
}

impl ExecutionGuardResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::AlreadyExecuted => "already_executed",
            Self::InvalidRequest => "invalid_request",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionGuardError> {
        match value {
            "allowed" => Ok(Self::Allowed),
            "already_executed" => Ok(Self::AlreadyExecuted),
            "invalid_request" => Ok(Self::InvalidRequest),
            _ => Err(ExecutionGuardError::InvalidResult(value.to_string())),
        }
    }

    pub fn validate(&self) -> Result<(), ExecutionGuardError> {
        Self::parse(self.as_str()).map(|_| ())
    }
}

/// Canonical execution request id used by Sprint 24–26 audit metadata.
pub fn execution_request_id_for_suggestion(suggestion_id: &str) -> String {
    format!("execution:{}", suggestion_id.trim())
}

/// Decide whether an execution request may proceed given derived outcomes.
///
/// Only a prior **Completed** outcome blocks. Failed/cancelled history does not
/// permanently block — retries remain allowed unless architecture defines
/// otherwise. Empty request ids are `InvalidRequest`.
pub fn evaluate_execution_guard(
    execution_request_id: &str,
    outcomes: &[ExecutionOutcome],
) -> ExecutionGuardResult {
    if execution_request_id.trim().is_empty() {
        return ExecutionGuardResult::InvalidRequest;
    }

    let canonical = execution_request_id.trim();
    for outcome in outcomes {
        if outcome.execution_request_id == canonical
            && outcome.status == ExecutionOutcomeStatus::Completed
        {
            return ExecutionGuardResult::AlreadyExecuted;
        }
    }

    ExecutionGuardResult::Allowed
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
    fn valid_result_allowed() {
        let result = evaluate_execution_guard("execution:s-1", &[]);
        assert_eq!(result, ExecutionGuardResult::Allowed);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn duplicate_result_when_completed() {
        let result = evaluate_execution_guard(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Completed)],
        );
        assert_eq!(result, ExecutionGuardResult::AlreadyExecuted);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn invalid_result_for_empty_request() {
        let result = evaluate_execution_guard("  ", &[]);
        assert_eq!(result, ExecutionGuardResult::InvalidRequest);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn failed_outcome_does_not_block() {
        let result = evaluate_execution_guard(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Failed)],
        );
        assert_eq!(result, ExecutionGuardResult::Allowed);
    }

    #[test]
    fn cancelled_outcome_does_not_block() {
        let result = evaluate_execution_guard(
            "execution:s-1",
            &[outcome("execution:s-1", ExecutionOutcomeStatus::Cancelled)],
        );
        assert_eq!(result, ExecutionGuardResult::Allowed);
    }

    #[test]
    fn unrelated_completed_does_not_block() {
        let result = evaluate_execution_guard(
            "execution:s-1",
            &[outcome("execution:other", ExecutionOutcomeStatus::Completed)],
        );
        assert_eq!(result, ExecutionGuardResult::Allowed);
    }

    #[test]
    fn parse_rejects_unknown() {
        assert_eq!(
            ExecutionGuardResult::parse("maybe"),
            Err(ExecutionGuardError::InvalidResult("maybe".into()))
        );
    }

    #[test]
    fn serializes_round_trip() {
        let result = ExecutionGuardResult::AlreadyExecuted;
        let json = serde_json::to_string(&result).unwrap();
        let restored: ExecutionGuardResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, restored);
    }
}
