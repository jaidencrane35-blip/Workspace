//! Programme III Batch 9 — Workspace Insight Coordination contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_insight_coordination, ActorContext, InsightCoordinationCompleteness,
    IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceInsightCoordinationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Insight Coordination WS".into(),
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
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn insight_coordination_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_insight_coordination(&snap));
    let current = snap.current.as_ref().expect("current coordination");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(current
        .clusters
        .iter()
        .all(|c| !c.evidence_references.is_empty()));
    assert!(current
        .intersections
        .iter()
        .all(|i| relationship_is_meaning_only(&i.relationship_type)));
}

fn relationship_is_meaning_only(kind: &str) -> bool {
    matches!(
        kind,
        "relates_to"
            | "overlaps"
            | "associated_with"
            | "observed_with"
            | "shares_evidence"
            | "references"
            | "derived_from"
            | "supported_by"
    )
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == InsightCoordinationCompleteness::Unavailable
            || current.completeness == InsightCoordinationCompleteness::Partial
            || current.completeness == InsightCoordinationCompleteness::Unknown
    );
    assert!(current.clusters.is_empty() || !current.gaps.is_empty());
    assert!(current.gaps.iter().all(|g| !g.actionable));
}

#[test]
fn insight_coordination_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_insight_coordination(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().coordination_id.clone();
    let second = CommandHandler::generate_insight_coordination(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert_ne!(
        second.current.as_ref().unwrap().coordination_id,
        first_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
}

#[test]
fn insight_coordination_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let generated = CommandHandler::generate_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().map(|c| &c.coordination_id),
        generated.current.as_ref().map(|c| &c.coordination_id)
    );
    assert!(recovery_must_not_fabricate_insight_coordination(&loaded));
}

#[test]
fn empty_insight_coordination_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_insight_coordination(&snap));
}

#[test]
fn insight_coordination_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_insight_coordination(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceInsightCoordinationService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        workspace_domain::InsightCoordinationFrame::all_surfaces(),
    )
    .unwrap();
    let after = CommandHandler::get_insight_coordination(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().coordination_id,
        after.current.as_ref().unwrap().coordination_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn insight_coordination_negative_authority_guards() {
    assert!(WorkspaceInsightCoordinationService::attempt_execute().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_create_task().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_grant_permissions().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_approve_policy().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_create_recommendation().is_err());
    assert!(
        WorkspaceInsightCoordinationService::attempt_become_memory_or_cognitive_model().is_err()
    );
    assert!(WorkspaceInsightCoordinationService::attempt_invent_missing_evidence().is_err());
    assert!(
        WorkspaceInsightCoordinationService::attempt_convert_correlation_to_causation().is_err()
    );
    assert!(
        WorkspaceInsightCoordinationService::attempt_convert_prioritisation_to_action().is_err()
    );
    assert!(WorkspaceInsightCoordinationService::attempt_silent_refresh().is_err());
    assert!(WorkspaceInsightCoordinationService::attempt_emit_command().is_err());
    assert!(CommandHandler::insight_coordination_attempt_execute().is_err());
}

#[test]
fn insight_coordination_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_insight_coordination::GenerateInsightCoordination::new(ws),
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
fn explain_insight_coordination_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    CommandHandler::generate_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let explanation = CommandHandler::explain_insight_coordination(
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
fn insight_coordination_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_insight_coordination(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap = CommandHandler::generate_insight_coordination(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
    let summary = CommandHandler::get_insight_coordination_summary(
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
fn intersections_are_meaning_only_not_causal() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_insight_coordination(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(!current.intersections.is_empty());
    for i in &current.intersections {
        assert!(!matches!(
            i.relationship_type.as_str(),
            "causes" | "triggers" | "requires" | "should_execute" | "authorises"
        ));
        assert!(!i.actionable);
    }
    assert!(current
        .attention_signals
        .iter()
        .all(|s| !s.actionable && s.authority_effect == "none"));
}
