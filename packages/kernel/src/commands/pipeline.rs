use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::r#trait::{permission_request, MutationCommand, QueryCommand};
use crate::error::Result;
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

    /// Executes a state-changing command through permission + handler logic.
    pub fn execute_mutation<C: MutationCommand>(self, command: C) -> Result<C::Output> {
        let command_name = command.name();
        log::info!(
            "COMMAND: {command_name} (actor={})",
            self.ctx.actor_context.actor.id
        );

        let request = permission_request(
            &self.ctx.actor_context,
            command_name,
            command.permission_subject(),
        );

        if let Err(error) = self.ctx.permission_gate.require(&request) {
            Self::record_command_failure(&self.ctx, command_name, &error);
            return Err(error);
        }

        match command.execute(&self.ctx) {
            Ok(output) => {
                Self::record_command_success(&self.ctx, command_name);
                Ok(output)
            }
            Err(error) => {
                Self::record_command_failure(&self.ctx, command_name, &error);
                Err(error)
            }
        }
    }

    /// Executes a read-only command.
    pub fn execute_query<Q: QueryCommand>(self, command: Q) -> Result<Q::Output> {
        log::info!(
            "COMMAND: {} (actor={})",
            command.name(),
            self.ctx.actor_context.actor.id
        );
        command.execute(&self.ctx)
    }

    fn record_command_success(ctx: &CommandContext<'_>, command_name: &str) {
        if let Err(error) = AuditService::record_command(
            &ctx.database,
            &ctx.actor_context,
            command_name,
            true,
            None,
        ) {
            log::error!("failed to record successful command audit for {command_name}: {error}");
        }
    }

    fn record_command_failure(
        ctx: &CommandContext<'_>,
        command_name: &str,
        error: &crate::error::KernelError,
    ) {
        let metadata = json!({ "error_code": error.to_public().code }).to_string();
        if let Err(audit_error) = AuditService::record_command(
            &ctx.database,
            &ctx.actor_context,
            command_name,
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
    use crate::security::{AllowAllPermissionGate, PermissionDecision, PermissionGate, PermissionRequest};
    use crate::error::KernelError;
    use crate::services::AuditService;
    use workspace_domain::{Actor, ActorContext, ActorType};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct DenyAllPermissionGate;

    impl PermissionGate for DenyAllPermissionGate {
        fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision> {
            Ok(PermissionDecision::Denied {
                reason: format!("denied: {}", request.command),
            })
        }
    }

    struct CountingPermissionGate {
        checks: AtomicUsize,
        last_actor: std::sync::Mutex<Option<Actor>>,
    }

    impl PermissionGate for CountingPermissionGate {
        fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision> {
            self.checks.fetch_add(1, Ordering::SeqCst);
            *self.last_actor.lock().unwrap() = Some(request.actor.clone());
            Ok(PermissionDecision::Allowed)
        }
    }

    fn ready_context<'a>(
        bus: &'a EventBus,
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        gate: &'a dyn PermissionGate,
        actor: ActorContext,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: actor,
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: gate,
        }
    }

    #[test]
    fn mutations_pass_through_pipeline_with_actor() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(&bus, &init, &AllowAllPermissionGate, ActorContext::local_user());

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Pipeline".into()))
            .unwrap();

        assert_eq!(workspace.name, "Pipeline");
    }

    #[test]
    fn permission_request_contains_actor() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let gate = CountingPermissionGate {
            checks: AtomicUsize::new(0),
            last_actor: std::sync::Mutex::new(None),
        };
        let ctx = ready_context(&bus, &init, &gate, ActorContext::local_user());

        CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
            }))
            .unwrap();

        assert_eq!(gate.checks.load(Ordering::SeqCst), 1);
        assert_eq!(
            gate.last_actor.lock().unwrap().as_ref(),
            Some(&Actor::local_user())
        );
    }

    #[test]
    fn denied_mutation_is_blocked() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(&bus, &init, &DenyAllPermissionGate, ActorContext::local_user());

        let error = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Blocked".into()))
            .unwrap_err();

        assert!(matches!(error, KernelError::PermissionDenied(_)));
    }

    #[test]
    fn successful_mutation_attributes_local_user_in_audit() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(&bus, &init, &AllowAllPermissionGate, ActorContext::local_user());

        CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Audited".into()))
            .unwrap();

        let records = AuditService::list_recent(&init.database.shared(), 20).unwrap();
        assert!(records.iter().any(|record| {
            record.command_name.as_deref() == Some("CreateWorkspace")
                && record.success
                && record.actor_type == ActorType::LocalUser
        }));
    }

    #[test]
    fn failed_mutation_generates_failed_command_audit_record() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_context(&bus, &init, &AllowAllPermissionGate, ActorContext::local_user());

        let _ = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate::default()))
            .unwrap_err();

        let records = AuditService::list_recent(&init.database.shared(), 20).unwrap();
        assert!(records.iter().any(|record| {
            record.command_name.as_deref() == Some("UpdateSettings")
                && !record.success
                && record.event_type == "command.failed"
        }));
    }
}
