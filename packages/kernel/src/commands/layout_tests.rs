//! Integration tests for Sprint 13 layout commands.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::layout::{
    CreateLayout, DeleteLayout, GetLayout, GetLayoutSnapshot, ResetLayout, UpdateLayout,
};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{
    Addressable, ActorContext, CapabilitySet, IntentContext, LayoutBounds, LayoutNode, Position2D,
    Size2D, Viewport,
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
fn reset_layout_clears_nodes() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Reset".into()))
        .unwrap();
    let zone = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateZone::new(workspace.id.clone(), "Zone".into(), None),
    ).unwrap();
    let layout = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id.clone()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(UpdateLayout::new(
            layout.id.clone(),
            Viewport::default(),
            vec![LayoutNode {
                resource_ref: zone.resource_ref(),
                bounds: LayoutBounds {
                    position: Position2D::new(5.0, 5.0),
                    size: Size2D::new(50.0, 50.0),
                },
                z_index: 0,
                collapsed: false,
                hidden: false,
                locked: false,
                metadata: None,
            }],
            None,
        ))
        .unwrap();

    let reset = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(ResetLayout::new(layout.id.clone()))
        .unwrap();
    assert!(reset.nodes.is_empty());
}

#[test]
fn delete_layout_removes_persisted_record() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Delete Layout".into()))
        .unwrap();
    let layout = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id.clone()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(DeleteLayout::new(
        layout.id.clone(),
        workspace.id.clone(),
    )).unwrap();

    let missing = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetLayout::new(workspace.id))
        .unwrap_err();
    assert!(matches!(missing, KernelError::LayoutNotFound));
}

#[test]
fn get_layout_snapshot_returns_captured_state() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Snapshot".into()))
        .unwrap();
    let layout = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id))
        .unwrap();

    let layout_id = layout.id.clone();
    let snapshot = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetLayoutSnapshot::new(layout_id.clone()))
        .unwrap();

    assert_eq!(snapshot.layout_id, layout_id);
    assert!(!snapshot.captured_at.is_empty());
}

#[test]
fn create_layout_rejects_duplicate_workspace_layout() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Dup".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id.clone()))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id))
        .unwrap_err();
    assert!(matches!(error, KernelError::DuplicateLayout));
}

#[test]
fn update_layout_rejects_negative_size() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Negative".into()))
        .unwrap();
    let zone = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateZone::new(workspace.id.clone(), "Z".into(), None),
    ).unwrap();
    let layout = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(UpdateLayout::new(
            layout.id,
            Viewport::default(),
            vec![LayoutNode {
                resource_ref: zone.resource_ref(),
                bounds: LayoutBounds {
                    position: Position2D::new(0.0, 0.0),
                    size: Size2D::new(-10.0, 20.0),
                },
                z_index: 0,
                collapsed: false,
                hidden: false,
                locked: false,
                metadata: None,
            }],
            None,
        ))
        .unwrap_err();

    assert!(matches!(error, KernelError::LayoutValidation { .. }));
}

#[test]
fn update_layout_rejects_invalid_viewport() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Viewport".into()))
        .unwrap();
    let layout = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id))
        .unwrap();

    let invalid_viewport = Viewport {
        origin: Position2D::new(0.0, 0.0),
        size: Size2D::new(100.0, 100.0),
        zoom: 0.0,
    };

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(UpdateLayout::new(layout.id, invalid_viewport, vec![], None))
        .unwrap_err();

    assert!(matches!(error, KernelError::LayoutValidation { .. }));
}

#[test]
fn layout_mutation_audit_records_workspace_resource_ref() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Audit Layout".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id.clone()))
        .unwrap();

    let records = crate::services::AuditService::list_recent(&init.database.shared(), 10).unwrap();
    assert!(records.iter().any(|record| {
        record.command_name.as_deref() == Some("CreateLayout")
            && record.resource_ref.as_deref() == Some(workspace.resource_ref().canonical().as_str())
    }));
}

#[test]
fn get_layout_returns_not_found_for_missing_workspace_layout() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("No Layout".into()))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetLayout::new(workspace.id))
        .unwrap_err();

    assert!(matches!(error, KernelError::LayoutNotFound));
}
