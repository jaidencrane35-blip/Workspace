use crate::commands::context::CommandContext;
use crate::commands::resource::{ensure_ready, ensure_workspace_exists, resource_lifecycle_event};
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, ResourceLifecycleEvent};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ZoneService;
use workspace_domain::{
    Addressable, Capability, ResourceKind, ResourceRef, WorkspaceId, Zone, ZoneId,
};

/// Creates a zone within a workspace.
pub struct CreateZone {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub position_metadata: Option<String>,
}

impl crate::commands::Command for CreateZone {
    fn name(&self) -> &'static str {
        "CreateZone"
    }
}

impl MutationCommand for CreateZone {
    type Output = Zone;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Zone)
    }

    fn required_capability(&self) -> Capability {
        Capability::zone_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Zone> {
        ensure_ready(ctx)?;
        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let zone = ctx.with_database(|db| {
            ZoneService::create(
                db,
                self.workspace_id.clone(),
                self.name.clone(),
                self.position_metadata.clone(),
            )
        })?;

        let lifecycle = resource_lifecycle_event(
            zone.resource_ref(),
            Some(zone.workspace_id.to_string()),
            ctx,
            Capability::zone_write(),
        );
        ctx.event_bus.publish(DomainEvent::ResourceCreated(lifecycle.clone()));
        ctx.event_bus.publish(DomainEvent::ZoneCreated(lifecycle));

        Ok(zone)
    }
}

impl CreateZone {
    pub fn new(
        workspace_id: WorkspaceId,
        name: String,
        position_metadata: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            name,
            position_metadata,
        }
    }
}

/// Deletes a zone by id.
pub struct DeleteZone {
    pub id: ZoneId,
}

impl crate::commands::Command for DeleteZone {
    fn name(&self) -> &'static str {
        "DeleteZone"
    }
}

impl MutationCommand for DeleteZone {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Zone)
    }

    fn required_capability(&self) -> Capability {
        Capability::zone_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Zone,
            workspace_domain::ResourceId::new(self.id.as_str()).expect("zone id is non-empty"),
        ))
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        ensure_ready(ctx)?;

        let zone = ctx.with_database(|db| ZoneService::load(db, &self.id))?;
        let resource_ref = zone.resource_ref();

        ctx.with_database(|db| ZoneService::delete(db, &self.id))?;

        ctx.event_bus.publish(DomainEvent::ResourceDeleted(ResourceLifecycleEvent {
            resource_ref: resource_ref.clone(),
            workspace_id: Some(zone.workspace_id.to_string()),
            actor: Some(ctx.actor_context.clone()),
            intent: Some(ctx.intent_context.clone()),
            capability: Some(Capability::zone_write()),
        }));

        Ok(())
    }
}

impl DeleteZone {
    pub fn new(id: ZoneId) -> Self {
        Self { id }
    }
}

/// Retrieves a zone by id.
pub struct GetZone {
    pub id: ZoneId,
}

impl crate::commands::Command for GetZone {
    fn name(&self) -> &'static str {
        "GetZone"
    }
}

impl QueryCommand for GetZone {
    type Output = Zone;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Zone)
    }

    fn required_capability(&self) -> Capability {
        Capability::zone_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Zone> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ctx.with_database(|db| ZoneService::load(db, &self.id))
    }
}

impl GetZone {
    pub fn new(id: ZoneId) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::{InitializeWorkspace, InitializeWorkspaceResult};
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    fn ready_ctx<'a>(
        init: &'a InitializeWorkspaceResult,
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
    fn create_zone_registers_graph_and_emits_events() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_ctx(&init, &bus);

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Zone Parent".into()))
            .unwrap();

        let zone = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
            CreateZone::new(workspace.id.clone(), "Primary".into(), None),
        ).unwrap();

        assert_eq!(zone.name, "Primary");
    }
}
