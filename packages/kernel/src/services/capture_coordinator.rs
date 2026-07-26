//! Observation capture orchestration boundary (Sprint 108).
//!
//! Sole entry for capture requests. Delegates implementation to
//! [`WorkspaceObservationService`]. Does not schedule, queue, or auto-trigger.
//!
//! ```text
//! Caller → CaptureCoordinator → WorkspaceObservationService → persisted snapshot
//! ```

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{ActorContext, CaptureRequest, IntentContext};
use workspace_windows_integration::DesktopCapturer;

use crate::error::{KernelError, Result};
use crate::services::{WorkspaceObservationCaptureResult, WorkspaceObservationService};

/// Internal capture lifecycle phases (not exposed over IPC / UI).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CaptureLifecycleState {
    Requested = 1,
    Started = 2,
    Completed = 3,
    Failed = 4,
}

impl CaptureLifecycleState {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Requested),
            2 => Some(Self::Started),
            3 => Some(Self::Completed),
            4 => Some(Self::Failed),
            _ => None,
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
        Self::run_capture(request, || WorkspaceObservationService::capture(db, actor, intent))
    }

    /// Request a capture with an injectable capturer (tests / fixtures).
    pub(crate) fn request_capture_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: CaptureRequest,
        capturer: &dyn DesktopCapturer,
    ) -> Result<CaptureCoordinatorResult> {
        Self::run_capture(request, || {
            WorkspaceObservationService::capture_with(db, actor, intent, capturer)
        })
    }

    fn run_capture(
        request: CaptureRequest,
        capture_fn: impl FnOnce() -> Result<WorkspaceObservationCaptureResult>,
    ) -> Result<CaptureCoordinatorResult> {
        let _request = request; // reserved for future audit / trigger attribution
        record_lifecycle(CaptureLifecycleState::Requested);
        let Some(_guard) = CaptureFlightGuard::try_acquire() else {
            return Ok(CaptureCoordinatorResult::RejectedConcurrent);
        };
        record_lifecycle(CaptureLifecycleState::Started);
        match capture_fn() {
            Ok(capture) => {
                record_lifecycle(CaptureLifecycleState::Completed);
                Ok(CaptureCoordinatorResult::Completed(capture))
            }
            Err(error) => {
                record_lifecycle(CaptureLifecycleState::Failed);
                Err(error)
            }
        }
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
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier, Mutex, OnceLock};
    use std::thread;

    use workspace_domain::{ActorContext, IntentContext};
    use workspace_windows_integration::{
        DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer, WindowsIntegrationError,
    };

    use crate::WorkspaceKernel;

    /// Serialize coordinator tests so the process-wide flight bit cannot race.
    fn coordinator_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
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

    #[test]
    fn coordinator_delegates_capture() {
        let _lock = coordinator_test_lock().lock().unwrap();
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
    fn concurrent_capture_rejected() {
        let _lock = coordinator_test_lock().lock().unwrap();
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
            CaptureRequest::system(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            second,
            CaptureCoordinatorResult::RejectedConcurrent
        ));

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
        let _lock = coordinator_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let err = CaptureCoordinator::request_capture_with(
            &kernel.shared_database(),
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

        let recovered = CaptureCoordinator::request_capture_with(
            &kernel.shared_database(),
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
        let _lock = coordinator_test_lock().lock().unwrap();
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
        let _lock = coordinator_test_lock().lock().unwrap();
        let err = CaptureCoordinator::into_capture_result(
            CaptureCoordinatorResult::RejectedConcurrent,
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::ObservationCaptureInProgress));
    }
}
