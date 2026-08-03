//! Live WorkspaceRuntimeState owner — cache, freshness phases, execution history.

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use workspace_database::ObservationPassRepository;
use workspace_domain::{
    ActorContext, IntentContext, ObservationCachePhase, ObservedMonitor, ObservedWindow,
    OperationOutcome, RestoreExecutionPhase, RestoreExecutionSummary, WorkspaceObservationPass,
    WorkspaceObservationSnapshot,
};
use workspace_windows_integration::StubDesktopCapturer;

use crate::services::{
    lock_observation_flight_for_tests, CaptureCoordinator, CaptureCoordinatorResult,
    WorkspaceRuntimeStateService, WorkspaceStateEngine,
};
use crate::WorkspaceKernel;

fn persist(kernel: &WorkspaceKernel, snap: &WorkspaceObservationSnapshot) {
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    ObservationPassRepository::new(&guard)
        .insert_snapshot(snap)
        .unwrap();
}

fn snapshot(pass_id: &str, captured_at: &str) -> WorkspaceObservationSnapshot {
    WorkspaceObservationSnapshot {
        pass: WorkspaceObservationPass {
            id: pass_id.into(),
            captured_at: captured_at.into(),
            schema_version: 1,
            source: "test_inject".into(),
            foreground_hwnd: Some("0x1".into()),
            window_count: 1,
            monitor_count: 1,
            duration_ms: Some(1),
            metadata_json: "{}".into(),
            authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
        },
        monitors: vec![ObservedMonitor {
            id: format!("{pass_id}-mon"),
            pass_id: pass_id.into(),
            monitor_index: 0,
            name: "Primary".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            work_x: 0,
            work_y: 0,
            work_w: 1920,
            work_h: 1040,
            is_primary: true,
            dpi_scale: None,
            authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
        }],
        windows: vec![ObservedWindow {
            id: format!("{pass_id}-win"),
            pass_id: pass_id.into(),
            hwnd: "0x1".into(),
            stable_window_id: Some("stable-a".into()),
            title: "Alpha".into(),
            process_id: 42,
            process_name: Some("alpha.exe".into()),
            x: 10,
            y: 10,
            width: 100,
            height: 100,
            monitor_id: Some(format!("{pass_id}-mon")),
            visible: true,
            minimized: false,
            focused: true,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        }],
        identities: Vec::new(),
        authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
    }
}

#[test]
fn observation_refresh_phases_and_cache() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    WorkspaceRuntimeStateService::note_refresh_requested();
    assert_eq!(
        WorkspaceRuntimeStateService::cache_phase_for_tests(),
        ObservationCachePhase::RefreshRequested
    );

    let completed = CaptureCoordinator::request_capture_with(
        &db,
        &actor,
        &intent,
        workspace_domain::CaptureRequest::manual().with_reason("runtime_state_test"),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();
    assert!(matches!(completed, CaptureCoordinatorResult::Completed(_)));
    assert_eq!(
        WorkspaceRuntimeStateService::cache_phase_for_tests(),
        ObservationCachePhase::RefreshCompleted
    );

    let runtime = WorkspaceRuntimeStateService::current(&db, &actor, &intent).unwrap();
    assert!(runtime.desktop.metadata.window_count > 0);
    assert_eq!(runtime.active_monitor_index, Some(0));
    assert!(runtime.capture_timestamp.is_some());
    assert_eq!(
        WorkspaceRuntimeStateService::cached_pass_id_for_tests(),
        runtime.observation.pass_id
    );

    // Second read hits cache (same pass).
    let again = WorkspaceRuntimeStateService::current(&db, &actor, &intent).unwrap();
    assert_eq!(again.generation, runtime.generation);
    assert_eq!(
        again.desktop.metadata.state_id,
        runtime.desktop.metadata.state_id
    );
}

#[test]
fn stale_observation_still_served_until_refresh() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    persist(
        &kernel,
        &snapshot("pass-old", "2020-01-01T00:00:00Z"),
    );

    let runtime = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap();
    assert_eq!(
        runtime.observation.pass_id.as_deref(),
        Some("pass-old")
    );
    assert_eq!(
        runtime.observation.freshness,
        workspace_domain::ObservationFreshness::Stale
    );
    assert_eq!(runtime.cache_phase, ObservationCachePhase::Idle);
}

#[test]
fn concurrent_observation_requests_reject_duplicate() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let db = kernel.shared_database();

    let barrier = Arc::new(Barrier::new(2));
    let holding = Arc::new(Mutex::new(true));

    struct HoldingCapturer {
        barrier: Arc<Barrier>,
        holding: Arc<Mutex<bool>>,
    }

    impl workspace_windows_integration::DesktopCapturer for HoldingCapturer {
        fn capture_desktop(
            &self,
        ) -> workspace_windows_integration::Result<
            workspace_windows_integration::DesktopObservationCapture,
        > {
            self.barrier.wait();
            while *self.holding.lock().unwrap() {
                thread::sleep(std::time::Duration::from_millis(5));
            }
            StubDesktopCapturer::fixture_dual_monitor().capture_desktop()
        }
    }

    let barrier_main = barrier.clone();
    let holding_main = holding.clone();
    let db_bg = db.clone();
    let actor_bg = actor.clone();
    let intent_bg = intent.clone();
    let bg = thread::spawn(move || {
        CaptureCoordinator::request_capture_with(
            &db_bg,
            &actor_bg,
            &intent_bg,
            workspace_domain::CaptureRequest::manual().with_reason("bg"),
            &HoldingCapturer {
                barrier: barrier_main,
                holding: holding_main,
            },
        )
    });

    barrier.wait();
    // First capture is in progress — second must be rejected without queuing.
    let rejected = CaptureCoordinator::request_capture_with(
        &db,
        &actor,
        &intent,
        workspace_domain::CaptureRequest::manual().with_reason("dup"),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();
    assert!(matches!(
        rejected,
        CaptureCoordinatorResult::RejectedConcurrent
    ));
    assert_eq!(
        WorkspaceRuntimeStateService::cache_phase_for_tests(),
        ObservationCachePhase::RefreshInProgress
    );

    *holding.lock().unwrap() = false;
    let first = bg.join().unwrap().unwrap();
    assert!(matches!(first, CaptureCoordinatorResult::Completed(_)));
    assert_eq!(
        WorkspaceRuntimeStateService::cache_phase_for_tests(),
        ObservationCachePhase::RefreshCompleted
    );
}

#[test]
fn execution_state_transitions_and_restore_history() {
    // Initialize first — hydrate resets the process-local owner.
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    WorkspaceRuntimeStateService::reset_for_tests();

    WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Planning);
    assert_eq!(
        WorkspaceRuntimeStateService::execution_phase_for_tests(),
        RestoreExecutionPhase::Planning
    );
    WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Validating);
    WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Executing);
    WorkspaceRuntimeStateService::note_execution_finished(
        "op-1",
        OperationOutcome::PartiallyCompleted,
        RestoreExecutionSummary {
            restored_windows: 1,
            skipped_windows: 1,
            missing_applications: 1,
            failed_operations: 0,
            duration_ms: 12,
        },
        "Resume saved context",
    );
    assert_eq!(
        WorkspaceRuntimeStateService::execution_phase_for_tests(),
        RestoreExecutionPhase::Partial
    );

    let runtime = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &ActorContext::local_user(),
        &IntentContext::user_request(),
    )
    .unwrap();
    assert_eq!(runtime.restore_history.len(), 1);
    assert_eq!(runtime.restore_history[0].operation_id, "op-1");
    assert_eq!(
        runtime.execution_phase,
        RestoreExecutionPhase::Partial
    );
    assert!(runtime.health.last_restore_at.is_some());
}

#[test]
fn invalidate_forces_projection_rebuild() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    persist(&kernel, &snapshot("pass-1", "2026-08-03T00:00:00Z"));

    let first = WorkspaceStateEngine::get_current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap();
    let gen_before = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap()
    .generation;

    WorkspaceRuntimeStateService::invalidate();
    let after = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap();
    assert!(after.generation > gen_before);
    assert_eq!(
        after.desktop.metadata.observation_pass_id,
        first.metadata.observation_pass_id
    );
}

#[test]
fn runtime_state_consistent_with_desktop_engine() {
    let _flight = lock_observation_flight_for_tests();
    WorkspaceRuntimeStateService::reset_for_tests();

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    persist(&kernel, &snapshot("pass-c", "2026-08-03T01:00:00Z"));

    let desktop = WorkspaceStateEngine::get_current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap();
    let runtime = WorkspaceRuntimeStateService::current(
        &kernel.shared_database(),
        &actor,
        &intent,
    )
    .unwrap();
    assert_eq!(desktop, runtime.desktop);
    assert_eq!(runtime.active_monitor_index, Some(0));
    assert!(runtime
        .desktop
        .active_applications
        .iter()
        .any(|app| app.process_name.as_deref() == Some("alpha.exe")));
}
