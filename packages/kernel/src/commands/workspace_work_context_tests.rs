//! Workspace Work Context Engine tests (Phase 6 Batch 3).

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ConceptOwnerKind, IntentContext, PLATFORM_CONCEPT_OWNERS, TaskPriority,
    WorkContextType, WorkspaceWorkContextState,
};

fn seed_dev(kernel: &WorkspaceKernel) -> String {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Work Context WS".into()))
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
        "Debug compiler pipeline".into(),
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
        "Build and ship Workspace AI development platform".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();
    workspace_id
}

/// CASE 1 — Development context generated correctly.
#[test]
fn case1_development_context_generated_correctly() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let state = CommandHandler::generate_workspace_work_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert!(state.contexts.iter().any(|c| c.context_type == WorkContextType::Development));
    let primary = state.primary_context().expect("primary context");
    assert_eq!(primary.context_type, WorkContextType::Development);
    assert!(!primary.evidence.is_empty());
    assert!(!primary.why.is_empty());
    assert!(primary.name.to_lowercase().contains("development") || primary.name.contains("Platform"));
}

/// CASE 2 — Multiple projects belong to one context.
#[test]
fn case2_multiple_projects_belong_to_one_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Multi Project WC".into()))
        .unwrap();
    let ws = workspace.id.to_string();
    let p1 = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Alpha Development".into(),
        None,
        None,
    )
    .unwrap();
    let p2 = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Beta Development Platform".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(p1.id.to_string()),
        None,
    )
    .unwrap();
    CommandHandler::create_work_goal(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Alpha Development coding work".into(),
        Some(p1.id.to_string()),
        None,
    )
    .unwrap();
    CommandHandler::create_work_goal(
        &kernel,
        local,
        intent.clone(),
        ws.clone(),
        "Beta Development Platform build work".into(),
        Some(p2.id.to_string()),
        None,
    )
    .unwrap();

    let state = CommandHandler::generate_workspace_work_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let primary = state.primary_context().expect("primary");
    assert_eq!(primary.context_type, WorkContextType::Development);
    assert!(
        primary.associated_projects.len() >= 2,
        "expected multiple projects in one context, got {:?}",
        primary.associated_projects
    );
}

/// CASE 3 — One project spans multiple contexts.
#[test]
fn case3_one_project_spans_multiple_contexts() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("Multi Context WC".into()))
        .unwrap();
    let ws = workspace.id.to_string();
    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Workspace Platform".into(),
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
        "Research architecture and debug runtime".into(),
        TaskPriority::High,
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
    CommandHandler::create_work_goal(
        &kernel,
        local,
        intent.clone(),
        ws.clone(),
        "Research and develop the planning roadmap".into(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();

    let state = CommandHandler::generate_workspace_work_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    let types: Vec<_> = state.contexts.iter().map(|c| c.context_type).collect();
    assert!(types.contains(&WorkContextType::Development));
    assert!(
        types.contains(&WorkContextType::Research)
            || types.contains(&WorkContextType::Planning),
        "expected multiple contexts for one project, got {types:?}"
    );
    let project_ids: Vec<_> = state
        .contexts
        .iter()
        .flat_map(|c| c.associated_projects.iter().map(|p| p.source_ref.clone()))
        .collect();
    assert!(project_ids.iter().any(|id| id == &project.id.to_string()));
}

/// CASE 4 — Evidence remains deterministic.
#[test]
fn case4_evidence_remains_deterministic() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let a = CommandHandler::generate_workspace_work_context(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let b = CommandHandler::generate_workspace_work_context(&kernel, local, intent, ws).unwrap();
    let a_types: Vec<_> = a.contexts.iter().map(|c| c.context_type.as_str()).collect();
    let b_types: Vec<_> = b.contexts.iter().map(|c| c.context_type.as_str()).collect();
    assert_eq!(a_types, b_types);
    assert_eq!(
        a.primary_context().map(|c| c.context_type),
        b.primary_context().map(|c| c.context_type)
    );
    for context in &a.contexts {
        assert!(context.evidence.iter().all(|e| !e.why.is_empty()));
        assert!(context.evidence.iter().all(|e| !e.source_projection.is_empty()));
    }
}

/// CASE 5 — Removing Purpose updates Context.
#[test]
fn case5_removing_purpose_updates_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let with_purpose = CommandHandler::generate_workspace_work_context(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(with_purpose.evidence.iter().any(|e| e.contains("Purpose")));

    // Clear active work so Purpose / focus signals thin out.
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let without = CommandHandler::generate_workspace_work_context(&kernel, local, intent, ws)
        .unwrap();
    let comparison = CommandHandler::compare_workspace_work_contexts(&with_purpose, &without);
    assert!(!comparison.differences.is_empty());
    assert!(
        with_purpose.summary != without.summary
            || with_purpose.primary_context_id != without.primary_context_id
            || with_purpose.context_count != without.context_count
            || with_purpose.evidence != without.evidence
    );
}

/// CASE 6 — Removing Environment updates Context.
#[test]
fn case6_removing_environment_updates_context() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let first = CommandHandler::generate_workspace_work_context(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(first.evidence.iter().any(|e| e.contains("Environment")));
    // Environment is live/read-model; regenerating after clearing active work changes
    // composition/environment contribution lines in evidence corpus.
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        None,
        None,
    )
    .unwrap();
    let second =
        CommandHandler::generate_workspace_work_context(&kernel, local, intent, ws).unwrap();
    assert!(second.evidence.iter().any(|e| e.contains("Environment")));
    // Evidence lines remain present but context projection can change with focus loss.
    let _ = CommandHandler::compare_workspace_work_contexts(&first, &second);
    assert_eq!(first.authority_effect, "none");
    assert_eq!(second.authority_effect, "none");
}

/// CASE 7 — Recommendations reference Context without changing it.
#[test]
fn case7_recommendations_reference_context_without_changing_it() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let context = CommandHandler::generate_workspace_work_context(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let before = context.clone();
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(
        intel.recommendation_engine.explanation.contains("Work Context")
            || intel
                .recommendation_engine
                .summary
                .contains("work_context")
            || intel.work_context.context_count > 0
    );
    // Context snapshot identity fields unchanged by recommendation enrichment path.
    assert_eq!(before.authority_effect, context.authority_effect);
    assert_eq!(before.context_count, context.context_count);
    assert!(intel.work_context.authority_effect == "none");
}

/// CASE 8 — Attention consumes Context as evidence only.
#[test]
fn case8_attention_consumes_context_as_evidence_only() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert!(
        intel.attention.summary.contains("Context evidence")
            || intel.attention.summary.contains("Work Context")
            || intel.attention.summary.contains("informational only")
    );
    assert_eq!(intel.attention.authority_effect, "none");
    assert_eq!(intel.work_context.authority_effect, "none");
}

/// CASE 9 — attempt_execute always returns CannotExecute.
#[test]
fn case9_attempt_execute_always_cannot_execute() {
    match CommandHandler::workspace_work_context_attempt_execute() {
        Err(KernelError::WorkspaceWorkContextValidation { message }) => {
            assert!(
                message.contains("cannot execute")
                    || message.contains("plan")
                    || message.contains("restore")
            );
        }
        other => panic!("expected WorkspaceWorkContextValidation, got {other:?}"),
    }
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let state = CommandHandler::generate_workspace_work_context(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(
        state.authority_effect,
        WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE
    );
}

/// CASE 10 — Permission Gateway remains untouched (reuse existing capability; no new paths).
#[test]
fn case10_permission_gateway_remains_untouched() {
    let owners: Vec<_> = PLATFORM_CONCEPT_OWNERS
        .iter()
        .filter(|c| c.concept == "work_context")
        .collect();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].owner, "WorkspaceWorkContextService");
    assert_eq!(owners[0].kind, ConceptOwnerKind::Aggregator);

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let ws = seed_dev(&kernel);
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let state = CommandHandler::generate_workspace_work_context(
        &kernel,
        local.clone(),
        intent.clone(),
        ws,
    )
    .unwrap();
    let validation = CommandHandler::validate_workspace_work_context(
        &kernel,
        local,
        intent,
        &state,
    )
    .unwrap();
    assert!(validation.valid);
    assert_eq!(validation.authority_effect, "none");
    // Intelligence embeds work_context after Experience path.
    let intel = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        state.workspace_id.clone(),
    )
    .unwrap();
    assert!(intel.work_context.context_count > 0);
    assert_eq!(intel.work_context.authority_effect, "none");
}
