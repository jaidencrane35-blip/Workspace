//! Programme II Batch 4 — Cognitive Graph contract tests.

use workspace_domain::{
    recovery_must_not_fabricate_cognitive_graph, ActorContext, IntentContext,
};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspaceCognitiveGraphService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    CommandHandler::create_workspace(
        kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        "Graph WS".into(),
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
        "Ship cognitive graph".into(),
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
        "Ship cognitive graph".into(),
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
        "Expose cross-domain topology".into(),
        None,
        85,
        70,
        30,
        None,
        Some(cognitive_goal.id.clone()),
    )
    .unwrap();
    let risk = CommandHandler::create_cognitive_node(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "risk".into(),
        "Graph becomes second SoT".into(),
        None,
        60,
        50,
        55,
        None,
        None,
    )
    .unwrap();
    CommandHandler::create_cognitive_relation(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        objective.id.clone(),
        risk.id,
        "constrains".into(),
        70,
        Some("Risk constrains objective".into()),
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
    CommandHandler::generate_reasoning_record(kernel, actor, intent, ws.to_string()).unwrap();
}

#[test]
fn graph_generation_is_reference_only_and_non_executing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let snap = CommandHandler::generate_cognitive_graph(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert!(recovery_must_not_fabricate_cognitive_graph(&snap));
    let current = snap.current.as_ref().expect("current graph");
    assert!(current.is_non_executing());
    assert!(!current.nodes.is_empty());
    assert!(current.nodes.iter().all(|n| n.external_ref.contains(':')));
    assert!(current
        .edges
        .iter()
        .any(|e| e.kind.as_str() == "constrains" || e.kind.as_str() == "derived_from"));
    assert!(current
        .nodes
        .iter()
        .any(|n| n.kind.as_str() == "plan" || n.kind.as_str() == "reasoning"));
}

#[test]
fn graph_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_cognitive_graph(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().meta.id.clone();
    let second = CommandHandler::generate_cognitive_graph(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_ne!(second.current.as_ref().unwrap().meta.id, first_id);
    assert_eq!(second.history_count, 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    assert_eq!(second.history[0].snapshot_id, first_id);
    let summary =
        CommandHandler::get_cognitive_graph_summary(&kernel, actor, intent, ws, 0).unwrap();
    assert_eq!(summary.history.len(), 0);
    assert_eq!(summary.history_count, 1);
}

#[test]
fn graph_restart_continuity_without_fabricating() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generated = CommandHandler::generate_cognitive_graph(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let id = generated.current.as_ref().unwrap().meta.id.clone();
    let loaded = CommandHandler::get_cognitive_graph(&kernel, actor, intent, ws).unwrap();
    assert_eq!(loaded.current.as_ref().unwrap().meta.id, id);
    assert!(recovery_must_not_fabricate_cognitive_graph(&loaded));
}

#[test]
fn empty_workspace_graph_remains_missing() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let snap = CommandHandler::get_cognitive_graph(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(snap.current.is_none());
    assert_eq!(snap.history_count, 0);
    assert!(recovery_must_not_fabricate_cognitive_graph(&snap));
}

#[test]
fn graph_transaction_rollback_leaves_no_partial_write() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_cognitive_graph(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let before_id = before.current.as_ref().unwrap().meta.id.clone();
    let before_count = before.history_count;
    WorkspaceCognitiveGraphService::generate_with_forced_rollback(
        &kernel.shared_database(),
        &actor,
        ws.clone(),
    )
    .unwrap();
    let after = CommandHandler::get_cognitive_graph(&kernel, actor, intent, ws).unwrap();
    assert_eq!(after.current.as_ref().unwrap().meta.id, before_id);
    assert_eq!(after.history_count, before_count);
}

#[test]
fn graph_negative_authority_guards() {
    for (label, result) in [
        ("execute", WorkspaceCognitiveGraphService::attempt_execute()),
        (
            "task",
            WorkspaceCognitiveGraphService::attempt_create_task(),
        ),
        (
            "recommendation",
            WorkspaceCognitiveGraphService::attempt_create_recommendation(),
        ),
        (
            "decision",
            WorkspaceCognitiveGraphService::attempt_create_decision(),
        ),
        (
            "lifecycle",
            WorkspaceCognitiveGraphService::attempt_mutate_lifecycle(),
        ),
        (
            "invent",
            WorkspaceCognitiveGraphService::attempt_invent_relationship(),
        ),
    ] {
        let err = result.expect_err(label);
        assert!(
            err.to_string().to_lowercase().contains("cannot")
                || err.to_string().to_lowercase().contains("graph"),
            "{label}: {err}"
        );
    }

    use workspace_domain::CapabilitySet;
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let mut ctx =
        kernel.command_context(ActorContext::local_user(), IntentContext::user_request());
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(
            crate::commands::workspace_cognitive_graph::GenerateCognitiveGraph::new(ws),
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
fn graph_duplicate_prevention_on_regenerate() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_sources(&kernel, &ws);
    let snap = CommandHandler::generate_cognitive_graph(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let current = snap.current.unwrap();
    let mut refs = current
        .nodes
        .iter()
        .map(|n| n.external_ref.clone())
        .collect::<Vec<_>>();
    refs.sort();
    let mut uniq = refs.clone();
    uniq.dedup();
    assert_eq!(refs.len(), uniq.len(), "duplicate node refs");
    let mut keys = current
        .edges
        .iter()
        .map(|e| e.dedup_key())
        .collect::<Vec<_>>();
    keys.sort();
    let mut uniq_keys = keys.clone();
    uniq_keys.dedup();
    assert_eq!(keys.len(), uniq_keys.len(), "duplicate edges");
}
