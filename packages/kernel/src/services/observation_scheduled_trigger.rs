//! Scheduled observation trigger (Sprint 113).
//!
//! First non-startup trigger caller boundary. Accepts an **explicit** schedule
//! tick/request only — does not create timers, workers, or background loops.
//!
//! ```text
//! Explicit schedule tick
//!         ↓
//! ObservationScheduledTrigger
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

/// Explicit-tick scheduled observation trigger (no timer owned here).
pub(crate) struct ObservationScheduledTrigger;

impl ObservationScheduledTrigger {
    pub const REASON: &'static str = "scheduled_refresh";
    pub const DEFAULT_CONTEXT: &'static str = "schedule:explicit_tick";

    /// Build a Scheduled [`ObservationTriggerRequest`] for one explicit tick.
    ///
    /// Freshness: [`ObservationFreshnessRequirement::NotStale`] — capture when
    /// unavailable or stale; skip when fresh/recent via refresh policy.
    pub(crate) fn trigger_request(context: Option<String>) -> ObservationTriggerRequest {
        ObservationTriggerRequest::scheduled(ObservationFreshnessRequirement::NotStale)
            .with_reason(Self::REASON)
            .with_context(context.unwrap_or_else(|| Self::DEFAULT_CONTEXT.into()))
    }

    /// Handle one explicit schedule tick through [`ObservationTriggerAuthority`].
    pub(crate) fn on_tick(
        kernel: &WorkspaceKernel,
        context: Option<String>,
    ) -> Result<ObservationTriggerDecision> {
        Self::on_tick_from_db(&kernel.shared_database(), context)
    }

    /// Schedule-runtime / low-level path: database handle only (no Win32 here).
    pub(crate) fn on_tick_from_db(
        db: &std::sync::Arc<std::sync::Mutex<workspace_database::Database>>,
        context: Option<String>,
    ) -> Result<ObservationTriggerDecision> {
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();
        ObservationTriggerAuthority::handle(db, &actor, &intent, Self::trigger_request(context))
    }

    /// Test/fixture path with an injectable capturer (still via trigger authority).
    pub(crate) fn on_tick_with(
        kernel: &WorkspaceKernel,
        context: Option<String>,
        capturer: &dyn DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        Self::on_tick_from_db_with(&kernel.shared_database(), context, capturer)
    }

    pub(crate) fn on_tick_from_db_with(
        db: &std::sync::Arc<std::sync::Mutex<workspace_database::Database>>,
        context: Option<String>,
        capturer: &dyn DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();
        ObservationTriggerAuthority::handle_with(
            db,
            &actor,
            &intent,
            Self::trigger_request(context),
            capturer,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{CaptureRequestSource, ObservationFreshness};
    use workspace_windows_integration::StubDesktopCapturer;

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{
        AuditService, ObservationTriggerAdmissionPolicy, ObservationTriggerDecision,
        WorkspaceObservationService,
    };

    fn begin_scheduled_test() -> std::sync::MutexGuard<'static, ()> {
        let lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();
        lock
    }

    #[test]
    fn scheduled_trigger_request_contract() {
        let request =
            ObservationScheduledTrigger::trigger_request(Some("schedule:test_tick".into()));
        assert_eq!(request.source, CaptureRequestSource::Scheduled);
        assert_eq!(
            request.reason.as_deref(),
            Some(ObservationScheduledTrigger::REASON)
        );
        assert_eq!(request.context.as_deref(), Some("schedule:test_tick"));
        assert_eq!(
            request.freshness_requirement,
            ObservationFreshnessRequirement::NotStale
        );
        let capture = request.to_capture_request();
        assert_eq!(capture.source, CaptureRequestSource::Scheduled);
        assert_eq!(
            capture.reason.as_deref(),
            Some(ObservationScheduledTrigger::REASON)
        );
    }

    #[test]
    fn scheduled_trigger_accepted() {
        let _lock = begin_scheduled_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let decision = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:accepted".into()),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));
    }

    #[test]
    fn scheduled_trigger_respects_admission_rate_limit() {
        let _lock = begin_scheduled_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let capturer = StubDesktopCapturer::fixture_dual_monitor();

        let first = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:first".into()),
            &capturer,
        )
        .unwrap();
        assert!(matches!(
            first,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));

        let second = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:second".into()),
            &capturer,
        )
        .unwrap();
        assert!(matches!(
            second,
            ObservationTriggerDecision::RateLimited { .. }
        ));
    }

    #[test]
    fn scheduled_provenance_preserved() {
        let _lock = begin_scheduled_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();

        ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:provenance".into()),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        let events = AuditService::list_recent(&db, 40).unwrap();
        let received = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.trigger.received")
            .expect("received");
        let received_meta = received.metadata.as_deref().expect("metadata");
        assert!(received_meta.contains("\"source\":\"scheduled\""));
        assert!(received_meta.contains(ObservationScheduledTrigger::REASON));
        assert!(received_meta.contains("schedule:provenance"));

        let capture_requested = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.capture.requested")
            .expect("capture requested");
        let capture_meta = capture_requested.metadata.as_deref().expect("metadata");
        assert!(capture_meta.contains("\"source\":\"scheduled\""));
        assert!(capture_meta.contains(ObservationScheduledTrigger::REASON));
        assert!(capture_meta.contains("schedule:provenance"));
    }

    #[test]
    fn fresh_observation_avoids_capture() {
        let _lock = begin_scheduled_test();
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

        let decision = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            None,
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
    fn stale_observation_causes_capture() {
        let _lock = begin_scheduled_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        WorkspaceObservationService::capture_with(
            &db,
            &local,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        let before_id = WorkspaceObservationService::get_status(&db, &local, &intent)
            .unwrap()
            .pass_id
            .unwrap();

        {
            let guard = db.lock().unwrap();
            guard
                .connection()
                .execute(
                    "UPDATE observation_passes SET captured_at = ?1",
                    ["2020-01-01T00:00:00Z"],
                )
                .unwrap();
        }

        let decision = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:stale".into()),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        match decision {
            ObservationTriggerDecision::AcceptedCapture(capture) => {
                assert_ne!(capture.snapshot_id, before_id);
            }
            other => panic!("expected AcceptedCapture, got {other:?}"),
        }
    }

    #[test]
    fn unavailable_observation_causes_capture() {
        let _lock = begin_scheduled_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let decision = ObservationScheduledTrigger::on_tick_with(
            &kernel,
            Some("schedule:unavailable".into()),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));
    }
}
