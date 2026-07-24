//! Workspace Environment Model tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
};
use workspace_windows_integration::DesktopWindowSnapshot;

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

#[test]
fn environment_aggregates_windows_to_applications() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let windows = vec![DesktopWindowSnapshot {
        hwnd: "0x1".into(),
        title: "main.rs - Visual Studio Code".into(),
        process_id: 100,
        visible: true,
    }];
    let apps = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        workspace_database::ApplicationRepository::new(&guard)
            .list_by_workspace(&workspace_domain::WorkspaceId::new(ws.clone()).unwrap())
            .unwrap()
    };
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        &ws,
        &windows,
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
    let windows = vec![DesktopWindowSnapshot {
        hwnd: "0x2".into(),
        title: "Unrelated Notepad".into(),
        process_id: 200,
        visible: true,
    }];
    let apps = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        workspace_database::ApplicationRepository::new(&guard)
            .list_by_workspace(&workspace_domain::WorkspaceId::new(ws.clone()).unwrap())
            .unwrap()
    };
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        &ws,
        &windows,
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
    let windows = vec![
        DesktopWindowSnapshot {
            hwnd: "0x3".into(),
            title: "a - Visual Studio Code".into(),
            process_id: 10,
            visible: true,
        },
        DesktopWindowSnapshot {
            hwnd: "0x4".into(),
            title: "b - Visual Studio Code".into(),
            process_id: 10,
            visible: true,
        },
    ];
    let apps = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        workspace_database::ApplicationRepository::new(&guard)
            .list_by_workspace(&workspace_domain::WorkspaceId::new(ws.clone()).unwrap())
            .unwrap()
    };
    let workflow = CommandHandler::get_workflow_context(
        &kernel,
        local.clone(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let state = crate::services::WorkspaceEnvironmentService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        &ws,
        &windows,
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
