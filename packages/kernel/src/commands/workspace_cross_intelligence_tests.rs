//! Programme III Batch 10 — Cross-Workspace Intelligence contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_cross_workspace_intelligence, ActorContext,
    CrossWorkspaceCompleteness, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceCrossIntelligenceService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel, name: &str) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        name.into(),
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
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

fn seed_two_workspaces_with_evidence(kernel: &WorkspaceKernel) -> (String, String) {
    let a = seed_workspace(kernel, "Cross Intel WS A");
    let b = seed_workspace(kernel, "Cross Intel WS B");
    seed_upstream_evidence(kernel, &a);
    seed_upstream_evidence(kernel, &b);
    (a, b)
}

#[test]
fn cross_workspace_intelligence_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let snap = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_cross_workspace_intelligence(&snap));
    let current = snap.current.as_ref().expect("current cross-workspace intelligence");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(current
        .patterns
        .iter()
        .all(|p| !p.evidence_references.is_empty()));
    assert!(current
        .themes
        .iter()
        .all(|t| !t.supporting_evidence.is_empty()));
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let _a = seed_workspace(&kernel, "Sparse A");
    let _b = seed_workspace(&kernel, "Sparse B");
    let snap = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == CrossWorkspaceCompleteness::Unavailable
            || current.completeness == CrossWorkspaceCompleteness::Partial
            || current.completeness == CrossWorkspaceCompleteness::Unknown
    );
    assert!(current.patterns.is_empty() || !current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn cross_workspace_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        actor.clone(),
        intent.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().intelligence_id.clone();
    let second =
        CommandHandler::generate_cross_workspace_intelligence(&kernel, actor, intent).unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().intelligence_id,
        first_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn cross_workspace_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let generated = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let loaded = CommandHandler::get_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.intelligence_id),
        generated.current.as_ref().map(|c| &c.intelligence_id)
    );
    assert!(recovery_must_not_fabricate_cross_workspace_intelligence(&loaded));
}

#[test]
fn empty_cross_workspace_intelligence_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let _ = seed_workspace(&kernel, "Empty until generated");
    let snap = CommandHandler::get_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_cross_workspace_intelligence(&snap));
}

#[test]
fn cross_workspace_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        actor.clone(),
        intent.clone(),
    )
    .unwrap();
    WorkspaceCrossIntelligenceService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
    )
    .unwrap();
    let after =
        CommandHandler::get_cross_workspace_intelligence(&kernel, actor, intent).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().intelligence_id,
        after.current.as_ref().unwrap().intelligence_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn cross_workspace_negative_authority_guards() {
    assert!(WorkspaceCrossIntelligenceService::attempt_execute().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_create_task().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_grant_permissions().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_approve_policy().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_create_recommendation().is_err());
    assert!(
        WorkspaceCrossIntelligenceService::attempt_become_memory_or_cognitive_model().is_err()
    );
    assert!(WorkspaceCrossIntelligenceService::attempt_invent_global_patterns().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_infer_missing_workspaces().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_fabricate_statistics().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_silent_refresh().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_emit_command().is_err());
    assert!(WorkspaceCrossIntelligenceService::attempt_convert_frequency_to_action().is_err());
    assert!(CommandHandler::cross_workspace_intelligence_attempt_execute().is_err());
}

#[test]
fn cross_workspace_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let _ = seed_workspace(&kernel, "Cap deny");
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_cross_intelligence::GenerateCrossWorkspaceIntelligence::new(
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
fn explain_cross_workspace_pattern_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_cross_workspace_pattern(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    assert!(!explanation.actionable);
    assert_eq!(explanation.authority_effect, "none");
}

#[test]
fn cross_workspace_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        actor.clone(),
        intent.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::generate_cross_workspace_intelligence(&kernel, actor, intent).unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_cross_workspace_summary(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        10,
    )
    .unwrap();
    assert!(summary.history_count >= summary.history.len());
}

#[test]
fn recurring_patterns_across_workspaces() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    seed_two_workspaces_with_evidence(&kernel);
    let snap = CommandHandler::generate_cross_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(current.participating_workspaces.len() >= 2);
    // Shared surfaces across seeded workspaces should yield recurring patterns
    // (surface co-presence and/or shared theme/constraint labels).
    assert!(
        !current.patterns.is_empty()
            || !current.themes.is_empty()
            || !current.constraint_patterns.is_empty()
            || !current.risk_signals.is_empty()
    );
    assert!(current.patterns.iter().all(|p| {
        !p.actionable && p.authority_effect == "none" && p.participating_workspaces.len() >= 2
    }));
}
