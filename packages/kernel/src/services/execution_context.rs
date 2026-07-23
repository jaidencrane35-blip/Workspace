//! Audit-derived execution context service (Sprint 26).
//!
//! Consumes [`ExecutionOutcomeService`] and produces a deterministic
//! [`ExecutionContextSummary`] for workspace context enrichment. No direct
//! database access beyond the outcome service, no repositories, no mutation.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::ExecutionContextSummary;

use super::ExecutionOutcomeService;
use crate::error::{KernelError, Result};

/// Derives an execution outcome context summary from recent outcomes.
pub struct ExecutionContextService;

impl ExecutionContextService {
    /// Returns a deterministic summary of up to `limit` recent outcomes.
    pub fn summarize(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<ExecutionContextSummary> {
        let outcomes = ExecutionOutcomeService::list_recent(db, limit)?;
        let summary = ExecutionContextSummary::from_outcomes(&outcomes);

        summary
            .validate()
            .map_err(|error| KernelError::ExecutionContextValidation {
                message: error.to_string(),
            })?;

        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{
        ActorContext, ExecutionOutcome, ExecutionOutcomeStatus, IntentContext,
    };

    fn seed_executed(kernel: &WorkspaceKernel) {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Exec Context WS".into(),
        )
        .unwrap();
        for index in 0..4 {
            CommandHandler::create_zone(
                kernel,
                actor.clone(),
                intent.clone(),
                workspace.id.to_string(),
                format!("Zone {index}"),
                None,
            )
            .unwrap();
        }
        let suggestions = CommandHandler::get_suggestions(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();
        let suggestion_id = suggestions[0].id.clone();
        CommandHandler::accept_suggestion(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();
        CommandHandler::create_suggestion_intent_request(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();
        CommandHandler::execute_intent_request(
            kernel,
            actor,
            intent,
            workspace.id.to_string(),
            suggestion_id,
        )
        .unwrap();
    }

    #[test]
    fn completed_outcomes_counted() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        seed_executed(&kernel);

        let summary =
            ExecutionContextService::summarize(&kernel.shared_database(), 50).unwrap();

        assert!(summary.recent_completed_count >= 1);
        assert!(summary.last_execution_time.is_some());
        assert!(!summary.recent_commands.is_empty());
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn empty_history_handled() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let summary =
            ExecutionContextService::summarize(&kernel.shared_database(), 50).unwrap();

        assert_eq!(summary, ExecutionContextSummary::empty());
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn failed_outcomes_counted() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Failed Context".into(),
        )
        .unwrap();

        let _ = CommandHandler::execute_intent_request(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            "missing-suggestion".into(),
        );

        let summary =
            ExecutionContextService::summarize(&kernel.shared_database(), 50).unwrap();

        assert!(summary.recent_failed_count >= 1);
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn cancelled_outcomes_counted() {
        // Cancel emitter is deferred (Sprint 25); verify aggregation path.
        let outcomes = vec![ExecutionOutcome {
            execution_request_id: "execution:cancel".into(),
            status: ExecutionOutcomeStatus::Cancelled,
            command_name: "CreateZone".into(),
            completed_at: "2026-07-24T12:00:00Z".into(),
            success: false,
            failure_reason: None,
            suggestion_id: None,
            intent_id: None,
        }];

        let summary = ExecutionContextSummary::from_outcomes(&outcomes);
        assert_eq!(summary.recent_cancelled_count, 1);
        assert_eq!(summary.recent_completed_count, 0);
        assert_eq!(summary.recent_failed_count, 0);
        assert!(summary.validate().is_ok());
    }
}
