//! Governed multi-step AI plan orchestration tests (Sprints 56–57).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AiOrchestratedPlanState, AiPlanStepState, ApprovalDecisionKind, IntentContext,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Orchestration WS".into()))
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
fn case1_multi_step_plan_created_without_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-1",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(plan.state, AiOrchestratedPlanState::Proposed);
    assert_eq!(plan.steps.len(), 2);
    assert!(plan.steps.iter().all(|s| s.state == AiPlanStepState::Pending));

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records.iter().any(|r| r.event_type == "ai.plan.created"));
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == workspace_domain::ActorType::AIAssistant
    }));
}

#[test]
fn case2_plan_action_enters_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-2",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    let advanced = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert!(matches!(
        advanced.state,
        AiOrchestratedPlanState::AwaitingApproval
            | AiOrchestratedPlanState::PartiallyApproved
    ));
    assert_eq!(advanced.steps[0].state, AiPlanStepState::AwaitingApproval);

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "permission.approval_required"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.plan.action_started"));
}

#[test]
fn case3_denied_action_does_not_execute_later_steps() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-3",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    let paused = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    let approval_id = paused.steps[0]
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

    let after = CommandHandler::resume_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert_eq!(after.state, AiOrchestratedPlanState::Failed);
    assert_eq!(after.steps[0].state, AiPlanStepState::Denied);
    assert_eq!(after.steps[1].state, AiPlanStepState::Pending);
}

#[test]
fn case4_approval_required_pauses_plan_safely() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-4",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    let paused = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert_eq!(paused.state, AiOrchestratedPlanState::AwaitingApproval);
    assert!(paused.awaiting_approval_step_index().is_some());
    assert!(paused.next_runnable_step_index().is_none());

    // Advancing while paused must fail closed (no silent continue).
    let err = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    );
    assert!(err.is_err());
}

#[test]
fn case5_approved_actions_execute_via_normal_flow() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-5",
        "Prepare my coding workspace",
        vec![app_a],
        None,
    )
    .unwrap();

    let paused = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    let approval_id = paused.steps[0]
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

    let done = CommandHandler::resume_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert_eq!(done.state, AiOrchestratedPlanState::Completed);
    assert_eq!(done.steps[0].state, AiPlanStepState::Completed);

    let records = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.plan.action_completed"));
}

#[test]
fn case6_plan_cannot_bypass_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-6",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    // Create alone never grants AI launch capability.
    let caps = workspace_domain::CapabilitySet::for_actor_type(
        workspace_domain::ActorType::AIAssistant,
    );
    assert_eq!(caps.iter().count(), 0);

    let advanced = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    assert_ne!(advanced.steps[0].state, AiPlanStepState::Completed);
    assert!(
        advanced.steps[0].approval_request_id.is_some()
            || advanced.steps[0].state == AiPlanStepState::Denied
    );
}

#[test]
fn case7_failed_action_is_recorded() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-7",
        "Prepare my coding workspace",
        vec![app_a.clone()],
        None,
    )
    .unwrap();

    let paused = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    let approval_id = paused.steps[0]
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

    // Remove target after approval so resume hits execution failure (not a silent skip).
    CommandHandler::delete_application(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        app_a,
    )
    .unwrap();

    let failed = CommandHandler::resume_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert_eq!(failed.state, AiOrchestratedPlanState::Failed);
    assert_eq!(failed.steps[0].state, AiPlanStepState::Failed);

    let records = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(records.iter().any(|r| r.event_type == "ai.plan.action_failed"));
}

#[test]
fn partial_approval_continues_approved_stops_denied() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::create_orchestrated_ai_plan(
        &kernel,
        "ai-orch-partial",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    // Step 1: approval required → allow once → resume → completed
    let paused = CommandHandler::advance_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    let approval_1 = paused.steps[0]
        .approval_request_id
        .clone()
        .expect("step1 approval");
    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        workspace_domain::PermissionApprovalRequestId::new(approval_1).unwrap(),
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    let after_first = CommandHandler::resume_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();
    assert_eq!(after_first.steps[0].state, AiPlanStepState::Completed);
    // Resume auto-advances into step 2 → typically ApprovalRequired again.
    assert!(matches!(
        after_first.state,
        AiOrchestratedPlanState::AwaitingApproval
            | AiOrchestratedPlanState::PartiallyApproved
    ));
    assert_eq!(after_first.steps[1].state, AiPlanStepState::AwaitingApproval);

    let approval_2 = after_first.steps[1]
        .approval_request_id
        .clone()
        .expect("step2 approval");
    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        workspace_domain::PermissionApprovalRequestId::new(approval_2).unwrap(),
        ApprovalDecisionKind::Deny,
    ))
    .unwrap();

    let final_plan = CommandHandler::resume_orchestrated_ai_plan_simulated(
        &kernel,
        plan.id.to_string(),
    )
    .unwrap();

    assert_eq!(final_plan.steps[0].state, AiPlanStepState::Completed);
    assert_eq!(final_plan.steps[1].state, AiPlanStepState::Denied);
    assert_eq!(final_plan.state, AiOrchestratedPlanState::Failed);
}
