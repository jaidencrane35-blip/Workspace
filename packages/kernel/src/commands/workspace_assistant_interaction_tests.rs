//! Programme IV Batch 15 — Workspace Assistant Interaction Intelligence contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_assistant_interaction, ActorContext, AssistantSurfaceScope,
    CapabilitySet, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceAssistantInteractionService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Assistant Interaction WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

#[test]
fn package_interaction_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "how is interaction flow packaged?".into(),
    )
    .unwrap();
    assert!(projection.is_non_commandable());
    assert!(recovery_must_not_fabricate_assistant_interaction(&projection));
    let current = projection.current.as_ref().unwrap();
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn unavailable_gaps_are_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().any(|g| {
        g.gap_kind == "unavailable_package"
            || g.gap_kind == "missing_context"
            || g.gap_kind == "missing_surface"
            || g.gap_kind == "missing_retrieval"
            || g.gap_kind == "missing_explanation"
            || g.gap_kind == "incomplete_flow"
    }));
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn supersede_history_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "first ask".into(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().interaction_id.clone();
    let second = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        actor,
        intent,
        ws,
        "second ask".into(),
    )
    .unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().interaction_id,
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
            crate::commands::workspace_assistant_interaction::PackageWorkspaceAssistantInteraction::new(
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
    let source = include_str!("../services/workspace_assistant_interaction.rs");
    assert!(!source.contains("::generate("));
    assert!(!source.contains("Service::generate"));
    assert!(source.contains("WorkspaceAssistantSurfaceService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantContextService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantRetrievalService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantExplanationService::load_snapshot"));
    assert!(!source.contains("WorkspaceAssistantExplanationService::package_explanation"));
    assert!(!source.contains("WorkspaceAssistantRetrievalService::package_retrieval"));
    assert!(!source.contains("WorkspaceAssistantContextService::generate"));
    assert!(!source.contains("WorkspaceAssistantSurfaceService::compose"));
}

#[test]
fn explain_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "ask".into(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_assistant_interaction(
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
    let before = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "before".into(),
    )
    .unwrap();
    WorkspaceAssistantInteractionService::package_interaction_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        "rollback ask",
        AssistantSurfaceScope::interaction_default(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_assistant_interaction(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().interaction_id,
        after.current.as_ref().unwrap().interaction_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn empty_ask_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_interaction(
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
    assert!(WorkspaceAssistantInteractionService::attempt_act_for_user().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_infer_intent().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_create_hidden_memory().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_execute().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_approve().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_decide().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_bypass_permission_gateway().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_autonomous_loop().is_err());
    assert!(WorkspaceAssistantInteractionService::attempt_create_commitment().is_err());
    assert!(CommandHandler::assistant_interaction_attempt_execute().is_err());
}

#[test]
fn narrative_has_no_agency_language() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "flow ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    let narrative = current.narrative.to_lowercase();
    assert!(!narrative.contains("i will handle"));
    assert!(!narrative.contains("acting on your behalf"));
    assert!(!narrative.contains("i commit"));
    assert!(!narrative.contains("i decided"));
    assert!(!narrative.contains("your goal is"));
    assert!(!narrative.contains("i remember that you"));
    assert!(narrative.contains("never acts for the user"));
    assert!(narrative.contains("coordinates conversation flow"));
}

#[test]
fn routed_package_refs_only_from_load_snapshot_currents() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_interaction(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    // Fresh workspace: no Batch 11–14 currents → no invented routed refs.
    assert!(current.route.routed_packages.is_empty());
    for step in &current.steps {
        if let Some(ref package_ref) = step.package_ref {
            panic!("must not invent package refs without load_snapshot currents: {package_ref}");
        }
    }
    assert!(!current.route.missing_packages.is_empty());
}

#[test]
fn no_execution_paths_in_service() {
    let source = include_str!("../services/workspace_assistant_interaction.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("DecisionEngineService"));
    assert!(!source.contains("WorkspaceRecommendationEngineService"));
}
