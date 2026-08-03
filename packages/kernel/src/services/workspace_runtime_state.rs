//! Process-local owner of [`WorkspaceRuntimeState`].
//!
//! Sole authoritative live desktop representation for runtime consumers.
//! Capture still flows through [`CaptureCoordinator`]; Action still mutates via
//! [`WindowMutator`]. This service owns projection cache, cache phase, execution
//! phase, restore history publication, and internal [`RuntimeHealth`].

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock};

use workspace_database::{Database, ObservationPassRepository};
use workspace_domain::{
    observation_now_rfc3339, ActorContext, IntentContext, ObservationCachePhase,
    OperationOutcome, PersistentWorkspaceSession, RecoveryDisposition, RestoreCompatibilitySummary,
    RestoreExecutionPhase, RestoreHistoryEntry, RuntimeHealth, WorkspaceObservationDelta,
    WorkspaceObservationStatus, WorkspaceRuntimeState, WorkspaceState,
};

use crate::error::Result;
use crate::services::configuration::ConfigurationService;
use crate::services::{
    CaptureCoordinator, ObservationDeltaService, WorkspaceObservationService, WorkspaceStateEngine,
};

#[derive(Debug)]
struct RuntimeOwnerInner {
    generation: u64,
    cached_pass_id: Option<String>,
    cached_desktop: Option<WorkspaceState>,
    cache_phase: ObservationCachePhase,
    execution_phase: RestoreExecutionPhase,
    restore_history: VecDeque<RestoreHistoryEntry>,
    confidence_band: Option<String>,
    health: RuntimeHealth,
}

impl Default for RuntimeOwnerInner {
    fn default() -> Self {
        Self {
            generation: 0,
            cached_pass_id: None,
            cached_desktop: None,
            cache_phase: ObservationCachePhase::Idle,
            execution_phase: RestoreExecutionPhase::Idle,
            restore_history: VecDeque::new(),
            confidence_band: None,
            health: RuntimeHealth::healthy(),
        }
    }
}

fn owner() -> &'static Mutex<RuntimeOwnerInner> {
    static OWNER: OnceLock<Mutex<RuntimeOwnerInner>> = OnceLock::new();
    OWNER.get_or_init(|| Mutex::new(RuntimeOwnerInner::default()))
}

/// Canonical live workspace runtime state service.
pub(crate) struct WorkspaceRuntimeStateService;

impl WorkspaceRuntimeStateService {
    /// Reset process-local owner (tests only).
    #[cfg(test)]
    pub(crate) fn reset_for_tests() {
        *owner().lock().expect("runtime owner") = RuntimeOwnerInner::default();
    }

    /// Seed durable fields from a recovered session (startup hydration).
    pub(crate) fn hydrate_from_session(session: &PersistentWorkspaceSession, health: RuntimeHealth) {
        let mut guard = owner().lock().expect("runtime owner");
        guard.cached_pass_id = session.last_observation_pass_id.clone();
        guard.cached_desktop = None;
        guard.confidence_band = session.last_confidence_band.clone();
        guard.restore_history = session.restore_history.iter().cloned().collect();
        guard.cache_phase = ObservationCachePhase::Idle;
        guard.execution_phase = RestoreExecutionPhase::Idle;
        guard.health = health;
        guard.generation = guard.generation.saturating_add(1);
    }

    pub(crate) fn note_refresh_requested() {
        let mut guard = owner().lock().expect("runtime owner");
        if guard.cache_phase != ObservationCachePhase::RefreshInProgress {
            guard.cache_phase = ObservationCachePhase::RefreshRequested;
        }
    }

    pub(crate) fn note_refresh_in_progress() {
        owner().lock().expect("runtime owner").cache_phase =
            ObservationCachePhase::RefreshInProgress;
    }

    pub(crate) fn note_refresh_completed(pass_id: Option<&str>) {
        let mut guard = owner().lock().expect("runtime owner");
        guard.cache_phase = ObservationCachePhase::RefreshCompleted;
        guard.generation = guard.generation.saturating_add(1);
        guard.cached_desktop = None;
        guard.cached_pass_id = pass_id.map(str::to_string);
        guard
            .health
            .note_observation_success(observation_now_rfc3339());
    }

    pub(crate) fn note_refresh_failed() {
        let mut guard = owner().lock().expect("runtime owner");
        guard.cache_phase = ObservationCachePhase::RefreshFailed;
        guard.generation = guard.generation.saturating_add(1);
        guard.cached_desktop = None;
    }

    pub(crate) fn note_refresh_rejected_concurrent() {
        // Keep in-progress — the winning flight owns the cache phase.
        let mut guard = owner().lock().expect("runtime owner");
        if guard.cache_phase != ObservationCachePhase::RefreshInProgress {
            guard.cache_phase = ObservationCachePhase::RefreshInProgress;
        }
    }

    pub(crate) fn note_needs_refresh_after_hydrate() {
        let mut guard = owner().lock().expect("runtime owner");
        if guard.health.recovery_disposition == RecoveryDisposition::None {
            guard.health.recovery_disposition = RecoveryDisposition::NeedsRefresh;
        }
        guard.health.recompute_degraded();
    }

    pub(crate) fn note_execution_phase(phase: RestoreExecutionPhase) {
        owner().lock().expect("runtime owner").execution_phase = phase;
    }

    pub(crate) fn note_compatibility(summary: &RestoreCompatibilitySummary) {
        owner().lock().expect("runtime owner").confidence_band =
            Some(summary.confidence_band.clone());
    }

    pub(crate) fn note_execution_finished(
        operation_id: impl Into<String>,
        outcome: OperationOutcome,
        summary: workspace_domain::RestoreExecutionSummary,
        purpose: impl Into<String>,
    ) {
        let mut guard = owner().lock().expect("runtime owner");
        let recorded_at = observation_now_rfc3339();
        guard.execution_phase = RestoreExecutionPhase::from_operation_outcome(&outcome);
        guard.restore_history.push_front(RestoreHistoryEntry {
            recorded_at: recorded_at.clone(),
            operation_id: operation_id.into(),
            outcome,
            summary,
            purpose: purpose.into(),
        });
        while guard.restore_history.len() > WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT {
            guard.restore_history.pop_back();
        }
        guard.health.note_restore_success(recorded_at);
    }

    pub(crate) fn note_persistence_success(at: &str) {
        owner()
            .lock()
            .expect("runtime owner")
            .health
            .note_persistence_success(at.to_string());
    }

    /// Invalidate cached desktop projection (forces rebuild on next read).
    pub(crate) fn invalidate() {
        let mut guard = owner().lock().expect("runtime owner");
        guard.cached_desktop = None;
        guard.generation = guard.generation.saturating_add(1);
    }

    /// Desktop projection only — cached when observation pass unchanged.
    pub(crate) fn desktop_projection(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceState> {
        Ok(Self::current(db, actor, intent)?.desktop)
    }

    /// Full canonical runtime state bundle.
    pub(crate) fn current(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceRuntimeState> {
        let observation = WorkspaceObservationService::get_status(db, actor, intent)?;
        let active_workspace_id = {
            let guard = db.lock().map_err(|_| crate::error::KernelError::NotReady)?;
            ConfigurationService::load(&guard)
                .ok()
                .and_then(|settings| settings.active_workspace_id)
        };

        {
            let mut guard = owner().lock().expect("runtime owner");
            if CaptureCoordinator::is_capture_in_progress() {
                guard.cache_phase = ObservationCachePhase::RefreshInProgress;
            }
        }

        let pass_id = observation.pass_id.clone();
        let cache_hit = {
            let guard = owner().lock().expect("runtime owner");
            matches!(
                (&guard.cached_desktop, &guard.cached_pass_id, &pass_id),
                (Some(_), Some(cached_pass), Some(pass)) if cached_pass == pass
            )
        };

        let desktop = if cache_hit {
            owner()
                .lock()
                .expect("runtime owner")
                .cached_desktop
                .clone()
                .expect("cache hit")
        } else {
            let rebuilt = Self::project_unlocked(db, actor, intent)?;
            let mut guard = owner().lock().expect("runtime owner");
            guard.cached_desktop = Some(rebuilt.clone());
            guard.cached_pass_id = pass_id.clone();
            rebuilt
        };

        let (generation, cache_phase, execution_phase, restore_history, confidence_band, health) = {
            let guard = owner().lock().expect("runtime owner");
            (
                guard.generation,
                guard.cache_phase,
                guard.execution_phase,
                guard.restore_history.iter().cloned().collect::<Vec<_>>(),
                guard.confidence_band.clone(),
                guard.health.clone(),
            )
        };

        let active_monitor_index = desktop
            .windows
            .iter()
            .find(|window| window.focused)
            .and_then(|window| window.monitor_index)
            .or_else(|| {
                desktop
                    .windows
                    .iter()
                    .find_map(|window| window.monitor_index)
            });

        Ok(WorkspaceRuntimeState {
            desktop,
            observation: observation.clone(),
            cache_phase,
            execution_phase,
            active_workspace_id,
            active_monitor_index,
            capture_timestamp: observation.captured_at.clone(),
            confidence_band,
            restore_history,
            health,
            generation,
            authority_effect: WorkspaceRuntimeState::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn project_unlocked(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceState> {
        let delta = ObservationDeltaService::get_latest(db, actor, intent)?;
        let observation = {
            let guard = db.lock().expect("database lock poisoned");
            ObservationPassRepository::new(&guard).load_latest_snapshot()?
        };
        Ok(WorkspaceStateEngine::build(observation.as_ref(), &delta))
    }

    /// Test helper: whether desktop projection is cached for a pass.
    #[cfg(test)]
    pub(crate) fn cached_pass_id_for_tests() -> Option<String> {
        owner().lock().expect("runtime owner").cached_pass_id.clone()
    }

    #[cfg(test)]
    pub(crate) fn cache_phase_for_tests() -> ObservationCachePhase {
        owner().lock().expect("runtime owner").cache_phase
    }

    #[cfg(test)]
    pub(crate) fn execution_phase_for_tests() -> RestoreExecutionPhase {
        owner().lock().expect("runtime owner").execution_phase
    }

    #[cfg(test)]
    pub(crate) fn health_for_tests() -> RuntimeHealth {
        owner().lock().expect("runtime owner").health.clone()
    }
}

/// Suppress unused import noise when delta type is only needed for docs.
#[allow(dead_code)]
fn _delta_type(_: &WorkspaceObservationDelta) {}

#[allow(dead_code)]
fn _status_type(_: &WorkspaceObservationStatus) {}
