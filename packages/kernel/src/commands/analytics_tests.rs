//! Integration tests for Sprint 18 analytics commands.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_workspace_metrics::GetWorkspaceMetrics;
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
fn metrics_aggregate_resource_and_lifecycle_activity() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Analyzed".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateZone::new(workspace.id.clone(), "Primary".into(), None))
        .unwrap();

    let metrics = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetWorkspaceMetrics::new(Some(200)))
        .unwrap();

    // workspace.entity.created + resource.created + zone.created are all
    // ResourceChange creations.
    assert!(metrics.resource_change_count >= 3);
    assert!(metrics.resource_creation_count >= 3);
    assert_eq!(metrics.resource_deletion_count, 0);

    let sum = metrics.lifecycle_activity_count
        + metrics.resource_change_count
        + metrics.layout_change_count
        + metrics.settings_change_count;
    assert_eq!(sum, metrics.observation_count);
    assert!(metrics.validate().is_ok());
}

#[test]
fn metrics_are_empty_without_activity() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let metrics = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetWorkspaceMetrics::new(Some(200)))
        .unwrap();

    assert_eq!(metrics.observation_count, 0);
    assert_eq!(metrics.activity_by_category.len(), 4);
    assert!(metrics.validate().is_ok());
}
