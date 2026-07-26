//! Observation trigger authority (Sprint 110).
//!
//! Sole evaluation boundary for observation trigger requests.
//! Does not schedule, hook, or invent triggers — only decides and optionally
//! delegates capture through [`CaptureCoordinator`].
//!
//! ```text
//! ObservationTriggerRequest
//!         ↓
//! ObservationTriggerAuthority
//!         ↓
//! ObservationRefreshPolicyService
//!         ↓
//! CaptureCoordinator   (only when refresh required / unavailable)
//!         ↓
//! WorkspaceObservationService
//! ```

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, IntentContext, ObservationFreshnessRequirement, ObservationRefreshContext,
    ObservationRefreshDecision, ObservationTriggerOutcome, ObservationTriggerRequest,
};
use workspace_windows_integration::DesktopCapturer;

use crate::error::Result;
use crate::services::{
    AuditService, CaptureCoordinator, CaptureCoordinatorResult, ObservationRefreshPolicyService,
    WorkspaceObservationCaptureResult,
};

/// Trigger authority decision with optional capture payload.
#[derive(Debug)]
#[allow(dead_code)] // `Unavailable` reserved for soft-decline / failed unavailable paths
pub(crate) enum ObservationTriggerDecision {
    AcceptedCapture(WorkspaceObservationCaptureResult),
    IgnoredFresh,
    BlockedCaptureInProgress,
    /// Observation unavailable and capture did not complete.
    Unavailable,
}

impl ObservationTriggerDecision {
    pub(crate) fn outcome(&self) -> ObservationTriggerOutcome {
        match self {
            Self::AcceptedCapture(_) => ObservationTriggerOutcome::AcceptedCapture,
            Self::IgnoredFresh => ObservationTriggerOutcome::IgnoredFresh,
            Self::BlockedCaptureInProgress => ObservationTriggerOutcome::BlockedCaptureInProgress,
            Self::Unavailable => ObservationTriggerOutcome::Unavailable,
        }
    }
}

/// Trigger evaluation authority — never calls Win32 or observation capture directly.
pub(crate) struct ObservationTriggerAuthority;

impl ObservationTriggerAuthority {
    /// Evaluate a trigger and optionally capture via [`CaptureCoordinator`].
    pub(crate) fn handle(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: ObservationTriggerRequest,
    ) -> Result<ObservationTriggerDecision> {
        Self::handle_inner(db, actor, intent, request, None)
    }

    /// Test/fixture path with an injectable capturer (still via CaptureCoordinator).
    pub(crate) fn handle_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: ObservationTriggerRequest,
        capturer: &dyn DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        Self::handle_inner(db, actor, intent, request, Some(capturer))
    }

    fn handle_inner(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: ObservationTriggerRequest,
        capturer: Option<&dyn DesktopCapturer>,
    ) -> Result<ObservationTriggerDecision> {
        Self::audit_received(db, actor, intent, &request)?;

        let refresh = ObservationRefreshPolicyService::evaluate(
            db,
            actor,
            intent,
            request.freshness_requirement.clone(),
            ObservationRefreshContext::new()
                .with_consumer(request.source.as_str())
                .with_purpose(request.reason.clone().unwrap_or_else(|| "trigger".into())),
        )?;

        let decision = match refresh {
            ObservationRefreshDecision::FreshEnough => ObservationTriggerDecision::IgnoredFresh,
            ObservationRefreshDecision::RefreshBlocked { .. } => {
                ObservationTriggerDecision::BlockedCaptureInProgress
            }
            ObservationRefreshDecision::RefreshRequired
            | ObservationRefreshDecision::ObservationUnavailable => {
                Self::delegate_capture(db, actor, intent, &request, capturer)?
            }
        };

        match &decision {
            ObservationTriggerDecision::AcceptedCapture(_) => {
                Self::audit_accepted(db, actor, intent, &request, &refresh)?;
            }
            ObservationTriggerDecision::IgnoredFresh
            | ObservationTriggerDecision::BlockedCaptureInProgress
            | ObservationTriggerDecision::Unavailable => {
                Self::audit_ignored(db, actor, intent, &request, &refresh, decision.outcome())?;
            }
        }

        Ok(decision)
    }

    fn delegate_capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &ObservationTriggerRequest,
        capturer: Option<&dyn DesktopCapturer>,
    ) -> Result<ObservationTriggerDecision> {
        let capture_request = request.to_capture_request();
        let coordinator_result = match capturer {
            Some(capturer) => CaptureCoordinator::request_capture_with(
                db,
                actor,
                intent,
                capture_request,
                capturer,
            )?,
            None => CaptureCoordinator::request_capture(db, actor, intent, capture_request)?,
        };

        Ok(match coordinator_result {
            CaptureCoordinatorResult::Completed(capture) => {
                ObservationTriggerDecision::AcceptedCapture(capture)
            }
            CaptureCoordinatorResult::RejectedConcurrent => {
                ObservationTriggerDecision::BlockedCaptureInProgress
            }
        })
    }

    fn audit_received(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &ObservationTriggerRequest,
    ) -> Result<()> {
        Self::audit(
            db,
            actor,
            intent,
            "workspace.observation.trigger.received",
            true,
            request,
            None,
            None,
        )
    }

    fn audit_accepted(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &ObservationTriggerRequest,
        refresh: &ObservationRefreshDecision,
    ) -> Result<()> {
        Self::audit(
            db,
            actor,
            intent,
            "workspace.observation.trigger.accepted",
            true,
            request,
            Some(refresh),
            Some(ObservationTriggerOutcome::AcceptedCapture),
        )
    }

    fn audit_ignored(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &ObservationTriggerRequest,
        refresh: &ObservationRefreshDecision,
        outcome: ObservationTriggerOutcome,
    ) -> Result<()> {
        Self::audit(
            db,
            actor,
            intent,
            "workspace.observation.trigger.ignored",
            true,
            request,
            Some(refresh),
            Some(outcome),
        )
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        event_type: &str,
        success: bool,
        request: &ObservationTriggerRequest,
        refresh: Option<&ObservationRefreshDecision>,
        outcome: Option<ObservationTriggerOutcome>,
    ) -> Result<()> {
        let provenance = request.provenance();
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            event_type,
            success,
            json!({
                "source": provenance.source.as_str(),
                "reason": provenance.reason,
                "context": provenance.context,
                "requirement": request.freshness_requirement.as_str(),
                "requirement_max_age_seconds": match &request.freshness_requirement {
                    ObservationFreshnessRequirement::MaxAgeSeconds { max_age_seconds } => {
                        Some(*max_age_seconds)
                    }
                    _ => None,
                },
                "refresh_decision": refresh.map(|value| value.as_str()),
                "outcome": outcome.map(|value| value.as_str()),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    use workspace_domain::{CaptureRequest, CaptureRequestSource};
    use workspace_windows_integration::{
        DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer,
    };

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{AuditService, CaptureCoordinator, WorkspaceObservationService};
    use crate::WorkspaceKernel;

    fn trigger_test_lock() -> &'static Mutex<()> {
        observation_flight_test_lock()
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

    #[test]
    fn fresh_observation_ignores_trigger() {
        let _lock = trigger_test_lock().lock().unwrap();
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

        let decision = ObservationTriggerAuthority::handle_with(
            &db,
            &local,
            &intent,
            ObservationTriggerRequest::system(ObservationFreshnessRequirement::Fresh)
                .with_reason("poll")
                .with_context("test:fresh"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            ObservationTriggerDecision::IgnoredFresh
        ));

        let types: Vec<_> = AuditService::list_recent(&db, 30)
            .unwrap()
            .into_iter()
            .map(|event| event.event_type)
            .collect();
        assert!(types.contains(&"workspace.observation.trigger.received".into()));
        assert!(types.contains(&"workspace.observation.trigger.ignored".into()));
        assert!(!types.iter().any(|t| t == "workspace.observation.capture.started"));
    }

    #[test]
    fn unavailable_observation_accepts_trigger() {
        let _lock = trigger_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let decision = ObservationTriggerAuthority::handle_with(
            &db,
            &local,
            &intent,
            ObservationTriggerRequest::manual(ObservationFreshnessRequirement::AnyAvailable)
                .with_reason("cold_start")
                .with_context("test:unavailable"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        match decision {
            ObservationTriggerDecision::AcceptedCapture(capture) => {
                assert!(capture.window_count > 0);
            }
            other => panic!("expected AcceptedCapture, got {other:?}"),
        }

        let types: Vec<_> = AuditService::list_recent(&db, 40)
            .unwrap()
            .into_iter()
            .map(|event| event.event_type)
            .collect();
        assert!(types.contains(&"workspace.observation.trigger.received".into()));
        assert!(types.contains(&"workspace.observation.trigger.accepted".into()));
        assert!(types.contains(&"workspace.observation.capture.completed".into()));
    }

    #[test]
    fn concurrent_capture_blocks_trigger() {
        let _lock = trigger_test_lock().lock().unwrap();
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

        let decision = ObservationTriggerAuthority::handle_with(
            &db,
            &local,
            &intent,
            ObservationTriggerRequest::event(ObservationFreshnessRequirement::Fresh)
                .with_reason("overlap"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            ObservationTriggerDecision::BlockedCaptureInProgress
        ));

        release.wait();
        first.join().expect("first capture").unwrap();
    }

    #[test]
    fn trigger_creates_coordinator_request_with_provenance() {
        let _lock = trigger_test_lock().lock().unwrap();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let request = ObservationTriggerRequest::plugin(ObservationFreshnessRequirement::NotStale)
            .with_reason("plugin_refresh")
            .with_context("plugin:demo");

        assert_eq!(request.source, CaptureRequestSource::Plugin);
        let capture_req = request.to_capture_request();
        assert_eq!(capture_req.source, CaptureRequestSource::Plugin);
        assert_eq!(capture_req.reason.as_deref(), Some("plugin_refresh"));
        assert_eq!(capture_req.context.as_deref(), Some("plugin:demo"));

        let decision = ObservationTriggerAuthority::handle_with(
            &db,
            &local,
            &intent,
            request,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));

        let events = AuditService::list_recent(&db, 40).unwrap();
        let received = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.trigger.received")
            .expect("received");
        let metadata = received.metadata.as_deref().expect("metadata");
        assert!(metadata.contains("plugin_refresh"));
        assert!(metadata.contains("plugin:demo"));
        assert!(metadata.contains("\"source\":\"plugin\""));

        let capture_requested = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.capture.requested")
            .expect("capture requested");
        let capture_meta = capture_requested.metadata.as_deref().expect("metadata");
        assert!(capture_meta.contains("plugin_refresh"));
        assert!(capture_meta.contains("\"source\":\"plugin\""));
    }

    #[test]
    fn stale_observation_refresh_required_accepts_trigger() {
        let _lock = trigger_test_lock().lock().unwrap();
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

        // Force stale captured_at so Fresh requirement fails.
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

        let decision = ObservationTriggerAuthority::handle_with(
            &db,
            &local,
            &intent,
            ObservationTriggerRequest::scheduled(ObservationFreshnessRequirement::Fresh)
                .with_reason("stale_repair"),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));
    }
}
