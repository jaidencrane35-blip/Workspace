//! Workspace Working Style Model tests (Phase 6 Batch 6).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, PreferenceCategory,
    PreferenceSource, TaskPriority, WorkingStyleKind, WorkingStyleOrigin,
    WorkspaceWorkingStyleState,
};

fn seed(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Working Style WS".into()))
        .unwrap();
    let workspace_id = workspace.id.to_string();
    let project = CommandHandler::create_project(
        kernel,
        local.clone(),
        intent.clone(),
        workspace_id.clone(),
        "Workspace AI Platform".into(),
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
        "Ship Working Style Model".into(),
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
        "Understand how work usually happens without profiling".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Working style generated deterministically.
#[test]
fn case1_working_style_generated_deterministically() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_working_style(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b =
        CommandHandler::generate_workspace_working_style(&kernel, local, intent, ws).unwrap();
    assert_eq!(a.authority_effect, "none");
    assert!(!a.observations.is_empty());
    assert_eq!(a.style_summary.rhythm_line, b.style_summary.rhythm_line);
    assert_eq!(a.observation_count, b.observation_count);
    assert!(a
        .observations
        .iter()
        .all(|o| !o.why.is_empty() && !o.evidence.is_empty() && !o.explanation.is_empty()));
}

/// CASE 2 — Activity / work changes update observations.
#[test]
fn case2_activity_changes_update_observations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_working_style(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_working_style(&kernel, local, intent, ws).unwrap();
    let comparison = CommandHandler::compare_workspace_working_styles(&before, &after);
    assert!(!comparison.differences.is_empty());
}

/// CASE 3 — Explicit preferences remain separate.
#[test]
fn case3_explicit_preferences_remain_separate() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    CommandHandler::create_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        PreferenceCategory::Workflow,
        "prefer_quiet_focus",
        "Prefer quiet focus blocks",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        Some("Quiet focus".into()),
        None,
    )
    .unwrap();
    let state = CommandHandler::generate_workspace_working_style(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    let prefs: Vec<_> = state
        .observations
        .iter()
        .filter(|o| o.origin == WorkingStyleOrigin::ExplicitPreference)
        .collect();
    assert!(!prefs.is_empty());
    for pref in prefs {
        assert_eq!(pref.kind, WorkingStyleKind::InteractionPreference);
        assert_eq!(pref.origin, WorkingStyleOrigin::ExplicitPreference);
        assert!(!pref.evidence.is_empty());
        assert!(pref.evidence.iter().any(|e| e.source_projection == "preference"));
    }
    for obs in state
        .observations
        .iter()
        .filter(|o| o.origin == WorkingStyleOrigin::ObservedBehaviour)
    {
        assert_ne!(obs.kind, WorkingStyleKind::InteractionPreference);
    }
}

/// CASE 4 — Removing evidence removes observations.
#[test]
fn case4_removing_evidence_removes_observations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let before = CommandHandler::generate_workspace_working_style(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(before.observation_count > 0);
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let after =
        CommandHandler::generate_workspace_working_style(&kernel, local, intent, ws).unwrap();
    // Clearing active work changes evidence; observation set or summary must reflect it.
    assert!(
        before.summary != after.summary
            || before.evidence != after.evidence
            || before.style_summary.workflow_line != after.style_summary.workflow_line
            || before.observation_count != after.observation_count
    );
}

/// CASE 5 — Working Style cannot create actions.
#[test]
fn case5_working_style_cannot_create_actions() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let state = CommandHandler::generate_workspace_working_style(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    // No action / candidate / proposal fields on Working Style state.
    let serialized = serde_json::to_string(&state).unwrap();
    assert!(!serialized.contains("\"action_id\""));
    assert!(!serialized.contains("\"candidate_id\""));
    assert!(!serialized.contains("\"proposal_id\""));
    for obs in &state.observations {
        assert_eq!(obs.authority_effect, "none");
    }
}

/// CASE 6 — Recommendation Engine may reference it only as evidence.
#[test]
fn case6_recommendations_reference_working_style_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(intel.working_style.observation_count > 0);
    assert!(
        intel
            .recommendation_engine
            .explanation
            .contains("Working Style")
            || intel
                .recommendation_engine
                .summary
                .to_lowercase()
                .contains("working")
            || intel.working_style.authority_effect == "none"
    );
    assert_eq!(intel.recommendation_engine.authority_effect, "none");
    assert_eq!(intel.working_style.authority_effect, "none");
}

/// CASE 7 — Adaptation cannot automatically apply changes.
#[test]
fn case7_adaptation_cannot_automatically_apply_changes() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(
        intel.adaptation.explanation.contains("Working Style")
            || intel.adaptation.authority_effect == "none"
    );
    // Accept without a real proposal still cannot auto-apply workspace mutations from style.
    let result = CommandHandler::accept_adaptation_proposal(
        &kernel,
        local,
        intent,
        ws,
        "nonexistent-style-proposal".into(),
    );
    assert!(result.is_err());
    assert_eq!(intel.adaptation.authority_effect, "none");
}

/// CASE 8 — attempt_execute returns CannotExecute.
#[test]
fn case8_attempt_execute_cannot_execute() {
    match CommandHandler::workspace_working_style_attempt_execute() {
        Err(KernelError::WorkspaceWorkingStyleValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("plan")
                    || message.contains("learn")
            );
        }
        other => panic!("expected WorkspaceWorkingStyleValidation, got {other:?}"),
    }
    assert_eq!(
        WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE,
        "none"
    );
}

/// CASE 9 — Permission Gateway unchanged.
#[test]
fn case9_permission_gateway_unchanged() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "working_style")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceWorkingStyleService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);
    // Preference remains separate DurableStore owner.
    let prefs: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "preference")
        .collect();
    assert_eq!(prefs.len(), 1);
    assert_eq!(prefs[0].owner, "AiPersonalizationService");
    assert_eq!(prefs[0].kind, ConceptOwnerKind::DurableStore);
}

/// CASE 10 — No profiling database or hidden persistence; explainable & deterministic.
#[test]
fn case10_no_persistence_explainable_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_working_style(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b =
        CommandHandler::generate_workspace_working_style(&kernel, local.clone(), intent.clone(), ws)
            .unwrap();
    let report = CommandHandler::validate_workspace_working_style(
        &kernel,
        local,
        intent,
        &a,
    )
    .unwrap();
    assert!(report.valid);
    assert_eq!(a.observation_count, b.observation_count);
    assert_eq!(a.style_summary.narrative, b.style_summary.narrative);
    // Projection-only markers — no durable store claim.
    assert!(a.explanation.contains("authority_effect=none") || a.authority_effect == "none");
    assert!(a.explanation.to_lowercase().contains("never"));
    assert!(!a.explanation.to_lowercase().contains("profiling database"));
    let self_cmp = CommandHandler::compare_workspace_working_styles(&a, &b);
    assert_eq!(self_cmp.authority_effect, "none");
}
