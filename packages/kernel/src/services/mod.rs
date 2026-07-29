//! Foundation service ownership for the Platform Kernel.

mod action_catalog;
mod ai_assistant;
mod ai_evaluation;
mod ai_memory;
mod ai_model_provider;
mod ai_orchestration;
mod ai_participation;
mod ai_personalization;
mod ai_planning;
mod analytics;
mod application;
mod application_launch;
mod audit;
mod automation_contract;
mod capture_coordinator;
mod configuration;
mod context;
mod database;
mod decision_engine;
mod decision_queue;
mod discovery;
mod execution_cancellation;
mod execution_context;
mod execution_guard;
mod execution_lifecycle;
mod execution_outcome;
mod execution_reconciliation;
pub(crate) mod explanation_catalog;
pub(crate) mod explanation_resolver;
mod graph;
mod intent_execution;
mod layout;
mod observation;
mod observation_delta;
mod observation_event_gateway;
mod observation_refresh_policy;
mod observation_scheduled_trigger;
mod observation_scheduler;
mod observation_startup_trigger;
mod observation_trigger_admission;
mod observation_trigger_authority;
mod permission_approval;
mod policy_governance;
mod projection;
mod registry;
mod resilience_validation;
mod suggestion;
mod suggestion_intent;
mod suggestion_lifecycle;
mod task_graph;
mod trigger_evaluator;
mod widget;
mod workspace;
mod workspace_activity;
mod workspace_adaptation;
mod workspace_assistant_context;
mod workspace_assistant_explanation;
mod workspace_assistant_interaction;
mod workspace_assistant_personalisation;
mod workspace_assistant_retrieval;
mod workspace_assistant_surface;
mod workspace_attention;
mod workspace_cognitive_agent_cast;
mod workspace_cognitive_autonomy;
mod workspace_cognitive_graph;
mod workspace_cognitive_model;
mod workspace_cognitive_orchestration;
mod workspace_composition;
mod workspace_contextual_understanding;
mod workspace_continuity;
mod workspace_cross_intelligence;
mod workspace_decision_support;
mod workspace_environment;
mod workspace_evidence_completeness;
mod workspace_evidence_consistency;
mod workspace_evidence_coverage;
mod workspace_evidence_dependency;
mod workspace_evidence_freshness;
mod workspace_evidence_navigation;
mod workspace_evidence_reliability;
mod workspace_evidence_trace;
mod workspace_evolution;
mod workspace_experience;
mod workspace_explanation;
mod workspace_historical_reconstruction;
mod workspace_insight_coordination;
mod workspace_intelligence;
mod workspace_intelligence_hub;
mod workspace_intent;
mod workspace_interaction;
mod workspace_knowledge_integration;
mod workspace_knowledge_synthesis;
mod workspace_learning_adaptation;
mod workspace_milestone;
mod workspace_navigation;
mod workspace_observation;
mod workspace_operating_state;
mod workspace_pattern;
mod workspace_planning;
mod workspace_profile;
mod workspace_purpose;
mod workspace_readiness;
mod workspace_reasoning_memory;
mod workspace_recommendation;
mod workspace_runtime;
mod workspace_scope;
mod workspace_semantic_query;
mod workspace_session;
mod workspace_state_composition;
mod workspace_state_engine;
mod workspace_temporal_intelligence;
mod workspace_transition;
mod workspace_work_context;
mod workspace_working_style;
mod zone;

pub use action_catalog::ActionCatalogService;
pub(crate) use ai_assistant::{AiAssistantService, AssistantWorkflowStore};
pub(crate) use ai_evaluation::AiEvaluationService;
pub(crate) use ai_memory::AiMemoryService;
pub(crate) use ai_model_provider::{
    DeterministicModelProvider, EchoModelProvider, ModelProviderRegistry, ModelProviderService,
    UnavailableModelProvider,
};
pub(crate) use ai_orchestration::{AiOrchestrationService, OrchestratedPlanStore};
pub(crate) use ai_participation::AiParticipationService;
pub(crate) use ai_personalization::AiPersonalizationService;
pub(crate) use ai_planning::AiPlanningService;
pub use analytics::WorkspaceAnalyticsService;
pub use application::ApplicationService;
pub(crate) use application_launch::ApplicationLaunchService;
pub use audit::AuditService;
pub(crate) use automation_contract::AutomationContractService;
#[cfg(test)]
pub(crate) use capture_coordinator::observation_flight_test_lock;
pub(crate) use capture_coordinator::{
    CaptureCoordinator, CaptureCoordinatorResult, CaptureLifecycleState,
};
pub use configuration::ConfigurationService;
pub use context::WorkspaceContextService;
pub use database::DatabaseServiceHandle;
pub(crate) use decision_engine::DecisionEngineService;
pub(crate) use decision_queue::DecisionQueueService;
pub use discovery::CapabilityResolver;
pub use execution_cancellation::ExecutionCancellationService;
pub use execution_context::ExecutionContextService;
pub use execution_guard::ExecutionGuardService;
pub(crate) use execution_lifecycle::ExecutionLifecycleService;
pub use execution_outcome::ExecutionOutcomeService;
pub use execution_reconciliation::ExecutionReconciliationService;
pub use graph::GraphService;
pub use intent_execution::GovernedIntentExecutionService;
pub use layout::LayoutService;
pub use observation::ObservationService;
pub(crate) use observation_delta::ObservationDeltaService;
pub(crate) use observation_event_gateway::ObservationEventGateway;
pub(crate) use observation_refresh_policy::ObservationRefreshPolicyService;
pub(crate) use observation_scheduled_trigger::ObservationScheduledTrigger;
pub(crate) use observation_scheduler::{ObservationScheduler, ObservationSchedulerDiagnostics};
pub(crate) use observation_startup_trigger::ObservationStartupTrigger;
pub(crate) use observation_trigger_admission::ObservationTriggerAdmissionPolicy;
pub(crate) use observation_trigger_authority::{
    ObservationTriggerAuthority, ObservationTriggerDecision,
};
pub(crate) use permission_approval::PermissionApprovalService;
pub(crate) use policy_governance::PolicyGovernanceService;
pub use projection::WorkspaceProjectionService;
pub use registry::{RegisteredService, ServiceRegistry, ServiceStatus};
pub(crate) use resilience_validation::{
    validate_accept_emits_no_intake, validate_adapter_preparation_boundary,
    validate_candidate_creation_bounded, validate_candidate_ranking_only,
    validate_candidate_score_only, validate_candidate_score_outcome_unchanged,
    validate_candidate_score_unchanged, validate_candidate_selection_bounded,
    validate_confirmation_post_action, validate_decision_boundary_constraints,
    validate_decision_confirmation_non_authoritative, validate_decision_context_boundary,
    validate_decision_engine_state_integrity, validate_decision_readiness_boundary,
    validate_decline_clears_intake_pipeline, validate_engine_acceptance_boundary,
    validate_evaluation_origin_contract_only, validate_evaluation_resolution_only,
    validate_explanation_view_non_authoritative, validate_handoff_request_boundary,
    validate_intake_candidate_phase, validate_intake_compatibility_boundary,
    validate_intake_disposition_only, validate_intake_evaluation_phase,
    validate_intake_inspection_boundary, validate_intake_package_seal_boundary,
    validate_intake_proceed_denial_boundary, validate_intake_receipt_observational,
    validate_intake_request_non_authoritative, validate_lifecycle_integration_only,
    validate_new_intake_candidate_bootstrap, validate_progression_acknowledgement_only,
    validate_progression_request_bounded, validate_provenance_retained,
};
pub use suggestion::SuggestionService;
pub use suggestion_intent::SuggestionIntentService;
pub use suggestion_lifecycle::SuggestionLifecycleService;
pub(crate) use task_graph::TaskGraphService;
pub(crate) use trigger_evaluator::{list_rejection_summaries, TriggerEvaluatorService};
pub use widget::WidgetService;
pub use workspace::WorkspaceService;
pub(crate) use workspace_activity::WorkspaceActivityGraphService;
pub(crate) use workspace_adaptation::WorkspaceAdaptationService;
pub(crate) use workspace_assistant_context::WorkspaceAssistantContextService;
pub(crate) use workspace_assistant_explanation::WorkspaceAssistantExplanationService;
pub(crate) use workspace_assistant_interaction::WorkspaceAssistantInteractionService;
pub(crate) use workspace_assistant_personalisation::WorkspaceAssistantPersonalisationService;
pub(crate) use workspace_assistant_retrieval::WorkspaceAssistantRetrievalService;
pub(crate) use workspace_assistant_surface::WorkspaceAssistantSurfaceService;
pub(crate) use workspace_attention::WorkspaceAttentionService;
pub(crate) use workspace_cognitive_agent_cast::WorkspaceCognitiveAgentCastService;
pub(crate) use workspace_cognitive_autonomy::WorkspaceCognitiveAutonomyService;
pub(crate) use workspace_cognitive_graph::WorkspaceCognitiveGraphService;
pub(crate) use workspace_cognitive_model::WorkspaceCognitiveModelService;
pub(crate) use workspace_cognitive_orchestration::WorkspaceCognitiveOrchestrationService;
pub(crate) use workspace_composition::WorkspaceCompositionService;
pub(crate) use workspace_contextual_understanding::WorkspaceContextualUnderstandingService;
pub(crate) use workspace_continuity::WorkspaceContinuityService;
pub(crate) use workspace_cross_intelligence::WorkspaceCrossIntelligenceService;
pub(crate) use workspace_decision_support::WorkspaceDecisionSupportService;
pub(crate) use workspace_environment::WorkspaceEnvironmentService;
pub(crate) use workspace_evidence_completeness::WorkspaceEvidenceCompletenessService;
pub(crate) use workspace_evidence_consistency::WorkspaceEvidenceConsistencyService;
pub(crate) use workspace_evidence_coverage::WorkspaceEvidenceCoverageService;
pub(crate) use workspace_evidence_dependency::WorkspaceEvidenceDependencyService;
pub(crate) use workspace_evidence_freshness::WorkspaceEvidenceFreshnessService;
pub(crate) use workspace_evidence_navigation::WorkspaceEvidenceNavigationService;
pub(crate) use workspace_evidence_reliability::WorkspaceEvidenceReliabilityService;
pub(crate) use workspace_evidence_trace::WorkspaceEvidenceTraceService;
pub(crate) use workspace_evolution::WorkspaceEvolutionService;
pub(crate) use workspace_experience::WorkspaceExperienceService;
pub(crate) use workspace_explanation::WorkspaceExplanationService;
pub(crate) use workspace_historical_reconstruction::WorkspaceHistoricalReconstructionService;
pub(crate) use workspace_insight_coordination::WorkspaceInsightCoordinationService;
pub(crate) use workspace_intelligence::WorkspaceIntelligenceService;
pub(crate) use workspace_intelligence_hub::WorkspaceIntelligenceHubService;
pub(crate) use workspace_intent::WorkspaceIntentService;
pub(crate) use workspace_interaction::WorkspaceInteractionService;
pub(crate) use workspace_knowledge_integration::WorkspaceKnowledgeIntegrationService;
pub(crate) use workspace_knowledge_synthesis::WorkspaceKnowledgeSynthesisService;
pub(crate) use workspace_learning_adaptation::WorkspaceLearningAdaptationService;
pub(crate) use workspace_milestone::WorkspaceMilestoneService;
pub(crate) use workspace_navigation::WorkspaceNavigationService;
pub use workspace_observation::WorkspaceObservationCaptureResult;
pub(crate) use workspace_observation::WorkspaceObservationService;
pub(crate) use workspace_operating_state::WorkspaceOperatingStateService;
pub(crate) use workspace_pattern::WorkspacePatternService;
pub(crate) use workspace_planning::WorkspacePlanningService;
pub(crate) use workspace_profile::WorkspaceProfileService;
pub(crate) use workspace_purpose::WorkspacePurposeService;
pub(crate) use workspace_readiness::WorkspaceReadinessService;
pub(crate) use workspace_reasoning_memory::WorkspaceReasoningMemoryService;
pub(crate) use workspace_recommendation::WorkspaceRecommendationEngineService;
pub(crate) use workspace_runtime::WorkspaceRuntimeService;
pub(crate) use workspace_scope::{approval_belongs_to_workspace, plan_belongs_to_workspace};
pub(crate) use workspace_semantic_query::WorkspaceSemanticQueryService;
pub(crate) use workspace_session::WorkspaceSessionService;
pub(crate) use workspace_state_composition::WorkspaceStateCompositionService;
pub(crate) use workspace_state_engine::WorkspaceStateEngine;
pub(crate) use workspace_temporal_intelligence::WorkspaceTemporalIntelligenceService;
pub(crate) use workspace_transition::WorkspaceTransitionService;
pub(crate) use workspace_work_context::WorkspaceWorkContextService;
pub(crate) use workspace_working_style::WorkspaceWorkingStyleService;
pub use zone::ZoneService;
