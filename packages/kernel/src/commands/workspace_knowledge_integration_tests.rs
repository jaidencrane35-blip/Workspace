//! Programme III Batch 8 — Workspace Knowledge Integration contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_knowledge_integration, ActorContext, IntentContext,
    KnowledgeIntegrationCompleteness, KnowledgeRetrievalFrame,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceKnowledgeIntegrationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Knowledge Integration WS".into(),
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
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn knowledge_integration_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_knowledge_integration(&snap));
    let current = snap.current.as_ref().expect("current integration");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(current
        .links
        .iter()
        .all(|l| !l.evidence_refs.is_empty()));
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == KnowledgeIntegrationCompleteness::Unavailable
            || current.completeness == KnowledgeIntegrationCompleteness::Partial
            || current.completeness == KnowledgeIntegrationCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
}

#[test]
fn knowledge_integration_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().integration_id,
        second.current.as_ref().unwrap().integration_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary = CommandHandler::get_workspace_knowledge_integration_summary(
        &kernel, actor, intent, ws, 0,
    )
    .unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn knowledge_integration_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded =
        CommandHandler::get_workspace_knowledge_integration(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().integration_id,
        loaded.current.as_ref().unwrap().integration_id
    );
    assert!(recovery_must_not_fabricate_knowledge_integration(&loaded));
}

#[test]
fn empty_knowledge_integration_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap = CommandHandler::get_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_knowledge_integration(&snap));
    let generated =
        CommandHandler::generate_workspace_knowledge_integration(&kernel, actor, intent, ws)
            .unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn knowledge_integration_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceKnowledgeIntegrationService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        KnowledgeRetrievalFrame::all_surfaces(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_knowledge_integration(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().integration_id,
        after.current.as_ref().unwrap().integration_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn knowledge_integration_negative_authority_guards() {
    assert!(WorkspaceKnowledgeIntegrationService::attempt_execute().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_create_task().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_approve().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_mutate_intent().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_modify_cognitive_model().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_alter_memory().is_err());
    assert!(
        WorkspaceKnowledgeIntegrationService::attempt_mutate_knowledge_synthesis().is_err()
    );
    assert!(WorkspaceKnowledgeIntegrationService::attempt_repair_contradictions().is_err());
    assert!(
        WorkspaceKnowledgeIntegrationService::attempt_convert_confidence_to_authority().is_err()
    );
    assert!(
        WorkspaceKnowledgeIntegrationService::attempt_fabricate_hits_without_evidence().is_err()
    );
    assert!(
        WorkspaceKnowledgeIntegrationService::attempt_treat_correlation_as_causation().is_err()
    );
    assert!(WorkspaceKnowledgeIntegrationService::attempt_silent_refresh().is_err());
    assert!(WorkspaceKnowledgeIntegrationService::attempt_emit_command().is_err());
    assert!(CommandHandler::knowledge_integration_attempt_execute().is_err());
}

#[test]
fn knowledge_integration_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_knowledge_integration::GenerateWorkspaceKnowledgeIntegration::new(
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
fn explain_and_retrieve_are_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_knowledge_integration(&kernel, actor.clone(), intent.clone(), ws.clone())
            .unwrap();
    assert!(!explanation.narrative.is_empty());
    assert_eq!(explanation.authority_effect, "none");
    assert!(!explanation.actionable);
    let retrieved = CommandHandler::retrieve_workspace_knowledge(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert!(retrieved.is_non_executing());
    assert!(!retrieved.links.iter().any(|l| l.evidence_refs.is_empty()));
}

#[test]
fn integration_links_are_meaning_only_not_causal() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    let forbidden = [
        "causes",
        "requires",
        "should_execute",
        "authorises",
        "requires_action",
        "triggers",
    ];
    for link in &current.links {
        assert!(
            !forbidden.contains(&link.kind.as_str()),
            "forbidden link kind: {}",
            link.kind
        );
        assert!(!link.actionable);
        assert_eq!(link.authority_effect, "none");
    }
}

#[test]
fn knowledge_integration_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::generate_workspace_knowledge_integration(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::get_workspace_knowledge_integration(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history_count >= 1);
    assert!(snap.history.iter().all(|h| h.is_non_actionable()));
}
