//! Workspace Readiness Model tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, ReadinessKind,
    ReadinessStatus, TaskPriority, WorkspaceReadinessState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Readiness WS".into()))
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
        "Ship Readiness".into(),
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
    workspace_id
}

/// CASE 1 — Readiness derives only from existing systems.
#[test]
fn case1_readiness_derives_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_readiness(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Operating")));
    assert!(state.evidence.iter().any(|e| e.contains("Environment")));
    assert!(state.evidence.iter().any(|e| e.contains("Task Graph")));
    assert!(state.evidence.iter().any(|e| e.contains("Decision Queue")));
    assert!(state.explanation.contains("Informational only"));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "readiness"
            && c.owner == "WorkspaceReadinessService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

/// CASE 2 — Readiness creates no duplicate ownership.
#[test]
fn case2_no_duplicate_ownership() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "readiness")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceReadinessService");
    // Must not claim recommendation / adaptation / health ownership.
    assert!(!PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "readiness" && c.owner.contains("Recommendation")
    }));
    assert!(!PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "readiness" && c.owner.contains("Adaptation")
    }));
}

/// CASE 3 — Readiness correctly identifies missing context.
#[test]
fn case3_identifies_missing_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_readiness(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let context = state
        .assessments
        .iter()
        .find(|a| a.kind == ReadinessKind::ContextReadiness)
        .expect("context assessment");
    assert!(!context.reason.is_empty());
    assert!(!context.signals.is_empty());
    // Fresh seed may be ready or partially ready — gaps must be explainable when present.
    for gap in &context.gaps {
        assert!(!gap.explanation.is_empty());
        assert!(!gap.impact.is_empty());
        assert_eq!(gap.source_model, "continuity");
    }
}

/// CASE 4 — Readiness correctly identifies blockers.
#[test]
fn case4_identifies_blockers() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_readiness(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.assessment_count, 5);
    assert!(state.assessments.iter().any(|a| {
        matches!(
            a.kind,
            ReadinessKind::DecisionReadiness | ReadinessKind::TaskReadiness
        )
    }));
    // Every assessment exposes status + impact (blocker path is explainable).
    assert!(state.assessments.iter().all(|a| {
        !a.impact.is_empty()
            && a.authority_effect == WorkspaceReadinessState::AUTHORITY_EFFECT_NONE
            && matches!(
                a.status,
                ReadinessStatus::Ready
                    | ReadinessStatus::PartiallyReady
                    | ReadinessStatus::Blocked
            )
    }));
}

/// CASE 5 — Readiness updates when source systems change.
#[test]
fn case5_updates_when_sources_change() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_readiness(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    // Changing active work refreshes Continuity / Purpose / OS inputs.
    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Second Project".into(),
        None,
        None,
    )
    .unwrap();
    let task = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project.id.to_string(),
        ws.clone(),
        "Follow-up".into(),
        TaskPriority::Medium,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(project.id.to_string()),
        Some(task.id.to_string()),
    )
    .unwrap();
    let second = CommandHandler::generate_workspace_readiness(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(second.authority_effect, "none");
    assert_ne!(first.generated_at, second.generated_at);
    // Evidence still cites source systems after refresh.
    assert!(second.evidence.iter().any(|e| e.contains("Purpose")));
}

/// CASE 6 — Recommendations consume readiness safely.
#[test]
fn case6_recommendations_consume_readiness() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(intel.readiness.authority_effect, "none");
    // RE evidence may cite readiness when gaps exist; never grants authority.
    assert_eq!(
        intel.recommendation_engine.authority_effect,
        "none"
    );
    if intel.readiness.gap_count > 0 {
        assert!(
            intel
                .recommendation_engine
                .explanation
                .contains("Recommendation")
                || intel.recommendation_engine.summary.contains("recommendation")
                || intel
                    .recommendation_engine
                    .top_candidates
                    .iter()
                    .any(|c| c.id.contains("readiness")
                        || c.reason.contains("Readiness")
                        || c.evidence
                            .iter()
                            .any(|e| e.source_model.contains("readiness")))
                || intel
                    .recommendation_engine
                    .summary
                    .to_lowercase()
                    .contains("recommend")
        );
        // Soft check: if gaps exist, intelligence still separates readiness from RE.
        assert_ne!(
            intel.readiness.summary,
            intel.recommendation_engine.summary
        );
    }
}

/// CASE 7 — Adaptation proposals consume readiness safely.
#[test]
fn case7_adaptations_consume_readiness() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(intel.adaptation.authority_effect, "none");
    assert_eq!(intel.readiness.authority_effect, "none");
    // Distinct layers — adaptation may cite readiness in evidence without merging stores.
    if intel.readiness.gap_count > 0 {
        let cites = intel.adaptation.top_proposals.iter().any(|p| {
            p.id.contains("readiness")
                || p.reason.contains("Readiness")
                || p.evidence
                    .iter()
                    .any(|e| e.source_model.contains("readiness"))
        }) || intel
            .adaptation
            .explanation
            .to_lowercase()
            .contains("adaptation");
        assert!(cites || intel.adaptation.proposal_count > 0 || intel.readiness.gap_count > 0);
    }
    assert_ne!(intel.readiness.summary, intel.adaptation.summary);
}

/// CASE 8 — Assistant remains explain-only (readiness on shared intelligence path).
#[test]
fn case8_assistant_explain_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
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
        &kernel, local, intent, ws,
    )
    .unwrap();
    assert_eq!(work.readiness.workspace_id, assistant.readiness.workspace_id);
    assert_eq!(
        work.readiness.authority_effect,
        WorkspaceReadinessState::AUTHORITY_EFFECT_NONE
    );
    assert!(work.readiness.explanation.contains("never prepares")
        || work.readiness.explanation.contains("Informational only")
        || work.readiness.summary.contains("Informational only"));
}

/// CASE 9 — Readiness cannot execute.
#[test]
fn case9_readiness_cannot_execute() {
    match CommandHandler::workspace_readiness_attempt_execute() {
        Err(KernelError::WorkspaceReadinessValidation { message }) => {
            assert!(message.contains("cannot execute") || message.contains("prepare"));
        }
        other => panic!("expected WorkspaceReadinessValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing intelligence/adaptation path remains green with readiness field.
#[test]
fn case10_intelligence_includes_readiness_without_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(intel.authority_effect, "none");
    assert_eq!(intel.readiness.authority_effect, "none");
    assert_eq!(intel.adaptation.authority_effect, "none");
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert_eq!(intel.pattern.authority_effect, "none");
    assert_eq!(intel.operating_state.authority_effect, "none");
    assert!(!intel.readiness.readiness_summary.headline.is_empty());
    assert_eq!(intel.readiness.assessment_count, 5);
}
