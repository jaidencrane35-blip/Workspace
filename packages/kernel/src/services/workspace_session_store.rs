//! WorkspaceSessionStore — durable load/save for persistent runtime session.
//!
//! Never crashes the process on corrupt/missing sessions. Desktop projection is
//! always recomputed; only [`PersistentWorkspaceSession`] is written.

use std::sync::{Arc, Mutex};

use workspace_database::{Database, PersistentWorkspaceSessionRepository};
use workspace_domain::{
    observation_now_rfc3339, ActorContext, IntentContext, ObservationCachePhase,
    PersistentWorkspaceSession, RestoreExecutionPhase, WorkspaceRuntimeState,
};

use crate::error::{KernelError, Result};
use crate::services::WorkspaceRuntimeStateService;

/// Persistence + hydration for the durable subset of WorkspaceRuntimeState.
pub(crate) struct WorkspaceSessionStore;

impl WorkspaceSessionStore {
    /// Load singleton session with migration + corruption recovery.
    ///
    /// Missing / corrupt / incompatible → empty session (never Err for recovery paths).
    pub(crate) fn load_recovered(db: &Database) -> PersistentWorkspaceSession {
        match PersistentWorkspaceSessionRepository::new(db).load() {
            Ok(Some(session)) => match session.migrate() {
                Ok(migrated) => migrated,
                Err(error) => {
                    log::warn!("workspace session migration failed; recovering empty: {error}");
                    PersistentWorkspaceSession::empty()
                }
            },
            Ok(None) => PersistentWorkspaceSession::empty(),
            Err(error) => {
                log::warn!("workspace session load corrupt; recovering empty: {error}");
                PersistentWorkspaceSession::empty()
            }
        }
    }

    /// Atomic save. Refuses partial in-memory snapshots that are mid-flight.
    pub(crate) fn save(db: &Database, session: &PersistentWorkspaceSession) -> Result<()> {
        PersistentWorkspaceSessionRepository::new(db)
            .save(session)
            .map_err(KernelError::from)
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

        let mut session = PersistentWorkspaceSession::from_runtime(runtime);
        if last_saved_context_id.is_some() {
            session.last_saved_context_id = last_saved_context_id;
        } else {
            // Preserve prior saved-context id when not supplied.
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let prior = Self::load_recovered(&guard);
            session.last_saved_context_id = prior.last_saved_context_id;
        }
        session.updated_at = observation_now_rfc3339();
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Self::save(&guard, &session)
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

    /// Startup hydration: load session → seed owner → rebuild derived desktop.
    ///
    /// Observation refresh is deferred (Product Proof: no ambient capture). The
    /// hydrated state marks observation as potentially stale; next Manual/Save/
    /// ensure_observation_freshness refreshes without disrupting Experience.
    pub(crate) fn hydrate_on_startup(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceRuntimeState> {
        let session = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            Self::load_recovered(&guard)
        };

        WorkspaceRuntimeStateService::hydrate_from_session(&session);

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
        }

        Ok(runtime)
    }
}
