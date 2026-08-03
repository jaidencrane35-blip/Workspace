//! Saving a bounded workspace context on explicit user instruction (PP-M1-01).
//!
//! Order matters here and is the point of the module: everything that can be
//! checked is checked *before* the desktop is read. A save that is going to be
//! refused must be refused without observing anything, because a user who is
//! told "that did not work" should not have to wonder what was looked at.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use uuid::Uuid;
use workspace_database::{Database, SavedContextRepository};
use workspace_domain::{
    ActorContext, CaptureRequest, IntentContext, ObservedMonitor, ObservedWindow,
    SaveContextRequest, SavedContext, SavedContextId, SavedContextMonitor,
    SavedContextRestoreIdentity, SavedContextWindow, WorkspaceObservationSnapshot,
};
use workspace_windows_integration::DesktopCapturer;

use crate::error::{KernelError, Result};
use crate::services::{CaptureCoordinator, WorkspaceService, WorkspaceSessionStore};

pub(crate) struct SavedContextService;

impl SavedContextService {
    pub(crate) fn list_for_workspace(
        db: &Arc<Mutex<Database>>,
        workspace_id: &workspace_domain::WorkspaceId,
    ) -> Result<Vec<SavedContext>> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(SavedContextRepository::new(&guard).list_for_workspace(workspace_id)?)
    }

    pub(crate) fn get_by_id(
        db: &Arc<Mutex<Database>>,
        id: &SavedContextId,
    ) -> Result<Option<SavedContext>> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(SavedContextRepository::new(&guard).get_by_id(id)?)
    }

    pub(crate) fn delete_by_id(db: &Arc<Mutex<Database>>, id: &SavedContextId) -> Result<bool> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(SavedContextRepository::new(&guard).delete_by_id(id)?)
    }

    /// Saves one named context, capturing the desktop through the platform capturer.
    pub(crate) fn save(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &SaveContextRequest,
    ) -> Result<SavedContext> {
        Self::save_capturing(db, actor, intent, request, |db| {
            CaptureCoordinator::request_capture(db, actor, intent, Self::capture_request())
                .and_then(CaptureCoordinator::into_capture_result)
        })
    }

    /// Saves one named context with an injected capturer (tests / fixtures).
    pub(crate) fn save_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &SaveContextRequest,
        capturer: &dyn DesktopCapturer,
    ) -> Result<SavedContext> {
        Self::save_capturing(db, actor, intent, request, |db| {
            CaptureCoordinator::request_capture_with(
                db,
                actor,
                intent,
                Self::capture_request(),
                capturer,
            )
            .and_then(CaptureCoordinator::into_capture_result)
        })
    }

    /// The capture is attributed to the person who asked for it, never to a
    /// schedule or a background trigger.
    fn capture_request() -> CaptureRequest {
        CaptureRequest::manual()
            .with_reason("save_workspace_context")
            .with_context("command:SaveWorkspaceContext")
    }

    fn save_capturing<C>(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        request: &SaveContextRequest,
        capture: C,
    ) -> Result<SavedContext>
    where
        C: FnOnce(
            &Arc<Mutex<Database>>,
        ) -> Result<crate::services::WorkspaceObservationCaptureResult>,
    {
        // Nothing below this line may observe the desktop.
        request.validate().map_err(KernelError::from)?;
        Self::require_workspace(db, request)?;

        // Durable fence: crash mid-save → Incomplete on next startup.
        let _ = WorkspaceSessionStore::begin_operation(
            db,
            workspace_domain::PendingOperationKind::Save,
            Some(request.workspace_id.to_string()),
        );

        // The user has named the context and confirmed the current scope, so the
        // desktop may now be read exactly once.
        let captured = match capture(db) {
            Ok(value) => value,
            Err(error) => {
                let _ = WorkspaceSessionStore::clear_operation_fence(db);
                return Err(error);
            }
        };
        if let Err(error) = Self::refuse_empty_platform_stub(&captured.snapshot) {
            let _ = WorkspaceSessionStore::clear_operation_fence(db);
            return Err(error);
        }

        // Capture success clears its Observation fence; re-fence Save for the
        // durable create boundary (crash here → Incomplete, not NeedsRefresh).
        let _ = WorkspaceSessionStore::begin_operation(
            db,
            workspace_domain::PendingOperationKind::Save,
            Some(request.workspace_id.to_string()),
        );

        let context = Self::assemble(request, &captured.snapshot);
        {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            if let Err(error) = SavedContextRepository::new(&guard).create(&context) {
                drop(guard);
                let _ = WorkspaceSessionStore::clear_operation_fence(db);
                return Err(error.into());
            }
        }
        let _ = WorkspaceSessionStore::checkpoint_current(
            db,
            actor,
            intent,
            Some(context.id.to_string()),
        );
        Ok(context)
    }

    fn require_workspace(db: &Arc<Mutex<Database>>, request: &SaveContextRequest) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        if WorkspaceService::exists(&guard, &request.workspace_id)? {
            Ok(())
        } else {
            Err(KernelError::WorkspaceNotFound)
        }
    }

    /// Empty `source=stub` captures mean the OS capturer was not live (non-Windows).
    /// Refuse rather than persisting a Moment that pretends production capture succeeded.
    fn refuse_empty_platform_stub(snapshot: &WorkspaceObservationSnapshot) -> Result<()> {
        let empty = snapshot.windows.is_empty() && snapshot.monitors.is_empty();
        if empty && snapshot.pass.source == "stub" {
            return Err(KernelError::DesktopObservationUnavailable);
        }
        Ok(())
    }

    /// Copies the approved fields out of the capture and leaves the rest behind.
    ///
    /// The saved context keeps its own copy because observation passes are a
    /// rolling buffer subject to retention, and a saved context that quietly
    /// emptied itself would misrepresent what the user agreed to keep.
    fn assemble(
        request: &SaveContextRequest,
        snapshot: &WorkspaceObservationSnapshot,
    ) -> SavedContext {
        let monitor_index_by_id = |monitor_id: Option<&String>| -> Option<i32> {
            let monitor_id = monitor_id?;
            snapshot
                .monitors
                .iter()
                .find(|monitor| &monitor.id == monitor_id)
                .map(|monitor| monitor.monitor_index)
        };

        let desktop_session_id = desktop_session_id_from_snapshot(snapshot);
        SavedContext {
            id: SavedContextId::generate(),
            workspace_id: request.workspace_id.clone(),
            name: request.name.trim().to_string(),
            created_at: Utc::now().to_rfc3339(),
            approved_scope: request.approved_scope.trim().to_string(),
            handoff_note: request.handoff_note.trim().to_string(),
            observation_pass_id: snapshot.pass.id.clone(),
            captured_at: snapshot.pass.captured_at.clone(),
            windows: snapshot
                .windows
                .iter()
                .map(|window| {
                    saved_window(
                        window,
                        monitor_index_by_id(window.monitor_id.as_ref()),
                        desktop_session_id.as_deref(),
                        &snapshot.pass.captured_at,
                    )
                })
                .collect(),
            monitors: snapshot.monitors.iter().map(saved_monitor).collect(),
        }
    }
}

fn desktop_session_id_from_snapshot(snapshot: &WorkspaceObservationSnapshot) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(&snapshot.pass.metadata_json)
        .ok()
        .and_then(|value| {
            value
                .get("desktop_session_id")
                .and_then(|entry| entry.as_str())
                .map(str::to_string)
        })
        .filter(|value| !value.trim().is_empty())
}

fn saved_window(
    window: &ObservedWindow,
    monitor_index: Option<i32>,
    desktop_session_id: Option<&str>,
    captured_at: &str,
) -> SavedContextWindow {
    let base = SavedContextWindow {
        id: Uuid::new_v4().to_string(),
        title: window.title.clone(),
        process_id: window.process_id,
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
        monitor_index,
        minimized: window.minimized,
        focused: window.focused,
        z_order: window.z_order,
        restore_identity: None,
        restore_identity_unavailable_reason: None,
    };

    match desktop_session_id {
        Some(session) if !window.hwnd.trim().is_empty() => {
            let identity = SavedContextRestoreIdentity::new(
                session,
                window.hwnd.clone(),
                window.process_id,
                &window.title,
                captured_at,
            );
            base.with_restore_identity(identity)
        }
        Some(_) => base.with_unavailable_reason("window handle missing at capture"),
        None => base.with_unavailable_reason("desktop session identity missing at capture"),
    }
}

fn saved_monitor(monitor: &ObservedMonitor) -> SavedContextMonitor {
    SavedContextMonitor {
        id: Uuid::new_v4().to_string(),
        monitor_index: monitor.monitor_index,
        name: monitor.name.clone(),
        x: monitor.x,
        y: monitor.y,
        width: monitor.width,
        height: monitor.height,
        is_primary: monitor.is_primary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;
    use workspace_database::ObservationPassRepository;
    use workspace_domain::{Workspace, WorkspaceId, SAVED_CONTEXT_SCOPE_ID};
    use workspace_windows_integration::StubDesktopCapturer;

    struct Fixture {
        kernel: WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace: Workspace,
    }

    impl Fixture {
        fn new() -> Self {
            let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
            let actor = ActorContext::local_user();
            let intent = IntentContext::user_request();
            let workspace = {
                let db = kernel.shared_database();
                let guard = db.lock().unwrap();
                WorkspaceService::create(&guard, "Product Proof".into()).unwrap()
            };
            Self {
                kernel,
                actor,
                intent,
                workspace,
            }
        }

        fn db(&self) -> Arc<Mutex<Database>> {
            self.kernel.shared_database()
        }

        fn request(&self, name: &str, scope: &str) -> SaveContextRequest {
            SaveContextRequest::new(
                self.workspace.id.clone(),
                name,
                scope,
                "Finish the client proposal outline",
            )
        }

        fn save(&self, request: &SaveContextRequest) -> Result<SavedContext> {
            SavedContextService::save_with(
                &self.db(),
                &self.actor,
                &self.intent,
                request,
                &StubDesktopCapturer::fixture_dual_monitor(),
            )
        }

        fn saved_context_count(&self) -> i64 {
            let db = self.db();
            let guard = db.lock().unwrap();
            guard
                .connection()
                .query_row("SELECT COUNT(*) FROM saved_contexts", [], |row| row.get(0))
                .unwrap()
        }

        fn observation_pass_count(&self) -> i64 {
            let db = self.db();
            let guard = db.lock().unwrap();
            guard
                .connection()
                .query_row("SELECT COUNT(*) FROM observation_passes", [], |row| {
                    row.get(0)
                })
                .unwrap()
        }
    }

    #[test]
    fn saves_the_named_context_the_user_confirmed() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();

        let saved = fixture
            .save(&fixture.request("  Tuesday review  ", SAVED_CONTEXT_SCOPE_ID))
            .unwrap();

        assert_eq!(saved.name, "Tuesday review");
        assert_eq!(saved.workspace_id, fixture.workspace.id);
        assert_eq!(saved.approved_scope, SAVED_CONTEXT_SCOPE_ID);
        assert_eq!(saved.window_count(), 4);
        assert_eq!(saved.monitor_count(), 2);
    }

    #[test]
    fn the_saved_context_is_readable_after_the_fact() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let saved = fixture
            .save(&fixture.request("Tuesday review", SAVED_CONTEXT_SCOPE_ID))
            .unwrap();

        let db = fixture.db();
        let guard = db.lock().unwrap();
        let loaded = SavedContextRepository::new(&guard)
            .get_by_id(&saved.id)
            .unwrap()
            .expect("the context was persisted");

        assert_eq!(loaded, saved);
    }

    #[test]
    fn it_keeps_its_own_copy_rather_than_pointing_at_the_observation_pass() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let saved = fixture
            .save(&fixture.request("Tuesday review", SAVED_CONTEXT_SCOPE_ID))
            .unwrap();

        // Observation passes are a rolling buffer. Take a later capture, then let
        // retention discard the one this context was built from.
        fixture
            .save(&fixture.request("A later save", SAVED_CONTEXT_SCOPE_ID))
            .unwrap();
        {
            let db = fixture.db();
            let guard = db.lock().unwrap();
            ObservationPassRepository::new(&guard)
                .purge_older_than_keep(1)
                .unwrap();
            let survivors: i64 = guard
                .connection()
                .query_row(
                    "SELECT COUNT(*) FROM observation_passes WHERE id = ?1",
                    [saved.observation_pass_id.as_str()],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(survivors, 0, "the originating pass was purged");
        }

        let db = fixture.db();
        let guard = db.lock().unwrap();
        let loaded = SavedContextRepository::new(&guard)
            .get_by_id(&saved.id)
            .unwrap()
            .expect("the context outlives the observation pass");
        assert_eq!(loaded.windows.len(), 4);
        assert_eq!(loaded.monitors.len(), 2);
        assert_eq!(loaded.observation_pass_id, saved.observation_pass_id);
    }

    #[test]
    fn it_persists_the_user_authored_handoff_unchanged() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let request = SaveContextRequest::new(
            fixture.workspace.id.clone(),
            "Tuesday review",
            SAVED_CONTEXT_SCOPE_ID,
            "  Return to the fee schedule tab and send the draft  ",
        );
        let saved = fixture.save(&request).unwrap();
        assert_eq!(
            saved.handoff_note,
            "Return to the fee schedule tab and send the draft"
        );

        let db = fixture.db();
        let guard = db.lock().unwrap();
        let loaded = SavedContextRepository::new(&guard)
            .get_by_id(&saved.id)
            .unwrap()
            .expect("persisted");
        assert_eq!(loaded.handoff_note, saved.handoff_note);
    }

    #[test]
    fn a_save_without_handoff_reads_nothing_and_stores_nothing() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let request = SaveContextRequest::new(
            fixture.workspace.id.clone(),
            "Tuesday review",
            SAVED_CONTEXT_SCOPE_ID,
            "   ",
        );
        let error = fixture.save(&request).unwrap_err();
        assert!(matches!(error, KernelError::SavedContextValidation { .. }));
        assert_eq!(fixture.observation_pass_count(), 0);
        assert_eq!(fixture.saved_context_count(), 0);
    }

    #[test]
    fn it_records_only_the_fields_the_scope_promised() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let saved = fixture
            .save(&fixture.request("Tuesday review", SAVED_CONTEXT_SCOPE_ID))
            .unwrap();

        let focused = saved
            .windows
            .iter()
            .find(|window| window.focused)
            .expect("the fixture has a focused window");
        assert!(!focused.title.is_empty());
        assert!(focused.process_id > 0);
        assert!(focused.width > 0 && focused.height > 0);
        assert!(focused.monitor_index.is_some());
        assert!(saved.monitors.iter().any(|monitor| monitor.is_primary));
        let identity = focused
            .restore_identity
            .as_ref()
            .expect("v2 save captures restore identity");
        assert!(!identity.desktop_session_id.is_empty());
        assert!(!identity.captured_hwnd.is_empty());
        assert!(!identity.title_fingerprint.is_empty());
    }

    #[test]
    fn an_unnamed_context_is_refused_without_reading_the_desktop() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();

        let error = fixture
            .save(&fixture.request("   ", SAVED_CONTEXT_SCOPE_ID))
            .unwrap_err();

        assert!(matches!(error, KernelError::SavedContextValidation { .. }));
        assert_eq!(fixture.observation_pass_count(), 0);
        assert_eq!(fixture.saved_context_count(), 0);
    }

    #[test]
    fn a_save_without_confirmed_scope_reads_nothing_and_stores_nothing() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();

        let error = fixture
            .save(&fixture.request("Tuesday review", ""))
            .unwrap_err();

        assert!(matches!(error, KernelError::SavedContextValidation { .. }));
        assert_eq!(fixture.observation_pass_count(), 0);
        assert_eq!(fixture.saved_context_count(), 0);
    }

    #[test]
    fn consent_for_a_different_scope_reads_nothing_and_stores_nothing() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();

        let error = fixture
            .save(&fixture.request("Tuesday review", "saved-context-scope-v0"))
            .unwrap_err();

        assert!(matches!(error, KernelError::SavedContextValidation { .. }));
        assert_eq!(fixture.observation_pass_count(), 0);
        assert_eq!(fixture.saved_context_count(), 0);
    }

    #[test]
    fn an_unknown_workspace_is_refused_without_reading_the_desktop() {
        let _flight = crate::services::lock_observation_flight_for_tests();
        let fixture = Fixture::new();
        let request = SaveContextRequest::new(
            WorkspaceId::new("ws-does-not-exist").unwrap(),
            "Tuesday review",
            SAVED_CONTEXT_SCOPE_ID,
            "Finish the client proposal outline",
        );

        let error = fixture.save(&request).unwrap_err();

        assert!(matches!(error, KernelError::WorkspaceNotFound));
        assert_eq!(fixture.observation_pass_count(), 0);
        assert_eq!(fixture.saved_context_count(), 0);
    }
}
