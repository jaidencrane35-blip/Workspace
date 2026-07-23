use crate::commands::context::CommandContext;
use crate::commands::resource::{ensure_ready, resource_lifecycle_event};
use crate::commands::r#trait::MutationCommand;
use crate::error::Result;
use crate::events::types::{DomainEvent, WorkspaceEntityCreated};
use crate::security::PermissionSubject;
use crate::services::WorkspaceService;
use workspace_domain::{Addressable, Capability, ResourceKind, ResourceRef, Workspace};

/// Creates a new workspace domain entity.
pub struct CreateWorkspace {
    pub name: String,
}

impl crate::commands::Command for CreateWorkspace {
    fn name(&self) -> &'static str {
        "CreateWorkspace"
    }
}

impl MutationCommand for CreateWorkspace {
    type Output = Workspace;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Workspace> {
        ensure_ready(ctx)?;
        let workspace = ctx.with_database(|db| WorkspaceService::create(db, self.name.clone()))?;
        let lifecycle = resource_lifecycle_event(
            workspace.resource_ref(),
            None,
            ctx,
            Capability::workspace_write(),
        );
        ctx.event_bus.publish(DomainEvent::ResourceCreated(lifecycle));
        ctx.event_bus.publish(DomainEvent::WorkspaceCreated(WorkspaceEntityCreated {
            workspace_id: workspace.id.to_string(),
            name: workspace.name.clone(),
            actor: Some(ctx.actor_context.clone()),
            intent: Some(ctx.intent_context.clone()),
            capability: Some(Capability::workspace_write()),
        }));
        Ok(workspace)
    }
}

impl CreateWorkspace {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::context::CommandContext;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use std::sync::{Arc, Mutex};
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    #[test]
    fn emits_workspace_created_event() {
        let bus = EventBus::new();
        let event_name = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&event_name);

        bus.subscribe(move |event| {
            *captured.lock().unwrap() = event.name().to_string();
        });

        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Dev".into()))
            .unwrap();

        assert_eq!(workspace.name, "Dev");
        assert_eq!(*event_name.lock().unwrap(), "workspace.entity.created");
    }
}
