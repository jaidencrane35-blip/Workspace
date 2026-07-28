//! Programme III Batch 4 — Temporal Intelligence contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_temporal_intelligence, ActorContext, IntentContext,
    TemporalAnalysisWindow, UnderstandingCompleteness,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceTemporalIntelligenceService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Temporal Intelligence WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_reconstruction_evidence(kernel: &WorkspaceKernel, ws: &str) {
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
    CommandHandler::generate_historical_workspace_view(
        kernel,
        actor,
        intent,
        ws.to_string(),
    )
    .unwrap();
}

#[test]
fn temporal_analysis_is_non_executing_understanding_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let snap = CommandHandler::generate_temporal_analysis(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_temporal_intelligence(&snap));
    let current = snap.current.as_ref().expect("current analysis");
    assert!(current.is_non_executing());
    assert_eq!(current.authority_effect, "none");
    assert!(!current.actionable);
    assert!(current
        .evidence_quality
        .limitations
        .iter()
        .any(|l| l.contains("≠ correctness") || l.contains("not a correctness")));
}

#[test]
fn missing_reconstruction_is_unavailable_not_invented() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::generate_temporal_analysis(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.as_ref().unwrap();
    assert_eq!(
        current.completeness,
        UnderstandingCompleteness::Unavailable
    );
    assert!(current.chain_summary.observed_transitions.is_empty());
    assert!(!current.gaps.is_empty());
}

#[test]
fn temporal_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().analysis_id,
        second.current.as_ref().unwrap().analysis_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary = CommandHandler::get_temporal_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn temporal_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded = CommandHandler::get_temporal_analysis(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().analysis_id,
        loaded.current.as_ref().unwrap().analysis_id
    );
    assert_eq!(
        generated.current.as_ref().unwrap().chain_summary,
        loaded.current.as_ref().unwrap().chain_summary
    );
    assert!(recovery_must_not_fabricate_temporal_intelligence(&loaded));
}

#[test]
fn empty_workspace_analysis_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap =
        CommandHandler::get_temporal_analysis(&kernel, actor.clone(), intent.clone(), ws.clone())
            .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_temporal_intelligence(&snap));
    let generated =
        CommandHandler::generate_temporal_analysis(&kernel, actor, intent, ws).unwrap();
    assert!(generated.current.is_some());
}

#[test]
fn temporal_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceTemporalIntelligenceService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
        TemporalAnalysisWindow::unbounded(),
    )
    .unwrap();
    let after = CommandHandler::get_temporal_analysis(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().analysis_id,
        after.current.as_ref().unwrap().analysis_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn temporal_negative_authority_guards() {
    assert!(WorkspaceTemporalIntelligenceService::attempt_execute().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_replay().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_simulate().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_forecast().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_fabricate_cause().is_err());
    assert!(WorkspaceTemporalIntelligenceService::attempt_silent_refresh().is_err());
    assert!(CommandHandler::temporal_intelligence_attempt_execute().is_err());
}

#[test]
fn temporal_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::temporal_intelligence::GenerateTemporalAnalysis::new(ws),
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
fn explain_temporal_change_uses_sequence_not_unsupported_cause() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let explanation =
        CommandHandler::explain_temporal_change(&kernel, actor, intent, ws).unwrap();
    assert!(!explanation.narrative.is_empty());
    assert_eq!(explanation.authority_effect, "none");
    assert!(!explanation.actionable);
    assert!(!explanation.narrative.to_lowercase().contains("because the user"));
    assert!(!explanation.narrative.to_lowercase().contains("because the system"));
}

#[test]
fn temporal_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_reconstruction_evidence(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::generate_temporal_analysis(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap = CommandHandler::get_temporal_analysis(&kernel, actor, intent, ws).unwrap();
    assert!(snap.history_count >= 1);
    assert!(snap.history.iter().all(|h| h.is_non_actionable()));
}
