//! Governed cancellation request command (Sprint 28).
//!
//! Records that cancellation was requested for an existing execution identity.
//! Does not interrupt running commands or bypass the permission pipeline.

use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::ExecutionCancellationService;
use workspace_domain::{CancellationRequest, Capability, ResourceRef, WorkspaceId};

/// Requests cancellation of a previously identified execution.
///
/// Decision/request boundary only — no runtime interruption in this sprint.
pub struct RequestExecutionCancellation {
    pub workspace_id: WorkspaceId,
    pub execution_request_id: String,
    pub reason: String,
}

impl RequestExecutionCancellation {
    pub fn new(
        workspace_id: WorkspaceId,
        execution_request_id: String,
        reason: String,
    ) -> Self {
        Self {
            workspace_id,
            execution_request_id,
            reason,
        }
    }
}

impl crate::commands::Command for RequestExecutionCancellation {
    fn name(&self) -> &'static str {
        "RequestExecutionCancellation"
    }
}

impl MutationCommand for RequestExecutionCancellation {
    type Output = CancellationRequest;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Same governance write boundary as ExecuteIntentRequest.
        Capability::audit_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        None
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "execution_request_id": output.execution_request_id,
                "cancellation_status": output.status.as_str(),
                "cancellation_id": output.id,
                "reason": output.reason,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<CancellationRequest> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let requested_by = ctx.actor_context.actor.id.to_string();
        ExecutionCancellationService::prepare_request(
            &ctx.database,
            &self.execution_request_id,
            &requested_by,
            &self.reason,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::execute_intent_request::ExecuteIntentRequest;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::r#trait::MutationCommand;
    use crate::commands::CommandContext;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::services::{AuditService, ExecutionOutcomeService};
    use workspace_domain::{
        ActorContext, CapabilitySet, ExecutionOutcomeStatus, IntentContext,
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

    #[test]
    fn requires_audit_write_capability() {
        let command = RequestExecutionCancellation::new(
            WorkspaceId::new("ws-1").unwrap(),
            "execution:s-1".into(),
            "stop".into(),
        );
        assert_eq!(command.required_capability(), Capability::audit_write());
    }

    #[test]
    fn cancellation_request_is_audited_and_projects_cancelled_outcome() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Cancel Cmd WS".into()))
            .unwrap();

        let suggestion_id = "missing-suggestion";
        let _ = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
            ExecuteIntentRequest::new(workspace.id.clone(), suggestion_id.into()),
        );
        let execution_request_id = format!("execution:{suggestion_id}");

        let request = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(RequestExecutionCancellation::new(
                workspace.id.clone(),
                execution_request_id.clone(),
                "operator stop".into(),
            ))
            .unwrap();

        assert_eq!(request.execution_request_id, execution_request_id);
        assert_eq!(request.status.as_str(), "requested");

        let audit = AuditService::list_recent(&init.database.shared(), 100).unwrap();
        assert!(audit.iter().any(|event| {
            event.success
                && event.command_name.as_deref() == Some("RequestExecutionCancellation")
                && event.metadata.as_deref().is_some_and(|m| {
                    m.contains(&execution_request_id)
                        && m.contains("\"cancellation_status\":\"requested\"")
                        && m.contains("operator stop")
                })
        }));

        let outcomes =
            ExecutionOutcomeService::list_recent(&init.database.shared(), 50).unwrap();
        assert!(outcomes.iter().any(|outcome| {
            outcome.execution_request_id == execution_request_id
                && outcome.status == ExecutionOutcomeStatus::Cancelled
        }));
    }

    #[test]
    fn completed_execution_cannot_be_cancelled() {
        use crate::commands::accept_suggestion::AcceptSuggestion;
        use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
        use crate::commands::get_suggestions::GetSuggestions;
        use crate::commands::zone::CreateZone;

        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Cancel Done WS".into()))
            .unwrap();
        for index in 0..4 {
            CommandPipeline::new(ready_ctx(&init, &bus))
                .execute_mutation(CreateZone::new(
                    workspace.id.clone(),
                    format!("Zone {index}"),
                    None,
                ))
                .unwrap();
        }
        let suggestions = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
            .unwrap();
        let suggestion_id = suggestions[0].id.clone();
        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(AcceptSuggestion::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();
        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateSuggestionIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();
        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();

        let err = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(RequestExecutionCancellation::new(
                workspace.id,
                format!("execution:{suggestion_id}"),
                "too late".into(),
            ))
            .unwrap_err();

        assert!(matches!(
            err,
            KernelError::CannotCancelCompletedExecution { .. }
        ));
    }
}
