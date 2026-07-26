//! Workspace Session Engine tests (Phase 6).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, SessionMember,
    TaskPriority, WorkspaceSessionState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Session WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace Platform".into(),
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
        "Ship Session".into(),
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
        "Build Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Session never owns source data.
#[test]
fn case1_session_never_owns_source_data() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_session(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.explanation.contains("owns no source data"));
    assert!(state.evidence.iter().any(|e| e.contains("Intelligence")));
    assert!(state.members.iter().all(|m| {
        !m.source_projection.is_empty()
            && !m.why.is_empty()
            && m.authority_effect == SessionMember::AUTHORITY_EFFECT_NONE
    }));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session"
            && c.owner == "WorkspaceSessionService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

/// CASE 2 — Session updates when projections change.
#[test]
fn case2_session_updates_when_projections_change() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_session(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Second Project".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        ws.clone(),
        "Follow-up".into(),
        TaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_session(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_ne!(first.generated_at, second.generated_at);
    assert_eq!(
        second.focus.project_label.as_deref(),
        Some("Second Project")
    );
}

/// CASE 3 — No duplicate ownership.
#[test]
fn case3_no_duplicate_ownership() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "session")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceSessionService");
    assert!(!PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session" && c.owner.contains("Intelligence")
    }));
}

/// CASE 4 — No authority.
#[test]
fn case4_no_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_session(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        state.authority_effect,
        WorkspaceSessionState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(state.focus.authority_effect, "none");
    match CommandHandler::workspace_session_attempt_execute() {
        Err(KernelError::WorkspaceSessionValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("prepare")
                    || message.contains("restore")
            );
        }
        other => panic!("expected WorkspaceSessionValidation, got {other:?}"),
    }
}

/// CASE 5 — Readiness remains independent.
#[test]
fn case5_readiness_remains_independent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let session = CommandHandler::generate_workspace_session(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let readiness = CommandHandler::generate_workspace_readiness(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(
        session.readiness.overall_status,
        readiness.overall_status
    );
    assert_eq!(session.readiness.source_projection, "readiness");
    assert_ne!(session.summary, readiness.summary);
}

/// CASE 6 — Recommendations remain independent.
#[test]
fn case6_recommendations_remain_independent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let session = CommandHandler::generate_workspace_session(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let recommendations = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(session
        .recommendations
        .iter()
        .all(|r| r.source_projection == "recommendation_engine"));
    assert_ne!(session.summary, recommendations.summary);
}

/// CASE 7 — Decision Queue remains independent.
#[test]
fn case7_decision_queue_remains_independent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let session = CommandHandler::generate_workspace_session(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let queue = CommandHandler::generate_decision_queue(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(session
        .decisions
        .iter()
        .all(|d| d.source_projection == "decision_queue"));
    assert_eq!(queue.authority_effect, "none");
    assert_ne!(session.summary, format!("{:?}", queue.pending_count));
}

/// CASE 8 — Session survives regression with Intelligence path.
#[test]
fn case8_session_survives_regression() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let session = CommandHandler::generate_workspace_session(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(session.authority_effect, "none");
    assert!(!session.session_summary.headline.is_empty());
    assert!(!session.intelligence_generated_at.is_empty());
}

/// CASE 9 — No circular dependencies (Session consumes Intelligence; Intelligence does not embed Session).
#[test]
fn case9_no_circular_dependencies() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    // Intelligence remains the understanding envelope — Session is not a field on it.
    let _ = intel.readiness;
    let _ = intel.recommendation_engine;
    let _ = intel.decision_queue;
    // Session ownership is aggregator-only, not Intelligence.
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session" && c.owner == "WorkspaceSessionService"
    }));
}

/// CASE 10 — Cognition surfaces remain green with Session present.
#[test]
fn case10_cognition_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let session = CommandHandler::generate_workspace_session(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let left = session.clone();
    let right = CommandHandler::generate_workspace_session(
        &kernel, local, intent, ws,
    )
    .unwrap();
    let comparison = CommandHandler::compare_workspace_sessions(&left, &right);
    assert_eq!(comparison.authority_effect, "none");
    assert!(!comparison.differences.is_empty());
    assert_eq!(session.health.source_projection, "workspace_health+readiness");
}
