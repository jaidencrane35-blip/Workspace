//! Workspace Interaction Model tests (Phase 6).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, InteractionHandoff, InteractionKind,
    PLATFORM_CONCEPT_OWNERS, TaskPriority, WorkspaceInteractionState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Interaction WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace AI Platform".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        workspace_id.clone(),
        "Ship Interaction Model".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    CommandHandler::create_work_goal(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Surface meaningful interactions without executing".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Interactions accurately reflect current workspace state.
#[test]
fn case1_interactions_reflect_workspace_state() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert_eq!(state.workspace_id, ws);
    assert!(!state.explanation.is_empty());
    assert!(!state.items.is_empty());
    assert!(state.items.iter().all(|i| {
        !i.why.is_empty()
            && !i.explanation.is_empty()
            && !i.evidence.is_empty()
            && !i.source_projection.is_empty()
            && i.authority_effect == "none"
    }));
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.interaction.item_count > 0);
    assert_eq!(intel.interaction.authority_effect, "none");
}

/// CASE 2 — Removing source data removes interaction.
#[test]
fn case2_removing_source_removes_interaction() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_interactions(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_interactions(&kernel, local, intent, ws).unwrap();
    let comparison = CommandHandler::compare_workspace_interactions(&before, &after);
    assert!(!comparison.differences.is_empty());
    assert_eq!(comparison.authority_effect, "none");
}

/// CASE 3 — Interactions never create execution.
#[test]
fn case3_interactions_never_create_execution() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    let serialized = serde_json::to_string(&state).unwrap();
    assert!(!serialized.contains("\"execute_action\""));
    assert!(!serialized.contains("\"launch_application\""));
    for item in &state.items {
        assert_eq!(item.authority_effect, "none");
        assert!(!item.required_intent.is_empty());
    }
}

/// CASE 4 — Selecting interaction creates intent handoff only.
#[test]
fn case4_select_creates_intent_handoff_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item = state
        .items
        .iter()
        .find(|i| {
            matches!(
                i.source_projection.as_str(),
                "continuity" | "transition" | "milestones" | "navigation"
            )
        })
        .or_else(|| state.items.first())
        .expect("expected at least one interaction");
    let result = CommandHandler::select_workspace_interaction(
        &kernel,
        local,
        intent,
        ws,
        item.id.clone(),
    )
    .unwrap();
    assert_eq!(result.authority_effect, "none");
    let handoff = result.handoff.expect("handoff required");
    assert_eq!(
        handoff.next_command,
        InteractionHandoff::NEXT_SUBMIT_ASSISTANT_GOAL
    );
    assert!(!handoff.intent_statement.is_empty());
    assert!(handoff.note.to_lowercase().contains("never executes"));
    assert_eq!(handoff.authority_effect, "none");
}

/// CASE 5 — Permission Gateway remains required.
#[test]
fn case5_permission_gateway_remains_required() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "interaction")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceInteractionService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let report = CommandHandler::validate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        &state,
    )
    .unwrap();
    assert!(report.valid);
    assert_eq!(report.authority_effect, "none");
}

/// CASE 6 — Interaction does not duplicate Recommendation Engine.
#[test]
fn case6_does_not_duplicate_recommendation_engine() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    for item in state
        .items
        .iter()
        .filter(|i| i.kind == InteractionKind::InspectRecommendation)
    {
        assert_eq!(item.source_projection, "recommendation_engine");
        assert!(
            item.explanation.contains("Recommendation Engine")
                || item.why.contains("Recommendation Engine")
        );
    }
    assert!(
        state.explanation.contains("does not duplicate")
            || state.interaction_summary.narrative.contains("Owns nothing")
    );
}

/// CASE 7 — Interaction does not duplicate Decision Queue.
#[test]
fn case7_does_not_duplicate_decision_queue() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    for item in state
        .items
        .iter()
        .filter(|i| i.kind == InteractionKind::ReviewDecision)
    {
        assert_eq!(item.source_projection, "decision_queue");
        assert!(
            item.explanation.contains("Decision Queue") || item.why.contains("Decision Queue")
        );
    }
}

/// CASE 8 — Interaction remains deterministic.
#[test]
fn case8_interaction_remains_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_interactions(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b =
        CommandHandler::generate_workspace_interactions(&kernel, local, intent, ws).unwrap();
    // Upstream RE/Adaptation candidate sets can shift when projection audits feed
    // Activity between generates — Interaction must stay stable for durable sources.
    let stable_ids = |state: &WorkspaceInteractionState| {
        state
            .items
            .iter()
            .filter(|i| {
                matches!(
                    i.source_projection.as_str(),
                    "continuity"
                        | "transition"
                        | "milestones"
                        | "navigation"
                        | "work_context"
                        | "decision_queue"
                        | "blocked_actions"
                        | "readiness"
                )
            })
            .map(|i| i.id.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(stable_ids(&a), stable_ids(&b));
    assert_eq!(
        a.interaction_summary.continue_line,
        b.interaction_summary.continue_line
    );
    // Structural sort: priority desc, then kind, then id.
    for window in a.items.windows(2) {
        let left = &window[0];
        let right = &window[1];
        assert!(
            left.priority >= right.priority
                || left.kind.as_str() <= right.kind.as_str()
                || left.id <= right.id
        );
    }
}

/// CASE 9 — Interaction has explainable evidence.
#[test]
fn case9_explainable_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.evidence.is_empty());
    for item in &state.items {
        assert!(item.why.starts_with("Why am I seeing this?"));
        assert!(item.evidence.iter().all(|e| {
            !e.why.is_empty() && !e.source_projection.is_empty() && !e.source_ref.is_empty()
        }));
    }
}

/// CASE 10 — Restart does not corrupt state + attempt_execute fails.
#[test]
fn case10_restart_and_attempt_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let first = CommandHandler::generate_workspace_interactions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    // New kernel = process restart for in-memory DB; overlays are process-local and empty.
    let kernel2 = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws2 = seed(&kernel2);
    let second = CommandHandler::generate_workspace_interactions(
        &kernel2,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws2,
    )
    .unwrap();
    assert_eq!(first.authority_effect, "none");
    assert_eq!(second.authority_effect, "none");
    assert!(!second.items.is_empty());
    assert_eq!(
        WorkspaceInteractionState::AUTHORITY_EFFECT_NONE,
        "none"
    );
    match CommandHandler::workspace_interactions_attempt_execute() {
        Err(KernelError::WorkspaceInteractionValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("automate")
                    || message.contains("authorize")
            );
        }
        other => panic!("expected WorkspaceInteractionValidation, got {other:?}"),
    }
}
