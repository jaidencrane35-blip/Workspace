//! Workspace Observation Layer command tests (Sprint 105).

use crate::commands::pipeline::CommandPipeline;
use crate::commands::{
    CaptureWorkspaceObservation, GateObservationRead, GetLatestObservationDelta,
    GetLatestWorkspaceObservation, GetWorkspaceObservationById, GetWorkspaceObservationStatus,
};
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{DesktopWindowService, WorkspaceObservationCaptureResult, WorkspaceObservationService};
use crate::WorkspaceKernel;
use chrono::{Duration, Utc};
use workspace_database::ObservationPassRepository;
use workspace_domain::{Actor, ActorContext, IntentContext, ObservationFreshness};
use workspace_windows_integration::{
    DesktopCapturer, DesktopObservationCapture, StubDesktopCapturer, WindowsIntegrationError,
};

struct FailingDesktopCapturer;

impl DesktopCapturer for FailingDesktopCapturer {
    fn capture_desktop(&self) -> workspace_windows_integration::Result<DesktopObservationCapture> {
        Err(WindowsIntegrationError::EnumerationFailed(
            "forced capture failure".into(),
        ))
    }
}

fn capture_with_stub(
    kernel: &WorkspaceKernel,
    local: &ActorContext,
    intent: &IntentContext,
) -> Result<WorkspaceObservationCaptureResult, KernelError> {
    CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_query(GateObservationRead)?;
    // Fixture seeding hits the observation service directly; production capture
    // commands route exclusively through CaptureCoordinator.
    WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        local,
        intent,
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
}

#[test]
fn capture_creates_snapshot_with_windows_and_monitors() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let result = capture_with_stub(&kernel, &local, &intent).unwrap();

    assert!(!result.snapshot_id.is_empty());
    assert_eq!(result.window_count, 4);
    assert_eq!(result.monitor_count, 2);
    assert!(result.identity_count >= 4);
}

#[test]
fn latest_snapshot_loads_after_capture() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();

    let latest = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetLatestWorkspaceObservation)
        .unwrap()
        .expect("latest snapshot");

    assert_eq!(latest.pass.id, captured.snapshot_id);
    assert_eq!(latest.windows.len(), 4);
    assert_eq!(latest.monitors.len(), 2);
}

#[test]
fn snapshot_lookup_by_id_works() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();

    let loaded = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetWorkspaceObservationById::new(captured.snapshot_id.clone()))
        .unwrap()
        .expect("snapshot by id");

    assert_eq!(loaded.pass.id, captured.snapshot_id);
}

#[test]
fn foreground_focus_persists_through_pipeline() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();
    let focused = captured
        .snapshot
        .windows
        .iter()
        .find(|window| window.focused)
        .expect("focused window");
    assert_eq!(
        captured.snapshot.pass.foreground_hwnd.as_deref(),
        Some(focused.hwnd.as_str())
    );
}

#[test]
fn identity_matching_high_and_low_confidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = capture_with_stub(&kernel, &local, &intent).unwrap();
    assert!(first.snapshot.identities.iter().any(|identity| {
        identity.confidence.as_str() == "high" || identity.confidence.as_str() == "ephemeral"
    }));

    let mut second_fixture = workspace_windows_integration::dual_monitor_fixture();
    second_fixture.windows[0].title = "Fixture Focus Renamed".into();
    let second = WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &StubDesktopCapturer::new(second_fixture),
    )
    .unwrap();
    assert!(second.snapshot.identities.iter().any(|identity| {
        identity.confidence.as_str() == "medium" || identity.confidence.as_str() == "low"
    }));
}

#[test]
fn same_process_windows_get_distinct_identities() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let mut fixture = workspace_windows_integration::DesktopObservationCapture::empty_stub();
    fixture.metadata.source = "test".into();
    fixture.foreground_hwnd = Some("0xA".into());
    fixture.monitors = workspace_windows_integration::dual_monitor_fixture().monitors;
    fixture.windows = vec![
        workspace_windows_integration::CapturedDesktopWindow {
            hwnd: "0xA".into(),
            title: "Google".into(),
            process_id: 5000,
            visible: true,
            minimized: false,
            focused: true,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            monitor_index: Some(0),
            z_order: Some(0),
        },
        workspace_windows_integration::CapturedDesktopWindow {
            hwnd: "0xB".into(),
            title: "Settings".into(),
            process_id: 5000,
            visible: true,
            minimized: false,
            focused: false,
            x: 100,
            y: 100,
            width: 400,
            height: 300,
            monitor_index: Some(0),
            z_order: Some(1),
        },
    ];
    let result = WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &StubDesktopCapturer::new(fixture),
    )
    .unwrap();
    assert_eq!(result.identity_count, 2);
    assert_ne!(
        result.snapshot.windows[0].stable_window_id,
        result.snapshot.windows[1].stable_window_id
    );
}

#[test]
fn historical_observation_omits_live_identity_mutations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = capture_with_stub(&kernel, &local, &intent).unwrap();
    let first_id = first.snapshot_id.clone();

    let mut second = workspace_windows_integration::dual_monitor_fixture();
    second.windows[0].title = "Fixture Focus Totally Changed".into();
    WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &StubDesktopCapturer::new(second),
    )
    .unwrap();

    let historical = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetWorkspaceObservationById::new(first_id))
        .unwrap()
        .expect("historical");
    assert!(historical.identities.is_empty());
    assert!(!historical.windows[0]
        .stable_window_id
        .as_deref()
        .unwrap_or("")
        .is_empty());
}

#[test]
fn new_window_creates_identity_on_capture() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let mut fixture = workspace_windows_integration::dual_monitor_fixture();
    fixture.windows.push(workspace_windows_integration::CapturedDesktopWindow {
        hwnd: "0x0000000000000NEW".into(),
        title: "Brand New Window".into(),
        process_id: 9999,
        visible: true,
        minimized: false,
        focused: false,
        x: 10,
        y: 10,
        width: 400,
        height: 300,
        monitor_index: Some(0),
        z_order: Some(99),
    });
    let result = WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &StubDesktopCapturer::new(fixture),
    )
    .unwrap();
    assert!(result.snapshot.identities.iter().any(|identity| {
        identity.process_id == 9999 && identity.confidence.as_str() == "ephemeral"
    }));
}

#[test]
fn governance_blocks_unauthorized_actor() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ai = ActorContext::new(Actor::ai_assistant("ai-observation").unwrap());
    let ctx = kernel.command_context(ai, IntentContext::user_request());
    let result = CommandPipeline::new(ctx).execute_query(CaptureWorkspaceObservation);
    match result {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected permission failure, got {other:?}"),
    }
}

#[test]
fn legacy_desktop_window_service_reads_latest_snapshot() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    capture_with_stub(&kernel, &local, &intent).unwrap();

    // Legacy IPC adapter over WorkspaceState (Sprint 121).
    let windows = DesktopWindowService::list_recent(&kernel.shared_database(), Some(10)).unwrap();
    assert_eq!(windows.len(), 4);
    assert!(windows.iter().any(|window| window.focused));
    assert!(windows.iter().any(|window| window.minimized));
}

#[test]
fn list_recent_returns_empty_without_snapshot() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    // Legacy IPC adapter over WorkspaceState (Sprint 121).
    let windows = DesktopWindowService::list_recent(&kernel.shared_database(), Some(10)).unwrap();
    assert!(windows.is_empty());
}

#[test]
fn single_snapshot_propagates_to_environment_and_intelligence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(crate::commands::create_workspace::CreateWorkspace::new(
            "Observation WS".into(),
        ))
        .unwrap();
    let ws = workspace.id.to_string();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();
    let focused_hwnd = captured
        .snapshot
        .windows
        .iter()
        .find(|window| window.focused)
        .map(|window| window.hwnd.clone())
        .expect("focused hwnd");

    let environment = CommandHandler::generate_workspace_environment(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(
        environment
            .windows
            .iter()
            .find(|window| window.state == workspace_domain::EnvironmentWindowState::Focused)
            .map(|window| window.hwnd.clone())
            .as_deref(),
        Some(focused_hwnd.as_str())
    );

    let intelligence = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(
        intelligence.environment.focused_window_title.as_deref(),
        Some("Fixture Focus")
    );
}

#[test]
fn observation_authority_guard_fails_execution() {
    match CommandHandler::workspace_observation_attempt_execute() {
        Err(KernelError::Config(message)) => assert!(message.contains("cannot execute")),
        other => panic!("expected config guard failure, got {other:?}"),
    }
}

#[test]
fn status_reports_unavailable_without_observation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let status = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetWorkspaceObservationStatus)
        .unwrap();
    assert!(!status.has_observation);
    assert_eq!(status.freshness, ObservationFreshness::Unavailable);
    assert!(status.pass_id.is_none());
    assert!(status.last_failure.is_none());
}

#[test]
fn status_reports_recent_observation_metadata() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();
    let status = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetWorkspaceObservationStatus)
        .unwrap();
    assert!(status.has_observation);
    assert_eq!(status.pass_id.as_deref(), Some(captured.snapshot_id.as_str()));
    assert_eq!(status.window_count, Some(4));
    assert_eq!(status.monitor_count, Some(2));
    assert_eq!(status.identity_count, Some(4));
    assert!(status.age_seconds.is_some());
    assert!(matches!(
        status.freshness,
        ObservationFreshness::Fresh | ObservationFreshness::Recent
    ));
}

#[test]
fn status_reports_stale_observation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let captured = capture_with_stub(&kernel, &local, &intent).unwrap();
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        let stale_at = (Utc::now() - Duration::seconds(600)).to_rfc3339();
        guard
            .connection()
            .execute(
                "UPDATE observation_passes SET captured_at = ?1 WHERE id = ?2",
                (&stale_at, &captured.snapshot_id),
            )
            .unwrap();
    }
    let status = WorkspaceObservationService::get_status_at(
        &kernel.shared_database(),
        &local,
        &intent,
        Utc::now(),
    )
    .unwrap();
    assert_eq!(status.freshness, ObservationFreshness::Stale);
    assert!(status.age_seconds.unwrap() >= 600);
}

#[test]
fn status_surfaces_capture_failure() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let err = WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &FailingDesktopCapturer,
    )
    .unwrap_err();
    assert!(matches!(err, KernelError::WindowsIntegration { .. }));

    let status = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetWorkspaceObservationStatus)
        .unwrap();
    let failure = status.last_failure.expect("failure");
    assert_eq!(failure.error_class.as_str(), "windows_integration");
    assert!(failure.message.contains("forced capture failure"));
}

#[test]
fn status_requires_desktop_read() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ai = ActorContext::new(Actor::ai_assistant("ai-status").unwrap());
    let ctx = kernel.command_context(ai, IntentContext::user_request());
    let result = CommandPipeline::new(ctx).execute_query(GetWorkspaceObservationStatus);
    match result {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected permission failure, got {other:?}"),
    }
}

#[test]
fn scheduler_status_reports_disabled_in_memory_kernel() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let status = CommandHandler::get_observation_scheduler_status(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert!(!status.enabled);
    assert!(!status.running);
    assert_eq!(status.ticks_emitted, 0);
    assert_eq!(status.authority_effect, "none");
}

#[test]
fn latest_observation_delta_empty_with_zero_or_one_snapshot() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let empty = CommandHandler::get_latest_observation_delta(&kernel, local.clone(), intent.clone())
        .unwrap();
    assert!(!empty.has_changes);
    assert!(empty.current_pass_id.is_none());

    capture_with_stub(&kernel, &local, &intent).unwrap();
    let first = CommandHandler::get_latest_observation_delta(&kernel, local.clone(), intent.clone())
        .unwrap();
    assert!(!first.has_changes);
    assert!(first.current_pass_id.is_some());
    assert!(first.previous_pass_id.is_none());

    // Pipeline path
    let via_cmd = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_query(GetLatestObservationDelta)
        .unwrap();
    assert_eq!(via_cmd.current_pass_id, first.current_pass_id);
}

#[test]
fn status_query_uses_metadata_not_full_snapshot_collections() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    capture_with_stub(&kernel, &local, &intent).unwrap();
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    let metadata = ObservationPassRepository::new(&guard)
        .get_latest_metadata()
        .unwrap()
        .expect("metadata");
    // Metadata carries counts only — callers must not expect child collections here.
    assert_eq!(metadata.window_count, 4);
    assert_eq!(metadata.monitor_count, 2);
    assert_eq!(metadata.identity_count, 4);
    assert!(!metadata.id.is_empty());
}
