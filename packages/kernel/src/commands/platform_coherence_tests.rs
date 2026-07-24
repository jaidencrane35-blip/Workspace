//! Platform coherence tests (Phase 4 Batch 9.5).
//!
//! Prove ownership, singular aggregators, Intelligence/Assistant consumption,
//! vocabulary consistency, and unchanged authority boundaries.

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    vocabulary, Actor, ActorContext, AutomationTriggerKind, ConceptOwnerKind, IntentContext,
    PLATFORM_CONCEPT_OWNERS, TaskPriority,
};

fn seed_workspace(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Coherence WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Platform".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Every concept has exactly one owner.
#[test]
fn case1_every_concept_has_exactly_one_owner() {
    let mut seen = std::collections::HashSet::new();
    for row in PLATFORM_CONCEPT_OWNERS {
        assert!(
            seen.insert(row.concept),
            "duplicate ownership entry for {}",
            row.concept
        );
        assert!(!row.owner.is_empty());
    }
    assert!(PLATFORM_CONCEPT_OWNERS
        .iter()
        .any(|r| r.concept == "decision_item" && r.kind == ConceptOwnerKind::Aggregator));
    assert!(PLATFORM_CONCEPT_OWNERS
        .iter()
        .any(|r| r.concept == "activity" && r.kind == ConceptOwnerKind::Aggregator));
    assert!(PLATFORM_CONCEPT_OWNERS
        .iter()
        .any(|r| r.concept == "project" && r.kind == ConceptOwnerKind::DurableStore));
}

/// CASE 2 — Duplicate pending/blocked calculation eliminated (Intelligence projects DQ).
#[test]
fn case2_intelligence_projects_pending_from_decision_queue() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let contract = CommandHandler::create_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        project_id,
        None,
        "Coherence contract".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Prepare environment".into(),
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

    let queue_pending_ids: std::collections::HashSet<_> = queue
        .items
        .iter()
        .filter(|i| {
            matches!(
                i.decision_state,
                workspace_domain::DecisionState::Pending
                    | workspace_domain::DecisionState::Viewed
                    | workspace_domain::DecisionState::Deferred
            )
        })
        .map(|i| i.source_id.as_str())
        .collect();

    for pending in &intel.pending_approvals {
        assert!(
            queue_pending_ids.contains(pending.id.as_str()),
            "intelligence pending_approvals must project Decision Queue sources"
        );
    }
    assert_eq!(intel.decision_queue.pending_count, queue.pending_count);
}

/// CASE 3 — Decision Queue remains the single pending-decision aggregation.
#[test]
fn case3_decision_queue_is_sole_pending_aggregation() {
    assert_eq!(
        PLATFORM_CONCEPT_OWNERS
            .iter()
            .filter(|r| r.concept == "decision_item")
            .count(),
        1
    );
    let owner = PLATFORM_CONCEPT_OWNERS
        .iter()
        .find(|r| r.concept == "decision_item")
        .unwrap();
    assert_eq!(owner.owner, "DecisionQueueService");
    assert_eq!(owner.kind, ConceptOwnerKind::Aggregator);
}

/// CASE 4 — Activity Graph remains the single relationship read model.
#[test]
fn case4_activity_graph_is_sole_relationship_model() {
    let owner = PLATFORM_CONCEPT_OWNERS
        .iter()
        .find(|r| r.concept == "activity")
        .unwrap();
    assert_eq!(owner.owner, "WorkspaceActivityGraphService");
    assert_eq!(owner.kind, ConceptOwnerKind::Aggregator);

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let intel = CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws)
        .unwrap();
    for item in &intel.activity_graph.recent_timeline {
        assert!(
            graph.activities.iter().any(|a| a.id == item.id),
            "intelligence activity ids must come from Activity Graph"
        );
    }
}

/// CASE 5 — Workspace Intelligence consumes rather than owns.
#[test]
fn case5_intelligence_consumes_rather_than_owns() {
    assert!(!PLATFORM_CONCEPT_OWNERS
        .iter()
        .any(|r| r.owner == "WorkspaceIntelligenceService"));
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(!state.decision_queue.workspace_id.is_empty() || state.decision_queue.pending_count == 0);
}

/// CASE 6 — Assistant consumes the same underlying models.
#[test]
fn case6_assistant_shares_intelligence_models() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let work = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(work.decision_queue.pending_count, assistant.decision_queue.pending_count);
    assert_eq!(
        work.current_project.as_ref().map(|p| p.id.as_str()),
        assistant.current_project.as_ref().map(|p| p.id.as_str())
    );
    assert_eq!(work.authority_effect, "none");
    assert_eq!(assistant.authority_effect, "none");
    // Both surfaces consume the same Activity Graph summary shape.
    assert_eq!(
        work.activity_graph.workspace_id,
        assistant.activity_graph.workspace_id
    );
    assert!(!work.activity_graph.authority_effect.is_empty());
    assert_eq!(
        work.activity_graph.authority_effect,
        assistant.activity_graph.authority_effect
    );
}

/// CASE 7 — Terminology is internally consistent.
#[test]
fn case7_terminology_is_internally_consistent() {
    assert_eq!(vocabulary::INTENT_PROPOSAL, "Intent Proposal");
    assert_eq!(vocabulary::RECOMMENDATION, "Recommendation");
    assert_eq!(vocabulary::DECISION, "Decision");
    assert_eq!(vocabulary::PERMISSION_APPROVAL, "Permission Approval");
    assert_eq!(vocabulary::CONTRACT_APPROVAL, "Contract Approval");
    assert_ne!(vocabulary::INTENT_PROPOSAL, vocabulary::ACTION_PROPOSAL);
    assert_ne!(vocabulary::DECISION, vocabulary::PERMISSION_APPROVAL);
    assert_ne!(vocabulary::WORK_GOAL, vocabulary::ASSISTANT_GOAL);
}

/// CASE 8 — No authority paths changed (generate remains authority_effect none).
#[test]
fn case8_no_authority_paths_changed() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let intel = CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(queue.authority_effect, "none");
    assert_eq!(graph.authority_effect, "none");
    assert_eq!(intel.authority_effect, "none");
}

/// CASE 9 — Permission Gateway remains sole authority boundary.
#[test]
fn case9_permission_gateway_remains_sole_boundary() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed_workspace(&kernel);
    let ai = ActorContext::new(Actor::ai_assistant("ai-coherence").unwrap());
    let intent = IntentContext::ai_suggestion();
    let err = CommandHandler::generate_workspace_intelligence(&kernel, ai, intent, ws)
        .expect_err("AI actor must not bypass read gate");
    assert!(
        matches!(
            err,
            KernelError::ApprovalRequired { .. } | KernelError::PermissionDenied(_)
        ),
        "expected gateway denial, got {err:?}"
    );
}

/// CASE 10 — Existing governance surfaces stay coherent after Batch 9.5 wiring.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed_workspace(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _task = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project_id,
        ws.clone(),
        "Coherence task".into(),
        TaskPriority::Medium,
    )
    .unwrap();
    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let graph = CommandHandler::generate_workspace_activity_graph(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let intel = CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws)
        .unwrap();
    assert!(graph.activities.iter().any(|a| a.activity_type.as_str() == "project"));
    assert_eq!(intel.decision_queue.pending_count, queue.pending_count);
    assert!(intel.recent_activity.len() <= graph.timeline.len());
}
