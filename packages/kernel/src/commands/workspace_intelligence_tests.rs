//! Workspace Intelligence integrity tests (Phase 4 Batch 5.5).
//!
//! Proves governance, isolation, persistence, shared path, and audit guarantees.
//! Does not expand intelligence capabilities.

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandHandler;
use crate::error::KernelError;
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    ActorContext, ActorType, ApprovalDecisionKind, CapabilitySet, IntentContext, MemoryType,
    PreferenceCategory, PreferenceSource, TaskPriority,
};

fn seed_workspace(kernel: &WorkspaceKernel, name: &str) -> (String, String, String) {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new(name.into()))
        .unwrap();
    let app_a = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "VS Code".into(),
            None,
            Some("code.exe".into()),
        ))
        .unwrap();
    let app_b = CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id.clone(),
            "Terminal".into(),
            None,
            Some("wt.exe".into()),
        ))
        .unwrap();
    (
        workspace.id.to_string(),
        app_a.id.to_string(),
        app_b.id.to_string(),
    )
}

/// CASE 1 — Workspace Intelligence cannot execute anything.
#[test]
fn case1_intelligence_cannot_execute_anything() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel CASE1");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_launches = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    let before_decisions = before
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("DecideApproval") && r.success)
        .count();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_launches = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("LaunchApplication") && r.success)
        .count();
    let after_decisions = after
        .iter()
        .filter(|r| r.command_name.as_deref() == Some("DecideApproval") && r.success)
        .count();
    assert_eq!(before_launches, after_launches);
    assert_eq!(before_decisions, after_decisions);
    let _ = app_a;
}

/// CASE 2 — Workspace Intelligence cannot bypass Permission Gateway.
#[test]
fn case2_intelligence_cannot_bypass_permission_gateway() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel, "Intel CASE2");
    let ai = ActorContext::new(workspace_domain::Actor::ai_assistant("ai-intel-5.5-2").unwrap());
    assert_eq!(
        CapabilitySet::for_actor_type(ActorType::AIAssistant)
            .iter()
            .count(),
        0
    );

    let err = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ai,
        IntentContext::ai_suggestion(),
        ws,
    );
    match err {
        Err(KernelError::PermissionDenied(_)) | Err(KernelError::ApprovalRequired { .. }) => {}
        other => panic!("expected permission boundary, got {other:?}"),
    }
}

/// CASE 3 — Removing memory changes intelligence output.
#[test]
fn case3_removing_memory_changes_intelligence_output() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel CASE3");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let memory = CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "preferred_application",
        "User prefers VS Code for coding workspaces",
        "intelligence_integrity",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let with_memory = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!with_memory.memory_highlights.is_empty());
    // Recommendations come from Attention; memory remains an informational highlight.
    assert!(with_memory
        .memory_highlights
        .iter()
        .any(|h| h.summary.to_lowercase().contains("vs code")
            || h.summary.to_lowercase().contains("coding")));

    CommandHandler::delete_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        memory.id.to_string(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(after.memory_highlights.is_empty());
}

/// CASE 4 — Changing preferences changes preference highlights (Attention owns prioritization).
#[test]
fn case4_changing_preferences_changes_recommendations() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel CASE4");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let before = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(before.preference_highlights.is_empty());
    // Recommendations come from Attention — not a parallel preference ranking path.
    assert!(before
        .recommended_actions
        .iter()
        .all(|r| r.kind.starts_with("attention:") || r.kind == "bootstrap"));

    let preference = CommandHandler::create_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        PreferenceCategory::Application,
        "default_editor",
        "VS Code is my default editor",
        PreferenceSource::UserDefined,
        Some(ws.clone()),
        Some("VS Code".into()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let with_pref = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(!with_pref.preference_highlights.is_empty());
    assert!(with_pref
        .preference_highlights
        .iter()
        .any(|h| h.summary.to_lowercase().contains("vs code") || h.label.contains("VS Code")));

    CommandHandler::delete_user_preference(
        &kernel,
        local.clone(),
        intent.clone(),
        preference.id.to_string(),
    )
    .unwrap();

    let after = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(after.preference_highlights.is_empty());
}

/// CASE 5 — Restart preserves Project and Task state.
#[test]
fn case5_restart_preserves_project_and_task_state() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("intent-persist-5.5.db");

    let project_id;
    let task_id;
    let workspace_id;
    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
            .execute_mutation(CreateWorkspace::new("Durable Intent WS".into()))
            .unwrap();
        workspace_id = workspace.id.to_string();
        let project = CommandHandler::create_project(
            &kernel,
            local.clone(),
            intent.clone(),
            workspace_id.clone(),
            "Workspace AI".into(),
            None,
            None,
        )
        .unwrap();
        let task = CommandHandler::create_task(
            &kernel,
            local.clone(),
            intent.clone(),
            project.id.to_string(),
            workspace_id.clone(),
            "Survive restart".into(),
            TaskPriority::High,
        )
        .unwrap();
        CommandHandler::set_active_work(
            &kernel,
            local,
            intent,
            workspace_id.clone(),
            Some(project.id.to_string()),
            Some(task.id.to_string()),
        )
        .unwrap();
        project_id = project.id.to_string();
        task_id = task.id.to_string();
    }

    {
        let kernel = WorkspaceKernel::initialize(&db_path).unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let project =
            CommandHandler::get_project(&kernel, local.clone(), intent.clone(), project_id.clone())
                .unwrap();
        let task =
            CommandHandler::get_task(&kernel, local.clone(), intent.clone(), task_id.clone())
                .unwrap();
        let context = CommandHandler::get_workflow_context(
            &kernel,
            local.clone(),
            intent.clone(),
            workspace_id.clone(),
        )
        .unwrap();
        let state = CommandHandler::generate_workspace_intelligence(
            &kernel,
            local,
            intent,
            workspace_id,
        )
        .unwrap();

        assert_eq!(project.name, "Workspace AI");
        assert_eq!(task.title, "Survive restart");
        assert_eq!(
            context.active_project_id.as_ref().map(|id| id.as_str()),
            Some(project_id.as_str())
        );
        assert_eq!(
            context.active_task_id.as_ref().map(|id| id.as_str()),
            Some(task_id.as_str())
        );
        assert_eq!(
            state.current_project.as_ref().map(|p| p.id.as_str()),
            Some(project_id.as_str())
        );
        assert_eq!(
            state.current_task.as_ref().map(|t| t.id.as_str()),
            Some(task_id.as_str())
        );
    }
}

/// CASE 6 — Multiple projects / workspaces remain isolated.
#[test]
fn case6_multiple_workspaces_remain_isolated() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let (ws_a, app_a, _) = seed_workspace(&kernel, "Workspace A");
    let (ws_b, app_b, _) = seed_workspace(&kernel, "Workspace B");

    let project_a = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_a.clone(),
        "Project A".into(),
        None,
        None,
    )
    .unwrap();
    let task_a = CommandHandler::create_task(
        &kernel,
        local.clone(),
        intent.clone(),
        project_a.id.to_string(),
        ws_a.clone(),
        "Task A".into(),
        TaskPriority::High,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_a.clone(),
        Some(project_a.id.to_string()),
        Some(task_a.id.to_string()),
    )
    .unwrap();
    CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "secret_a",
        "Memory only for workspace A",
        "isolation",
        Some(ws_a.clone()),
        None,
    )
    .unwrap();

    let project_b = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_b.clone(),
        "Project B".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_b.clone(),
        Some(project_b.id.to_string()),
        None,
    )
    .unwrap();

    // Cross-workspace active work must be rejected.
    let cross = CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_b.clone(),
        Some(project_a.id.to_string()),
        None,
    );
    assert!(matches!(
        cross,
        Err(KernelError::WorkspaceIntentValidation { .. })
    ));

    let _ = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai-a",
        "Prepare workspace A apps",
        vec![app_a.clone()],
        Some(ws_a.clone()),
    )
    .unwrap();
    let _ = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai-b",
        "Prepare workspace B apps",
        vec![app_b.clone()],
        Some(ws_b.clone()),
    )
    .unwrap();

    let state_a = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws_a.clone(),
    )
    .unwrap();
    let state_b = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws_b,
    )
    .unwrap();

    assert_eq!(
        state_a.current_project.as_ref().map(|p| p.name.as_str()),
        Some("Project A")
    );
    assert_eq!(
        state_b.current_project.as_ref().map(|p| p.name.as_str()),
        Some("Project B")
    );
    assert!(state_a
        .memory_highlights
        .iter()
        .any(|m| m.summary.contains("workspace A")));
    assert!(!state_b
        .memory_highlights
        .iter()
        .any(|m| m.summary.contains("workspace A")));
    assert!(state_a
        .pending_plans
        .iter()
        .any(|p| p.contains("Prepare workspace A apps")));
    assert!(!state_a
        .pending_plans
        .iter()
        .any(|p| p.contains("Prepare workspace B apps")));
    assert!(state_b
        .pending_plans
        .iter()
        .any(|p| p.contains("Prepare workspace B apps")));
    assert!(!state_b
        .pending_plans
        .iter()
        .any(|p| p.contains("Prepare workspace A apps")));
    assert!(!state_a.current_applications.iter().any(|a| a.id == app_b));
    assert!(!state_b.current_applications.iter().any(|a| a.id == app_a));
}

/// CASE 7 — Assistant and Work tab produce consistent intelligence (same API).
#[test]
fn case7_assistant_and_work_share_intelligence_path() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel, "Intel CASE7");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Shared Path".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();

    // Product Work tab and Assistant both call generate_workspace_intelligence.
    let work_view = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    let assistant_view = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    assert_eq!(work_view.workspace_id, assistant_view.workspace_id);
    assert_eq!(
        work_view.current_project.as_ref().map(|p| p.id.as_str()),
        assistant_view.current_project.as_ref().map(|p| p.id.as_str())
    );
    assert_eq!(work_view.authority_effect, "none");
    assert_eq!(assistant_view.authority_effect, "none");
    assert_eq!(
        work_view.environment.authority_effect,
        assistant_view.environment.authority_effect
    );
    assert_eq!(
        work_view.environment.authority_effect,
        workspace_domain::WorkspaceEnvironmentState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.composition.authority_effect,
        assistant_view.composition.authority_effect
    );
    assert_eq!(
        work_view.composition.authority_effect,
        workspace_domain::WorkspaceCompositionState::AUTHORITY_EFFECT_NONE
    );
    // Both product surfaces use Attention-projected recommendations (shared path).
    assert!(work_view
        .recommended_actions
        .iter()
        .any(|r| r.kind.starts_with("attention:") || r.kind == "bootstrap"));
    assert!(assistant_view
        .recommended_actions
        .iter()
        .any(|r| r.kind.starts_with("attention:") || r.kind == "bootstrap"));
    // Shared Environment Model is present on both Work and Assistant intelligence views.
    assert_eq!(
        work_view.environment.workspace_id,
        assistant_view.environment.workspace_id
    );
    assert_eq!(
        work_view.composition.workspace_id,
        assistant_view.composition.workspace_id
    );
    assert_eq!(
        work_view.purpose.authority_effect,
        assistant_view.purpose.authority_effect
    );
    assert_eq!(
        work_view.purpose.authority_effect,
        workspace_domain::WorkspacePurposeState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.purpose.workspace_id,
        assistant_view.purpose.workspace_id
    );
    assert_eq!(
        work_view.evolution.authority_effect,
        assistant_view.evolution.authority_effect
    );
    assert_eq!(
        work_view.evolution.authority_effect,
        workspace_domain::WorkspaceEvolutionState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.evolution.workspace_id,
        assistant_view.evolution.workspace_id
    );
    assert_eq!(
        work_view.recommendation_engine.authority_effect,
        assistant_view.recommendation_engine.authority_effect
    );
    assert_eq!(
        work_view.recommendation_engine.authority_effect,
        workspace_domain::WorkspaceRecommendationEngineState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.recommendation_engine.workspace_id,
        assistant_view.recommendation_engine.workspace_id
    );
    assert_eq!(
        work_view.operating_state.authority_effect,
        assistant_view.operating_state.authority_effect
    );
    assert_eq!(
        work_view.operating_state.authority_effect,
        workspace_domain::WorkspaceOperatingState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.operating_state.workspace_id,
        assistant_view.operating_state.workspace_id
    );
    assert_eq!(
        work_view.pattern.authority_effect,
        assistant_view.pattern.authority_effect
    );
    assert_eq!(
        work_view.pattern.authority_effect,
        workspace_domain::WorkspacePatternState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.pattern.workspace_id,
        assistant_view.pattern.workspace_id
    );
    assert_eq!(
        work_view.adaptation.authority_effect,
        assistant_view.adaptation.authority_effect
    );
    assert_eq!(
        work_view.adaptation.authority_effect,
        workspace_domain::WorkspaceAdaptationState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.adaptation.workspace_id,
        assistant_view.adaptation.workspace_id
    );
    assert_eq!(
        work_view.readiness.authority_effect,
        assistant_view.readiness.authority_effect
    );
    assert_eq!(
        work_view.readiness.authority_effect,
        workspace_domain::WorkspaceReadinessState::AUTHORITY_EFFECT_NONE
    );
    assert_eq!(
        work_view.readiness.workspace_id,
        assistant_view.readiness.workspace_id
    );
}

/// CASE 8 — Diagnostic Console uses the same intelligence path.
#[test]
fn case8_diagnostic_uses_same_intelligence_path() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel CASE8");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    CommandHandler::create_memory_entry(
        &kernel,
        local.clone(),
        intent.clone(),
        MemoryType::Workspace,
        "diagnostic_note",
        "Shared diagnostic memory",
        "integrity",
        Some(ws.clone()),
        Some(format!(r#"{{"application_id":"{app_a}"}}"#)),
    )
    .unwrap();

    let product = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    // Operator Console calls the identical CommandHandler entry point.
    let diagnostic = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();

    assert_eq!(product.memory_highlights.len(), diagnostic.memory_highlights.len());
    assert_eq!(product.summary, diagnostic.summary);
    assert_eq!(product.authority_effect, diagnostic.authority_effect);
    assert!(!product.memory_highlights.is_empty());
}

/// CASE 9 — No duplicate authority paths exist for intelligence.
#[test]
fn case9_no_duplicate_authority_paths() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel CASE9");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a],
        Some(ws.clone()),
    )
    .unwrap();
    assert_eq!(workflow.workspace_id.as_deref(), Some(ws.as_str()));

    let before = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let before_exec = before
        .iter()
        .filter(|r| {
            matches!(
                r.command_name.as_deref(),
                Some("LaunchApplication") | Some("DecideApproval") | Some("GrantCapability")
            ) && r.success
        })
        .count();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    // Intelligence may surface pending plans / approvals, but never decides them.
    assert!(
        !state.pending_plans.is_empty() || !state.pending_approvals.is_empty(),
        "expected informational pending surface"
    );

    let after = AuditService::list_recent(&kernel.shared_database(), 200).unwrap();
    let after_exec = after
        .iter()
        .filter(|r| {
            matches!(
                r.command_name.as_deref(),
                Some("LaunchApplication") | Some("DecideApproval") | Some("GrantCapability")
            ) && r.success
        })
        .count();
    assert_eq!(before_exec, after_exec);

    // Intelligence audit must never claim authority.
    let intel_events: Vec<_> = after
        .iter()
        .filter(|r| r.event_type.starts_with("workspace.intelligence"))
        .collect();
    assert!(!intel_events.is_empty());
    for event in intel_events {
        let meta = event.metadata.as_deref().unwrap_or("");
        assert!(
            meta.contains("\"authority_effect\":\"none\"") || meta.contains("authority_effect"),
            "intelligence audit missing authority_effect none: {meta}"
        );
        assert!(!meta.contains("\"authority_effect\":\"granted\""));
        assert!(!meta.contains("\"authority_effect\":\"executed\""));
    }
}

/// CASE 10 — All intelligence audit events remain authority_effect:none.
#[test]
fn case10_intelligence_audits_are_authority_effect_none() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel, "Intel CASE10");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let project = CommandHandler::create_project(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        "Audit Project".into(),
        None,
        None,
    )
    .unwrap();
    CommandHandler::set_active_work(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
        Some(project.id.to_string()),
        None,
    )
    .unwrap();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");

    let records = AuditService::list_recent(&kernel.shared_database(), 80).unwrap();
    let generated: Vec<_> = records
        .iter()
        .filter(|r| r.event_type == "workspace.intelligence.generated")
        .collect();
    assert_eq!(
        generated.len(),
        1,
        "expected single intelligence.generated audit, got {}",
        generated.len()
    );
    let meta = generated[0].metadata.as_deref().unwrap_or("");
    assert!(meta.contains("\"authority_effect\":\"none\""));

    // Fan-out summary/insight/recommendation events must not appear from generate.
    assert!(!records
        .iter()
        .any(|r| r.event_type == "workspace.summary.created"));
    assert!(!records
        .iter()
        .any(|r| r.event_type == "workspace.insight.generated"));
    assert!(!records
        .iter()
        .any(|r| r.event_type == "workspace.recommendation.generated"));
}

#[test]
fn pending_approvals_and_blocked_actions_are_workspace_scoped() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, app_a, _) = seed_workspace(&kernel, "Intel scoped surfaces");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let _ = CommandHandler::submit_ai_plan_simulated(
        &kernel,
        "diagnostic-ai",
        "Prepare my coding workspace",
        vec![app_a.clone()],
    )
    .unwrap();

    let with_pending = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(
        !with_pending.pending_approvals.is_empty(),
        "expected pending approvals linked via workspace apps: {:?}",
        with_pending.pending_approvals
    );

    let workflow = CommandHandler::submit_assistant_goal(
        &kernel,
        "diagnostic-ai-blocked",
        "Prepare my coding workspace",
        vec![app_a],
        Some(ws.clone()),
    )
    .unwrap();
    let confirmed = CommandHandler::confirm_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();
    let plan = CommandHandler::get_orchestrated_ai_plan(
        &kernel,
        confirmed.orchestrated_plan_id.as_ref().unwrap().to_string(),
    )
    .unwrap();
    let approval_id = plan.steps[0]
        .approval_request_id
        .clone()
        .expect("approval");
    CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(DecideApproval::new(
            workspace_domain::PermissionApprovalRequestId::new(approval_id).unwrap(),
            ApprovalDecisionKind::Deny,
        ))
        .unwrap();
    let _ = CommandHandler::resume_assistant_workflow_simulated(
        &kernel,
        workflow.id.to_string(),
    )
    .unwrap();

    let with_blocked = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert!(
        !with_blocked.blocked_actions.is_empty(),
        "expected blocked actions: {:?}",
        with_blocked.blocked_actions
    );
}

#[test]
fn generate_does_not_create_workflow_context_row() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel, "Intel readonly context");
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local.clone(),
        intent.clone(),
        ws.clone(),
    )
    .unwrap();
    assert!(state.workflow_context.active_project_id.is_none());

    // Readonly empty context must not persist a row until set_active_work.
    // After generate-only, a second generate still works without mutation side effects.
    let again = CommandHandler::generate_workspace_intelligence(
        &kernel,
        local,
        intent,
        ws,
    )
    .unwrap();
    assert_eq!(again.authority_effect, "none");
    assert!(again.workflow_context.active_project_id.is_none());
}

/// Phase 5.5 — architecture guards for layers that previously lacked attempt_execute.
#[test]
fn phase55_attempt_execute_guards_reject() {
    for (label, result) in [
        (
            "intelligence",
            CommandHandler::workspace_intelligence_attempt_execute(),
        ),
        ("decision_queue", CommandHandler::decision_queue_attempt_execute()),
        ("attention", CommandHandler::workspace_attention_attempt_execute()),
        ("continuity", CommandHandler::workspace_continuity_attempt_execute()),
        ("activity", CommandHandler::workspace_activity_attempt_execute()),
    ] {
        match result {
            Err(KernelError::WorkspaceIntelligenceValidation { message })
            | Err(KernelError::DecisionQueueValidation { message })
            | Err(KernelError::WorkspaceAttentionValidation { message })
            | Err(KernelError::WorkspaceContinuityValidation { message })
            | Err(KernelError::WorkspaceActivityValidation { message }) => {
                assert!(
                    message.contains("cannot execute") || message.contains("grant"),
                    "{label}: {message}"
                );
            }
            other => panic!("{label}: expected validation CannotExecute, got {other:?}"),
        }
    }
}

/// Phase 5.5 — Intelligence remains authority_effect none with readiness embedded.
#[test]
fn phase55_intelligence_embeds_readiness_without_authority() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let (ws, _, _) = seed_workspace(&kernel, "Intel Phase55");
    let state = CommandHandler::generate_workspace_intelligence(
        &kernel,
        ActorContext::local_user(),
        IntentContext::user_request(),
        ws,
    )
    .unwrap();
    assert_eq!(state.authority_effect, "none");
    assert_eq!(state.readiness.authority_effect, "none");
    assert_eq!(state.adaptation.authority_effect, "none");
    assert_eq!(state.recommendation_engine.authority_effect, "none");
    assert!(!state.readiness.readiness_summary.headline.is_empty());
}
