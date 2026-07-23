//! Governed memory boundary tests (Sprints 60–61).
//!
//! Memory improves planning context. Memory must not grant permissions or execute.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::memory::CreateMemoryEntry;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
use crate::security::StandardPermissionGate;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ActorType, CapabilitySet, IntentContext, MemoryLifecycleState, MemoryType,
};

fn seed_two_apps(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Memory WS".into()))
        .unwrap();

    let app_a = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "VS Code".into(),
            None,
            Some("code.exe".into()),
        ))
        .unwrap();
    let app_b = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Terminal".into(),
            None,
            Some("wt.exe".into()),
        ))
        .unwrap();

    (
        workspace.id.to_string(),
        app_a.id.to_string(),
        app_b.id.to_string(),
    )
}

#[test]
fn case1_memory_improves_planning_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    CommandHandler::create_memory_entry(
        &kernel,
        local,
        intent,
        MemoryType::Workspace,
        "preferred_application",
        "User prefers VS Code for coding workspaces",
        "operator_console_test",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let plan = CommandHandler::diagnose_ai_plan_preview(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone(), app_b.clone()],
        Some(ws),
    )
    .unwrap();

    assert!(!plan.proposals.is_empty());
    let joined = plan
        .proposals
        .iter()
        .map(|p| p.explanation.clone().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        joined.contains("Memory:") || joined.contains("preferred from memory"),
        "planner should receive memory context in explanations: {joined}"
    );
    let first_target = plan.proposals[0]
        .target_resource
        .as_ref()
        .map(|r| r.id.as_str().to_string());
    assert_eq!(
        first_target.as_deref(),
        Some(app_a.as_str()),
        "preferred app from memory should rank first"
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.memory.created"));
}

#[test]
fn case2_memory_does_not_execute_actions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    CommandHandler::create_memory_entry(
        &kernel,
        local,
        intent,
        MemoryType::Session,
        "active_task",
        "User is preparing a coding workspace",
        "operator_console_test",
        Some(ws.clone()),
        None,
    )
    .unwrap();

    let plan = CommandHandler::diagnose_ai_plan_preview(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        Some(ws),
    )
    .unwrap();
    assert!(!plan.proposals.is_empty());

    let records = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == ActorType::AIAssistant
    }));
}

#[test]
fn case3_memory_does_not_grant_permissions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert!(!before.contains(&workspace_domain::Capability::application_launch()));
    assert!(!before.contains(&workspace_domain::Capability::memory_write()));

    CommandHandler::create_memory_entry(
        &kernel,
        local,
        intent,
        MemoryType::UserPreference,
        "workflow_style",
        "User prefers sequential workspace prep",
        "operator_console_test",
        Some(ws.clone()),
        None,
    )
    .unwrap();

    let after = CapabilitySet::for_actor_type(ActorType::AIAssistant);
    assert_eq!(before, after);

    let submitted = CommandHandler::submit_ai_plan(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b],
        Some(ws),
    )
    .unwrap();

    for submission in &submitted.submissions {
        assert!(
            matches!(
                submission.outcome,
                workspace_domain::AiProposalAuthorityOutcome::ApprovalRequired { .. }
                    | workspace_domain::AiProposalAuthorityOutcome::Denied { .. }
            ),
            "gateway behavior must remain unchanged with memory present"
        );
    }
}

#[test]
fn case4_deleted_memory_is_removed_from_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, app_b) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let entry = CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "preferred_application",
        "Prefer Terminal",
        "operator_console_test",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_b}"}}"#)),
    )
    .unwrap();

    let deleted = CommandHandler::delete_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        entry.id.to_string(),
    )
    .unwrap();
    assert_eq!(deleted.lifecycle, MemoryLifecycleState::Deleted);

    let awareness = CommandHandler::get_memory_context(
        &kernel,
        local,
        intent,
        Some(ws.clone()),
        Some(20),
    )
    .unwrap();
    assert!(
        !awareness
            .entries
            .iter()
            .any(|e| e.id == entry.id && e.is_active()),
        "deleted memory must not appear in active context"
    );

    let plan = CommandHandler::diagnose_ai_plan_preview(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a, app_b.clone()],
        Some(ws),
    )
    .unwrap();
    let joined = plan
        .proposals
        .iter()
        .map(|p| p.explanation.clone().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !joined.contains("Prefer Terminal"),
        "deleted memory must not influence future planning: {joined}"
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "ai.memory.deleted"));
}

#[test]
fn case5_unauthorized_memory_mutation_rejected() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();

    let ai = ActorContext::new(Actor::ai_assistant("ai-memory-case-5").unwrap());
    let error = CommandPipeline::new(crate::commands::CommandContext {
        actor_context: ai,
        intent_context: IntentContext::ai_suggestion(),
        capability_set: CapabilitySet::new(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: &bus,
        permission_gate: &StandardPermissionGate,
        permission_policy: &CapabilityBoundPolicy,
    })
    .execute_mutation(CreateMemoryEntry::new(
        MemoryType::Session,
        "hidden".into(),
        "AI must not write memory without authority".into(),
        "ai_self".into(),
        None,
        None,
    ))
    .unwrap_err();

    assert!(matches!(
        error,
        KernelError::ApprovalRequired { .. } | KernelError::PermissionDenied(_)
    ));

    // Even with AllowAll policy, StandardPermissionGate still requires approval for AI.
    let error_gate = CommandPipeline::new(crate::commands::CommandContext {
        actor_context: ActorContext::new(Actor::ai_assistant("ai-memory-case-5b").unwrap()),
        intent_context: IntentContext::ai_suggestion(),
        capability_set: CapabilitySet::local_user_standard(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: &bus,
        permission_gate: &StandardPermissionGate,
        permission_policy: &AlwaysAllowPolicy,
    })
    .execute_mutation(CreateMemoryEntry::new(
        MemoryType::Session,
        "still_blocked".into(),
        "AI cannot self-write memory".into(),
        "ai_self".into(),
        None,
        None,
    ))
    .unwrap_err();
    assert!(matches!(
        error_gate,
        KernelError::ApprovalRequired { .. } | KernelError::PermissionDenied(_)
    ));
}

#[test]
fn case6_memory_separate_from_audit_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_two_apps(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::SystemKnowledge,
        "capability_note",
        "LaunchApplication requires application.launch — informational only",
        "operator_console_test",
        Some(ws),
        None,
    )
    .unwrap();

    let awareness = CommandHandler::get_memory_context(&kernel, local, intent, None, Some(10))
        .unwrap();
    assert!(!awareness.entries.is_empty());

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    let created = records
        .iter()
        .find(|r| r.event_type == "ai.memory.created")
        .expect("memory create audit");
    let metadata = created.metadata.as_deref().unwrap_or("");
    assert!(
        metadata.contains("\"authority_effect\":\"none\"")
            || metadata.contains("\"authority_effect\": \"none\""),
        "memory audits must declare no authority effect: {metadata}"
    );

    // Memory presence must not change AI nominal capabilities.
    assert!(!CapabilitySet::for_actor_type(ActorType::AIAssistant)
        .contains(&workspace_domain::Capability::application_launch()));

    // Direct AI launch still requires approval — memory cannot influence gateway.
    let launch = CommandHandler::submit_ai_application_launch(
        &kernel,
        "diagnostic-ai",
        app_a,
        Some("memory must not authorize".into()),
    );
    assert!(matches!(launch, Err(KernelError::ApprovalRequired { .. })));
}
