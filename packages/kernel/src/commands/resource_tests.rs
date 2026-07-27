//! Integration tests for Sprint 12 resource services and commands.

use crate::commands::application::{CreateApplication, DeleteApplication, GetApplication};
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::widget::{CreateWidget, DeleteWidget, GetWidget};
use crate::commands::zone::{CreateZone, DeleteZone, GetZone};
use crate::commands::CommandContext;
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::AllowAllPermissionGate;
use crate::services::GraphService;
use workspace_domain::{
    ActorContext, Addressable, ApplicationId, CapabilitySet, IntentContext, WidgetId, WorkspaceId,
    ZoneId,
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
fn create_zone_rejects_missing_workspace() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let ctx = ready_ctx(&init, &bus);

    let error = CommandPipeline::new(ctx)
        .execute_mutation(CreateZone::new(
            WorkspaceId::new("missing-workspace").unwrap(),
            "Orphan".into(),
            None,
        ))
        .unwrap_err();

    assert!(matches!(error, KernelError::WorkspaceNotFound));
}

#[test]
fn delete_zone_removes_graph_node() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let ctx = ready_ctx(&init, &bus);

    let workspace = CommandPipeline::new(ctx)
        .execute_mutation(CreateWorkspace::new("Delete Zone Parent".into()))
        .unwrap();

    let zone = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateZone::new(workspace.id.clone(), "Temporary".into(), None),
    ).unwrap();

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(DeleteZone::new(zone.id.clone()))
        .unwrap();

    init.database.with_database(|db| {
        assert!(!GraphService::exists(db, &zone.resource_ref()).unwrap());
        Ok(())
    }).unwrap();
}

#[test]
fn get_zone_returns_persisted_zone() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Get Zone Parent".into()))
        .unwrap();

    let created = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateZone::new(workspace.id, "Lookup".into(), Some(r#"{"x":0}"#.into())),
    ).unwrap();

    let loaded = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetZone::new(created.id.clone()))
        .unwrap();

    assert_eq!(loaded.name, "Lookup");
    assert_eq!(loaded.position_metadata.as_deref(), Some(r#"{"x":0}"#));
}

#[test]
fn application_lifecycle_through_commands() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("App Parent".into()))
        .unwrap();

    let app = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateApplication::new(
            workspace.id.clone(),
            "Browser".into(),
            Some("com.example.browser".into()),
            None,
        ),
    ).unwrap();

    let loaded = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetApplication::new(app.id.clone()))
        .unwrap();
    assert_eq!(loaded.name, "Browser");

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(DeleteApplication::new(app.id))
        .unwrap();

    let missing = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetApplication::new(ApplicationId::new(loaded.id.to_string()).unwrap()))
        .unwrap_err();
    assert!(matches!(missing, KernelError::ApplicationNotFound));
}

#[test]
fn widget_lifecycle_through_commands() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Widget Parent".into()))
        .unwrap();

    let widget = CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
        CreateWidget::new(workspace.id, "Weather".into(), Some("weather".into())),
    ).unwrap();

    let loaded = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetWidget::new(widget.id.clone()))
        .unwrap();
    assert_eq!(loaded.widget_type.as_deref(), Some("weather"));

    CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(DeleteWidget::new(widget.id.clone()))
        .unwrap();

    init.database.with_database(|db| {
        assert!(!GraphService::exists(db, &widget.resource_ref()).unwrap());
        Ok(())
    }).unwrap();
}

#[test]
fn create_zone_rejects_empty_name() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Validation Parent".into()))
        .unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateZone::new(workspace.id, "   ".into(), None))
        .unwrap_err();

    assert!(matches!(error, KernelError::Domain(_)));
}

#[test]
fn get_zone_returns_not_found_for_missing_id() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_query(GetZone::new(ZoneId::new("missing-zone").unwrap()))
        .unwrap_err();

    assert!(matches!(error, KernelError::ZoneNotFound));
}

#[test]
fn mutation_audit_records_resource_ref() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Audit Ref".into()))
        .unwrap();

    let records = crate::services::AuditService::list_recent(&init.database.shared(), 5).unwrap();

    assert!(records.iter().any(|record| {
        record.resource_ref.as_deref() == Some(workspace.resource_ref().canonical().as_str())
    }));
}

#[test]
fn duplicate_graph_registration_is_rejected() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(CreateWorkspace::new("Dup Graph".into()))
        .unwrap();

    let error = init.database.with_database(|db| {
        GraphService::register_node(db, &workspace.resource_ref())
    }).unwrap_err();

    assert!(matches!(error, KernelError::DuplicateResource { .. }));
}

#[test]
fn delete_widget_returns_not_found_for_missing_id() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let error = CommandPipeline::new(ready_ctx(&init, &bus))
        .execute_mutation(DeleteWidget::new(WidgetId::new("missing-widget").unwrap()))
        .unwrap_err();

    assert!(matches!(error, KernelError::WidgetNotFound));
}
