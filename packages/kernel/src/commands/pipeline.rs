use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::r#trait::{permission_request, policy_context, require_policy, MutationCommand, QueryCommand};
use crate::error::Result;
use crate::policy::{DefaultPolicyEvaluator, PolicyEvaluator};
use crate::services::AuditService;

/// Uniform execution path for kernel commands.
pub struct CommandPipeline<'a> {
    ctx: CommandContext<'a>,
}

impl<'a> CommandPipeline<'a> {
    pub fn new(ctx: CommandContext<'a>) -> Self {
        Self { ctx }
    }

    pub fn context(&self) -> &CommandContext<'a> {
        &self.ctx
    }

    /// Executes a state-changing command through policy, permission, and handler logic.
    pub fn execute_mutation<C: MutationCommand>(self, command: C) -> Result<C::Output> {
        let command_name = command.name();
        log::info!(
            "COMMAND: {command_name} (actor={}, intent={:?})",
            self.ctx.actor_context.actor.id,
            self.ctx.intent_context.intent.intent_type
        );

        let capability = command.required_capability();
        let request = permission_request(
            &self.ctx.actor_context,
            &self.ctx.intent_context,
            command_name,
            command.permission_subject(),
            capability.clone(),
        );

        let policy_result = DefaultPolicyEvaluator.evaluate(
            self.ctx.permission_policy,
            &policy_context(&request),
        )?;

        if let Err(error) = require_policy(policy_result) {
            Self::record_command_failure(&self.ctx, command_name, &capability, &error);
            return Err(error);
        }

        if let Err(error) = self.ctx.permission_gate.require(&request) {
            Self::record_command_failure(&self.ctx, command_name, &capability, &error);
            return Err(error);
        }

        match command.execute(&self.ctx) {
            Ok(output) => {
                Self::record_command_success(&self.ctx, command_name, &capability);
                Ok(output)
            }
            Err(error) => {
                Self::record_command_failure(&self.ctx, command_name, &capability, &error);
                Err(error)
            }
        }
    }

    /// Executes a read-only command.
    pub fn execute_query<Q: QueryCommand>(self, command: Q) -> Result<Q::Output> {
        log::info!(
            "COMMAND: {} (actor={}, intent={:?})",
            command.name(),
            self.ctx.actor_context.actor.id,
            self.ctx.intent_context.intent.intent_type
        );
        command.execute(&self.ctx)
    }

    fn record_command_success(
        ctx: &CommandContext<'_>,
        command_name: &str,
        capability: &workspace_domain::Capability,
    ) {
        if let Err(error) = AuditService::record_command(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            command_name,
            capability,
            true,
            None,
        ) {
            log::error!("failed to record successful command audit for {command_name}: {error}");
        }
    }

    fn record_command_failure(
        ctx: &CommandContext<'_>,
        command_name: &str,
        capability: &workspace_domain::Capability,
        error: &crate::error::KernelError,
    ) {
        let metadata = json!({ "error_code": error.to_public().code }).to_string();
        if let Err(audit_error) = AuditService::record_command(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            command_name,
            capability,
            false,
            Some(metadata),
        ) {
            log::error!(
                "failed to record failed command audit for {command_name}: {audit_error}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::update_settings::UpdateSettings;
    use crate::config::SettingsUpdate;
    use crate::events::EventBus;
    use crate::policy::{AlwaysAllowPolicy, PermissionPolicy, PolicyContext, PolicyResult};
    use crate::security::{
        AllowAllPermissionGate, PermissionDecision, PermissionGate, PermissionRequest,
    };
    use crate::error::KernelError;
    use crate::services::AuditService;
    use workspace_domain::{
        Actor, ActorContext, ActorType, Capability, IntentContext, IntentType,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct DenyAllPermissionGate;

    impl PermissionGate for DenyAllPermissionGate {
        fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision> {
            Ok(PermissionDecision::Denied {
                reason: format!("denied: {}", request.command),
            })
        }
    }

    struct DenyAllPolicy;

    impl PermissionPolicy for DenyAllPolicy {
        fn evaluate(&self, context: &PolicyContext) -> Result<PolicyResult> {
            Ok(PolicyResult::deny(format!("policy denied: {}", context.command)))
        }
    }

    struct CountingPermissionGate {
        checks: AtomicUsize,
        last_request: std::sync::Mutex<Option<PermissionRequest>>,
    }

    impl PermissionGate for CountingPermissionGate {
        fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision> {
            self.checks.fetch_add(1, Ordering::SeqCst);
            *self.last_request.lock().unwrap() = Some(request.clone());
            Ok(PermissionDecision::Allowed)
        }
    }

    fn ready_context<'a>(
        bus: &'a EventBus,
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        gate: &'a dyn PermissionGate,
        policy: &'a dyn PermissionPolicy,
        actor: ActorContext,
        intent: IntentContext,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: actor,
            intent_context: intent,
            capability_set: workspace_domain::CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: gate,
            permission_policy: policy,
        }
    }

    #[test]
    fn mutations_pass_through_pipeline_with_actor_and_intent() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(
            &bus,
            &init,
            &AllowAllPermissionGate,
            &AlwaysAllowPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Pipeline".into()))
            .unwrap();

        assert_eq!(workspace.name, "Pipeline");
    }

    #[test]
    fn permission_request_contains_actor_intent_and_capability() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let gate = CountingPermissionGate {
            checks: AtomicUsize::new(0),
            last_request: std::sync::Mutex::new(None),
        };
        let ctx = ready_context(
            &bus,
            &init,
            &gate,
            &AlwaysAllowPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
            }))
            .unwrap();

        assert_eq!(gate.checks.load(Ordering::SeqCst), 1);
        let request = gate.last_request.lock().unwrap().clone().unwrap();
        assert_eq!(request.actor, Actor::local_user());
        assert_eq!(request.intent.intent_type, IntentType::UserRequest);
        assert_eq!(request.capability, Capability::settings_write());
    }

    #[test]
    fn denied_mutation_is_blocked_by_gate() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(
            &bus,
            &init,
            &DenyAllPermissionGate,
            &AlwaysAllowPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        let error = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Blocked".into()))
            .unwrap_err();

        assert!(matches!(error, KernelError::PermissionDenied(_)));
    }

    #[test]
    fn denied_mutation_is_blocked_by_policy() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(
            &bus,
            &init,
            &AllowAllPermissionGate,
            &DenyAllPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        let error = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Blocked".into()))
            .unwrap_err();

        assert!(matches!(error, KernelError::PermissionDenied(_)));
    }

    #[test]
    fn successful_mutation_attributes_intent_and_capability_in_audit() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(
            &bus,
            &init,
            &AllowAllPermissionGate,
            &AlwaysAllowPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Audited".into()))
            .unwrap();

        let records = AuditService::list_recent(&init.database.shared(), 20).unwrap();
        assert!(records.iter().any(|record| {
            record.command_name.as_deref() == Some("CreateWorkspace")
                && record.success
                && record.actor_type == ActorType::LocalUser
                && record.intent_type == Some(IntentType::UserRequest)
                && record.capability.as_deref() == Some("workspace.write")
        }));
    }

    #[test]
    fn failed_mutation_generates_failed_command_audit_record() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(
            &bus,
            &init,
            &AllowAllPermissionGate,
            &AlwaysAllowPolicy,
            ActorContext::local_user(),
            IntentContext::user_request(),
        );

        let _ = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate::default()))
            .unwrap_err();

        let records = AuditService::list_recent(&init.database.shared(), 20).unwrap();
        assert!(records.iter().any(|record| {
            record.command_name.as_deref() == Some("UpdateSettings")
                && !record.success
                && record.event_type == "command.failed"
                && record.intent_type == Some(IntentType::UserRequest)
        }));
    }
}
