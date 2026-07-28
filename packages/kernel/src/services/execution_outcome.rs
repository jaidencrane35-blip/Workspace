//! Audit-derived execution outcome service (Sprint 25).
//!
//! Derives [`ExecutionOutcome`] records from existing ExecuteIntentRequest audit
//! events. No persistence, mutation, or inference.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{outcome_from_audit_event, ExecutionOutcome};

use super::{AuditService, ExecutionLifecycleService};
use crate::error::{KernelError, Result};

const MAX_AUDIT_SCAN: usize = 500;

/// Derives execution outcomes from persisted audit history.
pub struct ExecutionOutcomeService;

impl ExecutionOutcomeService {
    /// Returns up to `limit` recent outcomes, most recent first.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<ExecutionOutcome>> {
        let scan = limit.saturating_mul(4).clamp(limit.max(1), MAX_AUDIT_SCAN);
        let audit_events = AuditService::list_recent(db, scan)?;

        let mut outcomes = Vec::new();
        let mut durable_completed = std::collections::HashSet::new();
        for record in ExecutionLifecycleService::list_recent(db, limit)? {
            if let Some(outcome) = ExecutionLifecycleService::completed_outcome(&record)? {
                durable_completed.insert(outcome.execution_request_id.clone());
                outcomes.push(outcome);
            }
        }
        for event in audit_events {
            let Some(outcome) = outcome_from_audit_event(&event) else {
                continue;
            };
            if durable_completed.contains(&outcome.execution_request_id) {
                continue;
            }

            outcome
                .validate()
                .map_err(|error| KernelError::ExecutionOutcomeValidation {
                    message: error.to_string(),
                })?;

            outcomes.push(outcome);

            if outcomes.len() >= limit {
                break;
            }
        }

        outcomes.truncate(limit.max(1));
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, ExecutionOutcomeStatus, IntentContext};

    fn seed_executed(kernel: &WorkspaceKernel) -> (String, String) {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Outcome WS".into(),
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
        (workspace.id.to_string(), suggestion_id)
    }

    #[test]
    fn derives_completed_outcomes_from_successful_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let (_, suggestion_id) = seed_executed(&kernel);

        let outcomes =
            ExecutionOutcomeService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(outcomes.iter().any(|outcome| {
            outcome.suggestion_id.as_deref() == Some(suggestion_id.as_str())
                && outcome.status == ExecutionOutcomeStatus::Completed
                && outcome.success
        }));
        assert!(outcomes.iter().all(|outcome| outcome.validate().is_ok()));
    }

    #[test]
    fn derives_failed_outcomes_from_failed_execution() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Failed Outcome".into(),
        )
        .unwrap();

        let _ = CommandHandler::execute_intent_request(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            "missing-suggestion".into(),
        );

        let outcomes =
            ExecutionOutcomeService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(outcomes.iter().any(|outcome| {
            outcome.status == ExecutionOutcomeStatus::Failed
                && outcome.suggestion_id.as_deref() == Some("missing-suggestion")
                && !outcome.success
        }));
    }

    #[test]
    fn excludes_unrelated_audit_events() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        CommandHandler::create_workspace(&kernel, actor, intent, "Unrelated".into()).unwrap();

        let outcomes =
            ExecutionOutcomeService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(outcomes.is_empty());
    }
}
