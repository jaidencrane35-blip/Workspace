//! Workspace Experience Layer tests (Phase 6).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, ExperienceVisibility, IntentContext, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, WorkspaceExperienceState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Experience WS".into()))
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
        "Ship Experience".into(),
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

/// CASE 1 — Experience owns no data.
#[test]
fn case1_experience_owns_no_data() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_experience(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.explanation.contains("owns no cognition data")
        || state.explanation.contains("projected solely from WorkspaceSessionState"));
    assert!(state.evidence.iter().any(|e| e.contains("Session")));
    assert!(state.sections.iter().all(|s| !s.source_session_field.is_empty()));
    assert!(state.sections.iter().flat_map(|s| s.items.iter()).all(|i| {
        !i.source_session_field.is_empty() && i.authority_effect == "none"
    }));
}

/// CASE 2 — Experience consumes Session only.
#[test]
fn case2_experience_consumes_session_only() {
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
    let experience = CommandHandler::generate_workspace_experience(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(!experience.session_generated_at.is_empty());
    assert_eq!(experience.workspace_id, session.workspace_id);
    assert!(experience.explanation.contains("WorkspaceSessionState"));
    assert!(!experience.evidence.iter().any(|e| e.contains("Intelligence generated_at")));
}

/// CASE 3 — No duplicate ownership.
#[test]
fn case3_no_duplicate_ownership() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "experience")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceExperienceService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session" && c.owner == "WorkspaceSessionService"
    }));
}

/// CASE 4 — No authority.
#[test]
fn case4_no_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_experience(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        state.authority_effect,
        WorkspaceExperienceState::AUTHORITY_EFFECT_NONE
    );
    match CommandHandler::workspace_experience_attempt_execute() {
        Err(KernelError::WorkspaceExperienceValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("prepare")
                    || message.contains("restore")
            );
        }
        other => panic!("expected WorkspaceExperienceValidation, got {other:?}"),
    }
}

/// CASE 5 — Session remains canonical.
#[test]
fn case5_session_remains_canonical() {
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
    let experience = CommandHandler::generate_workspace_experience(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_ne!(session.summary, experience.summary);
    assert!(experience.session_generated_at.len() > 0);
    // Experience mirrors Session focus lines, does not replace Session.
    assert_eq!(
        experience.experience_summary.focus_line,
        session.session_summary.doing_line
    );
}

/// CASE 6 — UI updates correctly as Session changes (experience regenerates with focus).
#[test]
fn case6_updates_when_session_changes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_experience(
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
    let second = CommandHandler::generate_workspace_experience(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_ne!(first.generated_at, second.generated_at);
    assert!(second.experience_summary.focus_line.contains("Second Project"));
}

/// CASE 7 — No circular dependencies.
#[test]
fn case7_no_circular_dependencies() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "experience" && c.owner == "WorkspaceExperienceService"
    }));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session" && c.owner == "WorkspaceSessionService"
    }));
    // Experience must not claim session ownership.
    assert!(!PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "session" && c.owner.contains("Experience")
    }));
}

/// CASE 8 — No cognition regressions (Intelligence + Session still green).
#[test]
fn case8_no_cognition_regressions() {
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
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let experience = CommandHandler::generate_workspace_experience(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(session.authority_effect, "none");
    assert_eq!(experience.authority_effect, "none");
    assert_eq!(experience.section_count, 9);
}

/// CASE 9 — No governance regressions.
#[test]
fn case9_no_governance_regressions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_experience(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(state.immediate_count + state.highlighted_count + state.collapsed_count
        + state.deferred_count
        >= state.section_count);
    assert!(state.sections.iter().any(|s| {
        s.visibility == ExperienceVisibility::Immediate
            || s.visibility == ExperienceVisibility::Highlighted
    }));
    let left = state.clone();
    let right = state;
    let comparison = CommandHandler::compare_workspace_experiences(&left, &right);
    assert_eq!(comparison.authority_effect, "none");
}

/// CASE 10 — Existing cognition and platform tests remain green (smoke).
#[test]
fn case10_platform_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let readiness = CommandHandler::generate_workspace_readiness(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let experience = CommandHandler::generate_workspace_experience(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(readiness.authority_effect, "none");
    assert_eq!(experience.authority_effect, "none");
    assert!(!experience.experience_summary.headline.is_empty());
}
