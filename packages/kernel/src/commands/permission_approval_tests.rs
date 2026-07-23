//! Integration tests for permission approval + allow-once grants (Sprint 43–44).

use crate::commands::application::CreateApplication;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::get_permission_approvals::GetPermissionApprovals;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::launch_application::LaunchApplication;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::CommandContext;
use crate::error::KernelError;
use crate::events::EventBus;
use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
use crate::security::{AllowAllPermissionGate, StandardPermissionGate};
use crate::services::AuditService;
use workspace_domain::{
    Actor, ActorContext, ActorType, ApprovalDecisionKind, Capability, CapabilitySet, IntentContext,
    PermissionApprovalStatus,
};

fn ready_ctx<'a>(
    init: &'a crate::commands::initialize::InitializeWorkspaceResult,
    bus: &'a EventBus,
    gate: &'a dyn crate::security::PermissionGate,
    policy: &'a dyn crate::policy::PermissionPolicy,
    capabilities: CapabilitySet,
    actor: ActorContext,
) -> CommandContext<'a> {
    CommandContext {
        actor_context: actor,
        intent_context: IntentContext::user_request(),
        capability_set: capabilities,
        state: &init.state,
        database: init.database.shared(),
        event_bus: bus,
        permission_gate: gate,
        permission_policy: policy,
    }
}

fn seed_launchable_app(
    init: &crate::commands::initialize::InitializeWorkspaceResult,
    bus: &EventBus,
) -> workspace_domain::ApplicationId {
    let workspace = CommandPipeline::new(ready_ctx(
        init,
        bus,
        &AllowAllPermissionGate,
        &AlwaysAllowPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(CreateWorkspace::new("Approval WS".into()))
    .unwrap();

    let app = CommandPipeline::new(ready_ctx(
        init,
        bus,
        &AllowAllPermissionGate,
        &AlwaysAllowPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(CreateApplication::new(
        workspace.id,
        "Notepad".into(),
        None,
        Some("notepad.exe".into()),
    ))
    .unwrap();

    app.id
}

#[test]
fn approval_required_persists_pending_request() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);

    let error = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ActorContext::new(Actor::ai_assistant("ai-1").unwrap()),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired, got {error:?}");
    };
    assert!(!approval_request_id.is_empty());

    let approvals = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_query(GetPermissionApprovals::new(Some(20)))
    .unwrap();

    assert!(approvals.iter().any(|item| {
        item.id.as_str() == approval_request_id
            && item.status == PermissionApprovalStatus::Pending
            && item.requesting_actor_type == ActorType::AIAssistant
            && item.command_name == "LaunchApplication"
    }));
}

#[test]
fn allow_once_then_retry_succeeds_and_consumes_grant() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);
    let ai = ActorContext::new(Actor::ai_assistant("ai-approve").unwrap());

    let error = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai.clone(),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id.clone()))
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id).unwrap();

    let decided = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(DecideApproval::new(
        request_id,
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    assert_eq!(decided.request.status, PermissionApprovalStatus::Approved);
    assert!(decided.grant.is_some());

    let launched = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai.clone(),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id.clone()))
    .unwrap();

    assert_eq!(launched.name, "Notepad");

    // Grant is consumed — second retry without a new approval must fail closed.
    let second = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai,
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    assert!(matches!(second, KernelError::ApprovalRequired { .. }));

    let records = AuditService::list_recent(&init.database.shared(), 50).unwrap();
    assert!(records.iter().any(|r| {
        r.command_name.as_deref() == Some("DecideApproval") && r.success
    }));
}

#[test]
fn deny_keeps_execution_blocked() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);
    let ai = ActorContext::new(Actor::ai_assistant("ai-deny").unwrap());

    let error = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai.clone(),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id.clone()))
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id).unwrap();

    let decided = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(DecideApproval::new(request_id, ApprovalDecisionKind::Deny))
    .unwrap();

    assert_eq!(decided.request.status, PermissionApprovalStatus::Denied);
    assert!(decided.grant.is_none());

    let blocked = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai,
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    assert!(matches!(blocked, KernelError::ApprovalRequired { .. }));
}

#[test]
fn non_local_user_cannot_decide_approval() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);
    let ai = ActorContext::new(Actor::ai_assistant("ai-self-approve").unwrap());

    // Seed a pending request under production gate/policy.
    let error = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai.clone(),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id).unwrap();

    // Bypass gate so the LocalUser-only decide check is exercised (defense in depth).
    let denied = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &AllowAllPermissionGate,
        &AlwaysAllowPolicy,
        CapabilitySet::new().with_capability(&Capability::audit_write()),
        ai,
    ))
    .execute_mutation(DecideApproval::new(
        request_id,
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap_err();

    assert!(matches!(denied, KernelError::PermissionDenied(_)));
}

#[test]
fn double_decide_fails_closed() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);
    let ai = ActorContext::new(Actor::ai_assistant("ai-double").unwrap());

    let error = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ai,
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    let KernelError::ApprovalRequired {
        approval_request_id,
        ..
    } = error
    else {
        panic!("expected ApprovalRequired");
    };

    let request_id =
        workspace_domain::PermissionApprovalRequestId::new(approval_request_id).unwrap();

    CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(DecideApproval::new(
        request_id.clone(),
        ApprovalDecisionKind::AllowOnce,
    ))
    .unwrap();

    let second = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::local_user_standard(),
        ActorContext::local_user(),
    ))
    .execute_mutation(DecideApproval::new(
        request_id,
        ApprovalDecisionKind::Deny,
    ))
    .unwrap_err();

    assert!(matches!(
        second,
        KernelError::PermissionApprovalValidation { .. }
    ));
}

#[test]
fn permission_decision_audit_includes_actor_intent_and_resource() {
    let bus = EventBus::new();
    let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
    let app_id = seed_launchable_app(&init, &bus);

    let _ = CommandPipeline::new(ready_ctx(
        &init,
        &bus,
        &StandardPermissionGate,
        &CapabilityBoundPolicy,
        CapabilitySet::new(),
        ActorContext::new(Actor::ai_assistant("ai-audit").unwrap()),
    ))
    .execute_mutation(LaunchApplication::simulated(app_id))
    .unwrap_err();

    let records = AuditService::list_recent(&init.database.shared(), 50).unwrap();
    let decision = records
        .iter()
        .find(|r| r.event_type == "permission.approval_required")
        .expect("permission.approval_required audit missing");

    assert_eq!(decision.actor_type, ActorType::AIAssistant);
    assert_eq!(decision.command_name.as_deref(), Some("LaunchApplication"));
    assert!(!decision.success);
    let metadata = decision.metadata.as_deref().unwrap_or("");
    assert!(metadata.contains("approval_required"));
    assert!(metadata.contains("LaunchApplication"));
    assert!(metadata.contains("AIAssistant"));
}
