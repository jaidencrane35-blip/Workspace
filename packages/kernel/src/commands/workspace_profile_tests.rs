//! Workspace Environment Profile tests (Phase 6 Batch 3).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
    WorkspaceProfileMemberInput, WorkspaceProfileMemberType, WorkspaceProfileRelationship,
    WorkspaceProfileState,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Profile WS".into()))
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
        "Ship Profile Model".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    (workspace_id, project.id.to_string(), task.id.to_string())
}

fn sample_members(project_id: &str, task_id: &str) -> Vec<WorkspaceProfileMemberInput> {
    vec![
        WorkspaceProfileMemberInput {
            member_type: WorkspaceProfileMemberType::Application,
            reference_id: "vscode".into(),
            relationship: WorkspaceProfileRelationship::Expected,
            evidence: "Development setups usually include VS Code".into(),
            label: "VS Code".into(),
        },
        WorkspaceProfileMemberInput {
            member_type: WorkspaceProfileMemberType::Application,
            reference_id: "terminal".into(),
            relationship: WorkspaceProfileRelationship::Expected,
            evidence: "Development setups usually include a terminal".into(),
            label: "Terminal".into(),
        },
        WorkspaceProfileMemberInput {
            member_type: WorkspaceProfileMemberType::Project,
            reference_id: project_id.into(),
            relationship: WorkspaceProfileRelationship::Preferred,
            evidence: "Profile prefers this project".into(),
            label: "Workspace AI Platform".into(),
        },
        WorkspaceProfileMemberInput {
            member_type: WorkspaceProfileMemberType::Task,
            reference_id: task_id.into(),
            relationship: WorkspaceProfileRelationship::Related,
            evidence: "Related active task".into(),
            label: "Ship Profile Model".into(),
        },
    ]
}

/// CASE 1 — Create profile.
#[test]
fn case1_create_profile() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    let profile = CommandHandler::create_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "Development setup".into(),
        "Preferred coding environment".into(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    assert_eq!(profile.name, "Development setup");
    assert_eq!(profile.workspace_id.as_str(), ws);
    assert_eq!(profile.authority_effect, "none");
    assert_eq!(profile.members.len(), 4);
}

/// CASE 2 — Profile references existing workspace entities.
#[test]
fn case2_profile_references_existing_entities() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    let profile = CommandHandler::create_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        "Development setup".into(),
        String::new(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    assert!(profile
        .members
        .iter()
        .any(|m| m.member_type == WorkspaceProfileMemberType::Project
            && m.reference_id == project_id));
    assert!(profile
        .members
        .iter()
        .any(|m| m.member_type == WorkspaceProfileMemberType::Task
            && m.reference_id == task_id));
}

/// CASE 3 — Profile comparison is deterministic.
#[test]
fn case3_comparison_is_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    CommandHandler::create_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "Development setup".into(),
        String::new(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_profile_state(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b =
        CommandHandler::generate_workspace_profile_state(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.profile_count, b.profile_count);
    assert_eq!(
        a.comparisons
            .iter()
            .map(|c| (&c.profile_id, c.alignment, c.matched_count, c.missing_count))
            .collect::<Vec<_>>(),
        b.comparisons
            .iter()
            .map(|c| (&c.profile_id, c.alignment, c.matched_count, c.missing_count))
            .collect::<Vec<_>>()
    );
    assert_eq!(a.authority_effect, "none");
}

/// CASE 4 — Profile cannot execute actions.
#[test]
fn case4_cannot_execute() {
    match CommandHandler::workspace_profiles_attempt_execute() {
        Err(KernelError::WorkspaceProfileValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("restore")
                    || message.contains("launch")
            );
        }
        other => panic!("expected WorkspaceProfileValidation, got {other:?}"),
    }
    assert_eq!(WorkspaceProfileState::AUTHORITY_EFFECT_NONE, "none");
}

/// CASE 5 — Profile cannot bypass Gateway.
#[test]
fn case5_cannot_bypass_gateway() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "workspace_profile")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceProfileService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::DurableStore);
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    CommandHandler::create_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "Development setup".into(),
        String::new(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_profile_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let report = CommandHandler::validate_workspace_profile_state(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        &state,
    )
    .unwrap();
    assert!(report.valid);
    assert_eq!(report.authority_effect, "none");
}

/// CASE 6 — Deleting/clearing source focus changes comparison results.
#[test]
fn case6_source_change_updates_comparison() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_workspace_profile(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Development setup".into(),
        String::new(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    let before = CommandHandler::generate_workspace_profile_state(
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
        CommandHandler::generate_workspace_profile_state(&kernel, local, intent, ws).unwrap();
    let comparison = CommandHandler::compare_workspace_profile_states(&before, &after);
    assert!(!comparison.differences.is_empty());
    let before_matched = before.comparisons.first().map(|c| c.matched_count);
    let after_matched = after.comparisons.first().map(|c| c.matched_count);
    assert_ne!(before_matched, after_matched);
}

/// CASE 7 — Profiles survive restart (durable SQLite).
#[test]
fn case7_profiles_survive_restart() {
    let path = std::env::temp_dir().join(format!(
        "workspace-profile-test-{}.db",
        uuid::Uuid::new_v4()
    ));
    let ws = {
        let kernel = WorkspaceKernel::initialize(&path).unwrap();
        let (ws, project_id, task_id) = seed(&kernel);
        CommandHandler::create_workspace_profile(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws.clone(),
            "Writing setup".into(),
            "Docs and notes".into(),
            sample_members(&project_id, &task_id),
        )
        .unwrap();
        ws
    };
    let kernel2 = WorkspaceKernel::initialize(&path).unwrap();
    let listed = CommandHandler::list_workspace_profiles(
        &kernel2,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
        Some(10),
    )
    .unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "Writing setup");
    assert_eq!(listed[0].members.len(), 4);
    let _ = std::fs::remove_file(&path);
}

/// CASE 8 — Profile does not duplicate Task Graph ownership.
#[test]
fn case8_does_not_duplicate_task_graph() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "workspace_task" || c.concept == "workspace_profile")
        .collect();
    assert!(owners.iter().any(|c| c.concept == "workspace_task"));
    assert!(owners.iter().any(|c| c.concept == "workspace_profile"));
    assert_ne!(
        owners
            .iter()
            .find(|c| c.concept == "workspace_task")
            .map(|c| c.owner),
        owners
            .iter()
            .find(|c| c.concept == "workspace_profile")
            .map(|c| c.owner)
    );
}

/// CASE 9 — Profile does not duplicate Memory ownership.
#[test]
fn case9_does_not_duplicate_memory() {
    let memory = PLATFORM_CONCEPT_OWNERS
        .iter()
        .find(|c| c.concept == "memory" || c.concept == "ai_memory" || c.concept.contains("memory"));
    let profile = PLATFORM_CONCEPT_OWNERS
        .iter()
        .find(|c| c.concept == "workspace_profile")
        .unwrap();
    if let Some(memory) = memory {
        assert_ne!(memory.owner, profile.owner);
    }
    assert_eq!(profile.kind, ConceptOwnerKind::DurableStore);
}

/// CASE 10 — Profile remains user-controlled.
#[test]
fn case10_profile_remains_user_controlled() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed(&kernel);
    let created = CommandHandler::create_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "Research setup".into(),
        "Initial".into(),
        sample_members(&project_id, &task_id),
    )
    .unwrap();
    let updated = CommandHandler::update_workspace_profile(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        created.id.to_string(),
        Some("Research setup (revised)".into()),
        Some("User revised description".into()),
        None,
        Some(vec![WorkspaceProfileMemberInput {
            member_type: WorkspaceProfileMemberType::Project,
            reference_id: project_id,
            relationship: WorkspaceProfileRelationship::Expected,
            evidence: "User chose this project".into(),
            label: "Project".into(),
        }]),
    )
    .unwrap();
    assert_eq!(updated.name, "Research setup (revised)");
    assert_eq!(updated.members.len(), 1);
    assert_eq!(updated.authority_effect, "none");
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.profiles.profile_count >= 1);
    assert_eq!(intel.profiles.authority_effect, "none");
}
