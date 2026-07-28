use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::get_audit_history::GetAuditHistory;
use crate::commands::layout::GetLayoutSnapshot;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::MutationCommand;
use crate::commands::zone::CreateZone;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::{ExecutionGuardService, ExecutionLifecycleService};
use crate::services::GovernedIntentExecutionService;
use crate::services::LayoutService;
use workspace_domain::{
    execution_request_id_for_suggestion, ActionIntentRequest, Capability, IntentExecutionRequest,
    ResourceKind, ResourceRef, WorkspaceId,
};

/// Executes a governed intent request derived from an approved suggestion bridge.
///
/// Validates and prepares through [`GovernedIntentExecutionService`], then
/// dispatches the mapped command through [`CommandPipeline`]. Does not bypass
/// governance. Distinct from Sprint 15 action-intent validation.
pub struct ExecuteIntentRequest {
    pub workspace_id: WorkspaceId,
    pub suggestion_id: String,
}

impl ExecuteIntentRequest {
    pub fn new(workspace_id: WorkspaceId, suggestion_id: String) -> Self {
        Self {
            workspace_id,
            suggestion_id,
        }
    }
}

impl crate::commands::Command for ExecuteIntentRequest {
    fn name(&self) -> &'static str {
        "ExecuteIntentRequest"
    }
}

impl MutationCommand for ExecuteIntentRequest {
    type Output = IntentExecutionRequest;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        None
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "execution_request": true,
                "execution_request_id": format!("execution:{}", output.suggestion_id),
                "suggestion_id": output.suggestion_id,
                "intent_id": output.action_intent_id.as_str(),
                "execution_status": output.status.as_str(),
            })
            .to_string(),
        )
    }

    fn audit_failure_metadata(&self) -> Option<String> {
        Some(
            json!({
                "execution_request": true,
                "execution_request_id": format!("execution:{}", self.suggestion_id),
                "suggestion_id": self.suggestion_id,
                "execution_status": "failed",
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<IntentExecutionRequest> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        // Audit-derived idempotency: block only prior successful completions.
        // Runs inside the mutation after permission/policy — never bypasses governance.
        ExecutionGuardService::ensure_allowed(&ctx.database, &self.suggestion_id)?;

        let prepared = GovernedIntentExecutionService::prepare(
            &ctx.database,
            &self.suggestion_id,
            ctx.actor_context.actor.actor_type,
            Some(ctx.actor_context.actor.id.to_string()),
        )?;

        let execution_request_id = execution_request_id_for_suggestion(&self.suggestion_id);
        ExecutionLifecycleService::claim(
            &ctx.database,
            &execution_request_id,
            &self.suggestion_id,
            Some(prepared.execution_request.action_intent_id.as_str()),
        )?;

        if let Err(dispatch_error) =
            dispatch_mapped_command(ctx, prepared.command_name, &prepared.action_intent)
        {
            let retry_allowed =
                !matches!(&dispatch_error, KernelError::ExecutionAtomicity { .. });
            if let Err(lifecycle_error) = ExecutionLifecycleService::mark_failed(
                &ctx.database,
                &execution_request_id,
                retry_allowed,
                &dispatch_error.to_string(),
            ) {
                log::error!(
                    "execution dispatch failed ({dispatch_error}); failure persistence also failed: {lifecycle_error}"
                );
                return Err(lifecycle_error);
            }
            return Err(dispatch_error);
        }

        if let Err(completion_error) = ExecutionLifecycleService::complete(
            &ctx.database,
            &execution_request_id,
            Some(prepared.execution_request.action_intent_id.as_str()),
        ) {
            if let Err(failure_error) = ExecutionLifecycleService::mark_failed(
                &ctx.database,
                &execution_request_id,
                false,
                "dispatch succeeded but completion persistence failed",
            ) {
                log::error!(
                    "execution completion failed ({completion_error}); failure persistence also failed: {failure_error}"
                );
                return Err(failure_error);
            }
            return Err(completion_error);
        }

        prepared
            .execution_request
            .mark_executed()
            .map_err(|error| KernelError::IntentExecutionValidation {
                message: error.to_string(),
            })
    }
}

fn dispatch_mapped_command(
    ctx: &CommandContext<'_>,
    command_name: &str,
    action: &ActionIntentRequest,
) -> Result<()> {
    let pipeline = CommandPipeline::new(ctx.fork());

    match command_name {
        "CreateZone" => {
            let workspace_id = workspace_id_from_target(action)?;
            pipeline
                .execute_mutation_with_action(
                    action,
                    CreateZone::new(
                        workspace_id,
                        "Suggested organization zone".into(),
                        None,
                    ),
                )
                .map(|_| ())
        }
        "GetAuditHistory" => pipeline
            .execute_query_with_action(action, GetAuditHistory::new(Some(50)))
            .map(|_| ()),
        "GetLayoutSnapshot" => {
            let workspace_id = workspace_id_from_target(action)?;
            let layout_id = ctx.with_database(|db| {
                LayoutService::load_by_workspace(db, &workspace_id).map(|layout| layout.id)
            })?;
            pipeline
                .execute_query_with_action(action, GetLayoutSnapshot::new(layout_id))
                .map(|_| ())
        }
        other => Err(KernelError::IntentExecutionValidation {
            message: format!("unsupported mapped command for suggestion execution: {other}"),
        }),
    }
}

fn workspace_id_from_target(action: &ActionIntentRequest) -> Result<WorkspaceId> {
    let Some(target) = action.target_resource.as_ref() else {
        return Err(KernelError::IntentExecutionValidation {
            message: "mapped command requires a workspace target resource".into(),
        });
    };
    if target.kind != ResourceKind::Workspace {
        return Err(KernelError::IntentExecutionValidation {
            message: format!(
                "expected workspace target, got {}",
                target.kind.as_str()
            ),
        });
    }
    WorkspaceId::new(target.id.as_str()).map_err(KernelError::Domain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::accept_suggestion::AcceptSuggestion;
    use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::get_suggestions::GetSuggestions;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::zone::CreateZone;
    use crate::commands::CommandContext;
    use crate::commands::r#trait::MutationCommand;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::services::{AuditService, ZoneService};
    use workspace_domain::{
        ActorContext, CapabilitySet, IntentContext, IntentExecutionStatus,
    };

    fn ready_ctx<'a>(
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        bus: &'a EventBus,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        }
    }

    fn seed_through_intent_bridge(
        init: &crate::commands::initialize::InitializeWorkspaceResult,
        bus: &EventBus,
    ) -> (workspace_domain::Workspace, String) {
        let workspace = CommandPipeline::new(ready_ctx(init, bus))
            .execute_mutation(CreateWorkspace::new("Execute WS".into()))
            .unwrap();
        for index in 0..4 {
            CommandPipeline::new(ready_ctx(init, bus))
                .execute_mutation(CreateZone::new(
                    workspace.id.clone(),
                    format!("Zone {index}"),
                    None,
                ))
                .unwrap();
        }
        let suggestions = CommandPipeline::new(ready_ctx(init, bus))
            .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
            .unwrap();
        let suggestion_id = suggestions[0].id.clone();
        CommandPipeline::new(ready_ctx(init, bus))
            .execute_mutation(AcceptSuggestion::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();
        CommandPipeline::new(ready_ctx(init, bus))
            .execute_mutation(CreateSuggestionIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();
        (workspace, suggestion_id)
    }

    #[test]
    fn pipeline_executes_mapped_command_and_audits() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
        let (workspace, suggestion_id) = seed_through_intent_bridge(&init, &bus);

        let zone_count_before = {
            let db = init.database.shared();
            let db = db.lock().unwrap();
            ZoneService::list_by_workspace(&db, &workspace.id)
                .unwrap()
                .len()
        };

        let execution = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();

        assert_eq!(execution.status, IntentExecutionStatus::Executed);
        assert_eq!(execution.suggestion_id, suggestion_id);

        let zone_count_after = {
            let db = init.database.shared();
            let db = db.lock().unwrap();
            ZoneService::list_by_workspace(&db, &workspace.id)
                .unwrap()
                .len()
        };
        assert_eq!(zone_count_after, zone_count_before + 1);

        let audit = AuditService::list_recent(&init.database.shared(), 100).unwrap();
        assert!(audit.iter().any(|event| {
            event.success
                && event.command_name.as_deref() == Some("ExecuteIntentRequest")
                && event.metadata.as_deref().is_some_and(|m| {
                    m.contains("\"execution_request\":true")
                        && m.contains(&suggestion_id)
                        && m.contains("\"execution_status\":\"executed\"")
                        && m.contains(&format!("execution:{suggestion_id}"))
                })
        }));
    }

    #[test]
    fn requires_audit_write_capability() {
        let command = ExecuteIntentRequest::new(WorkspaceId::new("ws-1").unwrap(), "s-1".into());
        assert_eq!(command.required_capability(), Capability::audit_write());
    }

    #[test]
    fn duplicate_successful_execution_is_rejected() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
        let (workspace, suggestion_id) = seed_through_intent_bridge(&init, &bus);

        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();

        let err = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap_err();

        assert!(matches!(
            err,
            KernelError::DuplicateExecution { execution_request_id }
                if execution_request_id == format!("execution:{suggestion_id}")
        ));

        // Rejection uses the existing failure audit path (no new event types).
        let audit = AuditService::list_recent(&init.database.shared(), 100).unwrap();
        assert!(audit.iter().any(|event| {
            !event.success
                && event.command_name.as_deref() == Some("ExecuteIntentRequest")
                && event.metadata.as_deref().is_some_and(|m| {
                    m.contains("duplicate_execution")
                        || m.contains(&format!("execution:{suggestion_id}"))
                })
        }));
    }
}
