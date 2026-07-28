//! Programme III Batch 3 — Historical Workspace Reconstruction contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_historical_reconstruction, ActorContext, IntentContext,
    ReconstructionCompleteness,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceHistoricalReconstructionService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Historical Reconstruction WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_state_envelopes(kernel: &WorkspaceKernel, ws: &str, count: usize) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    for _ in 0..count {
        CommandHandler::generate_workspace_state_envelope(
            kernel,
            actor.clone(),
            intent.clone(),
            ws.to_string(),
        )
        .unwrap();
    }
}

#[test]
fn reconstruction_is_non_executing_explanation_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let snap = CommandHandler::generate_historical_workspace_view(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_historical_reconstruction(&snap));
    let current = snap.current.as_ref().expect("current reconstruction");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
}

#[test]
fn missing_envelope_evidence_is_unavailable_not_invented() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    // No envelopes — reconstruction must record Unavailable / gaps, not invent timeline.
    let snap = CommandHandler::generate_historical_workspace_view(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert_eq!(
        current.completeness,
        ReconstructionCompleteness::Unavailable
    );
    assert!(current.timeline.is_empty());
    assert!(!current.gaps.is_empty());
}

#[test]
fn reconstruction_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().reconstruction_id,
        second.current.as_ref().unwrap().reconstruction_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary =
        CommandHandler::get_historical_workspace_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn reconstruction_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded =
        CommandHandler::get_historical_workspace_view(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().reconstruction_id,
        loaded.current.as_ref().unwrap().reconstruction_id
    );
    assert_eq!(
        generated.current.as_ref().unwrap().timeline,
        loaded.current.as_ref().unwrap().timeline
    );
    assert!(recovery_must_not_fabricate_historical_reconstruction(&loaded));
}

#[test]
fn empty_workspace_reconstruction_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap =
        CommandHandler::get_historical_workspace_view(&kernel, actor.clone(), intent.clone(), ws.clone())
            .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_historical_reconstruction(&snap));
    let generated =
        CommandHandler::generate_historical_workspace_view(&kernel, actor, intent, ws).unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn reconstruction_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceHistoricalReconstructionService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
    )
    .unwrap();
    let after =
        CommandHandler::get_historical_workspace_view(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().reconstruction_id,
        after.current.as_ref().unwrap().reconstruction_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn reconstruction_negative_authority_guards() {
    assert!(WorkspaceHistoricalReconstructionService::attempt_execute().is_err());
    assert!(WorkspaceHistoricalReconstructionService::attempt_replay().is_err());
    assert!(WorkspaceHistoricalReconstructionService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceHistoricalReconstructionService::attempt_fabricate_transition().is_err());
    assert!(WorkspaceHistoricalReconstructionService::attempt_silent_refresh().is_err());
    assert!(CommandHandler::historical_reconstruction_attempt_execute().is_err());
}

#[test]
fn reconstruction_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::historical_reconstruction::GenerateHistoricalWorkspaceView::new(ws),
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
fn compare_workspace_revisions_uses_durable_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let envelope = CommandHandler::get_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let current_rev = envelope.current.as_ref().unwrap().revision.clone();
    let hist_rev = envelope
        .history
        .first()
        .map(|h| h.revision.clone())
        .unwrap_or_else(|| current_rev.clone());
    let comparison = CommandHandler::compare_workspace_revisions(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        hist_rev,
        current_rev,
    )
    .unwrap();
    assert_eq!(comparison.authority_effect, "none");
    assert!(!comparison.actionable);

    let missing = CommandHandler::compare_workspace_revisions(
        &kernel,
        actor,
        intent,
        ws,
        "rev:missing-left",
        "rev:missing-right",
    )
    .unwrap();
    assert_eq!(
        missing.result_status,
        ReconstructionCompleteness::Unavailable
    );
}

#[test]
fn explain_historical_change_separates_explanation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 2);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_historical_change(&kernel, actor, intent, ws).unwrap();
    assert!(!explanation.narrative.is_empty());
    assert_eq!(explanation.authority_effect, "none");
    assert!(!explanation.actionable);
}

#[test]
fn reconstruction_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_state_envelopes(&kernel, &ws, 1);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::generate_historical_workspace_view(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap =
        CommandHandler::get_historical_workspace_view(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history_count >= 1);
    assert!(snap.history.iter().all(|h| h.is_non_actionable()));
}
