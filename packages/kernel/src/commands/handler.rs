use std::path::Path;

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::application::{CreateApplication, DeleteApplication, GetApplication};
use crate::commands::execute_intent_request::ExecuteIntentRequest;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::get_actor_capabilities::GetActorCapabilities;
use crate::commands::get_audit_history::GetAuditHistory;
use crate::commands::get_permission_approvals::GetPermissionApprovals;
use crate::commands::get_desktop_windows::GetDesktopWindows;
use crate::commands::get_execution_outcomes::GetExecutionOutcomes;
use crate::commands::get_execution_state::GetExecutionState;
use crate::commands::get_execution_states::GetExecutionStates;
use crate::commands::get_observations::GetObservations;
use crate::commands::get_suggestion_lifecycle::GetSuggestionLifecycle;
use crate::commands::get_suggestions::GetSuggestions;
use crate::commands::get_workspace::GetWorkspace;
use crate::commands::get_workspace_context::GetWorkspaceContext;
use crate::commands::get_workspace_metrics::GetWorkspaceMetrics;
use crate::commands::get_workspace_snapshot::GetWorkspaceSnapshot;
use crate::commands::initialize::InitializeWorkspace;
use crate::commands::launch_application::LaunchApplication;
use crate::commands::layout::{
    CreateLayout, DeleteLayout, GetLayout, GetLayoutSnapshot, ResetLayout, UpdateLayout,
};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::request_execution_cancellation::RequestExecutionCancellation;
use crate::commands::update_settings::UpdateSettings;
use crate::commands::widget::{CreateWidget, DeleteWidget, GetWidget};
use crate::commands::zone::{CreateZone, DeleteZone, GetZone};
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceShutdown};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::{AiParticipationService, AiPlanningService, ConfigurationService};
use crate::WorkspaceKernel;
use workspace_domain::{
    Actor, ActorContext, AiPlan, AiPlanSubmissionResult, AiProposalAuthorityOutcome,
    AiProposalSubmission, ApplicationId, ApplicationReference, AuditEvent, Capability, Intent,
    IntentContext, Layout, LayoutId, LayoutMetadata, LayoutNode, LayoutSnapshot, Observation,
    Suggestion, SuggestionIntentRequest, SuggestionLifecycleRecord, IntentExecutionRequest, ExecutionOutcome, ExecutionReconciliation, CancellationRequest, WidgetId, WidgetReference, Workspace, WorkspaceContext, WorkspaceId,
    WorkspaceMetrics, WorkspaceSnapshot, CapabilityDiscovery, Zone, ZoneId,
};
use workspace_windows_integration::DesktopWindowSnapshot;

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
        executable_path: Option<String>,
    ) -> Result<ApplicationReference> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateApplication::new(workspace_id, name, identifier, executable_path),
        )
    }

    pub fn launch_application(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: String,
    ) -> Result<workspace_domain::ApplicationLaunchResult> {
        let application_id = ApplicationId::new(id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(LaunchApplication::new(application_id))
    }

    /// AI participation entry: propose launch → CommandPipeline → Permission Gateway.
    ///
    /// Does not call launch services directly. Uses `AIAssistant` + `AISuggestion`.
    pub fn submit_ai_application_launch(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        application_id: String,
        reason: Option<String>,
    ) -> Result<workspace_domain::ApplicationLaunchResult> {
        Self::submit_ai_application_launch_inner(
            kernel,
            actor_id,
            application_id,
            reason,
            false,
        )
    }

    /// Test helper — same path as production, stub launcher (no OS spawn).
    #[cfg(test)]
    pub(crate) fn submit_ai_application_launch_simulated(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        application_id: String,
        reason: Option<String>,
    ) -> Result<workspace_domain::ApplicationLaunchResult> {
        Self::submit_ai_application_launch_inner(
            kernel,
            actor_id,
            application_id,
            reason,
            true,
        )
    }

    fn submit_ai_application_launch_inner(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        application_id: String,
        reason: Option<String>,
        simulate: bool,
    ) -> Result<workspace_domain::ApplicationLaunchResult> {
        let actor_id = actor_id.into();
        let application_id = ApplicationId::new(application_id).map_err(KernelError::Domain)?;
        let request = AiParticipationService::propose_application_launch(
            actor_id.clone(),
            &application_id,
            reason,
        )?;
        let actor =
            ActorContext::new(Actor::ai_assistant(actor_id).map_err(KernelError::Domain)?);
        let intent = match request.reason.as_ref() {
            Some(label) => IntentContext::ai_suggestion_with_label(label.clone()),
            None => IntentContext::ai_suggestion(),
        };
        let ctx = kernel.command_context(actor, intent);
        AiParticipationService::ensure_ai_submission_context(&ctx, &request)?;

        let command = if simulate {
            LaunchApplication::simulated(request.application_id().map_err(KernelError::from)?)
        } else {
            LaunchApplication::new(request.application_id().map_err(KernelError::from)?)
        };
        CommandPipeline::new(ctx).execute_mutation(command)
    }

    /// AI planning: goal → proposals only. Does not submit or execute.
    pub fn plan_ai_goal(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
    ) -> Result<AiPlan> {
        let actor_id = actor_id.into();
        let apps = application_ids
            .into_iter()
            .map(ApplicationId::new)
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(KernelError::Domain)?;
        let plan =
            AiPlanningService::plan_prepare_workspace(actor_id.clone(), goal_statement, apps)?;
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiPlanningService::audit_plan_created(&kernel.shared_database(), &actor, &plan)?;
        for proposal in &plan.proposals {
            AiPlanningService::audit_proposal_created(
                &kernel.shared_database(),
                &actor,
                proposal,
            )?;
        }
        Ok(plan)
    }

    /// Plan then submit each proposal through the existing AI governance path.
    ///
    /// Unauthorized outcomes are recorded; proposals are not auto-retried.
    pub fn submit_ai_plan(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
    ) -> Result<AiPlanSubmissionResult> {
        Self::submit_ai_plan_inner(kernel, actor_id, goal_statement, application_ids, false)
    }

    #[cfg(test)]
    pub(crate) fn submit_ai_plan_simulated(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
    ) -> Result<AiPlanSubmissionResult> {
        Self::submit_ai_plan_inner(kernel, actor_id, goal_statement, application_ids, true)
    }

    fn submit_ai_plan_inner(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        simulate: bool,
    ) -> Result<AiPlanSubmissionResult> {
        let actor_id = actor_id.into();
        let plan = Self::plan_ai_goal(
            kernel,
            actor_id.clone(),
            goal_statement,
            application_ids,
        )?;

        let mut submissions = Vec::new();
        for proposal in &plan.proposals {
            let request = proposal
                .to_action_request(&plan.goal.requesting_actor_id)
                .map_err(KernelError::from)?;

            let application_id = request.application_id().map_err(KernelError::from)?;
            let submit_result = Self::submit_ai_application_launch_inner(
                kernel,
                actor_id.clone(),
                application_id.to_string(),
                request.reason.clone(),
                simulate,
            );

            let outcome = match submit_result {
                Ok(_) => AiProposalAuthorityOutcome::Allowed,
                Err(error @ KernelError::ApprovalRequired { .. })
                | Err(error @ KernelError::PermissionDenied(_)) => {
                    AiPlanningService::map_submission_error(error).0
                }
                Err(error) => return Err(error),
            };

            submissions.push(AiProposalSubmission {
                proposal: proposal.clone(),
                request,
                outcome,
            });
        }

        Ok(AiPlanSubmissionResult { plan, submissions })
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

    pub fn get_permission_approvals(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<workspace_domain::PermissionApprovalRequest>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetPermissionApprovals::new(limit))
    }

    pub fn decide_approval(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        request_id: String,
        decision: String,
    ) -> Result<workspace_domain::ApprovalDecisionResult> {
        let request_id = workspace_domain::PermissionApprovalRequestId::new(request_id)
            .map_err(KernelError::Domain)?;
        let decision = workspace_domain::ApprovalDecisionKind::parse(&decision).map_err(|error| {
            KernelError::PermissionApprovalValidation {
                message: error.to_string(),
            }
        })?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DecideApproval::new(request_id, decision))
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

    pub fn get_desktop_windows(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetDesktopWindows::new(limit))
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

    pub fn get_execution_outcomes(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<ExecutionOutcome>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetExecutionOutcomes::new(limit))
    }

    pub fn get_execution_state(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        execution_request_id: String,
    ) -> Result<ExecutionReconciliation> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetExecutionState::new(execution_request_id))
    }

    pub fn get_execution_states(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<ExecutionReconciliation>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetExecutionStates::new(limit))
    }

    pub fn request_execution_cancellation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        execution_request_id: String,
        reason: String,
    ) -> Result<CancellationRequest> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            RequestExecutionCancellation::new(workspace_id, execution_request_id, reason),
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
                active_workspace_id: None,
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
                active_workspace_id: None,
            },
        )
        .unwrap();
        assert_eq!(settings.theme, "dark");
    }
}
