//! Integration tests for Sprint 15 action intent execution.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_workspace::GetWorkspace;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::update_settings::UpdateSettings;
use crate::commands::CommandContext;
use crate::config::SettingsUpdate;
use crate::error::KernelError;
use crate::events::EventBus;
use crate::intent::{CommandIntentMapping, ActionIntentValidationService};
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{
    ActionIntentId, ActionIntentRegistry, ActionIntentRequest, ActorContext, Capability,
    CapabilitySet, IntentContext,
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
fn action_intent_executes_create_workspace_through_pipeline() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let action = ActionIntentRequest::new(ActionIntentId::new("create-workspace").unwrap());

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(&action, CreateWorkspace::new("Intent WS".into()))
        .unwrap();

    assert_eq!(workspace.name, "Intent WS");
}

#[test]
fn action_intent_rejects_capability_mismatch_before_pipeline() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let action = ActionIntentRequest::new(ActionIntentId::new("create-workspace").unwrap());

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(
            &action,
            UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
                active_workspace_id: None,
                personalization_enabled: None,
            }),
        )
        .unwrap_err();

    assert!(matches!(
        error,
        KernelError::ActionIntentCommandMismatch { .. }
    ));
}

#[test]
fn all_registry_intents_map_to_known_commands() {
    for definition in ActionIntentRegistry::all() {
        assert_eq!(
            CommandIntentMapping::resolve_command_name(&definition.id).unwrap(),
            definition.command_name
        );
    }
}

#[test]
fn mutation_command_mappings_match_capabilities() {
    let checks = [
        ("CreateWorkspace", "create-workspace", Capability::workspace_write()),
        ("UpdateSettings", "update-settings", Capability::settings_write()),
        ("CreateZone", "create-zone", Capability::zone_write()),
        ("DeleteZone", "delete-zone", Capability::zone_write()),
        ("CreateApplication", "create-application", Capability::application_write()),
        ("DeleteApplication", "delete-application", Capability::application_write()),
        ("LaunchApplication", "launch-application", Capability::application_launch()),
        ("DecideApproval", "decide-approval", Capability::audit_write()),
        ("CreateWidget", "create-widget", Capability::widget_write()),
        ("DeleteWidget", "delete-widget", Capability::widget_write()),
        ("CreateLayout", "create-layout", Capability::layout_write()),
        ("UpdateLayout", "update-layout", Capability::layout_write()),
        ("DeleteLayout", "delete-layout", Capability::layout_write()),
        ("ResetLayout", "reset-layout", Capability::layout_write()),
    ];

    for (command_name, intent_id, capability) in checks {
        let id = ActionIntentId::new(intent_id).unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        assert_eq!(definition.command_name, command_name);
        ActionIntentValidationService::validate_capability_match(&definition, &capability).unwrap();
    }
}

#[test]
fn read_action_intent_supports_get_workspace() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Read Intent".into()))
        .unwrap();

    let action = ActionIntentRequest::new(ActionIntentId::new("get-workspace").unwrap());
    let loaded = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query_with_action(&action, GetWorkspace::new(workspace.id))
        .unwrap();

    assert_eq!(loaded.name, "Read Intent");
}
