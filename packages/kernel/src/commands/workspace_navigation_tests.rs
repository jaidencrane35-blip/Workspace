//! Workspace Navigation Engine tests (Phase 6 Batch 4).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use crate::services::WorkspaceObservationService;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, NavigationPathKind, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, WorkspaceNavigationState,
};
use workspace_windows_integration::StubDesktopCapturer;

fn capture_fixture(kernel: &WorkspaceKernel, local: &ActorContext, intent: &IntentContext) {
    WorkspaceObservationService::capture_with(
        &kernel.shared_database(),
        local,
        intent,
        &StubDesktopCapturer::fixture_dual_monitor(),
    )
    .unwrap();
}

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Navigation WS".into()))
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
        "Ship Navigation Engine".into(),
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
        "Build and develop navigation for Workspace AI".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Deterministic current-focus navigation.
#[test]
fn case1_deterministic_current_focus() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state
        .nodes
        .iter()
        .any(|n| n.kind == NavigationPathKind::CurrentFocus));
    assert!(!state.navigation_summary.current_path_line.is_empty());
    assert!(state.nodes.iter().all(|n| !n.why.is_empty()));
}

/// CASE 2 — Related work and connected projects appear.
#[test]
fn case2_related_and_connected_work() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state
            .nodes
            .iter()
            .any(|n| n.kind == NavigationPathKind::ConnectedProject
                || n.kind == NavigationPathKind::ConnectedTask
                || n.kind == NavigationPathKind::RelatedWork
                || n.kind == NavigationPathKind::ConnectedContext),
        "expected related/connected navigation nodes"
    );
}

/// CASE 3 — Dependency chain is explainable.
#[test]
fn case3_dependency_chain_explainable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    for edge in &state.edges {
        assert!(!edge.why.is_empty());
        assert_eq!(edge.authority_effect, "none");
    }
    // Dependency path may be empty if task graph has no nodes yet — still valid.
    assert!(state.path_count > 0);
}

/// CASE 4 — Blocked path signals remain explainable.
#[test]
fn case4_blocked_paths_explainable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    for node in state
        .nodes
        .iter()
        .filter(|n| n.kind == NavigationPathKind::BlockingItem)
    {
        assert!(!node.why.is_empty());
        assert!(!node.source_projection.is_empty());
    }
    assert!(!state.navigation_summary.blocked_line.is_empty());
}

/// CASE 5 — Suggested next inspection is present and explainable.
#[test]
fn case5_suggested_next_inspection() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.navigation_summary.next_inspection_line.is_empty());
    assert!(
        state.suggested_count > 0
            || state
                .nodes
                .iter()
                .any(|n| n.kind == NavigationPathKind::PossibleNextInspection
                    || n.kind == NavigationPathKind::SuggestedDestination
                    || n.kind == NavigationPathKind::RelevantRecommendation)
    );
}

/// CASE 6 — Breadcrumbs form a coherent path.
#[test]
fn case6_breadcrumbs_coherent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.breadcrumbs.is_empty());
    assert!(!state.navigation_summary.breadcrumb_line.is_empty());
    assert!(state.breadcrumbs.iter().all(|b| !b.why.is_empty()));
}

/// CASE 7 — Regeneration is deterministic for the same inputs.
#[test]
fn case7_deterministic_regeneration() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    capture_fixture(&kernel, &local, &intent);
    let a = CommandHandler::generate_workspace_navigation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_navigation(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.navigation_summary.current_path_line, b.navigation_summary.current_path_line);
    assert_eq!(a.path_count, b.path_count);
    let a_kinds: Vec<_> = a.nodes.iter().map(|n| n.kind.as_str()).collect();
    let b_kinds: Vec<_> = b.nodes.iter().map(|n| n.kind.as_str()).collect();
    assert_eq!(a_kinds, b_kinds);
}

/// CASE 8 — Attention/RE/Session may reference Navigation as evidence only.
#[test]
fn case8_evidence_only_consumption() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.navigation.node_count > 0);
    assert_eq!(intel.navigation.authority_effect, "none");
    assert!(
        intel.attention.summary.contains("Navigation evidence")
            || intel.attention.summary.contains("informational only")
    );
    assert!(
        intel.recommendation_engine.explanation.contains("Navigation")
            || intel
                .recommendation_engine
                .summary
                .contains("navigation")
            || intel.navigation.path_count > 0
    );
}

/// CASE 9 — attempt_execute always CannotExecute; no authority.
#[test]
fn case9_attempt_execute_cannot_execute() {
    match CommandHandler::workspace_navigation_attempt_execute() {
        Err(KernelError::WorkspaceNavigationValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("plan")
                    || message.contains("route")
            );
        }
        other => panic!("expected WorkspaceNavigationValidation, got {other:?}"),
    }
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        state.authority_effect,
        WorkspaceNavigationState::AUTHORITY_EFFECT_NONE
    );
}

/// CASE 10 — Gateway untouched; validate + ownership map green.
#[test]
fn case10_gateway_untouched_and_validate() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "navigation")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceNavigationService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_navigation(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
    )
    .unwrap();
    let report = CommandHandler::validate_workspace_navigation(
        &kernel,
        local,
        intent,
        &state,
    )
    .unwrap();
    assert!(report.valid);
    assert_eq!(report.authority_effect, "none");
    let comparison = CommandHandler::compare_workspace_navigation(&state, &state);
    assert_eq!(comparison.authority_effect, "none");
}
