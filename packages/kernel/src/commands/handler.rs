use std::path::Path;

use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_audit_history::GetAuditHistory;
use crate::commands::get_workspace::GetWorkspace;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::pipeline::CommandPipeline;
use crate::commands::update_settings::UpdateSettings;
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceShutdown};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::ConfigurationService;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, AuditEvent, Capability, Intent, IntentContext, Workspace, WorkspaceId,
};

/// Executes kernel commands and coordinates services + events.
pub struct CommandHandler;

impl CommandHandler {
    pub fn initialize_workspace(
        kernel: &mut WorkspaceKernel,
        db_path: impl AsRef<Path>,
    ) -> Result<()> {
        let result = InitializeWorkspace::at_path(db_path).execute(kernel.event_bus())?;
        kernel.apply_runtime(result.state, result.database, result.services);
        Ok(())
    }

    pub fn initialize_workspace_in_memory(kernel: &mut WorkspaceKernel) -> Result<()> {
        let result = InitializeWorkspace::in_memory().execute(kernel.event_bus())?;
        kernel.apply_runtime(result.state, result.database, result.services);
        Ok(())
    }

    pub fn get_settings(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<WorkspaceSettings> {
        if !kernel.state().is_ready() {
            return Err(KernelError::NotReady);
        }
        let _ = (actor, intent);
        kernel.database.with_database(ConfigurationService::load)
    }

    pub fn update_settings(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        update: SettingsUpdate,
    ) -> Result<WorkspaceSettings> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(UpdateSettings::new(update))
    }

    pub fn create_workspace(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        name: String,
    ) -> Result<Workspace> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(CreateWorkspace::new(name))
    }

    pub fn get_workspace(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<Workspace> {
        let workspace_id = WorkspaceId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspace::new(workspace_id))
    }

    pub fn get_audit_history(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetAuditHistory::new(limit))
    }

    pub fn shutdown(kernel: &mut WorkspaceKernel) {
        let request = PermissionRequest {
            actor: Actor::system(),
            intent: Intent::system_shutdown(),
            capability: Capability::system_shutdown(),
            command: "ShutdownWorkspace",
            subject: PermissionSubject::System,
        };

        if let Err(error) = kernel.permission_gate().require(&request) {
            log::error!("ShutdownWorkspace denied: {error}");
            return;
        }

        log::info!("COMMAND: ShutdownWorkspace (actor=system, intent=SystemShutdown)");
        kernel.transition_lifecycle(LifecycleState::ShuttingDown);
        kernel.event_bus().publish(DomainEvent::WorkspaceShutdown(WorkspaceShutdown {
            actor: Some(ActorContext::system()),
            intent: Some(IntentContext::system_shutdown()),
            capability: Some(Capability::system_shutdown()),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;

    #[test]
    fn command_handler_runs_initialize_and_update() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();

        let settings = CommandHandler::update_settings(
            &kernel,
            ActorContext::local_user(),
            IntentContext::user_request(),
            SettingsUpdate {
                theme: Some("light".into()),
                first_run: Some(false),
            },
        )
        .unwrap();

        assert_eq!(settings.theme, "light");
        assert!(!settings.first_run);
    }

    #[test]
    fn all_mutations_route_through_pipeline_or_handler_boundary() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let workspace =
            CommandHandler::create_workspace(&kernel, actor.clone(), intent.clone(), "Boundary".into())
                .unwrap();
        assert_eq!(workspace.name, "Boundary");

        let settings = CommandHandler::update_settings(
            &kernel,
            actor,
            intent,
            SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
            },
        )
        .unwrap();
        assert_eq!(settings.theme, "dark");
    }
}
