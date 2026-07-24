//! Governed Trigger Evaluation tests (Phase 4 Batch 7).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::{AuditService, PermissionApprovalService};
use crate::WorkspaceKernel;
use workspace_database::AutomationContractRepository;
use workspace_domain::{
    ActorContext, AutomationIntentDefinition, AutomationIntentProposalStatus, AutomationTriggerKind,
    IntentContext, TaskPriority, TriggerEventType,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Trigger Eval WS".into()))
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

fn create_and_approve(
    kernel: &WorkspaceKernel,
    ws: String,
    project_id: String,
    task_id: Option<String>,
    statement: &str,
) -> workspace_domain::AutomationContract {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = CommandHandler::create_automation_contract(
        kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        task_id,
        "Prepare coding environment".into(),
        Some("User-approved future workflow".into()),
        AutomationTriggerKind::Manual,
        None,
        statement.into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap()
}

/// CASE 1 — Trigger events can be created.
#[test]
fn case1_trigger_events_can_be_created() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let event = CommandHandler::record_trigger_event(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        TriggerEventType::ManualEvaluationRequested,
        "operator_console".into(),
        r#"{"reason":"manual"}"#.into(),
        Some(project_id),
        Some(task_id),
    )
    .unwrap();

    assert_eq!(event.authority_effect, "none");
    assert_eq!(
        event.event_type,
        TriggerEventType::ManualEvaluationRequested
    );
    let listed =
        CommandHandler::list_trigger_events(&kernel, local, intent, ws, Some(10)).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, event.id);

    let audits = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(audits
        .iter()
        .any(|r| r.event_type == "automation.trigger.received"));
}

/// CASE 2 — Trigger evaluation finds matching contracts.
#[test]
fn case2_evaluation_finds_matching_contracts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, task_id) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        Some(task_id.clone()),
        "When ready, request opening my development environment.",
    );

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "operator_console".into(),
        "{}".into(),
        Some(project_id),
        Some(task_id),
    )
    .unwrap();

    assert_eq!(result.authority_effect, "none");
    assert_eq!(result.proposals.len(), 1);
    assert_eq!(result.proposals[0].contract_id, contract.id);
    assert_eq!(
        result.proposals[0].status,
        AutomationIntentProposalStatus::PendingReview
    );
    assert!(result.proposals[0].explanation.contains("matched"));
}

/// CASE 3 — Paused contracts are ignored.
#[test]
fn case3_paused_contracts_are_ignored() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );
    CommandHandler::pause_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();

    assert!(result.proposals.is_empty());
    assert!(result
        .rejections
        .iter()
        .any(|r| r.reason == "Contract paused."));
}

/// CASE 4 — Revoked contracts are ignored.
#[test]
fn case4_revoked_contracts_are_ignored() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );
    CommandHandler::revoke_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();

    assert!(result.proposals.is_empty());
    assert!(result
        .rejections
        .iter()
        .any(|r| r.reason == "Contract revoked."));
}

/// CASE 5 — Modified contracts with invalid fingerprints are rejected.
#[test]
fn case5_invalid_fingerprints_are_rejected() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );

    // Simulate approved row whose definition drifted without clearing binding.
    {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        let repo = AutomationContractRepository::new(&guard);
        let mut row = repo.get(&contract.id).unwrap().unwrap();
        row.intent_definition =
            AutomationIntentDefinition::new("Completely different intent definition.").unwrap();
        repo.upsert(&row).unwrap();
    }

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();

    assert!(result.proposals.is_empty());
    assert!(result
        .rejections
        .iter()
        .any(|r| r.reason == "Approval fingerprint no longer matches."));
}

/// CASE 6 — Scope mismatches are rejected.
#[test]
fn case6_scope_mismatches_are_rejected() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_a, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let project_b = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Other project".into(),
        None,
        None,
    )
    .unwrap();
    create_and_approve(
        &kernel,
        ws.clone(),
        project_a,
        None,
        "When ready, request opening my development environment.",
    );

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_b.id.to_string()),
        None,
    )
    .unwrap();

    assert!(result.proposals.is_empty());
    assert!(result
        .rejections
        .iter()
        .any(|r| r.reason == "Project scope mismatch."));
}

/// CASE 7 — Trigger evaluation cannot execute.
#[test]
fn case7_trigger_evaluation_cannot_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local,
        intent,
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();
    assert!(!result.proposals.is_empty());

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
    assert!(after
        .iter()
        .any(|r| r.event_type == "automation.intent_proposal.created"));
    assert!(!after
        .iter()
        .any(|r| r.event_type == "application.launched"));
}

/// CASE 8 — Trigger evaluation cannot bypass Permission Gateway.
#[test]
fn case8_evaluation_cannot_bypass_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );

    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();
    assert_eq!(result.proposals.len(), 1);

    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-trigger-7-8").unwrap());
    let err = CommandHandler::evaluate_triggers(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        result.trigger_event.id.to_string(),
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected governed evaluate path, got {other:?}"),
    }

    let grants = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        PermissionApprovalService::active_grants_for_actor(&guard, local.actor.id.as_str())
            .unwrap()
    };
    assert!(grants.is_empty());

    for event in AuditService::list_recent(&kernel.shared_database(), 100)
        .unwrap()
        .iter()
        .filter(|r| r.event_type.starts_with("automation.trigger")
            || r.event_type.starts_with("automation.intent_proposal"))
    {
        let meta = event.metadata.as_deref().unwrap_or("");
        assert!(
            meta.contains("\"authority_effect\":\"none\""),
            "expected authority_effect none in {meta}"
        );
    }
}

/// CASE 9 — Workspace Intelligence can display evaluations but cannot modify them.
#[test]
fn case9_intelligence_displays_but_cannot_modify() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );
    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();
    let proposal_id = result.proposals[0].id.to_string();

    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.pending_automation_proposals.len(), 1);
    assert_eq!(intel.pending_automation_proposals[0].id, proposal_id);

    // Generate again — still pending; intelligence must not accept/reject.
    let intel2 = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(intel2.pending_automation_proposals.len(), 1);

    let still_pending = CommandHandler::list_automation_intent_proposals(
        &kernel,
        local,
        intent,
        ws,
        Some(AutomationIntentProposalStatus::PendingReview),
        None,
    )
    .unwrap();
    assert_eq!(still_pending.len(), 1);
    assert_eq!(still_pending[0].id.to_string(), proposal_id);
}

/// CASE 10 — Accepted proposals still require normal governance flow.
#[test]
fn case10_accepted_proposals_still_require_governance() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = create_and_approve(
        &kernel,
        ws.clone(),
        project_id.clone(),
        None,
        "When ready, request opening my development environment.",
    );
    let result = CommandHandler::record_and_evaluate_triggers(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        TriggerEventType::ManualEvaluationRequested,
        "test".into(),
        "{}".into(),
        Some(project_id),
        None,
    )
    .unwrap();

    let accepted = CommandHandler::accept_automation_intent_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        result.proposals[0].id.to_string(),
    )
    .unwrap();
    assert_eq!(accepted.status, AutomationIntentProposalStatus::Accepted);

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let prepared = CommandHandler::prepare_automation_contract_intent(
        &kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    assert!(prepared.governance_note.contains("Permission Gateway"));
    assert!(prepared
        .audit_metadata
        .contains("\"execution_authorized\":false"));

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);

    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-trigger-7-10").unwrap());
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

    assert!(after
        .iter()
        .any(|r| r.event_type == "automation.intent_proposal.accepted"));
}
