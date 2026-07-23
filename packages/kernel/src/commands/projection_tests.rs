//! Integration tests for Sprint 14 workspace projection commands.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_workspace_snapshot::GetWorkspaceSnapshot;
use crate::commands::layout::CreateLayout;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::widget::CreateWidget;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
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

#[test]
fn snapshot_aggregates_workspace_resources() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Full Snapshot".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateZone::new(workspace.id.clone(), "Zone".into(), None),
    ).unwrap();
    CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateApplication::new(workspace.id.clone(), "App".into(), None, None),
    ).unwrap();
    CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateWidget::new(workspace.id.clone(), "Widget".into(), None),
    ).unwrap();
    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateLayout::new(workspace.id.clone()))
        .unwrap();

    let snapshot = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetWorkspaceSnapshot::new(workspace.id))
        .unwrap();

    assert_eq!(snapshot.zones.len(), 1);
    assert_eq!(snapshot.applications.len(), 1);
    assert_eq!(snapshot.widgets.len(), 1);
    assert_eq!(snapshot.relationships.len(), 3);
    assert!(snapshot.layout_id.is_some());
}
