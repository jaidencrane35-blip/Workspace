//! Observation schedule runtime (Sprint 114) with health telemetry (Sprint 115).
//!
//! Owns timing for scheduled observation ticks. Emits ticks that call
//! [`ObservationScheduledTrigger`] only — never CaptureCoordinator,
//! WorkspaceObservationService, or Win32.
//!
//! ```text
//! ObservationScheduler
//!         ↓
//! ObservationScheduledTrigger
//!         ↓
//! ObservationTriggerAuthority
//! ```

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    observation_now_rfc3339, ActorContext, IntentContext, ObservationScheduleConfig,
    ObservationSchedulerStatus, ObservationTriggerOutcome,
};

use crate::error::Result;
use crate::services::{
    AuditService, ObservationScheduledTrigger, ObservationTriggerDecision,
};

/// Process-local counters and timestamps for scheduler health.
struct SchedulerMetrics {
    started_at: Mutex<Option<String>>,
    last_tick_at: Mutex<Option<String>>,
    last_tick_duration_ms: Mutex<Option<u64>>,
    ticks_emitted: AtomicU64,
    captures_requested: AtomicU64,
    captures_skipped: AtomicU64,
    rate_limited_count: AtomicU64,
    consecutive_failures: AtomicU64,
}

impl SchedulerMetrics {
    fn new() -> Self {
        Self {
            started_at: Mutex::new(None),
            last_tick_at: Mutex::new(None),
            last_tick_duration_ms: Mutex::new(None),
            ticks_emitted: AtomicU64::new(0),
            captures_requested: AtomicU64::new(0),
            captures_skipped: AtomicU64::new(0),
            rate_limited_count: AtomicU64::new(0),
            consecutive_failures: AtomicU64::new(0),
        }
    }

    fn reset_for_start(&self) {
        *self.started_at.lock().expect("scheduler metrics lock") =
            Some(observation_now_rfc3339());
        *self.last_tick_at.lock().expect("scheduler metrics lock") = None;
        *self
            .last_tick_duration_ms
            .lock()
            .expect("scheduler metrics lock") = None;
        self.ticks_emitted.store(0, Ordering::Release);
        self.captures_requested.store(0, Ordering::Release);
        self.captures_skipped.store(0, Ordering::Release);
        self.rate_limited_count.store(0, Ordering::Release);
        self.consecutive_failures.store(0, Ordering::Release);
    }

    fn record_tick_ok(&self, decision: &ObservationTriggerDecision, duration_ms: u64) {
        self.ticks_emitted.fetch_add(1, Ordering::AcqRel);
        *self.last_tick_at.lock().expect("scheduler metrics lock") =
            Some(observation_now_rfc3339());
        *self
            .last_tick_duration_ms
            .lock()
            .expect("scheduler metrics lock") = Some(duration_ms);

        match decision.outcome() {
            ObservationTriggerOutcome::AcceptedCapture => {
                self.captures_requested.fetch_add(1, Ordering::AcqRel);
                self.consecutive_failures.store(0, Ordering::Release);
            }
            ObservationTriggerOutcome::RateLimited => {
                self.rate_limited_count.fetch_add(1, Ordering::AcqRel);
                self.consecutive_failures.store(0, Ordering::Release);
            }
            ObservationTriggerOutcome::IgnoredFresh
            | ObservationTriggerOutcome::BlockedCaptureInProgress
            | ObservationTriggerOutcome::Unavailable
            | ObservationTriggerOutcome::RejectedSource => {
                self.captures_skipped.fetch_add(1, Ordering::AcqRel);
                self.consecutive_failures.store(0, Ordering::Release);
            }
        }
    }

    fn record_tick_err(&self, duration_ms: u64) {
        self.ticks_emitted.fetch_add(1, Ordering::AcqRel);
        *self.last_tick_at.lock().expect("scheduler metrics lock") =
            Some(observation_now_rfc3339());
        *self
            .last_tick_duration_ms
            .lock()
            .expect("scheduler metrics lock") = Some(duration_ms);
        self.consecutive_failures.fetch_add(1, Ordering::AcqRel);
    }
}

/// Runtime owner of observation schedule ticks.
pub(crate) struct ObservationScheduler {
    config: ObservationScheduleConfig,
    stop: Arc<AtomicBool>,
    metrics: Arc<SchedulerMetrics>,
    /// Database handle retained while running for stop/start lifecycle audits.
    audit_db: Option<Arc<Mutex<Database>>>,
    handle: Option<JoinHandle<()>>,
}

impl ObservationScheduler {
    pub const CONTEXT: &'static str = "schedule:ObservationScheduler";
    pub const AUDIT_STARTED: &'static str = "workspace.observation.scheduler.started";
    pub const AUDIT_STOPPED: &'static str = "workspace.observation.scheduler.stopped";
    pub const AUDIT_TICK: &'static str = "workspace.observation.scheduler.tick";

    pub(crate) fn new(config: ObservationScheduleConfig) -> Self {
        Self {
            config,
            stop: Arc::new(AtomicBool::new(false)),
            metrics: Arc::new(SchedulerMetrics::new()),
            audit_db: None,
            handle: None,
        }
    }

    pub(crate) fn config(&self) -> &ObservationScheduleConfig {
        &self.config
    }

    pub(crate) fn configure(&mut self, config: ObservationScheduleConfig) {
        self.config = config;
    }

    pub(crate) fn is_running(&self) -> bool {
        self.handle.is_some() && !self.stop.load(Ordering::Acquire)
    }

    pub(crate) fn tick_count(&self) -> u64 {
        self.metrics.ticks_emitted.load(Ordering::Acquire)
    }

    /// Lightweight read-only health snapshot (no observation load).
    pub(crate) fn status(&self) -> ObservationSchedulerStatus {
        ObservationSchedulerStatus {
            running: self.is_running(),
            enabled: self.config.enabled,
            interval_seconds: self.config.interval_seconds,
            started_at: self
                .metrics
                .started_at
                .lock()
                .expect("scheduler metrics lock")
                .clone(),
            last_tick_at: self
                .metrics
                .last_tick_at
                .lock()
                .expect("scheduler metrics lock")
                .clone(),
            last_tick_duration_ms: *self
                .metrics
                .last_tick_duration_ms
                .lock()
                .expect("scheduler metrics lock"),
            ticks_emitted: self.metrics.ticks_emitted.load(Ordering::Acquire),
            captures_requested: self.metrics.captures_requested.load(Ordering::Acquire),
            captures_skipped: self.metrics.captures_skipped.load(Ordering::Acquire),
            rate_limited_count: self.metrics.rate_limited_count.load(Ordering::Acquire),
            consecutive_failures: self.metrics.consecutive_failures.load(Ordering::Acquire),
            authority_effect: ObservationSchedulerStatus::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Start the schedule loop after kernel Ready. No-op when disabled or already running.
    pub(crate) fn start(&mut self, db: Arc<Mutex<Database>>) {
        if !self.config.enabled {
            log::debug!("observation scheduler disabled; not starting");
            return;
        }
        if self.handle.is_some() {
            log::debug!("observation scheduler already running");
            return;
        }

        self.stop.store(false, Ordering::Release);
        self.metrics.reset_for_start();
        self.audit_db = Some(Arc::clone(&db));

        let stop = Arc::clone(&self.stop);
        let metrics = Arc::clone(&self.metrics);
        let interval = Duration::from_secs(self.config.interval_seconds.max(1));
        let context = Self::CONTEXT.to_string();
        let interval_secs = self.config.interval_seconds.max(1);
        let audit_db = Arc::clone(&db);

        Self::audit_lifecycle(
            &db,
            Self::AUDIT_STARTED,
            true,
            json!({
                "enabled": true,
                "interval_seconds": interval_secs,
                "context": Self::CONTEXT,
                "authority_effect": "none",
            }),
        );

        let handle = thread::Builder::new()
            .name("observation-scheduler".into())
            .spawn(move || {
                log::info!(
                    "observation scheduler started (interval={}s)",
                    interval.as_secs()
                );
                while !stop.load(Ordering::Acquire) {
                    // Sleep in slices so shutdown can stop promptly.
                    let slice = Duration::from_millis(50);
                    let mut waited = Duration::ZERO;
                    while waited < interval {
                        if stop.load(Ordering::Acquire) {
                            log::info!("observation scheduler stopping during wait");
                            return;
                        }
                        thread::sleep(slice);
                        waited = waited.saturating_add(slice);
                    }
                    if stop.load(Ordering::Acquire) {
                        break;
                    }
                    Self::run_tick(&audit_db, &metrics, &context);
                }
                log::info!("observation scheduler stopped");
            })
            .expect("failed to spawn observation scheduler thread");

        self.handle = Some(handle);
    }

    /// Stop the schedule loop (idempotent). Joins the worker thread.
    pub(crate) fn stop(&mut self) {
        let was_running = self.handle.is_some();
        self.stop.store(true, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        if was_running {
            if let Some(db) = self.audit_db.take() {
                let status = self.status();
                Self::audit_lifecycle(
                    &db,
                    Self::AUDIT_STOPPED,
                    true,
                    json!({
                        "ticks_emitted": status.ticks_emitted,
                        "captures_requested": status.captures_requested,
                        "captures_skipped": status.captures_skipped,
                        "rate_limited_count": status.rate_limited_count,
                        "consecutive_failures": status.consecutive_failures,
                        "context": Self::CONTEXT,
                        "authority_effect": "none",
                    }),
                );
            }
        }
    }

    fn run_tick(db: &Arc<Mutex<Database>>, metrics: &SchedulerMetrics, context: &str) {
        let started = Instant::now();
        let result = ObservationScheduledTrigger::on_tick_from_db(db, Some(context.to_string()));
        let duration_ms = started.elapsed().as_millis() as u64;

        match &result {
            Ok(decision) => {
                metrics.record_tick_ok(decision, duration_ms);
                log::debug!(
                    "observation scheduler tick finished: {}",
                    decision.outcome().as_str()
                );
                // Lean tick audit — one event per tick (default interval is minutes).
                Self::audit_lifecycle(
                    db,
                    Self::AUDIT_TICK,
                    true,
                    json!({
                        "outcome": decision.outcome().as_str(),
                        "duration_ms": duration_ms,
                        "context": context,
                        "authority_effect": "none",
                    }),
                );
            }
            Err(error) => {
                metrics.record_tick_err(duration_ms);
                log::warn!("observation scheduler tick failed: {error}");
                Self::audit_lifecycle(
                    db,
                    Self::AUDIT_TICK,
                    false,
                    json!({
                        "outcome": "error",
                        "duration_ms": duration_ms,
                        "error": error.to_string(),
                        "context": context,
                        "authority_effect": "none",
                    }),
                );
            }
        }
    }

    fn audit_lifecycle(
        db: &Arc<Mutex<Database>>,
        event_type: &str,
        success: bool,
        metadata: serde_json::Value,
    ) {
        let actor = ActorContext::system();
        let intent = IntentContext::system_startup();
        if let Err(error) = AuditService::record_ai_planning_event(
            db,
            &actor,
            &intent,
            event_type,
            success,
            metadata.to_string(),
        ) {
            log::warn!("observation scheduler audit failed ({event_type}): {error}");
        }
    }

    /// Run one tick immediately (tests). Still goes through ScheduledTrigger → Authority.
    #[cfg(test)]
    pub(crate) fn emit_tick_now_for_tests(
        &self,
        db: &Arc<Mutex<Database>>,
        capturer: &dyn workspace_windows_integration::DesktopCapturer,
    ) -> Result<ObservationTriggerDecision> {
        let started = Instant::now();
        let result = ObservationScheduledTrigger::on_tick_from_db_with(
            db,
            Some(Self::CONTEXT.into()),
            capturer,
        );
        let duration_ms = started.elapsed().as_millis() as u64;
        match &result {
            Ok(decision) => {
                self.metrics.record_tick_ok(decision, duration_ms);
                Self::audit_lifecycle(
                    db,
                    Self::AUDIT_TICK,
                    true,
                    json!({
                        "outcome": decision.outcome().as_str(),
                        "duration_ms": duration_ms,
                        "context": Self::CONTEXT,
                        "authority_effect": "none",
                    }),
                );
            }
            Err(_) => {
                self.metrics.record_tick_err(duration_ms);
                Self::audit_lifecycle(
                    db,
                    Self::AUDIT_TICK,
                    false,
                    json!({
                        "outcome": "error",
                        "duration_ms": duration_ms,
                        "context": Self::CONTEXT,
                        "authority_effect": "none",
                    }),
                );
            }
        }
        result
    }
}

impl Drop for ObservationScheduler {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Read-only scheduler diagnostics (Sprint 115). No start/stop control.
pub(crate) struct ObservationSchedulerDiagnostics;

impl ObservationSchedulerDiagnostics {
    pub(crate) fn get_status(scheduler: &ObservationScheduler) -> ObservationSchedulerStatus {
        scheduler.status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use workspace_windows_integration::{
        DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer, WindowsIntegrationError,
    };

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{
        ObservationTriggerAdmissionPolicy, WorkspaceObservationService,
    };
    use crate::WorkspaceKernel;

    struct FailingCapturer;

    impl DesktopCapturer for FailingCapturer {
        fn capture_desktop(&self) -> workspace_windows_integration::Result<DesktopObservationCapture> {
            Err(WindowsIntegrationError::EnumerationFailed(
                "scheduler forced failure".into(),
            ))
        }
    }

    fn begin_scheduler_test() -> std::sync::MutexGuard<'static, ()> {
        let lock = observation_flight_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ObservationTriggerAdmissionPolicy::reset_for_tests();
        lock
    }

    #[test]
    fn enabled_scheduler_emits_tick() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();

        let mut scheduler =
            ObservationScheduler::new(ObservationScheduleConfig::enabled_with_interval(1));
        assert_eq!(scheduler.tick_count(), 0);
        scheduler.start(Arc::clone(&db));
        assert!(scheduler.is_running());

        let deadline = std::time::Instant::now() + Duration::from_secs(3);
        while scheduler.tick_count() == 0 && std::time::Instant::now() < deadline {
            thread::sleep(Duration::from_millis(50));
        }
        assert!(
            scheduler.tick_count() >= 1,
            "expected at least one scheduler tick"
        );
        scheduler.stop();
        assert!(!scheduler.is_running());
    }

    #[test]
    fn disabled_scheduler_emits_no_tick() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let mut scheduler = ObservationScheduler::new(ObservationScheduleConfig::disabled());
        scheduler.start(kernel.shared_database());
        assert!(!scheduler.is_running());
        thread::sleep(Duration::from_millis(200));
        assert_eq!(scheduler.tick_count(), 0);
        let status = scheduler.status();
        assert!(!status.enabled);
        assert!(!status.running);
        assert_eq!(status.ticks_emitted, 0);
        scheduler.stop();
    }

    #[test]
    fn shutdown_stops_scheduler() {
        let _lock = begin_scheduler_test();
        let mut kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        {
            let db = kernel.shared_database();
            kernel
                .observation_scheduler_mut()
                .configure(ObservationScheduleConfig::enabled_with_interval(1));
            kernel.observation_scheduler_mut().start(db);
            assert!(kernel.observation_scheduler().is_running());
        }
        kernel.begin_shutdown();
        assert!(!kernel.observation_scheduler().is_running());
    }

    #[test]
    fn trigger_authority_remains_required_and_provenance_preserved() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let scheduler = ObservationScheduler::new(ObservationScheduleConfig::disabled());

        let decision = scheduler
            .emit_tick_now_for_tests(&db, &StubDesktopCapturer::fixture_dual_monitor())
            .unwrap();
        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));
        assert_eq!(scheduler.tick_count(), 1);

        let events = AuditService::list_recent(&db, 40).unwrap();
        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.trigger.received"
        }));
        let received = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.trigger.received")
            .unwrap();
        let meta = received.metadata.as_deref().unwrap();
        assert!(meta.contains("\"source\":\"scheduled\""));
        assert!(meta.contains(ObservationScheduledTrigger::REASON));
        assert!(meta.contains(ObservationScheduler::CONTEXT));
        assert!(events.iter().any(|event| {
            event.event_type == "workspace.observation.capture.requested"
        }));
        let capture = events
            .iter()
            .find(|event| event.event_type == "workspace.observation.capture.requested")
            .unwrap();
        let capture_meta = capture.metadata.as_deref().unwrap();
        assert!(capture_meta.contains("\"source\":\"scheduled\""));
        assert!(capture_meta.contains(ObservationScheduler::CONTEXT));
    }

    #[test]
    fn in_memory_kernel_does_not_auto_start_scheduler() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        assert!(!kernel.observation_scheduler().config().enabled);
        assert!(!kernel.observation_scheduler().is_running());
        assert_eq!(kernel.observation_scheduler().tick_count(), 0);

        // Deterministic: no observation unless a test captures explicitly.
        let status = WorkspaceObservationService::get_status(
            &kernel.shared_database(),
            &ActorContext::local_user(),
            &IntentContext::user_request(),
        )
        .unwrap();
        assert!(!status.has_observation);
    }

    #[test]
    fn scheduler_start_and_stop_audited() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let mut scheduler =
            ObservationScheduler::new(ObservationScheduleConfig::enabled_with_interval(60));

        scheduler.start(Arc::clone(&db));
        assert!(scheduler.is_running());
        let status = ObservationSchedulerDiagnostics::get_status(&scheduler);
        assert!(status.running);
        assert!(status.enabled);
        assert_eq!(status.interval_seconds, 60);
        assert!(status.started_at.is_some());

        let events = AuditService::list_recent(&db, 20).unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == ObservationScheduler::AUDIT_STARTED));

        scheduler.stop();
        assert!(!scheduler.is_running());
        let events = AuditService::list_recent(&db, 20).unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == ObservationScheduler::AUDIT_STOPPED));
        assert!(!ObservationSchedulerDiagnostics::get_status(&scheduler).running);
    }

    #[test]
    fn scheduler_status_reports_tick_accounting() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let scheduler = ObservationScheduler::new(ObservationScheduleConfig::disabled());

        let decision = scheduler
            .emit_tick_now_for_tests(&db, &StubDesktopCapturer::fixture_dual_monitor())
            .unwrap();
        assert!(matches!(
            decision,
            ObservationTriggerDecision::AcceptedCapture(_)
        ));

        let status = scheduler.status();
        assert_eq!(status.ticks_emitted, 1);
        assert_eq!(status.captures_requested, 1);
        assert_eq!(status.captures_skipped, 0);
        assert_eq!(status.rate_limited_count, 0);
        assert_eq!(status.consecutive_failures, 0);
        assert!(status.last_tick_at.is_some());
        assert!(status.last_tick_duration_ms.is_some());
        assert_eq!(
            status.authority_effect,
            ObservationSchedulerStatus::AUTHORITY_EFFECT_NONE
        );

        let events = AuditService::list_recent(&db, 30).unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == ObservationScheduler::AUDIT_TICK));
    }

    #[test]
    fn scheduler_failure_accounting() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let scheduler = ObservationScheduler::new(ObservationScheduleConfig::disabled());

        let err = scheduler.emit_tick_now_for_tests(&db, &FailingCapturer);
        assert!(err.is_err());

        let status = scheduler.status();
        assert_eq!(status.ticks_emitted, 1);
        assert_eq!(status.captures_requested, 0);
        assert_eq!(status.consecutive_failures, 1);
        assert!(status.last_tick_at.is_some());

        ObservationTriggerAdmissionPolicy::reset_for_tests();
        let err2 = scheduler.emit_tick_now_for_tests(&db, &FailingCapturer);
        assert!(err2.is_err());
        assert_eq!(scheduler.status().consecutive_failures, 2);
    }

    #[test]
    fn skipped_ticks_accounted_when_fresh() {
        let _lock = begin_scheduler_test();
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let scheduler = ObservationScheduler::new(ObservationScheduleConfig::disabled());
        let capturer = StubDesktopCapturer::fixture_dual_monitor();

        assert!(matches!(
            scheduler.emit_tick_now_for_tests(&db, &capturer).unwrap(),
            ObservationTriggerDecision::AcceptedCapture(_)
        ));
        ObservationTriggerAdmissionPolicy::reset_for_tests();
        let second = scheduler.emit_tick_now_for_tests(&db, &capturer).unwrap();
        // Fresh observation → ignored (NotStale requirement).
        assert!(matches!(second, ObservationTriggerDecision::IgnoredFresh));

        let status = scheduler.status();
        assert_eq!(status.ticks_emitted, 2);
        assert_eq!(status.captures_requested, 1);
        assert_eq!(status.captures_skipped, 1);
    }
}
