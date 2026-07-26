//! Observation schedule runtime (Sprint 114).
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
use std::time::Duration;

use workspace_database::Database;
use workspace_domain::ObservationScheduleConfig;

use crate::services::ObservationScheduledTrigger;

/// Runtime owner of observation schedule ticks.
pub(crate) struct ObservationScheduler {
    config: ObservationScheduleConfig,
    stop: Arc<AtomicBool>,
    tick_count: Arc<AtomicU64>,
    handle: Option<JoinHandle<()>>,
}

impl ObservationScheduler {
    pub const CONTEXT: &'static str = "schedule:ObservationScheduler";

    pub(crate) fn new(config: ObservationScheduleConfig) -> Self {
        Self {
            config,
            stop: Arc::new(AtomicBool::new(false)),
            tick_count: Arc::new(AtomicU64::new(0)),
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
        self.tick_count.load(Ordering::Acquire)
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
        let stop = Arc::clone(&self.stop);
        let tick_count = Arc::clone(&self.tick_count);
        let interval = Duration::from_secs(self.config.interval_seconds.max(1));
        let context = Self::CONTEXT.to_string();

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
                    tick_count.fetch_add(1, Ordering::AcqRel);
                    match ObservationScheduledTrigger::on_tick_from_db(&db, Some(context.clone())) {
                        Ok(decision) => {
                            log::debug!(
                                "observation scheduler tick finished: {}",
                                decision.outcome().as_str()
                            );
                        }
                        Err(error) => {
                            log::warn!("observation scheduler tick failed: {error}");
                        }
                    }
                }
                log::info!("observation scheduler stopped");
            })
            .expect("failed to spawn observation scheduler thread");

        self.handle = Some(handle);
    }

    /// Stop the schedule loop (idempotent). Joins the worker thread.
    pub(crate) fn stop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    /// Run one tick immediately (tests). Still goes through ScheduledTrigger → Authority.
    #[cfg(test)]
    pub(crate) fn emit_tick_now_for_tests(
        &self,
        db: &Arc<Mutex<Database>>,
        capturer: &dyn workspace_windows_integration::DesktopCapturer,
    ) -> crate::error::Result<crate::services::ObservationTriggerDecision> {
        self.tick_count.fetch_add(1, Ordering::AcqRel);
        ObservationScheduledTrigger::on_tick_from_db_with(
            db,
            Some(Self::CONTEXT.into()),
            capturer,
        )
    }
}

impl Drop for ObservationScheduler {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use workspace_windows_integration::StubDesktopCapturer;

    use crate::services::capture_coordinator::observation_flight_test_lock;
    use crate::services::{
        AuditService, ObservationTriggerAdmissionPolicy, ObservationTriggerDecision,
        WorkspaceObservationService,
    };
    use crate::WorkspaceKernel;
    use workspace_domain::{ActorContext, IntentContext};

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

        let mut scheduler = ObservationScheduler::new(ObservationScheduleConfig::enabled_with_interval(1));
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
}
