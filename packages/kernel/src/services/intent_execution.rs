//! Governed suggestion execution boundary service (Sprint 24).
//!
//! Validates that a [`SuggestionIntentRequest`] bridge exists in audit history,
//! confirms suggestion approval, validates the mapped action intent, and
//! prepares an [`IntentExecutionRequest`]. Does not execute commands or bypass
//! the pipeline — dispatch remains the command layer's responsibility.
//!
//! Distinct from [`crate::IntentExecutionService`] (Sprint 15), which validates
//! action-intent metadata before commands run. This service prepares
//! suggestion-derived execution requests only.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    ActionIntentRequest, ActorType, IntentExecutionError, IntentExecutionRequest,
    SuggestionIntentError,
};

use super::AuditService;
use crate::error::{KernelError, Result};
use crate::CommandIntentMapping;
use crate::IntentExecutionService as ActionIntentValidator;

const MAX_AUDIT_SCAN: usize = 500;

/// Prepared execution boundary — validated and authorized, not yet dispatched.
#[derive(Debug)]
pub struct PreparedIntentExecution {
    pub action_intent: ActionIntentRequest,
    pub command_name: &'static str,
    pub execution_request: IntentExecutionRequest,
}

/// Orchestrates governed execution request preparation from audit-derived bridges.
///
/// Named distinctly from Sprint 15 [`crate::IntentExecutionService`] (action-intent
/// validation) to avoid conflating metadata validation with suggestion execution.
pub struct GovernedIntentExecutionService;

impl GovernedIntentExecutionService {
    /// Validates audit history and prepares an authorized execution request.
    pub fn prepare(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
        actor_type: ActorType,
        actor_id: Option<String>,
    ) -> Result<PreparedIntentExecution> {
        if suggestion_id.trim().is_empty() {
            return Err(map_execution_error(IntentExecutionError::EmptySuggestionId));
        }

        if Self::is_rejected(db, suggestion_id)? {
            return Err(map_suggestion_error(SuggestionIntentError::Rejected(
                suggestion_id.to_string(),
            )));
        }

        if Self::find_accepted(db, suggestion_id)?.is_none() {
            return Err(map_suggestion_error(SuggestionIntentError::NotAccepted(
                suggestion_id.to_string(),
            )));
        }

        let bridge = Self::find_intent_bridge(db, suggestion_id)?.ok_or_else(|| {
            KernelError::IntentExecutionValidation {
                message: format!(
                    "no suggestion intent bridge found in audit for suggestion '{suggestion_id}'"
                ),
            }
        })?;

        let mut action_intent = ActionIntentRequest::new(bridge.intent_id.clone());
        if let Some(resource_ref) = bridge.resource_ref {
            action_intent = action_intent.with_target(resource_ref);
        }

        ActionIntentValidator::validate_request(&action_intent)?;
        let command_name = CommandIntentMapping::resolve_command_name(&bridge.intent_id)?;

        let execution_id = format!("execution:{suggestion_id}");
        let mut execution_request = IntentExecutionRequest::new(
            execution_id,
            suggestion_id.to_string(),
            bridge.intent_id,
            actor_type,
            actor_id,
        );

        execution_request = execution_request
            .authorize()
            .map_err(map_execution_error)?;

        execution_request
            .validate()
            .map_err(map_execution_error)?;

        Ok(PreparedIntentExecution {
            action_intent,
            command_name,
            execution_request,
        })
    }

    fn find_intent_bridge(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
    ) -> Result<Option<IntentBridgeRecord>> {
        for record in AuditService::list_recent(db, MAX_AUDIT_SCAN)? {
            if record.command_name.as_deref() != Some("CreateSuggestionIntentRequest")
                || !record.success
            {
                continue;
            }
            let Some(metadata) = record.metadata.as_deref() else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) else {
                continue;
            };
            if value.get("suggestion_id").and_then(|v| v.as_str()) != Some(suggestion_id) {
                continue;
            }
            if value.get("bridge").and_then(|v| v.as_str()) != Some("intent_request_created") {
                continue;
            }
            let Some(intent_id_str) = value.get("intent_id").and_then(|v| v.as_str()) else {
                continue;
            };
            let intent_id = workspace_domain::ActionIntentId::new(intent_id_str).map_err(|_| {
                KernelError::IntentExecutionValidation {
                    message: format!("invalid intent_id in audit bridge: {intent_id_str}"),
                }
            })?;

            return Ok(Some(IntentBridgeRecord {
                intent_id,
                resource_ref: record
                    .resource_ref
                    .as_deref()
                    .and_then(workspace_domain::parse_canonical_resource_ref),
            }));
        }
        Ok(None)
    }

    fn find_accepted(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
    ) -> Result<Option<()>> {
        for record in AuditService::list_recent(db, MAX_AUDIT_SCAN)? {
            if record.command_name.as_deref() != Some("AcceptSuggestion") || !record.success {
                continue;
            }
            let Some(metadata) = record.metadata.as_deref() else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) else {
                continue;
            };
            if value.get("suggestion_id").and_then(|v| v.as_str()) == Some(suggestion_id) {
                return Ok(Some(()));
            }
        }
        Ok(None)
    }

    fn is_rejected(db: &Arc<Mutex<Database>>, suggestion_id: &str) -> Result<bool> {
        for record in AuditService::list_recent(db, MAX_AUDIT_SCAN)? {
            if record.command_name.as_deref() != Some("RejectSuggestion") || !record.success {
                continue;
            }
            let Some(metadata) = record.metadata.as_deref() else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) else {
                continue;
            };
            if value.get("suggestion_id").and_then(|v| v.as_str()) == Some(suggestion_id) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

struct IntentBridgeRecord {
    intent_id: workspace_domain::ActionIntentId,
    resource_ref: Option<workspace_domain::ResourceRef>,
}

fn map_execution_error(error: IntentExecutionError) -> KernelError {
    KernelError::IntentExecutionValidation {
        message: error.to_string(),
    }
}

fn map_suggestion_error(error: SuggestionIntentError) -> KernelError {
    KernelError::SuggestionIntentValidation {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, IntentContext, IntentExecutionStatus};

    fn seed_through_intent_bridge(kernel: &WorkspaceKernel) -> (String, String) {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Execution WS".into(),
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
            actor,
            intent,
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();

        (workspace.id.to_string(), suggestion_id)
    }

    #[test]
    fn approved_suggestion_with_bridge_prepares_execution_request() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let (_, suggestion_id) = seed_through_intent_bridge(&kernel);

        let prepared = GovernedIntentExecutionService::prepare(
            &kernel.shared_database(),
            &suggestion_id,
            ActorType::LocalUser,
            Some("local-user".into()),
        )
        .unwrap();

        assert_eq!(prepared.command_name, "CreateZone");
        assert_eq!(
            prepared.execution_request.status,
            IntentExecutionStatus::Authorized
        );
        assert_eq!(prepared.execution_request.suggestion_id, suggestion_id);
    }

    #[test]
    fn rejected_suggestion_fails_prepare() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Rejected".into(),
        )
        .unwrap();
        for index in 0..4 {
            CommandHandler::create_zone(
                &kernel,
                actor.clone(),
                intent.clone(),
                workspace.id.to_string(),
                format!("Zone {index}"),
                None,
            )
            .unwrap();
        }
        let suggestions = CommandHandler::get_suggestions(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();
        let suggestion_id = suggestions[0].id.clone();

        CommandHandler::reject_suggestion(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();

        let error = GovernedIntentExecutionService::prepare(
            &kernel.shared_database(),
            &suggestion_id,
            ActorType::LocalUser,
            None,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            KernelError::SuggestionIntentValidation { .. }
        ));
    }

    #[test]
    fn pending_suggestion_without_bridge_fails_prepare() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Pending".into(),
        )
        .unwrap();
        for index in 0..4 {
            CommandHandler::create_zone(
                &kernel,
                actor.clone(),
                intent.clone(),
                workspace.id.to_string(),
                format!("Zone {index}"),
                None,
            )
            .unwrap();
        }
        let suggestions = CommandHandler::get_suggestions(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();
        let suggestion_id = suggestions[0].id.clone();

        let error = GovernedIntentExecutionService::prepare(
            &kernel.shared_database(),
            &suggestion_id,
            ActorType::LocalUser,
            None,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            KernelError::SuggestionIntentValidation { .. }
                | KernelError::IntentExecutionValidation { .. }
        ));
    }
}
