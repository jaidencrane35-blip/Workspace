//! Integration tests for Sprint 16 capability discovery commands.

use crate::commands::get_actor_capabilities::GetActorCapabilities;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{ActionIntentId, ActorContext, CapabilitySet, IntentContext};

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
fn discovery_includes_create_workspace_intent_for_local_user() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let discovery = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetActorCapabilities)
        .unwrap();

    let create_workspace = ActionIntentId::new("create-workspace").unwrap();
    assert!(discovery
        .available_intent_ids()
        .iter()
        .any(|intent_id| **intent_id == create_workspace));
}
