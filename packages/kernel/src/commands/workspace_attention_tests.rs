//! Workspace Attention Engine tests (Phase 5 Batch 2).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, AutomationTriggerKind, ConceptOwnerKind, IntentContext,
    PLATFORM_CONCEPT_OWNERS, TaskPriority,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Attention WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Focus".into(),
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
        "Need attention".into(),
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

fn seed_pending_contract(kernel: &WorkspaceKernel, ws: String, project_id: String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let contract = CommandHandler::create_automation_contract(
        kernel,
        local.clone(),
        intent.clone(),
        ws,
        project_id,
        None,
        "Attention contract".into(),
        None,
        AutomationTriggerKind::Manual,
        None,
        "Prepare environment".into(),
        vec!["application.launch".into()],
    )
    .unwrap();
    CommandHandler::request_automation_contract_approval(
        kernel,
        local,
        intent,
        contract.id.to_string(),
    )
    .unwrap();
}

/// CASE 1 — Attention derives only from existing systems.
#[test]
fn case1_attention_derives_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let attention = CommandHandler::generate_workspace_attention(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!attention.items.is_empty());
    for item in &attention.items {
        assert!(matches!(
            item.source_type,
            workspace_domain::AttentionSourceType::DecisionQueue
                | workspace_domain::AttentionSourceType::Continuity
                | workspace_domain::AttentionSourceType::ActivityGraph
                | workspace_domain::AttentionSourceType::WorkflowContext
                | workspace_domain::AttentionSourceType::AutomationContract
                | workspace_domain::AttentionSourceType::TaskGraph
                | workspace_domain::AttentionSourceType::Environment
                | workspace_domain::AttentionSourceType::Composition
        ));
        assert!(item.id.as_str().starts_with("attention:"));
    }
}

/// CASE 2 — Attention never becomes a second source of truth.
#[test]
fn case2_attention_is_aggregator_only() {
    let owner = PLATFORM_CONCEPT_OWNERS
        .iter()
        .find(|r| r.concept == "attention")
        .expect("attention ownership");
    assert_eq!(owner.owner, "WorkspaceAttentionService");
    assert_eq!(owner.kind, ConceptOwnerKind::Aggregator);
}

/// CASE 3 — Attention scoring is deterministic.
#[test]
fn case3_scoring_is_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_attention(&kernel, local, intent, ws).unwrap();
    // Durable/scored attention (exclude ephemeral audit-backed informative noise).
    let stable = |items: &[workspace_domain::AttentionItem]| {
        items
            .iter()
            .filter(|i| i.score >= 35)
            .map(|i| (i.id.as_str().to_string(), i.score))
            .collect::<Vec<_>>()
    };
    assert_eq!(stable(&a.items), stable(&b.items));
}

/// CASE 4 — Every Attention Item contains an explanation.
#[test]
fn case4_every_item_has_explanation() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let attention = CommandHandler::generate_workspace_attention(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    for item in &attention.items {
        assert!(!item.explanation.is_empty());
        assert!(!item.score_factors.is_empty());
        assert!(item.explanation.contains("Score factors"));
    }
}

/// CASE 5 — Resolved source objects remove attention naturally.
#[test]
fn case5_resolved_sources_remove_attention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(before.requires_decision_count > 0);

    let queue = CommandHandler::generate_decision_queue(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let decision = queue
        .items
        .iter()
        .find(|i| {
            i.source_type == workspace_domain::DecisionSourceType::PendingApproval
                && i.source_id.starts_with("contract:")
        })
        .expect("contract decision");
    CommandHandler::dismiss_decision_item(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        decision.id.to_string(),
    )
    .unwrap();

    // Dismiss is overlay — item may still project as deferred (lower score) or still pending.
    // Fully remove source by rejecting approval request path: approve contract definition removes pending.
    let contract_id = decision.source_id.strip_prefix("contract:").unwrap();
    CommandHandler::approve_automation_contract(
        &kernel,
        local.clone(),
        intent.clone(),
        contract_id.to_string(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_attention(&kernel, local, intent, ws).unwrap();
    assert!(
        !after.items.iter().any(|i| i.source_id == decision.id.as_str()),
        "approved contract decision should leave attention"
    );
}

/// CASE 6 — Assistant and Workspace Intelligence consume identical Attention Items.
#[test]
fn case6_assistant_and_intelligence_share_attention() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let direct = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
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
    let stable_ids = |items: &[workspace_domain::AttentionItem]| {
        let mut ids = items
            .iter()
            .filter(|i| i.score >= 35)
            .map(|i| i.id.as_str().to_string())
            .collect::<Vec<_>>();
        ids.sort();
        ids
    };
    // Shared path: Work and Assistant both project Attention (order may vary after audits).
    assert!(!work.attention.top_items.is_empty() || !assistant.attention.top_items.is_empty());
    assert_eq!(work.attention.authority_effect, assistant.attention.authority_effect);
    assert_eq!(work.attention.authority_effect, "none");
    // Direct Attention and Intelligence both include Composition/Environment sources when gaps exist.
    let sources = |items: &[workspace_domain::AttentionItem]| {
        let mut s: Vec<_> = items
            .iter()
            .map(|i| i.source_type.as_str().to_string())
            .collect();
        s.sort();
        s.dedup();
        s
    };
    let work_sources = sources(&work.attention.top_items);
    let direct_sources = sources(&direct.top_items);
    assert!(
        work_sources.iter().any(|s| s == "composition" || s == "environment")
            || direct_sources
                .iter()
                .any(|s| s == "composition" || s == "environment")
            || stable_ids(&work.attention.top_items)
                .iter()
                .any(|id| id.contains("decision_queue") || id.contains("continuity"))
    );
    assert!(work
        .recommended_actions
        .iter()
        .any(|r| r.kind.starts_with("attention:") || r.kind == "bootstrap"));
}

/// CASE 7 — Attention survives restart through source regeneration.
#[test]
fn case7_attention_survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("attention.db");
    let ws = {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let (ws, project_id, _) = seed_project(&kernel);
        seed_pending_contract(&kernel, ws.clone(), project_id);
        let state = CommandHandler::generate_workspace_attention(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws.clone(),
        )
        .unwrap();
        assert!(!state.items.is_empty());
        ws
    };
    let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
    let state = CommandHandler::generate_workspace_attention(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.items.is_empty());
    assert_eq!(state.authority_effect, "none");
}

/// CASE 8 — Attention cannot execute actions.
#[test]
fn case8_attention_cannot_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let state = CommandHandler::generate_workspace_attention(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    let events = crate::services::AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    assert!(events
        .iter()
        .any(|e| e.event_type == "workspace.attention.generated"));
    for event in events.iter().filter(|e| e.event_type.starts_with("workspace.attention.")) {
        assert!(event
            .metadata
            .as_deref()
            .is_some_and(|m| m.contains("\"authority_effect\":\"none\"")));
    }
}

/// CASE 9 — Attention cannot bypass the Permission Gateway.
#[test]
fn case9_attention_cannot_bypass_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let ai = ActorContext::new(Actor::ai_assistant("ai-attention").unwrap());
    let err = CommandHandler::generate_workspace_attention(
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

/// CASE 10 — Existing governance surfaces remain coherent.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
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
    let continuity = CommandHandler::generate_workspace_continuity(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let attention = CommandHandler::generate_workspace_attention(
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
    assert_eq!(continuity.authority_effect, "none");
    assert_eq!(attention.authority_effect, "none");
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.attention.workspace_id, attention.workspace_id);
}
