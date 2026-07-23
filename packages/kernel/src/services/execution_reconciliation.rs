//! Audit-derived execution reconciliation service (Sprint 29).
//!
//! Consumes [`ExecutionOutcomeService`] and produces an
//! [`ExecutionReconciliation`] for one execution request id. Read-only —
//! no repositories, mutation, or persistence.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{reconcile_execution_state, ExecutionReconciliation};

use super::ExecutionOutcomeService;
use crate::error::{KernelError, Result};

const OUTCOME_SCAN_LIMIT: usize = 200;

/// Reconciles outcome history into a current execution state interpretation.
pub struct ExecutionReconciliationService;

impl ExecutionReconciliationService {
    /// Returns the reconciled state for `execution_request_id`.
    pub fn reconcile(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
    ) -> Result<ExecutionReconciliation> {
        let execution_request_id = execution_request_id.trim();
        if execution_request_id.is_empty() {
            return Err(KernelError::ExecutionReconciliationValidation {
                message: "execution request id must not be empty".into(),
            });
        }

        let outcomes = ExecutionOutcomeService::list_recent(db, OUTCOME_SCAN_LIMIT)?;
        let reconciliation = reconcile_execution_state(execution_request_id, &outcomes);
        reconciliation
            .validate()
            .map_err(|error| KernelError::ExecutionReconciliationValidation {
                message: error.to_string(),
            })?;
        Ok(reconciliation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, ExecutionState, IntentContext};

    fn seed_failed(kernel: &WorkspaceKernel) -> String {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Reconcile Fail WS".into(),
        )
        .unwrap();
        let suggestion_id = "missing-suggestion".to_string();
        let _ = CommandHandler::execute_intent_request(
            kernel,
            actor,
            intent,
            workspace.id.to_string(),
            suggestion_id.clone(),
        );
        format!("execution:{suggestion_id}")
    }

    fn seed_completed(kernel: &WorkspaceKernel) -> String {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Reconcile Done WS".into(),
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
            suggestion_id.clone(),
        )
        .unwrap();
        format!("execution:{suggestion_id}")
    }

    fn seed_cancelled(kernel: &WorkspaceKernel) -> String {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Reconcile Cancel WS".into(),
        )
        .unwrap();
        let suggestion_id = "missing-for-cancel".to_string();
        let _ = CommandHandler::execute_intent_request(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id.clone(),
        );
        let execution_request_id = format!("execution:{suggestion_id}");
        CommandHandler::request_execution_cancellation(
            kernel,
            actor,
            intent,
            workspace.id.to_string(),
            execution_request_id.clone(),
            "stop".into(),
        )
        .unwrap();
        execution_request_id
    }

    #[test]
    fn unknown_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let result = ExecutionReconciliationService::reconcile(
            &kernel.shared_database(),
            "execution:never-seen",
        )
        .unwrap();
        assert_eq!(result.current_state, ExecutionState::Unknown);
        assert!(!result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn completed_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let id = seed_completed(&kernel);
        let result =
            ExecutionReconciliationService::reconcile(&kernel.shared_database(), &id).unwrap();
        assert_eq!(result.current_state, ExecutionState::Completed);
        assert!(!result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }

    #[test]
    fn failed_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let id = seed_failed(&kernel);
        let result =
            ExecutionReconciliationService::reconcile(&kernel.shared_database(), &id).unwrap();
        assert_eq!(result.current_state, ExecutionState::Failed);
        assert!(result.dispatch_allowed);
        assert!(result.cancellation_allowed);
    }

    #[test]
    fn cancelled_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let id = seed_cancelled(&kernel);
        let result =
            ExecutionReconciliationService::reconcile(&kernel.shared_database(), &id).unwrap();
        assert_eq!(result.current_state, ExecutionState::Cancelled);
        assert!(result.dispatch_allowed);
        assert!(!result.cancellation_allowed);
    }
}
