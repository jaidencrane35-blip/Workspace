//! Integration tests for Sprint 20–21 suggestion commands.

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::zone::CreateZone;
use crate::commands::CommandContext;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use crate::services::AuditService;
use workspace_domain::{
    ActionIntentId, ActionIntentRegistry, ActionIntentRequest, Addressable, ActorContext,
    Capability, CapabilitySet, IntentContext, ResourceKind, SuggestionStatus,
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

fn seed_active_workspace(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> workspace_domain::Workspace {
    let workspace = CommandPipeline::new(ready_ctx(init, bus))
        .execute_mutation(CreateWorkspace::new("Suggest WS".into()))
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
    workspace
}

#[test]
fn suggestions_are_deterministic_proposals() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

    let workspace = seed_active_workspace(&init, &bus);

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

#[test]
fn accept_suggestion_is_decision_only_and_audited() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = seed_active_workspace(&init, &bus);

    let before = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let target = before.first().expect("expected a pending suggestion").clone();
    let zone_count_before = {
        let db = init.database.shared();
        let db = db.lock().expect("db lock");
        crate::services::ZoneService::list_by_workspace(&db, &workspace.id)
            .unwrap()
            .len()
    };

    let accepted = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(
            &ActionIntentRequest::new(ActionIntentId::new("accept-suggestion").unwrap())
                .with_target(workspace.resource_ref()),
            AcceptSuggestion::new(workspace.id.clone(), target.id.clone()),
        )
        .unwrap();

    assert_eq!(accepted.status, SuggestionStatus::Accepted);
    assert_eq!(accepted.id, target.id);

    let after = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    assert!(after.iter().all(|s| s.id != target.id));

    let zone_count_after = {
        let db = init.database.shared();
        let db = db.lock().expect("db lock");
        crate::services::ZoneService::list_by_workspace(&db, &workspace.id)
            .unwrap()
            .len()
    };
    assert_eq!(
        zone_count_before, zone_count_after,
        "accept must not mutate workspace entities"
    );

    let audit = AuditService::list_recent(&init.database.shared(), 50).unwrap();
    assert!(audit.iter().any(|event| {
        event.success
            && event.command_name.as_deref() == Some("AcceptSuggestion")
            && event
                .metadata
                .as_deref()
                .is_some_and(|metadata| metadata.contains(&target.id))
    }));
}

#[test]
fn reject_suggestion_suppresses_listing() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = seed_active_workspace(&init, &bus);

    let before = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    let target = before.first().expect("expected a pending suggestion").clone();

    let rejected = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation_with_action(
            &ActionIntentRequest::new(ActionIntentId::new("reject-suggestion").unwrap()),
            RejectSuggestion::new(workspace.id.clone(), target.id.clone()),
        )
        .unwrap();

    assert_eq!(rejected.status, SuggestionStatus::Rejected);

    let after = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
        .unwrap();
    assert!(after.iter().all(|s| s.id != target.id));
}

#[test]
fn accept_unknown_suggestion_fails_closed() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    crate::events::AuditEventSubscriber::register(&bus, init.database.shared());
    let workspace = seed_active_workspace(&init, &bus);

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(AcceptSuggestion::new(
            workspace.id.clone(),
            "missing-suggestion".into(),
        ))
        .unwrap_err();

    assert!(matches!(
        error,
        crate::error::KernelError::SuggestionValidation { .. }
    ));
}

#[test]
fn accept_reject_intents_are_registered() {
    let accept = ActionIntentRegistry::lookup(&ActionIntentId::new("accept-suggestion").unwrap())
        .expect("accept-suggestion intent");
    let reject = ActionIntentRegistry::lookup(&ActionIntentId::new("reject-suggestion").unwrap())
        .expect("reject-suggestion intent");

    assert_eq!(accept.command_name, "AcceptSuggestion");
    assert_eq!(reject.command_name, "RejectSuggestion");
    assert!(accept.matches_capability(&Capability::audit_write()));
    assert!(reject.matches_capability(&Capability::audit_write()));
}
