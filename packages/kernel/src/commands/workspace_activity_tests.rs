//! Workspace Activity Graph tests (Phase 4 Batch 9).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ActivityType, AutomationIntentProposalStatus, AutomationTriggerKind,
    IntentContext, TaskPriority, TriggerEventType,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Activity Graph WS".into()))
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

fn seed_proposal(
    kernel: &WorkspaceKernel,
    ws: String,
    project_id: String,
) -> (workspace_domain::AutomationContract, workspace_domain::AutomationIntentProposal) {
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
    let proposal = result.proposals.into_iter().next().expect("proposal");
    (contract, proposal)
}

/// CASE 1 — Activities accurately represent existing sources.
#[test]
fn case1_activities_represent_sources() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let (contract, proposal) = seed_proposal(&kernel, ws.clone(), project_id.clone());
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    assert!(graph
        .activities
        .iter()
        .any(|a| a.activity_type == ActivityType::Project && a.source_id == project_id));
    assert!(graph.activities.iter().any(|a| {
        a.activity_type == ActivityType::AutomationContract && a.source_id == contract.id.as_str()
    }));
    assert!(graph.activities.iter().any(|a| {
        a.activity_type == ActivityType::IntentProposal && a.source_id == proposal.id.as_str()
    }));
    assert_eq!(graph.authority_effect, "none");
}

/// CASE 2 — Relationships remain correct.
#[test]
fn case2_relationships_correct() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let (contract, proposal) = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    let proposal_activity = graph
        .activities
        .iter()
        .find(|a| a.source_id == proposal.id.as_str())
        .unwrap();
    let contract_activity_id = format!("activity:automation_contract:{}", contract.id);
    assert_eq!(
        proposal_activity.parent_activity_id.as_deref(),
        Some(contract_activity_id.as_str())
    );
    assert!(proposal_activity
        .related_activity_ids
        .iter()
        .any(|id| id == &contract_activity_id));
    assert!(proposal_activity
        .related_activity_ids
        .iter()
        .any(|id| id.starts_with("activity:trigger_event:")));
}

/// CASE 3 — Graph contains no duplicate ownership (no activity payload table).
#[test]
fn case3_no_duplicate_ownership() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    // Synthetic ids only — sources remain authoritative.
    assert!(graph
        .activities
        .iter()
        .all(|a| a.id.as_str().starts_with("activity:")));
    let db = kernel.shared_database();
    let guard = db.lock().unwrap();
    let count: i64 = guard
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='workspace_activities'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}

/// CASE 4 — Deleting source objects updates relationships.
#[test]
fn case4_removing_source_updates_graph() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let (_, proposal) = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(before
        .activities
        .iter()
        .any(|a| a.source_id == proposal.id.as_str()));

    CommandHandler::reject_automation_intent_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        proposal.id.to_string(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(!after.activities.iter().any(|a| {
        a.activity_type == ActivityType::IntentProposal
            && a.source_id == proposal.id.as_str()
            && a.unresolved
    }));
    // Rejected proposals may still appear as historical nodes if list_proposals returns them.
    // Unresolved pending decision for that proposal must be gone.
    assert!(!after.activities.iter().any(|a| {
        a.activity_type == ActivityType::DecisionItem
            && a.related_activity_ids
                .iter()
                .any(|r| r.contains(proposal.id.as_str()))
            && a.unresolved
    }));
}

/// CASE 5 — Timeline ordering is deterministic.
#[test]
fn case5_timeline_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let first = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    let ids1: Vec<_> = first.timeline.iter().map(|a| a.id.to_string()).collect();
    let ids2: Vec<_> = second.timeline.iter().map(|a| a.id.to_string()).collect();
    assert_eq!(ids1, ids2);
    for window in first.timeline.windows(2) {
        assert!(window[0].timestamp <= window[1].timestamp);
    }
}

/// CASE 6 — Workspace Intelligence consumes the graph correctly.
#[test]
fn case6_intelligence_consumes_graph() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let graph = CommandHandler::generate_workspace_activity_graph(
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
    assert_eq!(intel.activity_graph.authority_effect, "none");
    let graph_ids: std::collections::HashSet<_> =
        graph.activities.iter().map(|a| a.id.to_string()).collect();
    for item in &intel.activity_graph.recent_timeline {
        assert!(graph_ids.contains(item.id.as_str()));
    }
}

/// CASE 7 — Assistant consumes the same graph (via intelligence field).
#[test]
fn case7_assistant_shares_intelligence_graph() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(intel.activity_graph.activity_count > 0);
    assert!(!intel.activity_graph.recent_timeline.is_empty());
}

/// CASE 8 — Graph survives restart.
#[test]
fn case8_graph_survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("activity-graph.db");
    let workspace_id;
    let contract_id;
    let proposal_id;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let (ws, project_id, _) = seed_project(&kernel);
        workspace_id = ws.clone();
        let (contract, proposal) = seed_proposal(&kernel, ws.clone(), project_id);
        contract_id = contract.id.to_string();
        proposal_id = proposal.id.to_string();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let graph = CommandHandler::generate_workspace_activity_graph(
            &kernel,
            local,
            intent,
            ws,
        )
        .unwrap();
        assert!(graph.activities.iter().any(|a| a.source_id == contract_id));
        assert!(graph.activities.iter().any(|a| a.source_id == proposal_id));
    }
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let graph = CommandHandler::generate_workspace_activity_graph(
            &kernel,
            local,
            intent,
            workspace_id,
        )
        .unwrap();
        assert!(graph.activities.iter().any(|a| a.source_id == contract_id));
        assert!(graph.activities.iter().any(|a| a.source_id == proposal_id));
    }
}

/// CASE 9 — Graph cannot execute actions.
#[test]
fn case9_graph_cannot_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let _ = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();

    let _ = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
    assert!(after
        .iter()
        .any(|r| r.event_type == "workspace.timeline.generated"));
}

/// CASE 10 — Graph cannot bypass Permission Gateway.
#[test]
fn case10_graph_cannot_bypass_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let ai = ActorContext::new(Actor::ai_assistant("ai-activity-graph").unwrap());
    let err = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        ws,
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected governed read path, got {other:?}"),
    }
}

#[test]
fn rejected_proposal_no_longer_pending_review_in_list() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let (_, proposal) = seed_proposal(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::reject_automation_intent_proposal(
        &kernel,
        local.clone(),
        intent.clone(),
        proposal.id.to_string(),
    )
    .unwrap();
    let pending = CommandHandler::list_automation_intent_proposals(
        &kernel,
        local,
        intent,
        ws,
        Some(AutomationIntentProposalStatus::PendingReview),
        None,
    )
    .unwrap();
    assert!(pending.is_empty());
}
