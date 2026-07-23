//! Integration tests for Sprint 17 observation commands.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_observations::GetObservations;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{classify_event, neutral_summary, ActorContext, CapabilitySet, IntentContext};

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
fn observations_include_resource_activity_and_exclude_command_bookkeeping() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Observed".into()))
        .unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateZone::new(workspace.id.clone(), "Primary".into(), None))
        .unwrap();

    let observations = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetObservations::new(Some(100)))
        .unwrap();

    let sources: Vec<&str> = observations
        .iter()
        .map(|obs| obs.source_event_type.as_str())
        .collect();

    assert!(sources.contains(&"resource.created"));
    assert!(sources.contains(&"zone.created"));
    assert!(!sources.contains(&"command.executed"));
}

#[test]
fn observations_are_valid_and_classification_matches() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Observed".into()))
        .unwrap();

    let observations = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetObservations::new(Some(100)))
        .unwrap();

    assert!(!observations.is_empty());
    for observation in observations {
        assert!(observation.validate().is_ok());

        let (category, importance) =
            classify_event(&observation.source_event_type).expect("classified event");
        assert_eq!(observation.category, category);
        assert_eq!(observation.importance, importance);
        assert_eq!(
            observation.summary,
            neutral_summary(category, &observation.source_event_type)
        );
    }
}
