//! OS process-kill recovery validation (Windows).
//!
//! Parent launches a child process that writes a restore-execution fence against
//! a shared on-disk session, then terminates the child with `taskkill /F`.
//! Parent restarts the kernel and validates RuntimeHealth + fence acknowledgment.
//!
//! This is operating-system interruption — not an in-process simulated clear.

#![cfg(windows)]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use tempfile::TempDir;
use workspace_domain::{
    ActorContext, IntentContext, PendingOperationKind, RecoveryDisposition, SessionIntegrity,
};

use crate::services::WorkspaceRuntimeStateService;
use crate::services::WorkspaceSessionStore;
use crate::WorkspaceKernel;

const CHILD_ENV: &str = "WORKSPACE_PROCESS_KILL_CHILD";
const DB_ENV: &str = "WORKSPACE_PROCESS_KILL_DB";
const READY_ENV: &str = "WORKSPACE_PROCESS_KILL_READY";

#[derive(Debug, Serialize)]
struct ProcessKillEvidence {
    collected_at: String,
    mode: String,
    child_pid: u32,
    kill_command: String,
    kill_ok: bool,
    recovery_disposition: String,
    session_integrity: String,
    fence_cleared: bool,
    last_recovery_kind: Option<String>,
    database_path: String,
}

fn evidence_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../architecture/evidence/process-kill-recovery.json")
}

/// Child entry: write fence, signal ready, sleep until killed.
fn run_child(db_path: &str, ready_path: &str) {
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize(db_path).expect("child initialize");
    WorkspaceSessionStore::begin_operation(
        &kernel.shared_database(),
        PendingOperationKind::RestoreExecution,
        Some("process-kill-restore".into()),
    )
    .expect("begin restore fence");
    fs::write(ready_path, b"ready").expect("ready signal");
    // Hold until parent terminates this process.
    thread::sleep(Duration::from_secs(120));
    // If we reach here, kill failed — exit non-zero for parent detection.
    std::process::exit(2);
}

#[test]
fn os_process_kill_during_restore_fence_recovers_deterministically() {
    if std::env::var(CHILD_ENV).is_ok() {
        let db = std::env::var(DB_ENV).expect("db path");
        let ready = std::env::var(READY_ENV).expect("ready path");
        run_child(&db, &ready);
        return;
    }

    WorkspaceRuntimeStateService::reset_for_tests();
    let temp = TempDir::new().expect("tempdir");
    let db_path = temp.path().join("kill-recovery.db");
    let ready_path = temp.path().join("ready.flag");
    let db_str = db_path.to_string_lossy().to_string();
    let ready_str = ready_path.to_string_lossy().to_string();

    // Seed DB schema via a brief parent initialize, then drop.
    {
        let _kernel = WorkspaceKernel::initialize(&db_path).expect("seed initialize");
    }
    WorkspaceRuntimeStateService::reset_for_tests();

    let exe = std::env::current_exe().expect("current_exe");
    let mut child = Command::new(&exe)
        .env(CHILD_ENV, "1")
        .env(DB_ENV, &db_str)
        .env(READY_ENV, &ready_str)
        .args([
            "--exact",
            "commands::process_kill_recovery_tests::os_process_kill_during_restore_fence_recovers_deterministically",
            "--nocapture",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn child");

    let pid = child.id();
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if ready_path.exists() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert!(
        ready_path.exists(),
        "child must write restore fence ready signal"
    );

    // Operating-system termination.
    let kill = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .output()
        .expect("taskkill");
    let kill_ok = kill.status.success();
    let _ = child.wait();

    // Restart + hydrate — must see Incomplete recovery for RestoreExecution.
    WorkspaceRuntimeStateService::reset_for_tests();
    let kernel = WorkspaceKernel::initialize(&db_path).expect("parent restart");
    let runtime = crate::CommandHandler::get_workspace_runtime_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .expect("runtime state");

    let session = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        WorkspaceSessionStore::load_recovered(&guard)
    };

    let evidence = ProcessKillEvidence {
        collected_at: format!("{:?}", std::time::SystemTime::now()),
        mode: "os_taskkill_during_restore_execution_fence".into(),
        child_pid: pid,
        kill_command: format!("taskkill /PID {pid} /F"),
        kill_ok,
        recovery_disposition: runtime.health.recovery_disposition.as_str().into(),
        session_integrity: runtime.health.session_integrity.as_str().into(),
        fence_cleared: session.pending_operation.is_none(),
        last_recovery_kind: session
            .last_recovery
            .as_ref()
            .map(|r| r.kind.as_str().into()),
        database_path: db_str,
    };

    let path = evidence_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(
        &path,
        serde_json::to_string_pretty(&evidence).expect("serialize"),
    )
    .expect("write process-kill evidence");

    assert!(kill_ok, "taskkill must succeed");
    assert_eq!(
        runtime.health.recovery_disposition,
        RecoveryDisposition::Incomplete
    );
    assert!(matches!(
        runtime.health.session_integrity,
        SessionIntegrity::Ok | SessionIntegrity::MissingRecovered
    ));
    assert!(session.pending_operation.is_none());
    assert_eq!(
        session.last_recovery.as_ref().map(|r| r.kind),
        Some(PendingOperationKind::RestoreExecution)
    );
}
