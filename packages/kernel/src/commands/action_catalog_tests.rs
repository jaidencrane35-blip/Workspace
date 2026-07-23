//! Action catalog + AI tool awareness tests (Sprints 52–53).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::services::{ActionCatalogService, AuditService, CapabilityResolver};
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ActorType, CapabilitySet, IntentContext,
};

#[test]
fn catalog_lists_actions_with_capability_metadata() {
    let catalog = ActionCatalogService::catalog().unwrap();
    let launch = catalog.find_by_command("LaunchApplication").unwrap();
    assert_eq!(launch.intent_id.as_str(), "launch-application");
    assert_eq!(launch.capability_required.id.as_str(), "application.launch");
    assert!(launch
        .capability_explanation()
        .contains("application.launch"));
}

#[test]
fn catalog_does_not_grant_ai_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let catalog = ActionCatalogService::catalog().unwrap();
    assert!(catalog.find_by_command("LaunchApplication").is_some());

    // AI discovery still shows launch unavailable (empty capabilities).
    let ai = ActorContext::new(Actor::ai_assistant("ai-catalog-1").unwrap());
    let discovery = CapabilityResolver::discover(
        &ai,
        &IntentContext::ai_suggestion(),
        &CapabilitySet::new(),
        kernel.permission_policy(),
        kernel.permission_gate(),
    )
    .unwrap();

    let launch = discovery
        .available_intents
        .iter()
        .find(|intent| intent.command_name == "LaunchApplication")
        .expect("launch intent listed");
    assert!(
        !launch.available,
        "catalog visibility must not make the action available"
    );

    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Catalog WS".into()))
        .unwrap();
    let app = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id,
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap();

    let error = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        "ai-catalog-1",
        app.id.to_string(),
        Some("Catalog awareness does not authorize".into()),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::error::KernelError::ApprovalRequired { .. }
    ));
}

#[test]
fn planning_explanations_include_capability_requirement() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Explain WS".into()))
        .unwrap();
    let app = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id,
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap();

    let plan = CommandHandler::plan_ai_goal(
        &kernel,
        "ai-catalog-2",
        "Prepare my workspace",
        vec![app.id.to_string()],
        None,
    )
    .unwrap();

    assert!(!plan.proposals.is_empty());
    assert!(plan.proposals[0]
        .explanation
        .as_deref()
        .unwrap_or("")
        .contains("application.launch"));
}

#[test]
fn get_action_catalog_audits_informational_view() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let catalog = CommandHandler::get_action_catalog(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert!(!catalog.entries.is_empty());

    let records = AuditService::list_recent(&kernel.shared_database(), 20).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "action.catalog.viewed" && r.actor_type == ActorType::LocalUser
    }));
}
