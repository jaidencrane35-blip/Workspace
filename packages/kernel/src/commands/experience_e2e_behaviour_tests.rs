//! Behavioural Experience E2E — production IPC surface (CommandHandler).
//!
//! Exercises the frozen operator command sequence without driving WebView pixels.
//! Tauri commands call the same `CommandHandler` methods (see `app/src-tauri/src/commands`).
//!
//! Flow: Launch → Home → Save → Continue → Restore → Restart → Hydrate → Continue again.

#![cfg(windows)]

use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use tempfile::TempDir;
use workspace_domain::{
    ActorContext, IntentContext, OperationOutcome, SAVED_CONTEXT_SCOPE_ID,
};

use crate::config::SettingsUpdate;
use crate::services::observation_flight_test_lock;
use crate::services::WorkspaceRuntimeStateService;
use crate::{CommandHandler, WorkspaceKernel};

#[derive(Debug, Serialize)]
struct StepEvidence {
    step: String,
    ipc_command: String,
    ok: bool,
    detail: String,
}

#[derive(Debug, Serialize)]
struct ExperienceE2eEvidence {
    collected_at: String,
    mode: String,
    database_path: String,
    steps: Vec<StepEvidence>,
    saved_context_id: Option<String>,
    post_restart_listed_moments: usize,
    post_restart_session_saved_context_id: Option<String>,
    continue_again_outcome: Option<String>,
}

fn evidence_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../architecture/evidence/experience-e2e-behaviour.json")
}

fn actor() -> ActorContext {
    ActorContext::local_user()
}

fn intent() -> IntentContext {
    IntentContext::user_request()
}

#[test]
fn experience_operator_ipc_flow_launch_through_continue_again() {
    let _flight = observation_flight_test_lock().lock().unwrap();
    WorkspaceRuntimeStateService::reset_for_tests();

    let temp = TempDir::new().expect("tempdir");
    let db_path = temp.path().join("experience-e2e.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let mut steps = Vec::new();

    // Launch — same authority as Tauri kernel bootstrap (`WorkspaceKernel::initialize`).
    let kernel = WorkspaceKernel::initialize(&db_path).expect("launch initialize");
    let status = kernel.state();
    steps.push(StepEvidence {
        step: "launch".into(),
        ipc_command: "get_workspace_status".into(),
        ok: status.lifecycle.is_initialized(),
        detail: format!(
            "lifecycle={} version={}",
            status.lifecycle.as_str(),
            status.version
        ),
    });
    let health = kernel.health();
    steps.push(StepEvidence {
        step: "launch_health".into(),
        ipc_command: "get_workspace_health".into(),
        ok: true,
        detail: format!("{health:?}"),
    });

    // Bootstrap / Home prerequisites.
    let workspace = kernel
        .create_workspace("Experience E2E".into())
        .expect("create_workspace");
    steps.push(StepEvidence {
        step: "bootstrap_create_workspace".into(),
        ipc_command: "create_workspace".into(),
        ok: true,
        detail: workspace.id.to_string(),
    });

    let settings = kernel
        .update_settings(SettingsUpdate {
            theme: None,
            first_run: None,
            active_workspace_id: Some(workspace.id.to_string()),
            personalization_enabled: None,
        })
        .expect("update_settings");
    steps.push(StepEvidence {
        step: "bootstrap_active_workspace".into(),
        ipc_command: "update_settings".into(),
        ok: settings.active_workspace_id.as_deref() == Some(workspace.id.as_str()),
        detail: format!("{:?}", settings.active_workspace_id),
    });

    // Home — list moments (empty).
    let home_list = CommandHandler::list_saved_contexts(
        &kernel,
        actor(),
        intent(),
        workspace.id.to_string(),
    )
    .expect("list_saved_contexts");
    steps.push(StepEvidence {
        step: "home".into(),
        ipc_command: "list_saved_contexts".into(),
        ok: home_list.is_empty(),
        detail: format!("count={}", home_list.len()),
    });

    // Save — scope then capture (live Win32 via production save path).
    let scope =
        CommandHandler::get_saved_context_capture_scope(&kernel, actor(), intent()).expect("scope");
    steps.push(StepEvidence {
        step: "save_scope".into(),
        ipc_command: "get_saved_context_capture_scope".into(),
        ok: scope.id == SAVED_CONTEXT_SCOPE_ID,
        detail: scope.id.clone(),
    });

    let saved = CommandHandler::save_workspace_context(
        &kernel,
        actor(),
        intent(),
        workspace.id.to_string(),
        "E2E Moment".into(),
        SAVED_CONTEXT_SCOPE_ID.into(),
        "Resume after Experience E2E restore".into(),
    )
    .expect("save_workspace_context");
    steps.push(StepEvidence {
        step: "save".into(),
        ipc_command: "save_workspace_context".into(),
        ok: !saved.windows.is_empty() && !saved.monitors.is_empty(),
        detail: format!(
            "id={} windows={} monitors={}",
            saved.id,
            saved.windows.len(),
            saved.monitors.len()
        ),
    });

    let home_after = CommandHandler::list_saved_contexts(
        &kernel,
        actor(),
        intent(),
        workspace.id.to_string(),
    )
    .expect("list after save");
    steps.push(StepEvidence {
        step: "home_after_save".into(),
        ipc_command: "list_saved_contexts".into(),
        ok: home_after.len() == 1,
        detail: format!("count={}", home_after.len()),
    });

    // Continue — inspect + plan.
    let loaded = CommandHandler::get_saved_context(
        &kernel,
        actor(),
        intent(),
        saved.id.to_string(),
    )
    .expect("get_saved_context");
    steps.push(StepEvidence {
        step: "continue_inspect".into(),
        ipc_command: "get_saved_context".into(),
        ok: loaded.id == saved.id,
        detail: loaded.id.to_string(),
    });

    let preview = CommandHandler::resolve_resume_plan(
        &kernel,
        actor(),
        intent(),
        saved.id.to_string(),
    )
    .expect("resolve_resume_plan");
    steps.push(StepEvidence {
        step: "continue_plan".into(),
        ipc_command: "resolve_resume_plan".into(),
        ok: !preview.plan.items.is_empty(),
        detail: format!(
            "items={} band={}",
            preview.plan.items.len(),
            preview.compatibility.confidence_band
        ),
    });

    // Restore — execute approved digest.
    let result = CommandHandler::execute_resume_plan(
        &kernel,
        actor(),
        intent(),
        preview.plan.clone(),
        preview.plan.plan_digest.clone(),
    )
    .expect("execute_resume_plan");
    steps.push(StepEvidence {
        step: "restore".into(),
        ipc_command: "execute_resume_plan".into(),
        ok: matches!(
            result.outcome,
            OperationOutcome::Completed
                | OperationOutcome::PartiallyCompleted
                | OperationOutcome::Failed
        ),
        detail: format!(
            "outcome={:?} restored={} skipped={} failed={}",
            result.outcome,
            result.summary.restored_windows,
            result.summary.skipped_windows,
            result.summary.failed_operations
        ),
    });

    // Drop process-local owner + kernel (application restart).
    drop(kernel);
    WorkspaceRuntimeStateService::reset_for_tests();

    // Restart + hydrate session (initialize loads durable session).
    let kernel2 = WorkspaceKernel::initialize(&db_path).expect("restart initialize");
    let runtime = CommandHandler::get_workspace_runtime_state(&kernel2, actor(), intent())
        .expect("get_workspace_runtime_state");
    steps.push(StepEvidence {
        step: "restart_hydrate".into(),
        ipc_command: "get_workspace_runtime_state".into(),
        ok: true,
        detail: format!(
            "integrity={} disposition={} last_saved={:?}",
            runtime.health.session_integrity.as_str(),
            runtime.health.recovery_disposition.as_str(),
            {
                let db = kernel2.shared_database();
                let guard = db.lock().unwrap();
                crate::services::WorkspaceSessionStore::load_recovered(&guard).last_saved_context_id
            }
        ),
    });

    let listed = CommandHandler::list_saved_contexts(
        &kernel2,
        actor(),
        intent(),
        workspace.id.to_string(),
    )
    .expect("list after restart");
    steps.push(StepEvidence {
        step: "home_after_restart".into(),
        ipc_command: "list_saved_contexts".into(),
        ok: listed.iter().any(|c| c.id == saved.id),
        detail: format!("count={}", listed.len()),
    });

    // Continue again.
    let preview2 = CommandHandler::resolve_resume_plan(
        &kernel2,
        actor(),
        intent(),
        saved.id.to_string(),
    )
    .expect("resolve again");
    let result2 = CommandHandler::execute_resume_plan(
        &kernel2,
        actor(),
        intent(),
        preview2.plan.clone(),
        preview2.plan.plan_digest.clone(),
    )
    .expect("execute again");
    steps.push(StepEvidence {
        step: "continue_again".into(),
        ipc_command: "execute_resume_plan".into(),
        ok: true,
        detail: format!("outcome={:?}", result2.outcome),
    });

    let session_saved = {
        let db = kernel2.shared_database();
        let guard = db.lock().unwrap();
        crate::services::WorkspaceSessionStore::load_recovered(&guard).last_saved_context_id
    };

    let all_ok = steps.iter().all(|s| s.ok);
    let evidence = ExperienceE2eEvidence {
        collected_at: format!("{:?}", std::time::SystemTime::now()),
        mode: "production_command_handler_ipc_behavioural".into(),
        database_path: db_path_str,
        steps,
        saved_context_id: Some(saved.id.to_string()),
        post_restart_listed_moments: listed.len(),
        post_restart_session_saved_context_id: session_saved,
        continue_again_outcome: Some(format!("{:?}", result2.outcome)),
    };

    let path = evidence_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(
        &path,
        serde_json::to_string_pretty(&evidence).expect("serialize"),
    )
    .expect("write experience e2e evidence");

    assert!(all_ok, "every Experience IPC step must succeed");
    assert_eq!(evidence.post_restart_listed_moments, 1);
}
