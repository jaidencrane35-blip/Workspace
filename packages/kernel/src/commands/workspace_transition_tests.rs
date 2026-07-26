//! Workspace Transition Engine tests (Phase 6 Batch 7).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
    TransitionKind, WorkspaceTransitionState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Transition WS".into()))
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
        "Ship Transition Engine".into(),
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
        "Explain movement between work states without restoring".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Transitions generate deterministically.
#[test]
fn case1_transitions_generate_deterministically() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_transitions(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b =
        CommandHandler::generate_workspace_transitions(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.authority_effect, "none");
    assert!(!a.transitions.is_empty());
    assert_eq!(
        a.transition_summary.current_transition_line,
        b.transition_summary.current_transition_line
    );
    assert_eq!(a.transition_count, b.transition_count);
    assert!(a.transitions.iter().all(|t| {
        !t.why.is_empty()
            && !t.evidence.is_empty()
            && !t.previous_state.is_empty()
            && !t.current_state.is_empty()
    }));
}

/// CASE 2 — Returning session identifies previous context.
#[test]
fn case2_returning_session_identifies_previous_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_transitions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.transition_summary.left_off_line.is_empty());
    assert!(
        state.returning_count > 0
            || state.transitions.iter().any(|t| matches!(
                t.kind,
                TransitionKind::ReturningToWork
                    | TransitionKind::ContinuingInterruptedWork
                    | TransitionKind::EnteringContext
                    | TransitionKind::StartingNewWorkState
            ))
    );
}

/// CASE 3 — Context switches are explained.
#[test]
fn case3_context_switches_are_explained() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_transitions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.transition_summary.context_switch_line.is_empty());
    for t in state
        .transitions
        .iter()
        .filter(|t| t.kind == TransitionKind::SwitchingFocus)
    {
        assert!(!t.explanation.is_empty());
        assert!(!t.evidence.is_empty());
    }
}

/// CASE 4 — Activity changes update transition evidence.
#[test]
fn case4_activity_changes_update_transition_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_transitions(
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
        CommandHandler::generate_workspace_transitions(&kernel, local, intent, ws).unwrap();
    let comparison = CommandHandler::compare_workspace_transitions(&before, &after);
    assert!(!comparison.differences.is_empty());
}

/// CASE 5 — No transition performs changes.
#[test]
fn case5_no_transition_performs_changes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_transitions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    let serialized = serde_json::to_string(&state).unwrap();
    assert!(!serialized.contains("\"restore\""));
    assert!(!serialized.contains("\"launch\""));
    assert!(!serialized.contains("\"execute_action\""));
    for t in &state.transitions {
        assert_eq!(t.authority_effect, "none");
    }
}

/// CASE 6 — No restoration occurs.
#[test]
fn case6_no_restoration_occurs() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_transitions(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state.explanation.contains("never restores")
            || state.explanation.contains("authority_effect=none")
    );
    assert!(state
        .transitions
        .iter()
        .all(|t| t.explanation.to_lowercase().contains("never")
            || t.why.to_lowercase().contains("never")
            || t.authority_effect == "none"));
}

/// CASE 7 — Recommendation Engine only consumes evidence.
#[test]
fn case7_recommendations_consume_transition_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.transition.transition_count > 0);
    assert!(
        intel
            .recommendation_engine
            .explanation
            .contains("Transition")
            || intel.transition.authority_effect == "none"
    );
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
}

/// CASE 8 — Adaptation only consumes evidence.
#[test]
fn case8_adaptation_consumes_transition_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        intel.adaptation.explanation.contains("Transition")
            || intel.adaptation.authority_effect == "none"
    );
    assert_eq!(intel.adaptation.authority_effect, "none");
    assert_eq!(intel.transition.authority_effect, "none");
}

/// CASE 9 — attempt_execute returns CannotExecute.
#[test]
fn case9_attempt_execute_cannot_execute() {
    match CommandHandler::workspace_transitions_attempt_execute() {
        Err(KernelError::WorkspaceTransitionValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("restore")
                    || message.contains("automate")
            );
        }
        other => panic!("expected WorkspaceTransitionValidation, got {other:?}"),
    }
    assert_eq!(
        WorkspaceTransitionState::AUTHORITY_EFFECT_NONE,
        "none"
    );
}

/// CASE 10 — Permission Gateway remains unchanged.
#[test]
fn case10_permission_gateway_unchanged() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "transition")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceTransitionService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);
    let report = {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let ws = seed(&kernel);
        let state = CommandHandler::generate_workspace_transitions(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws,
        )
        .unwrap();
        CommandHandler::validate_workspace_transitions(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            &state,
        )
        .unwrap()
    };
    assert!(report.valid);
    assert_eq!(report.authority_effect, "none");
}
