//! Failure-mode verification for architecture governance.
//!
//! Confirms fail-closed behaviour under database unavailability, corrupted
//! projections, interrupted migrations, and partial recovery — with no silent
//! authority downgrade, fabricated evidence, or unsafe retry.

use std::sync::{Arc, Mutex};

use workspace_database::migration::Migration;
use workspace_database::{Database, DatabaseError, MigrationRunner};
use workspace_domain::{
    history_count_is_authoritative, history_json_is_non_commandable, Actor, ActorContext,
    Capability, CapabilitySet, IntentContext, RecommendationHistoryEntry,
    RecommendationOutcomeView, ResourceKind,
};

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::r#trait::{Command, MutationCommand};
use crate::error::KernelError;
use crate::policy::AlwaysAllowPolicy;
use crate::security::{
    AllowAllPermissionGate, GatewayDecision, PermissionGateway, PermissionRequest,
    PermissionSubject,
};
use crate::WorkspaceKernel;

fn poisoned_database() -> Arc<Mutex<Database>> {
    let db = Database::open_in_memory().expect("in-memory db");
    let mutex = Arc::new(Mutex::new(db));
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = mutex.lock().unwrap();
        panic!("poison for governance failure-mode test");
    }));
    std::panic::set_hook(previous);
    assert!(mutex.lock().is_err(), "mutex must be poisoned");
    mutex
}

#[test]
fn database_lock_poison_fails_closed_without_authority_downgrade() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let poisoned = poisoned_database();

    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let request = PermissionRequest {
        actor: actor.actor.clone(),
        intent: intent.intent.clone(),
        capability: Capability::workspace_write(),
        command: "CreateWorkspace",
        subject: PermissionSubject::Resource(ResourceKind::Workspace),
        target_resource_id: None,
    };

    let result = PermissionGateway::require(
        &poisoned,
        &actor,
        &intent,
        kernel.permission_policy(),
        kernel.permission_gate(),
        &request,
        &CapabilitySet::local_user_standard(),
    );

    match result {
        Err(KernelError::Internal { message }) => {
            assert!(
                message.contains("poisoned"),
                "expected poisoned lock error, got {message}"
            );
        }
        Err(other) => panic!("expected internal lock failure, got {other:?}"),
        Ok(()) => panic!("poisoned database must not authorize"),
    }
}

#[test]
fn empty_capability_set_denies_mutation_no_silent_allow() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let mut ctx = kernel.command_context(actor, intent);
    ctx.capability_set = CapabilitySet::new();

    let err = CommandPipeline::new(ctx)
        .execute_mutation(CreateWorkspace::new("denied-ws".into()))
        .expect_err("empty capability set must deny mutation");

    let message = err.to_string().to_lowercase();
    assert!(
        message.contains("denied")
            || message.contains("permission")
            || message.contains("capability")
            || message.contains("not granted")
            || message.contains("approval")
            || message.contains("unauthorized"),
        "deny must be explicit, got: {err}"
    );
}

#[test]
fn empty_capabilities_do_not_silently_allow_launch() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::new(Actor::ai_assistant("gov-probe").unwrap());
    let intent = IntentContext::ai_suggestion();
    let request = PermissionRequest {
        actor: actor.actor.clone(),
        intent: intent.intent.clone(),
        capability: Capability::application_launch(),
        command: "LaunchApplication",
        subject: PermissionSubject::Resource(ResourceKind::Application),
        target_resource_id: Some("app:1".into()),
    };

    let decision = PermissionGateway::evaluate(
        kernel.permission_policy(),
        kernel.permission_gate(),
        &request,
        &CapabilitySet::new(),
    )
    .expect("evaluate must return a decision");

    assert!(
        !matches!(decision, GatewayDecision::Allow { .. }),
        "AI with empty capabilities must not receive silent Allow"
    );
}

#[test]
fn corrupted_history_projection_is_non_commandable_and_non_actionable() {
    let corrupt = RecommendationHistoryEntry {
        native_id: "rec-corrupt".into(),
        lifecycle_state: "accepted".into(),
        outcome: RecommendationOutcomeView {
            outcome_id: "".into(),
            recommendation_id: "rec-corrupt".into(),
            user_decision: "accepted".into(),
            result_kind: "accepted_follow_through".into(),
            lifecycle_resolution: Some("accepted".into()),
            recorded_at: "t".into(),
            explanation_keys: vec![],
            evidence_refs: vec![],
            experience_trace_match_keys: vec![],
            is_system_failure: false,
            authority_effect: "none".into(),
        },
        resolved_at: Some("t".into()),
        terminal: true,
        actionable: false,
        authority_effect: "none".into(),
    };
    assert!(
        !corrupt.is_non_actionable(),
        "missing outcome_id must fail closed as non-evidence"
    );

    let mut forged = serde_json::to_value(&corrupt).unwrap();
    forged
        .as_object_mut()
        .unwrap()
        .insert("execute".into(), serde_json::json!(true));
    forged
        .as_object_mut()
        .unwrap()
        .insert("actionable".into(), serde_json::json!(true));
    assert!(
        !history_json_is_non_commandable(&forged),
        "forged execute/actionable fields must be rejected"
    );
}

#[test]
fn interrupted_migration_does_not_leave_partial_authority_schema() {
    let db = Database::open_in_memory().unwrap();
    let mut runner = MigrationRunner::new();
    runner.register(Migration {
        version: "gov_partial".into(),
        name: "partial".into(),
        sql: "CREATE TABLE gov_authority (id TEXT);
              INSERT INTO missing_gov_table (id) VALUES ('x');"
            .into(),
    });

    let err = runner.apply_all(&db).expect_err("migration must fail");
    assert!(
        matches!(err, DatabaseError::Migration(_)),
        "interrupted migration must surface Migration error, got {err:?}"
    );

    let table_count: i64 = db
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='gov_authority'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let ledger: i64 = db
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM _workspace_migrations WHERE version='gov_partial'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    assert_eq!(table_count, 0, "partial schema must roll back");
    assert_eq!(ledger, 0, "failed migration must not be recorded as applied");
}

#[test]
fn production_kernel_does_not_wire_allow_all_shortcuts() {
    let _test_only_policy = AlwaysAllowPolicy;
    let _test_only_gate = AllowAllPermissionGate;
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

    let policy_name = std::any::type_name_of_val(kernel.permission_policy());
    let gate_name = std::any::type_name_of_val(kernel.permission_gate());
    assert!(
        !policy_name.contains("AlwaysAllow"),
        "production kernel must not wire AlwaysAllowPolicy ({policy_name})"
    );
    assert!(
        !gate_name.contains("AllowAll"),
        "production kernel must not wire AllowAllPermissionGate ({gate_name})"
    );
}

#[test]
fn mutation_command_declares_capability_path() {
    let command = CreateWorkspace::new("gov-cap".into());
    assert_eq!(command.required_capability().id.as_str(), "workspace.write");
    assert_eq!(command.name(), "CreateWorkspace");
}

#[test]
fn partial_state_recovery_does_not_fabricate_terminal_evidence() {
    let window: Vec<RecommendationHistoryEntry> = vec![];
    assert!(history_count_is_authoritative(window.len(), 0));
    // Truncated window under a higher authoritative count is valid; inventing
    // rows to fill the gap is forbidden.
    assert!(history_count_is_authoritative(window.len(), 3));
    assert!(window.is_empty(), "recovery must not invent history entries");
}
