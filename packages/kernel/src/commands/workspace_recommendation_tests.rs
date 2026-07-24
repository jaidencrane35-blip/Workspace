//! Workspace Recommendation Engine tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AttentionSourceType, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS,
    RecommendationKind, TaskPriority, WorkspaceTaskPriority,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Recommendation WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace Platform".into(),
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
        "Ship Recommendation Engine".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    CommandHandler::create_work_goal(
        kernel,
        local,
        intent,
        workspace_id.clone(),
        "Ship Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Recommendations derive only from existing systems.
#[test]
fn case1_recommendations_derive_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Attention")));
    assert!(state.evidence.iter().any(|e| e.contains("Purpose")));
    assert!(state.explanation.contains("suggestions only"));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "recommendation_candidate"
            && c.owner == "WorkspaceRecommendationEngineService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

/// CASE 2 — Recommendations are deterministic.
#[test]
fn case2_recommendations_are_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    // Within one snapshot: ids unique; kind priority order is stable.
    let mut ids: Vec<_> = a.candidates.iter().map(|c| c.id.clone()).collect();
    let before = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), before);
    let ranks: Vec<_> = a
        .candidates
        .iter()
        .map(|c| match c.kind {
            RecommendationKind::ResolveBlocker => 0u8,
            RecommendationKind::ReviewDecision => 1,
            RecommendationKind::RestoreContext => 2,
            RecommendationKind::ContinueWork => 3,
            RecommendationKind::CompleteTask => 4,
            RecommendationKind::ReorganizeWorkspace => 5,
            RecommendationKind::ExploreOpportunity => 6,
        })
        .collect();
    let mut sorted_ranks = ranks.clone();
    sorted_ranks.sort();
    assert_eq!(ranks, sorted_ranks);
    // Stable kinds survive a second generate despite audit noise.
    let b = CommandHandler::generate_workspace_recommendation_engine(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(a.authority_effect, b.authority_effect);
    let stable = |state: &workspace_domain::WorkspaceRecommendationEngineState| {
        let mut kinds: Vec<_> = state
            .candidates
            .iter()
            .filter(|c| {
                matches!(
                    c.kind,
                    RecommendationKind::ContinueWork
                        | RecommendationKind::CompleteTask
                        | RecommendationKind::ReorganizeWorkspace
                )
            })
            .map(|c| c.kind.as_str().to_string())
            .collect();
        kinds.sort();
        kinds
    };
    assert_eq!(stable(&a), stable(&b));
}

/// CASE 3 — Every recommendation has evidence.
#[test]
fn case3_every_recommendation_has_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Open Task Graph node".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert!(!state.candidates.is_empty());
    assert!(state.candidates.iter().all(|c| {
        !c.reason.is_empty()
            && !c.impact.is_empty()
            && !c.evidence.is_empty()
            && c.authority_effect == "none"
    }));
}

/// CASE 4 — Recommendations cannot modify source systems.
#[test]
fn case4_recommendations_cannot_modify_sources() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_purpose(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let after = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
    assert_eq!(before.label, after.label);
    assert_eq!(before.progress_percent, after.progress_percent);
    assert_eq!(before.authority_effect, after.authority_effect);
}

/// CASE 5 — Attention consumes recommendations correctly.
#[test]
fn case5_attention_consumes_recommendations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Open work for recommendations".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let recommendations = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!recommendations.candidates.is_empty());
    let base = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let enriched = crate::services::WorkspaceAttentionService::enrich_with_recommendations(
        &base,
        &recommendations,
    )
    .unwrap();
    assert!(enriched.items.len() >= base.items.len());
    assert!(enriched.items.iter().any(|i| {
        i.source_type == AttentionSourceType::RecommendationEngine
            && i.authority_effect == "none"
            && i.explanation.contains("Suggestion only")
    }));
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert!(intel.attention.item_count >= base.items.len());
}

/// CASE 6 — Assistant only explains recommendations (no accept/execute path).
#[test]
fn case6_assistant_only_explains() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let work = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant = CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(
        work.recommendation_engine.authority_effect,
        assistant.recommendation_engine.authority_effect
    );
    assert_eq!(
        work.recommendation_engine.authority_effect,
        workspace_domain::WorkspaceRecommendationEngineState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work.recommendation_engine.workspace_id,
        assistant.recommendation_engine.workspace_id
    );
    assert!(!work.recommendation_engine.summary.is_empty());
    assert!(
        work.recommendation_engine.summary.contains("never execute")
            || work
                .recommendation_engine
                .explanation
                .contains("suggestions only")
    );
}

/// CASE 7 — Purpose and Evolution context appears correctly.
#[test]
fn case7_purpose_and_evolution_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(state.evidence.iter().any(|e| e.contains("Purpose")));
    assert!(state.evidence.iter().any(|e| e.contains("Evolution")));
    assert!(state
        .candidates
        .iter()
        .any(|c| c.related_purpose_label.is_some()));
}

/// CASE 8 — Recommendations survive regeneration.
#[test]
fn case8_recommendations_survive_regeneration() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let _ = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Persistable open task".into(),
        Some(project_id),
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    let a = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_recommendation_engine(&kernel, local, intent, ws)
        .unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(a.authority_effect, "none");
    assert!(a.candidates.iter().any(|c| {
        matches!(
            c.kind,
            RecommendationKind::CompleteTask | RecommendationKind::ContinueWork
        )
    }));
    assert!(b.candidates.iter().any(|c| {
        matches!(
            c.kind,
            RecommendationKind::CompleteTask | RecommendationKind::ContinueWork
        )
    }));
    // Stable CompleteTask ids (task UUIDs) survive regeneration.
    let a_tasks: Vec<_> = a
        .candidates
        .iter()
        .filter(|c| c.kind == RecommendationKind::CompleteTask)
        .map(|c| c.id.clone())
        .collect();
    let b_tasks: Vec<_> = b
        .candidates
        .iter()
        .filter(|c| c.kind == RecommendationKind::CompleteTask)
        .map(|c| c.id.clone())
        .collect();
    assert_eq!(a_tasks, b_tasks);
}

/// CASE 9 — Recommendations cannot bypass Permission Gateway.
#[test]
fn case9_recommendations_cannot_bypass_gateway() {
    match CommandHandler::workspace_recommendation_engine_attempt_execute() {
        Err(KernelError::WorkspaceRecommendationEngineValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceRecommendationEngineValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing governance / intelligence surfaces remain green.
#[test]
fn case10_governance_surfaces_remain_green() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert_eq!(intel.evolution.authority_effect, "none");
    assert_eq!(intel.purpose.authority_effect, "none");
    assert_eq!(intel.composition.authority_effect, "none");
    assert_eq!(intel.environment.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_evolution(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_purpose(&kernel, local, intent, ws).unwrap();
}
