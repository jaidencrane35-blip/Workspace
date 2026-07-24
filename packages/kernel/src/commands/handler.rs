use std::path::Path;

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::application::{CreateApplication, DeleteApplication, GetApplication};
use crate::commands::execute_intent_request::ExecuteIntentRequest;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::get_action_catalog::GetActionCatalog;
use crate::commands::get_ai_evaluation_history::GetAiEvaluationHistory;
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
use crate::commands::memory::{
    ClearMemoryEntries, CreateMemoryEntry, DeleteMemoryEntry, GetMemoryContext, ListMemoryEntries,
};
use crate::commands::model_provider::{
    GetModelProviderMetadata, ListModelProviders, TestModelProviderRequest,
};
use crate::commands::personalization::{
    CreateUserPreference, DeleteUserPreference, GetPreferenceProfile, SetPersonalizationEnabled,
    UpdateUserPreference,
};
use crate::commands::pipeline::CommandPipeline;
use crate::commands::reject_suggestion::RejectSuggestion;
use crate::commands::request_execution_cancellation::RequestExecutionCancellation;
use crate::commands::update_settings::UpdateSettings;
use crate::commands::widget::{CreateWidget, DeleteWidget, GetWidget};
use crate::commands::workspace_intent::{
    CreateProject, CreateTask, CreateWorkGoal, GetProject, GetTask, GetWorkflowContext,
    ListProjects, ListTasks, SetActiveWork, UpdateProject, UpdateTask,
};
use crate::commands::zone::{CreateZone, DeleteZone, GetZone};
use crate::config::{SettingsUpdate, WorkspaceSettings};
use crate::error::{KernelError, Result};
use crate::events::types::{DomainEvent, WorkspaceShutdown};
use crate::lifecycle::LifecycleState;
use crate::security::{PermissionRequest, PermissionSubject};
use crate::services::{
    AiAssistantService, AiEvaluationService, AiOrchestrationService, AiParticipationService,
    AiPlanningService, ConfigurationService, DesktopWindowService, WorkspaceContextService,
    WorkspaceIntelligenceService,
};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActionCatalog, Actor, ActorContext, AiAssistantPlanComparison, AiAssistantWorkflow,
    AiMemoryAwareness, AiOrchestratedPlan, Project, ProjectStatus, Task, TaskPriority, TaskStatus,
    WorkGoal, WorkflowContext, WorkspaceIntelligenceComparison, WorkspaceIntelligenceState,
    AiPlan, AiPlanEvaluationReport, AiPlanSubmissionResult, AiProposalAuthorityOutcome,
    AiProposalEvaluation, AiProposalSubmission, ApplicationId, ApplicationReference, AuditEvent,
    Capability, CapabilitySet, Intent, IntentContext, Layout, LayoutId, LayoutMetadata, LayoutNode,
    LayoutSnapshot, MemoryEntry, MemoryType, ModelProviderDescriptor, ModelResponse, Observation,
    PersonalizedPlanComparison, PreferenceCategory, PreferenceSource, Suggestion,
    SuggestionIntentRequest, SuggestionLifecycleRecord, IntentExecutionRequest, ExecutionOutcome,
    ExecutionReconciliation, CancellationRequest, UserPreference, UserPreferenceProfile, WidgetId,
    WidgetReference, Workspace, WorkspaceContext, WorkspaceId, WorkspaceMetrics, WorkspaceSnapshot,
    CapabilityDiscovery, Zone, ZoneId,
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
    ///
    /// When `workspace_id` is provided, loads read-only [`WorkspaceContext`] via the
    /// ContextProvider (`WorkspaceContextService`) using the local-user provider
    /// identity — AI receives awareness, not mutation rights.
    pub fn plan_ai_goal(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiPlan> {
        Self::plan_ai_goal_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            None,
            None,
        )
    }

    /// Test helper: inject environment window titles (skips live desktop enumeration).
    #[cfg(test)]
    pub(crate) fn plan_ai_goal_with_environment(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
        environment_window_titles: Vec<String>,
    ) -> Result<AiPlan> {
        Self::plan_ai_goal_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            Some(environment_window_titles),
            None,
        )
    }

    fn plan_ai_goal_inner(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
        environment_override: Option<Vec<String>>,
        personalization_override: Option<bool>,
    ) -> Result<AiPlan> {
        let actor_id = actor_id.into();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;

        let plan = if let Some(workspace_id) = workspace_id {
            let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
            // Context is provided by the local-user path (read-only). AI does not
            // gain audit.read — awareness is injected into planning only.
            let provider = ActorContext::local_user();
            let provider_intent = IntentContext::user_request();
            let provider_caps = CapabilitySet::local_user_standard();
            let workspace_context = WorkspaceContextService::build(
                &kernel.shared_database(),
                &provider,
                &provider_intent,
                &provider_caps,
                kernel.permission_policy(),
                kernel.permission_gate(),
                &workspace_id,
                20,
            )?;

            let titles = match environment_override {
                Some(titles) => titles,
                None => DesktopWindowService::list_recent(Some(50))?
                    .into_iter()
                    .map(|window| window.title)
                    .filter(|title| !title.trim().is_empty())
                    .collect(),
            };

            let awareness =
                AiPlanningService::awareness_from_context(&workspace_context, titles);
            AiPlanningService::audit_awareness_used(
                &kernel.shared_database(),
                &actor,
                &awareness,
            )?;
            let memory_awareness = crate::services::AiMemoryService::assemble_awareness(
                &kernel.shared_database(),
                Some(workspace_id.as_str()),
                20,
            )
            .ok();
            let personalization_awareness =
                crate::services::AiPersonalizationService::assemble_awareness(
                    &kernel.shared_database(),
                    Some(workspace_id.as_str()),
                    20,
                    personalization_override,
                )
                .ok();
            AiPlanningService::plan_with_awareness_audited(
                &kernel.shared_database(),
                &actor,
                actor_id,
                goal_statement,
                awareness,
                memory_awareness,
                personalization_awareness,
            )?
        } else {
            let apps = application_ids
                .into_iter()
                .map(ApplicationId::new)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(KernelError::Domain)?;
            let memory_awareness = crate::services::AiMemoryService::assemble_awareness(
                &kernel.shared_database(),
                None,
                20,
            )
            .ok();
            let personalization_awareness =
                crate::services::AiPersonalizationService::assemble_awareness(
                    &kernel.shared_database(),
                    None,
                    20,
                    personalization_override,
                )
                .ok();
            AiPlanningService::plan_prepare_workspace_audited(
                &kernel.shared_database(),
                &actor,
                actor_id,
                goal_statement,
                apps,
                memory_awareness,
                personalization_awareness,
            )?
        };

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
        workspace_id: Option<String>,
    ) -> Result<AiPlanSubmissionResult> {
        Self::submit_ai_plan_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            false,
            None,
        )
    }

    #[cfg(test)]
    pub(crate) fn submit_ai_plan_simulated(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
    ) -> Result<AiPlanSubmissionResult> {
        Self::submit_ai_plan_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            None,
            true,
            None,
        )
    }

    #[cfg(test)]
    pub(crate) fn submit_ai_plan_simulated_with_workspace(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        workspace_id: String,
        environment_window_titles: Vec<String>,
    ) -> Result<AiPlanSubmissionResult> {
        Self::submit_ai_plan_inner(
            kernel,
            actor_id,
            goal_statement,
            Vec::new(),
            Some(workspace_id),
            true,
            Some(environment_window_titles),
        )
    }

    fn submit_ai_plan_inner(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
        simulate: bool,
        environment_override: Option<Vec<String>>,
    ) -> Result<AiPlanSubmissionResult> {
        let actor_id = actor_id.into();
        let plan = Self::plan_ai_goal_inner(
            kernel,
            actor_id.clone(),
            goal_statement,
            application_ids,
            workspace_id,
            environment_override,
            None,
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

        let result = AiPlanSubmissionResult { plan, submissions };
        // Measurement only — evaluation never grants authority or retries.
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        let report = AiEvaluationService::evaluate_submission_result(&result, None)?;
        AiEvaluationService::audit_report(&kernel.shared_database(), &actor, &report)?;
        Ok(result)
    }

    /// Plans and evaluates proposal quality without submitting or executing.
    pub fn diagnose_ai_plan_evaluation(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiPlanEvaluationReport> {
        Self::diagnose_ai_plan_evaluation_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            None,
        )
    }

    fn diagnose_ai_plan_evaluation_inner(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
        environment_override: Option<Vec<String>>,
    ) -> Result<AiPlanEvaluationReport> {
        let actor_id = actor_id.into();
        let environment_for_plan = environment_override.clone();
        let awareness = if let Some(workspace_id) = workspace_id.clone() {
            let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
            let provider = ActorContext::local_user();
            let provider_intent = IntentContext::user_request();
            let provider_caps = CapabilitySet::local_user_standard();
            let workspace_context = WorkspaceContextService::build(
                &kernel.shared_database(),
                &provider,
                &provider_intent,
                &provider_caps,
                kernel.permission_policy(),
                kernel.permission_gate(),
                &workspace_id,
                20,
            )?;
            let titles = match environment_override {
                Some(titles) => titles,
                None => DesktopWindowService::list_recent(Some(50))?
                    .into_iter()
                    .map(|window| window.title)
                    .filter(|title| !title.trim().is_empty())
                    .collect(),
            };
            Some(AiPlanningService::awareness_from_context(
                &workspace_context,
                titles,
            ))
        } else {
            None
        };

        let plan = Self::plan_ai_goal_inner(
            kernel,
            actor_id.clone(),
            goal_statement,
            application_ids,
            workspace_id,
            environment_for_plan,
            None,
        )?;

        // Note: context-aware planning already skips active apps, so "unnecessary"
        // appears when evaluating injected/poor proposals or when awareness differs.
        let report = AiEvaluationService::evaluate_plan(&plan, awareness.as_ref())?;
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiEvaluationService::audit_report(&kernel.shared_database(), &actor, &report)?;
        Ok(report)
    }

    /// Derived evaluation history from operational audits (read-only).
    pub fn get_ai_evaluation_history(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        limit: Option<usize>,
    ) -> Result<Vec<AiProposalEvaluation>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetAiEvaluationHistory::new(limit))
    }

    /// Creates a multi-step orchestrated plan from AI proposals (no execution).
    pub fn create_orchestrated_ai_plan(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiOrchestratedPlan> {
        let actor_id = actor_id.into();
        let ai_plan = Self::plan_ai_goal_inner(
            kernel,
            actor_id.clone(),
            goal_statement,
            application_ids,
            workspace_id,
            None,
            None,
        )?;
        let plan = AiOrchestrationService::create_from_plan(ai_plan)?;
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiOrchestrationService::audit_plan_created(&kernel.shared_database(), &actor, &plan)?;
        AiOrchestrationService::store_plan(&kernel.orchestrated_plans(), plan)
    }

    pub fn get_orchestrated_ai_plan(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        AiOrchestrationService::get_plan(&kernel.orchestrated_plans(), &plan_id.into())
    }

    /// Advances runnable steps through the existing AI → pipeline → gateway path.
    ///
    /// Pauses on ApprovalRequired. Stops on Denied/Failed. No silent continue.
    pub fn advance_orchestrated_ai_plan(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        Self::advance_orchestrated_ai_plan_inner(kernel, plan_id, false)
    }

    #[cfg(test)]
    pub(crate) fn advance_orchestrated_ai_plan_simulated(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        Self::advance_orchestrated_ai_plan_inner(kernel, plan_id, true)
    }

    /// Resumes a step paused on ApprovalRequired after human DecideApproval.
    pub fn resume_orchestrated_ai_plan(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        Self::resume_orchestrated_ai_plan_inner(kernel, plan_id, false)
    }

    #[cfg(test)]
    pub(crate) fn resume_orchestrated_ai_plan_simulated(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        Self::resume_orchestrated_ai_plan_inner(kernel, plan_id, true)
    }

    pub fn cancel_orchestrated_ai_plan(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
    ) -> Result<AiOrchestratedPlan> {
        let mut plan =
            AiOrchestrationService::get_plan(&kernel.orchestrated_plans(), &plan_id.into())?;
        let actor_id = plan.requesting_actor_id.as_str().to_string();
        plan.cancel().map_err(KernelError::from)?;
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiOrchestrationService::audit_plan_cancelled(&kernel.shared_database(), &actor, &plan)?;
        AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan)
    }

    fn advance_orchestrated_ai_plan_inner(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
        simulate: bool,
    ) -> Result<AiOrchestratedPlan> {
        let mut plan =
            AiOrchestrationService::get_plan(&kernel.orchestrated_plans(), &plan_id.into())?;
        let actor_id = plan.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;

        AiOrchestrationService::ensure_can_advance(&plan)?;

        while let Some(index) = plan.next_runnable_step_index() {
            plan.begin_step(index).map_err(KernelError::from)?;
            AiOrchestrationService::audit_action_started(
                &kernel.shared_database(),
                &actor,
                &plan,
                index,
            )?;

            let application_id = AiOrchestrationService::step_application_id(&plan, index)?;
            let reason = AiOrchestrationService::step_reason(&plan, index);
            let submit_result = Self::submit_ai_application_launch_inner(
                kernel,
                actor_id.clone(),
                application_id,
                reason,
                simulate,
            );

            match submit_result {
                Ok(_) => {
                    plan.apply_authority_outcome(index, &AiProposalAuthorityOutcome::Allowed)
                        .map_err(KernelError::from)?;
                    AiOrchestrationService::audit_action_completed(
                        &kernel.shared_database(),
                        &actor,
                        &plan,
                        index,
                    )?;
                }
                Err(error @ KernelError::ApprovalRequired { .. }) => {
                    let outcome = AiOrchestrationService::map_submission_error(error).0;
                    plan.apply_authority_outcome(index, &outcome)
                        .map_err(KernelError::from)?;
                    AiOrchestrationService::audit_action_failed(
                        &kernel.shared_database(),
                        &actor,
                        &plan,
                        index,
                        "approval_required",
                    )?;
                    break;
                }
                Err(error @ KernelError::PermissionDenied(_)) => {
                    let outcome = AiOrchestrationService::map_submission_error(error).0;
                    plan.apply_authority_outcome(index, &outcome)
                        .map_err(KernelError::from)?;
                    AiOrchestrationService::audit_action_failed(
                        &kernel.shared_database(),
                        &actor,
                        &plan,
                        index,
                        "denied",
                    )?;
                    break;
                }
                Err(error) => {
                    plan.apply_step_failure(index, error.to_string())
                        .map_err(KernelError::from)?;
                    AiOrchestrationService::audit_action_failed(
                        &kernel.shared_database(),
                        &actor,
                        &plan,
                        index,
                        "failed",
                    )?;
                    break;
                }
            }
        }

        AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan)
    }

    fn resume_orchestrated_ai_plan_inner(
        kernel: &WorkspaceKernel,
        plan_id: impl Into<String>,
        simulate: bool,
    ) -> Result<AiOrchestratedPlan> {
        let mut plan =
            AiOrchestrationService::get_plan(&kernel.orchestrated_plans(), &plan_id.into())?;
        let actor_id = plan.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        let index = AiOrchestrationService::ensure_can_resume(&plan)?;

        // If the human denied the paused approval, fail the step without re-submit / retry.
        if let Some(approval_id) = plan.steps[index].approval_request_id.clone() {
            let request_id =
                workspace_domain::PermissionApprovalRequestId::new(approval_id)
                    .map_err(KernelError::Domain)?;
            let db = kernel.shared_database();
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            if let Some(request) =
                crate::services::PermissionApprovalService::get_request(&guard, &request_id)?
            {
                if request.status == workspace_domain::PermissionApprovalStatus::Denied {
                    drop(guard);
                    plan.apply_authority_outcome(
                        index,
                        &AiProposalAuthorityOutcome::Denied {
                            reason: "approval denied by local user".into(),
                        },
                    )
                    .map_err(KernelError::from)?;
                    AiOrchestrationService::audit_action_failed(
                        &kernel.shared_database(),
                        &actor,
                        &plan,
                        index,
                        "denied",
                    )?;
                    return AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan);
                }
            }
        }

        plan.begin_step(index).map_err(KernelError::from)?;
        AiOrchestrationService::audit_action_started(
            &kernel.shared_database(),
            &actor,
            &plan,
            index,
        )?;

        let application_id = AiOrchestrationService::step_application_id(&plan, index)?;
        let reason = AiOrchestrationService::step_reason(&plan, index);
        let submit_result = Self::submit_ai_application_launch_inner(
            kernel,
            actor_id.clone(),
            application_id,
            reason,
            simulate,
        );

        match submit_result {
            Ok(_) => {
                plan.apply_authority_outcome(index, &AiProposalAuthorityOutcome::Allowed)
                    .map_err(KernelError::from)?;
                AiOrchestrationService::audit_action_completed(
                    &kernel.shared_database(),
                    &actor,
                    &plan,
                    index,
                )?;
            }
            Err(error @ KernelError::ApprovalRequired { .. }) => {
                let outcome = AiOrchestrationService::map_submission_error(error).0;
                plan.apply_authority_outcome(index, &outcome)
                    .map_err(KernelError::from)?;
                AiOrchestrationService::audit_action_failed(
                    &kernel.shared_database(),
                    &actor,
                    &plan,
                    index,
                    "approval_required",
                )?;
                return AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan);
            }
            Err(error @ KernelError::PermissionDenied(_)) => {
                let outcome = AiOrchestrationService::map_submission_error(error).0;
                plan.apply_authority_outcome(index, &outcome)
                    .map_err(KernelError::from)?;
                AiOrchestrationService::audit_action_failed(
                    &kernel.shared_database(),
                    &actor,
                    &plan,
                    index,
                    "denied",
                )?;
                return AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan);
            }
            Err(error) => {
                plan.apply_step_failure(index, error.to_string())
                    .map_err(KernelError::from)?;
                AiOrchestrationService::audit_action_failed(
                    &kernel.shared_database(),
                    &actor,
                    &plan,
                    index,
                    "failed",
                )?;
                return AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan);
            }
        }

        let plan = AiOrchestrationService::save_plan(&kernel.orchestrated_plans(), plan)?;
        // Continue remaining pending steps after a successful resume.
        if plan.state.is_terminal()
            || plan.awaiting_approval_step_index().is_some()
            || plan.next_runnable_step_index().is_none()
        {
            return Ok(plan);
        }
        Self::advance_orchestrated_ai_plan_inner(kernel, plan.id.to_string(), simulate)
    }

    /// Assistant: accept a natural-language goal and present a governed plan preview.
    ///
    /// Does not execute. Creates an orchestrated plan under the hood for later confirm.
    pub fn submit_assistant_goal(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        user_goal: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiAssistantWorkflow> {
        let actor_id = actor_id.into();
        let mut workflow =
            workspace_domain::AiAssistantWorkflow::receive_goal(user_goal, actor_id.clone())
                .map_err(KernelError::from)?
                .with_planning_context(application_ids.clone(), workspace_id.clone());
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiAssistantService::audit_goal_received(&kernel.shared_database(), &actor, &workflow)?;

        workflow.mark_understanding();
        Self::rebuild_assistant_plan(kernel, &mut workflow)?;
        AiAssistantService::audit_plan_presented(&kernel.shared_database(), &actor, &workflow)?;
        AiAssistantService::store_workflow(&kernel.assistant_workflows(), workflow)
    }

    /// Revise the goal and regenerate a governed plan (archives prior preview).
    pub fn revise_assistant_goal(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        new_goal: impl Into<String>,
        application_ids: Option<Vec<String>>,
        workspace_id: Option<String>,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        let previous_goal = workflow.user_goal.clone();

        if let Some(ids) = application_ids {
            workflow.application_ids = ids;
        }
        if workspace_id.is_some() {
            workflow.workspace_id = workspace_id;
        }

        workflow.revise_goal(new_goal).map_err(KernelError::from)?;
        AiAssistantService::audit_goal_updated(
            &kernel.shared_database(),
            &actor,
            &workflow,
            &previous_goal,
        )?;
        Self::rebuild_assistant_plan(kernel, &mut workflow)?;
        AiAssistantService::audit_plan_regenerated(
            &kernel.shared_database(),
            &actor,
            &workflow,
            "goal_revised",
        )?;
        AiAssistantService::save_workflow(&kernel.assistant_workflows(), workflow)
    }

    /// Regenerate the current plan without changing the goal.
    pub fn regenerate_assistant_plan(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        application_ids: Option<Vec<String>>,
        workspace_id: Option<String>,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;

        if let Some(ids) = application_ids {
            workflow.application_ids = ids;
        }
        if workspace_id.is_some() {
            workflow.workspace_id = workspace_id;
        }

        workflow.prepare_regenerate().map_err(KernelError::from)?;
        Self::rebuild_assistant_plan(kernel, &mut workflow)?;
        AiAssistantService::audit_plan_regenerated(
            &kernel.shared_database(),
            &actor,
            &workflow,
            "user_regenerate",
        )?;
        AiAssistantService::save_workflow(&kernel.assistant_workflows(), workflow)
    }

    /// Compare two plan revisions (or a revision vs the current preview).
    ///
    /// Pass `None` for a revision number to mean the current plan preview.
    pub fn compare_assistant_plan_revisions(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        left_revision: Option<u32>,
        right_revision: Option<u32>,
    ) -> Result<AiAssistantPlanComparison> {
        let workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;

        let left = Self::resolve_assistant_revision(&workflow, left_revision)?;
        let right = Self::resolve_assistant_revision(&workflow, right_revision)?;
        let left_num = left.revision;
        let right_num = right.revision;
        let mut comparison = AiAssistantPlanComparison::compare(left, right);
        comparison.workflow_id = workflow.id.to_string();

        AiAssistantService::audit_plan_compared(
            &kernel.shared_database(),
            &actor,
            workflow.id.as_str(),
            left_num,
            right_num,
            comparison.differences.len(),
        )?;
        Ok(comparison)
    }

    /// Record that the user viewed a structured proposal explanation (audit only).
    pub fn record_assistant_explanation_viewed(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        step_id: impl Into<String>,
    ) -> Result<()> {
        let workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        let step_id = step_id.into();
        AiAssistantService::audit_explanation_viewed(
            &kernel.shared_database(),
            &actor,
            workflow.id.as_str(),
            &step_id,
        )
    }

    fn resolve_assistant_revision(
        workflow: &AiAssistantWorkflow,
        revision: Option<u32>,
    ) -> Result<workspace_domain::AiAssistantPlanRevision> {
        match revision {
            Some(number) => workflow
                .revision_by_number(number)
                .cloned()
                .ok_or_else(|| KernelError::AiAssistantValidation {
                    message: format!("assistant plan revision not found: {number}"),
                }),
            None => workflow
                .current_as_revision()
                .ok_or_else(|| KernelError::AiAssistantValidation {
                    message: "assistant workflow has no current plan preview to compare".into(),
                }),
        }
    }

    fn rebuild_assistant_plan(
        kernel: &WorkspaceKernel,
        workflow: &mut AiAssistantWorkflow,
    ) -> Result<()> {
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        workflow.mark_generating_plan();
        let plan = Self::create_orchestrated_ai_plan(
            kernel,
            actor_id,
            workflow.user_goal.clone(),
            workflow.application_ids.clone(),
            workflow.workspace_id.clone(),
        )?;
        workflow.mark_evaluating();
        let preview = AiAssistantService::build_plan_preview(&plan);
        workflow
            .present_plan(&plan, preview)
            .map_err(KernelError::from)?;
        Ok(())
    }

    pub fn get_assistant_workflow(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        if let Some(plan_id) = workflow.orchestrated_plan_id.clone() {
            if let Ok(plan) =
                AiOrchestrationService::get_plan(&kernel.orchestrated_plans(), plan_id.as_str())
            {
                workflow.sync_from_plan(&plan);
                workflow = AiAssistantService::save_workflow(
                    &kernel.assistant_workflows(),
                    workflow,
                )?;
            }
        }
        Ok(workflow)
    }

    /// User confirms the presented plan → advance through Permission Gateway.
    pub fn confirm_assistant_workflow(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        Self::confirm_assistant_workflow_inner(kernel, workflow_id, false)
    }

    #[cfg(test)]
    pub(crate) fn confirm_assistant_workflow_simulated(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        Self::confirm_assistant_workflow_inner(kernel, workflow_id, true)
    }

    /// Resume assistant workflow after human permission decision on a paused step.
    pub fn resume_assistant_workflow(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        Self::resume_assistant_workflow_inner(kernel, workflow_id, false)
    }

    #[cfg(test)]
    pub(crate) fn resume_assistant_workflow_simulated(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        Self::resume_assistant_workflow_inner(kernel, workflow_id, true)
    }

    pub fn cancel_assistant_workflow(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;

        workflow.cancel().map_err(KernelError::from)?;
        if let Some(plan_id) = workflow.orchestrated_plan_id.clone() {
            let _ = Self::cancel_orchestrated_ai_plan(kernel, plan_id.to_string());
        }
        AiAssistantService::audit_cancelled(&kernel.shared_database(), &actor, &workflow)?;
        AiAssistantService::save_workflow(&kernel.assistant_workflows(), workflow)
    }

    fn confirm_assistant_workflow_inner(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        simulate: bool,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let actor_id = workflow.requesting_actor_id.as_str().to_string();
        let actor = AiPlanningService::ensure_ai_actor(&actor_id)?;
        AiAssistantService::ensure_awaiting_confirmation(&workflow)?;
        workflow.confirm().map_err(KernelError::from)?;
        AiAssistantService::audit_user_confirmed(&kernel.shared_database(), &actor, &workflow)?;

        let plan_id = workflow
            .orchestrated_plan_id
            .as_ref()
            .ok_or_else(|| KernelError::AiAssistantValidation {
                message: "assistant workflow has no linked plan".into(),
            })?
            .to_string();

        let plan = Self::advance_orchestrated_ai_plan_inner(kernel, plan_id, simulate)?;
        workflow.sync_from_plan(&plan);
        AiAssistantService::save_workflow(&kernel.assistant_workflows(), workflow)
    }

    fn resume_assistant_workflow_inner(
        kernel: &WorkspaceKernel,
        workflow_id: impl Into<String>,
        simulate: bool,
    ) -> Result<AiAssistantWorkflow> {
        let mut workflow =
            AiAssistantService::get_workflow(&kernel.assistant_workflows(), &workflow_id.into())?;
        let plan_id = workflow
            .orchestrated_plan_id
            .as_ref()
            .ok_or_else(|| KernelError::AiAssistantValidation {
                message: "assistant workflow has no linked plan".into(),
            })?
            .to_string();

        let plan = Self::resume_orchestrated_ai_plan_inner(kernel, plan_id, simulate)?;
        workflow.sync_from_plan(&plan);
        AiAssistantService::save_workflow(&kernel.assistant_workflows(), workflow)
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

    /// Informational action catalog — does not grant authority.
    pub fn get_action_catalog(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<ActionCatalog> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_query(GetActionCatalog)
    }

    /// Creates a governed memory entry (informational only — not authority).
    pub fn create_memory_entry(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        memory_type: MemoryType,
        key: impl Into<String>,
        summary: impl Into<String>,
        source: impl Into<String>,
        workspace_id: Option<String>,
        attributes: Option<String>,
    ) -> Result<MemoryEntry> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateMemoryEntry::new(
                memory_type,
                key.into(),
                summary.into(),
                source.into(),
                workspace_id,
                attributes,
            ),
        )
    }

    pub fn list_memory_entries(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: Option<String>,
        limit: Option<usize>,
    ) -> Result<Vec<MemoryEntry>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListMemoryEntries::new(workspace_id, limit))
    }

    pub fn get_memory_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: Option<String>,
        limit: Option<usize>,
    ) -> Result<AiMemoryAwareness> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetMemoryContext::new(workspace_id, limit))
    }

    pub fn delete_memory_entry(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: impl Into<String>,
    ) -> Result<MemoryEntry> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteMemoryEntry::new(id.into()))
    }

    pub fn clear_memory_entries(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        memory_type: Option<MemoryType>,
        workspace_id: Option<String>,
    ) -> Result<usize> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(ClearMemoryEntries::new(memory_type, workspace_id))
    }

    /// Planning-only diagnostic: goal → memory-aware proposals (no submission/execution).
    pub fn diagnose_ai_plan_preview(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiPlan> {
        Self::plan_ai_goal(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
        )
    }

    pub fn list_model_providers(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<Vec<ModelProviderDescriptor>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListModelProviders)
    }

    pub fn get_model_provider_metadata(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        provider_id: impl Into<String>,
    ) -> Result<ModelProviderDescriptor> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetModelProviderMetadata::new(provider_id.into()))
    }

    pub fn test_model_provider_request(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        task: impl Into<String>,
        application_ids: Vec<String>,
        preferred_provider_id: Option<String>,
    ) -> Result<ModelResponse> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_query(
            TestModelProviderRequest::new(task.into(), application_ids, preferred_provider_id),
        )
    }

    /// Diagnostic: provider → proposals only (no Permission Gateway submission).
    pub fn diagnose_model_proposal_generation(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<AiPlan> {
        Self::diagnose_ai_plan_preview(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
        )
    }

    pub fn create_user_preference(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        category: PreferenceCategory,
        key: impl Into<String>,
        value: impl Into<String>,
        source: PreferenceSource,
        workspace_id: Option<String>,
        label: Option<String>,
        attributes: Option<String>,
    ) -> Result<UserPreference> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateUserPreference::new(
                category,
                key.into(),
                value.into(),
                source,
                workspace_id,
                label,
                attributes,
            ),
        )
    }

    pub fn update_user_preference(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: impl Into<String>,
        value: Option<String>,
        label: Option<Option<String>>,
        attributes: Option<Option<String>>,
        confidence: Option<u8>,
    ) -> Result<UserPreference> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            UpdateUserPreference::new(id.into(), value, label, attributes, confidence),
        )
    }

    pub fn get_preference_profile(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: Option<String>,
        limit: Option<usize>,
    ) -> Result<UserPreferenceProfile> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetPreferenceProfile::new(workspace_id, limit))
    }

    pub fn delete_user_preference(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        id: impl Into<String>,
    ) -> Result<UserPreference> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(DeleteUserPreference::new(id.into()))
    }

    pub fn set_personalization_enabled(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        enabled: bool,
    ) -> Result<bool> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(SetPersonalizationEnabled::new(enabled))
    }

    /// Planning with an explicit personalization on/off override (diagnostics).
    pub fn diagnose_ai_plan_with_personalization(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
        personalization_enabled: bool,
    ) -> Result<AiPlan> {
        Self::plan_ai_goal_inner(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            None,
            Some(personalization_enabled),
        )
    }

    /// Compare personalized vs neutral planning (no execution).
    pub fn compare_personalized_vs_neutral_plan(
        kernel: &WorkspaceKernel,
        actor_id: impl Into<String>,
        goal_statement: impl Into<String>,
        application_ids: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<PersonalizedPlanComparison> {
        let actor_id = actor_id.into();
        let goal_statement = goal_statement.into();
        let personalized = Self::diagnose_ai_plan_with_personalization(
            kernel,
            actor_id.clone(),
            goal_statement.clone(),
            application_ids.clone(),
            workspace_id.clone(),
            true,
        )?;
        let neutral = Self::diagnose_ai_plan_with_personalization(
            kernel,
            actor_id,
            goal_statement,
            application_ids,
            workspace_id,
            false,
        )?;
        Ok(PersonalizedPlanComparison {
            personalized,
            neutral,
        })
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

    pub fn create_project(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        description: Option<String>,
        metadata: Option<String>,
    ) -> Result<Project> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateProject::new(workspace_id, name, description, metadata),
        )
    }

    pub fn update_project(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        project_id: String,
        name: Option<String>,
        description: Option<Option<String>>,
        status: Option<ProjectStatus>,
    ) -> Result<Project> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            UpdateProject::new(project_id, name, description, status),
        )
    }

    pub fn get_project(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        project_id: String,
    ) -> Result<Project> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetProject::new(project_id))
    }

    pub fn list_projects(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<Project>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListProjects::new(workspace_id, limit))
    }

    pub fn create_task(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        project_id: String,
        workspace_id: String,
        title: String,
        priority: TaskPriority,
    ) -> Result<Task> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateTask::new(project_id, workspace_id, title, priority),
        )
    }

    pub fn update_task(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        task_id: String,
        title: Option<String>,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
    ) -> Result<Task> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(UpdateTask::new(task_id, title, status, priority))
    }

    pub fn get_task(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        task_id: String,
    ) -> Result<Task> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetTask::new(task_id))
    }

    pub fn list_tasks(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        project_id: Option<String>,
        limit: Option<usize>,
    ) -> Result<Vec<Task>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListTasks::new(workspace_id, project_id, limit))
    }

    pub fn create_work_goal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        description: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<WorkGoal> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateWorkGoal::new(workspace_id, description, project_id, task_id),
        )
    }

    pub fn get_workflow_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkflowContext> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkflowContext::new(workspace_id))
    }

    pub fn set_active_work(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<WorkflowContext> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            SetActiveWork::new(workspace_id, project_id, task_id),
        )
    }

    /// Read-only workspace intelligence aggregation. Capability-gated; never executes.
    pub fn generate_workspace_intelligence(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceIntelligenceState> {
        // Gate on work_context.read via existing query path before aggregating.
        let _ =
            Self::get_workflow_context(kernel, actor.clone(), intent.clone(), workspace_id.clone())?;
        let workspace = Self::get_workspace(kernel, actor.clone(), intent.clone(), workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceIntelligenceService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    pub fn compare_workspace_intelligence_states(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        left_workspace_id: String,
        right_workspace_id: String,
    ) -> Result<WorkspaceIntelligenceComparison> {
        let left = Self::generate_workspace_intelligence(
            kernel,
            actor.clone(),
            intent.clone(),
            left_workspace_id,
        )?;
        let right =
            Self::generate_workspace_intelligence(kernel, actor, intent, right_workspace_id)?;
        Ok(WorkspaceIntelligenceComparison::compare(&left, &right))
    }

    pub fn shutdown(kernel: &mut WorkspaceKernel) {
        let request = PermissionRequest {
            actor: Actor::system(),
            intent: Intent::system_shutdown(),
            capability: Capability::system_shutdown(),
            command: "ShutdownWorkspace",
            subject: PermissionSubject::System,
            target_resource_id: None,
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
                personalization_enabled: None,
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
                personalization_enabled: None,
            },
        )
        .unwrap();
        assert_eq!(settings.theme, "dark");
    }
}
