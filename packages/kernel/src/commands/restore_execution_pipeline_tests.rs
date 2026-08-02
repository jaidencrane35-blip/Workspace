//! Restore execution pipeline: plan → RestoreExecutor → ActionOperationResult.

use workspace_domain::{
    ActionRequest, ActionTargetDescriptor, CapabilitySet, ItemDisposition, OperationOutcome,
    ProjectedDisposition, ProposedEffect, RestoreExecutionSummary, SavedContext,
    SavedContextId, SavedContextMonitor, SavedContextRestoreIdentity, SavedContextWindow,
    WorkspaceId, ACTION_TYPE_WINDOW_FOCUS, ACTION_TYPE_WINDOW_PLACE, SAVED_CONTEXT_SCOPE_ID,
};
use workspace_windows_integration::{StubWindowMutator, WindowMutator, WindowPlacementRequest};

use crate::commands::resume::resolve_and_execute_for_tests;
use crate::services::{
    action_request_from_saved_context, ActionExecutionControls, DesktopActionService,
    RestoreExecutor,
};

fn caps() -> CapabilitySet {
    CapabilitySet::local_user_standard()
}

fn sample_context() -> SavedContext {
    let identity = SavedContextRestoreIdentity::new(
        "stub-desktop-session-1",
        "0x00000000000000AA",
        100,
        "Fixture Focus",
        "2026-08-01T10:00:00Z",
    );
    SavedContext {
        id: SavedContextId::new("sc-exec-1").unwrap(),
        workspace_id: WorkspaceId::new("ws-exec").unwrap(),
        name: "Tuesday review".into(),
        created_at: "2026-08-01T10:00:00Z".into(),
        approved_scope: SAVED_CONTEXT_SCOPE_ID.into(),
        handoff_note: "Finish the client proposal outline".into(),
        observation_pass_id: "pass-1".into(),
        captured_at: "2026-08-01T10:00:00Z".into(),
        windows: vec![SavedContextWindow {
            id: "w-focus".into(),
            title: "Fixture Focus".into(),
            process_id: 100,
            x: 10,
            y: 20,
            width: 800,
            height: 600,
            monitor_index: Some(0),
            minimized: false,
            focused: true,
            z_order: Some(0),
            restore_identity: Some(identity),
            restore_identity_unavailable_reason: None,
        }],
        monitors: vec![SavedContextMonitor {
            id: "m-1".into(),
            monitor_index: 0,
            name: "Primary".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            is_primary: true,
        }],
    }
}

#[test]
fn exact_window_match_restores_place_and_focus() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let (plan, result) =
        resolve_and_execute_for_tests(&sample_context(), &caps(), &mutator, &Default::default())
            .unwrap();

    assert!(plan.items.iter().any(|i| {
        i.action_type == ACTION_TYPE_WINDOW_PLACE
            && i.projected_disposition == ProjectedDisposition::WillAttempt
    }));
    // z-order items are skip-unsupported → overall PartiallyCompleted is expected.
    assert!(matches!(
        result.outcome,
        OperationOutcome::Completed | OperationOutcome::PartiallyCompleted
    ));
    assert!(result.summary.restored_windows >= 2);
    assert_eq!(result.summary.failed_operations, 0);
    assert!(result.summary.skipped_windows >= 1);
    assert_eq!(
        mutator.window_placement("0x00000000000000AA"),
        Some((10, 20, 800, 600, false))
    );
    assert_eq!(
        mutator.focused_hwnd().as_deref(),
        Some("0x00000000000000AA")
    );
}

#[test]
fn partial_match_keeps_completed_and_skipped() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut context = sample_context();
    let second_identity = SavedContextRestoreIdentity::new(
        "stub-desktop-session-1",
        "0xDEADBEEFDEADBEEF",
        999,
        "Gone Window",
        "2026-08-01T10:00:00Z",
    );
    context.windows.push(SavedContextWindow {
        id: "w-gone".into(),
        title: "Gone Window".into(),
        process_id: 999,
        x: 40,
        y: 40,
        width: 400,
        height: 300,
        monitor_index: Some(0),
        minimized: false,
        focused: false,
        z_order: Some(1),
        restore_identity: Some(second_identity),
        restore_identity_unavailable_reason: None,
    });

    let (_, result) =
        resolve_and_execute_for_tests(&context, &caps(), &mutator, &Default::default()).unwrap();

    assert_eq!(result.outcome, OperationOutcome::PartiallyCompleted);
    assert!(result.summary.restored_windows > 0);
    assert!(result.summary.skipped_windows > 0);
    assert!(result.summary.missing_applications >= 1);
    assert!(result.items.iter().any(|i| {
        i.disposition == ItemDisposition::Completed
    }));
    assert!(result.items.iter().any(|i| {
        i.disposition == ItemDisposition::SkippedUnresolvable
            && i.error_code.as_deref() == Some("ACTION_TARGET_NOT_FOUND")
    }));
}

#[test]
fn missing_process_at_execute_is_safe_refusal() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let request = action_request_from_saved_context(&sample_context());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let proofs = DesktopActionService::proofs_for_plan(&plan, "local-user");

    // Identity still matches at resolve; process id changes before execute.
    mutator.set_window_process_id("0x00000000000000AA", 4242);

    let result = RestoreExecutor::execute(
        &plan,
        &proofs,
        &caps(),
        &mutator,
        &ActionExecutionControls::default(),
    )
    .unwrap();

    assert!(result.items.iter().any(|i| {
        i.disposition == ItemDisposition::RefusedChanged
            && i.error_code.as_deref() == Some("ACTION_PLAN_ITEM_CHANGED")
    }));
    assert!(result.summary.failed_operations >= 1);
    assert_ne!(result.outcome, OperationOutcome::Completed);
}

#[test]
fn skipped_restore_never_relaunches() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.remove_window("0x00000000000000AA");
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context(), &caps(), &mutator, &Default::default())
            .unwrap();

    assert!(result.summary.missing_applications >= 1);
    assert_eq!(result.summary.restored_windows, 0);
    assert!(!result
        .items
        .iter()
        .any(|i| i.action_type == "application.launch"));
    assert!(result
        .items
        .iter()
        .all(|i| i.disposition != ItemDisposition::Completed));
}

#[test]
fn already_open_workspace_is_idempotent() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let context = sample_context();
    let (_, first) =
        resolve_and_execute_for_tests(&context, &caps(), &mutator, &Default::default()).unwrap();
    assert!(first.summary.restored_windows >= 2);

    let (_, second) =
        resolve_and_execute_for_tests(&context, &caps(), &mutator, &Default::default()).unwrap();
    assert!(matches!(
        second.outcome,
        OperationOutcome::Completed | OperationOutcome::PartiallyCompleted
    ));
    assert_eq!(
        mutator.window_placement("0x00000000000000AA"),
        Some((10, 20, 800, 600, false))
    );
    assert!(second.summary.restored_windows >= 2);
}

#[test]
fn minimized_window_restore_clears_minimized_flag() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator
        .place_window(
            "0x00000000000000AA",
            &WindowPlacementRequest {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                minimized: true,
            },
        )
        .unwrap();
    assert_eq!(
        mutator.window_placement("0x00000000000000AA").map(|p| p.4),
        Some(true)
    );

    let (_, result) =
        resolve_and_execute_for_tests(&sample_context(), &caps(), &mutator, &Default::default())
            .unwrap();

    assert!(result.summary.restored_windows >= 1);
    assert_eq!(
        mutator.window_placement("0x00000000000000AA"),
        Some((10, 20, 800, 600, false))
    );
}

#[test]
fn execution_summary_and_timing_are_populated() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context(), &caps(), &mutator, &Default::default())
            .unwrap();

    let derived = RestoreExecutionSummary::from_items(&result.items, result.summary.duration_ms);
    assert_eq!(result.summary.restored_windows, derived.restored_windows);
    assert_eq!(result.summary.skipped_windows, derived.skipped_windows);
    assert_eq!(
        result.summary.missing_applications,
        derived.missing_applications
    );
    assert_eq!(result.summary.failed_operations, derived.failed_operations);
    // Fixture execute is fast; duration is always recorded (may be 0ms on coarse clocks).
    assert!(result.summary.duration_ms < 60_000);
}

#[test]
fn safe_failure_on_environment_refuse_is_partial() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.refuse_place();
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context(), &caps(), &mutator, &Default::default())
            .unwrap();

    assert!(result.summary.failed_operations >= 1);
    assert!(result.items.iter().any(|i| {
        i.disposition == ItemDisposition::Failed
            && i.error_code.as_deref() == Some("ACTION_TARGET_REFUSED_BY_ENVIRONMENT")
    }));
    // Focus may still succeed — partial success retained.
    assert!(
        matches!(
            result.outcome,
            OperationOutcome::PartiallyCompleted
                | OperationOutcome::Failed
                | OperationOutcome::Indeterminate
        ),
        "outcome={:?}",
        result.outcome
    );
}

#[test]
fn unsupported_effect_is_skipped_not_forced() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let request = ActionRequest {
        purpose: "probe".into(),
        items: vec![ActionTargetDescriptor {
            item_id: "z".into(),
            action_type: "window.z_order".into(),
            target_summary: "Z".into(),
            restore_identity: None,
            identity_unavailable_reason: Some("unsupported".into()),
            proposed_effect: ProposedEffect::Unsupported {
                intent: "z_order".into(),
            },
            confidence_threshold: None,
            saved_context_id: None,
        }],
    };
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let proofs = DesktopActionService::proofs_for_plan(&plan, "u");
    let result = RestoreExecutor::execute(
        &plan,
        &proofs,
        &caps(),
        &mutator,
        &ActionExecutionControls::default(),
    )
    .unwrap();

    assert_eq!(result.summary.restored_windows, 0);
    assert!(result.summary.skipped_windows >= 1);
    assert_eq!(
        result.items[0].disposition,
        ItemDisposition::SkippedUnsupported
    );
}
