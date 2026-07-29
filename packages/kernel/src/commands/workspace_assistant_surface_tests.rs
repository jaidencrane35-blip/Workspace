//! Programme IV Batch 11 — Workspace Assistant Surface contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_assistant_surface, ActorContext, AssistantSurfaceScope,
    CapabilitySet, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceAssistantSurfaceService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Assistant Surface WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn state_only_scope() -> AssistantSurfaceScope {
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
        include_state: true,
    }
}

#[test]
fn compose_turn_is_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "present recorded evidence".into(),
    )
    .unwrap();
    assert!(projection.is_non_commandable());
    assert!(recovery_must_not_fabricate_assistant_surface(&projection));
    let current = projection.current.as_ref().unwrap();
    assert!(current.is_non_executing());
    assert_eq!(current.utterance.role, "assistant");
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn unavailable_gaps_are_preserved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "what evidence is recorded?".into(),
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
    let first = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "first turn".into(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().surface_id.clone();
    let second = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        actor,
        intent,
        ws,
        "second turn".into(),
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().surface_id, first_id);
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn forced_rollback_leaves_current_turn_unchanged() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "baseline turn".into(),
    )
    .unwrap();
    WorkspaceAssistantSurfaceService::compose_turn_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        "rolled back turn",
        AssistantSurfaceScope::presentation_default(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_assistant_surface(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().surface_id,
        after.current.as_ref().unwrap().surface_id
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
            crate::commands::workspace_assistant_surface::ComposeWorkspaceAssistantTurn::new(
                ws,
                "present evidence",
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
    assert!(WorkspaceAssistantSurfaceService::attempt_execute().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_approve().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_decide().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_bypass_permission_gateway().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_create_hidden_memory().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_silent_mutate().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_replace_decision_engine().is_err());
    assert!(WorkspaceAssistantSurfaceService::attempt_emit_command().is_err());
    assert!(CommandHandler::assistant_surface_attempt_execute().is_err());
}

#[test]
fn empty_ask_is_preserved_as_gap() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let projection = CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "   ".into(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert_eq!(current.human_ask, "   ");
    assert!(current.gaps.iter().any(|g| g.gap_kind == "empty_ask"));
}

#[test]
fn explain_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    CommandHandler::compose_workspace_assistant_turn(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "present evidence".into(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_assistant_surface(
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
fn citations_retain_upstream_provenance_when_available() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let projection = WorkspaceAssistantSurfaceService::compose_turn(
        &kernel.shared_database(),
        &actor,
        ws,
        "present state evidence",
        state_only_scope(),
    )
    .unwrap();
    let current = projection.current.as_ref().unwrap();
    assert_eq!(current.utterance.citations.len(), 1);
    let citation = &current.utterance.citations[0];
    assert_eq!(citation.origin_domain, "workspace_state_envelope");
    assert!(citation.source_revision.is_some());
    assert!(!citation.artefact_ref.is_empty());
}

#[test]
fn service_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_assistant_surface.rs");
    assert!(!source.contains("::generate("));
    assert!(!source.contains("Service::generate"));
}

#[test]
fn no_execution_paths_in_service() {
    let source = include_str!("../services/workspace_assistant_surface.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("DecisionEngineService"));
    assert!(!source.contains("WorkspaceRecommendationEngineService"));
}
