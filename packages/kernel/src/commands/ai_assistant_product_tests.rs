//! Product assistant experience tests (Sprints 66–67 / Phase 4 Batch 4).
//!
//! Same governed pipeline as diagnostics — revise, compare, cancel, explain blocks,
//! preference/memory influence, no second authority path.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AiAssistantWorkflowState, ApprovalDecisionKind, IntentContext, MemoryType,
    PreferenceCategory, PreferenceSource,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Product Assistant WS".into()))
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

fn create_default_editor(
    kernel: &WorkspaceKernel,
    workspace_id: &str,
    app_id: &str,
    label: &str,
) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_user_preference(
        kernel,
        local,
        intent,
        PreferenceCategory::Application,
        "default_editor",
        format!("{label} is my default editor"),
        PreferenceSource::UserDefined,
        Some(workspace_id.to_string()),
        Some(label.to_string()),
        Some(format!(r#"{{"application_id":"{app_id}"}}"#)),
    )
    .unwrap();
}

#[test]
fn case1_user_revises_goal_plan_regenerates() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        None,
    )
    .unwrap();
    let first_plan = workflow.orchestrated_plan_id.clone();

    let revised = CommandHandler::revise_assistant_goal(
        &kernel,
        workflow.id.to_string(),
        "Prepare a lighter workspace",
        None,
        None,
    )
    .unwrap();

    assert_eq!(revised.user_goal, "Prepare a lighter workspace");
    assert_eq!(
        revised.state,
        AiAssistantWorkflowState::AwaitingConfirmation
    );
    assert_eq!(revised.plan_revisions.len(), 1);
    assert_ne!(revised.orchestrated_plan_id, first_plan);
    assert!(revised.plan_preview.is_some());

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.goal_updated"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.plan_regenerated"));
}

#[test]
fn case2_user_compares_plans_differences_explained() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        None,
    )
    .unwrap();

    let regenerated = CommandHandler::revise_assistant_goal(
        &kernel,
        workflow.id.to_string(),
        "Organize apps for focus work",
        None,
        None,
    )
    .unwrap();
    assert!(!regenerated.plan_revisions.is_empty());

    let left = regenerated.plan_revisions[0].revision;
    let comparison = CommandHandler::compare_assistant_plan_revisions(
        &kernel,
        regenerated.id.to_string(),
        Some(left),
        None,
    )
    .unwrap();

    assert!(!comparison.differences.is_empty());
    assert!(comparison.differences.iter().any(|d| {
        d.contains("Goal changed") || d.contains("Actions differ") || d.contains("Action count")
    }));

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.plan_compared"));
}

#[test]
fn case3_user_cancels_midway_execution_stops() {
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

    let cancelled =
        CommandHandler::cancel_assistant_workflow(&kernel, workflow.id.to_string()).unwrap();
    assert_eq!(cancelled.state, AiAssistantWorkflowState::Cancelled);

    let resume = CommandHandler::resume_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    );
    // Linked plan cancelled — resume should not complete the workflow.
    if let Ok(after) = resume {
        assert_ne!(after.state, AiAssistantWorkflowState::Completed);
    }

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.cancelled"));
}

#[test]
fn case4_permission_denied_assistant_explains_block() {
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

    let preview = workflow.plan_preview.as_ref().unwrap();
    let explanation = preview.actions[0]
        .structured_explanation
        .as_ref()
        .expect("structured explanation");
    assert!(!explanation.why_permission.is_empty());
    assert!(!explanation.what_if_approve.is_empty());

    CommandHandler::record_assistant_explanation_viewed(
        &kernel,
        workflow.id.to_string(),
        preview.actions[0].step_id.clone(),
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
    assert!(
        failed.status_message.to_lowercase().contains("denied")
            || failed.status_message.to_lowercase().contains("blocked")
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.assistant.explanation_viewed"));
    assert!(records.iter().any(|r| {
        r.event_type == "ai.assistant.explanation_viewed"
            && r.metadata
                .as_deref()
                .is_some_and(|m| m.contains("\"authority_effect\":\"none\""))
    }));
}

#[test]
fn case5_preferences_disabled_neutral_planning() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let with_prefs = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        Some(ws.clone()),
    )
    .unwrap();
    let with_text = with_prefs
        .plan_preview
        .as_ref()
        .unwrap()
        .actions
        .iter()
        .filter_map(|a| a.explanation.clone())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        with_text.to_lowercase().contains("preferred because")
            || with_text.to_lowercase().contains("default editor"),
        "expected preference influence: {with_text}"
    );

    CommandHandler::set_personalization_enabled(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        false,
    )
    .unwrap();

    let neutral = CommandHandler::regenerate_assistant_plan(
        &kernel,
        with_prefs.id.to_string(),
        None,
        Some(ws),
    )
    .unwrap();
    let neutral_text = neutral
        .plan_preview
        .as_ref()
        .unwrap()
        .actions
        .iter()
        .filter_map(|a| a.explanation.clone())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !neutral_text.to_lowercase().contains("preferred because"),
        "disabled personalization should yield neutral planning: {neutral_text}"
    );
}

#[test]
fn case6_memory_removed_planning_updates() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let entry = CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "preferred_application",
        "User prefers VS Code for coding workspaces",
        "product_assistant_test",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let with_memory = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        Some(ws.clone()),
    )
    .unwrap();
    let first = with_memory
        .plan_preview
        .as_ref()
        .unwrap()
        .actions
        .iter()
        .filter_map(|a| a.explanation.clone())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        first.contains("Memory:") || first.contains("preferred from memory"),
        "expected memory influence: {first}"
    );

    CommandHandler::delete_memory_entry(&kernel, local, intent, entry.id.to_string()).unwrap();

    let after = CommandHandler::regenerate_assistant_plan(
        &kernel,
        with_memory.id.to_string(),
        None,
        Some(ws),
    )
    .unwrap();
    let second = after
        .plan_preview
        .as_ref()
        .unwrap()
        .actions
        .iter()
        .filter_map(|a| a.explanation.clone())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !second.contains("Memory:") && !second.contains("preferred from memory"),
        "removed memory should not influence regenerated plan: {second}"
    );
}

#[test]
fn case7_product_and_diagnostics_identical_governed_behavior() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    // Product path and diagnostic path both call the same CommandHandler APIs.
    let product = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        None,
    )
    .unwrap();
    let diagnostic = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert_eq!(product.state, diagnostic.state);
    assert_eq!(
        product.plan_preview.as_ref().map(|p| p.actions.len()),
        diagnostic.plan_preview.as_ref().map(|p| p.actions.len())
    );
    assert_eq!(
        product
            .plan_preview
            .as_ref()
            .and_then(|p| p.actions.first())
            .and_then(|a| a.structured_explanation.as_ref())
            .map(|e| e.why_permission.clone()),
        diagnostic
            .plan_preview
            .as_ref()
            .and_then(|p| p.actions.first())
            .and_then(|a| a.structured_explanation.as_ref())
            .map(|e| e.why_permission.clone())
    );

    let product_regen = CommandHandler::regenerate_assistant_plan(
        &kernel,
        product.id.to_string(),
        None,
        None,
    )
    .unwrap();
    let diagnostic_regen = CommandHandler::regenerate_assistant_plan(
        &kernel,
        diagnostic.id.to_string(),
        None,
        None,
    )
    .unwrap();
    assert_eq!(product_regen.state, diagnostic_regen.state);
    assert_eq!(
        product_regen.state,
        AiAssistantWorkflowState::AwaitingConfirmation
    );

    // Confirming still requires Permission Gateway — neither path completes alone.
    let product_confirmed = CommandHandler::confirm_assistant_workflow_simulated(
        &kernel,
        product.id.to_string(),
    )
    .unwrap();
    assert_eq!(
        product_confirmed.state,
        AiAssistantWorkflowState::WaitingForPermission
    );
}
