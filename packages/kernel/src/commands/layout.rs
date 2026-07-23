use crate::commands::context::CommandContext;
use crate::commands::resource::{
    ensure_graph_nodes_exist, ensure_ready, ensure_workspace_exists, layout_lifecycle_event,
};
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::events::types::DomainEvent;
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::LayoutService;
use workspace_domain::{
    Capability, Layout, LayoutId, LayoutMetadata, LayoutNode, LayoutSnapshot, ResourceKind,
    ResourceRef, Viewport, WorkspaceId,
};

/// Creates an empty layout for a workspace.
pub struct CreateLayout {
    pub workspace_id: WorkspaceId,
}

impl crate::commands::Command for CreateLayout {
    fn name(&self) -> &'static str {
        "CreateLayout"
    }
}

impl MutationCommand for CreateLayout {
    type Output = Layout;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.workspace_resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Layout> {
        ensure_ready(ctx)?;
        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let layout = ctx.with_database(|db| LayoutService::create(db, self.workspace_id.clone()))?;
        ctx.event_bus.publish(DomainEvent::LayoutCreated(layout_lifecycle_event(
            &layout,
            ctx,
            Capability::layout_write(),
        )));
        Ok(layout)
    }
}

impl CreateLayout {
    pub fn new(workspace_id: WorkspaceId) -> Self {
        Self { workspace_id }
    }
}

/// Updates layout viewport and nodes.
pub struct UpdateLayout {
    pub layout_id: LayoutId,
    pub viewport: Viewport,
    pub nodes: Vec<LayoutNode>,
    pub metadata: Option<LayoutMetadata>,
}

impl crate::commands::Command for UpdateLayout {
    fn name(&self) -> &'static str {
        "UpdateLayout"
    }
}

impl MutationCommand for UpdateLayout {
    type Output = Layout;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.workspace_resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Layout> {
        ensure_ready(ctx)?;
        ensure_graph_nodes_exist(ctx, &self.nodes)?;

        let layout = ctx.with_database(|db| {
            LayoutService::update(
                db,
                &self.layout_id,
                self.viewport,
                self.nodes.clone(),
                self.metadata.clone(),
            )
        })?;

        ctx.event_bus.publish(DomainEvent::LayoutUpdated(layout_lifecycle_event(
            &layout,
            ctx,
            Capability::layout_write(),
        )));
        Ok(layout)
    }
}

impl UpdateLayout {
    pub fn new(
        layout_id: LayoutId,
        viewport: Viewport,
        nodes: Vec<LayoutNode>,
        metadata: Option<LayoutMetadata>,
    ) -> Self {
        Self {
            layout_id,
            viewport,
            nodes,
            metadata,
        }
    }
}

/// Deletes a workspace layout.
pub struct DeleteLayout {
    pub layout_id: LayoutId,
    pub workspace_id: WorkspaceId,
}

impl crate::commands::Command for DeleteLayout {
    fn name(&self) -> &'static str {
        "DeleteLayout"
    }
}

impl MutationCommand for DeleteLayout {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_write()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Workspace,
            workspace_domain::ResourceId::new(self.workspace_id.as_str())
                .expect("workspace id is non-empty"),
        ))
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        ensure_ready(ctx)?;
        ctx.with_database(|db| LayoutService::delete(db, &self.layout_id))?;
        ctx.event_bus.publish(DomainEvent::LayoutDeleted(
            crate::events::types::LayoutLifecycleEvent {
                layout_id: self.layout_id.clone(),
                workspace_id: self.workspace_id.clone(),
                resource_ref: self.audit_resource_ref(&()).expect("workspace ref"),
                actor: Some(ctx.actor_context.clone()),
                intent: Some(ctx.intent_context.clone()),
                capability: Some(Capability::layout_write()),
            },
        ));
        Ok(())
    }
}

impl DeleteLayout {
    pub fn new(layout_id: LayoutId, workspace_id: WorkspaceId) -> Self {
        Self {
            layout_id,
            workspace_id,
        }
    }
}

/// Resets a layout to default viewport with no nodes.
pub struct ResetLayout {
    pub layout_id: LayoutId,
}

impl crate::commands::Command for ResetLayout {
    fn name(&self) -> &'static str {
        "ResetLayout"
    }
}

impl MutationCommand for ResetLayout {
    type Output = Layout;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(output.workspace_resource_ref())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Layout> {
        ensure_ready(ctx)?;
        let layout = ctx.with_database(|db| LayoutService::reset(db, &self.layout_id))?;
        ctx.event_bus.publish(DomainEvent::LayoutReset(layout_lifecycle_event(
            &layout,
            ctx,
            Capability::layout_write(),
        )));
        Ok(layout)
    }
}

impl ResetLayout {
    pub fn new(layout_id: LayoutId) -> Self {
        Self { layout_id }
    }
}

/// Retrieves a layout by workspace id.
pub struct GetLayout {
    pub workspace_id: WorkspaceId,
}

impl crate::commands::Command for GetLayout {
    fn name(&self) -> &'static str {
        "GetLayout"
    }
}

impl QueryCommand for GetLayout {
    type Output = Layout;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Layout> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ctx.with_database(|db| LayoutService::load_by_workspace(db, &self.workspace_id))
    }
}

impl GetLayout {
    pub fn new(workspace_id: WorkspaceId) -> Self {
        Self { workspace_id }
    }
}

/// Retrieves a point-in-time layout snapshot.
pub struct GetLayoutSnapshot {
    pub layout_id: LayoutId,
}

impl crate::commands::Command for GetLayoutSnapshot {
    fn name(&self) -> &'static str {
        "GetLayoutSnapshot"
    }
}

impl QueryCommand for GetLayoutSnapshot {
    type Output = LayoutSnapshot;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::layout_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<LayoutSnapshot> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let snapshot =
            ctx.with_database(|db| LayoutService::snapshot(db, &self.layout_id))?;
        let layout = ctx.with_database(|db| LayoutService::load(db, &self.layout_id))?;
        ctx.event_bus.publish(DomainEvent::LayoutSnapshot(layout_lifecycle_event(
            &layout,
            &ctx,
            Capability::layout_read(),
        )));
        Ok(snapshot)
    }
}

impl GetLayoutSnapshot {
    pub fn new(layout_id: LayoutId) -> Self {
        Self { layout_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext, LayoutBounds, Position2D, Size2D};

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
    fn create_and_get_layout_for_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Layout WS".into()))
            .unwrap();

        let layout = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateLayout::new(workspace.id.clone()))
            .unwrap();

        let loaded = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetLayout::new(workspace.id))
            .unwrap();

        assert_eq!(loaded.id, layout.id);
    }

    #[test]
    fn update_layout_rejects_missing_graph_node() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Missing Node".into()))
            .unwrap();
        let layout = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateLayout::new(workspace.id))
            .unwrap();

        let orphan_node = LayoutNode {
            resource_ref: ResourceRef::new(
                ResourceKind::Zone,
                workspace_domain::ResourceId::new("missing-zone").unwrap(),
            ),
            bounds: LayoutBounds {
                position: Position2D::new(0.0, 0.0),
                size: Size2D::new(100.0, 100.0),
            },
            z_index: 0,
            collapsed: false,
            hidden: false,
            locked: false,
            metadata: None,
        };

        let error = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(UpdateLayout::new(
                layout.id,
                Viewport::default(),
                vec![orphan_node],
                None,
            ))
            .unwrap_err();

        assert!(matches!(error, KernelError::LayoutValidation { .. }));
    }
}
