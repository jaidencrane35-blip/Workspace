//! Programme IV Batch 12 — Workspace Assistant Context Intelligence contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_assistant_context, ActorContext, AssistantSurfaceScope,
    CapabilitySet, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceAssistantContextService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Assistant Context WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn empty_scope() -> AssistantSurfaceScope {
    AssistantSurfaceScope {
        include_semantic_query: false,
        include_evidence_navigation: false,
        include_evidence_trace: false,
        include_evidence_coverage: false,
        include_evidence_consistency: false,
        include_evidence_dependency: false,
        include_evidence_freshness: false,
        include_evidence_completeness: false,
        include_evidence_reliability: false,
        include_explanation: false,
        include_contextual: false,
        include_knowledge_integration: false,
        include_intelligence_hub: false,
        include_state: false,
    }
}

#[test]
fn package_context_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(projection.is_non_commandable());
    assert!(recovery_must_not_fabricate_assistant_context(&projection));
    let current = projection.current.as_ref().unwrap();
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn unavailable_gaps_are_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::package_workspace_assistant_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(!current.gaps.is_empty());
    assert!(current
        .gaps
        .iter()
        .any(|g| g.gap_kind == "unavailable_upstream"));
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn supersede_history_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::package_workspace_assistant_context(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().context_id.clone();
    let second =
        CommandHandler::package_workspace_assistant_context(&kernel, actor, intent, ws).unwrap();
    assert_ne!(second.current.as_ref().unwrap().context_id, first_id);
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn forced_rollback_leaves_current_unchanged() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::package_workspace_assistant_context(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceAssistantContextService::package_context_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        AssistantSurfaceScope::presentation_default(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_assistant_context(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().context_id,
        after.current.as_ref().unwrap().context_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn capability_deny_without_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx = kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_assistant_context::PackageWorkspaceAssistantContext::new(
                ws,
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
fn negative_authority_guards() {
    assert!(WorkspaceAssistantContextService::attempt_execute().is_err());
    assert!(WorkspaceAssistantContextService::attempt_approve().is_err());
    assert!(WorkspaceAssistantContextService::attempt_decide().is_err());
    assert!(WorkspaceAssistantContextService::attempt_bypass_permission_gateway().is_err());
    assert!(WorkspaceAssistantContextService::attempt_create_hidden_memory().is_err());
    assert!(WorkspaceAssistantContextService::attempt_silent_mutate().is_err());
    assert!(WorkspaceAssistantContextService::attempt_infer_intent().is_err());
    assert!(WorkspaceAssistantContextService::attempt_fabricate_continuity().is_err());
    assert!(WorkspaceAssistantContextService::attempt_create_plan().is_err());
    assert!(CommandHandler::assistant_context_attempt_execute().is_err());
}

#[test]
fn explain_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    CommandHandler::package_workspace_assistant_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_assistant_context(
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
fn service_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_assistant_context.rs");
    assert!(!source.contains("::generate("));
    assert!(!source.contains("Service::generate"));
    assert!(source.contains("WorkspaceAssistantSurfaceService::load_snapshot"));
    assert!(!source.contains("WorkspaceAssistantSurfaceService::compose"));
}

#[test]
fn continuity_from_prior_surface_turns_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let surface = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "recorded turn for continuity".into(),
    )
    .unwrap();
    let surface_id = surface.current.as_ref().unwrap().surface_id.clone();
    let projection = CommandHandler::package_workspace_assistant_context(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current
        .continuity
        .prior_turn_refs
        .iter()
        .any(|r| r == &surface_id));
    assert!(current
        .items
        .iter()
        .any(|i| i.kind == "prior_assistant_evidence" && i.artefact_ref == surface_id));
    assert!(!current
        .continuity
        .prior_turn_refs
        .iter()
        .any(|r| r.contains("invented") || r.contains("fabricat")));
}

#[test]
fn empty_scope_gap_is_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = WorkspaceAssistantContextService::package_context(
        &kernel.shared_database(),
        &ActorContext::local_user(),
        ws,
        empty_scope(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert!(current.gaps.iter().any(|g| g.gap_kind == "empty_scope"));
}

#[test]
fn no_execution_paths_in_service() {
    let source = include_str!("../services/workspace_assistant_context.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("DecisionEngineService"));
    assert!(!source.contains("WorkspaceRecommendationEngineService"));
}
