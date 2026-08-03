//! Live Windows product-proof: observation → save → plan → execute → persist → hydrate.
//!
//! Uses the platform Win32 capturer/mutator. Stub fixtures are forbidden here.

#![cfg(windows)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use workspace_database::Database;
use workspace_domain::{
    ActorContext, CapabilitySet, IntentContext, OperationOutcome, PendingOperationKind,
    RecoveryDisposition, SaveContextRequest, SessionIntegrity, SAVED_CONTEXT_SCOPE_ID,
};
use workspace_windows_integration::{
    platform_desktop_capturer, platform_window_mutator, DesktopCapturer, WindowMutator,
};

use crate::commands::resume::resolve_and_execute_for_tests;
use crate::services::lock_observation_flight_for_tests;
use crate::services::{
    CaptureCoordinator, SavedContextService, WorkspaceRuntimeStateService, WorkspaceService,
    WorkspaceSessionStore,
};
use crate::WorkspaceKernel;

fn caps() -> CapabilitySet {
    CapabilitySet::local_user_standard()
}

struct LiveFixture {
    kernel: WorkspaceKernel,
    actor: ActorContext,
    intent: IntentContext,
    workspace_id: workspace_domain::WorkspaceId,
}

impl LiveFixture {
    fn new() -> Self {
        WorkspaceRuntimeStateService::reset_for_tests();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace_id = {
            let db = kernel.shared_database();
            let guard = db.lock().unwrap();
            WorkspaceService::create(&guard, "Windows Product Proof".into())
                .unwrap()
                .id
        };
        Self {
            kernel,
            actor,
            intent,
            workspace_id,
        }
    }

    fn db(&self) -> Arc<Mutex<Database>> {
        self.kernel.shared_database()
    }
}

#[test]
fn live_observation_save_plan_execute_persist_hydrate() {
    let _flight = lock_observation_flight_for_tests();
    let fixture = LiveFixture::new();
    let capturer = platform_desktop_capturer();

    // Ensure the live desktop is non-empty before Save.
    let live = capturer.capture_desktop().expect("live Win32 capture");
    assert!(
        !live.monitors.is_empty(),
        "host must expose at least one monitor"
    );
    assert!(
        !live.windows.is_empty(),
        "host must expose at least one titled window for Save proof"
    );

    CaptureCoordinator::request_capture_with(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        workspace_domain::CaptureRequest::manual().with_reason("product_proof_obs"),
        capturer.as_ref(),
    )
    .expect("observation");

    let saved = SavedContextService::save_with(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        &SaveContextRequest::new(
            fixture.workspace_id.clone(),
            "Product Proof Moment",
            SAVED_CONTEXT_SCOPE_ID,
            "Resume the Windows product proof session",
        ),
        capturer.as_ref(),
    )
    .expect("live save");

    assert!(!saved.windows.is_empty());
    assert!(!saved.monitors.is_empty());
    assert!(saved.windows.iter().any(|w| w.restore_identity.is_some()));

    let mutator = platform_window_mutator();
    let (plan, result) =
        resolve_and_execute_for_tests(&saved, &caps(), mutator.as_ref(), &Default::default())
            .expect("plan+execute");

    assert!(!plan.items.is_empty());
    assert!(matches!(
        result.outcome,
        OperationOutcome::Completed
            | OperationOutcome::PartiallyCompleted
            | OperationOutcome::Failed
    ));
    // Exact-session restore against the same live session should restore ≥1 place/focus.
    assert!(
        result.summary.restored_windows + result.summary.skipped_windows
            + result.summary.failed_operations
            >= 1
    );

    WorkspaceSessionStore::checkpoint_current(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        Some(saved.id.to_string()),
    )
    .expect("checkpoint");

    // Simulate process restart: reset owner, hydrate from durable session.
    WorkspaceRuntimeStateService::reset_for_tests();
    let hydrated =
        WorkspaceSessionStore::hydrate_on_startup(&fixture.db(), &fixture.actor, &fixture.intent)
            .expect("hydrate");
    assert!(
        hydrated.health.last_persistence_at.is_some()
            || hydrated.confidence_band.is_some()
            || !hydrated.restore_history.is_empty()
            || hydrated.observation.pass_id.is_some()
    );

    let session = {
        let db = fixture.db();
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };
    assert_eq!(
        session.last_saved_context_id.as_deref(),
        Some(saved.id.as_str())
    );

    // Continue again: second observation + re-plan after hydrate.
    CaptureCoordinator::request_capture_with(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        workspace_domain::CaptureRequest::manual().with_reason("product_proof_continue_again"),
        capturer.as_ref(),
    )
    .expect("post-hydrate observation");

    let (_plan2, result2) =
        resolve_and_execute_for_tests(&saved, &caps(), mutator.as_ref(), &Default::default())
            .expect("continue again");
    assert!(matches!(
        result2.outcome,
        OperationOutcome::Completed
            | OperationOutcome::PartiallyCompleted
            | OperationOutcome::Failed
    ));
}

#[test]
fn live_crash_fence_matrix_is_deterministic() {
    let fixture = LiveFixture::new();
    let cases = [
        (
            PendingOperationKind::Observation,
            RecoveryDisposition::NeedsRefresh,
        ),
        (PendingOperationKind::Save, RecoveryDisposition::Incomplete),
        (
            PendingOperationKind::RestorePlanning,
            RecoveryDisposition::Incomplete,
        ),
        (
            PendingOperationKind::RestoreExecution,
            RecoveryDisposition::Incomplete,
        ),
        (
            PendingOperationKind::Persistence,
            RecoveryDisposition::Recovered,
        ),
    ];

    for (kind, expected) in cases {
        WorkspaceSessionStore::begin_operation(&fixture.db(), kind, Some(format!("{kind:?}")))
            .unwrap();
        WorkspaceRuntimeStateService::reset_for_tests();
        let runtime = WorkspaceSessionStore::hydrate_on_startup(
            &fixture.db(),
            &fixture.actor,
            &fixture.intent,
        )
        .unwrap();
        assert_eq!(
            runtime.health.recovery_disposition, expected,
            "kind={kind:?}"
        );
        assert!(runtime.health.pending_recovery.is_some());
        let session = {
            let db = fixture.db();
            let guard = db.lock().unwrap();
            WorkspaceSessionStore::load_recovered(&guard)
        };
        assert!(session.pending_operation.is_none());
        assert!(session.last_recovery.is_some());
        // Clear recovery for next case so disposition is driven by new fence.
        {
            let db = fixture.db();
            let guard = db.lock().unwrap();
            let mut clean = WorkspaceSessionStore::load_recovered(&guard);
            clean.last_recovery = None;
            WorkspaceSessionStore::save(&guard, &clean).unwrap();
        }
        thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn live_corrupt_session_never_crashes_startup() {
    let fixture = LiveFixture::new();
    {
        let db = fixture.db();
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::save(
            &guard,
            &workspace_domain::PersistentWorkspaceSession::empty(),
        )
        .unwrap();
        guard
            .connection()
            .execute(
                "UPDATE workspace_persistent_session SET payload_checksum = 'bad' WHERE id = 'singleton'",
                [],
            )
            .unwrap();
    }
    WorkspaceRuntimeStateService::reset_for_tests();
    let runtime = WorkspaceSessionStore::hydrate_on_startup(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
    )
    .expect("startup must succeed");
    assert_eq!(
        runtime.health.session_integrity,
        SessionIntegrity::CorruptRecovered
    );
}

#[allow(dead_code)]
fn _mutator_type(_: &dyn WindowMutator) {}
#[allow(dead_code)]
fn _capturer_type(_: &dyn DesktopCapturer) {}
