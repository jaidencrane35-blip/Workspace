//! Observation event gateway (Sprint 117).
//!
//! Normalization and governance boundary for future event sources.
//! Not an OS listener — does not call Win32, capture, schedule, or hook events.
//!
//! ```text
//! Future OS / adapter event
//!         ↓
//! ObservationEventGateway  (validate → normalize → map)
//!         ↓
//! ObservationTriggerAuthority
//!         ↓
//! Admission (Event still rejected)
//! ```

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    ActorContext, IntentContext, ObservationEvent, ObservationTriggerRequest,
};
use workspace_windows_integration::DesktopCapturer;

use crate::error::{KernelError, Result};
use crate::services::{ObservationTriggerAuthority, ObservationTriggerDecision};

/// Gateway: ObservationEvent → ObservationTriggerRequest → TriggerAuthority.
pub(crate) struct ObservationEventGateway;

impl ObservationEventGateway {
    pub const CONTEXT_PREFIX: &'static str = ObservationEvent::GATEWAY_CONTEXT_PREFIX;

    /// Validate + normalize an inbound event (no I/O).
    pub(crate) fn normalize(event: ObservationEvent) -> Result<ObservationEvent> {
        event
            .normalize()
            .map_err(|error| KernelError::Config(error.to_string()))
    }

    /// Map a normalized event to an Event-source trigger request.
    pub(crate) fn to_trigger_request(event: &ObservationEvent) -> ObservationTriggerRequest {
        event.to_trigger_request()
    }

    /// Accept an event, normalize it, and forward to ObservationTriggerAuthority.
    ///
    /// Admission still rejects `Event` today — no capture occurs.
    pub(crate) fn handle(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        event: ObservationEvent,
    ) -> Result<ObservationTriggerDecision> {
        let normalized = Self::normalize(event)?;
        let request = Self::to_trigger_request(&normalized);
        ObservationTriggerAuthority::handle(db, actor, intent, request)
    }

    /// Test path with injectable capturer (still blocked by Event admission).
    pub(crate) fn handle_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        event: ObservationEvent,
        capturer: &dyn DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        let normalized = Self::normalize(event)?;
        let request = Self::to_trigger_request(&normalized);
        ObservationTriggerAuthority::handle_with(db, actor, intent, request, capturer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{ObservationEventKind, ObservationFreshnessRequirement};
    use workspace_windows_integration::StubDesktopCapturer;

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{
        AuditService, ObservationTriggerAdmissionPolicy, ObservationTriggerDecision,
        WorkspaceObservationService,
    };
    use crate::WorkspaceKernel;

    fn begin_gateway_test() -> std::sync::MutexGuard<'static, ()> {
        let lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();
        lock
    }

    fn sample_event() -> ObservationEvent {
        ObservationEvent::new(
            "synthetic_probe",
            ObservationEventKind::FocusChanged,
            "2026-07-26T12:00:00Z",
        )
        .with_context("focus-probe")
        .with_correlation_id("corr-117")
        .with_metadata(r#"{"hwnd":"0x1"}"#)
    }

    #[test]
    fn valid_normalization() {
        let normalized = ObservationEventGateway::normalize(sample_event()).unwrap();
        assert_eq!(normalized.source, "synthetic_probe");
        assert_eq!(normalized.event_kind, ObservationEventKind::FocusChanged);
        assert_eq!(normalized.correlation_id.as_deref(), Some("corr-117"));
    }

    #[test]
    fn invalid_event_rejection() {
        let bad = ObservationEvent::new("", ObservationEventKind::Unknown, "2026-07-26T12:00:00Z");
        let err = ObservationEventGateway::normalize(bad).unwrap_err();
        assert!(matches!(err, KernelError::Config(_)));
        assert!(err.to_string().contains("source"));
    }

    #[test]
    fn mapping_correctness() {
        let normalized = ObservationEventGateway::normalize(sample_event()).unwrap();
        let request = ObservationEventGateway::to_trigger_request(&normalized);
        assert_eq!(
            request.source,
            workspace_domain::CaptureRequestSource::Event
        );
        assert_eq!(
            request.reason.as_deref(),
            Some(ObservationEvent::TRIGGER_REASON)
        );
        assert_eq!(
            request.freshness_requirement,
            ObservationFreshnessRequirement::NotStale
        );
        let ctx = request.context.as_deref().unwrap();
        assert!(ctx.contains(ObservationEventGateway::CONTEXT_PREFIX));
        assert!(ctx.contains("kind=focus_changed"));
        assert!(ctx.contains("source=synthetic_probe"));
        assert!(ctx.contains("correlation_id=corr-117"));
        assert!(ctx.contains("context=focus-probe"));
        assert!(ctx.contains(r#"metadata={"hwnd":"0x1"}"#));
    }

    #[test]
    fn admission_rejects_event_and_no_capture_occurs() {
        let _lock = begin_gateway_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();

        let decision = ObservationEventGateway::handle_with(
            &db,
            &actor,
            &intent,
            sample_event(),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            decision,
            ObservationTriggerDecision::RejectedSource { .. }
        ));

        let events = AuditService::list_recent(&db, 30).unwrap();
        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.trigger.received"
        }));
        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.trigger.rejected"
        }));
        assert!(!events.iter().any(|event| {
            event.event_type == "workspace.observation.capture.requested"
        }));

        let status = WorkspaceObservationService::get_status(
            &db,
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert!(!status.has_observation);
    }

    #[test]
    fn provenance_preserved_through_trigger_audits() {
        let _lock = begin_gateway_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();

        let _ = ObservationEventGateway::handle_with(
            &db,
            &ActorContext::system(),
            &IntentContext::system_startup(),
            sample_event(),
            &StubDesktopCapturer::empty(),
        )
        .unwrap();

        let events = AuditService::list_recent(&db, 20).unwrap();
        let received = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.trigger.received")
            .expect("received audit");
        let meta = received.metadata.as_deref().unwrap();
        assert!(meta.contains("\"source\":\"event\""));
        assert!(meta.contains(ObservationEvent::TRIGGER_REASON));
        assert!(meta.contains(ObservationEventGateway::CONTEXT_PREFIX));
        assert!(meta.contains("corr-117"));
        assert!(meta.contains("focus_changed"));
    }
}
