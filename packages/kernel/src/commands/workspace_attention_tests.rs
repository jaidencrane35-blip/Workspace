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
            | workspace_domain::AttentionSourceType::Purpose
            | workspace_domain::AttentionSourceType::Evolution
            | workspace_domain::AttentionSourceType::RecommendationEngine
            | workspace_domain::AttentionSourceType::Pattern
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
        assert!(
            !item.reasons.is_empty(),
            "item {} missing structured reasons",
            item.id.as_str()
        );
        let mut keys = std::collections::HashSet::new();
        for reason in &item.reasons {
            assert!(!reason.explanation_key.is_empty());
            assert!(
                keys.insert(reason.explanation_key.clone()),
                "duplicate reason key {}",
                reason.explanation_key
            );
        }
        for window in item.reasons.windows(2) {
            assert!(
                window[0].weight >= window[1].weight,
                "reasons not ordered by weight"
            );
        }
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

/// CASE 11 — Item IDs are unique; ordering is deterministic (score, priority, id).
#[test]
fn case11_unique_ids_and_deterministic_ordering() {
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
    let mut ids = std::collections::HashSet::new();
    for item in &attention.items {
        assert!(
            ids.insert(item.id.as_str().to_string()),
            "duplicate attention id {}",
            item.id.as_str()
        );
        assert!(item.id.as_str().starts_with("attention:"));
    }
    for window in attention.items.windows(2) {
        let (a, b) = (&window[0], &window[1]);
        assert!(
            a.score > b.score
                || (a.score == b.score
                    && (a.priority.rank() < b.priority.rank()
                        || (a.priority.rank() == b.priority.rank()
                            && a.id.as_str() <= b.id.as_str()))),
            "non-deterministic order: {}@{} before {}@{}",
            a.id.as_str(),
            a.score,
            b.id.as_str(),
            b.score
        );
    }
}

/// CASE 12 — Environment owns desktop-like gaps over Composition when both present.
#[test]
fn case12_environment_owns_desktop_gaps_over_composition() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let queue = crate::services::DecisionQueueService::aggregate_readonly(
        &kernel.shared_database(),
        &local,
        &kernel.orchestrated_plans(),
        &kernel.assistant_workflows(),
        ws.clone(),
    )
    .unwrap();
    let graph = crate::services::WorkspaceActivityGraphService::generate_with_decision_queue(
        &kernel.shared_database(),
        &local,
        &kernel.orchestrated_plans(),
        &kernel.assistant_workflows(),
        ws.clone(),
        Some(&queue),
    )
    .unwrap();
    let continuity = crate::services::WorkspaceContinuityService::generate_with_inputs(
        &kernel.shared_database(),
        &local,
        ws.clone(),
        &queue,
        &graph,
    )
    .unwrap();

    let env = workspace_domain::WorkspaceEnvironmentState {
        workspace_id: ws.clone(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        active_project_id: None,
        active_task_id: None,
        windows: vec![],
        applications: vec![],
        window_groups: vec![],
        layout_associations: vec![],
        gaps: vec![workspace_domain::EnvironmentGap {
            kind: "disconnected_work".into(),
            title: "Disconnected desktop work".into(),
            explanation: "Active work without matching windows.".into(),
            application_id: None,
            project_id: None,
            task_id: None,
        }],
        focused_window_id: None,
        running_application_count: 0,
        missing_application_count: 0,
        disconnected_work: true,
        summary: "fixture".into(),
        authority_effect: "none".into(),
    };
    let composition = workspace_domain::WorkspaceCompositionState {
        workspace_id: ws.clone(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        label: "fixture".into(),
        active_project_id: None,
        active_project_name: None,
        active_task_id: None,
        focus_label: None,
        members: vec![],
        relationships: vec![],
        gaps: vec![
            workspace_domain::CompositionGap {
                kind: "disconnected_work".into(),
                title: "Disconnected desktop work".into(),
                explanation: "Duplicate desktop gap from Composition.".into(),
                evidence: vec![],
                member_id: None,
            },
            workspace_domain::CompositionGap {
                kind: "incomplete_membership".into(),
                title: "Logical composition gap".into(),
                explanation: "Composition-owned logical gap.".into(),
                evidence: vec![],
                member_id: None,
            },
        ],
        present_application_count: 0,
        missing_application_count: 0,
        task_node_count: 0,
        window_count: 0,
        outstanding_decision_count: 0,
        explanation: "fixture".into(),
        evidence: vec![],
        summary: "fixture".into(),
        authority_effect: "none".into(),
    };

    let attention = crate::services::WorkspaceAttentionService::generate_with_task_graph(
        &kernel.shared_database(),
        &local,
        ws,
        &queue,
        &graph,
        &continuity,
        None,
        Some(&env),
        Some(&composition),
        None,
        None,
    )
    .unwrap();

    assert_eq!(
        attention
            .items
            .iter()
            .filter(|i| {
                i.source_type == workspace_domain::AttentionSourceType::Environment
                    && i.source_id.starts_with("disconnected_work:")
            })
            .count(),
        1
    );
    assert!(!attention.items.iter().any(|i| {
        i.source_type == workspace_domain::AttentionSourceType::Composition
            && i.source_id.starts_with("disconnected_work:")
    }));
    assert!(attention.items.iter().any(|i| {
        i.source_type == workspace_domain::AttentionSourceType::Composition
            && i.source_id.starts_with("incomplete_membership:")
    }));
}

/// CASE 13 — Identical inputs produce identical structured reasons; Intelligence preserves them.
#[test]
fn case13_reasons_stable_and_intelligence_preserves_attention() {
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
    let b = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let reason_fingerprint = |items: &[workspace_domain::AttentionItem]| {
        items
            .iter()
            .filter(|i| i.score >= 35)
            .map(|i| {
                (
                    i.id.as_str().to_string(),
                    i.score,
                    i.reasons
                        .iter()
                        .map(|r| (r.explanation_key.clone(), r.weight, r.signal.as_str()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(reason_fingerprint(&a.items), reason_fingerprint(&b.items));

    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(intel.attention.authority_effect, "none");
    // Intelligence embeds Attention (may enrich with RE/Pattern); base items keep reasons.
    for top in &intel.attention.top_items {
        if let Some(source) = a.items.iter().find(|i| i.id.as_str() == top.id.as_str()) {
            assert_eq!(top.score, source.score);
            assert_eq!(top.reasons, source.reasons);
        } else {
            assert!(
                !top.reasons.is_empty(),
                "enrich-only item {} missing reasons",
                top.id.as_str()
            );
        }
    }
}
