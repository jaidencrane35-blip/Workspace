use crate::error::KernelError;
use crate::services::{
    AuditService, ExecutionGuardService, ExecutionLifecycleService, ExecutionOutcomeService,
    ExecutionReconciliationService,
};
use crate::{CommandHandler, WorkspaceKernel};
use workspace_database::ExecutionLifecycleRepository;
use workspace_domain::{
    Actor, ActorContext, AuditEvent, ExecutionLifecycleRecord, ExecutionOutcomeStatus,
    ExecutionState, IntentContext,
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

#[test]
fn outcome_projection_preserves_history_and_global_recency() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        let repository = ExecutionLifecycleRepository::new(&guard);
        let record = ExecutionLifecycleRecord {
            execution_request_id: "execution:historical".into(),
            suggestion_id: "historical".into(),
            intent_id: Some("intent:test".into()),
            state: ExecutionState::InProgress,
            claimed_at: "2026-01-01T00:00:00Z".into(),
            completed_at: None,
            updated_at: "2026-01-01T00:00:00Z".into(),
        };
        repository.insert_claim(&record).unwrap();
        repository
            .mark_completed(
                &record.execution_request_id,
                record.intent_id.as_deref(),
                "2026-01-01T00:01:00Z",
            )
            .unwrap();
    }

    let mut earlier_failure =
        AuditEvent::from_actor("command.failed", &Actor::local_user(), false)
            .with_command_name("ExecuteIntentRequest")
            .with_metadata(
                r#"{"execution_request":true,"execution_request_id":"execution:historical","suggestion_id":"historical","execution_status":"failed"}"#,
            );
    earlier_failure.timestamp = "2025-12-31T23:59:00Z".into();
    AuditService::append(&kernel.shared_database(), earlier_failure).unwrap();

    let mut newer_failure =
        AuditEvent::from_actor("command.failed", &Actor::local_user(), false)
            .with_command_name("ExecuteIntentRequest")
            .with_metadata(
                r#"{"execution_request":true,"execution_request_id":"execution:newer","suggestion_id":"newer","execution_status":"failed"}"#,
            );
    newer_failure.timestamp = "2026-02-01T00:00:00Z".into();
    AuditService::append(&kernel.shared_database(), newer_failure).unwrap();

    let outcomes = ExecutionOutcomeService::list_recent(&kernel.shared_database(), 10).unwrap();
    assert!(outcomes.iter().any(|outcome| {
        outcome.execution_request_id == "execution:historical"
            && outcome.status == ExecutionOutcomeStatus::Completed
    }));
    assert!(outcomes.iter().any(|outcome| {
        outcome.execution_request_id == "execution:historical"
            && outcome.status == ExecutionOutcomeStatus::Failed
    }));
    assert_eq!(
        ExecutionOutcomeService::list_recent(&kernel.shared_database(), 1)
            .unwrap()
            .first()
            .unwrap()
            .execution_request_id,
        "execution:newer"
    );
}

#[test]
fn cancellation_audit_failure_remains_durable_and_nonduplicable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let workspace = CommandHandler::create_workspace(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Durable cancellation".into(),
    )
    .unwrap();
    let suggestion_id = "missing-for-durable-cancel";
    let execution_request_id = format!("execution:{suggestion_id}");
    let _ = CommandHandler::execute_intent_request(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace.id.to_string(),
        suggestion_id.into(),
    );
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        guard
            .connection()
            .execute_batch(
                "CREATE TRIGGER fail_cancellation_completion
                 BEFORE INSERT ON audit_events
                 WHEN NEW.event_type = 'command.executed'
                  AND NEW.command_name = 'RequestExecutionCancellation'
                 BEGIN
                   SELECT RAISE(FAIL, 'injected cancellation audit failure');
                 END;",
            )
            .unwrap();
    }

    CommandHandler::request_execution_cancellation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace.id.to_string(),
        execution_request_id.clone(),
        "stop".into(),
    )
    .unwrap();
    let retry = CommandHandler::request_execution_cancellation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace.id.to_string(),
        execution_request_id.clone(),
        "stop again".into(),
    );
    assert!(matches!(
        retry,
        Err(KernelError::ExecutionCancellationValidation { .. })
    ));

    let state = ExecutionReconciliationService::reconcile(
        &kernel.shared_database(),
        &execution_request_id,
    )
    .unwrap();
    assert_eq!(state.current_state, ExecutionState::Cancelled);
    assert!(state.dispatch_allowed);
    assert!(!state.cancellation_allowed);
}
