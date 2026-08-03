//! Persistent WorkspaceSessionStore — save/load/migrate/recovery/hydration.

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use workspace_database::PersistentWorkspaceSessionRepository;
use workspace_domain::{
    ActorContext, IntentContext, OperationOutcome, PersistentWorkspaceSession,
    RestoreExecutionSummary, RestoreHistoryEntry, WORKSPACE_SESSION_SCHEMA_VERSION,
};
use workspace_windows_integration::StubDesktopCapturer;

use crate::services::lock_observation_flight_for_tests;
use crate::services::{
    CaptureCoordinator, WorkspaceRuntimeStateService, WorkspaceSessionStore,
};
use crate::WorkspaceKernel;

#[test]
fn save_load_checkpoint_round_trip() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    CaptureCoordinator::request_capture_with(
        &db,
        &actor,
        &intent,
        workspace_domain::CaptureRequest::manual().with_reason("session_store"),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();

    // Stabilize process-local phases before durable write (shared across tests).
    WorkspaceRuntimeStateService::note_execution_phase(
        workspace_domain::RestoreExecutionPhase::Idle,
    );
    WorkspaceSessionStore::checkpoint_current(&db, &actor, &intent, Some("sc-1".into()))
        .expect("checkpoint");

    let loaded = {
        let guard = db.lock().unwrap();
        PersistentWorkspaceSessionRepository::new(&guard)
            .load()
            .expect("load")
            .expect("persisted")
    };
    assert_eq!(loaded.schema_version, WORKSPACE_SESSION_SCHEMA_VERSION);
    assert!(loaded.last_observation_pass_id.is_some());
    assert_eq!(loaded.last_saved_context_id.as_deref(), Some("sc-1"));
}

#[test]
fn migration_v0_to_v2_on_load_path() {
    let mut session = PersistentWorkspaceSession::empty();
    session.schema_version = 0;
    session.active_workspace_id = Some("ws-mig".into());
    let migrated = session.migrate().unwrap();
    assert_eq!(migrated.schema_version, WORKSPACE_SESSION_SCHEMA_VERSION);
    assert_eq!(migrated.active_workspace_id.as_deref(), Some("ws-mig"));
    assert!(migrated.pending_operation.is_none());
}

#[test]
fn corruption_recovery_returns_empty() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    {
        let guard = db.lock().unwrap();
        let repo = PersistentWorkspaceSessionRepository::new(&guard);
        repo.save(&PersistentWorkspaceSession::empty()).unwrap();
        guard
            .connection()
            .execute(
                "UPDATE workspace_persistent_session SET payload_checksum = 'corrupt' WHERE id = 'singleton'",
                [],
            )
            .unwrap();
        let recovered = WorkspaceSessionStore::load_recovered(&guard);
        assert!(recovered.last_observation_pass_id.is_none());
        assert!(recovered.restore_history.is_empty());
    }
}

#[test]
fn partial_write_guard_skips_in_flight_execution() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    WorkspaceRuntimeStateService::note_execution_phase(
        workspace_domain::RestoreExecutionPhase::Executing,
    );
    WorkspaceSessionStore::checkpoint_current(&db, &actor, &intent, None).unwrap();
    let loaded = {
        let guard = db.lock().unwrap();
        PersistentWorkspaceSessionRepository::new(&guard).load().unwrap()
    };
    assert!(loaded.is_none(), "in-flight execute must not checkpoint");
}

#[test]
fn concurrent_persistence_last_writer_wins() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    let barrier = Arc::new(Barrier::new(2));

    let make_session = |id: &str| {
        let mut session = PersistentWorkspaceSession::empty();
        session.active_workspace_id = Some(id.into());
        session
    };

    let db_a = db.clone();
    let barrier_a = barrier.clone();
    let t1 = thread::spawn(move || {
        barrier_a.wait();
        let guard = db_a.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &make_session("ws-a")).unwrap();
    });
    let db_b = db.clone();
    let barrier_b = barrier;
    let t2 = thread::spawn(move || {
        barrier_b.wait();
        let guard = db_b.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &make_session("ws-b")).unwrap();
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let loaded = {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert!(matches!(
        loaded.active_workspace_id.as_deref(),
        Some("ws-a") | Some("ws-b")
    ));
}

#[test]
fn startup_hydration_restores_history() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    let mut session = PersistentWorkspaceSession::empty();
    session.last_confidence_band = Some("steady".into());
    session.restore_history.push(RestoreHistoryEntry {
        recorded_at: "2026-08-03T00:00:00Z".into(),
        operation_id: "op-hydrate".into(),
        outcome: OperationOutcome::PartiallyCompleted,
        summary: RestoreExecutionSummary {
            restored_windows: 1,
            skipped_windows: 1,
            missing_applications: 1,
            failed_operations: 0,
            duration_ms: 9,
        },
        purpose: "Resume".into(),
    });
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &session).unwrap();
    }

    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime =
        WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    assert_eq!(runtime.confidence_band.as_deref(), Some("steady"));
    assert_eq!(runtime.restore_history.len(), 1);
    assert_eq!(runtime.restore_history[0].operation_id, "op-hydrate");
    assert_eq!(
        runtime.cache_phase,
        workspace_domain::ObservationCachePhase::Idle
    );
}

#[test]
fn observation_refresh_after_hydration_updates_session() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    CaptureCoordinator::request_capture_with(
        &db,
        &actor,
        &intent,
        workspace_domain::CaptureRequest::manual().with_reason("post_hydrate"),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();

    let loaded = {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert!(loaded.last_observation_pass_id.is_some());
    assert!(loaded.last_capture_timestamp.is_some());
}

#[allow(dead_code)]
fn _mutex_type(_: &Mutex<()>) {}
