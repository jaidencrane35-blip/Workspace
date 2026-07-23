//! Integration tests for Sprint 19 workspace context commands.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_workspace_context::GetWorkspaceContext;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
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
fn context_composes_all_derived_layers() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Context WS".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateZone::new(workspace.id.clone(), "Primary".into(), None))
        .unwrap();

    let context = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetWorkspaceContext::new(workspace.id.clone(), Some(200)))
        .unwrap();

    // State (projection)
    assert_eq!(context.snapshot.workspace_name, "Context WS");
    assert_eq!(context.snapshot.zones.len(), 1);
    // Activity (observations)
    assert!(context
        .observations
        .iter()
        .any(|obs| obs.source_event_type == "zone.created"));
    // Analytics (metrics)
    assert!(context.metrics.resource_change_count >= 2);
    // Authority (capability discovery)
    assert!(!context.capabilities.capabilities.is_empty());
    // ResourceRef identity preserved
    assert_eq!(context.workspace, context.snapshot.workspace);

    assert!(context.validate().is_ok());
}
