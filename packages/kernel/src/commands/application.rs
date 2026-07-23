use crate::commands::context::CommandContext;
use crate::commands::resource::{ensure_ready, ensure_workspace_exists, resource_lifecycle_event};
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, ResourceLifecycleEvent};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::ApplicationService;
use workspace_domain::{
    Addressable, ApplicationId, ApplicationReference, Capability, ResourceId, ResourceKind,
    ResourceRef, WorkspaceId,
};

/// Creates an application reference within a workspace.
pub struct CreateApplication {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub identifier: Option<String>,
}

impl crate::commands::Command for CreateApplication {
    fn name(&self) -> &'static str {
        "CreateApplication"
    }
}

impl MutationCommand for CreateApplication {
    type Output = ApplicationReference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Application)
    }

    fn required_capability(&self) -> Capability {
        Capability::application_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ApplicationReference> {
        ensure_ready(ctx)?;
        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let application = ctx.with_database(|db| {
            ApplicationService::create(
                db,
                self.workspace_id.clone(),
                self.name.clone(),
                self.identifier.clone(),
            )
        })?;

        let lifecycle = resource_lifecycle_event(
            application.resource_ref(),
            Some(application.workspace_id.to_string()),
            ctx,
            Capability::application_write(),
        );
        ctx.event_bus
            .publish(DomainEvent::ResourceCreated(lifecycle.clone()));
        ctx.event_bus
            .publish(DomainEvent::ApplicationCreated(lifecycle));

        Ok(application)
    }
}

impl CreateApplication {
    pub fn new(
        workspace_id: WorkspaceId,
        name: String,
        identifier: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            name,
            identifier,
        }
    }
}

/// Deletes an application reference by id.
pub struct DeleteApplication {
    pub id: ApplicationId,
}

impl crate::commands::Command for DeleteApplication {
    fn name(&self) -> &'static str {
        "DeleteApplication"
    }
}

impl MutationCommand for DeleteApplication {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Application)
    }

    fn required_capability(&self) -> Capability {
        Capability::application_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Application,
            ResourceId::new(self.id.as_str()).expect("application id is non-empty"),
        ))
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        ensure_ready(ctx)?;

        let application = ctx.with_database(|db| ApplicationService::load(db, &self.id))?;
        let resource_ref = application.resource_ref();

        ctx.with_database(|db| ApplicationService::delete(db, &self.id))?;

        ctx.event_bus.publish(DomainEvent::ResourceDeleted(ResourceLifecycleEvent {
            resource_ref: resource_ref.clone(),
            workspace_id: Some(application.workspace_id.to_string()),
            actor: Some(ctx.actor_context.clone()),
            intent: Some(ctx.intent_context.clone()),
            capability: Some(Capability::application_write()),
        }));

        Ok(())
    }
}

impl DeleteApplication {
    pub fn new(id: ApplicationId) -> Self {
        Self { id }
    }
}

/// Retrieves an application reference by id.
pub struct GetApplication {
    pub id: ApplicationId,
}

impl crate::commands::Command for GetApplication {
    fn name(&self) -> &'static str {
        "GetApplication"
    }
}

impl QueryCommand for GetApplication {
    type Output = ApplicationReference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Application)
    }

    fn required_capability(&self) -> Capability {
        Capability::application_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ApplicationReference> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ctx.with_database(|db| ApplicationService::load(db, &self.id))
    }
}

impl GetApplication {
    pub fn new(id: ApplicationId) -> Self {
        Self { id }
    }
}
