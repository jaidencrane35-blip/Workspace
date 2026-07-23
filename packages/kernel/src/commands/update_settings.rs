use crate::commands::context::CommandContext;
use crate::commands::r#trait::{Command, MutationCommand};
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, SettingsChanged};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::ConfigurationService;

/// Updates persisted workspace settings through the configuration service.
pub struct UpdateSettings {
    pub update: SettingsUpdate,
}

impl crate::commands::Command for UpdateSettings {
    fn name(&self) -> &'static str {
        "UpdateSettings"
    }
}

impl MutationCommand for UpdateSettings {
    type Output = WorkspaceSettings;

    fn permission_request(&self) -> PermissionRequest {
        PermissionRequest {
            command: self.name(),
            subject: PermissionSubject::Settings,
        }
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceSettings> {
        Self::ensure_ready(ctx.state)?;
        Self::validate(&self.update)?;

        let settings = ConfigurationService::update(ctx.database, self.update).map_err(|error| {
            log::error!("UpdateSettings command failed: {error}");
            error
        })?;

        ctx.event_bus.publish(DomainEvent::SettingsChanged(SettingsChanged {
            theme: settings.theme.clone(),
            first_run: settings.first_run,
            settings_version: settings.settings_version,
        }));

        Ok(settings)
    }
}

impl UpdateSettings {
    pub fn new(update: SettingsUpdate) -> Self {
        Self { update }
    }

    fn ensure_ready(state: &crate::state::WorkspaceState) -> Result<()> {
        if state.lifecycle == LifecycleState::Ready {
            Ok(())
        } else {
            Err(KernelError::NotReady)
        }
    }

    fn validate(update: &SettingsUpdate) -> Result<()> {
        if update.theme.is_none() && update.first_run.is_none() {
            return Err(KernelError::InvalidSettings(
                "at least one setting must be provided".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::context::CommandContext;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::security::AllowAllPermissionGate;
    use std::sync::{Arc, Mutex};

    #[test]
    fn executes_successfully_and_emits_event() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let event_name = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&event_name);

        bus.subscribe(move |event| {
            *captured.lock().unwrap() = event.name().to_string();
        });

        let ctx = CommandContext {
            state: &init.state,
            database: init.database.database(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        };

        let settings = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
            }))
            .unwrap();

        assert_eq!(settings.theme, "dark");
        assert_eq!(*event_name.lock().unwrap(), "system.settings.changed");
    }

    #[test]
    fn rejects_empty_update() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            state: &init.state,
            database: init.database.database(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        };

        let error = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate::default()))
            .unwrap_err();

        assert!(matches!(error, KernelError::InvalidSettings(_)));
    }
}
