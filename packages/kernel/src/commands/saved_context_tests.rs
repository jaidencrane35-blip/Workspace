//! Command-level acceptance for saving a bounded workspace context (PP-M1-01).
//!
//! Every case here stops before the desktop is read. The capture itself is
//! covered deterministically in `services::saved_context` with a stub capturer,
//! so these tests deliberately never observe the real machine.

use crate::commands::context::CommandContext;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::initialize::{InitializeWorkspace, InitializeWorkspaceResult};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::saved_context::{GetSavedContextCaptureScope, SaveWorkspaceContext};
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::AlwaysAllowPolicy;
use crate::security::PermissionDecision;
use crate::security::{AllowAllPermissionGate, PermissionGate, PermissionRequest};
use crate::services::AuditService;
use workspace_domain::{
    ActorContext, Capability, CapabilitySet, IntentContext, SaveContextRequest, Workspace,
    WorkspaceId, SAVED_CONTEXT_SCOPE_ID,
};

struct DenyAllPermissionGate;

impl PermissionGate for DenyAllPermissionGate {
    fn authorize(&self, request: &PermissionRequest) -> crate::error::Result<PermissionDecision> {
        Ok(PermissionDecision::Denied {
            reason: format!("denied: {}", request.command),
        })
    }
}

fn context<'a>(
    bus: &'a EventBus,
    init: &'a InitializeWorkspaceResult,
    gate: &'a dyn PermissionGate,
    capability_set: CapabilitySet,
) -> CommandContext<'a> {
    CommandContext {
        actor_context: ActorContext::local_user(),
        intent_context: IntentContext::user_request(),
        capability_set,
        state: &init.state,
        database: init.database.shared(),
        event_bus: bus,
        permission_gate: gate,
        permission_policy: &AlwaysAllowPolicy,
    }
}

fn seed_workspace(bus: &EventBus, init: &InitializeWorkspaceResult) -> Workspace {
    CommandPipeline::new(context(
        bus,
        init,
        &AllowAllPermissionGate,
        CapabilitySet::local_user_standard(),
    ))
    .execute_mutation(CreateWorkspace::new("Product Proof".into()))
    .unwrap()
}

fn request(workspace_id: &WorkspaceId, scope: &str) -> SaveContextRequest {
    SaveContextRequest::new(
        workspace_id.clone(),
        "Tuesday review",
        scope,
        "Finish the client proposal outline",
    )
}

fn saved_context_count(init: &InitializeWorkspaceResult) -> i64 {
    let db = init.database.shared();
    let guard = db.lock().unwrap();
    guard
        .connection()
        .query_row("SELECT COUNT(*) FROM saved_contexts", [], |row| row.get(0))
        .unwrap()
}

fn observation_pass_count(init: &InitializeWorkspaceResult) -> i64 {
    let db = init.database.shared();
    let guard = db.lock().unwrap();
    guard
        .connection()
        .query_row("SELECT COUNT(*) FROM observation_passes", [], |row| {
            row.get(0)
        })
        .unwrap()
}

#[test]
fn the_capture_scope_is_readable_without_observing_anything() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let scope = CommandPipeline::new(context(
        &bus,
        &init,
        &AllowAllPermissionGate,
        CapabilitySet::local_user_standard(),
    ))
    .execute_query(GetSavedContextCaptureScope)
    .unwrap();

    assert_eq!(scope.id, SAVED_CONTEXT_SCOPE_ID);
    assert!(!scope.captured.is_empty());
    assert!(!scope.excluded.is_empty());
    assert_eq!(observation_pass_count(&init), 0);
}

#[test]
fn a_denied_actor_cannot_save_and_nothing_is_observed() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let workspace = seed_workspace(&bus, &init);

    let error = CommandPipeline::new(context(
        &bus,
        &init,
        &DenyAllPermissionGate,
        CapabilitySet::local_user_standard(),
    ))
    .execute_mutation(SaveWorkspaceContext::new(request(
        &workspace.id,
        SAVED_CONTEXT_SCOPE_ID,
    )))
    .unwrap_err();

    assert!(matches!(error, KernelError::PermissionDenied(_)));
    assert_eq!(observation_pass_count(&init), 0);
    assert_eq!(saved_context_count(&init), 0);
}

#[test]
fn saving_requires_the_capability_that_governs_reading_the_desktop() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let workspace = seed_workspace(&bus, &init);

    // Authorised to write workspace records, but not to read the desktop. A save
    // must not become a route to desktop state that `desktop.read` guards.
    let without_desktop_read = CapabilitySet::new()
        .with_capability(&Capability::workspace_read())
        .with_capability(&Capability::workspace_write());

    let error = CommandPipeline::new(context(
        &bus,
        &init,
        &AllowAllPermissionGate,
        without_desktop_read,
    ))
    .execute_mutation(SaveWorkspaceContext::new(request(
        &workspace.id,
        SAVED_CONTEXT_SCOPE_ID,
    )))
    .unwrap_err();

    assert!(matches!(error, KernelError::PermissionDenied(_)));
    assert_eq!(observation_pass_count(&init), 0);
    assert_eq!(saved_context_count(&init), 0);
}

#[test]
fn a_stale_confirmation_is_refused_and_recorded_as_a_failed_command() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let workspace = seed_workspace(&bus, &init);

    let error = CommandPipeline::new(context(
        &bus,
        &init,
        &AllowAllPermissionGate,
        CapabilitySet::local_user_standard(),
    ))
    .execute_mutation(SaveWorkspaceContext::new(request(
        &workspace.id,
        "saved-context-scope-v0",
    )))
    .unwrap_err();

    assert!(matches!(error, KernelError::SavedContextValidation { .. }));
    assert_eq!(observation_pass_count(&init), 0);
    assert_eq!(saved_context_count(&init), 0);

    let records = AuditService::list_recent(&init.database.shared(), 20).unwrap();
    assert!(records.iter().any(|record| {
        record.command_name.as_deref() == Some("SaveWorkspaceContext") && !record.success
    }));
}

#[test]
fn saving_into_a_workspace_that_does_not_exist_observes_nothing() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

    let error = CommandPipeline::new(context(
        &bus,
        &init,
        &AllowAllPermissionGate,
        CapabilitySet::local_user_standard(),
    ))
    .execute_mutation(SaveWorkspaceContext::new(request(
        &WorkspaceId::new("ws-does-not-exist").unwrap(),
        SAVED_CONTEXT_SCOPE_ID,
    )))
    .unwrap_err();

    assert!(matches!(error, KernelError::WorkspaceNotFound));
    assert_eq!(observation_pass_count(&init), 0);
    assert_eq!(saved_context_count(&init), 0);
}
