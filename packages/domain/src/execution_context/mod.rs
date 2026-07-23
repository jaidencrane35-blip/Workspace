//! Deterministic execution outcome context — pure domain aggregation (Sprint 26).
//!
//! [`ExecutionContextSummary`] answers "what happened after previous executions?"
//! by counting and listing recent [`ExecutionOutcome`] records. Pure aggregation
//! only — no scoring, interpretation, recommendations, AI, or persistence.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::execution_outcome::{ExecutionOutcome, ExecutionOutcomeStatus};

/// Execution-context-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionContextError {
    #[error("Execution context last_execution_time must not be empty when present")]
    EmptyLastExecutionTime,

    #[error(
        "Execution context counts ({sum}) do not match recent_commands length ({command_count})"
    )]
    InconsistentCounts {
        sum: usize,
        command_count: usize,
    },

    #[error("Execution context last_execution_time must be set when outcomes exist")]
    MissingLastExecutionTime,

    #[error("Execution context last_execution_time must be absent when no outcomes exist")]
    UnexpectedLastExecutionTime,

    #[error("Execution context recent_commands entry must not be empty")]
    EmptyCommandName,
}

/// Deterministic summary of recent execution outcomes for workspace context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextSummary {
    pub recent_completed_count: usize,
    pub recent_failed_count: usize,
    pub recent_cancelled_count: usize,
    /// RFC 3339 timestamp of the most recent outcome, if any.
    pub last_execution_time: Option<String>,
    /// Command names from recent outcomes, most recent first.
    pub recent_commands: Vec<String>,
}

impl ExecutionContextSummary {
    /// Empty summary — valid baseline when no execution history exists.
    pub fn empty() -> Self {
        Self {
            recent_completed_count: 0,
            recent_failed_count: 0,
            recent_cancelled_count: 0,
            last_execution_time: None,
            recent_commands: vec![],
        }
    }

    /// Deterministically aggregates outcomes into an execution context summary.
    ///
    /// Outcomes are expected most-recent-first (as produced by
    /// [`crate::execution_outcome`] consumers). No scoring or ranking.
    pub fn from_outcomes(outcomes: &[ExecutionOutcome]) -> Self {
        let mut recent_completed_count = 0;
        let mut recent_failed_count = 0;
        let mut recent_cancelled_count = 0;
        let mut recent_commands = Vec::with_capacity(outcomes.len());

        for outcome in outcomes {
            match outcome.status {
                ExecutionOutcomeStatus::Completed => recent_completed_count += 1,
                ExecutionOutcomeStatus::Failed => recent_failed_count += 1,
                ExecutionOutcomeStatus::Cancelled => recent_cancelled_count += 1,
            }
            recent_commands.push(outcome.command_name.clone());
        }

        let last_execution_time = outcomes
            .first()
            .map(|outcome| outcome.completed_at.clone());

        Self {
            recent_completed_count,
            recent_failed_count,
            recent_cancelled_count,
            last_execution_time,
            recent_commands,
        }
    }

    pub fn validate(&self) -> Result<(), ExecutionContextError> {
        let sum = self
            .recent_completed_count
            .saturating_add(self.recent_failed_count)
            .saturating_add(self.recent_cancelled_count);

        if sum != self.recent_commands.len() {
            return Err(ExecutionContextError::InconsistentCounts {
                sum,
                command_count: self.recent_commands.len(),
            });
        }

        for command in &self.recent_commands {
            if command.trim().is_empty() {
                return Err(ExecutionContextError::EmptyCommandName);
            }
        }

        match (sum, &self.last_execution_time) {
            (0, None) => Ok(()),
            (0, Some(_)) => Err(ExecutionContextError::UnexpectedLastExecutionTime),
            (_, None) => Err(ExecutionContextError::MissingLastExecutionTime),
            (_, Some(timestamp)) if timestamp.trim().is_empty() => {
                Err(ExecutionContextError::EmptyLastExecutionTime)
            }
            (_, Some(_)) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_outcome::ExecutionOutcomeStatus;

    fn outcome(
        status: ExecutionOutcomeStatus,
        command: &str,
        completed_at: &str,
    ) -> ExecutionOutcome {
        ExecutionOutcome {
            execution_request_id: "execution:test".into(),
            status,
            command_name: command.into(),
            completed_at: completed_at.into(),
            success: matches!(status, ExecutionOutcomeStatus::Completed),
            failure_reason: None,
            suggestion_id: None,
            intent_id: None,
        }
    }

    #[test]
    fn empty_context_validates() {
        let summary = ExecutionContextSummary::empty();
        assert!(summary.validate().is_ok());
        assert_eq!(summary.recent_completed_count, 0);
        assert_eq!(summary.recent_failed_count, 0);
        assert_eq!(summary.recent_cancelled_count, 0);
        assert!(summary.last_execution_time.is_none());
        assert!(summary.recent_commands.is_empty());
    }

    #[test]
    fn counts_aggregate_correctly() {
        let summary = ExecutionContextSummary::from_outcomes(&[
            outcome(
                ExecutionOutcomeStatus::Completed,
                "CreateZone",
                "2026-07-24T12:00:00Z",
            ),
            outcome(
                ExecutionOutcomeStatus::Failed,
                "CreateZone",
                "2026-07-24T11:00:00Z",
            ),
            outcome(
                ExecutionOutcomeStatus::Cancelled,
                "CreateApplication",
                "2026-07-24T10:00:00Z",
            ),
            outcome(
                ExecutionOutcomeStatus::Completed,
                "CreateWidget",
                "2026-07-24T09:00:00Z",
            ),
        ]);

        assert_eq!(summary.recent_completed_count, 2);
        assert_eq!(summary.recent_failed_count, 1);
        assert_eq!(summary.recent_cancelled_count, 1);
        assert_eq!(
            summary.last_execution_time.as_deref(),
            Some("2026-07-24T12:00:00Z")
        );
        assert_eq!(
            summary.recent_commands,
            vec![
                "CreateZone",
                "CreateZone",
                "CreateApplication",
                "CreateWidget"
            ]
        );
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn rejects_inconsistent_counts() {
        let mut summary = ExecutionContextSummary::empty();
        summary.recent_completed_count = 1;
        assert_eq!(
            summary.validate(),
            Err(ExecutionContextError::InconsistentCounts {
                sum: 1,
                command_count: 0,
            })
        );
    }

    #[test]
    fn rejects_missing_last_execution_time() {
        let mut summary = ExecutionContextSummary::from_outcomes(&[outcome(
            ExecutionOutcomeStatus::Completed,
            "CreateZone",
            "2026-07-24T12:00:00Z",
        )]);
        summary.last_execution_time = None;
        assert_eq!(
            summary.validate(),
            Err(ExecutionContextError::MissingLastExecutionTime)
        );
    }

    #[test]
    fn rejects_unexpected_last_execution_time() {
        let mut summary = ExecutionContextSummary::empty();
        summary.last_execution_time = Some("2026-07-24T12:00:00Z".into());
        assert_eq!(
            summary.validate(),
            Err(ExecutionContextError::UnexpectedLastExecutionTime)
        );
    }

    #[test]
    fn rejects_empty_command_name() {
        let mut summary = ExecutionContextSummary::from_outcomes(&[outcome(
            ExecutionOutcomeStatus::Completed,
            "CreateZone",
            "2026-07-24T12:00:00Z",
        )]);
        summary.recent_commands = vec![String::new()];
        assert_eq!(
            summary.validate(),
            Err(ExecutionContextError::EmptyCommandName)
        );
    }

    #[test]
    fn serializes_round_trip() {
        let summary = ExecutionContextSummary::from_outcomes(&[outcome(
            ExecutionOutcomeStatus::Failed,
            "CreateZone",
            "2026-07-24T12:00:00Z",
        )]);
        let json = serde_json::to_string(&summary).unwrap();
        let restored: ExecutionContextSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, restored);
    }
}
