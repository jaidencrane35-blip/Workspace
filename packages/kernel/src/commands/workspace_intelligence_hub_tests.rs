//! Programme III Batch 12 — Workspace Intelligence Hub contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_intelligence_hub, ActorContext, IntelligenceHubCompleteness,
    IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceIntelligenceHubService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Intelligence Hub WS".into(),
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
}

fn seed_second_workspace_for_cross(kernel: &WorkspaceKernel) -> String {
    let other = seed_workspace(kernel);
    seed_upstream_evidence(kernel, &other);
    other
}

#[test]
fn intelligence_hub_is_non_executing_aggregation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_intelligence_hub(&snap));
    let current = snap.current.as_ref().expect("current intelligence hub");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(!current.packages.is_empty());
    assert!(current
        .packages
        .iter()
        .filter(|p| p.availability == "available")
        .all(|p| !p.evidence_refs.is_empty()));
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == IntelligenceHubCompleteness::Unavailable
            || current.completeness == IntelligenceHubCompleteness::Partial
            || current.completeness == IntelligenceHubCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn intelligence_hub_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().hub_id.clone();
    let second = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().hub_id, first_id);
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn intelligence_hub_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let generated = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.hub_id),
        generated.current.as_ref().map(|c| &c.hub_id)
    );
    assert!(recovery_must_not_fabricate_intelligence_hub(&loaded));
}

#[test]
fn empty_intelligence_hub_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_intelligence_hub(&snap));
}

#[test]
fn intelligence_hub_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceIntelligenceHubService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        workspace_domain::IntelligenceHubFrame::all_surfaces(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_intelligence_hub(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().hub_id,
        after.current.as_ref().unwrap().hub_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn intelligence_hub_negative_authority_guards() {
    assert!(WorkspaceIntelligenceHubService::attempt_execute().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_create_recommendation().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_become_decision_maker().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_invent_intelligence().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_resolve_conflicts().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_fabricate_lineage().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_silent_refresh().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_emit_command().is_err());
    assert!(WorkspaceIntelligenceHubService::attempt_mutate_upstream().is_err());
    assert!(CommandHandler::intelligence_hub_attempt_execute().is_err());
}

#[test]
fn intelligence_hub_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_intelligence_hub::GenerateWorkspaceIntelligenceHub::new(ws),
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
fn explain_intelligence_hub_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_workspace_intelligence(
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
fn intelligence_hub_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::generate_workspace_intelligence_hub(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_workspace_intelligence_hub_summary(
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
fn conflicts_retained_not_resolved() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    for _ in 0..2 {
        CommandHandler::generate_workspace_state_envelope(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws.clone(),
        )
        .unwrap();
    }
    let snap = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    if !current.conflicts.is_empty() {
        assert!(current.conflicts.iter().all(|c| !c.actionable));
        assert!(current.conflicts.iter().all(|c| c.authority_effect == "none"));
    }
    assert!(current
        .summary
        .notes
        .iter()
        .any(|n| n.contains("conflict record")));
}

#[test]
fn hub_service_source_never_calls_foreign_generate() {
    let source = include_str!("../services/workspace_intelligence_hub.rs");
    assert!(!source.contains("WorkspaceStateCompositionService::generate"));
    assert!(!source.contains("WorkspaceDecisionSupportService::generate"));
    assert!(!source.contains("WorkspaceInsightCoordinationService::generate"));
    assert!(!source.contains("WorkspaceCrossIntelligenceService::generate"));
    assert!(!source.contains("DecisionEngineService::"));
    assert!(!source.contains("WorkspaceRecommendationEngineService::"));
    assert!(!source.contains("ExecutionLifecycleService::"));
}

#[test]
fn decision_support_surface_included_as_reference_when_present() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let _other = seed_second_workspace_for_cross(&kernel);
    CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let snap = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(current
        .provenance_links
        .iter()
        .any(|p| p.origin_domain == "workspace_decision_support"));
    assert!(current
        .packages
        .iter()
        .any(|p| p.surface == "workspace_decision_support"));
}

#[test]
fn lineage_is_reference_only_not_fabricated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_intelligence_hub(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.lineage.actionable);
    assert_eq!(current.lineage.authority_effect, "none");
    assert!(current.lineage.upstream_sources.len() <= current.packages.len());
}
