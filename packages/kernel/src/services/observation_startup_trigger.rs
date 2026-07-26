//! Startup observation trigger (Sprint 111).
//!
//! First real trigger source: fires once after the kernel reaches Ready during
//! production initialization. Does not schedule, hook, or run in the background.
//!
//! ```text
//! WorkspaceKernel::initialize (Ready)
//!         ↓
//! ObservationStartupTrigger
//!         ↓
//! ObservationTriggerAuthority
//! ```

use workspace_domain::{
    ActorContext, IntentContext, ObservationFreshnessRequirement, ObservationTriggerRequest,
};
use workspace_windows_integration::DesktopCapturer;

use crate::error::Result;
use crate::services::{ObservationTriggerAuthority, ObservationTriggerDecision};
use crate::WorkspaceKernel;

/// Canonical startup-only observation trigger.
pub(crate) struct ObservationStartupTrigger;

impl ObservationStartupTrigger {
    pub const REASON: &'static str = "startup_initialization";
    pub const CONTEXT: &'static str = "lifecycle:WorkspaceKernel::initialize";

    /// Build the startup [`ObservationTriggerRequest`].
    ///
    /// Freshness: [`ObservationFreshnessRequirement::AnyAvailable`] — capture only when
    /// no observation exists; do not recapture merely because a pass is stale/recent.
    pub(crate) fn trigger_request() -> ObservationTriggerRequest {
        ObservationTriggerRequest::system(ObservationFreshnessRequirement::AnyAvailable)
            .with_reason(Self::REASON)
            .with_context(Self::CONTEXT)
    }

    /// Evaluate the startup trigger through [`ObservationTriggerAuthority`].
    pub(crate) fn run(kernel: &WorkspaceKernel) -> Result<ObservationTriggerDecision> {
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();
        ObservationTriggerAuthority::handle(
            &kernel.shared_database(),
            &actor,
            &intent,
            Self::trigger_request(),
        )
    }

    /// Test/fixture path with an injectable capturer (still via trigger authority).
    pub(crate) fn run_with(
        kernel: &WorkspaceKernel,
        capturer: &dyn DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();
        ObservationTriggerAuthority::handle_with(
            &kernel.shared_database(),
            &actor,
            &intent,
            Self::trigger_request(),
            capturer,
        )
    }

    /// Fire once after Ready. Capture failures stay on the observation failure path;
    /// they must not fail kernel startup.
    pub(crate) fn fire(kernel: &WorkspaceKernel) {
        match Self::run(kernel) {
            Ok(decision) => {
                log::info!(
                    "startup observation trigger finished: {}",
                    decision.outcome().as_str()
                );
            }
            Err(error) => {
                log::warn!("startup observation trigger failed (kernel remains ready): {error}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{CaptureRequestSource, ObservationFreshness};
    use workspace_windows_integration::{
        DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer, WindowsIntegrationError,
    };

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{
        AuditService, ObservationTriggerDecision, WorkspaceObservationService,
    };

    struct FailingCapturer;

    impl DesktopCapturer for FailingCapturer {
        fn capture_desktop(&self) -> workspace_windows_integration::Result<DesktopObservationCapture> {
            Err(WindowsIntegrationError::EnumerationFailed(
                "startup forced failure".into(),
            ))
        }
    }

    #[test]
    fn startup_request_uses_system_provenance() {
        let request = ObservationStartupTrigger::trigger_request();
        assert_eq!(request.source, CaptureRequestSource::System);
        assert_eq!(
            request.reason.as_deref(),
            Some(ObservationStartupTrigger::REASON)
        );
        assert_eq!(
            request.context.as_deref(),
            Some(ObservationStartupTrigger::CONTEXT)
        );
        assert_eq!(
            request.freshness_requirement,
            ObservationFreshnessRequirement::AnyAvailable
        );
        let capture = request.to_capture_request();
        assert_eq!(capture.source, CaptureRequestSource::System);
        assert_eq!(
            capture.reason.as_deref(),
            Some(ObservationStartupTrigger::REASON)
        );
    }

    #[test]
    fn no_observation_startup_results_in_capture() {
        let _lock = observation_flight_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let decision = ObservationStartupTrigger::run_with(
            &kernel,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        match decision {
            ObservationTriggerDecision::AcceptedCapture(capture) => {
                assert!(capture.window_count > 0);
            }
            other => panic!("expected AcceptedCapture, got {other:?}"),
        }

        let status = WorkspaceObservationService::get_status(
            &kernel.shared_database(),
            &ActorContext::system(),
            &IntentContext::system_startup(),
        )
        .unwrap();
        assert!(status.has_observation);
    }

    #[test]
    fn fresh_observation_startup_ignored_by_policy() {
        let _lock = observation_flight_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        WorkspaceObservationService::capture_with(
            &kernel.shared_database(),
            &local,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let before = WorkspaceObservationService::get_status(
            &kernel.shared_database(),
            &local,
            &intent,
        )
        .unwrap();
        assert_eq!(before.freshness, ObservationFreshness::Fresh);
        let pass_id = before.pass_id.clone();

        let decision = ObservationStartupTrigger::run_with(
            &kernel,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            decision,
            ObservationTriggerDecision::IgnoredFresh
        ));

        let after = WorkspaceObservationService::get_status(
            &kernel.shared_database(),
            &local,
            &intent,
        )
        .unwrap();
        assert_eq!(after.pass_id, pass_id);
    }

    #[test]
    fn startup_provenance_preserved_through_capture_lifecycle() {
        let _lock = observation_flight_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();

        ObservationStartupTrigger::run_with(
            &kernel,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let events = AuditService::list_recent(&db, 40).unwrap();

        let received = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.trigger.received")
            .expect("trigger received");
        let received_meta = received.metadata.as_deref().expect("metadata");
        assert!(received_meta.contains("\"source\":\"system\""));
        assert!(received_meta.contains(ObservationStartupTrigger::REASON));
        assert!(received_meta.contains(ObservationStartupTrigger::CONTEXT));

        let capture_requested = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.capture.requested")
            .expect("capture requested");
        let capture_meta = capture_requested.metadata.as_deref().expect("metadata");
        assert!(capture_meta.contains("\"source\":\"system\""));
        assert!(capture_meta.contains(ObservationStartupTrigger::REASON));
        assert!(capture_meta.contains(ObservationStartupTrigger::CONTEXT));

        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.trigger.accepted"
        }));
        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.capture.completed"
        }));
    }

    #[test]
    fn startup_failure_visible_on_observation_failure_path() {
        let _lock = observation_flight_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let err = ObservationStartupTrigger::run_with(&kernel, &FailingCapturer).unwrap_err();
        assert!(err.to_string().contains("startup forced failure"));

        let status = WorkspaceObservationService::get_status(
            &kernel.shared_database(),
            &ActorContext::system(),
            &IntentContext::system_startup(),
        )
        .unwrap();
        assert!(!status.has_observation);
        let failure = status.last_failure.expect("capture failure recorded");
        assert!(failure.message.contains("startup forced failure"));
    }
}
