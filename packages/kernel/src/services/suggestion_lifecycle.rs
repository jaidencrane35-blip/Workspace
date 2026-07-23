//! Derived suggestion lifecycle service — read-only projection over the audit
//! trail (Sprint 22).
//!
//! Lifecycle records reinterpret governance history into per-suggestion lifecycle
//! visibility. The audit trail is the durable record; this service adds no
//! persistence, mutation, or inference.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    classify_suggestion_lifecycle_event, extract_suggestion_id, parse_canonical_resource_ref,
    SuggestionLifecycleRecord,
};

use super::AuditService;
use crate::error::{KernelError, Result};

const MAX_AUDIT_SCAN: usize = 500;

/// Derives suggestion lifecycle records from persisted audit history.
pub struct SuggestionLifecycleService;

impl SuggestionLifecycleService {
    /// Returns up to `limit` recent lifecycle records, most recent first.
    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<SuggestionLifecycleRecord>> {
        let scan = limit.saturating_mul(4).clamp(limit.max(1), MAX_AUDIT_SCAN);
        let audit_events = AuditService::list_recent(db, scan)?;

        let mut records = Vec::new();
        for event in audit_events {
            let Some(state) = classify_suggestion_lifecycle_event(&event) else {
                continue;
            };

            let suggestion_id = match extract_suggestion_id(event.metadata.as_deref()) {
                Some(id) => id,
                None => continue,
            };

            let related_resource_ref = event
                .resource_ref
                .as_deref()
                .and_then(parse_canonical_resource_ref);

            let record = SuggestionLifecycleRecord {
                suggestion_id,
                state,
                occurred_at: event.timestamp,
                actor_type: event.actor_type,
                actor_id: event.actor_id,
                related_resource_ref,
                metadata: event.metadata,
            };

            record
                .validate()
                .map_err(|error| KernelError::SuggestionLifecycleValidation {
                    message: error.to_string(),
                })?;

            records.push(record);

            if records.len() >= limit {
                break;
            }
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandHandler, WorkspaceKernel};
    use workspace_domain::{
        ActorContext, IntentContext, SuggestionLifecycleState,
    };

    fn local_actor() -> (ActorContext, IntentContext) {
        (ActorContext::local_user(), IntentContext::user_request())
    }

    #[test]
    fn derives_accept_lifecycle_records_from_audit() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let (actor, intent) = local_actor();

        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Lifecycle WS".into(),
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

        CommandHandler::accept_suggestion(
            &kernel,
            actor,
            intent,
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();

        let records =
            SuggestionLifecycleService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(records.iter().any(|record| {
            record.suggestion_id == suggestion_id
                && record.state == SuggestionLifecycleState::Accepted
        }));
        assert!(records.iter().all(|record| record.validate().is_ok()));
    }

    #[test]
    fn excludes_unrelated_audit_events() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let (actor, intent) = local_actor();

        CommandHandler::create_workspace(&kernel, actor, intent, "Unrelated".into()).unwrap();

        let records =
            SuggestionLifecycleService::list_recent(&kernel.shared_database(), 50).unwrap();

        assert!(records.is_empty());
    }
}
