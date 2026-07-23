//! Integration tests for Sprint 23 suggestion intent bridge commands.

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::commands::r#trait::{permission_request, require_policy, Command, MutationCommand};
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::{AlwaysAllowPolicy, PermissionPolicy};
use crate::DefaultPermissionPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{ActorContext, Capability, CapabilitySet, IntentContext};

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

fn seed_workspace_with_suggestion(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> (workspace_domain::Workspace, String) {
    let workspace = CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(CreateWorkspace::new("Intent WS".into()))
        .unwrap();
    for index in 0..4 {
        CommandPipeline::new(ready_ctx(init, bus))
            .execute_mutation(CreateZone::new(
                workspace.id.clone(),
                format!("Zone {index}"),
                None,
            ))
            .unwrap();
    }
    let suggestions = CommandPipeline::new(ready_ctx(init, bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    (workspace, suggestions[0].id.clone())
}

#[test]
fn bridge_routes_through_pipeline_with_permission() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (workspace, suggestion_id) = seed_workspace_with_suggestion(&init, &bus);

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(AcceptSuggestion::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();

    let request = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateSuggestionIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap();

    assert_eq!(request.intent_id.as_str(), "create-zone");
}

#[test]
fn rejected_suggestion_cannot_create_intent_request() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (workspace, suggestion_id) = seed_workspace_with_suggestion(&init, &bus);

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(RejectSuggestion::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateSuggestionIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap_err();

    assert!(matches!(
        error,
        KernelError::SuggestionIntentValidation { .. }
    ));
}

#[test]
fn pending_suggestion_cannot_create_intent_request() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (workspace, suggestion_id) = seed_workspace_with_suggestion(&init, &bus);

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateSuggestionIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap_err();

    assert!(matches!(
        error,
        KernelError::SuggestionIntentValidation { .. }
    ));
}

#[test]
fn permission_required_for_bridge_command() {
    let command = CreateSuggestionIntentRequest::new(
        workspace_domain::WorkspaceId::new("ws-1").unwrap(),
        "s-1".into(),
    );
    assert_eq!(command.required_capability(), Capability::audit_write());

    let request = permission_request(
        &ActorContext::local_user(),
        &IntentContext::user_request(),
        command.name(),
        command.permission_subject(),
        command.required_capability(),
    );
    let policy = DefaultPermissionPolicy;
    let context = crate::commands::r#trait::policy_context(&request);
    let result = policy.evaluate(&context).unwrap();
    require_policy(result).expect("default policy allows local user");
    assert_eq!(request.capability.id.as_str(), "audit.write");
}
