use crate::commands::context::CommandContext;
use crate::commands::r#trait::MutationCommand;
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, SettingsChanged};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::ConfigurationService;
use workspace_domain::Capability;

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

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<WorkspaceSettings> {
        Self::ensure_ready(ctx.state)?;
        Self::validate(&self.update)?;

        let settings = ctx.with_database(|db| {
            ConfigurationService::update(db, self.update.clone()).map_err(|error| {
                log::error!("UpdateSettings command failed: {error}");
                error
            })
        })?;

        ctx.event_bus.publish(DomainEvent::SettingsChanged(SettingsChanged {
            theme: settings.theme.clone(),
            first_run: settings.first_run,
            settings_version: settings.settings_version,
            actor: Some(ctx.actor_context.clone()),
            intent: Some(ctx.intent_context.clone()),
            capability: Some(Capability::settings_write()),
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
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use std::sync::{Arc, Mutex};
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

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
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        let settings = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
                active_workspace_id: None,
                personalization_enabled: None,
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
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        let error = CommandPipeline::new(ctx)
            .execute_mutation(UpdateSettings::new(SettingsUpdate::default()))
            .unwrap_err();

        assert!(matches!(error, KernelError::InvalidSettings(_)));
    }
}
