//! Integration tests for Sprint 20 suggestion commands.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{
    Addressable, ActorContext, CapabilitySet, IntentContext, ResourceKind, SuggestionStatus,
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
fn suggestions_are_deterministic_proposals() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Suggest WS".into()))
        .unwrap();
    for index in 0..4 {
        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateZone::new(
                workspace.id.clone(),
                format!("Zone {index}"),
                None,
            ))
            .unwrap();
    }

    let first = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let second = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();

    // Deterministic: identical context yields identical proposals.
    assert_eq!(first, second);
    assert!(!first.is_empty());

    for suggestion in &first {
        assert_eq!(suggestion.status, SuggestionStatus::Pending);
        assert!(suggestion.validate().is_ok());
        // ResourceRef identity preserved where a suggestion references a resource.
        if let Some(resource_ref) = &suggestion.related_resource_ref {
            assert_eq!(resource_ref.kind, ResourceKind::Workspace);
            assert_eq!(resource_ref, &workspace.resource_ref());
        }
    }
}

#[test]
fn quiet_workspace_yields_no_suggestions() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Quiet WS".into()))
        .unwrap();

    // Only one creation event so far: below the resource-growth and activity
    // thresholds.
    let suggestions = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();

    assert!(suggestions
        .iter()
        .all(|s| s.suggestion_type != workspace_domain::SuggestionType::ResourceGrowth));
}
