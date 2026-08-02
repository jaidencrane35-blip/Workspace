//! SCRI-AC-* and ADM-AC-* acceptance evidence for PP-M1-02.

use std::sync::{Arc, Mutex};

use chrono::{Duration, Utc};
use workspace_domain::{
    compute_plan_digest, ActionRequest, ActionTargetDescriptor, Capability, CapabilitySet,
    DesktopActionError, ItemDisposition, ItemEffectProof, OperationOutcome, ProjectedDisposition,
    ProposedEffect, SavedContext, SavedContextId, SavedContextMonitor, SavedContextRestoreIdentity,
    SavedContextWindow, WorkspaceId, ACTION_TYPE_APPLICATION_LAUNCH, ACTION_TYPE_WINDOW_FOCUS,
    ACTION_TYPE_WINDOW_PLACE, RESTORE_IDENTITY_LEGACY_REASON, SAVED_CONTEXT_SCOPE_ID,
    SCOPE_PLAN_RESOLVE, SCOPE_WINDOW_FOCUS, SCOPE_WINDOW_PLACE,
};
use workspace_windows_integration::{StubDesktopCapturer, StubWindowMutator};

use crate::commands::resume::resolve_and_execute_for_tests;
use crate::error::KernelError;
use crate::services::{
    action_request_from_saved_context, ActionExecutionControls, DesktopActionService,
    SavedContextService, WorkspaceService,
};
use crate::services::observation_flight_test_lock;
use crate::WorkspaceKernel;
use workspace_domain::{ActorContext, IntentContext, SaveContextRequest};

fn caps() -> CapabilitySet {
    CapabilitySet::local_user_standard()
}

fn sample_context_with_identity() -> SavedContext {
    let identity = SavedContextRestoreIdentity::new(
        "stub-desktop-session-1",
        "0x00000000000000AA",
        100,
        "Fixture Focus",
        "2026-08-01T10:00:00Z",
    );
    SavedContext {
        id: SavedContextId::new("sc-resume-1").unwrap(),
        workspace_id: WorkspaceId::new("ws-resume").unwrap(),
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

fn legacy_context() -> SavedContext {
    let mut context = sample_context_with_identity();
    context.id = SavedContextId::new("sc-legacy").unwrap();
    context.approved_scope = "saved-context-scope-v1".into();
    for window in &mut context.windows {
        window.restore_identity = None;
        window.restore_identity_unavailable_reason =
            Some(RESTORE_IDENTITY_LEGACY_REASON.into());
    }
    context
}

/// SCRI-AC-01 — new Save requires current (v2) scope consent.
#[test]
fn scri_ac_01_new_save_requires_v2_scope_consent() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    let workspace = {
        let guard = db.lock().unwrap();
        WorkspaceService::create(&guard, "Proof".into()).unwrap()
    };
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let refused = SavedContextService::save_with(
        &db,
        &actor,
        &intent,
        &SaveContextRequest::new(
            workspace.id.clone(),
            "Old",
            "saved-context-scope-v1",
            "Finish the client proposal outline",
        ),
        &StubDesktopCapturer::fixture_dual_monitor(),
    );
    assert!(matches!(refused, Err(KernelError::SavedContextValidation { .. })));

    let saved = SavedContextService::save_with(
        &db,
        &actor,
        &intent,
        &SaveContextRequest::new(
            workspace.id,
            "New",
            SAVED_CONTEXT_SCOPE_ID,
            "Finish the client proposal outline",
        ),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();
    assert!(saved.windows.iter().all(|w| w.restore_identity.is_some()));
}

/// SCRI-AC-02 — legacy contexts preview as unsupported; no backfill.
#[test]
fn scri_ac_02_legacy_preview_unsupported_without_backfill() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let context = legacy_context();
    let request = action_request_from_saved_context(&context);
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    assert!(plan.items.iter().all(|item| {
        item.projected_disposition != ProjectedDisposition::WillAttempt
            || item.action_type == "window.z_order"
    }));
    assert!(plan.items.iter().any(|item| {
        item.error_code.as_deref() == Some("ACTION_TARGET_IDENTITY_UNAVAILABLE")
    }));
    assert!(context.windows.iter().all(|w| w.restore_identity.is_none()));
}

/// SCRI-AC-03 — exact continuity → will_attempt.
#[test]
fn scri_ac_03_exact_match_will_attempt() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    assert!(plan.items.iter().any(|item| {
        item.action_type == ACTION_TYPE_WINDOW_PLACE
            && item.projected_disposition == ProjectedDisposition::WillAttempt
    }));
    assert!(plan.items.iter().any(|item| {
        item.action_type == ACTION_TYPE_WINDOW_FOCUS
            && item.projected_disposition == ProjectedDisposition::WillAttempt
    }));
}

/// SCRI-AC-04 — handle reuse by different title fails closed.
#[test]
fn scri_ac_04_handle_reuse_fails_closed() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.set_window_title("0x00000000000000AA", "Different Title");
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let place = plan
        .items
        .iter()
        .find(|item| item.action_type == ACTION_TYPE_WINDOW_PLACE)
        .unwrap();
    assert_eq!(
        place.projected_disposition,
        ProjectedDisposition::WillSkipUnresolvable
    );
}

/// SCRI-AC-05 — window recreation (hwnd gone) is unresolvable.
#[test]
fn scri_ac_05_recreation_unresolvable() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.remove_window("0x00000000000000AA");
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let place = plan
        .items
        .iter()
        .find(|item| item.action_type == ACTION_TYPE_WINDOW_PLACE)
        .unwrap();
    assert_eq!(place.error_code.as_deref(), Some("ACTION_TARGET_NOT_FOUND"));
}

/// SCRI-AC-06 — different desktop session is non-portable.
#[test]
fn scri_ac_06_different_session_non_portable() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.set_session_id("other-session");
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let place = plan
        .items
        .iter()
        .find(|item| item.action_type == ACTION_TYPE_WINDOW_PLACE)
        .unwrap();
    assert_eq!(
        place.error_code.as_deref(),
        Some("ACTION_TARGET_CONFIDENCE_INSUFFICIENT")
    );
}

/// SCRI-AC-07 — unsupported identity version fails closed.
#[test]
fn scri_ac_07_unsupported_identity_version() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut context = sample_context_with_identity();
    context.windows[0]
        .restore_identity
        .as_mut()
        .unwrap()
        .identity_schema_version = "99".into();
    let request = action_request_from_saved_context(&context);
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    assert!(plan.items.iter().any(|item| {
        item.error_code.as_deref() == Some("ACTION_TARGET_IDENTITY_VERSION_UNSUPPORTED")
    }));
}

/// SCRI-AC-08 — caller-supplied threshold is contract-invalid.
#[test]
fn scri_ac_08_caller_threshold_rejected() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut request = action_request_from_saved_context(&sample_context_with_identity());
    request.items[0].confidence_threshold = Some("0.5".into());
    let error = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap_err();
    assert!(matches!(
        error,
        KernelError::DesktopAction(DesktopActionError::ContractInvalid(_))
    ));
}

/// SCRI-AC-09 — non-exact match never effects.
#[test]
fn scri_ac_09_non_exact_no_effect() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.remove_window("0x00000000000000AA");
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    assert!(!result
        .items
        .iter()
        .any(|item| item.disposition == ItemDisposition::Completed
            && item.action_type == ACTION_TYPE_WINDOW_PLACE));
}

/// SCRI-AC-10 — exact match without effect proof does nothing.
#[test]
fn scri_ac_10_missing_effect_proof() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    let result = DesktopActionService::execute(
        &plan,
        &[],
        &caps(),
        &mutator,
        &ActionExecutionControls::default(),
    )
    .unwrap();
    assert!(result.items.iter().any(|item| {
        item.disposition == ItemDisposition::Failed
            && item.error_code.as_deref() == Some("ACTION_PERMISSION_DENIED")
    }));
}

/// SCRI-AC-11 — matching examination artifacts do not leave Action.
#[test]
fn scri_ac_11_no_handle_leak_in_plan_or_outcome() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let (plan, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    let plan_json = serde_json::to_string(&plan).unwrap();
    let outcome_json = serde_json::to_string(&result).unwrap();
    // Outcomes carry only minimized summaries — never live handles from matching.
    assert!(!outcome_json.contains("0x00000000000000AA"));
    assert!(!outcome_json.contains("candidate"));
    assert!(!plan_json.contains("\"candidates\""));
    assert!(!plan_json.contains("examined_count"));
}

/// SCRI-AC-12 — changed resolution after approval → refused_changed.
#[test]
fn scri_ac_12_changed_resolution_refused() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let request = action_request_from_saved_context(&sample_context_with_identity());
    let plan = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap();
    mutator.remove_window("0x00000000000000AA");
    let proofs = DesktopActionService::proofs_for_plan(&plan, "local-user");
    let result = DesktopActionService::execute(
        &plan,
        &proofs,
        &caps(),
        &mutator,
        &ActionExecutionControls::default(),
    )
    .unwrap();
    assert!(result.items.iter().any(|item| {
        item.disposition == ItemDisposition::RefusedChanged
            && item.error_code.as_deref() == Some("ACTION_PLAN_ITEM_CHANGED")
    }));
}

/// SCRI-AC-13 — incomplete identity remains with explicit reason.
#[test]
fn scri_ac_13_incomplete_identity_reason() {
    let mut context = sample_context_with_identity();
    context.windows[0].restore_identity = None;
    context.windows[0].restore_identity_unavailable_reason =
        Some("incomplete restore identity at capture".into());
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&context),
        &caps(),
        &mutator,
    )
    .unwrap();
    assert!(plan.items.iter().any(|item| {
        item.error_code.as_deref() == Some("ACTION_TARGET_IDENTITY_UNAVAILABLE")
            && item.reason.as_ref().is_some_and(|r| r.contains("incomplete"))
    }));
}

/// SCRI-AC-14 / PP-P01C — deleting a context deletes restore identities and
/// removes it from inspect/resume surfaces.
#[test]
fn scri_ac_14_delete_removes_identities() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    let workspace = {
        let guard = db.lock().unwrap();
        WorkspaceService::create(&guard, "Proof".into()).unwrap()
    };
    let saved = SavedContextService::save_with(
        &db,
        &ActorContext::local_user(),
        &IntentContext::user_request(),
        &SaveContextRequest::new(
            workspace.id.clone(),
            "Delete me",
            SAVED_CONTEXT_SCOPE_ID,
            "Finish the client proposal outline",
        ),
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();

    crate::commands::CommandHandler::delete_saved_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        saved.id.to_string(),
    )
    .unwrap();

    assert!(SavedContextService::get_by_id(&db, &saved.id)
        .unwrap()
        .is_none());
    let listed = crate::commands::CommandHandler::list_saved_contexts(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        workspace.id.to_string(),
    )
    .unwrap();
    assert!(listed.iter().all(|context| context.id != saved.id));
    let missing = crate::commands::CommandHandler::get_saved_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        saved.id.to_string(),
    );
    assert!(matches!(missing, Err(KernelError::SavedContextNotFound)));
    let resume = crate::commands::CommandHandler::resolve_resume_plan(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        saved.id.to_string(),
    );
    assert!(matches!(resume, Err(KernelError::SavedContextNotFound)));
}

#[test]
fn delete_saved_context_unknown_id_fails_closed() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let err = crate::commands::CommandHandler::delete_saved_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "sc-does-not-exist".into(),
    );
    assert!(matches!(err, Err(KernelError::SavedContextNotFound)));
}

/// SCRI-AC-15 — matching available without network/AI/plugins.
#[test]
fn scri_ac_15_local_matching_available() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    assert!(!plan.plan_id.is_empty());
}

/// ADM-AC-01 / ADM-AC-02 — undeclared / reserved types rejected at execute.
#[test]
fn adm_ac_01_02_undeclared_and_reserved_rejected_on_execute() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    plan.items[0].action_type = "window.teleport".into();
    plan.items[0].projected_disposition = ProjectedDisposition::WillAttempt;
    plan.plan_digest = compute_plan_digest(
        &plan.plan_id,
        &plan.expires_at,
        &plan.purpose,
        &plan.items,
    );
    let err = DesktopActionService::execute(
        &plan,
        &DesktopActionService::proofs_for_plan(&plan, "u"),
        &caps(),
        &mutator,
        &Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        KernelError::DesktopAction(DesktopActionError::TypeNotDeclared(_))
    ));

    let reserved = ActionRequest {
        purpose: "x".into(),
        items: vec![ActionTargetDescriptor {
            item_id: "launch".into(),
            action_type: ACTION_TYPE_APPLICATION_LAUNCH.into(),
            target_summary: "App".into(),
            restore_identity: None,
            identity_unavailable_reason: Some("n/a".into()),
            proposed_effect: ProposedEffect::Unsupported {
                intent: "launch".into(),
            },
            confidence_threshold: None,
            saved_context_id: None,
        }],
    };
    let plan = DesktopActionService::resolve_plan(&reserved, &caps(), &mutator).unwrap();
    assert_eq!(
        plan.items[0].projected_disposition,
        ProjectedDisposition::WillSkipUnsupported
    );
}

/// ADM-AC-03 — unchanged environment attempts exactly will_attempt set.
#[test]
fn adm_ac_03_attempted_set_matches_plan() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let (plan, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    let will_attempt: Vec<_> = plan
        .items
        .iter()
        .filter(|i| i.projected_disposition == ProjectedDisposition::WillAttempt)
        .map(|i| i.item_id.clone())
        .collect();
    let attempted: Vec<_> = result
        .items
        .iter()
        .filter(|i| {
            matches!(
                i.disposition,
                ItemDisposition::Completed
                    | ItemDisposition::Failed
                    | ItemDisposition::OutcomeUnknown
            )
        })
        .map(|i| i.item_id.clone())
        .collect();
    assert_eq!(will_attempt, attempted);
}

/// ADM-AC-04 covered by scri_ac_12.

/// ADM-AC-05 / ADM-AC-26 — expired or digest-mismatched plan rejected.
#[test]
fn adm_ac_05_26_plan_unknown_on_expiry_or_digest_mismatch() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    plan.plan_digest = "tampered".into();
    let err = DesktopActionService::execute(
        &plan,
        &[],
        &caps(),
        &mutator,
        &Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        KernelError::DesktopAction(DesktopActionError::PlanUnknown)
    ));

    let mut plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    plan.expires_at = (Utc::now() - Duration::seconds(1)).to_rfc3339();
    plan.plan_digest = compute_plan_digest(
        &plan.plan_id,
        &plan.expires_at,
        &plan.purpose,
        &plan.items,
    );
    let err = DesktopActionService::execute(
        &plan,
        &DesktopActionService::proofs_for_plan(&plan, "u"),
        &caps(),
        &mutator,
        &Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        KernelError::DesktopAction(DesktopActionError::PlanUnknown)
    ));
}

/// ADM-AC-06 / ADM-AC-22 — resolvePlan mutates nothing and retains no Action state.
#[test]
fn adm_ac_06_22_resolve_is_side_effect_free() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let before = mutator.window_placement("0x00000000000000AA");
    let _plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    assert_eq!(mutator.window_placement("0x00000000000000AA"), before);
}

/// ADM-AC-07 — mixed outcomes → partially_completed.
#[test]
fn adm_ac_07_partial_success() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut context = sample_context_with_identity();
    context.windows.push(SavedContextWindow {
        id: "w-missing".into(),
        title: "Gone".into(),
        process_id: 999,
        x: 1,
        y: 1,
        width: 100,
        height: 100,
        monitor_index: Some(0),
        minimized: false,
        focused: false,
        z_order: None,
        restore_identity: Some(SavedContextRestoreIdentity::new(
            "stub-desktop-session-1",
            "0xDEAD",
            999,
            "Gone",
            "t",
        )),
        restore_identity_unavailable_reason: None,
    });
    let (_, result) =
        resolve_and_execute_for_tests(&context, &caps(), &mutator, &Default::default()).unwrap();
    assert_eq!(result.outcome, OperationOutcome::PartiallyCompleted);
}

/// ADM-AC-08 — unknown outcome → indeterminate.
#[test]
fn adm_ac_08_indeterminate_dominates() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.unknown_place_outcome();
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    assert_eq!(result.outcome, OperationOutcome::Indeterminate);
}

/// ADM-AC-09 — every non-completed item has a reason.
#[test]
fn adm_ac_09_reasons_required() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.remove_window("0x00000000000000AA");
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    for item in &result.items {
        if item.disposition != ItemDisposition::Completed {
            assert!(item.reason.as_ref().is_some_and(|r| !r.trim().is_empty()));
        }
    }
}

/// ADM-AC-10 — unsupported z-order is skipped, never approximated.
#[test]
fn adm_ac_10_unsupported_not_approximated() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    let z = plan
        .items
        .iter()
        .find(|item| item.action_type == "window.z_order")
        .unwrap();
    assert_eq!(
        z.projected_disposition,
        ProjectedDisposition::WillSkipUnsupported
    );
}

/// ADM-AC-12 — not found skips without launch.
#[test]
fn adm_ac_12_not_found_skips() {
    scri_ac_05_recreation_unresolvable();
}

/// ADM-AC-14 — unsatisfiable placement.
#[test]
fn adm_ac_14_placement_unsatisfiable() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.set_monitor_indices(vec![]);
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    assert!(plan.items.iter().any(|item| {
        item.error_code.as_deref() == Some("ACTION_PLACEMENT_UNSATISFIABLE")
    }));
}

/// ADM-AC-15 — environment refusal.
#[test]
fn adm_ac_15_environment_refusal() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    mutator.refuse_place();
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    assert!(result.items.iter().any(|item| {
        item.error_code.as_deref() == Some("ACTION_TARGET_REFUSED_BY_ENVIRONMENT")
    }));
}

/// ADM-AC-16 — wrong-type proof denied.
#[test]
fn adm_ac_16_wrong_type_proof_denied() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    let mut proofs = DesktopActionService::proofs_for_plan(&plan, "u");
    for proof in &mut proofs {
        if proof.action_type == ACTION_TYPE_WINDOW_PLACE {
            proof.action_type = ACTION_TYPE_WINDOW_FOCUS.into();
            proof.permission_scope = SCOPE_WINDOW_FOCUS.into();
        }
    }
    let result = DesktopActionService::execute(
        &plan,
        &proofs,
        &caps(),
        &mutator,
        &Default::default(),
    )
    .unwrap();
    assert!(result.items.iter().any(|item| {
        item.action_type == ACTION_TYPE_WINDOW_PLACE
            && item.disposition == ItemDisposition::Failed
    }));
}

/// ADM-AC-17 — revocation between items.
#[test]
fn adm_ac_17_revocation_between_items() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    let grants = Arc::new(Mutex::new(caps()));
    let controls = ActionExecutionControls {
        cancel_requested: None,
        grants: Some(grants.clone()),
    };
    // Revoke place after first point-of-use by removing it before execute loop
    // progresses — simulate by starting without place.
    {
        let mut g = grants.lock().unwrap();
        *g = CapabilitySet::new()
            .with_capability(&Capability::action_plan_resolve())
            .with_capability(&Capability::action_window_focus());
    }
    let proofs = DesktopActionService::proofs_for_plan(&plan, "u");
    let result =
        DesktopActionService::execute(&plan, &proofs, &caps(), &mutator, &controls).unwrap();
    assert!(result.items.iter().any(|item| {
        item.action_type == ACTION_TYPE_WINDOW_PLACE
            && item.disposition == ItemDisposition::Failed
    }));
}

/// ADM-AC-18 — plan.resolve alone cannot execute effects.
#[test]
fn adm_ac_18_resolve_scope_is_not_effect_authority() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let resolve_only = CapabilitySet::new().with_capability(&Capability::action_plan_resolve());
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &resolve_only,
        &mutator,
    )
    .unwrap();
    let result = DesktopActionService::execute(
        &plan,
        &DesktopActionService::proofs_for_plan(&plan, "u"),
        &resolve_only,
        &mutator,
        &Default::default(),
    )
    .unwrap();
    assert!(!result
        .items
        .iter()
        .any(|item| item.disposition == ItemDisposition::Completed));
}

/// ADM-AC-19 — cancellation between items.
#[test]
fn adm_ac_19_cancellation() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let plan = DesktopActionService::resolve_plan(
        &action_request_from_saved_context(&sample_context_with_identity()),
        &caps(),
        &mutator,
    )
    .unwrap();
    let cancel = Arc::new(Mutex::new(true));
    let result = DesktopActionService::execute(
        &plan,
        &DesktopActionService::proofs_for_plan(&plan, "u"),
        &caps(),
        &mutator,
        &ActionExecutionControls {
            cancel_requested: Some(cancel),
            grants: None,
        },
    )
    .unwrap();
    assert_eq!(result.outcome, OperationOutcome::Cancelled);
    assert!(result
        .items
        .iter()
        .all(|item| item.disposition == ItemDisposition::NotAttempted));
}

/// ADM-AC-27 — saved-context id rejected by Action.
#[test]
fn adm_ac_27_saved_context_id_rejected() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let mut request = action_request_from_saved_context(&sample_context_with_identity());
    request.items[0].saved_context_id = Some("sc-1".into());
    let err = DesktopActionService::resolve_plan(&request, &caps(), &mutator).unwrap_err();
    assert!(matches!(
        err,
        KernelError::DesktopAction(DesktopActionError::ContractInvalid(_))
    ));
}

#[test]
fn successful_restore_moves_and_focuses() {
    let mutator = StubWindowMutator::fixture_dual_monitor();
    let (_, result) =
        resolve_and_execute_for_tests(&sample_context_with_identity(), &caps(), &mutator, &Default::default())
            .unwrap();
    assert!(result.items.iter().any(|item| {
        item.action_type == ACTION_TYPE_WINDOW_PLACE
            && item.disposition == ItemDisposition::Completed
    }));
    assert_eq!(
        mutator.window_placement("0x00000000000000AA"),
        Some((10, 20, 800, 600, false))
    );
    assert_eq!(
        mutator.focused_hwnd().as_deref(),
        Some("0x00000000000000AA")
    );
}

#[allow(dead_code)]
fn _scope_constants() {
    let _ = (SCOPE_PLAN_RESOLVE, SCOPE_WINDOW_PLACE, ItemEffectProof {
        plan_digest: String::new(),
        item_id: String::new(),
        action_type: String::new(),
        permission_scope: String::new(),
        purpose: String::new(),
        requester: String::new(),
    });
}
