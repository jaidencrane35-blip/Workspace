//! Durable lifecycle-first execution reconciliation service.
//!
//! Consumes [`ExecutionOutcomeService`] and produces an
//! [`ExecutionReconciliation`] for one execution request id. Read-only —
//! Audit outcomes remain fallback evidence for legacy identities.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{reconcile_execution_state, reconcile_execution_states, ExecutionReconciliation};

use super::{ExecutionLifecycleService, ExecutionOutcomeService};
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

        if let Some(record) = ExecutionLifecycleService::get(db, execution_request_id)? {
            let reconciliation = ExecutionLifecycleService::reconciliation(&record);
            reconciliation.validate().map_err(|error| {
                KernelError::ExecutionReconciliationValidation {
                    message: error.to_string(),
                }
            })?;
            return Ok(reconciliation);
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

    /// Returns up to `limit` reconciled execution states, most recently seen first.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<ExecutionReconciliation>> {
        let lifecycle = ExecutionLifecycleService::list_recent(db, 500)?;
        let mut seen: std::collections::HashSet<_> = lifecycle
            .iter()
            .map(|record| record.execution_request_id.clone())
            .collect();
        let mut states: Vec<_> = lifecycle
            .iter()
            .map(|record| {
                (
                    record.updated_at.clone(),
                    ExecutionLifecycleService::reconciliation(record),
                )
            })
            .collect();
        let outcomes = ExecutionOutcomeService::list_recent(db, OUTCOME_SCAN_LIMIT)?;
        for state in reconcile_execution_states(&outcomes) {
            if !seen.insert(state.execution_request_id.clone()) {
                continue;
            }
            let occurred_at = outcomes
                .iter()
                .filter(|outcome| outcome.execution_request_id == state.execution_request_id)
                .map(|outcome| outcome.completed_at.as_str())
                .max()
                .unwrap_or_default()
                .to_string();
            states.push((occurred_at, state));
        }
        states.sort_by(|(left_at, left), (right_at, right)| {
            right_at.cmp(left_at).then_with(|| {
                left.execution_request_id
                    .cmp(&right.execution_request_id)
            })
        });
        states.truncate(limit.max(1));
        let states: Vec<_> = states.into_iter().map(|(_, state)| state).collect();
        for state in &states {
            state.validate().map_err(|error| {
                KernelError::ExecutionReconciliationValidation {
                    message: error.to_string(),
                }
            })?;
        }
        Ok(states)
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

    #[test]
    fn list_recent_returns_multiple_states() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let _ = seed_failed(&kernel);
        let _ = seed_completed(&kernel);

        let states =
            ExecutionReconciliationService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(states.len() >= 2);
        assert!(states
            .iter()
            .any(|s| s.current_state == ExecutionState::Failed));
        assert!(states
            .iter()
            .any(|s| s.current_state == ExecutionState::Completed));
        assert!(states.iter().all(|s| s.validate().is_ok()));
    }

    #[test]
    fn list_recent_empty_history() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let states =
            ExecutionReconciliationService::list_recent(&kernel.shared_database(), 50).unwrap();
        assert!(states.is_empty());
    }
}
