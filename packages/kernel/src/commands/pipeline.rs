use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::Result;

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
        log::info!("COMMAND: {}", command.name());
        self.ctx
            .permission_gate
            .require(&command.permission_request())?;
        command.execute(&self.ctx)
    }

    /// Executes a read-only command.
    pub fn execute_query<Q: QueryCommand>(self, command: Q) -> Result<Q::Output> {
        log::info!("COMMAND: {}", command.name());
        command.execute(&self.ctx)
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
    }

    impl PermissionGate for CountingPermissionGate {
        fn authorize(&self, _request: &PermissionRequest) -> Result<PermissionDecision> {
            self.checks.fetch_add(1, Ordering::SeqCst);
            Ok(PermissionDecision::Allowed)
        }
    }

    fn ready_context<'a>(
        bus: &'a EventBus,
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        gate: &'a dyn PermissionGate,
    ) -> CommandContext<'a> {
        CommandContext {
            state: &init.state,
            database: init.database.database(),
            event_bus: bus,
            permission_gate: gate,
        }
    }

    #[test]
    fn mutations_pass_through_pipeline() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let gate = AllowAllPermissionGate;
        let ctx = ready_context(&bus, &init, &gate);

        let workspace = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Pipeline".into()))
            .unwrap();

        assert_eq!(workspace.name, "Pipeline");
    }

    #[test]
    fn denied_mutation_is_blocked() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let gate = DenyAllPermissionGate;
        let ctx = ready_context(&bus, &init, &gate);

        let error = CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Blocked".into()))
            .unwrap_err();

        assert!(matches!(error, KernelError::PermissionDenied(_)));
    }

    #[test]
    fn update_settings_checks_permission_gate() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let gate = CountingPermissionGate {
            checks: AtomicUsize::new(0),
        };
        let ctx = ready_context(&bus, &init, &gate);

        CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
            }))
            .unwrap();

        assert_eq!(gate.checks.load(Ordering::SeqCst), 1);
    }
}
