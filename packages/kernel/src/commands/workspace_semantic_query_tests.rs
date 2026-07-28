//! Programme IV Batch 1 — Workspace Semantic Query Engine contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_semantic_query, ActorContext, IntentContext, SemanticQuery,
    SemanticQueryCompleteness, SemanticQueryScope,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceSemanticQueryService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Semantic Query WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_upstream_evidence(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    for _ in 0..2 {
        CommandHandler::generate_workspace_state_envelope(
            kernel,
            actor.clone(),
            intent.clone(),
            ws.to_string(),
        )
        .unwrap();
    }
    CommandHandler::generate_governance_evaluation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_historical_workspace_view(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_temporal_analysis(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_explanation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_contextual_workspace_understanding(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_knowledge_synthesis(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_knowledge_integration(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_insight_coordination(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_decision_support(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_intelligence_hub(
        kernel,
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn semantic_query_is_non_executing_retrieval() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "knowledge state".into(),
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_semantic_query(&snap));
    let current = snap.current.as_ref().expect("current semantic query");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(!current.query.executable);
    assert!(current.result.matches.iter().all(|m| !m.actionable));
}

#[test]
fn unavailable_sources_preserved_as_gaps() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "knowledge".into(),
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == SemanticQueryCompleteness::Unavailable
            || current.completeness == SemanticQueryCompleteness::Partial
            || current.completeness == SemanticQueryCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn semantic_query_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "knowledge".into(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().query_id.clone();
    let second = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        actor,
        intent,
        ws,
        "state policy".into(),
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().query_id, first_id);
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn semantic_query_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let generated = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "intelligence".into(),
    )
    .unwrap();
    let loaded = CommandHandler::get_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.query_id),
        generated.current.as_ref().map(|c| &c.query_id)
    );
    assert!(recovery_must_not_fabricate_semantic_query(&loaded));
}

#[test]
fn empty_semantic_query_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_semantic_query(&snap));
}

#[test]
fn semantic_query_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "knowledge".into(),
    )
    .unwrap();
    let query = SemanticQuery::request(
        "knowledge",
        SemanticQueryScope::all_surfaces(),
        vec![],
        vec![],
    )
    .unwrap();
    WorkspaceSemanticQueryService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        query,
    )
    .unwrap();
    let after = CommandHandler::get_workspace_semantic_query(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().query_id,
        after.current.as_ref().unwrap().query_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn semantic_query_negative_authority_guards() {
    assert!(WorkspaceSemanticQueryService::attempt_execute().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_create_recommendation().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_reason().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_plan().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_invent_matches().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_fabricate_relevance().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_invent_lineage().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_infer_absent_evidence().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_silent_refresh().is_err());
    assert!(WorkspaceSemanticQueryService::attempt_emit_command().is_err());
    assert!(CommandHandler::semantic_query_attempt_execute().is_err());
}

#[test]
fn semantic_query_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_semantic_query::GenerateWorkspaceSemanticQuery::new(
                ws,
                "knowledge",
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
fn explain_semantic_query_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    CommandHandler::generate_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "knowledge".into(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_workspace_semantic_query(
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
fn semantic_query_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_semantic_query(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "knowledge".into(),
    )
    .unwrap();
    let snap = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        actor,
        intent,
        ws,
        "state".into(),
    )
    .unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_workspace_semantic_query_summary(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        snap.workspace_id.clone(),
        10,
    )
    .unwrap();
    assert!(summary.history_count >= summary.history.len());
}

#[test]
fn provenance_retained_from_contributing_sources() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_semantic_query(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "knowledge intelligence".into(),
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.lineage.actionable);
    assert_eq!(current.lineage.authority_effect, "none");
    assert!(!current.provenance_links.is_empty() || !current.gaps.is_empty());
}

#[test]
fn semantic_query_service_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_semantic_query.rs");
    assert!(!source.contains("WorkspaceStateCompositionService::generate"));
    assert!(!source.contains("WorkspaceDecisionSupportService::generate"));
    assert!(!source.contains("WorkspaceInsightCoordinationService::generate"));
    assert!(!source.contains("WorkspaceIntelligenceHubService::generate"));
    assert!(!source.contains("WorkspaceCrossIntelligenceService::generate"));
    assert!(!source.contains("DecisionEngineService::"));
    assert!(!source.contains("WorkspaceRecommendationEngineService::"));
    assert!(!source.contains("ExecutionLifecycleService::"));
}

#[test]
fn no_execution_paths_in_semantic_query_service() {
    let source = include_str!("../services/workspace_semantic_query.rs");
    assert!(!source.contains("ExecutionLifecycle"));
    assert!(!source.contains("PermissionGateway"));
    assert!(!source.contains("CommandPipeline"));
    assert!(!source.contains("attempt_approve"));
}
