//! Version 1.0 hardening — contract stability for release-critical surfaces.

use workspace_domain::{
    ItemDisposition, OperationOutcome, PendingOperationKind, PersistentWorkspaceSession,
    RecoveryDisposition, RestoreExecutionSummary, RuntimeHealth, SessionIntegrity,
    WorkspaceRuntimeState, ACTION_TYPE_WINDOW_FOCUS, ACTION_TYPE_WINDOW_PLACE,
    ACTION_TYPE_WINDOW_Z_ORDER, WORKSPACE_SESSION_SCHEMA_VERSION,
};
// RecoveryDisposition is used by interruption_disposition_matrix_is_stable.

use crate::services::WorkspaceRuntimeStateService;
use crate::WorkspaceKernel;
use workspace_domain::{ActorContext, IntentContext};

#[test]
fn session_schema_version_is_explicit_v2() {
    assert_eq!(WORKSPACE_SESSION_SCHEMA_VERSION, 2);
    let session = PersistentWorkspaceSession::empty();
    assert_eq!(session.schema_version, 2);
    assert!(session.migrate().is_ok());
}

#[test]
fn session_schema_rejects_future_without_implicit_upgrade() {
    let mut session = PersistentWorkspaceSession::empty();
    session.schema_version = WORKSPACE_SESSION_SCHEMA_VERSION + 1;
    assert!(session.migrate().is_err());
}

#[test]
fn restore_execution_summary_aggregates_deterministically() {
    use workspace_domain::{ActionItemOutcome, ItemDisposition as D};

    let item = |id: &str, action: &str, disposition: D, error_code: Option<&str>| ActionItemOutcome {
        item_id: id.into(),
        action_type: action.into(),
        target_summary: id.into(),
        disposition,
        what: action.into(),
        why: "hardening".into(),
        reason: None,
        error_code: error_code.map(str::to_string),
        user_action_available: "none".into(),
    };
    let items = vec![
        item("a", ACTION_TYPE_WINDOW_PLACE, D::Completed, None),
        item("b", ACTION_TYPE_WINDOW_FOCUS, D::Completed, None),
        item("c", ACTION_TYPE_WINDOW_Z_ORDER, D::SkippedUnsupported, None),
        item(
            "d",
            ACTION_TYPE_WINDOW_PLACE,
            D::SkippedUnresolvable,
            Some("ACTION_TARGET_NOT_FOUND"),
        ),
        item("e", ACTION_TYPE_WINDOW_PLACE, D::Failed, Some("REFUSED")),
    ];
    let summary = RestoreExecutionSummary::from_items(&items, 42);
    assert_eq!(summary.restored_windows, 2);
    assert_eq!(summary.skipped_windows, 2);
    assert_eq!(summary.missing_applications, 1);
    assert_eq!(summary.failed_operations, 1);
    assert_eq!(summary.duration_ms, 42);
}

#[test]
fn runtime_health_publishes_on_workspace_runtime_state() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let runtime = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &ActorContext::local_user(),
        &IntentContext::user_request(),
    )
    .unwrap();
    // Fresh in-memory DB has no session row → MissingRecovered (not degraded).
    assert_eq!(
        runtime.health.session_integrity,
        SessionIntegrity::MissingRecovered
    );
    assert!(!runtime.health.degraded);
    assert!(RuntimeHealth::healthy().pending_recovery.is_none());
}

#[test]
fn interruption_disposition_matrix_is_stable() {
    assert_eq!(
        PendingOperationKind::Observation.interruption_disposition(),
        RecoveryDisposition::NeedsRefresh
    );
    assert_eq!(
        PendingOperationKind::Save.interruption_disposition(),
        RecoveryDisposition::Incomplete
    );
    assert_eq!(
        PendingOperationKind::RestorePlanning.interruption_disposition(),
        RecoveryDisposition::Incomplete
    );
    assert_eq!(
        PendingOperationKind::RestoreExecution.interruption_disposition(),
        RecoveryDisposition::Incomplete
    );
    assert_eq!(
        PendingOperationKind::Persistence.interruption_disposition(),
        RecoveryDisposition::Recovered
    );
}

#[test]
fn declared_action_type_names_remain_stable() {
    assert_eq!(ACTION_TYPE_WINDOW_PLACE, "window.place");
    assert_eq!(ACTION_TYPE_WINDOW_FOCUS, "window.focus");
    assert_eq!(ACTION_TYPE_WINDOW_Z_ORDER, "window.z_order");
}

#[test]
fn restore_history_limit_is_bounded() {
    assert_eq!(WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT, 20);
}

#[test]
fn operation_outcome_maps_to_execution_phase_stably() {
    use workspace_domain::RestoreExecutionPhase;
    assert_eq!(
        RestoreExecutionPhase::from_operation_outcome(&OperationOutcome::Completed),
        RestoreExecutionPhase::Completed
    );
    assert_eq!(
        RestoreExecutionPhase::from_operation_outcome(&OperationOutcome::PartiallyCompleted),
        RestoreExecutionPhase::Partial
    );
    assert_eq!(
        RestoreExecutionPhase::from_operation_outcome(&OperationOutcome::Failed),
        RestoreExecutionPhase::Failed
    );
}

#[allow(dead_code)]
fn _item_disposition(_: ItemDisposition) {}
