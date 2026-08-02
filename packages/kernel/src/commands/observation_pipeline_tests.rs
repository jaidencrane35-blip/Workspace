//! End-to-end observation pipeline: capture → persist → reload → restore plan.
//!
//! Uses StubDesktopCapturer fixtures where live Win32 is unavailable.

use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{
    ActorContext, CapabilitySet, IntentContext, ProjectedDisposition, RestoreCompatibilitySummary,
    SaveContextRequest, SAVED_CONTEXT_SCOPE_ID,
};
use workspace_windows_integration::{StubDesktopCapturer, StubWindowMutator};

use crate::error::KernelError;
use crate::services::{
    action_request_from_saved_context, DesktopActionService, SavedContextService, WorkspaceService,
};
use crate::services::observation_flight_test_lock;
use crate::WorkspaceKernel;

fn caps() -> CapabilitySet {
    CapabilitySet::local_user_standard()
}

struct PipelineFixture {
    kernel: WorkspaceKernel,
    actor: ActorContext,
    intent: IntentContext,
    workspace_id: workspace_domain::WorkspaceId,
}

impl PipelineFixture {
    fn new() -> Self {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace_id = {
            let db = kernel.shared_database();
            let guard = db.lock().unwrap();
            WorkspaceService::create(&guard, "Observation Pipeline".into())
                .unwrap()
                .id
        };
        Self {
            kernel,
            actor,
            intent,
            workspace_id,
        }
    }

    fn db(&self) -> Arc<Mutex<Database>> {
        self.kernel.shared_database()
    }

    fn save_fixture(&self, name: &str) -> workspace_domain::SavedContext {
        SavedContextService::save_with(
            &self.db(),
            &self.actor,
            &self.intent,
            &SaveContextRequest::new(
                self.workspace_id.clone(),
                name,
                SAVED_CONTEXT_SCOPE_ID,
                "Return to the proposal outline",
            ),
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap()
    }
}

#[test]
fn capture_persist_reload_preserves_geometry_and_restore_identity() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let fixture = PipelineFixture::new();

    let saved = fixture.save_fixture("Tuesday review");
    assert_eq!(saved.window_count(), 4);
    assert_eq!(saved.monitor_count(), 2);
    assert!(saved.windows.iter().all(|w| w.restore_identity.is_some()));
    assert!(saved.windows.iter().any(|w| w.focused));
    assert!(saved.monitors.iter().any(|m| m.is_primary));

    let listed = SavedContextService::list_for_workspace(&fixture.db(), &fixture.workspace_id)
        .unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, saved.id);

    let loaded = SavedContextService::get_by_id(&fixture.db(), &saved.id)
        .unwrap()
        .expect("persisted");
    assert_eq!(loaded, saved);
    let focused = loaded
        .windows
        .iter()
        .find(|w| w.focused)
        .expect("focused window");
    let identity = focused.restore_identity.as_ref().unwrap();
    assert_eq!(identity.captured_hwnd, "0x00000000000000AA");
    assert_eq!(identity.desktop_session_id, "stub-desktop-session-1");
}

#[test]
fn restore_planning_scores_exact_session_matches_as_eligible() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let fixture = PipelineFixture::new();
    let saved = fixture.save_fixture("Tuesday review");

    let request = action_request_from_saved_context(&saved);
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let summary = RestoreCompatibilitySummary::from_plan(&plan);

    assert!(summary.restore_eligible);
    assert!(summary.will_attempt > 0);
    assert!(matches!(
        summary.confidence_band.as_str(),
        "high" | "steady" | "limited"
    ));
    assert!(plan.items.iter().any(|item| {
        item.action_type == "window.place"
            && item.projected_disposition == ProjectedDisposition::WillAttempt
    }));
}

#[test]
fn missing_window_is_unresolvable_not_relaunched() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let fixture = PipelineFixture::new();
    let mut saved = fixture.save_fixture("Tuesday review");

    // Close the focused window in the live mutator view by pointing identity at a gone hwnd.
    if let Some(window) = saved.windows.iter_mut().find(|w| w.focused) {
        if let Some(identity) = window.restore_identity.as_mut() {
            identity.captured_hwnd = "0xDEADBEEFDEADBEEF".into();
        }
    }

    let request = action_request_from_saved_context(&saved);
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let summary = RestoreCompatibilitySummary::from_plan(&plan);

    assert!(plan.items.iter().any(|item| {
        item.projected_disposition == ProjectedDisposition::WillSkipUnresolvable
            && item.error_code.as_deref() == Some("ACTION_TARGET_NOT_FOUND")
    }));
    assert!(summary.missing_window_count >= 1);
    assert!(!plan
        .items
        .iter()
        .any(|item| item.action_type == "application.launch"));
}

#[test]
fn empty_platform_stub_save_is_refused() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let fixture = PipelineFixture::new();

    let error = SavedContextService::save_with(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        &SaveContextRequest::new(
            fixture.workspace_id.clone(),
            "Should fail",
            SAVED_CONTEXT_SCOPE_ID,
            "Return to the proposal outline",
        ),
        &StubDesktopCapturer::empty(),
    )
    .unwrap_err();

    assert!(matches!(error, KernelError::DesktopObservationUnavailable));
    assert!(
        SavedContextService::list_for_workspace(&fixture.db(), &fixture.workspace_id)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn observation_pass_carries_process_metadata_from_fixture() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let fixture = PipelineFixture::new();

    // Enrich fixture with process_name before save so observation buffer records it.
    let mut capture = workspace_windows_integration::dual_monitor_fixture();
    for (index, window) in capture.windows.iter_mut().enumerate() {
        window.process_name = Some(format!("fixture-{index}.exe"));
    }
    let capturer = StubDesktopCapturer::new(capture);

    let saved = SavedContextService::save_with(
        &fixture.db(),
        &fixture.actor,
        &fixture.intent,
        &SaveContextRequest::new(
            fixture.workspace_id.clone(),
            "With process metadata",
            SAVED_CONTEXT_SCOPE_ID,
            "Return to the proposal outline",
        ),
        &capturer,
    )
    .unwrap();

    // SavedContext scope excludes program names — only process_id remains.
    assert!(saved.windows.iter().all(|w| w.process_id > 0));

    let snapshot = crate::services::WorkspaceObservationService::get_latest_snapshot(&fixture.db())
        .unwrap()
        .expect("observation snapshot");
    assert!(
        snapshot
            .windows
            .iter()
            .any(|w| w.process_name.as_deref() == Some("fixture-0.exe")),
        "ephemeral observation retains process_name; SavedContext does not"
    );
}
