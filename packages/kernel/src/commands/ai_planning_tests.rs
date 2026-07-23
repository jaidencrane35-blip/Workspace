//! AI planning foundation tests (Sprints 48–49).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{AiPlanningService, AuditService};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, AiProposalAuthorityOutcome, ApprovalDecisionKind, IntentContext,
    IntentType,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Plan WS".into()))
        .unwrap();

    let app_a = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap();
    let app_b = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Calculator".into(),
            None,
            Some("calc.exe".into()),
        ))
        .unwrap();

    (
        workspace.id.to_string(),
        app_a.id.to_string(),
        app_b.id.to_string(),
    )
}

#[test]
fn case1_plan_creates_proposals_without_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let plan = CommandHandler::plan_ai_goal(
        &kernel,
        "ai-planner-1",
        "Prepare my workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(plan.goal.statement, "Prepare my workspace");
    assert_eq!(plan.proposals.len(), 2);
    assert!(plan
        .proposals
        .iter()
        .all(|p| p.command_name == "LaunchApplication"));

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.plan_created"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.proposal_created"));
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == ActorType::AIAssistant
    }));
}

#[test]
fn case2_proposal_becomes_action_request_on_governance_path() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let result = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "ai-planner-2",
        "Prepare my workspace",
        vec![app_a],
    )
    .unwrap();

    assert_eq!(result.submissions.len(), 1);
    let submission = &result.submissions[0];
    assert_eq!(submission.request.command_name, "LaunchApplication");
    assert_eq!(
        submission.request.goal_id.as_deref(),
        Some(result.plan.goal.id.as_str())
    );
    assert_eq!(
        submission.request.proposal_id.as_deref(),
        Some(submission.proposal.id.as_str())
    );
    assert!(matches!(
        submission.outcome,
        AiProposalAuthorityOutcome::ApprovalRequired { .. }
    ));

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "permission.approval_required"
            && r.actor_type == ActorType::AIAssistant
            && r.intent_type == Some(IntentType::AISuggestion)
    }));
}

#[test]
fn case3_unauthorized_plan_submission_is_not_retried() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let result = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "ai-planner-3",
        "Prepare my workspace",
        vec![app_a, app_b],
    )
    .unwrap();

    assert_eq!(result.submissions.len(), 2);
    for submission in &result.submissions {
        assert!(matches!(
            submission.outcome,
            AiProposalAuthorityOutcome::ApprovalRequired { .. }
                | AiProposalAuthorityOutcome::Denied { .. }
        ));
    }
}

#[test]
fn case4_approved_proposal_executes_via_existing_flow() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);

    let result = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "ai-planner-4",
        "Prepare my workspace",
        vec![app_a.clone()],
    )
    .unwrap();

    let AiProposalAuthorityOutcome::ApprovalRequired {
        approval_request_id,
        ..
    } = &result.submissions[0].outcome
    else {
        panic!("expected ApprovalRequired");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id.clone()).unwrap();
    CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        request_id,
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    let launched = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        "ai-planner-4",
        app_a,
        Some("retry after allow-once".into()),
    )
    .unwrap();
    assert_eq!(launched.name, "Notepad");
}

#[test]
fn case5_planner_cannot_bypass_via_direct_launch_service() {
    // Architecture seal: ApplicationLaunchService is crate-internal; planning
    // only exposes plan/submit helpers that enter the pipeline.
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);
    let plan = AiPlanningService::plan_prepare_workspace(
        "ai-planner-5",
        "Prepare my workspace",
        vec![workspace_domain::ApplicationId::new(app_a).unwrap()],
        None,
        None,
    )
    .unwrap();

    assert!(!plan.proposals.is_empty());
    // Submitting with wrong intent motivation is rejected before execution.
    let proposal = &plan.proposals[0];
    let request = proposal
        .to_action_request(&plan.goal.requesting_actor_id)
        .unwrap();
    let ctx = kernel.command_context(
        ActorContext::new(
            workspace_domain::Actor::ai_assistant("ai-planner-5").unwrap(),
        ),
        IntentContext::user_request(),
    );
    let error = crate::services::AiParticipationService::ensure_ai_submission_context(
        &ctx, &request,
    )
    .unwrap_err();
    assert!(matches!(error, KernelError::AiRequestValidation { .. }));
}

#[test]
fn batch3_context_awareness_skips_already_active_apps() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, _app_a, _app_b) = seed_two_apps(&kernel);

    let without_active = CommandHandler::plan_ai_goal_with_environment(
        &kernel,
        "ai-context-1",
        "Prepare my workspace",
        Vec::new(),
        Some(workspace_id.clone()),
        vec![],
    )
    .unwrap();
    assert_eq!(without_active.proposals.len(), 2);

    let with_notepad_active = CommandHandler::plan_ai_goal_with_environment(
        &kernel,
        "ai-context-2",
        "Prepare my workspace",
        Vec::new(),
        Some(workspace_id),
        vec!["Untitled - Notepad".into()],
    )
    .unwrap();
    assert_eq!(with_notepad_active.proposals.len(), 1);
    assert!(with_notepad_active.proposals[0]
        .explanation
        .as_deref()
        .unwrap_or("")
        .contains("Calculator"));

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.planning.awareness_used"));
}

#[test]
fn batch3_context_aware_plan_still_requires_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (workspace_id, _, _) = seed_two_apps(&kernel);

    let result = CommandHandler::submit_ai_plan_simulated_with_workspace(
        &kernel,
        "ai-context-3",
        "Prepare my workspace",
        workspace_id,
        vec![],
    )
    .unwrap();

    assert!(!result.submissions.is_empty());
    for submission in &result.submissions {
        assert!(matches!(
            submission.outcome,
            AiProposalAuthorityOutcome::ApprovalRequired { .. }
        ));
    }
}
