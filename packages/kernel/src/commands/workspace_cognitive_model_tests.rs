//! Programme II Batch 1 — Cognitive Model contract tests.

use workspace_domain::{ActorContext, IntentContext};

use crate::commands::handler::CommandHandler;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_workspace(kernel, actor, intent, "Cognitive WS".into())
        .unwrap()
        .id
        .to_string()
}

#[test]
fn cognitive_model_builds_objectives_relations_and_focus() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let goal = CommandHandler::create_work_goal(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "Ship cognitive layer".into(),
        None,
        None,
    )
    .unwrap();

    let cognitive_goal = CommandHandler::create_cognitive_node(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "goal".into(),
        "Ship cognitive layer".into(),
        None,
        90,
        80,
        25,
        Some(format!("work_goal:{}", goal.id)),
        None,
    )
    .unwrap();
    assert_eq!(cognitive_goal.authority_effect, "none");

    let objective = CommandHandler::create_cognitive_node(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "objective".into(),
        "Define ontology".into(),
        Some("Batch 1".into()),
        85,
        75,
        30,
        None,
        Some(cognitive_goal.id.clone()),
    )
    .unwrap();

    let risk = CommandHandler::create_cognitive_node(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        "risk".into(),
        "Duplicate lifecycle creep".into(),
        None,
        70,
        60,
        50,
        None,
        None,
    )
    .unwrap();

    let relation = CommandHandler::create_cognitive_relation(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        objective.id.clone(),
        risk.id.clone(),
        "constrains".into(),
        70,
        Some("Risk constrains objective scope".into()),
    )
    .unwrap();
    assert_eq!(relation.kind.as_str(), "constrains");

    let focused = CommandHandler::set_cognitive_focus(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
        objective.id.clone(),
    )
    .unwrap();
    assert!(focused.is_current_focus);

    let model = CommandHandler::generate_cognitive_model(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert!(model.is_non_commandable());
    assert_eq!(model.current_focus_ids, vec![objective.id.clone()]);
    assert!(model.nodes.iter().any(|n| n.id == cognitive_goal.id));
    assert!(model.relations.iter().any(|r| r.id == relation.id));
}

#[test]
fn goal_node_without_work_goal_ref_fails_closed() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let err = CommandHandler::create_cognitive_node(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "goal".into(),
        "Missing ref".into(),
        None,
        50,
        50,
        50,
        None,
        None,
    )
    .expect_err("goal requires work_goal ref");
    assert!(
        err.to_string().to_lowercase().contains("work_goal")
            || err.to_string().to_lowercase().contains("cognitive"),
        "got {err}"
    );
}

#[test]
fn cognitive_mutations_require_capability() {
    use workspace_domain::CapabilitySet;

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let mut ctx = kernel.command_context(actor, intent);
    ctx.capability_set = CapabilitySet::new();

    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(crate::commands::workspace_cognitive_model::CreateCognitiveNode::new(
            ws,
            "objective",
            "Denied",
            None,
            50,
            50,
            50,
            None,
            None,
        ))
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
