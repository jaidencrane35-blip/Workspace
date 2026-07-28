//! Durable lifecycle-first execution idempotency guard.
//!
//! Consumes [`ExecutionOutcomeService`] history to decide whether an
//! `ExecuteIntentRequest` may dispatch. Read-only — no repositories, mutation,
//! Audit is a fallback for legacy execution identities without lifecycle rows.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    evaluate_execution_guard, execution_request_id_for_suggestion, ExecutionGuardResult,
};

use super::{ExecutionLifecycleService, ExecutionOutcomeService};
use crate::error::{KernelError, Result};

const OUTCOME_SCAN_LIMIT: usize = 200;

/// Guards governed execution against duplicate successful dispatch.
pub struct ExecutionGuardService;

impl ExecutionGuardService {
    /// Evaluates whether `suggestion_id` may be executed again.
    pub fn evaluate(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
    ) -> Result<ExecutionGuardResult> {
        if suggestion_id.trim().is_empty() {
            return Ok(ExecutionGuardResult::InvalidRequest);
        }

        let execution_request_id = execution_request_id_for_suggestion(suggestion_id);
        if let Some(record) = ExecutionLifecycleService::get(db, &execution_request_id)? {
            let reconciliation = ExecutionLifecycleService::reconciliation(&record);
            if reconciliation.dispatch_allowed {
                return Ok(ExecutionGuardResult::Allowed);
            }
            return match record.state {
                workspace_domain::ExecutionState::Completed => {
                    Ok(ExecutionGuardResult::AlreadyExecuted)
                }
                workspace_domain::ExecutionState::InProgress => {
                    Err(KernelError::ExecutionInProgress {
                        execution_request_id,
                    })
                }
                workspace_domain::ExecutionState::Failed => {
                    Err(KernelError::ExecutionReconciliationRequired {
                        execution_request_id,
                    })
                }
                state => Err(KernelError::IntegrityViolation {
                    message: format!(
                        "invalid durable execution lifecycle state: {}",
                        state.as_str()
                    ),
                }),
            };
        }
        let outcomes = ExecutionOutcomeService::list_recent(db, OUTCOME_SCAN_LIMIT)?;
        Ok(evaluate_execution_guard(&execution_request_id, &outcomes))
    }

    /// Returns `Ok(())` when dispatch may proceed; otherwise a kernel error.
    pub fn ensure_allowed(db: &Arc<Mutex<Database>>, suggestion_id: &str) -> Result<()> {
        let execution_request_id = execution_request_id_for_suggestion(suggestion_id);
        match Self::evaluate(db, suggestion_id)? {
            ExecutionGuardResult::Allowed => Ok(()),
            ExecutionGuardResult::AlreadyExecuted => Err(KernelError::DuplicateExecution {
                execution_request_id,
            }),
            ExecutionGuardResult::InvalidRequest => Err(KernelError::ExecutionGuardValidation {
                message: "execution request suggestion id must not be empty".into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, IntentContext};

    fn seed_successful_execution(kernel: &WorkspaceKernel) -> String {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Guard WS".into(),
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
        suggestion_id
    }

    #[test]
    fn unseen_request_allowed() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let result =
            ExecutionGuardService::evaluate(&kernel.shared_database(), "never-seen").unwrap();
        assert_eq!(result, ExecutionGuardResult::Allowed);
        ExecutionGuardService::ensure_allowed(&kernel.shared_database(), "never-seen").unwrap();
    }

    #[test]
    fn executed_request_blocked() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let suggestion_id = seed_successful_execution(&kernel);

        let result =
            ExecutionGuardService::evaluate(&kernel.shared_database(), &suggestion_id).unwrap();
        assert_eq!(result, ExecutionGuardResult::AlreadyExecuted);

        let err = ExecutionGuardService::ensure_allowed(&kernel.shared_database(), &suggestion_id)
            .unwrap_err();
        assert!(matches!(err, KernelError::DuplicateExecution { .. }));
    }

    #[test]
    fn failed_execution_does_not_permanently_block() {
        // Documented semantics: failures remain retryable — only Completed blocks.
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Failed Guard".into(),
        )
        .unwrap();

        let _ = CommandHandler::execute_intent_request(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            "missing-suggestion".into(),
        );

        let result =
            ExecutionGuardService::evaluate(&kernel.shared_database(), "missing-suggestion")
                .unwrap();
        assert_eq!(
            result,
            ExecutionGuardResult::Allowed,
            "failed executions must remain retryable"
        );
    }

    #[test]
    fn empty_suggestion_is_invalid() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let result = ExecutionGuardService::evaluate(&kernel.shared_database(), "  ").unwrap();
        assert_eq!(result, ExecutionGuardResult::InvalidRequest);
    }
}
