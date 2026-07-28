//! Programme III Batch 1 — Unified Workspace State Envelope contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_workspace_state_envelope, ActorContext, IntentContext,
    FreshnessStatus,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceStateCompositionService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "State Envelope WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_cognitive_sources(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let goal = CommandHandler::create_work_goal(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "Ship envelope".into(),
        None,
        None,
    )
    .unwrap();
    let cognitive_goal = CommandHandler::create_cognitive_node(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "goal".into(),
        "Ship envelope".into(),
        None,
        90,
        80,
        20,
        Some(format!("work_goal:{}", goal.id.as_str())),
        None,
    )
    .unwrap();
    let objective = CommandHandler::create_cognitive_node(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "objective".into(),
        "Compose state".into(),
        None,
        85,
        70,
        30,
        None,
        Some(cognitive_goal.id.clone()),
    )
    .unwrap();
    CommandHandler::set_cognitive_focus(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        objective.id,
    )
    .unwrap();
    CommandHandler::generate_planning_snapshot(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_reasoning_record(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_cognitive_graph(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_workspace_orchestration(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_learning_snapshot(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
    )
    .unwrap();
    CommandHandler::generate_cognitive_agent_cast(kernel, actor.clone(), intent.clone(), ws.to_string())
        .unwrap();
    CommandHandler::generate_cognitive_autonomy(kernel, actor, intent, ws.to_string()).unwrap();
}

#[test]
fn envelope_generation_is_composition_only_and_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive_sources(&kernel, &ws);
    let snap = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_workspace_state_envelope(&snap));
    let current = snap.current.as_ref().expect("current envelope");
    assert!(current.is_non_executing());
    assert!(!current.sources.is_empty());
    assert!(current
        .sources
        .iter()
        .any(|s| s.source_type == "planning" && s.availability_status
            == workspace_domain::AvailabilityStatus::Available));
    // Explicit unavailable sources must not be assumed fresh/current.
    assert!(current.sources.iter().any(|s| {
        s.source_type == "execution" && s.freshness_status == FreshnessStatus::Unavailable
    }));
}

#[test]
fn envelope_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(
        first.current.as_ref().unwrap().state_id,
        second.current.as_ref().unwrap().state_id
    );
    assert!(second.history_count >= 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    let summary =
        CommandHandler::get_workspace_state_envelope_summary(&kernel, actor, intent, ws, 0)
            .unwrap();
    assert!(summary.history_count >= 1);
    assert!(summary.history.is_empty());
}

#[test]
fn envelope_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let loaded =
        CommandHandler::get_workspace_state_envelope(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        generated.current.as_ref().unwrap().state_id,
        loaded.current.as_ref().unwrap().state_id
    );
    assert_eq!(
        generated.current.as_ref().unwrap().revision,
        loaded.current.as_ref().unwrap().revision
    );
    assert!(recovery_must_not_fabricate_workspace_state_envelope(&loaded));
}

#[test]
fn empty_workspace_envelope_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let snap =
        CommandHandler::get_workspace_state_envelope(&kernel, actor.clone(), intent.clone(), ws.clone())
            .unwrap();
    assert!(snap.current.is_none());
    assert!(recovery_must_not_fabricate_workspace_state_envelope(&snap));
    let generated =
        CommandHandler::generate_workspace_state_envelope(&kernel, actor, intent, ws).unwrap();
    assert!(generated.current.is_some());
    // Missing operational sources stay unavailable — never assumed fresh.
    let current = generated.current.as_ref().unwrap();
    assert!(current.sources.iter().any(|s| {
        s.source_type == "task_graph" && s.freshness_status == FreshnessStatus::Unavailable
    }));
    assert_ne!(current.freshness, FreshnessStatus::Fresh);
}

#[test]
fn envelope_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    WorkspaceStateCompositionService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
    )
    .unwrap();
    let after =
        CommandHandler::get_workspace_state_envelope(&kernel, actor, intent, ws).unwrap();
    assert_eq!(
        before.current.as_ref().unwrap().state_id,
        after.current.as_ref().unwrap().state_id
    );
    assert_eq!(before.history_count, after.history_count);
}

#[test]
fn envelope_negative_authority_guards() {
    assert!(WorkspaceStateCompositionService::attempt_execute().is_err());
    assert!(WorkspaceStateCompositionService::attempt_mutate_source().is_err());
    assert!(WorkspaceStateCompositionService::attempt_repair().is_err());
    assert!(WorkspaceStateCompositionService::attempt_silent_refresh().is_err());
    assert!(WorkspaceStateCompositionService::attempt_approve().is_err());
    assert!(WorkspaceStateCompositionService::attempt_grant().is_err());
    assert!(CommandHandler::workspace_state_envelope_attempt_execute().is_err());
}

#[test]
fn envelope_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_state_envelope::GenerateWorkspaceStateEnvelope::new(ws),
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
fn envelope_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_state_envelope(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert!(second.history.iter().all(|h| {
        h.terminal && !h.actionable && h.authority_effect == "none"
    }));
}
