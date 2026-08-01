//! Observation trigger admission policy (Sprint 112).
//!
//! Decides whether a trigger may proceed before refresh evaluation.
//! Does not capture, call Win32, or bypass [`ObservationTriggerAuthority`].

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use workspace_domain::{
    CaptureRequestSource, ObservationTriggerAdmissionDecision, ObservationTriggerRequest,
    OBSERVATION_TRIGGER_ADMIT_WINDOW_SECS, OBSERVATION_TRIGGER_MAX_ADMITS_PER_WINDOW,
    OBSERVATION_TRIGGER_MIN_ADMIT_INTERVAL_SECS,
};

/// Lightweight process-local rate state (no durable history / scheduler).
#[derive(Debug)]
struct TriggerAdmissionRateState {
    last_admitted_at: Option<Instant>,
    last_capture_at: Option<Instant>,
    window_started_at: Instant,
    admissions_in_window: u32,
}

impl TriggerAdmissionRateState {
    fn new(now: Instant) -> Self {
        Self {
            last_admitted_at: None,
            last_capture_at: None,
            window_started_at: now,
            admissions_in_window: 0,
        }
    }
}

/// Product Proof: ambient capture authorization, closed for the process lifetime.
///
/// Startup and schedule callers stay implemented and wired, but observe nothing
/// until capture is explicitly authorized. Nothing in the running product opens
/// this gate, so a fresh process never inspects the desktop before the user
/// initiates a Save.
static AMBIENT_CAPTURE_AUTHORIZED: AtomicBool = AtomicBool::new(false);

fn rate_state() -> &'static Mutex<TriggerAdmissionRateState> {
    static STATE: OnceLock<Mutex<TriggerAdmissionRateState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(TriggerAdmissionRateState::new(Instant::now())))
}

/// Admission gate for observation triggers.
pub(crate) struct ObservationTriggerAdmissionPolicy;

impl ObservationTriggerAdmissionPolicy {
    fn min_interval() -> Duration {
        Duration::from_secs(OBSERVATION_TRIGGER_MIN_ADMIT_INTERVAL_SECS)
    }

    fn window() -> Duration {
        Duration::from_secs(OBSERVATION_TRIGGER_ADMIT_WINDOW_SECS)
    }

    /// Sources admitted for execution today (contracts for others remain defined).
    ///
    /// Product Proof permits no ambient capture: the desktop may only be observed
    /// after the user explicitly initiates a Save, which reaches this gate as a
    /// `Manual` trigger. `System` (startup) and `Scheduled` (background) callers
    /// remain implemented and wired but are inert while ambient capture is closed.
    pub(crate) fn source_is_admitted(source: CaptureRequestSource) -> bool {
        match source {
            CaptureRequestSource::Manual => true,
            CaptureRequestSource::System | CaptureRequestSource::Scheduled => {
                Self::ambient_capture_authorized()
            }
            CaptureRequestSource::Event | CaptureRequestSource::Plugin => false,
        }
    }

    /// Whether non-user-initiated capture is currently authorized.
    pub(crate) fn ambient_capture_authorized() -> bool {
        AMBIENT_CAPTURE_AUTHORIZED.load(Ordering::Acquire)
    }

    /// Evaluate whether `request` may proceed to refresh/capture.
    pub(crate) fn evaluate(
        request: &ObservationTriggerRequest,
    ) -> ObservationTriggerAdmissionDecision {
        Self::evaluate_at(request, Instant::now())
    }

    pub(crate) fn evaluate_at(
        request: &ObservationTriggerRequest,
        now: Instant,
    ) -> ObservationTriggerAdmissionDecision {
        if !Self::source_is_admitted(request.source) {
            return ObservationTriggerAdmissionDecision::RejectedSource {
                explanation: format!(
                    "trigger source '{}' is not admitted; observation requires an explicit user-initiated capture",
                    request.source.as_str()
                ),
            };
        }

        let Ok(mut state) = rate_state().lock() else {
            return ObservationTriggerAdmissionDecision::RateLimited {
                explanation: "trigger admission rate state unavailable".into(),
            };
        };

        if now.duration_since(state.window_started_at) >= Self::window() {
            state.window_started_at = now;
            state.admissions_in_window = 0;
        }

        if let Some(last) = state.last_admitted_at {
            let elapsed = now.duration_since(last);
            if elapsed < Self::min_interval() {
                return ObservationTriggerAdmissionDecision::RateLimited {
                    explanation: format!(
                        "minimum admit interval {}s not elapsed ({}ms since last admission)",
                        OBSERVATION_TRIGGER_MIN_ADMIT_INTERVAL_SECS,
                        elapsed.as_millis()
                    ),
                };
            }
        }

        if state.admissions_in_window >= OBSERVATION_TRIGGER_MAX_ADMITS_PER_WINDOW {
            return ObservationTriggerAdmissionDecision::RateLimited {
                explanation: format!(
                    "admission window limit of {} reached",
                    OBSERVATION_TRIGGER_MAX_ADMITS_PER_WINDOW
                ),
            };
        }

        ObservationTriggerAdmissionDecision::Admitted
    }

    /// Record a successful admission (call only when proceeding past the gate).
    pub(crate) fn record_admitted() {
        Self::record_admitted_at(Instant::now());
    }

    pub(crate) fn record_admitted_at(now: Instant) {
        if let Ok(mut state) = rate_state().lock() {
            if now.duration_since(state.window_started_at) >= Self::window() {
                state.window_started_at = now;
                state.admissions_in_window = 0;
            }
            state.last_admitted_at = Some(now);
            state.admissions_in_window = state.admissions_in_window.saturating_add(1);
        }
    }

    /// Record a completed capture accepted via trigger authority.
    pub(crate) fn record_capture() {
        Self::record_capture_at(Instant::now());
    }

    pub(crate) fn record_capture_at(now: Instant) {
        if let Ok(mut state) = rate_state().lock() {
            state.last_capture_at = Some(now);
        }
    }

    #[cfg(test)]
    pub(crate) fn reset_for_tests() {
        AMBIENT_CAPTURE_AUTHORIZED.store(false, Ordering::Release);
        let mut state = rate_state()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *state = TriggerAdmissionRateState::new(Instant::now());
    }

    /// Exercise the retained startup/schedule implementation, which is inert in
    /// the running product. Callers must hold `observation_flight_test_lock`.
    #[cfg(test)]
    pub(crate) fn authorize_ambient_capture_for_tests() {
        AMBIENT_CAPTURE_AUTHORIZED.store(true, Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn last_capture_at_for_tests() -> Option<Instant> {
        rate_state()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .last_capture_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::ObservationFreshnessRequirement;

    use crate::services::capture_coordinator::observation_flight_test_lock;

    #[test]
    fn admits_only_explicit_manual_capture_by_default() {
        let _lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();

        assert!(!ObservationTriggerAdmissionPolicy::ambient_capture_authorized());
        assert!(ObservationTriggerAdmissionPolicy::source_is_admitted(
            CaptureRequestSource::Manual
        ));
        for ambient in [CaptureRequestSource::System, CaptureRequestSource::Scheduled] {
            assert!(
                !ObservationTriggerAdmissionPolicy::source_is_admitted(ambient),
                "{} must not observe before an explicit user-initiated capture",
                ambient.as_str()
            );
        }
        assert!(!ObservationTriggerAdmissionPolicy::source_is_admitted(
            CaptureRequestSource::Event
        ));
        assert!(!ObservationTriggerAdmissionPolicy::source_is_admitted(
            CaptureRequestSource::Plugin
        ));

        for request in [
            ObservationTriggerRequest::plugin(ObservationFreshnessRequirement::AnyAvailable),
            ObservationTriggerRequest::system(ObservationFreshnessRequirement::AnyAvailable),
            ObservationTriggerRequest::scheduled(ObservationFreshnessRequirement::NotStale)
                .with_reason("scheduled_refresh"),
        ] {
            assert!(matches!(
                ObservationTriggerAdmissionPolicy::evaluate(&request),
                ObservationTriggerAdmissionDecision::RejectedSource { .. }
            ));
        }

        assert!(ObservationTriggerAdmissionPolicy::evaluate(
            &ObservationTriggerRequest::manual(ObservationFreshnessRequirement::AnyAvailable)
        )
        .is_admitted());
    }

    #[test]
    fn retained_ambient_callers_admit_only_once_authorized() {
        let _lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();

        let scheduled =
            ObservationTriggerRequest::scheduled(ObservationFreshnessRequirement::NotStale);
        assert!(matches!(
            ObservationTriggerAdmissionPolicy::evaluate(&scheduled),
            ObservationTriggerAdmissionDecision::RejectedSource { .. }
        ));

        ObservationTriggerAdmissionPolicy::authorize_ambient_capture_for_tests();
        assert!(ObservationTriggerAdmissionPolicy::evaluate(&scheduled).is_admitted());

        ObservationTriggerAdmissionPolicy::reset_for_tests();
        assert!(
            !ObservationTriggerAdmissionPolicy::ambient_capture_authorized(),
            "reset must close the ambient gate"
        );
    }

    #[test]
    fn repeated_admission_rate_limited() {
        let _lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();
        let now = Instant::now();
        let request =
            ObservationTriggerRequest::manual(ObservationFreshnessRequirement::AnyAvailable);

        assert!(ObservationTriggerAdmissionPolicy::evaluate_at(&request, now).is_admitted());
        ObservationTriggerAdmissionPolicy::record_admitted_at(now);

        let limited = ObservationTriggerAdmissionPolicy::evaluate_at(&request, now);
        assert!(matches!(
            limited,
            ObservationTriggerAdmissionDecision::RateLimited { .. }
        ));
    }
}
