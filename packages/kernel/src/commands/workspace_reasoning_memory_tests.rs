//! Programme II Batch 3 — Reasoning Memory contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_reasoning, ActorContext, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceReasoningMemoryService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Reasoning WS".into(),
    )
    .unwrap()
    .id
    .to_string()
}

fn seed_planning(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let goal = CommandHandler::create_work_goal(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "Ship reasoning memory".into(),
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
        "Ship reasoning memory".into(),
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
        "Capture rationale without re-executing".into(),
        None,
        85,
        70,
        30,
        None,
        Some(cognitive_goal.id),
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
    CommandHandler::generate_planning_snapshot(kernel, actor, intent, ws.to_string()).unwrap();
}

#[test]
fn reasoning_generation_is_non_executing_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_planning(&kernel, &ws);
    let snap = CommandHandler::generate_reasoning_record(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_reasoning(&snap));
    let current = snap.current.as_ref().expect("current reasoning");
    assert!(current.is_non_executing());
    assert!(!current.hypothesis.is_empty());
    assert!(!current.rationale.is_empty());
    assert!(!current.reflection.is_empty());
    assert!(!current.lessons.is_empty());
    assert!(current
        .links
        .iter()
        .any(|l| l.kind == "planning" || l.external_ref.starts_with("planning_plan:")));
    assert_eq!(snap.history_count, 0);
}

#[test]
fn reasoning_recomputation_appends_history_and_retains_superseded() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_planning(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let first = CommandHandler::generate_reasoning_record(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().id.clone();

    let second = CommandHandler::generate_reasoning_record(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().id, first_id);
    assert_eq!(second.history_count, 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    assert_eq!(second.history[0].record_id, first_id);

    let summary =
        CommandHandler::get_reasoning_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert_eq!(summary.history.len(), 0);
    assert_eq!(summary.history_count, 1);
    assert!(summary.has_current);
}

#[test]
fn reasoning_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_planning(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let generated = CommandHandler::generate_reasoning_record(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = generated.current.as_ref().unwrap().id.clone();
    let conf = generated.current.as_ref().unwrap().confidence;
    let reflection = generated.current.as_ref().unwrap().reflection.clone();

    let loaded =
        CommandHandler::get_reasoning_record(&kernel, actor, intent, ws).unwrap();
    assert_eq!(loaded.current.as_ref().unwrap().id, id);
    assert_eq!(loaded.current.as_ref().unwrap().confidence, conf);
    assert_eq!(loaded.current.as_ref().unwrap().reflection, reflection);
    assert!(recovery_must_not_fabricate_reasoning(&loaded));
}

#[test]
fn empty_workspace_reasoning_remains_missing_on_load() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_reasoning_record(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_reasoning(&snap));
}

#[test]
fn reasoning_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_planning(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = CommandHandler::generate_reasoning_record(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let before_id = before.current.as_ref().unwrap().id.clone();
    let before_count = before.history_count;

    WorkspaceReasoningMemoryService::generate_with_forced_rollback(
        &kernel.shared_database(),
        ws.clone(),
    )
    .unwrap();

    let after =
        CommandHandler::get_reasoning_record(&kernel, actor, intent, ws).unwrap();
    assert_eq!(after.current.as_ref().unwrap().id, before_id);
    assert_eq!(after.history_count, before_count);
}

#[test]
fn reasoning_never_executes_and_requires_capability() {
    let err = WorkspaceReasoningMemoryService::attempt_execute().expect_err("must fail");
    assert!(
        err.to_string().to_lowercase().contains("cannot execute")
            || err.to_string().to_lowercase().contains("reasoning"),
        "got {err}"
    );

    use workspace_domain::CapabilitySet;
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_reasoning_memory::GenerateReasoningRecord::new(ws),
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
fn planning_plus_reasoning_atomic_path_produces_linked_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let goal = CommandHandler::create_work_goal(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "Atomic path".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::create_cognitive_node(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "goal".into(),
        "Atomic path".into(),
        None,
        80,
        70,
        30,
        Some(format!("work_goal:{}", goal.id.as_str())),
        None,
    )
    .unwrap();

    let snap = WorkspaceReasoningMemoryService::generate_with_planning(
        &kernel.shared_database(),
        &actor,
        &kernel.orchestrated_plans(),
        &kernel.assistant_workflows(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_some());
    assert!(snap
        .current
        .as_ref()
        .unwrap()
        .links
        .iter()
        .any(|l| l.kind == "planning"));
}
