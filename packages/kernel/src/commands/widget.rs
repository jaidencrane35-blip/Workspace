use crate::commands::context::CommandContext;
use crate::commands::resource::{ensure_ready, ensure_workspace_exists, resource_lifecycle_event};
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, ResourceLifecycleEvent};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WidgetService;
use workspace_domain::{
    Addressable, Capability, ResourceId, ResourceKind, ResourceRef, WidgetId, WidgetReference,
    WorkspaceId,
};

/// Creates a widget reference within a workspace.
pub struct CreateWidget {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub widget_type: Option<String>,
}

impl crate::commands::Command for CreateWidget {
    fn name(&self) -> &'static str {
        "CreateWidget"
    }
}

impl MutationCommand for CreateWidget {
    type Output = WidgetReference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Widget)
    }

    fn required_capability(&self) -> Capability {
        Capability::widget_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<WidgetReference> {
        ensure_ready(ctx)?;
        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let widget = ctx.with_database(|db| {
            WidgetService::create(
                db,
                self.workspace_id.clone(),
                self.name.clone(),
                self.widget_type.clone(),
            )
        })?;

        let lifecycle = resource_lifecycle_event(
            widget.resource_ref(),
            Some(widget.workspace_id.to_string()),
            ctx,
            Capability::widget_write(),
        );
        ctx.event_bus
            .publish(DomainEvent::ResourceCreated(lifecycle.clone()));
        ctx.event_bus.publish(DomainEvent::WidgetCreated(lifecycle));

        Ok(widget)
    }
}

impl CreateWidget {
    pub fn new(
        workspace_id: WorkspaceId,
        name: String,
        widget_type: Option<String>,
    ) -> Self {
        Self {
            workspace_id,
            name,
            widget_type,
        }
    }
}

/// Deletes a widget reference by id.
pub struct DeleteWidget {
    pub id: WidgetId,
}

impl crate::commands::Command for DeleteWidget {
    fn name(&self) -> &'static str {
        "DeleteWidget"
    }
}

impl MutationCommand for DeleteWidget {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Widget)
    }

    fn required_capability(&self) -> Capability {
        Capability::widget_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Widget,
            ResourceId::new(self.id.as_str()).expect("widget id is non-empty"),
        ))
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        ensure_ready(ctx)?;

        let widget = ctx.with_database(|db| WidgetService::load(db, &self.id))?;
        let resource_ref = widget.resource_ref();

        ctx.with_database(|db| WidgetService::delete(db, &self.id))?;

        ctx.event_bus.publish(DomainEvent::ResourceDeleted(ResourceLifecycleEvent {
            resource_ref: resource_ref.clone(),
            workspace_id: Some(widget.workspace_id.to_string()),
            actor: Some(ctx.actor_context.clone()),
            intent: Some(ctx.intent_context.clone()),
            capability: Some(Capability::widget_write()),
        }));

        Ok(())
    }
}

impl DeleteWidget {
    pub fn new(id: WidgetId) -> Self {
        Self { id }
    }
}

/// Retrieves a widget reference by id.
pub struct GetWidget {
    pub id: WidgetId,
}

impl crate::commands::Command for GetWidget {
    fn name(&self) -> &'static str {
        "GetWidget"
    }
}

impl QueryCommand for GetWidget {
    type Output = WidgetReference;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Widget)
    }

    fn required_capability(&self) -> Capability {
        Capability::widget_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WidgetReference> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ctx.with_database(|db| WidgetService::load(db, &self.id))
    }
}

impl GetWidget {
    pub fn new(id: WidgetId) -> Self {
        Self { id }
    }
}
