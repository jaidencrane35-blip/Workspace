//! Deterministic workspace context service — the "Context" boundary (Sprint 19;
//! enriched Sprint 26).
//!
//! Composes the existing derived read layers into a single [`WorkspaceContext`]:
//! projection (state), observations (activity), analytics (metrics), capability
//! discovery (authority), and execution outcome summary (history). It performs
//! no mutation, executes no commands, grants no permissions, and adds no
//! persistence — it orchestrates existing services and validates the
//! composition via the pure domain type.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, CapabilitySet, IntentContext, WorkspaceContext, WorkspaceId,
};

use super::{
    CapabilityResolver, ExecutionContextService, ObservationService, WorkspaceAnalyticsService,
    WorkspaceProjectionService,
};
use crate::error::{KernelError, Result};
use crate::policy::PermissionPolicy;
use crate::security::PermissionGate;

/// Assembles the derived read layers into a deterministic workspace context.
pub struct WorkspaceContextService;

impl WorkspaceContextService {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        capability_set: &CapabilitySet,
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        workspace_id: &WorkspaceId,
        limit: usize,
    ) -> Result<WorkspaceContext> {
        // Build the snapshot under a short-lived lock, then release it before
        // calling services that lock the database themselves (std Mutex is not
        // reentrant).
        let snapshot = {
            let guard = db.lock().expect("database lock poisoned");
            WorkspaceProjectionService::build_snapshot(&guard, workspace_id)?
        };

        let observations = ObservationService::list_recent(db, limit)?;
        let metrics = WorkspaceAnalyticsService::compute(db, limit)?;
        let capabilities = CapabilityResolver::discover(
            actor_context,
            intent_context,
            capability_set,
            policy,
            gate,
        )?;
        let execution_context = ExecutionContextService::summarize(db, limit)?;

        let context = WorkspaceContext {
            generated_at: Utc::now().to_rfc3339(),
            workspace: snapshot.workspace.clone(),
            snapshot,
            observations,
            metrics,
            capabilities,
            execution_context,
        };

        context
            .validate()
            .map_err(|error| KernelError::ContextValidation {
                message: error.to_string(),
            })?;

        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::WorkspaceKernel;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    #[test]
    fn composes_projection_observations_and_metrics() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let workspace = kernel.create_workspace("Context WS".into()).unwrap();

        let context = WorkspaceContextService::build(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
            &workspace.id,
            200,
        )
        .unwrap();

        assert_eq!(context.snapshot.workspace_name, "Context WS");
        assert!(context.metrics.observation_count > 0);
        assert!(context
            .observations
            .iter()
            .any(|obs| obs.source_event_type == "workspace.entity.created"));
        assert_eq!(
            context.execution_context,
            workspace_domain::ExecutionContextSummary::empty()
        );
        assert!(context.validate().is_ok());
    }

    #[test]
    fn includes_execution_outcome_summary_when_present() {
        use crate::CommandHandler;
        use workspace_domain::{ActorContext, IntentContext};

        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandHandler::create_workspace(
            &kernel,
            actor.clone(),
            intent.clone(),
            "Outcome Context WS".into(),
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
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();
        CommandHandler::create_suggestion_intent_request(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id.clone(),
        )
        .unwrap();
        CommandHandler::execute_intent_request(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            suggestion_id,
        )
        .unwrap();

        let context = WorkspaceContextService::build(
            &kernel.shared_database(),
            &actor,
            &intent,
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
            &workspace.id,
            200,
        )
        .unwrap();

        assert!(context.execution_context.recent_completed_count >= 1);
        assert!(context.execution_context.last_execution_time.is_some());
        assert!(!context.execution_context.recent_commands.is_empty());
        assert!(context.validate().is_ok());
    }

    #[test]
    fn preserves_workspace_resource_ref_identity() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let workspace = kernel.create_workspace("Identity WS".into()).unwrap();

        let context = WorkspaceContextService::build(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
            &workspace.id,
            200,
        )
        .unwrap();

        assert_eq!(context.workspace, context.snapshot.workspace);
        assert_eq!(
            context.workspace.canonical(),
            format!("workspace:{}", workspace.id.as_str())
        );
    }
}
