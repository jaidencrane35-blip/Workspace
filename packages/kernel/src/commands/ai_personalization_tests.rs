//! Governed personalization boundary tests (Sprints 64–65).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{AuditService, ModelProviderService};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, AiPlanningContext, AiPersonalizationAwareness, CapabilitySet,
    IntentContext, ModelRequest, PreferenceCategory, PreferenceSource, UserPreference,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Personalization WS".into()))
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
) -> UserPreference {
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
    .unwrap()
}

#[test]
fn case1_preferences_improve_proposal_ranking() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let plan = CommandHandler::diagnose_ai_plan_with_personalization(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b],
        Some(ws),
        true,
    )
    .unwrap();

    assert!(!plan.proposals.is_empty());
    let first = plan.proposals[0]
        .target_resource
        .as_ref()
        .map(|r| r.id.as_str().to_string());
    assert_eq!(first.as_deref(), Some(app_a.as_str()));
    let explanation = plan.proposals[0].explanation.clone().unwrap_or_default();
    assert!(
        explanation.contains("Preferred because you marked VS Code"),
        "expected personalization explanation: {explanation}"
    );
}

#[test]
fn case2_deleting_preference_removes_influence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let preference = create_default_editor(&kernel, &ws, &app_b, "Terminal");

    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::delete_user_preference(
        &kernel,
        local,
        intent,
        preference.id.to_string(),
    )
    .unwrap();

    let plan = CommandHandler::diagnose_ai_plan_with_personalization(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        Some(ws),
        true,
    )
    .unwrap();
    let joined = plan
        .proposals
        .iter()
        .map(|p| p.explanation.clone().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !joined.contains("Preferred because you marked Terminal"),
        "deleted preference must not influence planning: {joined}"
    );
}

#[test]
fn case3_disabling_personalization_restores_neutral_planning() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_b, "Terminal");

    let comparison = CommandHandler::compare_personalized_vs_neutral_plan(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b.clone()],
        Some(ws),
    )
    .unwrap();

    let personalized_first = comparison.personalized.proposals[0]
        .target_resource
        .as_ref()
        .map(|r| r.id.as_str().to_string());
    assert_eq!(personalized_first.as_deref(), Some(app_b.as_str()));

    let neutral_joined = comparison
        .neutral
        .proposals
        .iter()
        .map(|p| p.explanation.clone().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !neutral_joined.contains("Preferred because you marked Terminal"),
        "neutral plan must ignore preferences: {neutral_joined}"
    );
}

#[test]
fn case4_preferences_cannot_grant_permissions() {
    let before = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert!(!before.contains(&workspace_domain::Capability::application_launch()));

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let after = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert_eq!(before, after);
}

#[test]
fn case5_gateway_unchanged_with_personalization() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let submitted = CommandHandler::submit_ai_plan(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        Some(ws),
    )
    .unwrap();

    for submission in &submitted.submissions {
        assert!(matches!(
            submission.outcome,
            workspace_domain::AiProposalAuthorityOutcome::ApprovalRequired { .. }
                | workspace_domain::AiProposalAuthorityOutcome::Denied { .. }
        ));
    }
}

#[test]
fn case6_model_providers_receive_preference_context_but_cannot_mutate() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let preference = create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let awareness = AiPersonalizationAwareness::from_preferences(vec![preference], true);
    let goal = workspace_domain::AiGoal::new("Prepare my workspace", "diagnostic-ai").unwrap();
    let context = AiPlanningContext::new(
        goal,
        vec![
            workspace_domain::ApplicationId::new(app_a).unwrap(),
            workspace_domain::ApplicationId::new(app_b).unwrap(),
        ],
    )
    .with_personalization_awareness(awareness);
    let request = ModelRequest::from_planning_context(&context).unwrap();
    let summary = request.context_summary.clone().unwrap_or_default();
    assert!(
        summary.contains("personalization_enabled=true"),
        "provider request should include preference context: {summary}"
    );

    // Providers can complete; they have no preference mutation API.
    let response = ModelProviderService::invoke(&request, Some("deterministic")).unwrap();
    assert_eq!(response.status, workspace_domain::ModelResponseStatus::Success);

    // AI still cannot write preferences through the governed path.
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-pref-6").unwrap());
    let error = CommandHandler::create_user_preference(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        PreferenceCategory::Application,
        "default_editor",
        "AI trying to write preference",
        PreferenceSource::UserDefined,
        None,
        None,
        None,
    )
    .unwrap_err();
    // AI actor has empty caps in command_context — need StandardPermissionGate path.
    // CommandHandler uses kernel.command_context which applies actor's nominal caps.
    assert!(matches!(
        error,
        KernelError::ApprovalRequired { .. } | KernelError::PermissionDenied(_)
    ));
}

#[test]
fn case7_explanations_identify_applied_preferences() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    create_default_editor(&kernel, &ws, &app_a, "VS Code");

    let plan = CommandHandler::diagnose_ai_plan_with_personalization(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        Some(ws),
        true,
    )
    .unwrap();

    let explanation = plan.proposals[0].explanation.clone().unwrap_or_default();
    assert!(
        explanation.contains("Preferred because you marked VS Code as your default editor."),
        "explanation must identify applied preference: {explanation}"
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.personalization.created"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.personalization.used"));
}
