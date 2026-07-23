use workspace_database::Database;

use crate::commands::Command;
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, SettingsChanged};
use crate::events::EventBus;
use crate::lifecycle::LifecycleState;
use crate::services::ConfigurationService;
use crate::state::WorkspaceState;

/// Updates persisted workspace settings through the configuration service.
pub struct UpdateSettings {
    pub update: SettingsUpdate,
}

impl Command for UpdateSettings {
    fn name(&self) -> &'static str {
        "UpdateSettings"
    }
}

impl UpdateSettings {
    pub fn new(update: SettingsUpdate) -> Self {
        Self { update }
    }

    pub fn execute(
        self,
        state: &WorkspaceState,
        database: &Database,
        event_bus: &EventBus,
    ) -> Result<WorkspaceSettings> {
        log::info!("COMMAND: UpdateSettings");

        Self::ensure_ready(state)?;
        Self::validate(&self.update)?;

        let settings = ConfigurationService::update(database, self.update).map_err(|error| {
            log::error!("UpdateSettings command failed: {error}");
            error
        })?;

        event_bus.publish(DomainEvent::SettingsChanged(SettingsChanged {
            theme: settings.theme.clone(),
            first_run: settings.first_run,
            settings_version: settings.settings_version,
        }));

        Ok(settings)
    }

    fn ensure_ready(state: &WorkspaceState) -> Result<()> {
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
    use crate::commands::initialize::{InitializeWorkspace, InitializeWorkspaceResult};
    use crate::events::EventBus;
    use std::sync::{Arc, Mutex};

    fn initialized_context() -> (EventBus, InitializeWorkspaceResult) {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        (bus, init)
    }

    #[test]
    fn executes_successfully_and_emits_event() {
        let (bus, init) = initialized_context();
        let event_name = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&event_name);

        bus.subscribe(move |event| {
            *captured.lock().unwrap() = event.name().to_string();
        });

        let settings = UpdateSettings::new(SettingsUpdate {
            theme: Some("dark".into()),
            first_run: None,
        })
        .execute(&init.state, init.database.database(), &bus)
        .unwrap();

        assert_eq!(settings.theme, "dark");
        assert_eq!(*event_name.lock().unwrap(), "system.settings.changed");
    }

    #[test]
    fn rejects_empty_update() {
        let (bus, init) = initialized_context();

        let error = UpdateSettings::new(SettingsUpdate::default())
            .execute(&init.state, init.database.database(), &bus)
            .unwrap_err();

        assert!(matches!(error, KernelError::InvalidSettings(_)));
    }
}
