//! Governed Decision Queue tests (Phase 4 Batch 8).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::{AuditService, PermissionApprovalService};
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ApprovalDecisionKind, AutomationIntentProposalStatus, AutomationTriggerKind,
    Capability, DecisionSourceType, DecisionState, IntentContext, ResourceKind, TaskPriority,
    TriggerEventType,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Decision Queue WS".into()))
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

fn seed_pending_approval(kernel: &WorkspaceKernel, workspace_id: &str) -> String {
    let ai = Actor::ai_assistant("ai-decision-queue").unwrap();
    let request = PermissionRequest {
        actor: ai,
        intent: workspace_domain::Intent::ai_suggestion(),
        command: "LaunchApplication",
        capability: Capability::application_launch(),
        subject: PermissionSubject::Resource(ResourceKind::Application),
        target_resource_id: Some(format!("app-for-{workspace_id}")),
    };
    let reason = format!("Needs launch approval in workspace {workspace_id}");
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    PermissionApprovalService::ensure_pending_request(&guard, &request, &reason)
        .unwrap()
        .id
        .to_string()
}

fn seed_pending_proposal(
    kernel: &WorkspaceKernel,
    ws: String,
    project_id: String,
) -> workspace_domain::AutomationIntentProposal {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = CommandHandler::create_automation_contract(
        kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        project_id.clone(),
        None,
        "Prepare coding environment".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "When ready, request opening my development environment.".into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::approve_automation_contract(
        kernel,
        local.clone(),
        intent.clone(),
        contract.id.to_string(),
    )
    .unwrap();
    let result = CommandHandler::record_and_evaluate_triggers(
        kernel,
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
    result.proposals.into_iter().next().expect("proposal")
}

/// CASE 1 — Every pending approval appears once.
#[test]
fn case1_pending_approval_appears_once() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let approval_id = seed_pending_approval(&kernel, &ws);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let matches: Vec<_> = queue
        .items
        .iter()
        .filter(|i| {
            i.source_type == DecisionSourceType::PendingApproval && i.source_id == approval_id
        })
        .collect();
    assert_eq!(matches.len(), 1);

    let again = CommandHandler::generate_decision_queue(&kernel, local, intent, ws).unwrap();
    let again_matches: Vec<_> = again
        .items
        .iter()
        .filter(|i| {
            i.source_type == DecisionSourceType::PendingApproval && i.source_id == approval_id
        })
        .collect();
    assert_eq!(again_matches.len(), 1);
}

/// CASE 2 — Every trigger proposal appears once.
#[test]
fn case2_trigger_proposal_appears_once() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue =
        CommandHandler::generate_decision_queue(&kernel, local, intent, ws).unwrap();
    let matches: Vec<_> = queue
        .items
        .iter()
        .filter(|i| {
            i.source_type == DecisionSourceType::IntentProposal
                && i.source_id == proposal.id.as_str()
        })
        .collect();
    assert_eq!(matches.len(), 1);
}

/// CASE 3 — Blocked actions appear correctly.
#[test]
fn case3_blocked_actions_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let app = CommandHandler::create_application(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Editor".into(),
        None,
        Some("editor.exe".into()),
    )
    .unwrap();

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "ai-decision-blocked",
        "Prepare my coding workspace",
        vec![app.id.to_string()],
        Some(ws.clone()),
    )
    .unwrap();
    let confirmed =
        CommandHandler::confirm_assistant_workflow_simulated(&kernel, workflow.id.to_string())
            .unwrap();
    let plan = CommandHandler::get_orchestrated_ai_plan(
        &kernel,
        confirmed.orchestrated_plan_id.as_ref().unwrap().to_string(),
    )
    .unwrap();
    let approval_id = plan.steps[0]
        .approval_request_id
        .clone()
        .expect("approval");
    CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(DecideApproval::new(
            workspace_domain::PermissionApprovalRequestId::new(approval_id).unwrap(),
            ApprovalDecisionKind::Deny,
        ))
        .unwrap();
    let _ = CommandHandler::resume_assistant_workflow_simulated(&kernel, workflow.id.to_string())
        .unwrap();

    let queue =
        CommandHandler::generate_decision_queue(&kernel, local, intent, ws).unwrap();
    assert!(
        queue
            .items
            .iter()
            .any(|i| i.source_type == DecisionSourceType::BlockedAction),
        "expected blocked action in {:?}",
        queue.items
    );
}

/// CASE 4 — Planning continuations appear correctly.
#[test]
fn case4_planning_continuations_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let app = CommandHandler::create_application(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Editor".into(),
        None,
        Some("editor.exe".into()),
    )
    .unwrap();
    let _ = CommandHandler::submit_assistant_goal(
        &kernel,
        "ai-decision-plan",
        "Prepare my coding workspace",
        vec![app.id.to_string()],
        Some(ws.clone()),
    )
    .unwrap();

    let queue =
        CommandHandler::generate_decision_queue(&kernel, local, intent, ws).unwrap();
    assert!(
        queue
            .items
            .iter()
            .any(|i| i.source_type == DecisionSourceType::PlanningContinuation),
        "expected planning continuation in {:?}",
        queue.items
    );
}

/// CASE 5 — Decision Queue cannot execute actions.
#[test]
fn case5_decision_queue_cannot_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let proposal_item = queue
        .items
        .iter()
        .find(|i| i.source_type == DecisionSourceType::IntentProposal)
        .unwrap();
    let _ = CommandHandler::accept_decision_item(
        &kernel,
        local,
        intent,
        ws,
        proposal_item.id.to_string(),
    )
    .unwrap();

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
    assert!(!after
        .iter()
        .any(|r| r.event_type == "application.launched"));
}

/// CASE 6 — Decision Queue cannot approve permissions.
#[test]
fn case6_decision_queue_cannot_approve_permissions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let approval_id = seed_pending_approval(&kernel, &ws);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item = queue
        .items
        .iter()
        .find(|i| i.source_id == approval_id)
        .unwrap();
    let result = CommandHandler::accept_decision_item(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
        item.id.to_string(),
    )
    .unwrap();
    assert!(!result.delegated);
    assert!(result.handoff.is_some());
    assert_eq!(
        result.handoff.as_ref().unwrap().next_command,
        "decide_approval"
    );

    let approvals = CommandHandler::get_permission_approvals(&kernel, local, intent, Some(50))
        .unwrap();
    let still_pending = approvals
        .iter()
        .find(|a| a.id.as_str() == approval_id)
        .unwrap();
    assert_eq!(
        still_pending.status,
        workspace_domain::PermissionApprovalStatus::Pending
    );

    let grants = {
        let db = kernel.shared_database();
        let guard = db.lock().unwrap();
        PermissionApprovalService::active_grants_for_actor(&guard, "ai-decision-queue").unwrap()
    };
    assert!(grants.is_empty());
}

/// CASE 7 — Removing a source removes the DecisionItem.
#[test]
fn case7_removing_source_removes_decision_item() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(queue
        .items
        .iter()
        .any(|i| i.source_id == proposal.id.as_str()));

    CommandHandler::reject_automation_intent_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        proposal.id.to_string(),
    )
    .unwrap();

    let after =
        CommandHandler::generate_decision_queue(&kernel, local, intent, ws).unwrap();
    assert!(!after
        .items
        .iter()
        .any(|i| i.source_id == proposal.id.as_str()));
}

/// CASE 8 — Accepting a DecisionItem delegates to the source subsystem.
#[test]
fn case8_accept_delegates_to_source() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item = queue
        .items
        .iter()
        .find(|i| i.source_id == proposal.id.as_str())
        .unwrap();
    let result = CommandHandler::accept_decision_item(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        item.id.to_string(),
    )
    .unwrap();
    assert!(result.delegated);
    assert_eq!(
        result.item.as_ref().unwrap().decision_state,
        DecisionState::Accepted
    );

    let listed = CommandHandler::list_automation_intent_proposals(
        &kernel,
        local,
        intent,
        ws,
        Some(AutomationIntentProposalStatus::Accepted),
        None,
    )
    .unwrap();
    assert!(listed.iter().any(|p| p.id == proposal.id));
}

/// CASE 9 — Assistant and Work tab consume the same Decision Queue.
#[test]
fn case9_intelligence_and_queue_share_items() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let _ = seed_pending_approval(&kernel, &ws);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    assert_eq!(intel.decision_queue.authority_effect, "none");
    let queue_ids: std::collections::HashSet<_> =
        queue.items.iter().map(|i| i.id.to_string()).collect();
    for item in &intel.decision_queue.items {
        assert!(
            queue_ids.contains(item.id.as_str()),
            "intelligence item missing from direct queue: {}",
            item.id
        );
    }
    assert_eq!(intel.decision_queue.pending_count, queue.pending_count);
}

/// CASE 10 — Decision Queue survives restart without becoming a second source of truth.
#[test]
fn case10_restart_keeps_sources_authoritative() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("decision-queue.db");
    let workspace_id;
    let proposal_id;
    let decision_item_id;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let (ws, project_id, _) = seed_project(&kernel);
        workspace_id = ws.clone();
        let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
        proposal_id = proposal.id.to_string();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let queue = CommandHandler::generate_decision_queue(
            &kernel,
            local.clone(),
            intent.clone(),
            ws.clone(),
        )
        .unwrap();
        let item = queue
            .items
            .iter()
            .find(|i| i.source_id == proposal_id)
            .unwrap();
        decision_item_id = item.id.to_string();
        CommandHandler::mark_decision_item_viewed(
            &kernel,
            local,
            intent,
            ws,
            decision_item_id.clone(),
        )
        .unwrap();
    }
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let queue = CommandHandler::generate_decision_queue(
            &kernel,
            local.clone(),
            intent.clone(),
            workspace_id.clone(),
        )
        .unwrap();
        let item = queue
            .items
            .iter()
            .find(|i| i.id.as_str() == decision_item_id)
            .expect("durable proposal should reappear from source");
        assert_eq!(item.decision_state, DecisionState::Viewed);
        assert_eq!(item.source_id, proposal_id);

        // Overlay table has no payload columns — source statement still comes from proposal.
        assert!(!item.summary.is_empty());
        let proposals = CommandHandler::list_automation_intent_proposals(
            &kernel,
            local,
            intent,
            workspace_id,
            Some(AutomationIntentProposalStatus::PendingReview),
            None,
        )
        .unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].id.as_str(), proposal_id);
    }
}

#[test]
fn dismiss_does_not_mutate_source_proposal() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item = queue
        .items
        .iter()
        .find(|i| i.source_id == proposal.id.as_str())
        .unwrap();
    CommandHandler::dismiss_decision_item(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        item.id.to_string(),
    )
    .unwrap();
    let proposals = CommandHandler::list_automation_intent_proposals(
        &kernel,
        local,
        intent,
        ws,
        Some(AutomationIntentProposalStatus::PendingReview),
        None,
    )
    .unwrap();
    assert_eq!(proposals.len(), 1);
}

#[test]
fn ai_actor_cannot_write_decision_queue() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let proposal = seed_pending_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local,
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let item_id = queue
        .items
        .iter()
        .find(|i| i.source_id == proposal.id.as_str())
        .unwrap()
        .id
        .to_string();

    let ai = ActorContext::new(Actor::ai_assistant("ai-decision-write").unwrap());
    let err = CommandHandler::mark_decision_item_viewed(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        ws,
        item_id,
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected governed write path, got {other:?}"),
    }
}
