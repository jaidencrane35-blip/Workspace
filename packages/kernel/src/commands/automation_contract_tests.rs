//! Governed Automation Contract foundation tests (Phase 4 Batch 6).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{AuditService, PermissionApprovalService};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, AutomationContractApprovalState, AutomationContractStatus,
    AutomationTriggerKind, CapabilitySet, IntentContext, TaskPriority,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Automation WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Coding".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        kernel,
        local,
        intent,
        project.id.to_string(),
        workspace_id.clone(),
        "Open env".into(),
        TaskPriority::Medium,
    )
    .unwrap();
    (workspace_id, project.id.to_string(), task.id.to_string())
}

/// CASE 1 — Automation contracts persist correctly.
#[test]
fn case1_automation_contracts_persist() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("automation-persist.db");
    let contract_id;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let (ws, project_id, _) = seed_project(&kernel);
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let contract = CommandHandler::create_automation_contract(
            &kernel,
            local,
            intent,
            ws,
            project_id,
            None,
            "Prepare coding environment".into(),
            Some("User-approved future workflow".into()),
            AutomationTriggerKind::Manual,
            None,
            "When ready, request opening my development environment.".into(),
            vec!["application.launch".into()],
        )
        .unwrap();
        contract_id = contract.id.to_string();
        assert_eq!(contract.status, AutomationContractStatus::Draft);
        assert_eq!(
            contract.approval_state,
            AutomationContractApprovalState::NotApproved
        );
    }
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let loaded =
            CommandHandler::get_automation_contract(&kernel, local, intent, contract_id).unwrap();
        assert_eq!(loaded.name, "Prepare coding environment");
        assert_eq!(
            loaded.intent_definition.statement,
            "When ready, request opening my development environment."
        );
    }
}

/// CASE 2 — Contracts belong to correct workspace/project.
#[test]
fn case2_contracts_belong_to_workspace_project() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws_a, project_a, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace_b = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Other WS".into()))
        .unwrap();
    let project_b = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        workspace_b.id.to_string(),
        "Other".into(),
        None,
        None,
    )
    .unwrap();

    let err = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_a.clone(),
        project_b.id.to_string(),
        None,
        "Cross".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request something.".into(),
        vec![],
    );
    assert!(matches!(
        err,
        Err(KernelError::AutomationContractValidation { .. })
    ));

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_a.clone(),
        project_a,
        None,
        "Owned".into(),
        None,
        AutomationTriggerKind::Event,
        Some("workspace.focus".into()),
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();
    let listed =
        CommandHandler::list_automation_contracts(&kernel, local, intent, ws_a, Some(10)).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, contract.id);
}

/// CASE 3 — Contracts cannot execute commands.
#[test]
fn case3_contracts_cannot_execute_commands() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "No exec".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request opening my development environment.".into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let prepared = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert!(prepared.governance_note.contains("Permission Gateway"));

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
}

/// CASE 4 — Contracts cannot bypass Permission Gateway.
#[test]
fn case4_contracts_cannot_bypass_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-4").unwrap());
    assert_eq!(
        CapabilitySet::for_actor_type(ActorType::AIAssistant)
            .iter()
            .count(),
        0
    );

    let err = CommandHandler::create_automation_contract(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        ws,
        project_id,
        None,
        "AI draft".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request something.".into(),
        vec![],
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected permission boundary, got {other:?}"),
    }
}

/// CASE 5 — Approval state does not equal execution permission.
#[test]
fn case5_approval_is_not_execution_permission() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "Approved def".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::request_automation_contract_approval(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let approved = CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert_eq!(approved.approval_state, AutomationContractApprovalState::Approved);
    assert_eq!(approved.status, AutomationContractStatus::Approved);

    let grants = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        PermissionApprovalService::active_grants_for_actor(&guard, local.actor.id.as_str())
            .unwrap()
    };
    assert!(
        grants.is_empty(),
        "contract approval must not create capability grants: {grants:?}"
    );
}

/// CASE 6 — Revoked contracts cannot be treated as active.
#[test]
fn case6_revoked_contracts_are_not_active() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "Revoke me".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    CommandHandler::revoke_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let err = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    );
    assert!(matches!(
        err,
        Err(KernelError::AutomationContractValidation { .. })
    ));
}

/// CASE 7 — Workspace Intelligence can read contracts.
#[test]
fn case7_intelligence_can_read_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        project_id,
        None,
        "Visible".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let state =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(state
        .automation_contracts
        .iter()
        .any(|c| c.id == contract.id.as_str()));
    assert!(state.summary.contains("approved automation"));
    assert_eq!(state.authority_effect, "none");
}

/// CASE 8 — Workspace Intelligence cannot modify contracts.
#[test]
fn case8_intelligence_cannot_modify_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        project_id,
        None,
        "Stable".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();
    let before = CommandHandler::get_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let _ = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
    )
    .unwrap();

    let after = CommandHandler::get_automation_contract(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert_eq!(before.status, after.status);
    assert_eq!(before.approval_state, after.approval_state);
    assert_eq!(before.updated_at, after.updated_at);
}

/// CASE 9 — Assistant can explain contracts but cannot authorize them.
#[test]
fn case9_assistant_cannot_authorize_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        project_id,
        None,
        "Explainable".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();

    // Shared intelligence path surfaces the contract for explanation.
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(intel
        .automation_contracts
        .iter()
        .any(|c| c.id == contract.id.as_str() && c.approval_state == "not_approved"));

    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-9").unwrap());
    let err = CommandHandler::approve_automation_contract(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        contract.id.to_string(),
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected AI cannot authorize contracts, got {other:?}"),
    }
}

/// CASE 10 — All future execution pathways still require Command Pipeline.
#[test]
fn case10_future_pathways_require_command_pipeline() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "Boundary".into(),
        None,
        AutomationTriggerKind::Scheduled,
        Some("weekday mornings".into()),
        "Request opening my development environment.".into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    // Materialization itself is a governed query (pipeline + work_context.read).
    let prepared = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local.clone(),
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert!(!prepared.required_capabilities.is_empty());
    assert!(prepared.governance_note.contains("Command Pipeline"));

    // AI cannot even prepare the intent template without capabilities.
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-10").unwrap());
    let err = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        contract.id.to_string(),
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected governed prepare path, got {other:?}"),
    }

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "automation.contract.created"));
    assert!(records
        .iter()
        .any(|r| r.event_type == "automation.contract.approved"));
    for event in records.iter().filter(|r| r.event_type.starts_with("automation.contract")) {
        let meta = event.metadata.as_deref().unwrap_or("");
        assert!(
            meta.contains("\"authority_effect\":\"none\""),
            "expected authority_effect none in {meta}"
        );
    }
}

#[test]
fn paused_contract_cannot_materialize_intent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "Pause me".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Request prepare coding workspace.".into(),
        vec![],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    CommandHandler::pause_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let err = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    );
    assert!(err.is_err());
}
