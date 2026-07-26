//! Workspace Evolution Model tests (Phase 5).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, EvolutionInsightKind, IntentContext, PLATFORM_CONCEPT_OWNERS,
    TaskPriority, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn seed(kernel: &WorkspaceKernel) -> (String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Evolution WS".into()))
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
        "Ship v1".into(),
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

/// CASE 1 — Evolution derives only from existing systems.
#[test]
fn case1_evolution_derives_from_existing_systems() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_evolution(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.evidence.iter().any(|e| e.contains("Activity Graph")));
    assert!(state.explanation.contains("does not store a second history"));
}

/// CASE 2 — Evolution creates no duplicate history source.
#[test]
fn case2_no_duplicate_history_source() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "evolution"
            && c.owner == "WorkspaceEvolutionService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "activity"
            && c.owner == "WorkspaceActivityGraphService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "evolution")
        .collect();
    assert_eq!(owners.len(), 1);
}

/// CASE 3 — Evolution remains deterministic.
#[test]
fn case3_evolution_is_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_evolution(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    // Within one snapshot: insight ids are sorted and unique (deterministic projection).
    let mut ids: Vec<_> = a.insights.iter().map(|i| i.id.clone()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
    ids.dedup();
    assert_eq!(ids.len(), a.insights.len());
    // A second generate sees the first one's audit trail. Since evaluation telemetry is no
    // longer a cognitive input (Sprint 128), the whole insight set must survive — not just
    // a hand-picked subset of kinds.
    let b = CommandHandler::generate_workspace_evolution(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.label, b.label);
    assert_eq!(a.authority_effect, b.authority_effect);
    let insight_ids = |state: &workspace_domain::WorkspaceEvolutionState| {
        state
            .insights
            .iter()
            .map(|i| (i.id.clone(), i.kind.as_str().to_string(), i.title.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(insight_ids(&a), insight_ids(&b));
}

/// CASE 4 — Evolution explanations are user understandable.
#[test]
fn case4_explanations_are_user_understandable() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_evolution(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(!state.summary.is_empty());
    assert!(!state.explanation.is_empty());
    assert!(state.events.iter().all(|e| {
        !e.change.is_empty() && !e.impact.is_empty() && !e.evidence.is_empty()
    }));
    assert!(state
        .insights
        .iter()
        .all(|i| !i.title.is_empty() && !i.explanation.is_empty()));
}

/// CASE 5 — Task changes appear correctly.
#[test]
fn case5_task_changes_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let task = CommandHandler::create_workspace_task(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Complete Task Graph".into(),
        Some(project_id),
        WorkspaceTaskPriority::High,
    )
    .unwrap();
    CommandHandler::update_workspace_task_status(
        &kernel,
        local.clone(),
        intent.clone(),
        task.id.to_string(),
        WorkspaceTaskStatus::Completed,
        Some("Done".into()),
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_evolution(&kernel, local, intent, ws).unwrap();
    assert!(
        state
            .insights
            .iter()
            .any(|i| i.kind == EvolutionInsightKind::TaskProgression)
            || state
                .events
                .iter()
                .any(|e| e.title.contains("Complete Task Graph") || e.kind.contains("task")),
        "task progression should appear in evolution"
    );
}

/// CASE 6 — Purpose progression appears correctly.
#[test]
fn case6_purpose_progression_appears() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_evolution(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state
            .insights
            .iter()
            .any(|i| i.kind == EvolutionInsightKind::PurposeProgression)
            || state.label.contains("Ship Workspace AI"),
        "purpose progression should appear"
    );
}

/// CASE 7 — Composition changes appear correctly.
#[test]
fn case7_composition_changes_appear() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let state = CommandHandler::generate_workspace_evolution(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state.evidence.iter().any(|e| e.contains("Composition"))
            || state
                .insights
                .iter()
                .any(|i| i.kind == EvolutionInsightKind::CompositionShift),
        "composition evidence should appear"
    );
}

/// CASE 8 — Assistant only explains Evolution (shared Intelligence path).
#[test]
fn case8_assistant_only_explains_evolution() {
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
    let assistant = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(work.evolution.authority_effect, "none");
    assert_eq!(assistant.evolution.authority_effect, "none");
    assert_eq!(work.evolution.workspace_id, assistant.evolution.workspace_id);
    assert!(!work.evolution.summary.is_empty());
}

/// CASE 9 — Evolution cannot execute.
#[test]
fn case9_evolution_cannot_execute() {
    match CommandHandler::workspace_evolution_attempt_execute() {
        Err(KernelError::WorkspaceEvolutionValidation { message }) => {
            assert!(message.contains("cannot execute"));
        }
        other => panic!("expected WorkspaceEvolutionValidation, got {other:?}"),
    }
}

/// CASE 10 — Existing governance / intelligence / purpose / composition paths remain green.
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
    assert_eq!(intel.evolution.authority_effect, "none");
    assert_eq!(intel.purpose.authority_effect, "none");
    assert_eq!(intel.composition.authority_effect, "none");
    assert_eq!(intel.environment.authority_effect, "none");
    let _ = CommandHandler::generate_workspace_purpose(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let _ = CommandHandler::generate_workspace_composition(&kernel, local, intent, ws).unwrap();
}

/// CASE 11 — Evaluation does not feed itself: a full Intelligence cycle writes audit
/// events, but the next cycle over unchanged work produces the same cognition.
#[test]
fn case11_evaluation_does_not_contaminate_its_own_inputs() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _) = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let generate = || {
        CommandHandler::generate_workspace_intelligence(
            &kernel,
            local.clone(),
            intent.clone(),
            ws.clone(),
        )
        .unwrap()
    };

    let first = generate();
    let audits_after_first =
        crate::services::AuditService::list_recent(&kernel.shared_database(), 500)
            .unwrap()
            .len();
    let second = generate();
    let audits_after_second =
        crate::services::AuditService::list_recent(&kernel.shared_database(), 500)
            .unwrap()
            .len();
    // The loop is only meaningful if evaluation really does write audit history, and it
    // writes enough of it to have swamped the analysis windows before Sprint 128.
    assert!(
        audits_after_second - audits_after_first > 20,
        "evaluation should still emit telemetry (delta was {})",
        audits_after_second - audits_after_first
    );

    let insights = |state: &workspace_domain::WorkspaceIntelligenceState| {
        state
            .evolution
            .top_insights
            .iter()
            .map(|i| i.id.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(insights(&first), insights(&second));

    let attention = |state: &workspace_domain::WorkspaceIntelligenceState| {
        state
            .attention
            .top_items
            .iter()
            .map(|i| (i.id.as_str().to_string(), i.score))
            .collect::<Vec<_>>()
    };
    assert_eq!(attention(&first), attention(&second));

    let progress = |state: &workspace_domain::WorkspaceIntelligenceState| {
        state.purpose.recent_progress.clone()
    };
    assert_eq!(progress(&first), progress(&second));
}
