//! Governed Automation Contract integrity tests (Phase 4 Batch 6.5).

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
        .execute_mutation(CreateWorkspace::new("Automation Integrity WS".into()))
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

fn create_draft(
    kernel: &WorkspaceKernel,
    ws: String,
    project_id: String,
    statement: &str,
) -> workspace_domain::AutomationContract {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_automation_contract(
        kernel,
        local,
        intent,
        ws,
        project_id,
        None,
        "Prepare coding environment".into(),
        Some("User-approved future workflow".into()),
        AutomationTriggerKind::Manual,
        None,
        statement.into(),
        vec!["application.launch".into()],
    )
    .unwrap()
}

/// CASE 1 — Approved contracts cannot execute directly.
#[test]
fn case1_approved_contracts_cannot_execute_directly() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(
        &kernel,
        ws,
        project_id,
        "When ready, request opening my development environment.",
    );
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let prepared = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert!(prepared.governance_note.contains("Permission Gateway"));
    assert!(prepared.audit_metadata.contains("\"execution_authorized\":false"));

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
}

/// CASE 2 — Modified contracts invalidate stale approval when required.
#[test]
fn case2_material_edits_invalidate_stale_approval() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(
        &kernel,
        ws,
        project_id,
        "When ready, request opening my development environment.",
    );
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let updated = CommandHandler::update_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
        None,
        None,
        Some("Request a completely different workspace setup.".into()),
        None,
        None,
        None,
    )
    .unwrap();
    assert_eq!(updated.status, AutomationContractStatus::Draft);
    assert_eq!(
        updated.approval_state,
        AutomationContractApprovalState::NotApproved
    );
    assert!(updated.approved_definition_fingerprint.is_none());

    let err = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    );
    assert!(err.is_err());

    // Cosmetic rename must not invalidate a fresh approval.
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let renamed = CommandHandler::update_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
        Some("Renamed contract".into()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert_eq!(renamed.status, AutomationContractStatus::Approved);
    assert_eq!(
        renamed.approval_state,
        AutomationContractApprovalState::Approved
    );
    assert!(renamed.is_active_definition());

    // Pending consent is also invalidated by material edits.
    let other = create_draft(
        &kernel,
        renamed.workspace_id.to_string(),
        renamed.project_id.to_string(),
        "Pending definition.",
    );
    CommandHandler::request_automation_contract_approval(
        &kernel,
        local.clone(),
        intent.clone(),
        other.id.to_string(),
    )
    .unwrap();
    let cleared = CommandHandler::update_automation_contract(
        &kernel,
        local,
        intent,
        other.id.to_string(),
        None,
        None,
        None,
        None,
        None,
        Some(vec!["application.launch".into(), "workspace.write".into()]),
    )
    .unwrap();
    assert_eq!(
        cleared.approval_state,
        AutomationContractApprovalState::NotApproved
    );
}

/// CASE 3 — Revoked contracts cannot create active intents.
#[test]
fn case3_revoked_contracts_cannot_create_active_intents() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(&kernel, ws, project_id, "Request prepare coding workspace.");
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

/// CASE 4 — Paused contracts cannot be treated as active.
#[test]
fn case4_paused_contracts_are_not_active() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(&kernel, ws, project_id, "Request prepare coding workspace.");
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
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    );
    assert!(err.is_err());

    let resumed = CommandHandler::resume_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    assert!(resumed.is_active_definition());
    CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
}

/// CASE 5 — Contracts cannot bypass Permission Gateway.
#[test]
fn case5_contracts_cannot_bypass_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-6.5-5").unwrap());
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

/// CASE 6 — Workspace Intelligence cannot mutate contracts.
#[test]
fn case6_intelligence_cannot_mutate_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(&kernel, ws.clone(), project_id, "Request prepare coding workspace.");
    let before = CommandHandler::get_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let state =
        CommandHandler::generate_workspace_intelligence(&kernel, local.clone(), intent.clone(), ws)
            .unwrap();
    assert!(state
        .automation_contracts
        .iter()
        .any(|c| c.id == contract.id.as_str()));
    assert_eq!(state.authority_effect, "none");
    let after = CommandHandler::get_automation_contract(
        &kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert_eq!(before.updated_at, after.updated_at);
    assert_eq!(before.approval_state, after.approval_state);
}

/// CASE 7 — Assistant cannot authorize contracts.
#[test]
fn case7_assistant_cannot_authorize_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(&kernel, ws.clone(), project_id, "Request prepare coding workspace.");
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(intel
        .automation_contracts
        .iter()
        .any(|c| c.id == contract.id.as_str()));

    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-6.5-7").unwrap());
    let err = CommandHandler::approve_automation_contract(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        contract.id.to_string(),
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected AI cannot authorize, got {other:?}"),
    }
}

/// CASE 8 — Contract state survives restart.
#[test]
fn case8_contract_state_survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("automation-integrity.db");
    let contract_id;
    let fingerprint;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let (ws, project_id, _) = seed_project(&kernel);
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let contract = create_draft(
            &kernel,
            ws,
            project_id,
            "When ready, request opening my development environment.",
        );
        let approved = CommandHandler::approve_automation_contract(
            &kernel,
            local,
            intent,
            contract.id.to_string(),
        )
        .unwrap();
        contract_id = approved.id.to_string();
        fingerprint = approved.approved_definition_fingerprint.clone().unwrap();
        assert_eq!(
            approved.approved_by_actor.as_deref(),
            Some(ActorContext::local_user().actor.id.as_str())
        );
    }
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let loaded = CommandHandler::get_automation_contract(
            &kernel,
            local.clone(),
            intent.clone(),
            contract_id.clone(),
        )
        .unwrap();
        assert_eq!(loaded.status, AutomationContractStatus::Approved);
        assert_eq!(
            loaded.approval_state,
            AutomationContractApprovalState::Approved
        );
        assert_eq!(
            loaded.approved_definition_fingerprint.as_deref(),
            Some(fingerprint.as_str())
        );
        CommandHandler::prepare_automation_contract_intent(
            &kernel,
            local,
            intent,
            contract_id,
        )
        .unwrap();
    }
}

/// CASE 9 — Workspace/project isolation is enforced.
#[test]
fn case9_workspace_project_isolation() {
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
    let contract = create_draft(&kernel, ws_a.clone(), project_a, "Owned intent.");
    let listed_a =
        CommandHandler::list_automation_contracts(&kernel, local.clone(), intent.clone(), ws_a, None)
            .unwrap();
    let listed_b = CommandHandler::list_automation_contracts(
        &kernel,
        local,
        intent,
        workspace_b.id.to_string(),
        None,
    )
    .unwrap();
    assert_eq!(listed_a.len(), 1);
    assert_eq!(listed_a[0].id, contract.id);
    assert!(listed_b.is_empty());
}

/// CASE 10 — All execution pathways still enter Command Pipeline.
#[test]
fn case10_execution_pathways_require_command_pipeline() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_draft(&kernel, ws, project_id, "Request opening my development environment.");
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let prepared = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local.clone(),
        intent,
        contract.id.to_string(),
    )
    .unwrap();
    assert!(!prepared.requesting_actor_id.is_empty());
    assert!(!prepared.definition_fingerprint.is_empty());
    assert!(prepared.audit_metadata.contains("authority_effect"));

    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-contract-6.5-10").unwrap());
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

    let records = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(records
        .iter()
        .any(|r| r.event_type == "automation.contract.intent.prepared"));
    for event in records
        .iter()
        .filter(|r| r.event_type.starts_with("automation.contract"))
    {
        let meta = event.metadata.as_deref().unwrap_or("");
        assert!(
            meta.contains("\"authority_effect\":\"none\""),
            "expected authority_effect none in {meta}"
        );
    }

    let grants = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        PermissionApprovalService::active_grants_for_actor(&guard, local.actor.id.as_str())
            .unwrap()
    };
    assert!(grants.is_empty());
}
