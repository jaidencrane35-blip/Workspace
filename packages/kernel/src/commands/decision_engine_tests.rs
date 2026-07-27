//! Decision Engine tests (Phase 5 — Sprints 80–81).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, AutomationTriggerKind, ConceptOwnerKind, IntentContext, MemoryType,
    PLATFORM_CONCEPT_OWNERS, PreferenceCategory, PreferenceSource, TaskPriority,
};

fn seed_project(kernel: &WorkspaceKernel) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Decision Engine WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Engine Project".into(),
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
        "Engine Task".into(),
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
        "Engine contract".into(),
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

/// CASE 1 — Workspace state changes → decision ranking updates.
#[test]
fn case1_workspace_state_changes_update_ranking() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let before = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let after = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(after.candidates.len() >= before.candidates.len());
    assert_eq!(after.authority_effect, "none");
    // Ranking should reflect new pending approval attention.
    assert!(
        after.context.pending_approval_count > before.context.pending_approval_count
            || after.top_candidates.first().map(|c| c.score.total)
                != before.top_candidates.first().map(|c| c.score.total)
            || after.candidates.len() != before.candidates.len()
    );
}

/// CASE 2 — Attention changes → recommendations reprioritize.
#[test]
fn case2_attention_changes_reprioritize() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    let before = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let after = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(after.context.attention_item_count >= before.context.attention_item_count);
    assert!(!after.top_candidates.is_empty());
    assert!(after.top_candidates[0].explanation.reasons.iter().any(|r| {
        r.kind == "attention" || r.kind == "attention_signal" || r.kind == "approval"
    }));
}

/// CASE 12 — Attention reasons reach Decision Engine structurally, and Decision Engine
/// neither reorders nor rescores what Attention ranked.
#[test]
fn case12_attention_reasons_survive_into_decision_candidates() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let engine = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();

    let mut checked = 0;
    for candidate in &engine.candidates {
        let Some(ref attention_id) = candidate.attention_item_id else {
            continue;
        };
        let Some(source) = intel
            .attention
            .top_items
            .iter()
            .find(|i| i.id.as_str() == attention_id)
        else {
            continue;
        };
        // Every Attention reason arrives whole and in Attention's order.
        let carried: Vec<_> = candidate
            .explanation
            .reasons
            .iter()
            .filter_map(|r| r.attention_reason.clone())
            .collect();
        assert_eq!(
            carried, source.reasons,
            "candidate {} lost or reordered Attention reasons",
            candidate.id
        );
        // Decision Engine adds its own framing but never rewrites the upstream weighting.
        assert_eq!(
            candidate.score.attention_contribution,
            source.score.min(100),
            "candidate {} rescored the Attention item",
            candidate.id
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "expected at least one Attention-derived decision candidate"
    );
}

/// CASE 13 — Rationale is structured end to end: no Attention-sourced reason is left as
/// bare text, and repeated generation is stable.
#[test]
fn case13_attention_rationale_is_structured_and_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let generate = || {
        CommandHandler::generate_decision_engine(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            ws.clone(),
        )
        .unwrap()
    };
    let first = generate();
    let second = generate();

    let rationale = |state: &workspace_domain::DecisionEngineState| {
        state
            .candidates
            .iter()
            .map(|c| {
                let reasons: Vec<_> = c
                    .explanation
                    .reasons
                    .iter()
                    .map(|r| {
                        (
                            r.kind.clone(),
                            r.attention_reason
                                .as_ref()
                                .map(|a| (a.signal.as_str(), a.weight)),
                        )
                    })
                    .collect();
                (c.id.to_string(), reasons)
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(rationale(&first), rationale(&second));

    for candidate in &first.candidates {
        for reason in &candidate.explanation.reasons {
            if reason.kind == "attention_signal" {
                let attention_reason = reason
                    .attention_reason
                    .as_ref()
                    .expect("attention_signal reason must carry structured data");
                assert!(
                    !attention_reason.explanation_key.is_empty(),
                    "structured reason needs a key for Experience to translate"
                );
            }
        }
    }
}

/// CASE 3 — Preferences removed → explanations update.
#[test]
fn case3_preferences_removed_update_explanations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let pref = CommandHandler::create_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        PreferenceCategory::Application,
        "preferred_editor",
        "VS Code",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        Some("Preferred editor".into()),
        None,
    )
    .unwrap();
    let with_pref = CommandHandler::generate_decision_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(with_pref.context.preference_highlight_count > 0);
    assert!(with_pref.candidates.iter().any(|c| {
        c.score.personalization_contribution > 0
            || c.explanation
                .reasons
                .iter()
                .any(|r| r.kind == "personalization")
    }));

    CommandHandler::delete_user_preference(&kernel, local.clone(), intent.clone(), pref.id.to_string())
        .unwrap();
    let without = CommandHandler::generate_decision_engine(&kernel, local, intent, ws).unwrap();
    assert_eq!(without.context.preference_highlight_count, 0);
    for candidate in &without.candidates {
        assert_eq!(candidate.score.personalization_contribution, 0);
        assert!(!candidate
            .explanation
            .reasons
            .iter()
            .any(|r| r.kind == "personalization"));
    }
}

/// CASE 4 — Memory removed → evidence updates.
#[test]
fn case4_memory_removed_updates_evidence() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_project(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let entry = CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "last_tool",
        "Used Terminal frequently",
        "test",
        Some(ws.clone()),
        None,
    )
    .unwrap();
    let with_mem = CommandHandler::generate_decision_engine(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(with_mem.context.memory_highlight_count > 0);
    assert!(with_mem.candidates.iter().any(|c| {
        c.score.memory_contribution > 0
            || c.explanation.reasons.iter().any(|r| r.kind == "memory")
    }));

    CommandHandler::delete_memory_entry(&kernel, local.clone(), intent.clone(), entry.id.to_string())
        .unwrap();
    let without = CommandHandler::generate_decision_engine(&kernel, local, intent, ws).unwrap();
    assert_eq!(without.context.memory_highlight_count, 0);
    for candidate in &without.candidates {
        assert_eq!(candidate.score.memory_contribution, 0);
        assert!(!candidate
            .explanation
            .reasons
            .iter()
            .any(|r| r.kind == "memory"));
    }
}

/// CASE 5 — Permission denied → decision remains informational.
#[test]
fn case5_permission_denied_remains_informational() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let state = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    for candidate in &state.candidates {
        assert_eq!(candidate.authority_effect, "none");
        assert_eq!(
            candidate.handoff_command,
            workspace_domain::DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL
        );
    }
}

/// CASE 6 — Decision accepted → Planner receives selected recommendation (handoff).
#[test]
fn case6_accept_hands_off_to_planner() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    let state = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
    )
    .unwrap();
    let candidate = state
        .top_candidates
        .first()
        .expect("expected a recommendation");
    let result = CommandHandler::select_decision_candidate(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        candidate.id.to_string(),
    )
    .unwrap();
    let handoff = result.handoff.expect("expected planner handoff");
    assert_eq!(handoff.next_command, "submit_assistant_goal");
    assert_eq!(handoff.authority_effect, "none");
    assert!(!handoff.goal_statement.is_empty());
    assert_eq!(handoff.workspace_id, ws);
    // Planner receives the recommendation when submit_assistant_goal is invoked with handoff.
    // Decision Engine itself must not create the plan (keeps layers independent).
    let workflows = kernel.assistant_workflows();
    assert!(workflows.lock().unwrap().list_all().is_empty());
}

/// CASE 7 — Decision Engine attempts execution → architecture prevents it.
#[test]
fn case7_execution_attempt_blocked() {
    let err = CommandHandler::decision_engine_attempt_execute().unwrap_err();
    match err {
        KernelError::DecisionEngineCannotExecute => {}
        other => panic!("expected DecisionEngineCannotExecute, got {other:?}"),
    }
}

#[test]
fn decision_engine_concept_ownership_registered() {
    assert!(PLATFORM_CONCEPT_OWNERS.iter().any(|c| {
        c.concept == "decision_candidate"
            && c.owner == "DecisionEngineService"
            && c.kind == ConceptOwnerKind::Aggregator
    }));
}

#[test]
fn multiple_candidates_supported() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, project_id, _) = seed_project(&kernel);
    seed_pending_contract(&kernel, ws.clone(), project_id);
    CommandHandler::create_work_goal(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws.clone(),
        "Ship decision engine".into(),
        None,
        None,
    )
    .unwrap();
    let state = CommandHandler::generate_decision_engine(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        state.candidates.len() >= 2,
        "Decision Engine must support multiple candidates, got {}",
        state.candidates.len()
    );
}
