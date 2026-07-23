//! Deterministic suggestion service — Suggest + decision support (Sprint 20–21).
//!
//! Consumes a [`WorkspaceContext`] (built via [`WorkspaceContextService`]) and
//! produces deterministic proposals via the pure domain generator. Listing is
//! read-only. Accept/Reject decisions are recorded by mutation commands through
//! the existing command pipeline; this service only resolves pending proposals
//! and suppresses previously decided ids using the audit trail (no suggestion
//! store).

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    derive_suggestions, find_pending_suggestion, ActorContext, CapabilitySet, IntentContext,
    Suggestion, WorkspaceId,
};

use super::{AuditService, WorkspaceContextService};
use crate::error::{KernelError, Result};
use crate::policy::PermissionPolicy;
use crate::security::PermissionGate;

/// Derives deterministic proposals from an assembled workspace context.
pub struct SuggestionService;

impl SuggestionService {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        capability_set: &CapabilitySet,
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        workspace_id: &WorkspaceId,
        limit: usize,
    ) -> Result<Vec<Suggestion>> {
        let context = WorkspaceContextService::build(
            db,
            actor_context,
            intent_context,
            capability_set,
            policy,
            gate,
            workspace_id,
            limit,
        )?;

        let suggestions = derive_suggestions(&context);

        for suggestion in &suggestions {
            suggestion
                .validate()
                .map_err(|error| KernelError::SuggestionValidation {
                    message: error.to_string(),
                })?;
        }

        let decided = Self::decided_suggestion_ids(db)?;
        Ok(suggestions
            .into_iter()
            .filter(|suggestion| !decided.contains(&suggestion.id))
            .collect())
    }

    /// Resolves a currently pending derived suggestion by id (fail closed).
    #[allow(clippy::too_many_arguments)]
    pub fn resolve_pending(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        capability_set: &CapabilitySet,
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        workspace_id: &WorkspaceId,
        suggestion_id: &str,
    ) -> Result<Suggestion> {
        let pending = Self::list(
            db,
            actor_context,
            intent_context,
            capability_set,
            policy,
            gate,
            workspace_id,
            500,
        )?;

        find_pending_suggestion(&pending, suggestion_id).map_err(|error| {
            KernelError::SuggestionValidation {
                message: error.to_string(),
            }
        })
    }

    fn decided_suggestion_ids(db: &Arc<Mutex<Database>>) -> Result<BTreeSet<String>> {
        let records = AuditService::list_recent(db, 500)?;
        let mut decided = BTreeSet::new();

        for record in records {
            let Some(command_name) = record.command_name.as_deref() else {
                continue;
            };
            if command_name != "AcceptSuggestion" && command_name != "RejectSuggestion" {
                continue;
            }
            if !record.success {
                continue;
            }
            let Some(metadata) = record.metadata.as_deref() else {
                continue;
            };
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) {
                if let Some(suggestion_id) = value.get("suggestion_id").and_then(|v| v.as_str()) {
                    decided.insert(suggestion_id.to_string());
                }
            }
        }

        Ok(decided)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::WorkspaceKernel;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext, SuggestionStatus};

    fn list_for(kernel: &WorkspaceKernel, workspace_id: &WorkspaceId) -> Vec<Suggestion> {
        SuggestionService::list(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
            workspace_id,
            200,
        )
        .unwrap()
    }

    #[test]
    fn produces_deterministic_suggestions_from_activity() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let workspace = kernel.create_workspace("Suggest WS".into()).unwrap();
        // Create several more workspaces to cross the resource-growth threshold
        // (observations/metrics are global to the workspace instance).
        for index in 0..4 {
            kernel.create_workspace(format!("Extra {index}")).unwrap();
        }

        let first = list_for(&kernel, &workspace.id);
        let second = list_for(&kernel, &workspace.id);

        assert_eq!(first, second, "suggestions must be deterministic");
        assert!(!first.is_empty());
        assert!(first.iter().all(|s| s.status == SuggestionStatus::Pending));
        assert!(first.iter().all(|s| s.validate().is_ok()));
    }

    #[test]
    fn listing_does_not_mutate_state() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let workspace = kernel.create_workspace("Stable WS".into()).unwrap();

        let audit_before =
            crate::services::AuditService::list_recent(&kernel.shared_database(), 500)
                .unwrap()
                .len();
        let _ = list_for(&kernel, &workspace.id);
        let audit_after =
            crate::services::AuditService::list_recent(&kernel.shared_database(), 500)
                .unwrap()
                .len();

        // Deriving suggestions writes nothing to the durable record.
        assert_eq!(audit_before, audit_after);
    }
}
