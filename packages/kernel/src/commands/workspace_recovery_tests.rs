//! Runtime resilience & recovery — fences, health, long-run, consistency.

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use workspace_database::PersistentWorkspaceSessionRepository;
use workspace_domain::{
    ActorContext, IntentContext, ObservationCachePhase, PendingOperationFence,
    PendingOperationKind, PersistentWorkspaceSession, RecoveryDisposition, RestoreExecutionPhase,
    RuntimeHealth, SessionIntegrity, WorkspaceRuntimeState, WORKSPACE_SESSION_SCHEMA_VERSION,
};
use workspace_windows_integration::StubDesktopCapturer;

use crate::services::observation_flight_test_lock;
use crate::services::{
    assert_runtime_session_consistent, CaptureCoordinator, WorkspaceRuntimeStateService,
    WorkspaceSessionStore,
};
use crate::WorkspaceKernel;

#[test]
fn interrupted_observation_recovers_as_needs_refresh() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    WorkspaceSessionStore::begin_operation(
        &db,
        PendingOperationKind::Observation,
        Some("crash-mid-obs".into()),
    )
    .unwrap();

    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime = WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    assert_eq!(
        runtime.health.recovery_disposition,
        RecoveryDisposition::NeedsRefresh
    );
    assert!(runtime.health.degraded);
    assert!(runtime.health.pending_recovery.is_some());

    let session = {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert!(session.pending_operation.is_none(), "fence acknowledged");
    assert!(session.last_recovery.is_some(), "never silently discarded");
}

#[test]
fn interrupted_restore_recovers_as_incomplete() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    WorkspaceSessionStore::begin_operation(
        &db,
        PendingOperationKind::RestoreExecution,
        Some("digest".into()),
    )
    .unwrap();

    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime = WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    assert_eq!(
        runtime.health.recovery_disposition,
        RecoveryDisposition::Incomplete
    );
    assert_eq!(
        runtime
            .health
            .pending_recovery
            .as_ref()
            .map(|r| r.kind),
        Some(PendingOperationKind::RestoreExecution)
    );
}

#[test]
fn interrupted_persistence_recovers_prior_state() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    let mut prior = PersistentWorkspaceSession::empty();
    prior.active_workspace_id = Some("ws-keep".into());
    prior.last_observation_pass_id = Some("pass-keep".into());
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &prior).unwrap();
    }

    // Simulate crash during persistence: fence set, prior row still valid.
    WorkspaceSessionStore::begin_operation(&db, PendingOperationKind::Persistence, None)
        .unwrap();

    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime = WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    assert_eq!(
        runtime.health.recovery_disposition,
        RecoveryDisposition::Recovered
    );
    let session = {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert_eq!(session.active_workspace_id.as_deref(), Some("ws-keep"));
    assert_eq!(
        session.last_observation_pass_id.as_deref(),
        Some("pass-keep")
    );
}

#[test]
fn interrupted_startup_hydration_never_panics() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    // Corrupt session + fence material is impossible; corrupt checksum recovers empty.
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &PersistentWorkspaceSession::empty()).unwrap();
        guard
            .connection()
            .execute(
                "UPDATE workspace_persistent_session SET payload_checksum = 'bad', pending_operation_json = '{not-json'",
                [],
            )
            .unwrap();
    }

    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime = WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
    assert_eq!(
        runtime.health.session_integrity,
        SessionIntegrity::CorruptRecovered
    );
    assert_eq!(runtime.cache_phase, ObservationCachePhase::Idle);
    assert_eq!(runtime.execution_phase, RestoreExecutionPhase::Idle);
}

#[test]
fn checksum_corruption_recovery_preserves_process() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &PersistentWorkspaceSession::empty()).unwrap();
        guard
            .connection()
            .execute(
                "UPDATE workspace_persistent_session SET payload_checksum = 'corrupt' WHERE id = 'singleton'",
                [],
            )
            .unwrap();
        let outcome = WorkspaceSessionStore::load_outcome(&guard);
        assert_eq!(outcome.integrity, SessionIntegrity::CorruptRecovered);
        assert!(outcome.session.last_observation_pass_id.is_none());
    }
}

#[test]
fn migration_failure_recovers_empty() {
    let mut session = PersistentWorkspaceSession::empty();
    session.schema_version = 99;
    assert!(session.migrate().is_err());

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    {
        let guard = db.lock().unwrap();
        // Force-write incompatible schema bypassing save() refusal via SQL.
        guard
            .connection()
            .execute(
                "INSERT INTO workspace_persistent_session (
                    id, schema_version, updated_at, restore_history_json, payload_checksum
                 ) VALUES ('singleton', 99, 't', '[]', '')",
                [],
            )
            .unwrap();
        let outcome = WorkspaceSessionStore::load_outcome(&guard);
        assert_eq!(outcome.integrity, SessionIntegrity::IncompatibleRecovered);
        assert_eq!(outcome.session.schema_version, WORKSPACE_SESSION_SCHEMA_VERSION);
    }
}

#[test]
fn concurrent_observation_and_persistence() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();
    let barrier = Arc::new(Barrier::new(2));

    let db_obs = db.clone();
    let barrier_obs = barrier.clone();
    let t_obs = thread::spawn(move || {
        barrier_obs.wait();
        CaptureCoordinator::request_capture_with(
            &db_obs,
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            workspace_domain::CaptureRequest::manual().with_reason("concurrent"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
    });

    let db_persist = db.clone();
    let barrier_p = barrier;
    let t_persist = thread::spawn(move || {
        barrier_p.wait();
        for i in 0..5 {
            let mut session = PersistentWorkspaceSession::empty();
            session.active_workspace_id = Some(format!("ws-{i}"));
            let guard = db_persist.lock().unwrap();
            let _ = WorkspaceSessionStore::save(&guard, &session);
        }
    });

    let obs_result = t_obs.join().unwrap();
    t_persist.join().unwrap();
    assert!(obs_result.is_ok());

    // Runtime remains coherent after concurrent writers.
    let runtime =
        WorkspaceRuntimeStateService::current(&db, &actor, &intent).unwrap();
    assert!(matches!(
        runtime.cache_phase,
        ObservationCachePhase::RefreshCompleted | ObservationCachePhase::Idle
    ));
}

#[test]
fn long_run_observation_loop_no_stale_accumulation() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    for i in 0..12 {
        CaptureCoordinator::request_capture_with(
            &db,
            &actor,
            &intent,
            workspace_domain::CaptureRequest::manual()
                .with_reason(format!("long-run-{i}")),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
    }

    let runtime = WorkspaceRuntimeStateService::current(&db, &actor, &intent).unwrap();
    assert!(runtime.restore_history.len() <= WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT);
    assert!(runtime.health.last_successful_observation_at.is_some());
    assert!(runtime.health.last_persistence_at.is_some());
    // Desktop cache holds one projection — not a growing bag.
    assert!(WorkspaceRuntimeStateService::cached_pass_id_for_tests().is_some());
}

#[test]
fn restart_loop_hydration_stable() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    let mut session = PersistentWorkspaceSession::empty();
    session.last_confidence_band = Some("high".into());
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &session).unwrap();
    }

    for _ in 0..8 {
        WorkspaceRuntimeStateService::reset_for_tests();
        let runtime =
            WorkspaceSessionStore::hydrate_on_startup(&db, &actor, &intent).unwrap();
        assert_eq!(runtime.confidence_band.as_deref(), Some("high"));
        assert_eq!(runtime.cache_phase, ObservationCachePhase::Idle);
        assert!(!runtime.health.degraded || runtime.health.pending_recovery.is_some());
    }
}

#[test]
fn consistency_assertion_passes_after_checkpoint() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    CaptureCoordinator::request_capture_with(
        &db,
        &actor,
        &intent,
        workspace_domain::CaptureRequest::manual().with_reason("consistency"),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();
    WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Idle);

    let runtime = WorkspaceRuntimeStateService::current(&db, &actor, &intent).unwrap();
    let session = {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert!(assert_runtime_session_consistent(&runtime, &session).is_ok());
}

#[test]
fn failed_mutation_retains_previous_valid_session() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();

    let mut prior = PersistentWorkspaceSession::empty();
    prior.active_workspace_id = Some("retain-me".into());
    {
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(&guard, &prior).unwrap();
    }

    // In-flight execute must not overwrite prior.
    WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Executing);
    WorkspaceSessionStore::checkpoint_current(
        &db,
        &ActorContext::local_user(),
        &IntentContext::user_request(),
        None,
    )
    .unwrap();

    let loaded = {
        let guard = db.lock().unwrap();
        PersistentWorkspaceSessionRepository::new(&guard)
            .load()
            .unwrap()
            .unwrap()
    };
    assert_eq!(loaded.active_workspace_id.as_deref(), Some("retain-me"));
}

#[test]
fn runtime_health_published_via_runtime_state() {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let health = RuntimeHealth::healthy();
    WorkspaceRuntimeStateService::hydrate_from_session(
        &PersistentWorkspaceSession::empty(),
        health,
    );
    let runtime = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &ActorContext::local_user(),
        &IntentContext::user_request(),
    )
    .unwrap();
    assert_eq!(runtime.health.session_integrity, SessionIntegrity::Ok);
    assert_eq!(
        runtime.health.recovery_disposition,
        RecoveryDisposition::None
    );
}

#[allow(dead_code)]
fn _mutex_type(_: &Mutex<()>) {}
