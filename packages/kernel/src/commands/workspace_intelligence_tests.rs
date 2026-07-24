//! Workspace Intelligence Foundation tests (Phase 4 Batch 5).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, ApprovalDecisionKind, CapabilitySet, IntentContext, MemoryType,
    PreferenceCategory, PreferenceSource, TaskPriority,
};

fn seed_workspace(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Intelligence WS".into()))
        .unwrap();
    let app_a = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "VS Code".into(),
            None,
            Some("code.exe".into()),
        ))
        .unwrap();
    let app_b = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Terminal".into(),
            None,
            Some("wt.exe".into()),
        ))
        .unwrap();
    (
        workspace.id.to_string(),
        app_a.id.to_string(),
        app_b.id.to_string(),
    )
}

#[test]
fn case1_intelligence_aggregates_workspace_state() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Workspace AI".into(),
        Some("Core platform".into()),
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        ws.clone(),
        "Build Workspace Intelligence Layer".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws.clone(),
    )
    .unwrap();

    assert_eq!(state.workspace_id, ws);
    assert_eq!(
        state.current_project.as_ref().map(|p| p.id.as_str()),
        Some(project.id.as_str())
    );
    assert_eq!(
        state.current_task.as_ref().map(|t| t.id.as_str()),
        Some(task.id.as_str())
    );
    assert_eq!(state.authority_effect, "none");
    assert!(!state.summary.is_empty());
    assert!(!state.current_applications.is_empty());

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "workspace.intelligence.generated"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "workspace.summary.created"));
}

#[test]
fn case2_projects_and_tasks_persist() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Workspace AI".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        ws.clone(),
        "Persist intent".into(),
        TaskPriority::Medium,
    )
    .unwrap();

    let loaded_project =
        CommandHandler::get_project(&kernel, local.clone(), intent.clone(), project.id.to_string())
            .unwrap();
    let loaded_task =
        CommandHandler::get_task(&kernel, local.clone(), intent.clone(), task.id.to_string())
            .unwrap();
    let listed = CommandHandler::list_tasks(
        &kernel,
        local,
        intent,
        ws,
        Some(project.id.to_string()),
        Some(10),
    )
    .unwrap();

    assert_eq!(loaded_project.name, "Workspace AI");
    assert_eq!(loaded_task.title, "Persist intent");
    assert_eq!(listed.len(), 1);
}

#[test]
fn case3_memory_updates_intelligence_output() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(before.memory_highlights.is_empty());

    CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "preferred_application",
        "User prefers VS Code for coding workspaces",
        "intelligence_test",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let after =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(!after.memory_highlights.is_empty());
    assert!(after
        .recommended_actions
        .iter()
        .any(|r| r.kind == "memory" || r.explanation.contains("memory")));
}

#[test]
fn case4_preferences_update_recommendations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    CommandHandler::create_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        PreferenceCategory::Application,
        "default_editor",
        "VS Code is my default editor",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        Some("VS Code".into()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let state =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(!state.preference_highlights.is_empty());
    assert!(state
        .recommended_actions
        .iter()
        .any(|r| r.kind == "preference" || r.explanation.to_lowercase().contains("prefer")));
}

#[test]
fn case5_pending_approvals_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let _ = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a],
    )
    .unwrap();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(
        !state.pending_approvals.is_empty(),
        "expected pending approvals in intelligence: {:?}",
        state.pending_approvals
    );
}

#[test]
fn case6_blocked_actions_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a],
        None,
    )
    .unwrap();
    let confirmed = CommandHandler::confirm_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();
    let plan = CommandHandler::get_orchestrated_ai_plan(
        &kernel,
        confirmed.orchestrated_plan_id.as_ref().unwrap().to_string(),
    )
    .unwrap();
    let approval_id = plan.steps[0]
        .approval_request_id
        .clone()
        .expect("approval");
    CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(DecideApproval::new(
            workspace_domain::PermissionApprovalRequestId::new(approval_id).unwrap(),
            ApprovalDecisionKind::Deny,
        ))
        .unwrap();
    let _ = CommandHandler::resume_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();

    let state =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(
        !state.blocked_actions.is_empty(),
        "expected blocked actions: {:?}",
        state.blocked_actions
    );
}

#[test]
fn case7_intelligence_cannot_execute_commands() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let _ = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
    let _ = app_a;
}

#[test]
fn case8_intelligence_cannot_bypass_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel);
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-intel-8").unwrap());
    assert_eq!(
        CapabilitySet::for_actor_type(ActorType::AIAssistant)
            .iter()
            .count(),
        0
    );

    let err = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        ws,
    );
    match err {
        Err(KernelError::PermissionDenied(_))
        | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected permission boundary, got {other:?}"),
    }
}

#[test]
fn case9_removing_memory_and_preferences_removes_influence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let memory = CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "preferred_application",
        "User prefers VS Code",
        "intelligence_test",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();
    let preference = CommandHandler::create_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        PreferenceCategory::Application,
        "default_editor",
        "VS Code is my default editor",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        Some("VS Code".into()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let with_influence = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!with_influence.memory_highlights.is_empty());
    assert!(!with_influence.preference_highlights.is_empty());

    CommandHandler::delete_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        memory.id.to_string(),
    )
    .unwrap();
    CommandHandler::delete_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        preference.id.to_string(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(after.memory_highlights.is_empty());
    assert!(after.preference_highlights.is_empty());
}

#[test]
fn case10_restart_preserves_durable_intent_state() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("intent-persist.db");

    let project_id;
    let task_id;
    let workspace_id;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
            .execute_mutation(CreateWorkspace::new("Durable Intent WS".into()))
            .unwrap();
        workspace_id = workspace.id.to_string();
        let project = CommandHandler::create_project(
            &kernel,
            local.clone(),
            intent.clone(),
            workspace_id.clone(),
            "Workspace AI".into(),
            None,
            None,
        )
        .unwrap();
        let task = CommandHandler::create_task(
            &kernel,
            local.clone(),
            intent.clone(),
            project.id.to_string(),
            workspace_id.clone(),
            "Survive restart".into(),
            TaskPriority::High,
        )
        .unwrap();
        CommandHandler::set_active_work(
            &kernel,
            local,
            intent,
            workspace_id.clone(),
            Some(project.id.to_string()),
            Some(task.id.to_string()),
        )
        .unwrap();
        project_id = project.id.to_string();
        task_id = task.id.to_string();
    }

    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let project =
            CommandHandler::get_project(&kernel, local.clone(), intent.clone(), project_id)
                .unwrap();
        let task = CommandHandler::get_task(&kernel, local.clone(), intent.clone(), task_id)
            .unwrap();
        let context =
            CommandHandler::get_workflow_context(&kernel, local, intent, workspace_id).unwrap();
        assert_eq!(project.name, "Workspace AI");
        assert_eq!(task.title, "Survive restart");
        assert_eq!(
            context.active_project_id.as_ref().map(|id| id.as_str()),
            Some(project.id.as_str())
        );
        assert_eq!(
            context.active_task_id.as_ref().map(|id| id.as_str()),
            Some(task.id.as_str())
        );
    }
}
