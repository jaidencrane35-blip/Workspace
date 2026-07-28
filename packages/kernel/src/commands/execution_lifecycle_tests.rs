use crate::error::KernelError;
use crate::services::{
    AuditService, ExecutionGuardService, ExecutionLifecycleService,
    ExecutionReconciliationService,
};
use crate::{CommandHandler, WorkspaceKernel};
use workspace_domain::{
    Actor, ActorContext, AuditEvent, ExecutionState, IntentContext,
};

fn seed_executable(kernel: &WorkspaceKernel) -> (String, String) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandHandler::create_workspace(
        kernel,
        actor.clone(),
        intent.clone(),
        "Durable execution".into(),
    )
    .unwrap();
    for index in 0..4 {
        CommandHandler::create_zone(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            format!("Zone {index}"),
            None,
        )
        .unwrap();
    }
    let suggestion_id = CommandHandler::get_suggestions(
        kernel,
        actor.clone(),
        intent.clone(),
        workspace.id.to_string(),
        Some(200),
    )
    .unwrap()
    .first()
    .expect("executable suggestion")
    .id
    .clone();
    CommandHandler::accept_suggestion(
        kernel,
        actor.clone(),
        intent.clone(),
        workspace.id.to_string(),
        suggestion_id.clone(),
    )
    .unwrap();
    CommandHandler::create_suggestion_intent_request(
        kernel,
        actor,
        intent,
        workspace.id.to_string(),
        suggestion_id.clone(),
    )
    .unwrap();
    (workspace.id.to_string(), suggestion_id)
}

fn execute(kernel: &WorkspaceKernel, workspace_id: &str, suggestion_id: &str) {
    CommandHandler::execute_intent_request(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.into(),
        suggestion_id.into(),
    )
    .unwrap();
}

fn zone_count(kernel: &WorkspaceKernel, workspace_id: &str) -> i64 {
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    guard
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM zones WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )
        .unwrap()
}

#[test]
fn completion_audit_failure_remains_terminal_and_reconciled() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, suggestion_id) = seed_executable(&kernel);
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        guard
            .connection()
            .execute_batch(
                "CREATE TRIGGER fail_execution_completion
                 BEFORE INSERT ON audit_events
                 WHEN NEW.event_type = 'command.executed'
                  AND NEW.command_name = 'ExecuteIntentRequest'
                 BEGIN
                   SELECT RAISE(FAIL, 'injected completion audit failure');
                 END;",
            )
            .unwrap();
    }

    execute(&kernel, &workspace_id, &suggestion_id);
    let count_after_first = zone_count(&kernel, &workspace_id);
    let error = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.clone(),
        suggestion_id.clone(),
    )
    .unwrap_err();
    assert!(matches!(error, KernelError::DuplicateExecution { .. }));
    assert_eq!(zone_count(&kernel, &workspace_id), count_after_first);

    let state = ExecutionReconciliationService::reconcile(
        &kernel.shared_database(),
        &format!("execution:{suggestion_id}"),
    )
    .unwrap();
    assert_eq!(state.current_state, ExecutionState::Completed);
    assert!(!state.dispatch_allowed);
}

#[test]
fn claim_persistence_failure_blocks_dispatch() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, suggestion_id) = seed_executable(&kernel);
    let count_before = zone_count(&kernel, &workspace_id);
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        guard
            .connection()
            .execute_batch(
                "CREATE TRIGGER fail_execution_claim
                 BEFORE INSERT ON execution_lifecycle
                 BEGIN
                   SELECT RAISE(FAIL, 'injected claim failure');
                 END;",
            )
            .unwrap();
    }

    let error = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.clone(),
        suggestion_id,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        KernelError::ExecutionLifecyclePersistence { stage: "claim", .. }
    ));
    assert_eq!(zone_count(&kernel, &workspace_id), count_before);
}

#[test]
fn completion_persistence_failure_leaves_non_dispatchable_claim() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, suggestion_id) = seed_executable(&kernel);
    let count_before = zone_count(&kernel, &workspace_id);
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        guard
            .connection()
            .execute_batch(
                "CREATE TRIGGER fail_execution_terminal
                 BEFORE UPDATE OF state ON execution_lifecycle
                 WHEN NEW.state = 'completed'
                 BEGIN
                   SELECT RAISE(FAIL, 'injected terminal failure');
                 END;",
            )
            .unwrap();
    }

    let error = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.clone(),
        suggestion_id.clone(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        KernelError::ExecutionLifecyclePersistence {
            stage: "completion",
            ..
        }
    ));
    assert_eq!(zone_count(&kernel, &workspace_id), count_before + 1);
    let retry = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.clone(),
        suggestion_id,
    );
    assert!(matches!(retry, Err(KernelError::ExecutionInProgress { .. })));
    assert_eq!(zone_count(&kernel, &workspace_id), count_before + 1);
}

#[test]
fn completion_survives_restart_and_blocks_retry() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.db");
    let (workspace_id, suggestion_id, count_after_first) = {
        let kernel = WorkspaceKernel::initialize(&path).unwrap();
        let (workspace_id, suggestion_id) = seed_executable(&kernel);
        execute(&kernel, &workspace_id, &suggestion_id);
        let count = zone_count(&kernel, &workspace_id);
        (workspace_id, suggestion_id, count)
    };

    let kernel = WorkspaceKernel::initialize(&path).unwrap();
    let error = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace_id.clone(),
        suggestion_id,
    )
    .unwrap_err();
    assert!(matches!(error, KernelError::DuplicateExecution { .. }));
    assert_eq!(zone_count(&kernel, &workspace_id), count_after_first);
}

#[test]
fn durable_completion_ignores_bounded_audit_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, suggestion_id) = seed_executable(&kernel);
    execute(&kernel, &workspace_id, &suggestion_id);

    for index in 0..600 {
        AuditService::append(
            &kernel.shared_database(),
            AuditEvent::from_actor(
                format!("unrelated.execution.noise.{index}"),
                &Actor::system(),
                true,
            ),
        )
        .unwrap();
    }

    let error =
        ExecutionGuardService::ensure_allowed(&kernel.shared_database(), &suggestion_id)
            .unwrap_err();
    assert!(matches!(error, KernelError::DuplicateExecution { .. }));
}

#[test]
fn unresolved_claim_survives_restart_and_fails_closed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.db");
    let suggestion_id = "interrupted-suggestion";
    let execution_request_id = format!("execution:{suggestion_id}");
    {
        let kernel = WorkspaceKernel::initialize(&path).unwrap();
        ExecutionLifecycleService::claim(
            &kernel.shared_database(),
            &execution_request_id,
            suggestion_id,
            Some("intent:test"),
        )
        .unwrap();
    }

    let kernel = WorkspaceKernel::initialize(&path).unwrap();
    let state = ExecutionReconciliationService::reconcile(
        &kernel.shared_database(),
        &execution_request_id,
    )
    .unwrap();
    assert_eq!(state.current_state, ExecutionState::InProgress);
    assert!(!state.dispatch_allowed);
    assert!(matches!(
        ExecutionGuardService::ensure_allowed(&kernel.shared_database(), suggestion_id),
        Err(KernelError::ExecutionInProgress { .. })
    ));
}
