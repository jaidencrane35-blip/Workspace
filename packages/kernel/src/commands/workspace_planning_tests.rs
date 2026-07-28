//! Programme II Batch 2 — Planning Engine contract tests.

use workspace_domain::{ActorContext, IntentContext};

use crate::commands::handler::CommandHandler;
use crate::services::WorkspacePlanningService;
use crate::WorkspaceKernel;

fn seed_workspace(kernel: &WorkspaceKernel) -> String {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_workspace(kernel, actor, intent, "Planning WS".into())
        .unwrap()
        .id
        .to_string()
}

fn seed_cognitive(kernel: &WorkspaceKernel, ws: &str) {
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let goal = CommandHandler::create_work_goal(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "Ship planning layer".into(),
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
        "Ship planning layer".into(),
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
        "Define planning ownership".into(),
        None,
        85,
        70,
        30,
        None,
        Some(cognitive_goal.id.clone()),
    )
    .unwrap();
    let _risk = CommandHandler::create_cognitive_node(
        kernel,
        actor.clone(),
        intent.clone(),
        ws.to_string(),
        "risk".into(),
        "Planner absorbs Intent".into(),
        None,
        60,
        50,
        55,
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_cognitive_focus(
        kernel,
        actor,
        intent,
        ws.to_string(),
        objective.id,
    )
    .unwrap();
}

#[test]
fn planning_generation_produces_non_executing_snapshot() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let snap = CommandHandler::generate_planning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(snap.is_non_commandable());
    assert_eq!(snap.authority_effect, "none");
    let current = snap.current.as_ref().expect("active proposal");
    assert!(current.is_non_executing());
    assert!(!current.steps.is_empty());
    assert!(current.steps.iter().any(|s| s.title.contains("focus")
        || s.title.contains("objective")
        || s.title.contains("Focus")
        || s.title.contains("objective")
        || s.evidence_refs.iter().any(|r| r.starts_with("cognitive:"))));
    assert!(current.explanation.next.to_lowercase().contains("execution"));
    assert_eq!(snap.history_count, 0);
}

#[test]
fn planning_recomputation_supersedes_and_separates_history() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let first = CommandHandler::generate_planning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let first_id = first.current.as_ref().unwrap().plan.id.clone();

    let second = CommandHandler::generate_planning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(second.current.is_some());
    assert_ne!(
        second.current.as_ref().unwrap().plan.id,
        first_id
    );
    assert_eq!(second.history_count, 1);
    assert!(second.history.iter().all(|h| h.is_non_actionable()));
    assert!(second.history.iter().all(|h| !h.actionable));
    assert_eq!(second.history[0].plan_id, first_id);

    let summary = CommandHandler::get_planning_summary(
        &kernel,
        actor,
        intent,
        ws,
        0,
    )
    .unwrap();
    assert_eq!(summary.history.len(), 0);
    assert_eq!(summary.history_count, 1);
    assert!(summary.has_active_plan);
}

#[test]
fn planning_restart_continuity_without_replay() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive(&kernel, &ws);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let generated = CommandHandler::generate_planning_snapshot(
        &kernel,
        actor.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let plan_id = generated.current.as_ref().unwrap().plan.id.clone();

    let loaded = CommandHandler::get_planning_snapshot(
        &kernel,
        actor,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(
        loaded.current.as_ref().unwrap().plan.id,
        plan_id
    );
    assert!(loaded.is_non_commandable());
    // No fabricated history on empty prior
    assert_eq!(loaded.history_count, 0);
}

#[test]
fn planning_never_executes_and_does_not_bypass_gateway() {
    let err = WorkspacePlanningService::attempt_execute().expect_err("must fail");
    assert!(
        err.to_string().to_lowercase().contains("cannot execute")
            || err.to_string().to_lowercase().contains("planning"),
        "got {err}"
    );

    use workspace_domain::CapabilitySet;
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let mut ctx = kernel.command_context(actor, intent);
    ctx.capability_set = CapabilitySet::new();
    let err = crate::commands::pipeline::CommandPipeline::new(ctx)
        .execute_mutation(crate::commands::workspace_planning::GeneratePlanningSnapshot::new(ws))
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
fn planning_reference_integrity_to_cognitive_and_intent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_workspace(&kernel);
    seed_cognitive(&kernel, &ws);
    let snap = CommandHandler::generate_planning_snapshot(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let proposal = snap.current.unwrap();
    assert!(
        proposal
            .steps
            .iter()
            .flat_map(|s| s.evidence_refs.iter())
            .any(|r| r.starts_with("cognitive:") || r.starts_with("task:")),
        "steps must reference cognitive/task identities"
    );
    assert!(
        proposal
            .evidence_refs
            .iter()
            .any(|e| e.external_ref.starts_with("work_goal:")),
        "proposal must cite Intent WorkGoals by reference"
    );
    assert!(
        proposal.risks.iter().any(|r| r.statement.contains("risk")
            || r.evidence_refs.iter().any(|e| e.starts_with("cognitive:"))),
        "cognitive risks should surface"
    );
}
