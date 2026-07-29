mod accept_suggestion;
#[cfg(test)]
mod action_catalog_tests;
#[cfg(test)]
mod ai_assistant_product_tests;
#[cfg(test)]
mod ai_assistant_tests;
#[cfg(test)]
mod ai_evaluation_tests;
#[cfg(test)]
mod ai_memory_tests;
#[cfg(test)]
mod ai_model_provider_tests;
#[cfg(test)]
mod ai_orchestration_tests;
#[cfg(test)]
mod ai_participation_tests;
#[cfg(test)]
mod ai_personalization_tests;
#[cfg(test)]
mod ai_planning_tests;
#[cfg(test)]
mod analytics_tests;
mod application;
mod automation_contract;
#[cfg(test)]
mod automation_contract_tests;
mod automation_trigger;
#[cfg(test)]
mod automation_trigger_tests;
mod context;
#[cfg(test)]
mod context_tests;
mod create_suggestion_intent_request;
mod create_workspace;
mod decide_approval;
mod decision_engine;
#[cfg(test)]
mod decision_engine_candidate_creation_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_creation_request_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_evaluation_origin_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_evaluation_resolution_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_lifecycle_integration_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_progression_acknowledgement_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_progression_request_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_ranking_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_score_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_selection_contract_tests;
#[cfg(test)]
mod decision_engine_intake_assessment_contract_tests;
#[cfg(test)]
mod decision_engine_intake_candidate_contract_tests;
#[cfg(test)]
mod decision_engine_intake_candidate_lifecycle_contract_tests;
#[cfg(test)]
mod decision_engine_intake_disposition_contract_tests;
#[cfg(test)]
mod decision_engine_intake_eligibility_contract_tests;
#[cfg(test)]
mod decision_engine_intake_evaluation_contract_tests;
#[cfg(test)]
mod decision_engine_intake_promotion_boundary_contract_tests;
#[cfg(test)]
mod decision_engine_intake_receipt_contract_tests;
#[cfg(test)]
mod decision_engine_tests;
mod decision_queue;
#[cfg(test)]
mod decision_queue_tests;
#[cfg(test)]
mod discovery_tests;
mod execute_intent_request;
#[cfg(test)]
mod execution_lifecycle_tests;
#[cfg(test)]
mod execution_outcome_tests;
mod get_action_catalog;
mod get_actor_capabilities;
mod get_ai_evaluation_history;
mod get_audit_history;
mod get_execution_outcomes;
mod get_execution_state;
mod get_execution_states;
mod get_observations;
mod get_permission_approvals;
mod get_suggestion_lifecycle;
mod get_suggestions;
mod get_workspace;
mod get_workspace_context;
mod get_workspace_metrics;
mod get_workspace_snapshot;
#[cfg(test)]
mod governance_failure_tests;
mod handler;
mod historical_reconstruction;
#[cfg(test)]
mod historical_reconstruction_tests;
mod initialize;
#[cfg(test)]
mod intent_execution_tests;
#[cfg(test)]
mod intent_tests;
mod launch_application;
mod layout;
#[cfg(test)]
mod layout_tests;
mod memory;
mod model_provider;
#[cfg(test)]
mod observation_tests;
#[cfg(test)]
mod operational_recovery_tests;
#[cfg(test)]
mod permission_approval_tests;
mod personalization;
mod pipeline;
#[cfg(test)]
mod platform_coherence_tests;
mod policy_governance;
#[cfg(test)]
mod policy_governance_tests;
#[cfg(test)]
mod projection_tests;
#[cfg(test)]
mod recovery_invariant_tests;
mod reject_suggestion;
mod request_execution_cancellation;
#[cfg(test)]
mod resilience_tests;
mod resource;
#[cfg(test)]
mod resource_tests;
#[cfg(test)]
mod suggestion_intent_tests;
#[cfg(test)]
mod suggestion_lifecycle_tests;
#[cfg(test)]
mod suggestion_tests;
mod task_graph;
#[cfg(test)]
mod task_graph_tests;
mod temporal_intelligence;
#[cfg(test)]
mod temporal_intelligence_tests;
mod r#trait;
mod update_settings;
mod widget;
mod workspace_activity;
#[cfg(test)]
mod workspace_activity_tests;
mod workspace_adaptation;
#[cfg(test)]
mod workspace_adaptation_governance_contract_tests;
#[cfg(test)]
mod workspace_adaptation_review_contract_tests;
#[cfg(test)]
mod workspace_adaptation_tests;
mod workspace_assistant_context;
mod workspace_assistant_explanation;
mod workspace_assistant_retrieval;
#[cfg(test)]
mod workspace_assistant_context_tests;
#[cfg(test)]
mod workspace_assistant_explanation_tests;
#[cfg(test)]
mod workspace_assistant_retrieval_tests;
mod workspace_assistant_surface;
#[cfg(test)]
mod workspace_assistant_surface_tests;
mod workspace_attention;
#[cfg(test)]
mod workspace_attention_tests;
#[cfg(test)]
mod workspace_cognition_pipeline_contract_tests;
mod workspace_cognitive_agent_cast;
#[cfg(test)]
mod workspace_cognitive_agent_cast_tests;
mod workspace_cognitive_autonomy;
#[cfg(test)]
mod workspace_cognitive_autonomy_tests;
mod workspace_cognitive_graph;
#[cfg(test)]
mod workspace_cognitive_graph_tests;
mod workspace_cognitive_model;
#[cfg(test)]
mod workspace_cognitive_model_tests;
mod workspace_cognitive_orchestration;
#[cfg(test)]
mod workspace_cognitive_orchestration_tests;
mod workspace_composition;
#[cfg(test)]
mod workspace_composition_tests;
mod workspace_contextual_understanding;
#[cfg(test)]
mod workspace_contextual_understanding_tests;
mod workspace_continuity;
#[cfg(test)]
mod workspace_continuity_tests;
#[cfg(test)]
mod workspace_controlled_change_contract_tests;
mod workspace_cross_intelligence;
#[cfg(test)]
mod workspace_cross_intelligence_tests;
mod workspace_decision_support;
#[cfg(test)]
mod workspace_decision_support_tests;
mod workspace_environment;
#[cfg(test)]
mod workspace_environment_tests;
mod workspace_evidence_completeness;
#[cfg(test)]
mod workspace_evidence_completeness_tests;
mod workspace_evidence_consistency;
mod workspace_evidence_consistency_tests;
mod workspace_evidence_coverage;
mod workspace_evidence_coverage_tests;
mod workspace_evidence_dependency;
mod workspace_evidence_dependency_tests;
mod workspace_evidence_freshness;
#[cfg(test)]
mod workspace_evidence_freshness_tests;
mod workspace_evidence_navigation;
mod workspace_evidence_navigation_tests;
mod workspace_evidence_reliability;
#[cfg(test)]
mod workspace_evidence_reliability_tests;
mod workspace_evidence_trace;
mod workspace_evidence_trace_tests;
mod workspace_evolution;
#[cfg(test)]
mod workspace_evolution_tests;
mod workspace_experience;
#[cfg(test)]
mod workspace_experience_contract_tests;
mod workspace_experience_tests;
mod workspace_explanation;
#[cfg(test)]
mod workspace_explanation_tests;
#[cfg(test)]
mod workspace_governance_archive_contract_tests;
#[cfg(test)]
mod workspace_governance_compatibility_contract_tests;
#[cfg(test)]
mod workspace_governance_compliance_contract_tests;
#[cfg(test)]
mod workspace_governance_conditions_contract_tests;
#[cfg(test)]
mod workspace_governance_conflict_contract_tests;
#[cfg(test)]
mod workspace_governance_consolidation_contract_tests;
#[cfg(test)]
mod workspace_governance_dashboard_contract_tests;
#[cfg(test)]
mod workspace_governance_decision_package_contract_tests;
#[cfg(test)]
mod workspace_governance_delegation_contract_tests;
#[cfg(test)]
mod workspace_governance_evidence_contract_tests;
#[cfg(test)]
mod workspace_governance_export_contract_tests;
#[cfg(test)]
mod workspace_governance_failure_contract_tests;
#[cfg(test)]
mod workspace_governance_integrity_contract_tests;
#[cfg(test)]
mod workspace_governance_ledger_contract_tests;
#[cfg(test)]
mod workspace_governance_lifecycle_contract_tests;
#[cfg(test)]
mod workspace_governance_metrics_contract_tests;
#[cfg(test)]
mod workspace_governance_notifications_contract_tests;
#[cfg(test)]
mod workspace_governance_policy_contract_tests;
#[cfg(test)]
mod workspace_governance_reporting_contract_tests;
#[cfg(test)]
mod workspace_governance_review_workflow_contract_tests;
#[cfg(test)]
mod workspace_governance_risk_contract_tests;
#[cfg(test)]
mod workspace_governance_workspace_contract_tests;
mod workspace_insight_coordination;
#[cfg(test)]
mod workspace_insight_coordination_tests;
mod workspace_intelligence;
mod workspace_intelligence_hub;
#[cfg(test)]
mod workspace_intelligence_hub_tests;
#[cfg(test)]
mod workspace_intelligence_tests;
mod workspace_intent;
mod workspace_interaction;
mod workspace_interaction_tests;
mod workspace_knowledge_integration;
#[cfg(test)]
mod workspace_knowledge_integration_tests;
mod workspace_knowledge_synthesis;
#[cfg(test)]
mod workspace_knowledge_synthesis_tests;
mod workspace_learning_adaptation;
#[cfg(test)]
mod workspace_learning_adaptation_tests;
mod workspace_milestone;
mod workspace_milestone_tests;
mod workspace_navigation;
mod workspace_navigation_tests;
mod workspace_observation;
#[cfg(test)]
mod workspace_observation_freshness_wiring_contract_tests;
mod workspace_observation_tests;
mod workspace_operating_state;
#[cfg(test)]
mod workspace_operating_state_tests;
mod workspace_pattern;
#[cfg(test)]
mod workspace_pattern_tests;
mod workspace_planning;
#[cfg(test)]
mod workspace_planning_tests;
mod workspace_profile;
mod workspace_profile_tests;
#[cfg(test)]
#[cfg(test)]
mod workspace_publication_safety_contract_tests;
mod workspace_purpose;
#[cfg(test)]
mod workspace_purpose_tests;
mod workspace_readiness;
mod workspace_readiness_tests;
mod workspace_reasoning_memory;
#[cfg(test)]
mod workspace_reasoning_memory_tests;
mod workspace_recommendation;
#[cfg(test)]
mod workspace_recommendation_decision_boundary_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_confirmation_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_context_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_engine_acceptance_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_handoff_request_contract_tests;
mod workspace_recommendation_decision_intake_adapter_preparation_contract_tests;
mod workspace_recommendation_decision_intake_compatibility_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_intake_contract_tests;
mod workspace_recommendation_decision_intake_inspection_contract_tests;
mod workspace_recommendation_decision_intake_package_seal_contract_tests;
mod workspace_recommendation_decision_intake_proceed_denial_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_readiness_contract_tests;
#[cfg(test)]
mod workspace_recommendation_lifecycle_contract_tests;
#[cfg(test)]
mod workspace_recommendation_outcome_contract_tests;
#[cfg(test)]
mod workspace_recommendation_provenance_contract_tests;
#[cfg(test)]
mod workspace_recommendation_tests;
mod workspace_runtime;
#[cfg(test)]
mod workspace_runtime_diagnostic_closure_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_consolidation_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_consumption_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_continuity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_evolution_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_integrity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_layering_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_maturity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_trust_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostics_contract_tests;
#[cfg(test)]
mod workspace_runtime_integration_contract_tests;
#[cfg(test)]
mod workspace_runtime_projection_contract_tests;
mod workspace_semantic_query;
mod workspace_semantic_query_tests;
mod workspace_session;
mod workspace_session_tests;
mod workspace_state;
mod workspace_state_envelope;
#[cfg(test)]
mod workspace_state_envelope_tests;
mod workspace_transition;
mod workspace_transition_tests;
mod workspace_work_context;
mod workspace_work_context_tests;
mod workspace_working_style;
mod workspace_working_style_tests;
mod zone;

pub use accept_suggestion::AcceptSuggestion;
pub use application::{CreateApplication, DeleteApplication, GetApplication};
pub use automation_contract::{
    ApproveAutomationContract, CreateAutomationContract, GetAutomationContract,
    ListAutomationContracts, PauseAutomationContract, PrepareAutomationContractIntent,
    RequestAutomationContractApproval, ResumeAutomationContract, RevokeAutomationContract,
    UpdateAutomationContract,
};
pub use automation_trigger::{
    AcceptAutomationIntentProposal, EvaluateTriggers, ListAutomationIntentProposals,
    ListTriggerEvents, RecordAndEvaluateTriggers, RecordTriggerEvent,
    RejectAutomationIntentProposal,
};
pub use context::CommandContext;
pub use create_suggestion_intent_request::CreateSuggestionIntentRequest;
pub use create_workspace::CreateWorkspace;
pub use decide_approval::DecideApproval;
pub use decision_queue::{GateDecisionQueueRead, GateDecisionQueueWrite};
pub use execute_intent_request::ExecuteIntentRequest;
pub use get_action_catalog::GetActionCatalog;
pub use get_actor_capabilities::GetActorCapabilities;
pub use get_ai_evaluation_history::GetAiEvaluationHistory;
pub use get_audit_history::GetAuditHistory;
pub use get_execution_outcomes::GetExecutionOutcomes;
pub use get_execution_state::GetExecutionState;
pub use get_execution_states::GetExecutionStates;
pub use get_observations::GetObservations;
pub use get_permission_approvals::GetPermissionApprovals;
pub use get_suggestion_lifecycle::GetSuggestionLifecycle;
pub use get_suggestions::GetSuggestions;
pub use get_workspace::GetWorkspace;
pub use get_workspace_context::GetWorkspaceContext;
pub use get_workspace_metrics::GetWorkspaceMetrics;
pub use get_workspace_snapshot::GetWorkspaceSnapshot;
pub use handler::CommandHandler;
pub use historical_reconstruction::{
    CompareWorkspaceRevisions, ExplainHistoricalChange, GenerateHistoricalWorkspaceView,
    GetHistoricalWorkspaceSummary, GetHistoricalWorkspaceView,
};
pub use initialize::InitializeWorkspace;
pub use launch_application::LaunchApplication;
pub use layout::{
    CreateLayout, DeleteLayout, GetLayout, GetLayoutSnapshot, ResetLayout, UpdateLayout,
};
pub use memory::{
    ClearMemoryEntries, CreateMemoryEntry, DeleteMemoryEntry, GetMemoryContext, ListMemoryEntries,
};
pub use model_provider::{GetModelProviderMetadata, ListModelProviders, TestModelProviderRequest};
pub use personalization::{
    CreateUserPreference, DeleteUserPreference, GetPreferenceProfile, SetPersonalizationEnabled,
    UpdateUserPreference,
};
pub use pipeline::CommandPipeline;
pub use policy_governance::{
    ExplainGovernanceDecision, GenerateGovernanceEvaluation, GetGovernanceEvaluation,
    GetGovernanceSummary,
};
pub use r#trait::{Command, MutationCommand, QueryCommand};
pub use reject_suggestion::RejectSuggestion;
pub use request_execution_cancellation::RequestExecutionCancellation;
pub use temporal_intelligence::{
    ExplainTemporalChange, GenerateTemporalAnalysis, GetTemporalAnalysis, GetTemporalSummary,
};
pub use update_settings::UpdateSettings;
pub use widget::{CreateWidget, DeleteWidget, GetWidget};
pub use workspace_activity::GateActivityGraphRead;
pub use workspace_cognitive_agent_cast::{
    GenerateCognitiveAgentCast, GetCognitiveAgentCast, GetCognitiveAgentCastSummary,
};
pub use workspace_cognitive_autonomy::{
    GenerateCognitiveAutonomy, GetCognitiveAutonomy, GetCognitiveAutonomySummary,
};
pub use workspace_cognitive_graph::{
    GenerateCognitiveGraph, GetCognitiveGraph, GetCognitiveGraphSummary,
};
pub use workspace_cognitive_model::{
    CreateCognitiveNode, CreateCognitiveRelation, GenerateCognitiveModel, SetCognitiveFocus,
};
pub use workspace_cognitive_orchestration::{
    GenerateWorkspaceOrchestration, GetWorkspaceOrchestration, GetWorkspaceOrchestrationSummary,
};
pub use workspace_contextual_understanding::{
    ExplainWorkspaceContext, GenerateContextualWorkspaceUnderstanding,
    GetContextualWorkspaceUnderstanding, GetContextualWorkspaceUnderstandingSummary,
};
pub use workspace_cross_intelligence::{
    ExplainCrossWorkspacePattern, GenerateCrossWorkspaceIntelligence,
    GetCrossWorkspaceIntelligence, GetCrossWorkspaceSummary,
};
pub use workspace_decision_support::{
    ExplainWorkspaceDecisionSupport, GenerateWorkspaceDecisionSupport, GetWorkspaceDecisionSupport,
    GetWorkspaceDecisionSupportSummary,
};
pub use workspace_evidence_completeness::{
    ExplainEvidenceCompleteness, GenerateWorkspaceEvidenceCompleteness,
    GetWorkspaceEvidenceCompleteness, GetWorkspaceEvidenceCompletenessSummary,
};
pub use workspace_evidence_consistency::{
    ExplainEvidenceConsistency, GenerateWorkspaceEvidenceConsistency,
    GetWorkspaceEvidenceConsistency, GetWorkspaceEvidenceConsistencySummary,
};
pub use workspace_evidence_coverage::{
    ExplainEvidenceCoverage, GenerateWorkspaceEvidenceCoverage, GetWorkspaceEvidenceCoverage,
    GetWorkspaceEvidenceCoverageSummary,
};
pub use workspace_evidence_dependency::{
    ExplainEvidenceDependency, GenerateWorkspaceEvidenceDependency, GetWorkspaceEvidenceDependency,
    GetWorkspaceEvidenceDependencySummary,
};
pub use workspace_evidence_freshness::{
    ExplainEvidenceFreshness, GenerateWorkspaceEvidenceFreshness, GetWorkspaceEvidenceFreshness,
    GetWorkspaceEvidenceFreshnessSummary,
};
pub use workspace_evidence_navigation::{
    ExplainEvidenceNavigation, GenerateWorkspaceEvidenceNavigation, GetWorkspaceEvidenceNavigation,
    GetWorkspaceEvidenceNavigationSummary,
};
pub use workspace_evidence_reliability::{
    ExplainEvidenceReliability, GenerateWorkspaceEvidenceReliability,
    GetWorkspaceEvidenceReliability, GetWorkspaceEvidenceReliabilitySummary,
};
pub use workspace_evidence_trace::{
    ExplainEvidenceTrace, GenerateWorkspaceEvidenceTrace, GetWorkspaceEvidenceTrace,
    GetWorkspaceEvidenceTraceSummary,
};
pub use workspace_explanation::{
    ExplainWorkspaceSituation, GenerateWorkspaceExplanation, GetWorkspaceExplanation,
    GetWorkspaceExplanationSummary,
};
pub use workspace_insight_coordination::{
    ExplainInsightCoordination, GenerateInsightCoordination, GetInsightCoordination,
    GetInsightCoordinationSummary,
};
pub use workspace_intelligence_hub::{
    ExplainWorkspaceIntelligence, GenerateWorkspaceIntelligenceHub, GetWorkspaceIntelligenceHub,
    GetWorkspaceIntelligenceHubSummary,
};
pub use workspace_intent::{
    CreateProject, CreateTask, CreateWorkGoal, GetProject, GetTask, GetWorkflowContext,
    ListProjects, ListTasks, SetActiveWork, UpdateProject, UpdateTask,
};
pub use workspace_knowledge_integration::{
    ExplainKnowledgeIntegration, GenerateWorkspaceKnowledgeIntegration,
    GetWorkspaceKnowledgeIntegration, GetWorkspaceKnowledgeIntegrationSummary,
    RetrieveWorkspaceKnowledge,
};
pub use workspace_knowledge_synthesis::{
    ExplainKnowledgeSynthesis, GenerateWorkspaceKnowledgeSynthesis, GetWorkspaceKnowledgeSummary,
    GetWorkspaceKnowledgeSynthesis,
};
pub use workspace_learning_adaptation::{
    GenerateLearningSnapshot, GetLearningSnapshot, GetLearningSummary,
};
pub use workspace_observation::{
    CaptureWorkspaceObservation, GateObservationRead, GetLatestObservationDelta,
    GetLatestWorkspaceObservation, GetObservationSchedulerStatus, GetWorkspaceObservationById,
    GetWorkspaceObservationStatus,
};
pub use workspace_planning::{GeneratePlanningSnapshot, GetPlanningSnapshot, GetPlanningSummary};
pub use workspace_reasoning_memory::{
    GenerateReasoningRecord, GetReasoningRecord, GetReasoningSummary,
};
pub use workspace_semantic_query::{
    ExplainWorkspaceSemanticQuery, GenerateWorkspaceSemanticQuery, GetWorkspaceSemanticQuery,
    GetWorkspaceSemanticQuerySummary,
};
pub use workspace_state::GetWorkspaceState;
pub use workspace_state_envelope::{
    GenerateWorkspaceStateEnvelope, GetWorkspaceStateEnvelope, GetWorkspaceStateEnvelopeSummary,
};
pub use zone::{CreateZone, DeleteZone, GetZone};
