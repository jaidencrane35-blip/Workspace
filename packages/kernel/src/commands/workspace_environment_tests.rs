//! Workspace Environment Model tests (Phase 5 / Sprint 119–121).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_database::ObservationPassRepository;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, EnvironmentWindowState, IntentContext,
    ObservedMonitor, ObservedWindow, PLATFORM_CONCEPT_OWNERS, TaskPriority,
    WorkspaceObservationPass, WorkspaceObservationSnapshot, WorkspaceState,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Environment WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Env Project".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        workspace_id.clone(),
        "Env Task".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    let app = CommandHandler::create_application(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Visual Studio Code".into(),
        Some("Code.exe".into()),
        None,
    )
    .unwrap();
    (workspace_id, app.id.to_string())
}

fn state_window(
    hwnd: &str,
    title: &str,
    process_id: i32,
    focused: bool,
) -> workspace_domain::WorkspaceStateWindow {
    WorkspaceState::fixture_window(hwnd, title, process_id, focused)
}

fn load_apps(
    kernel: &WorkspaceKernel,
    ws: &str,
) -> Vec<workspace_domain::ApplicationReference> {
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    workspace_database::ApplicationRepository::new(&guard)
        .list_by_workspace(&workspace_domain::WorkspaceId::new(ws).unwrap())
        .unwrap()
}

#[test]
fn environment_aggregates_windows_to_applications() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let workspace_state = WorkspaceState::from_windows(vec![state_window(
        "0x1",
        "main.rs - Visual Studio Code",
        100,
        false,
    )]);
    let apps = load_apps(&kernel, &ws);
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_from_state(
        &kernel.shared_database(),
        &local,
        &ws,
        &workspace_state,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert_eq!(state.windows.len(), 1);
    assert_eq!(
        state.windows[0].matched_application_id.as_deref(),
        Some(app_id.as_str())
    );
    assert!(state
        .applications
        .iter()
        .any(|a| a.application_id == app_id && a.appears_running));
    assert_eq!(state.running_application_count, 1);
    assert_eq!(state.missing_application_count, 0);
}

#[test]
fn missing_applications_and_disconnected_work_surface_gaps() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let workspace_state = WorkspaceState::from_windows(vec![state_window(
        "0x2",
        "Unrelated Notepad",
        200,
        false,
    )]);
    let apps = load_apps(&kernel, &ws);
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_from_state(
        &kernel.shared_database(),
        &local,
        &ws,
        &workspace_state,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();
    assert!(state.missing_application_count >= 1);
    assert!(state.disconnected_work);
    assert!(state.gaps.iter().any(|g| g.kind == "missing_application"));
    assert!(state.gaps.iter().any(|g| g.kind == "disconnected_work"));
}

#[test]
fn window_groups_form_by_application() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let workspace_state = WorkspaceState::from_windows(vec![
        state_window("0x3", "a - Visual Studio Code", 10, false),
        state_window("0x4", "b - Visual Studio Code", 10, false),
    ]);
    let apps = load_apps(&kernel, &ws);
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_from_state(
        &kernel.shared_database(),
        &local,
        &ws,
        &workspace_state,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();
    assert!(state.window_groups.iter().any(|g| g.window_ids.len() == 2));
}

#[test]
fn intelligence_embeds_environment_summary() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(intel.environment.authority_effect, "none");
    assert!(!intel.environment.summary.is_empty());
}

#[test]
fn permission_gateway_unchanged_authority_none() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_environment(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    for window in &state.windows {
        assert_eq!(window.authority_effect, "none");
    }
}

#[test]
fn environment_cannot_execute() {
    let err = CommandHandler::workspace_environment_attempt_execute().unwrap_err();
    match err {
        KernelError::WorkspaceEnvironmentValidation { message } => {
            assert!(message.contains("cannot execute") || message.contains("authorize"));
        }
        other => panic!("expected WorkspaceEnvironmentValidation, got {other:?}"),
    }
}

#[test]
fn environment_concept_ownership_registered() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "environment"
            && c.owner == "WorkspaceEnvironmentService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

#[test]
fn environment_uses_workspace_state_focus_not_list_order() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let workspace_state = WorkspaceState::from_windows(vec![
        state_window("0x10", "Background App", 10, false),
        state_window("0x11", "Focused App", 11, true),
    ]);
    let apps = load_apps(&kernel, &ws);
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_from_state(
        &kernel.shared_database(),
        &local,
        &ws,
        &workspace_state,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();
    let focused_window = state
        .windows
        .iter()
        .find(|window| window.hwnd == "0x11")
        .expect("focused window row");
    assert_eq!(focused_window.state, EnvironmentWindowState::Focused);
    assert_eq!(
        state.focused_window_id.as_deref(),
        Some(focused_window.id.as_str())
    );
    let background_window = state
        .windows
        .iter()
        .find(|window| window.hwnd == "0x10")
        .expect("background window row");
    assert_eq!(background_window.state, EnvironmentWindowState::Open);
}

#[test]
fn generate_path_matches_generate_from_state_after_observation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let pass_id = "env-pass-1";
    let snapshot = WorkspaceObservationSnapshot {
        pass: WorkspaceObservationPass {
            id: pass_id.into(),
            captured_at: "2026-07-26T12:00:00Z".into(),
            schema_version: 1,
            source: "test_inject".into(),
            foreground_hwnd: Some("0xAA".into()),
            window_count: 1,
            monitor_count: 1,
            duration_ms: Some(1),
            metadata_json: "{}".into(),
            authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
        },
        monitors: vec![ObservedMonitor {
            id: format!("{pass_id}-mon"),
            pass_id: pass_id.into(),
            monitor_index: 0,
            name: "Primary".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            work_x: 0,
            work_y: 0,
            work_w: 1920,
            work_h: 1040,
            is_primary: true,
            dpi_scale: None,
            authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
        }],
        windows: vec![ObservedWindow {
            id: format!("{pass_id}-win"),
            pass_id: pass_id.into(),
            hwnd: "0xAA".into(),
            stable_window_id: Some("stable-vscode".into()),
            title: "main.rs - Visual Studio Code".into(),
            process_id: 4242,
            process_name: Some("Code.exe".into()),
            x: 10,
            y: 10,
            width: 800,
            height: 600,
            monitor_id: Some(format!("{pass_id}-mon")),
            visible: true,
            minimized: false,
            focused: true,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        }],
        identities: Vec::new(),
        authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
    };
    {
        let db = kernel.shared_database();
        ObservationPassRepository::new(&db.lock().unwrap())
            .insert_snapshot(&snapshot)
            .unwrap();
    }

    let via_generate = CommandHandler::generate_workspace_environment(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();

    let workspace_state = crate::services::WorkspaceStateEngine::get_current(
        &kernel.shared_database(),
        &local,
        &intent,
    )
    .unwrap();
    let apps = load_apps(&kernel, &ws);
    let workflow =
        CommandHandler::get_workflow_context(&kernel, local.clone(), intent, ws.clone()).unwrap();
    let via_state = crate::services::WorkspaceEnvironmentService::generate_from_state(
        &kernel.shared_database(),
        &local,
        &ws,
        &workspace_state,
        &apps,
        &workflow,
        None,
        None,
    )
    .unwrap();

    assert_eq!(via_generate.windows.len(), via_state.windows.len());
    assert_eq!(via_generate.focused_window_id, via_state.focused_window_id);
    assert_eq!(
        via_generate.windows[0].matched_application_id.as_deref(),
        Some(app_id.as_str())
    );
    assert_eq!(
        via_generate.windows[0].matched_application_id,
        via_state.windows[0].matched_application_id
    );
    assert_eq!(via_generate.running_application_count, 1);
    assert_eq!(
        via_generate.running_application_count,
        via_state.running_application_count
    );
    assert_eq!(
        workspace_state.metadata.observation_pass_id.as_deref(),
        Some(pass_id)
    );
}
