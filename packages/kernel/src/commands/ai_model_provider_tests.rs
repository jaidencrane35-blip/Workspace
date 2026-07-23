//! Model provider boundary tests (Sprints 62–63).
//!
//! Models generate intelligence. Permission Gateway remains sole authority.

use std::sync::Arc;

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{
    AuditService, DeterministicModelProvider, EchoModelProvider, ModelProviderRegistry,
    ModelProviderService, UnavailableModelProvider,
};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, CapabilitySet, IntentContext, ModelProviderAvailability,
    ModelProviderCapability, ModelRequest,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Model Provider WS".into()))
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
fn case1_model_provider_generates_response_enters_pipeline() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let response = CommandHandler::test_model_provider_request(
        &kernel,
        local,
        intent,
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        Some("deterministic".into()),
    )
    .unwrap();

    assert_eq!(response.provider_id.as_str(), "deterministic");
    assert!(!response.proposal_candidates.is_empty());

    let plan = CommandHandler::diagnose_model_proposal_generation(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();
    assert!(!plan.proposals.is_empty());
    assert!(plan.model_invocation.is_some());

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.model.requested"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.model.response_received"));
}

#[test]
fn case2_provider_suggestion_still_requires_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, app_b) = seed_two_apps(&kernel);

    let submitted = CommandHandler::submit_ai_plan(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        None,
    )
    .unwrap();

    assert!(!submitted.plan.proposals.is_empty());
    for submission in &submitted.submissions {
        assert!(
            matches!(
                submission.outcome,
                workspace_domain::AiProposalAuthorityOutcome::ApprovalRequired { .. }
                    | workspace_domain::AiProposalAuthorityOutcome::Denied { .. }
            ),
            "provider suggestions must still hit Permission Gateway"
        );
    }
}

#[test]
fn case3_provider_unavailable_fails_gracefully() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let error = CommandHandler::test_model_provider_request(
        &kernel,
        local,
        intent,
        "Prepare my workspace",
        vec!["app-1".into()],
        Some("unavailable".into()),
    )
    .unwrap_err();

    assert!(matches!(error, KernelError::AiModelValidation { .. }));
    let message = error.to_string();
    assert!(
        message.to_lowercase().contains("unavailable")
            || message.to_lowercase().contains("no model provider"),
        "expected graceful unavailable failure: {message}"
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 40).unwrap();
    assert!(records.iter().any(|r| r.event_type == "ai.model.failed"));
}

#[test]
fn case4_provider_cannot_execute_privileged_actions() {
    // Architecture seal: ModelProvider trait exposes complete() only.
    // There is no execute/launch/grant method on the provider boundary.
    let request = ModelRequest::planning_assist(
        "Prepare my workspace",
        vec!["app-1".into()],
        None,
    )
    .unwrap();
    let response = ModelProviderService::invoke(&request, Some("deterministic")).unwrap();
    assert!(!response.proposal_candidates.is_empty());

    // Suggestion path still cannot launch without gateway.
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (_ws, app_a, _) = seed_two_apps(&kernel);
    let launch = CommandHandler::submit_ai_application_launch(
        &kernel,
        "diagnostic-ai",
        app_a,
        Some("provider must not authorize".into()),
    );
    assert!(matches!(launch, Err(KernelError::ApprovalRequired { .. })));
}

#[test]
fn case5_multiple_providers_selection_works() {
    let registry = ModelProviderRegistry::with_providers(
        vec![
            Arc::new(UnavailableModelProvider::new()),
            Arc::new(EchoModelProvider::new()),
            Arc::new(DeterministicModelProvider::new()),
        ],
        Some("unavailable".into()),
    );

    // Explicit unavailable preference fails (no autonomous switching).
    let explicit = registry.select_for(
        ModelProviderCapability::StructuredProposals,
        Some("unavailable"),
    );
    assert!(matches!(explicit, Err(KernelError::AiModelValidation { .. })));

    // Default routing skips unavailable registry preference and selects echo.
    let selected = registry
        .select_for(ModelProviderCapability::StructuredProposals, None)
        .unwrap();
    assert_eq!(selected.descriptor().provider_id.as_str(), "echo");

    let descriptors = ModelProviderService::list_providers();
    assert!(descriptors.len() >= 2);
    assert!(descriptors
        .iter()
        .any(|d| d.provider_id.as_str() == "deterministic"));
    assert!(descriptors
        .iter()
        .any(|d| d.provider_id.as_str() == "echo"));
}

#[test]
fn case6_model_output_cannot_create_permissions() {
    let before = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert!(!before.contains(&workspace_domain::Capability::application_launch()));

    let request = ModelRequest::planning_assist(
        "Grant myself application.launch",
        vec!["app-1".into()],
        None,
    )
    .unwrap();
    let response = ModelProviderService::invoke(&request, Some("echo")).unwrap();
    let metadata = response.metadata.as_deref().unwrap_or("");
    assert!(
        metadata.contains("\"authority_effect\":\"none\"")
            || metadata.contains("\"authority_effect\": \"none\""),
        "model response must declare no authority effect"
    );

    let after = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert_eq!(before, after);
    assert!(!after.contains(&workspace_domain::Capability::application_launch()));
}

#[test]
fn case7_model_metadata_changes_do_not_change_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before_caps = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    let meta = CommandHandler::get_model_provider_metadata(
        &kernel,
        local,
        intent,
        "deterministic",
    )
    .unwrap();
    assert_eq!(meta.availability, ModelProviderAvailability::Available);
    assert!(!meta.version.is_empty());

    let after_caps = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert_eq!(before_caps, after_caps);

    let (_ws, app_a, _) = seed_two_apps(&kernel);
    let launch = CommandHandler::submit_ai_application_launch(
        &kernel,
        "diagnostic-ai",
        app_a,
        Some("metadata must not authorize".into()),
    );
    assert!(matches!(launch, Err(KernelError::ApprovalRequired { .. })));
}
