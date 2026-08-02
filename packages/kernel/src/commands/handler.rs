use std::path::Path;

use crate::commands::accept_suggestion::AcceptSuggestion;
use crate::commands::application::{CreateApplication, DeleteApplication, GetApplication};
use crate::commands::automation_contract::{
    ApproveAutomationContract, CreateAutomationContract, GetAutomationContract,
    ListAutomationContracts, PauseAutomationContract, PrepareAutomationContractIntent,
    RequestAutomationContractApproval, ResumeAutomationContract, RevokeAutomationContract,
    UpdateAutomationContract,
};
use crate::commands::automation_trigger::{
    AcceptAutomationIntentProposal, EvaluateTriggers, ListAutomationIntentProposals,
    ListTriggerEvents, RecordAndEvaluateTriggers, RecordTriggerEvent,
    RejectAutomationIntentProposal,
};
use crate::commands::decision_engine::{GateDecisionEngineRead, GateDecisionEngineWrite};
use crate::commands::decision_queue::{GateDecisionQueueRead, GateDecisionQueueWrite};
use crate::commands::task_graph::{GateTaskGraphRead, GateTaskGraphWrite};
use crate::commands::workspace_environment::GateEnvironmentRead;
use crate::commands::workspace_composition::GateCompositionRead;
use crate::commands::workspace_purpose::GatePurposeRead;
use crate::commands::workspace_evolution::GateEvolutionRead;
use crate::commands::workspace_recommendation::{
    GateRecommendationEngineRead, GateRecommendationEngineWrite,
};
use crate::commands::workspace_operating_state::GateOperatingStateRead;
use crate::commands::workspace_pattern::GatePatternRead;
use crate::commands::workspace_adaptation::{GateAdaptationRead, GateAdaptationWrite};
use crate::commands::workspace_readiness::GateReadinessRead;
use crate::commands::workspace_runtime::GateRuntimeRead;
use crate::commands::workspace_intelligence::GateIntelligenceRead;
use crate::commands::workspace_session::GateSessionRead;
use crate::commands::workspace_experience::GateExperienceRead;
use crate::commands::workspace_work_context::GateWorkContextEngineRead;
use crate::commands::workspace_navigation::GateNavigationRead;
use crate::commands::workspace_milestone::GateMilestoneRead;
use crate::commands::workspace_working_style::GateWorkingStyleRead;
use crate::commands::workspace_transition::GateTransitionRead;
use crate::commands::workspace_interaction::{
    GateWorkspaceInteractionRead, GateWorkspaceInteractionWrite,
};
use crate::commands::workspace_profile::{GateWorkspaceProfileRead, GateWorkspaceProfileWrite};
use crate::commands::workspace_activity::GateActivityGraphRead;
use crate::commands::workspace_attention::GateAttentionRead;
use crate::commands::workspace_continuity::GateContinuityRead;
use crate::commands::execute_intent_request::ExecuteIntentRequest;
use crate::commands::create_suggestion_intent_request::CreateSuggestionIntentRequest;
use crate::commands::create_workspace::CreateWorkspace;
use crate::commands::decide_approval::DecideApproval;
use crate::commands::get_action_catalog::GetActionCatalog;
use crate::commands::get_ai_evaluation_history::GetAiEvaluationHistory;
use crate::commands::get_actor_capabilities::GetActorCapabilities;
use crate::commands::get_audit_history::GetAuditHistory;
use crate::commands::get_permission_approvals::GetPermissionApprovals;
use crate::commands::workspace_observation::{
    CaptureWorkspaceObservation, GateObservationRead, GetLatestObservationDelta,
    GetLatestWorkspaceObservation, GetObservationSchedulerStatus, GetWorkspaceObservationById,
    GetWorkspaceObservationStatus,
};
use crate::commands::workspace_state::GetWorkspaceState;
use crate::commands::saved_context::{GetSavedContextCaptureScope, SaveWorkspaceContext};
use crate::commands::resume::{
    ExecuteResumePlan, GetSavedContext, ListSavedContexts, ResolveResumePlan, ResumePlanPreview,
};
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
    AiPlanningService, ConfigurationService, DecisionEngineService, DecisionQueueService,
    TaskGraphService, WorkspaceEnvironmentService, WorkspaceStateEngine,
    WorkspaceCompositionService, WorkspacePurposeService, WorkspaceEvolutionService,
    WorkspaceRecommendationEngineService, WorkspaceOperatingStateService,
    WorkspacePatternService, WorkspaceAdaptationService, WorkspaceReadinessService,
    WorkspaceRuntimeService, WorkspaceSessionService, WorkspaceExperienceService,
    WorkspaceWorkContextService,
    WorkspaceNavigationService, WorkspaceMilestoneService, WorkspaceWorkingStyleService,
    WorkspaceTransitionService, WorkspaceInteractionService, WorkspaceProfileService,
    WorkspaceObservationCaptureResult, WorkspaceObservationService, ObservationTriggerAuthority,
    ObservationSchedulerDiagnostics, WorkspaceActivityGraphService,
    WorkspaceAttentionService, WorkspaceContextService, WorkspaceContinuityService,
    WorkspaceIntelligenceService,
};
use crate::WorkspaceKernel;
use workspace_domain::{
    ActionCatalog, Actor, ActorContext, AiAssistantPlanComparison, AiAssistantWorkflow,
    AiMemoryAwareness, AiOrchestratedPlan, AutomationContract, AutomationContractIntentRequest,
    AutomationIntentProposal, AutomationIntentProposalStatus, AutomationTriggerKind,
    DecisionActionResult, DecisionEngineActionResult, DecisionEngineState, DecisionItem,
    DecisionQueue, Project, ProjectStatus, Task, TaskGraph, TaskPriority, TaskRelationship,
    TaskRelationshipKind, TaskStatus, TriggerEvaluationResult, TriggerEvent, TriggerEventType,
    WorkGoal, WorkflowContext, WorkspaceActivity, WorkspaceActivityGraph, WorkspaceAttentionState,
    WorkspaceContinuityState, WorkspaceEnvironmentState, WorkspaceCompositionState,
    WorkspacePurposeState, WorkspaceEvolutionState, RecommendationReviewActionResult,
    WorkspaceRecommendationEngineState,
    WorkspaceOperatingState, WorkspacePatternState, AdaptationActionResult, WorkspaceAdaptationState,
    WorkspaceReadinessState, WorkspaceRuntimeOperatorView, WorkspaceSessionComparison,
    WorkspaceSessionState, WorkspaceExperienceComparison, WorkspaceExperienceState,
    WorkspaceWorkContextComparison, WorkspaceWorkContextState, WorkspaceWorkContextValidation,
    WorkspaceNavigationComparison, WorkspaceNavigationState, WorkspaceNavigationValidation,
    WorkspaceMilestoneComparison, WorkspaceMilestoneState, WorkspaceMilestoneValidation,
    WorkspaceWorkingStyleComparison, WorkspaceWorkingStyleState, WorkspaceWorkingStyleValidation,
    WorkspaceTransitionComparison, WorkspaceTransitionState, WorkspaceTransitionValidation,
    InteractionSelectResult, WorkspaceInteractionComparison, WorkspaceInteractionState,
    WorkspaceInteractionValidation,
    WorkspaceProfile, WorkspaceProfileComparison, WorkspaceProfileMemberInput,
    WorkspaceProfileState, WorkspaceProfileStateComparison, WorkspaceProfileStatus,
    ActionOperationResult, ActionPlan, SaveContextRequest, SavedContext, SavedContextCaptureScope,
    SavedContextId,
    WorkspaceProfileValidation, WorkspaceObservationSnapshot, WorkspaceObservationStatus,
    ObservationConsumerFreshnessNeed, ObservationFreshnessEnsureResult, ObservationSchedulerStatus,
    WorkspaceObservationDelta,
    WorkspaceState as ProjectedWorkspaceState, WorkspaceTask, WorkspaceTaskPriority,
    WorkspaceTaskStatus,
    WorkspaceIntelligenceComparison, WorkspaceIntelligenceState, AiPlan, AiPlanEvaluationReport,
    AiPlanSubmissionResult,
    AiProposalAuthorityOutcome, AiProposalEvaluation, AiProposalSubmission, ApplicationId,
    ApplicationReference, AuditEvent, Capability, CapabilitySet, Intent, IntentContext, Layout,
    LayoutId, LayoutMetadata, LayoutNode, LayoutSnapshot, MemoryEntry, MemoryType,
    ModelProviderDescriptor, ModelResponse, Observation, PersonalizedPlanComparison,
    PreferenceCategory, PreferenceSource, Suggestion, SuggestionIntentRequest,
    SuggestionLifecycleRecord, IntentExecutionRequest, ExecutionOutcome, ExecutionReconciliation,
    CancellationRequest, UserPreference, UserPreferenceProfile, WidgetId, WidgetReference,
    Workspace, WorkspaceContext, WorkspaceId, WorkspaceMetrics, WorkspaceSnapshot,
    CapabilityDiscovery, Zone, ZoneId,
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
                None => WorkspaceStateEngine::get_current(
                    &kernel.shared_database(),
                    &ActorContext::system(),
                    &IntentContext::user_request(),
                )?
                .windows
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
                None => WorkspaceStateEngine::get_current(
                    &kernel.shared_database(),
                    &ActorContext::system(),
                    &IntentContext::user_request(),
                )?
                .windows
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

    pub fn capture_workspace_observation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<WorkspaceObservationCaptureResult> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(CaptureWorkspaceObservation)
    }

    /// Describes what saving a context would capture. Observes nothing.
    pub fn get_saved_context_capture_scope(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<SavedContextCaptureScope> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetSavedContextCaptureScope)
    }

    /// Saves one named bounded context after the user confirmed the capture scope.
    pub fn save_workspace_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        approved_scope: String,
        handoff_note: String,
    ) -> Result<SavedContext> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            SaveWorkspaceContext::new(SaveContextRequest::new(
                workspace_id,
                name,
                approved_scope,
                handoff_note,
            )),
        )
    }

    pub fn list_saved_contexts(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<Vec<SavedContext>> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListSavedContexts { workspace_id })
    }

    pub fn get_saved_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        saved_context_id: String,
    ) -> Result<SavedContext> {
        let saved_context_id =
            SavedContextId::new(saved_context_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetSavedContext { saved_context_id })
    }

    pub fn resolve_resume_plan(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        saved_context_id: String,
    ) -> Result<ResumePlanPreview> {
        let saved_context_id =
            SavedContextId::new(saved_context_id).map_err(KernelError::Domain)?;
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ResolveResumePlan { saved_context_id })
    }

    pub fn execute_resume_plan(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        plan: ActionPlan,
        approved_plan_digest: String,
    ) -> Result<ActionOperationResult> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            ExecuteResumePlan {
                plan,
                approved_plan_digest,
            },
        )
    }

    pub fn get_latest_workspace_observation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<Option<WorkspaceObservationSnapshot>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetLatestWorkspaceObservation)
    }

    pub fn get_workspace_observation_by_id(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        pass_id: String,
    ) -> Result<Option<WorkspaceObservationSnapshot>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceObservationById::new(pass_id))
    }

    pub fn get_workspace_observation_status(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<WorkspaceObservationStatus> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceObservationStatus)
    }

    /// Explicit Manual ensure for a consumer freshness need via TriggerAuthority.
    /// Never Event/Plugin. Never silent — caller must invoke deliberately.
    pub fn ensure_observation_freshness(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        consumer_id: Option<String>,
    ) -> Result<ObservationFreshnessEnsureResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateObservationRead)?;
        let need = match consumer_id.as_deref() {
            Some(ObservationConsumerFreshnessNeed::INTELLIGENCE_CONSUMER) => {
                ObservationConsumerFreshnessNeed::for_intelligence()
            }
            _ => ObservationConsumerFreshnessNeed::for_environment(),
        };
        ObservationTriggerAuthority::ensure_for_consumer(
            &kernel.shared_database(),
            &actor,
            &intent,
            &need,
        )
    }

    /// Scheduler runtime health only — does not load observation snapshots.
    pub fn get_observation_scheduler_status(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<ObservationSchedulerStatus> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetObservationSchedulerStatus)?;
        Ok(ObservationSchedulerDiagnostics::get_status(
            kernel.observation_scheduler(),
        ))
    }

    pub fn get_latest_observation_delta(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<WorkspaceObservationDelta> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetLatestObservationDelta)
    }

    pub fn get_workspace_state(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
    ) -> Result<ProjectedWorkspaceState> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetWorkspaceState)
    }

    /// Architecture guard — Observation Layer must never execute.
    pub fn workspace_observation_attempt_execute() -> Result<()> {
        WorkspaceObservationService::attempt_execute()
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

    #[allow(clippy::too_many_arguments)]
    pub fn create_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        project_id: String,
        task_id: Option<String>,
        name: String,
        description: Option<String>,
        trigger_kind: AutomationTriggerKind,
        trigger_definition: Option<String>,
        intent_statement: String,
        required_capabilities: Vec<String>,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            CreateAutomationContract::new(
                workspace_id,
                project_id,
                task_id,
                name,
                description,
                trigger_kind,
                trigger_definition,
                intent_statement,
                required_capabilities,
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
        name: Option<String>,
        description: Option<Option<String>>,
        intent_statement: Option<String>,
        trigger_kind: Option<AutomationTriggerKind>,
        trigger_definition: Option<String>,
        required_capabilities: Option<Vec<String>>,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            UpdateAutomationContract::new(
                contract_id,
                name,
                description,
                intent_statement,
                trigger_kind,
                trigger_definition,
                required_capabilities,
            ),
        )
    }

    pub fn request_automation_contract_approval(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(RequestAutomationContractApproval::new(contract_id))
    }

    pub fn approve_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(ApproveAutomationContract::new(contract_id))
    }

    pub fn pause_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(PauseAutomationContract::new(contract_id))
    }

    pub fn resume_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(ResumeAutomationContract::new(contract_id))
    }

    pub fn revoke_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(RevokeAutomationContract::new(contract_id))
    }

    pub fn get_automation_contract(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContract> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GetAutomationContract::new(contract_id))
    }

    pub fn list_automation_contracts(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<AutomationContract>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListAutomationContracts::new(workspace_id, limit))
    }

    /// Materializes a future Intent template from an approved contract.
    /// Does not execute — caller must still enter Command Pipeline → Gateway.
    pub fn prepare_automation_contract_intent(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        contract_id: String,
    ) -> Result<AutomationContractIntentRequest> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(PrepareAutomationContractIntent::new(contract_id))
    }

    pub fn record_trigger_event(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        event_type: TriggerEventType,
        source: String,
        context: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<TriggerEvent> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            RecordTriggerEvent::new(
                workspace_id,
                event_type,
                source,
                context,
                project_id,
                task_id,
            ),
        )
    }

    pub fn evaluate_triggers(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        trigger_event_id: String,
    ) -> Result<TriggerEvaluationResult> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(EvaluateTriggers::new(trigger_event_id))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_and_evaluate_triggers(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        event_type: TriggerEventType,
        source: String,
        context: String,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<TriggerEvaluationResult> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_mutation(
            RecordAndEvaluateTriggers::new(
                workspace_id,
                event_type,
                source,
                context,
                project_id,
                task_id,
            ),
        )
    }

    pub fn list_trigger_events(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<TriggerEvent>> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(ListTriggerEvents::new(workspace_id, limit))
    }

    pub fn list_automation_intent_proposals(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        status: Option<AutomationIntentProposalStatus>,
        limit: Option<usize>,
    ) -> Result<Vec<AutomationIntentProposal>> {
        CommandPipeline::new(kernel.command_context(actor, intent)).execute_query(
            ListAutomationIntentProposals::new(workspace_id, status, limit),
        )
    }

    /// Marks a proposal accepted for review handoff — never executes.
    pub fn accept_automation_intent_proposal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        proposal_id: String,
    ) -> Result<AutomationIntentProposal> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(AcceptAutomationIntentProposal::new(proposal_id))
    }

    pub fn reject_automation_intent_proposal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        proposal_id: String,
    ) -> Result<AutomationIntentProposal> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_mutation(RejectAutomationIntentProposal::new(proposal_id))
    }

    /// Aggregate the Workspace Decision Queue (read-only aggregation; never executes).
    pub fn generate_decision_queue(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<DecisionQueue> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateDecisionQueueRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        DecisionQueueService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    pub fn mark_decision_item_viewed(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        decision_item_id: String,
    ) -> Result<DecisionItem> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionQueueWrite)?;
        DecisionQueueService::mark_viewed(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            decision_item_id,
        )
    }

    pub fn defer_decision_item(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        decision_item_id: String,
    ) -> Result<DecisionItem> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionQueueWrite)?;
        DecisionQueueService::defer(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            decision_item_id,
        )
    }

    pub fn dismiss_decision_item(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        decision_item_id: String,
    ) -> Result<DecisionItem> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionQueueWrite)?;
        DecisionQueueService::dismiss(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            decision_item_id,
        )
    }

    /// Accept delegates to source subsystems or returns a governed handoff.
    pub fn accept_decision_item(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        decision_item_id: String,
    ) -> Result<DecisionActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionQueueWrite)?;
        DecisionQueueService::accept(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            decision_item_id,
        )
    }

    pub fn reject_decision_item(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        decision_item_id: String,
    ) -> Result<DecisionActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionQueueWrite)?;
        DecisionQueueService::reject(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            decision_item_id,
        )
    }

    /// Aggregate the Workspace Activity Graph (read-only; never executes).
    pub fn generate_workspace_activity_graph(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceActivityGraph> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateActivityGraphRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceActivityGraphService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Chronological timeline slice from the same Activity Graph generate.
    pub fn get_workspace_activity_timeline(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<WorkspaceActivity>> {
        let graph = Self::generate_workspace_activity_graph(
            kernel,
            actor,
            intent,
            workspace_id,
        )?;
        let limit = limit.unwrap_or(50).min(graph.timeline.len());
        Ok(graph
            .timeline
            .into_iter()
            .rev()
            .take(limit)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect())
    }

    /// Aggregate Workspace Continuity (read-only; never executes).
    pub fn generate_workspace_continuity(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceContinuityState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateContinuityRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceContinuityService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Aggregate Workspace Attention (read-only; never executes).
    pub fn generate_workspace_attention(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceAttentionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateAttentionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceAttentionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Aggregate Decision Engine recommendations (never executes).
    pub fn generate_decision_engine(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<DecisionEngineState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateDecisionEngineRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        DecisionEngineService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Accept a Decision Engine recommendation — returns planner handoff; never executes.
    pub fn select_decision_candidate(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
    ) -> Result<DecisionEngineActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        // Handoff only — caller invokes submit_assistant_goal explicitly so Planner plans.
        DecisionEngineService::select(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
        )
    }

    pub fn dismiss_decision_candidate(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
    ) -> Result<DecisionEngineActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::dismiss(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
        )
    }

    pub fn postpone_decision_candidate(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
    ) -> Result<DecisionEngineActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::postpone(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
        )
    }

    /// Create a DE-owned DecisionScore for an accepted candidate — never ranks/selects/plans.
    pub fn score_decision_candidate(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
    ) -> Result<(
        workspace_domain::DecisionCandidateScore,
        workspace_domain::DecisionCandidate,
    )> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::score_decision_candidate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
        )
    }

    /// Project DE-owned comparative ranking of scored candidates — never selects/plans.
    pub fn rank_decision_candidates(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<workspace_domain::DecisionCandidateRanking> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateDecisionEngineRead)?;
        let state = Self::generate_decision_engine(kernel, actor, intent, workspace_id)?;
        state
            .candidate_ranking
            .ok_or_else(|| KernelError::from(workspace_domain::DecisionEngineError::NotFound))
    }

    /// Resolve DE-owned selection after ranking — never executes or planner-handoffs.
    pub fn resolve_decision_candidate_selection(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
        action: String,
        reason: String,
    ) -> Result<(
        workspace_domain::DecisionCandidateSelection,
        workspace_domain::DecisionCandidate,
    )> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::resolve_decision_candidate_selection(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
            action,
            reason,
        )
    }

    /// Issue DE-owned progression request after selection — never planner/execution.
    pub fn request_decision_candidate_progression(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
        reason: String,
    ) -> Result<(
        workspace_domain::DecisionCandidateProgressionRequest,
        workspace_domain::DecisionCandidate,
    )> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::request_decision_candidate_progression(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
            reason,
        )
    }

    /// Acknowledge DE-owned progression request — never planner/execution.
    pub fn acknowledge_decision_candidate_progression(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        candidate_id: String,
        action: String,
        reason: String,
    ) -> Result<(
        workspace_domain::DecisionCandidateProgressionAcknowledgement,
        workspace_domain::DecisionCandidate,
    )> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateDecisionEngineWrite)?;
        DecisionEngineService::acknowledge_decision_candidate_progression(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            candidate_id,
            action,
            reason,
        )
    }

    /// Architecture guard — Decision Engine must never execute.
    pub fn decision_engine_attempt_execute() -> Result<()> {
        DecisionEngineService::attempt_execute()
    }

    /// Generate / refresh Workspace Task Graph (persistent work model).
    pub fn generate_task_graph(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<TaskGraph> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateTaskGraphRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        TaskGraphService::generate(&kernel.shared_database(), &actor, workspace_id)
    }

    pub fn create_workspace_task(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        title: String,
        project_id: Option<String>,
        priority: WorkspaceTaskPriority,
    ) -> Result<WorkspaceTask> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateTaskGraphWrite)?;
        TaskGraphService::create_task(
            &kernel.shared_database(),
            &actor,
            workspace_id,
            title,
            project_id,
            priority,
        )
    }

    pub fn update_workspace_task_status(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        task_id: String,
        status: WorkspaceTaskStatus,
        explanation: Option<String>,
    ) -> Result<WorkspaceTask> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateTaskGraphWrite)?;
        TaskGraphService::update_task_status(
            &kernel.shared_database(),
            &actor,
            task_id,
            status,
            explanation,
        )
    }

    pub fn add_task_relationship(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        from_task_id: String,
        to_task_id: String,
        kind: TaskRelationshipKind,
    ) -> Result<TaskRelationship> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateTaskGraphWrite)?;
        TaskGraphService::add_relationship(
            &kernel.shared_database(),
            &actor,
            workspace_id,
            from_task_id,
            to_task_id,
            kind,
        )
    }

    pub fn remove_task_relationship(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        relationship_id: String,
    ) -> Result<()> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateTaskGraphWrite)?;
        TaskGraphService::remove_relationship(
            &kernel.shared_database(),
            &actor,
            relationship_id,
        )
    }

    pub fn validate_task_graph(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<TaskGraph> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateTaskGraphRead)?;
        TaskGraphService::validate(&kernel.shared_database(), &actor, workspace_id)
    }

    /// Planner input: open incomplete graph nodes (no duplicated planner state).
    pub fn get_task_graph_planning_inputs(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<Vec<WorkspaceTask>> {
        let graph = Self::generate_task_graph(kernel, actor, intent, workspace_id)?;
        Ok(TaskGraphService::planning_inputs(&graph))
    }

    /// Architecture guard — Task Graph must never execute.
    pub fn task_graph_attempt_execute() -> Result<()> {
        TaskGraphService::attempt_execute()
    }

    /// Aggregate Workspace Environment Model (read-only desktop + work association).
    pub fn generate_workspace_environment(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceEnvironmentState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateEnvironmentRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceEnvironmentService::generate(&kernel.shared_database(), &actor, workspace_id)
    }

    /// Architecture guard — Environment Model must never execute.
    pub fn workspace_environment_attempt_execute() -> Result<()> {
        WorkspaceEnvironmentService::attempt_execute()
    }

    /// Aggregate Workspace Composition Engine (read-only working environment meaning).
    pub fn generate_workspace_composition(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceCompositionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateCompositionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceCompositionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Composition Engine must never execute.
    pub fn workspace_composition_attempt_execute() -> Result<()> {
        WorkspaceCompositionService::attempt_execute()
    }

    /// Aggregate Workspace Purpose Model (read-only why-work-exists projection).
    pub fn generate_workspace_purpose(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspacePurposeState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GatePurposeRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspacePurposeService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Purpose Model must never execute.
    pub fn workspace_purpose_attempt_execute() -> Result<()> {
        WorkspacePurposeService::attempt_execute()
    }

    /// Aggregate Workspace Evolution Model (read-only change narrative).
    pub fn generate_workspace_evolution(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceEvolutionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateEvolutionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceEvolutionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Evolution Model must never execute.
    pub fn workspace_evolution_attempt_execute() -> Result<()> {
        WorkspaceEvolutionService::attempt_execute()
    }

    /// Aggregate Workspace Recommendation Engine (read-only next-step suggestions).
    pub fn generate_workspace_recommendation_engine(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceRecommendationEngineState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateRecommendationEngineRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceRecommendationEngineService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Recommendation Engine must never execute.
    pub fn workspace_recommendation_engine_attempt_execute() -> Result<()> {
        WorkspaceRecommendationEngineService::attempt_execute()
    }

    /// Present a Recommendation Engine candidate for human review — lifecycle only.
    pub fn present_recommendation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::present_recommendation(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Accept a Recommendation Engine candidate — records human decision only; never executes.
    pub fn accept_recommendation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::accept_recommendation(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Reject a Recommendation Engine candidate — records human decision only; never executes.
    pub fn reject_recommendation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::reject_recommendation(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Confirm desire for future Decision Engine consideration — never creates DE/intent.
    pub fn confirm_recommendation_decision(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
        confirmation_intent: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::confirm_recommendation_decision(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
            confirmation_intent,
        )
    }

    /// Decline future Decision Engine consideration — never executes.
    pub fn decline_recommendation_decision(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::decline_recommendation_decision(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Revoke prepared adapter path — reversible; never invokes adapter or creates DE objects.
    pub fn revoke_recommendation_adapter_preparation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::revoke_recommendation_adapter_preparation(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Record Decision Engine acceptance of handoff request — never transfers ownership or creates DE objects.
    pub fn accept_recommendation_decision_engine_acceptance(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::accept_recommendation_decision_engine_acceptance(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Decline Decision Engine acceptance of handoff request — never executes.
    pub fn decline_recommendation_decision_engine_acceptance(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        recommendation_id: String,
    ) -> Result<RecommendationReviewActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateRecommendationEngineWrite)?;
        WorkspaceRecommendationEngineService::decline_recommendation_decision_engine_acceptance(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            recommendation_id,
        )
    }

    /// Aggregate Workspace Operating State (read-only current-situation snapshot).
    pub fn generate_workspace_operating_state(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceOperatingState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateOperatingStateRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceOperatingStateService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Operating State must never execute.
    pub fn workspace_operating_state_attempt_execute() -> Result<()> {
        WorkspaceOperatingStateService::attempt_execute()
    }

    /// Aggregate Workspace Pattern Model (read-only recurring structures).
    pub fn generate_workspace_pattern(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspacePatternState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GatePatternRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspacePatternService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Pattern Model must never execute.
    pub fn workspace_pattern_attempt_execute() -> Result<()> {
        WorkspacePatternService::attempt_execute()
    }

    /// Aggregate Workspace Adaptation Proposals (read-only improvement suggestions).
    pub fn generate_workspace_adaptation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceAdaptationState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateAdaptationRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceAdaptationService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Mark an adaptation proposal reviewed (audit only — never executes).
    pub fn review_adaptation_proposal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        proposal_id: String,
    ) -> Result<AdaptationActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateAdaptationWrite)?;
        WorkspaceAdaptationService::review(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            proposal_id,
        )
    }

    /// Accept an adaptation — returns Intent handoff only. Never executes.
    pub fn accept_adaptation_proposal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        proposal_id: String,
    ) -> Result<AdaptationActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateAdaptationWrite)?;
        WorkspaceAdaptationService::accept(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            proposal_id,
        )
    }

    /// Reject an adaptation (audit only — never modifies workspace state).
    pub fn reject_adaptation_proposal(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        proposal_id: String,
    ) -> Result<AdaptationActionResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateAdaptationWrite)?;
        WorkspaceAdaptationService::reject(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            proposal_id,
        )
    }

    /// Architecture guard — Adaptation must never execute.
    pub fn workspace_adaptation_attempt_execute() -> Result<()> {
        WorkspaceAdaptationService::attempt_execute()
    }

    /// Aggregate Workspace Readiness Model (read-only preparedness projection).
    pub fn generate_workspace_readiness(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceReadinessState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateReadinessRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceReadinessService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
        )
    }

    /// Architecture guard — Readiness must never execute or prepare.
    pub fn workspace_readiness_attempt_execute() -> Result<()> {
        WorkspaceReadinessService::attempt_execute()
    }

    /// Project live Workspace Runtime Operator View (context/health/overview — never executes).
    pub fn generate_workspace_runtime_overview(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceRuntimeOperatorView> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateRuntimeRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent.clone(), workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceRuntimeService::project(
            &kernel.shared_database(),
            &actor,
            &intent,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Architecture guard — Runtime projection must never execute.
    pub fn workspace_runtime_attempt_execute() -> Result<()> {
        WorkspaceRuntimeService::attempt_execute()
    }

    /// Architecture guard — Intelligence must never execute.
    pub fn workspace_intelligence_attempt_execute() -> Result<()> {
        WorkspaceIntelligenceService::attempt_execute()
    }

    /// Architecture guard — Decision Queue must never execute or grant.
    pub fn decision_queue_attempt_execute() -> Result<()> {
        DecisionQueueService::attempt_execute()
    }

    /// Architecture guard — Attention must never execute.
    pub fn workspace_attention_attempt_execute() -> Result<()> {
        WorkspaceAttentionService::attempt_execute()
    }

    /// Architecture guard — Continuity must never execute.
    pub fn workspace_continuity_attempt_execute() -> Result<()> {
        WorkspaceContinuityService::attempt_execute()
    }

    /// Architecture guard — Activity Graph must never execute.
    pub fn workspace_activity_attempt_execute() -> Result<()> {
        WorkspaceActivityGraphService::attempt_execute()
    }

    /// Project Workspace Session (runtime orchestration over Intelligence — never executes).
    pub fn generate_workspace_session(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceSessionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateSessionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceSessionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two session snapshots (informational).
    pub fn compare_workspace_sessions(
        left: &WorkspaceSessionState,
        right: &WorkspaceSessionState,
    ) -> WorkspaceSessionComparison {
        WorkspaceSessionService::compare(left, right)
    }

    /// Architecture guard — Session must never execute, prepare, or restore.
    pub fn workspace_session_attempt_execute() -> Result<()> {
        WorkspaceSessionService::attempt_execute()
    }

    /// Project Workspace Experience (presentation over Session — never executes).
    pub fn generate_workspace_experience(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceExperienceState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateExperienceRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceExperienceService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two experience snapshots (informational).
    pub fn compare_workspace_experiences(
        left: &WorkspaceExperienceState,
        right: &WorkspaceExperienceState,
    ) -> WorkspaceExperienceComparison {
        WorkspaceExperienceService::compare(left, right)
    }

    /// Architecture guard — Experience must never execute or authorize.
    pub fn workspace_experience_attempt_execute() -> Result<()> {
        WorkspaceExperienceService::attempt_execute()
    }

    /// Project Workspace Work Context (semantic kind-of-work over Session/Experience/Intelligence).
    pub fn generate_workspace_work_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceWorkContextState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkContextEngineRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceWorkContextService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two work-context snapshots (informational).
    pub fn compare_workspace_work_contexts(
        left: &WorkspaceWorkContextState,
        right: &WorkspaceWorkContextState,
    ) -> WorkspaceWorkContextComparison {
        WorkspaceWorkContextService::compare(left, right)
    }

    /// Validate a work-context snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_work_context(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceWorkContextState,
    ) -> Result<WorkspaceWorkContextValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateWorkContextEngineRead)?;
        WorkspaceWorkContextService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Work Context must never execute or authorize.
    pub fn workspace_work_context_attempt_execute() -> Result<()> {
        WorkspaceWorkContextService::attempt_execute()
    }

    /// Project Workspace Navigation (interaction paths over understanding — never executes).
    pub fn generate_workspace_navigation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceNavigationState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateNavigationRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceNavigationService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two navigation snapshots (informational).
    pub fn compare_workspace_navigation(
        left: &WorkspaceNavigationState,
        right: &WorkspaceNavigationState,
    ) -> WorkspaceNavigationComparison {
        WorkspaceNavigationService::compare(left, right)
    }

    /// Validate a navigation snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_navigation(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceNavigationState,
    ) -> Result<WorkspaceNavigationValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateNavigationRead)?;
        WorkspaceNavigationService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Navigation must never execute or authorize.
    pub fn workspace_navigation_attempt_execute() -> Result<()> {
        WorkspaceNavigationService::attempt_execute()
    }

    /// Project Workspace Milestones (coordination outcomes — never plans or executes).
    pub fn generate_workspace_milestones(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceMilestoneState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateMilestoneRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceMilestoneService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two milestone snapshots (informational).
    pub fn compare_workspace_milestones(
        left: &WorkspaceMilestoneState,
        right: &WorkspaceMilestoneState,
    ) -> WorkspaceMilestoneComparison {
        WorkspaceMilestoneService::compare(left, right)
    }

    /// Validate a milestone snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_milestones(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceMilestoneState,
    ) -> Result<WorkspaceMilestoneValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateMilestoneRead)?;
        WorkspaceMilestoneService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Milestones must never execute or authorize.
    pub fn workspace_milestones_attempt_execute() -> Result<()> {
        WorkspaceMilestoneService::attempt_execute()
    }

    /// Project Workspace Working Style (operating patterns — never profiles or executes).
    pub fn generate_workspace_working_style(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceWorkingStyleState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkingStyleRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceWorkingStyleService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two working-style snapshots (informational).
    pub fn compare_workspace_working_styles(
        left: &WorkspaceWorkingStyleState,
        right: &WorkspaceWorkingStyleState,
    ) -> WorkspaceWorkingStyleComparison {
        WorkspaceWorkingStyleService::compare(left, right)
    }

    /// Validate a working-style snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_working_style(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceWorkingStyleValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateWorkingStyleRead)?;
        WorkspaceWorkingStyleService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Working Style must never execute or authorize.
    pub fn workspace_working_style_attempt_execute() -> Result<()> {
        WorkspaceWorkingStyleService::attempt_execute()
    }

    /// Project Workspace Transitions (movement explanation — never restores or executes).
    pub fn generate_workspace_transitions(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceTransitionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateTransitionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceTransitionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two transition snapshots (informational).
    pub fn compare_workspace_transitions(
        left: &WorkspaceTransitionState,
        right: &WorkspaceTransitionState,
    ) -> WorkspaceTransitionComparison {
        WorkspaceTransitionService::compare(left, right)
    }

    /// Validate a transition snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_transitions(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceTransitionState,
    ) -> Result<WorkspaceTransitionValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateTransitionRead)?;
        WorkspaceTransitionService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Transitions must never execute or authorize.
    pub fn workspace_transitions_attempt_execute() -> Result<()> {
        WorkspaceTransitionService::attempt_execute()
    }

    /// Project Workspace Interactions (opportunities — never executes).
    pub fn generate_workspace_interactions(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceInteractionState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkspaceInteractionRead)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent.clone(),
            workspace_id.clone(),
        )?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceInteractionService::generate(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
        )
    }

    /// Compare two interaction snapshots (informational).
    pub fn compare_workspace_interactions(
        left: &WorkspaceInteractionState,
        right: &WorkspaceInteractionState,
    ) -> WorkspaceInteractionComparison {
        WorkspaceInteractionService::compare(left, right)
    }

    /// Validate an interaction snapshot (Operator / IPC) and audit.
    pub fn validate_workspace_interactions(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceInteractionState,
    ) -> Result<WorkspaceInteractionValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateWorkspaceInteractionRead)?;
        WorkspaceInteractionService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Select an interaction — returns Intent handoff only. Never executes.
    pub fn select_workspace_interaction(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        interaction_id: String,
    ) -> Result<InteractionSelectResult> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_mutation(GateWorkspaceInteractionWrite)?;
        let workspace =
            Self::get_workspace(kernel, actor.clone(), intent, workspace_id.clone())?;
        let health = kernel.health();
        WorkspaceInteractionService::select(
            &kernel.shared_database(),
            &actor,
            &kernel.orchestrated_plans(),
            &kernel.assistant_workflows(),
            workspace_id,
            workspace.name,
            health.status.clone(),
            interaction_id,
        )
    }

    /// Architecture guard — Interactions must never execute or authorize.
    pub fn workspace_interactions_attempt_execute() -> Result<()> {
        WorkspaceInteractionService::attempt_execute()
    }

    /// Create a durable Workspace Environment Profile (user-owned; never executes).
    pub fn create_workspace_profile(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        name: String,
        description: String,
        members: Vec<WorkspaceProfileMemberInput>,
    ) -> Result<WorkspaceProfile> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_mutation(GateWorkspaceProfileWrite)?;
        let _ = Self::get_workflow_context(
            kernel,
            actor.clone(),
            intent,
            workspace_id.clone(),
        )?;
        WorkspaceProfileService::create(
            &kernel.shared_database(),
            &actor,
            workspace_id,
            name,
            description,
            members,
        )
    }

    /// Update a Workspace Environment Profile (never executes).
    pub fn update_workspace_profile(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        profile_id: String,
        name: Option<String>,
        description: Option<String>,
        status: Option<WorkspaceProfileStatus>,
        members: Option<Vec<WorkspaceProfileMemberInput>>,
    ) -> Result<WorkspaceProfile> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_mutation(GateWorkspaceProfileWrite)?;
        WorkspaceProfileService::update(
            &kernel.shared_database(),
            &actor,
            profile_id,
            name,
            description,
            status,
            members,
        )
    }

    /// List profiles for a workspace.
    pub fn list_workspace_profiles(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        limit: Option<usize>,
    ) -> Result<Vec<WorkspaceProfile>> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkspaceProfileRead)?;
        let _ = Self::get_workflow_context(kernel, actor, intent, workspace_id.clone())?;
        WorkspaceProfileService::list(
            &kernel.shared_database(),
            workspace_id,
            limit.unwrap_or(50),
        )
    }

    /// Get one profile by id.
    pub fn get_workspace_profile(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        profile_id: String,
    ) -> Result<WorkspaceProfile> {
        CommandPipeline::new(kernel.command_context(actor, intent))
            .execute_query(GateWorkspaceProfileRead)?;
        WorkspaceProfileService::get(&kernel.shared_database(), profile_id)
    }

    /// Generate profile state + comparisons against current workspace (informational).
    pub fn generate_workspace_profile_state(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceProfileState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkspaceProfileRead)?;
        let intelligence = Self::generate_workspace_intelligence(
            kernel,
            actor.clone(),
            intent,
            workspace_id,
        )?;
        WorkspaceProfileService::generate_state(
            &kernel.shared_database(),
            &actor,
            &intelligence,
        )
    }

    /// Compare one profile against current workspace (informational).
    pub fn compare_workspace_profile(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
        profile_id: String,
    ) -> Result<WorkspaceProfileComparison> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateWorkspaceProfileRead)?;
        let intelligence = Self::generate_workspace_intelligence(
            kernel,
            actor.clone(),
            intent,
            workspace_id,
        )?;
        WorkspaceProfileService::compare_one(
            &kernel.shared_database(),
            &actor,
            &intelligence,
            profile_id,
        )
    }

    /// Compare two profile state snapshots (informational).
    pub fn compare_workspace_profile_states(
        left: &WorkspaceProfileState,
        right: &WorkspaceProfileState,
    ) -> WorkspaceProfileStateComparison {
        WorkspaceProfileService::compare_states(left, right)
    }

    /// Validate a profile state snapshot.
    pub fn validate_workspace_profile_state(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        state: &WorkspaceProfileState,
    ) -> Result<WorkspaceProfileValidation> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent))
            .execute_query(GateWorkspaceProfileRead)?;
        WorkspaceProfileService::validate_and_audit(
            &kernel.shared_database(),
            &actor,
            state,
        )
    }

    /// Architecture guard — Profiles must never execute or authorize.
    pub fn workspace_profiles_attempt_execute() -> Result<()> {
        WorkspaceProfileService::attempt_execute()
    }

    /// Read-only workspace intelligence aggregation. Capability-gated; never executes.
    pub fn generate_workspace_intelligence(
        kernel: &WorkspaceKernel,
        actor: ActorContext,
        intent: IntentContext,
        workspace_id: String,
    ) -> Result<WorkspaceIntelligenceState> {
        CommandPipeline::new(kernel.command_context(actor.clone(), intent.clone()))
            .execute_query(GateIntelligenceRead)?;
        // Also require workflow-context read (same capability class) before aggregating.
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
