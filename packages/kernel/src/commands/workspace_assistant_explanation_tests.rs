//! Programme IV Batch 14 — Workspace Assistant Explanation Intelligence contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_assistant_explanation, ActorContext, AssistantSurfaceScope,
    CapabilitySet, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceAssistantExplanationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Assistant Explanation WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

#[test]
fn package_explanation_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "what does recorded evidence say?".into(),
    )
    .unwrap();
    assert!(projection.is_non_commandable());
    assert!(recovery_must_not_fabricate_assistant_explanation(&projection));
    let current = projection.current.as_ref().unwrap();
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn unavailable_gaps_are_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_explanation(
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
        g.gap_kind == "unavailable_upstream" || g.gap_kind == "missing_explanation_layer"
    }));
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn supersede_history_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "first ask".into(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().explanation_id.clone();
    let second = CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        actor,
        intent,
        ws,
        "second ask".into(),
    )
    .unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().explanation_id,
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
            crate::commands::workspace_assistant_explanation::PackageWorkspaceAssistantExplanation::new(
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
    let source = include_str!("../services/workspace_assistant_explanation.rs");
    assert!(!source.contains("::generate("));
    assert!(!source.contains("Service::generate"));
    assert!(source.contains("WorkspaceExplanationService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantRetrievalService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantContextService::load_snapshot"));
    assert!(source.contains("WorkspaceAssistantSurfaceService::load_snapshot"));
    assert!(source.contains("WorkspaceEvidenceTraceService::load_snapshot"));
    assert!(source.contains("WorkspaceEvidenceNavigationService::load_snapshot"));
    assert!(!source.contains("WorkspaceExplanationService::generate"));
}

#[test]
fn explain_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "ask".into(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_assistant_explanation(
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
    let before = CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "before".into(),
    )
    .unwrap();
    WorkspaceAssistantExplanationService::package_explanation_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        "rollback ask",
        AssistantSurfaceScope::explanation_default(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_assistant_explanation(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().explanation_id,
        after.current.as_ref().unwrap().explanation_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn empty_ask_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_explanation(
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
    assert!(WorkspaceAssistantExplanationService::attempt_conclude_truth().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_invent_cause().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_recommend().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_decide().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_execute().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_bypass_permission_gateway().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_replace_explanation_layer().is_err());
    assert!(WorkspaceAssistantExplanationService::attempt_hide_uncertainty().is_err());
    assert!(CommandHandler::assistant_explanation_attempt_execute().is_err());
}

#[test]
fn narrative_has_no_conclusion_language() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_explanation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "clarity ask".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    let narrative = current.narrative.to_lowercase();
    assert!(!narrative.contains("therefore this is true"));
    assert!(!narrative.contains("this happened because"));
    assert!(!narrative.contains("i conclude"));
    assert!(!narrative.contains("definitely true"));
    assert!(narrative.contains("never concludes"));
    assert!(narrative.contains("clarifies recorded"));
}

#[test]
fn no_execution_paths_in_service() {
    let source = include_str!("../services/workspace_assistant_explanation.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("DecisionEngineService"));
    assert!(!source.contains("WorkspaceRecommendationEngineService"));
}
