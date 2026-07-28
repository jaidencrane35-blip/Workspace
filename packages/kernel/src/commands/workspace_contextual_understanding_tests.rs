//! Programme III Batch 6 — Contextual Workspace Understanding contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_contextual_understanding, ActorContext, ContextFrame,
    ContextualCompleteness, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceContextualUnderstandingService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Contextual Understanding WS".into(),
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
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn contextual_understanding_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_contextual_understanding(&snap));
    let current = snap.current.as_ref().expect("current understanding");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(current
        .limitations
        .iter()
        .any(|l| l.contains("does not change reality") || l.contains("not change reality")));
}

#[test]
fn missing_upstreams_produce_gaps_not_invention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == ContextualCompleteness::Unavailable
            || current.completeness == ContextualCompleteness::Partial
            || current.completeness == ContextualCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().any(|g| g.surface == "state"
        || g.surface == "policy"
        || g.surface == "reconstruction"
        || g.surface == "temporal"
        || g.surface == "explanation"));
}

#[test]
fn contextual_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().understanding_id,
        second.current.as_ref().unwrap().understanding_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary = CommandHandler::get_contextual_workspace_understanding_summary(
        &kernel, actor, intent, ws, 0,
    )
    .unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn contextual_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded =
        CommandHandler::get_contextual_workspace_understanding(&kernel, actor, intent, ws)
            .unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().understanding_id,
        loaded.current.as_ref().unwrap().understanding_id
    );
    assert_eq!(
        generated.current.as_ref().unwrap().narrative,
        loaded.current.as_ref().unwrap().narrative
    );
    assert!(recovery_must_not_fabricate_contextual_understanding(&loaded));
}

#[test]
fn empty_contextual_understanding_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap = CommandHandler::get_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_contextual_understanding(&snap));
    let generated = CommandHandler::generate_contextual_workspace_understanding(
        &kernel, actor, intent, ws,
    )
    .unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn contextual_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceContextualUnderstandingService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        ContextFrame::all_surfaces(),
    )
    .unwrap();
    let after =
        CommandHandler::get_contextual_workspace_understanding(&kernel, actor, intent, ws)
            .unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().understanding_id,
        after.current.as_ref().unwrap().understanding_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn contextual_negative_authority_guards() {
    assert!(WorkspaceContextualUnderstandingService::attempt_execute().is_err());
    assert!(WorkspaceContextualUnderstandingService::attempt_approve().is_err());
    assert!(WorkspaceContextualUnderstandingService::attempt_create_task().is_err());
    assert!(WorkspaceContextualUnderstandingService::attempt_mutate_intent().is_err());
    assert!(WorkspaceContextualUnderstandingService::attempt_mutate_task_graph().is_err());
    assert!(
        WorkspaceContextualUnderstandingService::attempt_alter_workspace_state_envelope().is_err()
    );
    assert!(
        WorkspaceContextualUnderstandingService::attempt_convert_insight_to_recommendation()
            .is_err()
    );
    assert!(
        WorkspaceContextualUnderstandingService::attempt_invent_causal_explanations().is_err()
    );
    assert!(WorkspaceContextualUnderstandingService::attempt_silent_refresh().is_err());
    assert!(WorkspaceContextualUnderstandingService::attempt_emit_command().is_err());
    assert!(CommandHandler::contextual_understanding_attempt_execute().is_err());
}

#[test]
fn contextual_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_contextual_understanding::GenerateContextualWorkspaceUnderstanding::new(
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
fn explain_workspace_context_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_workspace_context(&kernel, actor, intent, ws).unwrap();
    assert!(!explanation.narrative.is_empty());
    assert_eq!(explanation.authority_effect, "none");
    assert!(!explanation.actionable);
    assert!(!explanation
        .narrative
        .to_lowercase()
        .contains("because the user"));
    assert!(!explanation
        .narrative
        .to_lowercase()
        .contains("because the system"));
    assert!(!explanation
        .narrative
        .to_lowercase()
        .contains("recommend that you"));
}

#[test]
fn contextual_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::generate_contextual_workspace_understanding(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::get_contextual_workspace_understanding(&kernel, actor, intent, ws)
            .unwrap();
    assert!(snap.history_count >= 1);
    assert!(snap.history.iter().all(|h| h.is_non_actionable()));
}
