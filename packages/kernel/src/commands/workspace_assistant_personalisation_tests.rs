//! Programme IV Batch 16 — Workspace Assistant Personalisation Boundary contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_assistant_personalisation, ActorContext, AssistantSurfaceScope,
    CapabilitySet, IntentContext, PreferenceCategory, PreferenceSource,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceAssistantPersonalisationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Assistant Personalisation WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

#[test]
fn package_personalisation_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "how are presentation preferences packaged?".into(),
    )
    .unwrap();
    assert!(projection.is_non_commandable());
    assert!(recovery_must_not_fabricate_assistant_personalisation(&projection));
    let current = projection.current.as_ref().unwrap();
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn disabled_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::set_personalization_enabled(&kernel, actor.clone(), intent.clone(), false)
        .unwrap();
    CommandHandler::create_user_preference(
        &kernel,
        actor.clone(),
        intent.clone(),
        PreferenceCategory::Communication,
        "citation_density",
        "concise",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        None,
        None,
    )
    .unwrap();
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        actor,
        intent,
        ws,
        "ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current
        .gaps
        .iter()
        .any(|g| g.gap_kind == "personalization_disabled"));
    assert!(!current.adaptation.personalization_enabled);
    assert!(current.adaptation.applied_preference_refs.is_empty());
}

#[test]
fn no_prefs_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current
        .gaps
        .iter()
        .any(|g| g.gap_kind == "no_preferences_recorded"));
    assert!(current.adaptation.applied_preference_refs.is_empty());
}

#[test]
fn supersede_history_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "first ask".into(),
    )
    .unwrap();
    let first_id = first
        .current
        .as_ref()
        .unwrap()
        .personalisation_id
        .clone();
    let second = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        actor,
        intent,
        ws,
        "second ask".into(),
    )
    .unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().personalisation_id,
        first_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn capability_deny_without_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx = kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_assistant_personalisation::PackageWorkspaceAssistantPersonalisation::new(
                ws,
                "ask",
            ),
        )
        .expect_err("empty capabilities must deny");
    let message = err.to_string().to_lowercase();
    assert!(
        message.contains("denied")
            || message.contains("permission")
            || message.contains("capability")
            || message.contains("not granted"),
        "got {err}"
    );
}

#[test]
fn service_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_assistant_personalisation.rs");
    assert!(!source.contains("::generate("));
    assert!(!source.contains("Service::generate"));
    assert!(!source.contains("WorkspaceWorkingStyleService::generate"));
    assert!(source.contains("WorkspaceAssistantSurfaceService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantContextService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantRetrievalService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantExplanationService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantInteractionService::load_snapshot"));
    assert!(!source.contains("WorkspaceAssistantInteractionService::package_interaction"));
    assert!(!source.contains("WorkspaceAssistantExplanationService::package_explanation"));
    assert!(!source.contains("WorkspaceAssistantRetrievalService::package_retrieval"));
    assert!(!source.contains("WorkspaceAssistantContextService::generate"));
    assert!(!source.contains("WorkspaceAssistantSurfaceService::compose"));
}

#[test]
fn service_never_calls_preference_write_methods() {
    let source = include_str!("../services/workspace_assistant_personalisation.rs");
    assert!(!source.contains("AiPersonalizationService::create("));
    assert!(!source.contains("AiPersonalizationService::update("));
    assert!(!source.contains("AiPersonalizationService::delete("));
    assert!(!source.contains("AiPersonalizationService::set_enabled("));
    assert!(source.contains("AiPersonalizationService::is_enabled"));
    assert!(source.contains("AiPersonalizationService::get_profile"));
}

#[test]
fn explain_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "ask".into(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!explanation.actionable);
    assert_eq!(explanation.authority_effect, "none");
}

#[test]
fn forced_rollback_leaves_current_unchanged() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "before".into(),
    )
    .unwrap();
    WorkspaceAssistantPersonalisationService::package_personalisation_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        "rollback ask",
        AssistantSurfaceScope::personalisation_default(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_assistant_personalisation(&kernel, actor, intent, ws)
            .unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().personalisation_id,
        after.current.as_ref().unwrap().personalisation_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn empty_ask_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "   ".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current.gaps.iter().any(|g| g.gap_kind == "empty_ask"));
}

#[test]
fn negative_authority_guards() {
    assert!(WorkspaceAssistantPersonalisationService::attempt_infer_identity().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_infer_personality().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_infer_preference().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_write_preference().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_create_hidden_profile().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_autonomous_adapt().is_err());
    assert!(
        WorkspaceAssistantPersonalisationService::attempt_bypass_permission_gateway().is_err()
    );
    assert!(WorkspaceAssistantPersonalisationService::attempt_influence_permission().is_err());
    assert!(WorkspaceAssistantPersonalisationService::attempt_execute().is_err());
    assert!(CommandHandler::assistant_personalisation_attempt_execute().is_err());
}

#[test]
fn narrative_has_no_inference_language() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "prefs ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    let narrative = current.narrative.to_lowercase();
    assert!(!narrative.contains("you are the kind of"));
    assert!(!narrative.contains("your personality"));
    assert!(!narrative.contains("i inferred"));
    assert!(!narrative.contains("based on your behaviour"));
    assert!(!narrative.contains("based on your behavior"));
    assert!(!narrative.contains("i learned that you"));
    assert!(!narrative.contains("secretly"));
    assert!(narrative.contains("never invents who the user is"));
    assert!(narrative.contains("adapts presentation from explicit preferences only"));
}

#[test]
fn items_only_from_get_profile_list_active() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let pref = CommandHandler::create_user_preference(
        &kernel,
        actor.clone(),
        intent.clone(),
        PreferenceCategory::Communication,
        "citation_density",
        "concise",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        None,
        None,
    )
    .unwrap();
    let projection = CommandHandler::package_workspace_assistant_personalisation(
        &kernel,
        actor,
        intent,
        ws,
        "ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current
        .adaptation
        .applied_preference_refs
        .contains(&pref.id.as_str().to_string()));
    let explicit: Vec<_> = current
        .items
        .iter()
        .filter(|i| i.kind == "explicit_preference")
        .collect();
    assert_eq!(explicit.len(), 1);
    assert_eq!(
        explicit[0].preference_ref.as_deref(),
        Some(pref.id.as_str())
    );
    assert_eq!(explicit[0].source.as_deref(), Some("user_defined"));
    // Fresh workspace: no invented preference refs beyond get_profile results.
    assert_eq!(current.adaptation.applied_preference_refs.len(), 1);
}

#[test]
fn no_execution_paths_in_service() {
    let source = include_str!("../services/workspace_assistant_personalisation.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("DecisionEngineService"));
    assert!(!source.contains("WorkspaceRecommendationEngineService"));
}
