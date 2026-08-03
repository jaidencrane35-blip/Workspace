//! Observation capture orchestration boundary (Sprint 108–109).
//!
//! Sole entry for capture requests. Delegates implementation to
//! [`WorkspaceObservationService`]. Does not schedule, queue, or auto-trigger.
//!
//! ```text
//! Caller → CaptureCoordinator → WorkspaceObservationService → persisted snapshot
//! ```

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{ActorContext, CaptureRequest, IntentContext};
use workspace_windows_integration::DesktopCapturer;

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceObservationCaptureResult, WorkspaceObservationService,
    WorkspaceRuntimeStateService, WorkspaceSessionStore,
};

/// Internal capture lifecycle phases (not exposed over IPC / UI).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CaptureLifecycleState {
    Requested = 1,
    Started = 2,
    Completed = 3,
    Failed = 4,
    RejectedConcurrent = 5,
}

impl CaptureLifecycleState {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Requested),
            2 => Some(Self::Started),
            3 => Some(Self::Completed),
            4 => Some(Self::Failed),
            5 => Some(Self::RejectedConcurrent),
            _ => None,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::RejectedConcurrent => "rejected_concurrent",
        }
    }

    fn audit_event_type(self) -> &'static str {
        match self {
            Self::Requested => "workspace.observation.capture.requested",
            Self::Started => "workspace.observation.capture.started",
            Self::Completed => "workspace.observation.capture.completed",
            Self::Failed => "workspace.observation.capture.failed",
            Self::RejectedConcurrent => "workspace.observation.capture.rejected_concurrent",
        }
    }
}

/// Process-wide single-flight guard: at most one capture executes at a time.
static CAPTURE_IN_FLIGHT: AtomicBool = AtomicBool::new(false);
/// Last observed lifecycle phase (diagnostics / tests; not persisted).
static LAST_LIFECYCLE: AtomicU8 = AtomicU8::new(0);

fn record_lifecycle(state: CaptureLifecycleState) {
    LAST_LIFECYCLE.store(state as u8, Ordering::Release);
}

struct CaptureFlightGuard;

impl CaptureFlightGuard {
    fn try_acquire() -> Option<Self> {
        CAPTURE_IN_FLIGHT
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .ok()
            .map(|_| Self)
    }
}

impl Drop for CaptureFlightGuard {
    fn drop(&mut self) {
        CAPTURE_IN_FLIGHT.store(false, Ordering::Release);
    }
}

/// Explicit coordinator outcome for a capture request.
#[derive(Debug)]
pub(crate) enum CaptureCoordinatorResult {
    /// Capture ran to completion and produced a snapshot.
    Completed(WorkspaceObservationCaptureResult),
    /// Another capture was already running; this request was not queued.
    RejectedConcurrent,
}

/// Capture lifecycle authority — concurrency gate + delegation only.
pub(crate) struct CaptureCoordinator;

impl CaptureCoordinator {
    /// Request a capture via the platform capturer.
    pub(crate) fn request_capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: CaptureRequest,
    ) -> Result<CaptureCoordinatorResult> {
        Self::run_capture(db, actor, intent, request, || {
            WorkspaceObservationService::capture(db, actor, intent)
        })
    }

    /// Request a capture with an injectable capturer (tests / fixtures).
    pub(crate) fn request_capture_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: CaptureRequest,
        capturer: &dyn DesktopCapturer,
    ) -> Result<CaptureCoordinatorResult> {
        Self::run_capture(db, actor, intent, request, || {
            WorkspaceObservationService::capture_with(db, actor, intent, capturer)
        })
    }

    fn run_capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: CaptureRequest,
        capture_fn: impl FnOnce() -> Result<WorkspaceObservationCaptureResult>,
    ) -> Result<CaptureCoordinatorResult> {
        record_lifecycle(CaptureLifecycleState::Requested);
        WorkspaceRuntimeStateService::note_refresh_requested();
        Self::audit_lifecycle(
            db,
            actor,
            intent,
            &request,
            CaptureLifecycleState::Requested,
            true,
            None,
            None,
        )?;

        let Some(_guard) = CaptureFlightGuard::try_acquire() else {
            record_lifecycle(CaptureLifecycleState::RejectedConcurrent);
            WorkspaceRuntimeStateService::note_refresh_rejected_concurrent();
            Self::audit_lifecycle(
                db,
                actor,
                intent,
                &request,
                CaptureLifecycleState::RejectedConcurrent,
                false,
                None,
                Some("observation capture already in progress"),
            )?;
            return Ok(CaptureCoordinatorResult::RejectedConcurrent);
        };

        record_lifecycle(CaptureLifecycleState::Started);
        WorkspaceRuntimeStateService::note_refresh_in_progress();
        // Durable fence: crash mid-capture → NeedsRefresh on next startup.
        let _ = WorkspaceSessionStore::begin_operation(
            db,
            workspace_domain::PendingOperationKind::Observation,
            request.reason.clone(),
        );
        Self::audit_lifecycle(
            db,
            actor,
            intent,
            &request,
            CaptureLifecycleState::Started,
            true,
            None,
            None,
        )?;

        let outcome = match capture_fn() {
            Ok(capture) => {
                record_lifecycle(CaptureLifecycleState::Completed);
                WorkspaceRuntimeStateService::note_refresh_completed(Some(
                    capture.snapshot_id.as_str(),
                ));
                Self::audit_lifecycle(
                    db,
                    actor,
                    intent,
                    &request,
                    CaptureLifecycleState::Completed,
                    true,
                    Some(capture.snapshot_id.as_str()),
                    None,
                )?;
                Ok(CaptureCoordinatorResult::Completed(capture))
            }
            Err(error) => {
                record_lifecycle(CaptureLifecycleState::Failed);
                WorkspaceRuntimeStateService::note_refresh_failed();
                let _ = Self::audit_lifecycle(
                    db,
                    actor,
                    intent,
                    &request,
                    CaptureLifecycleState::Failed,
                    false,
                    None,
                    Some(&error.to_string()),
                );
                Err(error)
            }
        };
        // Drop the single-flight guard before checkpoint so persistence does not
        // observe capture-in-progress and skip the durable write.
        drop(_guard);
        if matches!(&outcome, Ok(CaptureCoordinatorResult::Completed(_))) {
            let _ = WorkspaceSessionStore::checkpoint_current(db, actor, intent, None);
        } else {
            // Intentional failure — clear fence (crash would have left it).
            let _ = WorkspaceSessionStore::clear_operation_fence(db);
        }
        outcome
    }

    fn audit_lifecycle(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &CaptureRequest,
        lifecycle: CaptureLifecycleState,
        success: bool,
        pass_id: Option<&str>,
        error_message: Option<&str>,
    ) -> Result<()> {
        let provenance = request.provenance();
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            lifecycle.audit_event_type(),
            success,
            json!({
                "lifecycle": lifecycle.as_str(),
                "source": provenance.source.as_str(),
                "reason": provenance.reason,
                "context": provenance.context,
                "pass_id": pass_id,
                "error": error_message,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    /// Whether a capture is currently executing under the coordinator.
    pub(crate) fn is_capture_in_progress() -> bool {
        CAPTURE_IN_FLIGHT.load(Ordering::Acquire)
    }

    /// Last lifecycle phase recorded by the coordinator (process-local).
    pub(crate) fn last_lifecycle() -> Option<CaptureLifecycleState> {
        CaptureLifecycleState::from_u8(LAST_LIFECYCLE.load(Ordering::Acquire))
    }

    /// Map coordinator outcome to the command/IPC capture result.
    pub(crate) fn into_capture_result(
        result: CaptureCoordinatorResult,
    ) -> Result<WorkspaceObservationCaptureResult> {
        match result {
            CaptureCoordinatorResult::Completed(capture) => Ok(capture),
            CaptureCoordinatorResult::RejectedConcurrent => {
                Err(KernelError::ObservationCaptureInProgress)
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn observation_flight_test_lock() -> &'static std::sync::Mutex<()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    use workspace_domain::{ActorContext, IntentContext};
    use workspace_windows_integration::{
        DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer, WindowsIntegrationError,
    };

    use crate::services::AuditService;
    use crate::WorkspaceKernel;

    /// Serialize coordinator tests so the process-wide flight bit cannot race.
    fn coordinator_test_lock() -> std::sync::MutexGuard<'static, ()> {
        observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    struct BlockingCapturer {
        entered: Arc<Barrier>,
        release: Arc<Barrier>,
        inner: StubDesktopCapturer,
    }

    impl DesktopCapturer for BlockingCapturer {
        fn capture_desktop(&self) -> workspace_windows_integration::Result<DesktopObservationCapture> {
            self.entered.wait();
            self.release.wait();
            self.inner.capture_desktop()
        }
    }

    struct FailingCapturer;

    impl DesktopCapturer for FailingCapturer {
        fn capture_desktop(&self) -> workspace_windows_integration::Result<DesktopObservationCapture> {
            Err(WindowsIntegrationError::EnumerationFailed(
                "coordinator forced failure".into(),
            ))
        }
    }

    fn audit_types(db: &Arc<Mutex<workspace_database::Database>>) -> Vec<String> {
        AuditService::list_recent(db, 50)
            .unwrap()
            .into_iter()
            .map(|event| event.event_type)
            .collect()
    }

    #[test]
    fn coordinator_delegates_capture() {
        let _lock = coordinator_test_lock();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let result = CaptureCoordinator::request_capture_with(
            &kernel.shared_database(),
            &local,
            &intent,
            CaptureRequest::manual(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        match result {
            CaptureCoordinatorResult::Completed(capture) => {
                assert!(capture.window_count > 0);
                assert_eq!(
                    CaptureCoordinator::last_lifecycle(),
                    Some(CaptureLifecycleState::Completed)
                );
                assert!(!CaptureCoordinator::is_capture_in_progress());
            }
            other => panic!("expected Completed, got {other:?}"),
        }
    }

    #[test]
    fn request_provenance_audited_on_lifecycle() {
        let _lock = coordinator_test_lock();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let request = CaptureRequest::manual()
            .with_reason("diagnostic")
            .with_context("ipc:capture_workspace_observation");

        CaptureCoordinator::request_capture_with(
            &db,
            &local,
            &intent,
            request,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let events = AuditService::list_recent(&db, 20).unwrap();
        let requested = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.capture.requested")
            .expect("requested audit");
        let metadata = requested.metadata.as_deref().expect("metadata");
        assert!(metadata.contains("diagnostic"));
        assert!(metadata.contains("ipc:capture_workspace_observation"));
        assert!(metadata.contains("\"source\":\"manual\""));

        let types = audit_types(&db);
        assert!(types.contains(&"workspace.observation.capture.requested".into()));
        assert!(types.contains(&"workspace.observation.capture.started".into()));
        assert!(types.contains(&"workspace.observation.capture.completed".into()));
    }

    #[test]
    fn concurrent_capture_rejected() {
        let _lock = coordinator_test_lock();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let capturer = BlockingCapturer {
            entered: Arc::clone(&entered),
            release: Arc::clone(&release),
            inner: StubDesktopCapturer::fixture_dual_monitor(),
        };

        let db_first = Arc::clone(&db);
        let local_first = local.clone();
        let intent_first = intent.clone();
        let first = thread::spawn(move || {
            CaptureCoordinator::request_capture_with(
                &db_first,
                &local_first,
                &intent_first,
                CaptureRequest::manual(),
                &capturer,
            )
        });

        entered.wait();
        assert!(CaptureCoordinator::is_capture_in_progress());
        assert_eq!(
            CaptureCoordinator::last_lifecycle(),
            Some(CaptureLifecycleState::Started)
        );

        let second = CaptureCoordinator::request_capture_with(
            &db,
            &local,
            &intent,
            CaptureRequest::system().with_reason("overlap"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            second,
            CaptureCoordinatorResult::RejectedConcurrent
        ));
        assert_eq!(
            CaptureCoordinator::last_lifecycle(),
            Some(CaptureLifecycleState::RejectedConcurrent)
        );

        let types = audit_types(&db);
        assert!(types.contains(&"workspace.observation.capture.rejected_concurrent".into()));

        release.wait();
        let first_result = first.join().expect("first capture thread").unwrap();
        assert!(matches!(
            first_result,
            CaptureCoordinatorResult::Completed(_)
        ));
        assert!(!CaptureCoordinator::is_capture_in_progress());
        assert_eq!(
            CaptureCoordinator::last_lifecycle(),
            Some(CaptureLifecycleState::Completed)
        );
    }

    #[test]
    fn failure_releases_coordinator_state() {
        let _lock = coordinator_test_lock();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let err = CaptureCoordinator::request_capture_with(
            &db,
            &local,
            &intent,
            CaptureRequest::manual(),
            &FailingCapturer,
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::WindowsIntegration { .. }));
        assert!(!CaptureCoordinator::is_capture_in_progress());
        assert_eq!(
            CaptureCoordinator::last_lifecycle(),
            Some(CaptureLifecycleState::Failed)
        );
        assert!(audit_types(&db).contains(&"workspace.observation.capture.failed".into()));

        let recovered = CaptureCoordinator::request_capture_with(
            &db,
            &local,
            &intent,
            CaptureRequest::manual(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            recovered,
            CaptureCoordinatorResult::Completed(_)
        ));
        assert!(!CaptureCoordinator::is_capture_in_progress());
    }

    #[test]
    fn success_releases_coordinator_state() {
        let _lock = coordinator_test_lock();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let first = CaptureCoordinator::request_capture_with(
            &kernel.shared_database(),
            &local,
            &intent,
            CaptureRequest::manual(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(first, CaptureCoordinatorResult::Completed(_)));
        assert!(!CaptureCoordinator::is_capture_in_progress());

        let second = CaptureCoordinator::request_capture_with(
            &kernel.shared_database(),
            &local,
            &intent,
            CaptureRequest::manual(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(second, CaptureCoordinatorResult::Completed(_)));
        assert!(!CaptureCoordinator::is_capture_in_progress());
    }

    #[test]
    fn into_capture_result_maps_busy_to_kernel_error() {
        let _lock = coordinator_test_lock();
        let err = CaptureCoordinator::into_capture_result(
            CaptureCoordinatorResult::RejectedConcurrent,
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::ObservationCaptureInProgress));
    }
}
