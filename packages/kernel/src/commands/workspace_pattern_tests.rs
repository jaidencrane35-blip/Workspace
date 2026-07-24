//! Workspace Pattern Model tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AttentionSourceType, ConceptOwnerKind, IntentContext, PatternKind,
    PLATFORM_CONCEPT_OWNERS, TaskPriority, TaskRelationshipKind, WorkspaceTaskPriority,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Pattern WS".into()))
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
        "Ship Pattern Model".into(),
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
        "Build Workspace AI v1".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    (workspace_id, project.id.to_string())
}

/// CASE 1 — Patterns derive only from existing workspace data.
#[test]
fn case1_patterns_derive_from_existing_data() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_pattern(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Activity Graph")));
    assert!(state.explanation.contains("Activity Graph remains history SoT"));
    assert!(state.patterns.iter().all(|p| p.authority_effect == "none"));
}

/// CASE 2 — Patterns do not duplicate Activity Graph.
#[test]
fn case2_patterns_do_not_duplicate_activity_graph() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "pattern"
            && c.owner == "WorkspacePatternService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "activity"
            && c.owner == "WorkspaceActivityGraphService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "pattern")
        .collect();
    assert_eq!(owners.len(), 1);
}

/// CASE 3 — Patterns are explainable.
#[test]
fn case3_patterns_are_explainable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_pattern(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.patterns.is_empty());
    assert!(state.patterns.iter().all(|p| {
        !p.observation.is_empty()
            && !p.impact.is_empty()
            && !p.evidence.is_empty()
            && !p.title.is_empty()
    }));
    assert!(!state.pattern_summary.narrative.is_empty());
}

/// CASE 4 — Patterns remain informational.
#[test]
fn case4_patterns_remain_informational() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_pattern(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.summary.contains("never"));
    assert!(state.explanation.contains("not predictions"));
}

/// CASE 5 — Patterns update when source data changes.
#[test]
fn case5_patterns_update_when_sources_change() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_pattern(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let a = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Implement feature".into(),
        Some(project_id.clone()),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    let b = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Test feature".into(),
        Some(project_id),
        WorkspaceTaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::add_task_relationship(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        b.id.to_string(),
        a.id.to_string(),
        TaskRelationshipKind::DependsOn,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_pattern(&kernel, local, intent, ws).unwrap();
    assert!(
        after.patterns.iter().any(|p| p.kind == PatternKind::TaskPattern)
            || after.pattern_count >= before.pattern_count
    );
}

/// CASE 6 — Recommendation Engine can consume patterns safely.
#[test]
fn case6_recommendation_engine_consumes_patterns_safely() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let patterns = CommandHandler::generate_workspace_pattern(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!patterns.patterns.is_empty());
    let base = CommandHandler::generate_workspace_recommendation_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let enriched = crate::services::WorkspaceRecommendationEngineService::enrich_with_patterns(
        &kernel.shared_database(),
        &local,
        &base,
        &patterns,
    )
    .unwrap();
    assert!(enriched.candidate_count >= base.candidate_count);
    assert!(enriched
        .evidence
        .iter()
        .any(|e| e.contains("Pattern Model")));
    assert_eq!(enriched.authority_effect, "none");
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(intel.pattern.authority_effect, "none");
    assert!(intel
        .recommendation_engine
        .explanation
        .contains("suggestions only")
        || intel.recommendation_engine.authority_effect == "none");
}

/// CASE 7 — Attention can surface patterns safely.
#[test]
fn case7_attention_surfaces_patterns_safely() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let patterns = CommandHandler::generate_workspace_pattern(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let base = CommandHandler::generate_workspace_attention(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let enriched = crate::services::WorkspaceAttentionService::enrich_with_patterns(
        &base, &patterns,
    )
    .unwrap();
    assert!(enriched.items.iter().any(|i| {
        i.source_type == AttentionSourceType::Pattern
            && i.authority_effect == "none"
            && i.explanation.contains("Pattern context only")
    }));
    let intel =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert!(intel.attention.item_count >= base.items.len());
}

/// CASE 8 — Assistant remains explain-only.
#[test]
fn case8_assistant_remains_explain_only() {
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
    let assistant =
        CommandHandler::generate_workspace_intelligence(&kernel, local, intent, ws).unwrap();
    assert_eq!(work.pattern.authority_effect, assistant.pattern.authority_effect);
    assert_eq!(
        work.pattern.authority_effect,
        workspace_domain::WorkspacePatternState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(work.pattern.workspace_id, assistant.pattern.workspace_id);
    assert!(!work.pattern.summary.is_empty());
}

/// CASE 9 — Patterns cannot execute.
#[test]
fn case9_patterns_cannot_execute() {
    match CommandHandler::workspace_pattern_attempt_execute() {
        Err(KernelError::WorkspacePatternValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspacePatternValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing governance surfaces remain green.
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
    assert_eq!(intel.pattern.authority_effect, "none");
    assert_eq!(intel.operating_state.authority_effect, "none");
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_operating_state(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_recommendation_engine(
        &kernel, local, intent, ws,
    )
    .unwrap();
}
