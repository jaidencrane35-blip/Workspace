//! Integration tests for Sprint 24 governed execution boundary.

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::execute_intent_request::ExecuteIntentRequest;
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
use crate::security::{AllowAllPermissionGate, PermissionDecision, PermissionGate, PermissionRequest};
use crate::DefaultPermissionPolicy;
use workspace_domain::{ActorContext, Capability, CapabilitySet, IntentContext};

fn ready_ctx<'a>(
    init: &'a crate::commands::initialize::InitializeWorkspaceResult,
    bus: &'a EventBus,
    gate: &'a dyn PermissionGate,
) -> CommandContext<'a> {
    CommandContext {
        actor_context: ActorContext::local_user(),
        intent_context: IntentContext::user_request(),
        capability_set: CapabilitySet::local_user_standard(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: bus,
        permission_gate: gate,
        permission_policy: &AlwaysAllowPolicy,
    }
}

fn seed_through_intent_bridge(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> (workspace_domain::Workspace, String) {
    let workspace = CommandPipeline::new(ready_ctx(init, bus, &AllowAllPermissionGate))
        .execute_mutation(CreateWorkspace::new("Governed Execute".into()))
        .unwrap();
    for index in 0..4 {
        CommandPipeline::new(ready_ctx(init, bus, &AllowAllPermissionGate))
            .execute_mutation(CreateZone::new(
                workspace.id.clone(),
                format!("Zone {index}"),
                None,
            ))
            .unwrap();
    }
    let suggestions = CommandPipeline::new(ready_ctx(init, bus, &AllowAllPermissionGate))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let suggestion_id = suggestions[0].id.clone();
    CommandPipeline::new(ready_ctx(init, bus, &AllowAllPermissionGate))
        .execute_mutation(AcceptSuggestion::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();
    CommandPipeline::new(ready_ctx(init, bus, &AllowAllPermissionGate))
        .execute_mutation(CreateSuggestionIntentRequest::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();
    (workspace, suggestion_id)
}

struct DenyExecuteGate;

impl PermissionGate for DenyExecuteGate {
    fn authorize(
        &self,
        request: &PermissionRequest,
    ) -> crate::error::Result<PermissionDecision> {
        if request.command == "ExecuteIntentRequest" {
            return Ok(PermissionDecision::Denied {
                reason: "execution denied".into(),
            });
        }
        Ok(PermissionDecision::Allowed)
    }
}

#[test]
fn approved_suggestion_executes_through_pipeline() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (workspace, suggestion_id) = seed_through_intent_bridge(&init, &bus);

    let execution = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(ExecuteIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap();

    assert_eq!(execution.action_intent_id.as_str(), "create-zone");
}

#[test]
fn rejected_suggestion_cannot_execute() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(CreateWorkspace::new("Reject Exec".into()))
        .unwrap();
    for index in 0..4 {
        CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
            .execute_mutation(CreateZone::new(
                workspace.id.clone(),
                format!("Zone {index}"),
                None,
            ))
            .unwrap();
    }
    let suggestions = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let suggestion_id = suggestions[0].id.clone();
    CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(RejectSuggestion::new(
            workspace.id.clone(),
            suggestion_id.clone(),
        ))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(ExecuteIntentRequest::new(
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
fn pending_suggestion_without_bridge_cannot_execute() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(CreateWorkspace::new("Pending Exec".into()))
        .unwrap();
    for index in 0..4 {
        CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
            .execute_mutation(CreateZone::new(
                workspace.id.clone(),
                format!("Zone {index}"),
                None,
            ))
            .unwrap();
    }
    let suggestions = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let suggestion_id = suggestions[0].id.clone();

    let error = CommandPipeline::new(ready_ctx(&init, &bus, &AllowAllPermissionGate))
        .execute_mutation(ExecuteIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap_err();

    assert!(matches!(
        error,
        KernelError::SuggestionIntentValidation { .. }
            | KernelError::IntentExecutionValidation { .. }
    ));
}

#[test]
fn permission_denial_blocks_execution() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let (workspace, suggestion_id) = seed_through_intent_bridge(&init, &bus);

    let error = CommandPipeline::new(ready_ctx(&init, &bus, &DenyExecuteGate))
        .execute_mutation(ExecuteIntentRequest::new(
            workspace.id.clone(),
            suggestion_id,
        ))
        .unwrap_err();

    assert!(matches!(error, KernelError::PermissionDenied(_)));
}

#[test]
fn execution_command_requires_audit_write_capability() {
    let command = ExecuteIntentRequest::new(
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
}
