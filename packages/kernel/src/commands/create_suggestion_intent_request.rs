use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::SuggestionIntentService;
use workspace_domain::{Capability, ResourceRef, SuggestionIntentRequest, WorkspaceId};

/// Creates an approval-gated intent bridge for an accepted suggestion.
///
/// Records the governance transition in audit metadata. Does NOT execute the
/// mapped action intent or dispatch any follow-on command.
pub struct CreateSuggestionIntentRequest {
    pub workspace_id: WorkspaceId,
    pub suggestion_id: String,
}

impl CreateSuggestionIntentRequest {
    pub fn new(workspace_id: WorkspaceId, suggestion_id: String) -> Self {
        Self {
            workspace_id,
            suggestion_id,
        }
    }
}

impl crate::commands::Command for CreateSuggestionIntentRequest {
    fn name(&self) -> &'static str {
        "CreateSuggestionIntentRequest"
    }
}

impl MutationCommand for CreateSuggestionIntentRequest {
    type Output = SuggestionIntentRequest;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Bridge records a governance transition in audit — same boundary as accept/reject.
        Capability::audit_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        output.resource_ref.clone()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "suggestion_id": output.suggestion_id,
                "intent_id": output.intent_id.as_str(),
                "bridge": "intent_request_created",
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<SuggestionIntentRequest> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        SuggestionIntentService::create_request(
            &ctx.database,
            &self.suggestion_id,
            ctx.actor_context.actor.actor_type,
            Some(ctx.actor_context.actor.id.to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::accept_suggestion::AcceptSuggestion;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::get_suggestions::GetSuggestions;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::zone::CreateZone;
    use crate::commands::CommandContext;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::services::{AuditService, ZoneService};
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

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

    fn seed_accepted_suggestion(
        init: &crate::commands::initialize::InitializeWorkspaceResult,
        bus: &EventBus,
    ) -> (workspace_domain::Workspace, String) {
        let workspace = CommandPipeline::new(ready_ctx(init, bus))
            .execute_mutation(CreateWorkspace::new("Bridge WS".into()))
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
        (workspace, suggestion_id)
    }

    #[test]
    fn pipeline_creates_intent_request_and_audits() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
        let (workspace, suggestion_id) = seed_accepted_suggestion(&init, &bus);

        let request = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateSuggestionIntentRequest::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();

        assert_eq!(request.suggestion_id, suggestion_id);
        assert_eq!(request.intent_id.as_str(), "create-zone");
        assert!(request.validate().is_ok());

        let audit = AuditService::list_recent(&init.database.shared(), 50).unwrap();
        assert!(audit.iter().any(|event| {
            event.success
                && event.command_name.as_deref() == Some("CreateSuggestionIntentRequest")
                && event
                    .metadata
                    .as_deref()
                    .is_some_and(|m| m.contains(&suggestion_id))
        }));
    }

    #[test]
    fn command_does_not_execute_mapped_intent() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
        let (workspace, suggestion_id) = seed_accepted_suggestion(&init, &bus);

        let zone_count_before = {
            let db = init.database.shared();
            let db = db.lock().unwrap();
            ZoneService::list_by_workspace(&db, &workspace.id)
                .unwrap()
                .len()
        };

        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateSuggestionIntentRequest::new(
                workspace.id.clone(),
                suggestion_id,
            ))
            .unwrap();

        let zone_count_after = {
            let db = init.database.shared();
            let db = db.lock().unwrap();
            ZoneService::list_by_workspace(&db, &workspace.id)
                .unwrap()
                .len()
        };

        assert_eq!(zone_count_before, zone_count_after);
    }

    #[test]
    fn requires_audit_write_capability() {
        use crate::commands::r#trait::MutationCommand;

        let command = CreateSuggestionIntentRequest::new(
            WorkspaceId::new("ws-1").unwrap(),
            "s-1".into(),
        );
        assert_eq!(command.required_capability(), Capability::audit_write());
    }
}
