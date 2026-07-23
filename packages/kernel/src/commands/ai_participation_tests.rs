//! AI actor foundation + governed participation tests (Sprints 46–47).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::zone::{CreateZone, DeleteZone};
use crate::commands::{CommandContext, CommandHandler};
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
use crate::security::{AllowAllPermissionGate, StandardPermissionGate};
use crate::services::AuditService;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ActorType, ApprovalDecisionKind, CapabilitySet, IntentContext,
    IntentType, PermissionApprovalStatus,
};

fn seed_app(kernel: &WorkspaceKernel) -> workspace_domain::ApplicationId {
    let local = ActorContext::local_user();
    let intent = IntentContext::user_request();

    let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
        .execute_mutation(CreateWorkspace::new("AI Actor WS".into()))
        .unwrap();

    CommandPipeline::new(kernel.command_context(local, intent))
        .execute_mutation(CreateApplication::new(
            workspace.id,
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        ))
        .unwrap()
        .id
}

#[test]
fn case1_ai_launch_requires_approval_and_does_not_execute() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let app_id = seed_app(&kernel);

    let error = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        "ai-case-1",
        app_id.to_string(),
        Some("Propose Notepad launch".into()),
    )
    .unwrap_err();

    assert!(matches!(error, KernelError::ApprovalRequired { .. }));

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    assert!(records.iter().any(|r| {
        r.event_type == "permission.approval_required"
            && r.actor_type == ActorType::AIAssistant
            && r.command_name.as_deref() == Some("LaunchApplication")
            && !r.success
    }));
    assert!(!records.iter().any(|r| {
        r.command_name.as_deref() == Some("LaunchApplication")
            && r.event_type.starts_with("command.")
            && r.success
            && r.actor_type == ActorType::AIAssistant
    }));
}

#[test]
fn case2_ai_unauthorized_mutation_requires_approval() {
    let bus = EventBus::new();
    let init = crate::commands::initialize::InitializeWorkspace::in_memory()
        .execute(&bus)
        .unwrap();

    let workspace = CommandPipeline::new(CommandContext {
        actor_context: ActorContext::local_user(),
        intent_context: IntentContext::user_request(),
        capability_set: CapabilitySet::local_user_standard(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: &bus,
        permission_gate: &AllowAllPermissionGate,
        permission_policy: &AlwaysAllowPolicy,
    })
    .execute_mutation(CreateWorkspace::new("AI Deny WS".into()))
    .unwrap();

    let zone = CommandPipeline::new(CommandContext {
        actor_context: ActorContext::local_user(),
        intent_context: IntentContext::user_request(),
        capability_set: CapabilitySet::local_user_standard(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: &bus,
        permission_gate: &AllowAllPermissionGate,
        permission_policy: &AlwaysAllowPolicy,
    })
    .execute_mutation(CreateZone::new(workspace.id, "Z1".into(), None))
    .unwrap();

    let ai = ActorContext::new(Actor::ai_assistant("ai-case-2").unwrap());
    let error = CommandPipeline::new(CommandContext {
        actor_context: ai,
        intent_context: IntentContext::ai_suggestion(),
        capability_set: CapabilitySet::new(),
        state: &init.state,
        database: init.database.shared(),
        event_bus: &bus,
        permission_gate: &StandardPermissionGate,
        permission_policy: &CapabilityBoundPolicy,
    })
    .execute_mutation(DeleteZone::new(zone.id))
    .unwrap_err();

    assert!(matches!(
        error,
        KernelError::ApprovalRequired { .. } | KernelError::PermissionDenied(_)
    ));
}

#[test]
fn case3_human_approved_ai_request_executes_and_consumes_grant() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let app_id = seed_app(&kernel);
    let ai_id = "ai-case-3";

    let error = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        ai_id,
        app_id.to_string(),
        Some("Approved path validation".into()),
    )
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired, got {error:?}");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id).unwrap();

    let decided = CommandPipeline::new(kernel.command_context(
        ActorContext::local_user(),
        IntentContext::user_request(),
    ))
    .execute_mutation(DecideApproval::new(
        request_id,
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    assert_eq!(decided.request.status, PermissionApprovalStatus::Approved);
    assert_eq!(
        decided.request.intent_type,
        IntentType::AISuggestion
    );

    let launched = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        ai_id,
        app_id.to_string(),
        None,
    )
    .unwrap();
    assert_eq!(launched.name, "Notepad");

    let retry = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        ai_id,
        app_id.to_string(),
        None,
    )
    .unwrap_err();
    assert!(matches!(retry, KernelError::ApprovalRequired { .. }));
}

#[test]
fn case4_ai_participation_requires_aisuggestion_intent() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let app_id = seed_app(&kernel);

    let request = crate::services::AiParticipationService::propose_application_launch(
        "ai-case-4",
        &app_id,
        None,
    )
    .unwrap();

    // Privileged launch/approval services are crate-internal (Sprint 45 seal).
    // AI participation rejects non-AISuggestion motivation before pipeline execute.
    let ctx = kernel.command_context(
        ActorContext::new(Actor::ai_assistant("ai-case-4").unwrap()),
        IntentContext::user_request(),
    );
    let error = crate::services::AiParticipationService::ensure_ai_submission_context(
        &ctx, &request,
    )
    .unwrap_err();

    assert!(matches!(error, KernelError::AiRequestValidation { .. }));
}

#[test]
fn ai_request_audit_includes_actor_intent_command_and_decision() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let app_id = seed_app(&kernel);

    let _ = CommandHandler::submit_ai_application_launch_simulated(
        &kernel,
        "ai-audit",
        app_id.to_string(),
        Some("Audit field check".into()),
    );

    let records = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    let decision = records
        .iter()
        .find(|r| r.event_type == "permission.approval_required")
        .expect("permission decision audit missing");

    assert_eq!(decision.actor_type, ActorType::AIAssistant);
    assert_eq!(decision.intent_type, Some(IntentType::AISuggestion));
    assert_eq!(decision.command_name.as_deref(), Some("LaunchApplication"));
    let metadata = decision.metadata.as_deref().unwrap_or("");
    assert!(metadata.contains("approval_required"));
    assert!(metadata.contains("AIAssistant"));
}
