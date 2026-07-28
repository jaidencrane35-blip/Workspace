//! Governed execution cancellation service (Sprint 28).
//!
//! Validates cancellation requests against existing audit-derived outcomes.
//! Produces a [`CancellationRequest`] decision only — no persistence, command
//! dispatch, or runtime interruption.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    CancellationRequest, CancellationStatus, ExecutionOutcomeStatus,
};

use super::{ExecutionLifecycleService, ExecutionOutcomeService};
use crate::error::{KernelError, Result};

const OUTCOME_SCAN_LIMIT: usize = 200;

/// Orchestrates cancellation-request validation against execution history.
pub struct ExecutionCancellationService;

impl ExecutionCancellationService {
    /// Builds a validated cancellation request for a known execution identity.
    ///
    /// Rules:
    /// - unknown execution → rejected
    /// - completed execution → rejected explicitly (nothing left to cancel)
    /// - already cancelled → rejected
    /// - otherwise → `CancellationStatus::Requested` (no runtime stop)
    pub(crate) fn prepare_request(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
        requested_by: &str,
        reason: &str,
    ) -> Result<CancellationRequest> {
        let execution_request_id = execution_request_id.trim();
        if execution_request_id.is_empty() {
            return Err(KernelError::ExecutionCancellationValidation {
                message: "execution request id must not be empty".into(),
            });
        }
        if requested_by.trim().is_empty() {
            return Err(KernelError::ExecutionCancellationValidation {
                message: "requested_by must not be empty".into(),
            });
        }

        if let Some(record) = ExecutionLifecycleService::get(db, execution_request_id)? {
            return match record.state {
                workspace_domain::ExecutionState::Completed => {
                    Err(KernelError::CannotCancelCompletedExecution {
                        execution_request_id: execution_request_id.to_string(),
                    })
                }
                workspace_domain::ExecutionState::InProgress => {
                    Err(KernelError::ExecutionInProgress {
                        execution_request_id: execution_request_id.to_string(),
                    })
                }
                workspace_domain::ExecutionState::Cancelled => {
                    Err(KernelError::ExecutionCancellationValidation {
                        message: format!(
                            "execution request '{execution_request_id}' is already cancelled"
                        ),
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
        let related: Vec<_> = outcomes
            .iter()
            .filter(|outcome| outcome.execution_request_id == execution_request_id)
            .collect();

        if related.is_empty() {
            return Err(KernelError::UnknownExecutionRequest {
                execution_request_id: execution_request_id.to_string(),
            });
        }

        if related
            .iter()
            .any(|outcome| outcome.status == ExecutionOutcomeStatus::Completed)
        {
            return Err(KernelError::CannotCancelCompletedExecution {
                execution_request_id: execution_request_id.to_string(),
            });
        }

        if related
            .iter()
            .any(|outcome| outcome.status == ExecutionOutcomeStatus::Cancelled)
        {
            return Err(KernelError::ExecutionCancellationValidation {
                message: format!(
                    "execution request '{execution_request_id}' is already cancelled"
                ),
            });
        }

        let request =
            CancellationRequest::new(execution_request_id, requested_by.trim(), reason);
        request
            .validate()
            .map_err(|error| KernelError::ExecutionCancellationValidation {
                message: error.to_string(),
            })?;
        if request.status != CancellationStatus::Requested {
            return Err(KernelError::IntegrityViolation {
                message: "new cancellation request did not enter requested state".into(),
            });
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, IntentContext};

    fn seed_failed_execution(kernel: &WorkspaceKernel) -> (String, String) {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Cancel Svc WS".into(),
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
        (
            workspace.id.to_string(),
            format!("execution:{suggestion_id}"),
        )
    }

    fn seed_completed_execution(kernel: &WorkspaceKernel) -> String {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Cancel Completed WS".into(),
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

    #[test]
    fn known_execution_can_request_cancellation() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let (_, execution_request_id) = seed_failed_execution(&kernel);

        let request = ExecutionCancellationService::prepare_request(
            &kernel.shared_database(),
            &execution_request_id,
            "local-user",
            "operator stop",
        )
        .unwrap();

        assert_eq!(request.execution_request_id, execution_request_id);
        assert_eq!(request.status, CancellationStatus::Requested);
        assert_eq!(request.reason, "operator stop");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn unknown_execution_rejected() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let err = ExecutionCancellationService::prepare_request(
            &kernel.shared_database(),
            "execution:never-seen",
            "local-user",
            "n/a",
        )
        .unwrap_err();
        assert!(matches!(
            err,
            KernelError::UnknownExecutionRequest { .. }
        ));
    }

    #[test]
    fn completed_execution_cancellation_handled_explicitly() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let execution_request_id = seed_completed_execution(&kernel);

        let err = ExecutionCancellationService::prepare_request(
            &kernel.shared_database(),
            &execution_request_id,
            "local-user",
            "too late",
        )
        .unwrap_err();

        assert!(matches!(
            err,
            KernelError::CannotCancelCompletedExecution { .. }
        ));
    }
}
