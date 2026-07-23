//! Integration tests for Sprint 22 suggestion lifecycle commands.

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_suggestion_lifecycle::GetSuggestionLifecycle;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{
    ActionIntentId, ActionIntentRequest, ActorContext, CapabilitySet, IntentContext,
    SuggestionLifecycleState,
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

fn seed_workspace_with_suggestion(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> (workspace_domain::Workspace, String) {
    let workspace = CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(CreateWorkspace::new("Lifecycle WS".into()))
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
fn lifecycle_records_derived_from_accept_and_reject_audit() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let (workspace, suggestion_id) = seed_workspace_with_suggestion(&init, &bus);

    let accepted = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(
            &ActionIntentRequest::new(ActionIntentId::new("accept-suggestion").unwrap()),
            AcceptSuggestion::new(workspace.id.clone(), suggestion_id.clone()),
        )
        .unwrap();
    assert_eq!(accepted.id, suggestion_id);

    let records = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestionLifecycle::new(Some(50)))
        .unwrap();

    assert!(records.iter().any(|record| {
        record.suggestion_id == suggestion_id
            && record.state == SuggestionLifecycleState::Accepted
    }));
}

#[test]
fn unrelated_audit_events_are_excluded() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Quiet".into()))
        .unwrap();

    let records = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestionLifecycle::new(Some(50)))
        .unwrap();

    assert!(records.is_empty());
}

#[test]
fn command_bookkeeping_is_not_lifecycle() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    seed_workspace_with_suggestion(&init, &bus);

    let records = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestionLifecycle::new(Some(200)))
        .unwrap();

    assert!(records
        .iter()
        .all(|record| matches!(record.state, SuggestionLifecycleState::Accepted)));
}

#[test]
fn reject_records_appear_in_lifecycle_projection() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let (workspace, suggestion_id) = seed_workspace_with_suggestion(&init, &bus);

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(
            &ActionIntentRequest::new(ActionIntentId::new("reject-suggestion").unwrap()),
            RejectSuggestion::new(workspace.id.clone(), suggestion_id.clone()),
        )
        .unwrap();

    let records = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestionLifecycle::new(Some(50)))
        .unwrap();

    assert!(records.iter().any(|record| {
        record.suggestion_id == suggestion_id
            && record.state == SuggestionLifecycleState::Rejected
    }));
}
