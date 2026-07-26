//! Workspace Observation Layer command tests (Sprint 105).

use crate::commands::pipeline::CommandPipeline;
use crate::commands::{
    CaptureWorkspaceObservation, GateObservationRead, GetLatestWorkspaceObservation,
    GetWorkspaceObservationById,
};
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{DesktopWindowService, WorkspaceObservationCaptureResult, WorkspaceObservationService};
use crate::WorkspaceKernel;
use workspace_domain::{Actor, ActorContext, IntentContext};
use workspace_windows_integration::{StubDesktopCapturer, StubWindowEnumerator};

fn capture_with_stub(
    kernel: &WorkspaceKernel,
    local: &ActorContext,
    intent: &IntentContext,
) -> Result<WorkspaceObservationCaptureResult, KernelError> {
    CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_query(GateObservationRead)?;
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

    let windows = DesktopWindowService::list_recent(&kernel.shared_database(), Some(10)).unwrap();
    assert_eq!(windows.len(), 3);
    assert!(windows.iter().all(|window| window.visible));
}

#[test]
fn legacy_enumerator_path_still_works_without_snapshot() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let windows = DesktopWindowService::list_with(
        &kernel.shared_database(),
        &StubWindowEnumerator,
        Some(10),
    )
    .unwrap();
    assert!(windows.is_empty());
}

#[test]
fn observation_authority_guard_fails_execution() {
    match CommandHandler::workspace_observation_attempt_execute() {
        Err(KernelError::Config(message)) => assert!(message.contains("cannot execute")),
        other => panic!("expected config guard failure, got {other:?}"),
    }
}
