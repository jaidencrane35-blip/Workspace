//! Integration tests for Sprint 25 execution outcome projection.

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::execute_intent_request::ExecuteIntentRequest;
use crate::commands::get_execution_outcomes::GetExecutionOutcomes;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use workspace_domain::{
    ActorContext, Capability, CapabilitySet, ExecutionOutcomeStatus, IntentContext,
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

fn seed_through_execution(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> (workspace_domain::Workspace, String) {
    let workspace = CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(CreateWorkspace::new("Outcome Integration".into()))
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
    let suggestion_id = suggestions[0].id.clone();
    CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(AcceptSuggestion::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();
    CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(CreateSuggestionIntentRequest::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();
    CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(ExecuteIntentRequest::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();
    (workspace, suggestion_id)
}

#[test]
fn outcomes_derived_from_successful_execute_intent_request() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (_, suggestion_id) = seed_through_execution(&init, &bus);

    let outcomes = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetExecutionOutcomes::new(Some(50)))
        .unwrap();

    assert!(outcomes.iter().any(|outcome| {
        outcome.status == ExecutionOutcomeStatus::Completed
            && outcome.suggestion_id.as_deref() == Some(suggestion_id.as_str())
            && outcome.execution_request_id == format!("execution:{suggestion_id}")
    }));
}

#[test]
fn failed_execute_intent_request_yields_failed_outcome() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Fail Outcome".into()))
        .unwrap();

    let _ = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        ExecuteIntentRequest::new(workspace.id.clone(), "not-accepted".into()),
    );

    let outcomes = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetExecutionOutcomes::new(Some(50)))
        .unwrap();

    assert!(outcomes.iter().any(|outcome| {
        outcome.status == ExecutionOutcomeStatus::Failed
            && outcome.suggestion_id.as_deref() == Some("not-accepted")
            && outcome.failure_reason.is_some()
    }));
}

#[test]
fn unrelated_audit_events_are_excluded() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("No Outcomes".into()))
        .unwrap();

    let outcomes = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetExecutionOutcomes::new(Some(50)))
        .unwrap();

    assert!(outcomes.is_empty());
}

#[test]
fn governance_classification_is_audit_read_governed() {
    let command = GetExecutionOutcomes::new(None);
    assert_eq!(command.required_capability(), Capability::audit_read());
    assert_eq!(
        command.governance_class(),
        crate::policy::GovernanceClass::Governed
    );
}
