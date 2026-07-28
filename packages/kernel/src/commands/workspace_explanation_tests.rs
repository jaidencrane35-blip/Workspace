//! Programme III Batch 5 — Workspace Explanation Layer contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_workspace_explanation, ActorContext, ExplanationCompleteness,
    ExplanationScope, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceExplanationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Workspace Explanation WS".into(),
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
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn explanation_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_explanation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_workspace_explanation(&snap));
    let current = snap.current.as_ref().expect("current explanation");
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
    let snap = CommandHandler::generate_workspace_explanation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert!(
        current.completeness == ExplanationCompleteness::Unavailable
            || current.completeness == ExplanationCompleteness::Partial
            || current.completeness == ExplanationCompleteness::Unknown
    );
    assert!(!current.gaps.is_empty());
    assert!(current.gaps.iter().any(|g| g.surface == "state"
        || g.surface == "policy"
        || g.surface == "reconstruction"
        || g.surface == "temporal"));
}

#[test]
fn explanation_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().explanation_id,
        second.current.as_ref().unwrap().explanation_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary =
        CommandHandler::get_workspace_explanation_summary(&kernel, actor, intent, ws, 0)
            .unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn explanation_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_workspace_explanation(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().explanation_id,
        loaded.current.as_ref().unwrap().explanation_id
    );
    assert_eq!(
        generated.current.as_ref().unwrap().narrative,
        loaded.current.as_ref().unwrap().narrative
    );
    assert!(recovery_must_not_fabricate_workspace_explanation(&loaded));
}

#[test]
fn empty_workspace_explanation_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap = CommandHandler::get_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_workspace_explanation(&snap));
    let generated =
        CommandHandler::generate_workspace_explanation(&kernel, actor, intent, ws).unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn explanation_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceExplanationService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        ExplanationScope::all_surfaces(),
    )
    .unwrap();
    let after = CommandHandler::get_workspace_explanation(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().explanation_id,
        after.current.as_ref().unwrap().explanation_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn explanation_negative_authority_guards() {
    assert!(WorkspaceExplanationService::attempt_execute().is_err());
    assert!(WorkspaceExplanationService::attempt_approve().is_err());
    assert!(WorkspaceExplanationService::attempt_mutate_policy().is_err());
    assert!(WorkspaceExplanationService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceExplanationService::attempt_create_task().is_err());
    assert!(WorkspaceExplanationService::attempt_resolve_conflict().is_err());
    assert!(WorkspaceExplanationService::attempt_fabricate_evidence().is_err());
    assert!(WorkspaceExplanationService::attempt_silent_refresh().is_err());
    assert!(WorkspaceExplanationService::attempt_emit_command().is_err());
    assert!(CommandHandler::workspace_explanation_attempt_execute().is_err());
}

#[test]
fn explanation_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_explanation::GenerateWorkspaceExplanation::new(ws),
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
fn explain_workspace_situation_is_non_actionable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_workspace_situation(&kernel, actor, intent, ws).unwrap();
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
}

#[test]
fn explanation_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_upstream_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::generate_workspace_explanation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap = CommandHandler::get_workspace_explanation(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history_count >= 1);
    assert!(snap.history.iter().all(|h| h.is_non_actionable()));
}
