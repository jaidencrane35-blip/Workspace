use std::path::Path;

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::application::{CreateApplication, DeleteApplication, GetApplication};
use crate::commands::execute_intent_request::ExecuteIntentRequest;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::get_actor_capabilities::GetActorCapabilities;
use crate::commands::get_audit_history::GetAuditHistory;
use crate::commands::get_observations::GetObservations;
use crate::commands::get_suggestion_lifecycle::GetSuggestionLifecycle;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::get_workspace::GetWorkspace;
use crate::commands::get_workspace_context::GetWorkspaceContext;
use crate::commands::get_workspace_metrics::GetWorkspaceMetrics;
use crate::commands::get_workspace_snapshot::GetWorkspaceSnapshot;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::layout::{
    CreateLayout, DeleteLayout, GetLayout, GetLayoutSnapshot, ResetLayout, UpdateLayout,
};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::update_settings::UpdateSettings;
use crate::commands::widget::{CreateWidget, DeleteWidget, GetWidget};
use crate::commands::zone::{CreateZone, DeleteZone, GetZone};
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceShutdown};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::ConfigurationService;
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, ApplicationId, ApplicationReference, AuditEvent, Capability, Intent,
    IntentContext, Layout, LayoutId, LayoutMetadata, LayoutNode, LayoutSnapshot, Observation,
    Suggestion, SuggestionIntentRequest, SuggestionLifecycleRecord, IntentExecutionRequest, WidgetId, WidgetReference, Workspace, WorkspaceContext, WorkspaceId,
    WorkspaceMetrics, WorkspaceSnapshot, CapabilityDiscovery, Zone, ZoneId,
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

    pub fn create_zone(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        position_metadata: Option<String>,
    ) -> Result<Zone> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateZone::new(workspace_id, name, position_metadata),
        )
    }

    pub fn delete_zone(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<()> {
        let zone_id = ZoneId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteZone::new(zone_id))
    }

    pub fn get_zone(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<Zone> {
        let zone_id = ZoneId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetZone::new(zone_id))
    }

    pub fn create_application(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        identifier: Option<String>,
    ) -> Result<ApplicationReference> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateApplication::new(workspace_id, name, identifier),
        )
    }

    pub fn delete_application(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<()> {
        let application_id = ApplicationId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteApplication::new(application_id))
    }

    pub fn get_application(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<ApplicationReference> {
        let application_id = ApplicationId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetApplication::new(application_id))
    }

    pub fn create_widget(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        widget_type: Option<String>,
    ) -> Result<WidgetReference> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateWidget::new(workspace_id, name, widget_type),
        )
    }

    pub fn delete_widget(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<()> {
        let widget_id = WidgetId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteWidget::new(widget_id))
    }

    pub fn get_widget(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<WidgetReference> {
        let widget_id = WidgetId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWidget::new(widget_id))
    }

    pub fn create_layout(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<Layout> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(CreateLayout::new(workspace_id))
    }

    pub fn update_layout(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        layout_id: String,
        viewport: workspace_domain::Viewport,
        nodes: Vec<LayoutNode>,
        metadata: Option<LayoutMetadata>,
    ) -> Result<Layout> {
        let layout_id = LayoutId::new(layout_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            UpdateLayout::new(layout_id, viewport, nodes, metadata),
        )
    }

    pub fn delete_layout(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        layout_id: String,
        workspace_id: String,
    ) -> Result<()> {
        let layout_id = LayoutId::new(layout_id).map_err(KernelError::Domain)?;
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteLayout::new(layout_id, workspace_id))
    }

    pub fn reset_layout(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        layout_id: String,
    ) -> Result<Layout> {
        let layout_id = LayoutId::new(layout_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(ResetLayout::new(layout_id))
    }

    pub fn get_layout(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<Layout> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetLayout::new(workspace_id))
    }

    pub fn get_layout_snapshot(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        layout_id: String,
    ) -> Result<LayoutSnapshot> {
        let layout_id = LayoutId::new(layout_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetLayoutSnapshot::new(layout_id))
    }

    pub fn get_workspace_snapshot(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceSnapshot> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceSnapshot::new(workspace_id))
    }

    pub fn get_actor_capabilities(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<CapabilityDiscovery> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetActorCapabilities)
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

    pub fn get_observations(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<Observation>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetObservations::new(limit))
    }

    pub fn get_workspace_metrics(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<WorkspaceMetrics> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceMetrics::new(limit))
    }

    pub fn get_workspace_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<WorkspaceContext> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceContext::new(workspace_id, limit))
    }

    pub fn get_suggestions(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<Suggestion>> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetSuggestions::new(workspace_id, limit))
    }

    pub fn accept_suggestion(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        suggestion_id: String,
    ) -> Result<Suggestion> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(AcceptSuggestion::new(workspace_id, suggestion_id))
    }

    pub fn reject_suggestion(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        suggestion_id: String,
    ) -> Result<Suggestion> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(RejectSuggestion::new(workspace_id, suggestion_id))
    }

    pub fn get_suggestion_lifecycle(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<SuggestionLifecycleRecord>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetSuggestionLifecycle::new(limit))
    }

    pub fn create_suggestion_intent_request(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        suggestion_id: String,
    ) -> Result<SuggestionIntentRequest> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateSuggestionIntentRequest::new(workspace_id, suggestion_id),
        )
    }

    pub fn execute_intent_request(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        suggestion_id: String,
    ) -> Result<IntentExecutionRequest> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            ExecuteIntentRequest::new(workspace_id, suggestion_id),
        )
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

        let zone = CommandHandler::create_zone(
            &kernel,
            actor.clone(),
            intent.clone(),
            workspace.id.to_string(),
            "Primary".into(),
            None,
        )
        .unwrap();
        assert_eq!(zone.name, "Primary");

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
