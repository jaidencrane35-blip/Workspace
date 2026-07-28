//! Programme II Batch 6 — Learning & Adaptation contract tests.

use workspace_domain::{recovery_must_not_fabricate_learning, ActorContext, IntentContext};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceLearningAdaptationService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Learning WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_sources(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let goal = CommandHandler::create_work_goal(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "Ship learning".into(),
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
        "Ship learning".into(),
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
        "Observe outcomes".into(),
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
    CommandHandler::generate_workspace_orchestration(kernel, actor, intent, ws.to_string())
        .unwrap();
}

#[test]
fn learning_generation_is_observational_and_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let snap = CommandHandler::generate_learning_snapshot(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_learning(&snap));
    let current = snap.current.as_ref().expect("current learning");
    assert!(current.is_non_executing());
    assert!(current.adaptation_candidates.iter().all(|c| c.is_non_actionable()));
    assert_eq!(current.meta.authority_effect, "none");
}

#[test]
fn learning_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_learning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().meta.learning_id.clone();
    let second = CommandHandler::generate_learning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().meta.learning_id, first_id);
    assert_eq!(second.history_count, 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    assert_eq!(second.history[0].learning_id, first_id);
    let summary =
        CommandHandler::get_learning_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert_eq!(summary.history.len(), 0);
    assert_eq!(summary.history_count, 1);
}

#[test]
fn learning_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_learning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = generated.current.as_ref().unwrap().meta.learning_id.clone();
    let loaded = CommandHandler::get_learning_snapshot(&kernel, actor, intent, ws).unwrap();
    assert_eq!(loaded.current.as_ref().unwrap().meta.learning_id, id);
    assert!(recovery_must_not_fabricate_learning(&loaded));
}

#[test]
fn empty_workspace_learning_remains_missing_until_generated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_learning_snapshot(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_learning(&snap));

    let generated = CommandHandler::generate_learning_snapshot(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = generated.current.as_ref().unwrap();
    assert!(current
        .observations
        .iter()
        .any(|o| o.kind == "missing_outcome"));
    assert!(current.is_non_executing());
}

#[test]
fn learning_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_learning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let before_id = before.current.as_ref().unwrap().meta.learning_id.clone();
    let before_count = before.history_count;

    WorkspaceLearningAdaptationService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
    )
    .unwrap();

    let after = CommandHandler::get_learning_snapshot(&kernel, actor, intent, ws).unwrap();
    assert_eq!(after.current.as_ref().unwrap().meta.learning_id, before_id);
    assert_eq!(after.history_count, before_count);
}

#[test]
fn learning_negative_authority_guards() {
    assert!(WorkspaceLearningAdaptationService::attempt_execute().is_err());
    assert!(WorkspaceLearningAdaptationService::attempt_auto_change_plan().is_err());
    assert!(WorkspaceLearningAdaptationService::attempt_mutate_lifecycle().is_err());
    assert!(WorkspaceLearningAdaptationService::attempt_approve_decision().is_err());
    assert!(WorkspaceLearningAdaptationService::attempt_auto_apply_adaptation().is_err());
    assert!(WorkspaceLearningAdaptationService::attempt_rewrite_history().is_err());
    assert!(CommandHandler::workspace_learning_adaptation_attempt_execute().is_err());
}

#[test]
fn learning_capability_deny_without_write() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_learning_adaptation::GenerateLearningSnapshot::new(ws),
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
fn learning_history_is_non_actionable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::generate_learning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let snap = CommandHandler::generate_learning_snapshot(&kernel, actor, intent, ws).unwrap();
    assert!(snap
        .history
        .iter()
        .all(|h| h.terminal && !h.actionable && h.authority_effect == "none"));
}
