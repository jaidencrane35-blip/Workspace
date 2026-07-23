//! Approval-gated suggestion → intent bridge service (Sprint 23).
//!
//! Orchestrates creation of [`SuggestionIntentRequest`] records from accepted
//! suggestions. Validates mapped action intents but never executes commands.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use workspace_database::Database;
use workspace_domain::{
    map_suggestion_type_to_intent, ActionIntentRequest, ActorType,
    SuggestionIntentError, SuggestionIntentRequest, SuggestionType,
};

use super::AuditService;
use crate::error::{KernelError, Result};
use crate::IntentExecutionService;

const MAX_AUDIT_SCAN: usize = 500;

/// Resolved acceptance metadata from the audit trail.
struct AcceptedSuggestionRecord {
    suggestion_type: SuggestionType,
    resource_ref: Option<workspace_domain::ResourceRef>,
}

/// Orchestrates approval-gated intent request creation. No persistence, no execution.
pub struct SuggestionIntentService;

impl SuggestionIntentService {
    /// Creates an intent bridge request for an accepted suggestion.
    ///
    /// Verifies acceptance via audit, maps suggestion type to intent, validates
    /// the resulting action intent shape, and returns the bridge record.
    pub fn create_request(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
        actor_type: ActorType,
        actor_id: Option<String>,
    ) -> Result<SuggestionIntentRequest> {
        if suggestion_id.trim().is_empty() {
            return Err(map_intent_error(SuggestionIntentError::EmptySuggestionId));
        }

        if Self::is_rejected(db, suggestion_id)? {
            return Err(map_intent_error(SuggestionIntentError::Rejected(
                suggestion_id.to_string(),
            )));
        }

        let accepted = Self::find_accepted(db, suggestion_id)?.ok_or_else(|| {
            map_intent_error(SuggestionIntentError::NotAccepted(
                suggestion_id.to_string(),
            ))
        })?;

        let intent_id = map_suggestion_type_to_intent(accepted.suggestion_type)
            .map_err(map_intent_error)?;

        let mut action_request = ActionIntentRequest::new(intent_id.clone());
        if let Some(resource_ref) = accepted.resource_ref.clone() {
            action_request = action_request.with_target(resource_ref.clone());
        }

        IntentExecutionService::validate_request(&action_request)?;

        let metadata = Some(
            serde_json::json!({
                "suggestion_id": suggestion_id,
                "suggestion_type": accepted.suggestion_type,
                "intent_id": intent_id.as_str(),
                "bridge": "intent_request_created",
                "actor_id": actor_id,
            })
            .to_string(),
        );

        let request = SuggestionIntentRequest {
            suggestion_id: suggestion_id.to_string(),
            intent_id,
            resource_ref: accepted.resource_ref,
            created_at: Utc::now().to_rfc3339(),
            actor_type,
            metadata,
        };

        request
            .validate()
            .map_err(map_intent_error)?;

        Ok(request)
    }

    fn find_accepted(
        db: &Arc<Mutex<Database>>,
        suggestion_id: &str,
    ) -> Result<Option<AcceptedSuggestionRecord>> {
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
            if value.get("suggestion_id").and_then(|v| v.as_str()) != Some(suggestion_id) {
                continue;
            }
            let Some(type_str) = value.get("suggestion_type").and_then(|v| v.as_str()) else {
                continue;
            };
            let suggestion_type: SuggestionType =
                serde_json::from_value(serde_json::Value::String(type_str.to_string()))
                    .map_err(|error| KernelError::SuggestionIntentValidation {
                        message: format!("invalid suggestion_type in audit: {error}"),
                    })?;

            return Ok(Some(AcceptedSuggestionRecord {
                suggestion_type,
                resource_ref: record
                    .resource_ref
                    .as_deref()
                    .and_then(workspace_domain::parse_canonical_resource_ref),
            }));
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

fn map_intent_error(error: SuggestionIntentError) -> KernelError {
    KernelError::SuggestionIntentValidation {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{ActorContext, IntentContext, SuggestionType};

    fn seed_workspace_with_suggestion(kernel: &WorkspaceKernel) -> (String, String) {
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            kernel,
            actor.clone(),
            intent.clone(),
            "Intent Bridge WS".into(),
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
            actor,
            intent,
            workspace.id.to_string(),
            Some(200),
        )
        .unwrap();
        (workspace.id.to_string(), suggestions[0].id.clone())
    }

    #[test]
    fn accepted_suggestion_creates_intent_request() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let (workspace_id, suggestion_id) = seed_workspace_with_suggestion(&kernel);

        CommandHandler::accept_suggestion(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace_id,
            suggestion_id.clone(),
        )
        .unwrap();

        let request = SuggestionIntentService::create_request(
            &kernel.shared_database(),
            &suggestion_id,
            ActorType::LocalUser,
            Some("local-user".into()),
        )
        .unwrap();

        assert_eq!(request.suggestion_id, suggestion_id);
        assert_eq!(request.intent_id.as_str(), "create-zone");
        assert!(request.resource_ref.is_some());
        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejected_suggestion_fails() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let (workspace_id, suggestion_id) = seed_workspace_with_suggestion(&kernel);

        CommandHandler::reject_suggestion(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace_id,
            suggestion_id.clone(),
        )
        .unwrap();

        let error = SuggestionIntentService::create_request(
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
    fn unmapped_suggestion_type_would_fail_at_mapping_layer() {
        // All current SuggestionType variants map — verify mapping is exhaustive.
        for suggestion_type in [
            SuggestionType::ResourceGrowth,
            SuggestionType::LayoutActivity,
            SuggestionType::WorkspaceActivity,
        ] {
            assert!(map_suggestion_type_to_intent(suggestion_type).is_ok());
        }
    }

    #[test]
    fn service_does_not_execute_commands() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let (workspace_id, suggestion_id) = seed_workspace_with_suggestion(&kernel);

        CommandHandler::accept_suggestion(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
            suggestion_id.clone(),
        )
        .unwrap();

        let zones_before = {
            let db = kernel.shared_database();
            let db = db.lock().unwrap();
            crate::services::ZoneService::list_by_workspace(
                &db,
                &workspace_domain::WorkspaceId::new(&workspace_id).unwrap(),
            )
            .unwrap()
            .len()
        };

        let _ = SuggestionIntentService::create_request(
            &kernel.shared_database(),
            &suggestion_id,
            ActorType::LocalUser,
            None,
        )
        .unwrap();

        let zones_after = {
            let db = kernel.shared_database();
            let db = db.lock().unwrap();
            crate::services::ZoneService::list_by_workspace(
                &db,
                &workspace_domain::WorkspaceId::new(&workspace_id).unwrap(),
            )
            .unwrap()
            .len()
        };

        assert_eq!(zones_before, zones_after);
    }
}
