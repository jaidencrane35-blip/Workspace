//! Kernel + IPC pathway tests for DAF-1d desktop arrangement restore.

use std::sync::Mutex;

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::desktop_arrangement::{
    CaptureDesktopArrangement, GetDesktopArrangement, ListDesktopArrangements,
    RestoreDesktopArrangement,
};
use crate::commands::pipeline::CommandPipeline;
use crate::error::KernelError;
use crate::services::{
    AuditService, DesktopArrangementService, WorkspaceObservationService,
};
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ActorType, Capability, CapabilitySet, DesktopArrangementApplyStatus,
    DesktopArrangementId, IntentContext, WorkspaceId,
};
use workspace_windows_integration::{
    FocusWindowRequest, RecordedWindowControl, SetWindowBoundsRequest, StubDesktopCapturer,
    StubWindowController, WindowController, WindowsIntegrationError,
};

fn seed_observation(kernel: &WorkspaceKernel) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &local,
        &intent,
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .expect("observation fixture");
}

fn create_workspace(kernel: &WorkspaceKernel) -> WorkspaceId {
    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CreateWorkspace::new("Arrangement WS".into()))
    .unwrap()
    .id
}

struct FailAfterFirstController {
    inner: StubWindowController,
    calls: Mutex<usize>,
}

impl WindowController for FailAfterFirstController {
    fn set_bounds(
        &self,
        request: &SetWindowBoundsRequest,
    ) -> workspace_windows_integration::Result<workspace_windows_integration::WindowControlOutcome>
    {
        let mut calls = self.calls.lock().unwrap();
        *calls += 1;
        if *calls > 1 {
            return Err(WindowsIntegrationError::InvalidWindowHandle(
                "injected failure".into(),
            ));
        }
        self.inner.set_bounds(request)
    }

    fn focus(
        &self,
        request: &FocusWindowRequest,
    ) -> workspace_windows_integration::Result<workspace_windows_integration::WindowControlOutcome>
    {
        self.inner.focus(request)
    }
}

#[test]
fn capture_and_get_arrangement_through_pipeline() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id.clone(),
        "Focus".into(),
        "coding".into(),
        Some(DesktopArrangementId::new("arr-focus-1").unwrap()),
        false,
    ))
    .unwrap();

    assert_eq!(arrangement.name, "Focus");
    assert!(!arrangement.entries.is_empty());
    assert!(arrangement.entries.iter().all(|e| e.stored_bounds().is_some()));

    let loaded = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_query(GetDesktopArrangement::new(arrangement.id.clone()))
    .unwrap()
    .expect("loaded");
    assert_eq!(loaded.entries.len(), arrangement.entries.len());

    let listed = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_query(ListDesktopArrangements::new(workspace_id, Some(10)))
    .unwrap();
    assert_eq!(listed.len(), 1);
}

#[test]
fn allowed_restore_uses_window_controller_boundary() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-restore-ok").unwrap()),
        false,
    ))
    .unwrap();

    let controller = StubWindowController::new();
    let result = DesktopArrangementService::restore_with(
        &kernel.shared_database(),
        &arrangement.id,
        true,
        &controller,
    )
    .unwrap();

    assert!(result.applied_count > 0);
    assert_eq!(result.failed_count, 0);
    assert!(!controller.recorded().is_empty());
    assert!(controller
        .recorded()
        .iter()
        .any(|op| matches!(op, RecordedWindowControl::SetBounds(_))));
}

#[test]
fn permission_denied_restore_does_not_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-deny").unwrap()),
        false,
    ))
    .unwrap();

    let mut ctx = kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    );
    ctx.capability_set = CapabilitySet::new();
    let error = CommandPipeline::new(ctx)
        .execute_mutation(RestoreDesktopArrangement::simulated(
            arrangement.id.clone(),
            true,
        ))
        .unwrap_err();

    assert!(matches!(error, KernelError::PermissionDenied(_)));

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "permission.denied"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
    }));
    assert!(!records.iter().any(|r| {
        r.event_type == "command.executed"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
    }));
}

#[test]
fn allowed_restore_pathway_audits_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-allow").unwrap()),
        false,
    ))
    .unwrap();

    let result = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(RestoreDesktopArrangement::simulated(arrangement.id, true))
    .unwrap();

    assert!(result.applied_count > 0);

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "permission.allowed"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
    }));
    assert!(records.iter().any(|r| {
        r.event_type == "command.executed"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
            && r.success
    }));
}

#[test]
fn restore_halts_after_failure_without_automatic_repair() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-halt").unwrap()),
        false,
    ))
    .unwrap();

    assert!(
        arrangement.entries.len() >= 2,
        "fixture must provide multiple windows"
    );

    let controller = FailAfterFirstController {
        inner: StubWindowController::new(),
        calls: Mutex::new(0),
    };
    let result = DesktopArrangementService::restore_with(
        &kernel.shared_database(),
        &arrangement.id,
        false,
        &controller,
    )
    .unwrap();

    assert_eq!(result.applied_count, 1);
    assert_eq!(result.failed_count, 1);
    assert!(result
        .outcomes
        .iter()
        .any(|o| o.status == DesktopArrangementApplyStatus::Skipped));
}

#[test]
fn missing_windows_remain_visible_in_diagnostics() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-gap").unwrap()),
        false,
    ))
    .unwrap();

    let mut fixture = workspace_windows_integration::dual_monitor_fixture();
    fixture.windows.truncate(1);
    WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        &ActorContext::local_user(),
        &IntentContext::user_request(),
        &StubDesktopCapturer::new(fixture),
    )
    .unwrap();

    let result = DesktopArrangementService::restore_simulated(
        &kernel.shared_database(),
        &arrangement.id,
        true,
    )
    .unwrap();

    assert!(result.gap_count > 0);
    assert!(result.diagnostics.iter().any(|d| d.restore_gap));
    assert!(result.applied_count < arrangement.entries.len());
}

#[test]
fn ai_assistant_cannot_restore_without_approval() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let arrangement = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(CaptureDesktopArrangement::new(
        workspace_id,
        "Focus".into(),
        String::new(),
        Some(DesktopArrangementId::new("arr-ai").unwrap()),
        false,
    ))
    .unwrap();

    let mut ctx = kernel.command_context(
        ActorContext::new(Actor::ai_assistant("ai-1").unwrap()),
        IntentContext::user_request(),
    );
    ctx.capability_set = CapabilitySet::local_user_standard();
    let error = CommandPipeline::new(ctx)
        .execute_mutation(RestoreDesktopArrangement::simulated(arrangement.id, true))
        .unwrap_err();

    assert!(matches!(error, KernelError::ApprovalRequired { .. }));

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "permission.approval_required"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
            && r.actor_type == ActorType::AIAssistant
    }));
    assert!(!records.iter().any(|r| {
        r.event_type == "command.executed"
            && r.command_name.as_deref() == Some("RestoreDesktopArrangement")
    }));
}

#[test]
fn no_assistant_execution_path_on_service() {
    assert!(matches!(
        DesktopArrangementService::attempt_assistant_restore(),
        Err(KernelError::IntegrityViolation { .. })
    ));
}

#[test]
fn capture_requires_desktop_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_observation(&kernel);
    let workspace_id = create_workspace(&kernel);

    let mut ctx = kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    );
    ctx.capability_set = CapabilitySet::new().with_capability(&Capability::desktop_read());
    let error = CommandPipeline::new(ctx)
        .execute_mutation(CaptureDesktopArrangement::new(
            workspace_id,
            "Focus".into(),
            String::new(),
            None,
            false,
        ))
        .unwrap_err();

    assert!(matches!(error, KernelError::PermissionDenied(_)));
}
