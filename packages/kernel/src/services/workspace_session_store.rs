//! WorkspaceSessionStore — durable load/save for persistent runtime session.
//!
//! Never crashes the process on corrupt/missing sessions. Desktop projection is
//! always recomputed; only [`PersistentWorkspaceSession`] is written.
//! Recovery fences + health: `architecture/28_Runtime_Recovery_Model.md`.

use std::sync::{Arc, Mutex};

use workspace_database::{Database, PersistentWorkspaceSessionRepository};
use workspace_domain::{
    observation_now_rfc3339, ActorContext, IntentContext, ObservationCachePhase,
    PendingOperationFence, PendingOperationKind, PersistentWorkspaceSession, RecoveryDisposition,
    RecoveryRecord, RestoreExecutionPhase, RuntimeHealth, SessionIntegrity,
    WorkspaceRuntimeState,
};

use crate::error::{KernelError, Result};
use crate::services::WorkspaceRuntimeStateService;

/// Outcome of a recovered load (never panics the process).
#[derive(Debug, Clone)]
pub(crate) struct SessionLoadOutcome {
    pub session: PersistentWorkspaceSession,
    pub integrity: SessionIntegrity,
    pub interrupted: Option<PendingOperationFence>,
}

/// Persistence + hydration for the durable subset of WorkspaceRuntimeState.
pub(crate) struct WorkspaceSessionStore;

impl WorkspaceSessionStore {
    /// Load singleton session with migration + corruption recovery metadata.
    pub(crate) fn load_outcome(db: &Database) -> SessionLoadOutcome {
        match PersistentWorkspaceSessionRepository::new(db).load() {
            Ok(Some(session)) => match session.migrate() {
                Ok(migrated) => SessionLoadOutcome {
                    interrupted: migrated.pending_operation.clone(),
                    session: migrated,
                    integrity: SessionIntegrity::Ok,
                },
                Err(error) => {
                    log::warn!("workspace session migration failed; recovering empty: {error}");
                    let integrity = if error.to_string().contains("incompatible") {
                        SessionIntegrity::IncompatibleRecovered
                    } else {
                        SessionIntegrity::MigrationRecovered
                    };
                    SessionLoadOutcome {
                        session: PersistentWorkspaceSession::empty(),
                        integrity,
                        interrupted: None,
                    }
                }
            },
            Ok(None) => SessionLoadOutcome {
                session: PersistentWorkspaceSession::empty(),
                integrity: SessionIntegrity::MissingRecovered,
                interrupted: None,
            },
            Err(error) => {
                log::warn!("workspace session load corrupt; recovering empty: {error}");
                SessionLoadOutcome {
                    session: PersistentWorkspaceSession::empty(),
                    integrity: SessionIntegrity::CorruptRecovered,
                    interrupted: None,
                }
            }
        }
    }

    /// Load singleton session with migration + corruption recovery.
    ///
    /// Missing / corrupt / incompatible → empty session (never Err for recovery paths).
    pub(crate) fn load_recovered(db: &Database) -> PersistentWorkspaceSession {
        Self::load_outcome(db).session
    }

    /// Atomic save. Refuses partial in-memory snapshots that are mid-flight.
    pub(crate) fn save(db: &Database, session: &PersistentWorkspaceSession) -> Result<()> {
        PersistentWorkspaceSessionRepository::new(db)
            .save(session)
            .map_err(KernelError::from)
    }

    /// Write a durable operation fence before a mutating runtime path.
    pub(crate) fn begin_operation(
        db: &Arc<Mutex<Database>>,
        kind: PendingOperationKind,
        correlation_id: Option<String>,
    ) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        let mut session = Self::load_recovered(&guard);
        session.pending_operation = Some(PendingOperationFence::new(kind, correlation_id));
        session.updated_at = observation_now_rfc3339();
        Self::save(&guard, &session)?;
        Ok(())
    }

    /// Clear a fence after a failed (non-crash) operation without discarding prior durable fields.
    pub(crate) fn clear_operation_fence(db: &Arc<Mutex<Database>>) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        let mut session = Self::load_recovered(&guard);
        if session.pending_operation.is_none() {
            return Ok(());
        }
        session.pending_operation = None;
        session.updated_at = observation_now_rfc3339();
        Self::save(&guard, &session)
    }

    /// Checkpoint from a fully assembled runtime state (success paths only).
    pub(crate) fn checkpoint_runtime(
        db: &Arc<Mutex<Database>>,
        runtime: &WorkspaceRuntimeState,
        last_saved_context_id: Option<String>,
    ) -> Result<()> {
        // Never persist while capture/execute is in-flight (partial write guard).
        // RefreshCompleted / Failed / Idle are durable-safe.
        let in_flight_cache = matches!(
            runtime.cache_phase,
            ObservationCachePhase::RefreshRequested | ObservationCachePhase::RefreshInProgress
        );
        let in_flight_exec = matches!(
            runtime.execution_phase,
            RestoreExecutionPhase::Planning
                | RestoreExecutionPhase::Validating
                | RestoreExecutionPhase::Executing
        );
        if in_flight_cache || in_flight_exec {
            log::debug!(
                "skipping session checkpoint during in-flight phase cache={:?} exec={:?}",
                runtime.cache_phase,
                runtime.execution_phase
            );
            return Ok(());
        }

        let prior = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            Self::load_recovered(&guard)
        };

        let mut session = PersistentWorkspaceSession::from_runtime(runtime);
        if last_saved_context_id.is_some() {
            session.last_saved_context_id = last_saved_context_id;
        } else {
            session.last_saved_context_id = prior.last_saved_context_id;
        }
        // Preserve acknowledged recovery; clear fence (success path).
        session.last_recovery = prior.last_recovery;
        session.pending_operation = None;
        session.updated_at = observation_now_rfc3339();

        if let Err(message) = assert_runtime_session_consistent(runtime, &session) {
            // Production diagnostic (operator logs). Never panics the process.
            log::warn!("runtime/session consistency warning before checkpoint: {message}");
            debug_assert!(
                false,
                "runtime/session inconsistency before checkpoint: {message}"
            );
        }

        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Self::save(&guard, &session)?;
        WorkspaceRuntimeStateService::note_persistence_success(&session.updated_at);
        Ok(())
    }

    /// Convenience: assemble current runtime and checkpoint.
    pub(crate) fn checkpoint_current(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        last_saved_context_id: Option<String>,
    ) -> Result<()> {
        let runtime = WorkspaceRuntimeStateService::current(db, actor, intent)?;
        Self::checkpoint_runtime(db, &runtime, last_saved_context_id)
    }

    /// Startup hydration: load → acknowledge interruptions → seed owner → rebuild derived.
    ///
    /// Observation refresh is deferred (Product Proof: no ambient capture). The
    /// hydrated state marks observation as potentially stale; next Manual/Save/
    /// ensure_observation_freshness refreshes without disrupting Experience.
    pub(crate) fn hydrate_on_startup(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceRuntimeState> {
        let outcome = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            Self::load_outcome(&guard)
        };

        let mut health = RuntimeHealth::healthy().with_integrity(outcome.integrity);
        let mut session = outcome.session;

        // Detect interrupted operations — never silently discard.
        if let Some(fence) = outcome.interrupted {
            let record = RecoveryRecord::from_fence(
                &fence,
                format!(
                    "startup detected uncleared {} fence",
                    fence.kind.as_str()
                ),
            );
            log::warn!(
                "workspace session interrupted operation recovered: kind={} disposition={}",
                record.kind.as_str(),
                record.disposition.as_str()
            );
            health = health.with_recovery(record.clone());
            session.last_recovery = Some(record);
            session.pending_operation = None;
            session.updated_at = observation_now_rfc3339();
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            // Best-effort acknowledge; failure must not crash startup.
            if let Err(error) = Self::save(&guard, &session) {
                log::warn!("failed to persist recovery acknowledgment: {error}");
            }
        } else if let Some(prior) = session.last_recovery.clone() {
            // Re-surface prior acknowledged recovery into process health.
            health.pending_recovery = Some(prior.clone());
            health.recovery_disposition = prior.disposition;
            health.recompute_degraded();
        }

        if outcome.integrity == SessionIntegrity::MissingRecovered {
            // First boot / empty — not degraded.
            health.session_integrity = SessionIntegrity::MissingRecovered;
            health.recovery_disposition = RecoveryDisposition::None;
            health.degraded = false;
        }

        if let Some(ts) = session.last_capture_timestamp.clone() {
            health.last_successful_observation_at = Some(ts);
        }
        health.last_persistence_at = Some(session.updated_at.clone());
        if let Some(entry) = session.restore_history.first() {
            health.last_restore_at = Some(entry.recorded_at.clone());
        }

        WorkspaceRuntimeStateService::hydrate_from_session(&session, health);

        // Rebuild derived desktop from latest observation (may be empty).
        let runtime = WorkspaceRuntimeStateService::current(db, actor, intent)?;

        // If persisted pass differs from live observation, keep live (derived)
        // and leave cache Idle so a later explicit refresh can update.
        if session.last_observation_pass_id.is_some()
            && session.last_observation_pass_id != runtime.observation.pass_id
        {
            log::info!(
                "persisted observation pass {:?} differs from live {:?}; publishing live projection",
                session.last_observation_pass_id,
                runtime.observation.pass_id
            );
            WorkspaceRuntimeStateService::note_needs_refresh_after_hydrate();
        }

        Ok(WorkspaceRuntimeStateService::current(db, actor, intent)?)
    }
}

/// Consistency gate: runtime vs durable session must not disagree after success.
pub(crate) fn assert_runtime_session_consistent(
    runtime: &WorkspaceRuntimeState,
    session: &PersistentWorkspaceSession,
) -> std::result::Result<(), String> {
    if runtime.observation.pass_id != session.last_observation_pass_id {
        return Err(format!(
            "observation pass mismatch runtime={:?} session={:?}",
            runtime.observation.pass_id, session.last_observation_pass_id
        ));
    }
    if runtime.confidence_band != session.last_confidence_band {
        return Err(format!(
            "confidence band mismatch runtime={:?} session={:?}",
            runtime.confidence_band, session.last_confidence_band
        ));
    }
    let runtime_hist: Vec<_> = runtime
        .restore_history
        .iter()
        .take(WorkspaceRuntimeState::RESTORE_HISTORY_LIMIT)
        .cloned()
        .collect();
    if runtime_hist != session.restore_history {
        return Err("restore_history mismatch between runtime and session".into());
    }
    if session.pending_operation.is_some() {
        return Err("success checkpoint must clear pending_operation".into());
    }
    Ok(())
}
