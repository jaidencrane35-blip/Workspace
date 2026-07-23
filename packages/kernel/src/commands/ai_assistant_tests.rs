//! Governed AI assistant tests (Sprints 58–59).
//!
//! Assistant is an interface over orchestration — no bypass of Permission Gateway.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AiAssistantWorkflowState, AiPlanStepState, ApprovalDecisionKind, IntentContext,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Assistant WS".into()))
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
fn case1_user_submits_goal_creates_governed_workflow() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(
        workflow.state,
        AiAssistantWorkflowState::AwaitingConfirmation
    );
    assert!(workflow.orchestrated_plan_id.is_some());
    assert!(workflow.plan_preview.is_some());
    assert_eq!(workflow.plan_preview.as_ref().unwrap().actions.len(), 2);

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.goal_received"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.plan_presented"));
}

#[test]
fn case2_plan_generated_without_execution_before_confirm() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Set up my work environment",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(
        workflow.state,
        AiAssistantWorkflowState::AwaitingConfirmation
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == workspace_domain::ActorType::AIAssistant
    }));
    assert!(!records
        .iter()
        .any(|r| r.event_type == "permission.approval_required"));
}

#[test]
fn case3_user_cancel_prevents_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Help organize my applications",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    let cancelled =
        CommandHandler::cancel_assistant_workflow(&kernel, workflow.id.to_string()).unwrap();
    assert_eq!(cancelled.state, AiAssistantWorkflowState::Cancelled);

    let err = CommandHandler::confirm_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    );
    assert!(err.is_err());

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.cancelled"));
}

#[test]
fn case4_approved_workflow_passes_through_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

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
    assert_eq!(
        confirmed.state,
        AiAssistantWorkflowState::WaitingForPermission
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.user_confirmed"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "permission.approval_required"));

    let plan_id = confirmed.orchestrated_plan_id.as_ref().unwrap().to_string();
    let plan = CommandHandler::get_orchestrated_ai_plan(&kernel, plan_id).unwrap();
    let approval_id = plan.steps[0]
        .approval_request_id
        .clone()
        .expect("approval id");

    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        workspace_domain::PermissionApprovalRequestId::new(approval_id).unwrap(),
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    let done = CommandHandler::resume_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();
    assert_eq!(done.state, AiAssistantWorkflowState::Completed);
}

#[test]
fn case5_denied_permission_reported_by_assistant() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

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
        .expect("approval id");

    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        workspace_domain::PermissionApprovalRequestId::new(approval_id).unwrap(),
        ApprovalDecisionKind::Deny,
    ))
    .unwrap();

    let failed = CommandHandler::resume_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();

    assert_eq!(failed.state, AiAssistantWorkflowState::Failed);
    assert!(failed.status_message.to_lowercase().contains("denied")
        || failed.status_message.to_lowercase().contains("blocked"));

    let plan = CommandHandler::get_orchestrated_ai_plan(
        &kernel,
        failed.orchestrated_plan_id.as_ref().unwrap().to_string(),
    )
    .unwrap();
    assert_eq!(plan.steps[0].state, AiPlanStepState::Denied);
}

#[test]
fn case6_assistant_cannot_bypass_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let caps = workspace_domain::CapabilitySet::for_actor_type(
        workspace_domain::ActorType::AIAssistant,
    );
    assert_eq!(caps.iter().count(), 0);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    // Confirm still hits gateway — cannot complete without human approval.
    let confirmed = CommandHandler::confirm_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();
    assert_ne!(confirmed.state, AiAssistantWorkflowState::Completed);
    assert_eq!(
        confirmed.state,
        AiAssistantWorkflowState::WaitingForPermission
    );
}
