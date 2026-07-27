mod accept_suggestion;
mod application;
mod automation_contract;
mod automation_trigger;
mod decision_engine;
mod decision_queue;
mod task_graph;
mod workspace_activity;
mod workspace_attention;
mod workspace_continuity;
mod workspace_environment;
mod workspace_composition;
mod workspace_purpose;
mod workspace_evolution;
mod workspace_recommendation;
mod workspace_operating_state;
mod workspace_pattern;
mod workspace_adaptation;
mod workspace_readiness;
mod workspace_runtime;
mod workspace_intelligence;
mod workspace_session;
mod workspace_experience;
mod workspace_work_context;
mod workspace_navigation;
mod workspace_milestone;
mod workspace_working_style;
mod workspace_transition;
mod workspace_interaction;
mod workspace_profile;
mod workspace_observation;
mod workspace_state;
mod context;
mod execute_intent_request;
mod create_suggestion_intent_request;
mod create_workspace;
mod decide_approval;
mod get_action_catalog;
mod get_ai_evaluation_history;
mod get_actor_capabilities;
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
mod handler;
mod initialize;
mod launch_application;
mod memory;
mod model_provider;
mod personalization;
mod pipeline;
mod reject_suggestion;
mod request_execution_cancellation;
#[cfg(test)]
mod analytics_tests;
#[cfg(test)]
mod context_tests;
#[cfg(test)]
mod discovery_tests;
#[cfg(test)]
mod suggestion_lifecycle_tests;
#[cfg(test)]
mod intent_execution_tests;
#[cfg(test)]
mod execution_outcome_tests;
#[cfg(test)]
mod suggestion_intent_tests;
#[cfg(test)]
mod suggestion_tests;
#[cfg(test)]
mod intent_tests;
#[cfg(test)]
mod permission_approval_tests;
#[cfg(test)]
mod ai_participation_tests;
#[cfg(test)]
mod ai_planning_tests;
#[cfg(test)]
mod action_catalog_tests;
#[cfg(test)]
mod ai_evaluation_tests;
#[cfg(test)]
mod ai_orchestration_tests;
#[cfg(test)]
mod ai_assistant_tests;
#[cfg(test)]
mod ai_assistant_product_tests;
#[cfg(test)]
mod workspace_intelligence_tests;
#[cfg(test)]
mod automation_contract_tests;
#[cfg(test)]
mod automation_trigger_tests;
#[cfg(test)]
mod decision_queue_tests;
#[cfg(test)]
mod workspace_activity_tests;
#[cfg(test)]
mod platform_coherence_tests;
#[cfg(test)]
mod workspace_continuity_tests;
#[cfg(test)]
mod workspace_attention_tests;
#[cfg(test)]
mod decision_engine_tests;
#[cfg(test)]
mod decision_engine_intake_receipt_contract_tests;
#[cfg(test)]
mod decision_engine_intake_assessment_contract_tests;
#[cfg(test)]
mod decision_engine_intake_eligibility_contract_tests;
#[cfg(test)]
mod decision_engine_intake_candidate_contract_tests;
#[cfg(test)]
mod decision_engine_intake_candidate_lifecycle_contract_tests;
#[cfg(test)]
mod decision_engine_intake_evaluation_contract_tests;
#[cfg(test)]
mod decision_engine_intake_disposition_contract_tests;
#[cfg(test)]
mod decision_engine_intake_promotion_boundary_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_creation_request_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_creation_contract_tests;
#[cfg(test)]
mod decision_engine_candidate_lifecycle_integration_contract_tests;
#[cfg(test)]
mod task_graph_tests;
#[cfg(test)]
mod workspace_environment_tests;
#[cfg(test)]
mod workspace_composition_tests;
#[cfg(test)]
mod workspace_purpose_tests;
#[cfg(test)]
mod workspace_evolution_tests;
#[cfg(test)]
mod workspace_recommendation_tests;
#[cfg(test)]
mod workspace_operating_state_tests;
#[cfg(test)]
mod workspace_pattern_tests;
#[cfg(test)]
mod workspace_adaptation_tests;
mod workspace_readiness_tests;
mod workspace_session_tests;
mod workspace_experience_tests;
#[cfg(test)]
mod workspace_experience_contract_tests;
#[cfg(test)]
mod workspace_cognition_pipeline_contract_tests;
#[cfg(test)]
mod workspace_recommendation_provenance_contract_tests;
#[cfg(test)]
mod workspace_recommendation_lifecycle_contract_tests;
#[cfg(test)]
mod workspace_recommendation_outcome_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_readiness_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_context_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_boundary_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_confirmation_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_intake_contract_tests;
mod workspace_recommendation_decision_intake_inspection_contract_tests;
mod workspace_recommendation_decision_intake_compatibility_contract_tests;
mod workspace_recommendation_decision_intake_proceed_denial_contract_tests;
mod workspace_recommendation_decision_intake_package_seal_contract_tests;
mod workspace_recommendation_decision_intake_adapter_preparation_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_handoff_request_contract_tests;
#[cfg(test)]
mod workspace_recommendation_decision_engine_acceptance_contract_tests;
#[cfg(test)]
mod workspace_adaptation_governance_contract_tests;
#[cfg(test)]
mod workspace_adaptation_review_contract_tests;
#[cfg(test)]
mod workspace_controlled_change_contract_tests;
#[cfg(test)]
mod workspace_governance_ledger_contract_tests;
#[cfg(test)]
mod workspace_governance_policy_contract_tests;
#[cfg(test)]
mod workspace_governance_risk_contract_tests;
#[cfg(test)]
mod workspace_governance_evidence_contract_tests;
#[cfg(test)]
mod workspace_governance_workspace_contract_tests;
#[cfg(test)]
mod workspace_governance_lifecycle_contract_tests;
#[cfg(test)]
#[cfg(test)]
mod workspace_publication_safety_contract_tests;
#[cfg(test)]
mod workspace_governance_failure_contract_tests;
#[cfg(test)]
mod workspace_governance_conditions_contract_tests;
#[cfg(test)]
mod workspace_governance_compatibility_contract_tests;
#[cfg(test)]
mod workspace_governance_integrity_contract_tests;
#[cfg(test)]
mod workspace_governance_archive_contract_tests;
#[cfg(test)]
mod workspace_governance_review_workflow_contract_tests;
#[cfg(test)]
mod workspace_governance_conflict_contract_tests;
#[cfg(test)]
mod workspace_governance_decision_package_contract_tests;
#[cfg(test)]
mod workspace_governance_compliance_contract_tests;
#[cfg(test)]
mod workspace_governance_dashboard_contract_tests;
#[cfg(test)]
mod workspace_governance_notifications_contract_tests;
#[cfg(test)]
mod workspace_governance_delegation_contract_tests;
#[cfg(test)]
mod workspace_governance_metrics_contract_tests;
#[cfg(test)]
mod workspace_governance_reporting_contract_tests;
#[cfg(test)]
mod workspace_governance_export_contract_tests;
#[cfg(test)]
mod workspace_governance_consolidation_contract_tests;
#[cfg(test)]
mod workspace_runtime_integration_contract_tests;
#[cfg(test)]
mod workspace_runtime_projection_contract_tests;
#[cfg(test)]
mod workspace_observation_freshness_wiring_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostics_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_continuity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_evolution_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_integrity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_consumption_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_trust_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_closure_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_maturity_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_consolidation_contract_tests;
#[cfg(test)]
mod workspace_runtime_diagnostic_layering_contract_tests;
mod workspace_work_context_tests;
mod workspace_navigation_tests;
mod workspace_milestone_tests;
mod workspace_working_style_tests;
mod workspace_transition_tests;
mod workspace_interaction_tests;
mod workspace_profile_tests;
mod workspace_observation_tests;
#[cfg(test)]
mod ai_memory_tests;
#[cfg(test)]
mod ai_model_provider_tests;
#[cfg(test)]
mod ai_personalization_tests;
#[cfg(test)]
mod observation_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod projection_tests;
mod layout;
mod resource;
#[cfg(test)]
mod resource_tests;
mod r#trait;
mod update_settings;
mod widget;
mod workspace_intent;
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
pub use decision_queue::{GateDecisionQueueRead, GateDecisionQueueWrite};
pub use workspace_activity::GateActivityGraphRead;
pub use layout::{
    CreateLayout, DeleteLayout, GetLayout, GetLayoutSnapshot, ResetLayout, UpdateLayout,
};
pub use context::CommandContext;
pub use execute_intent_request::ExecuteIntentRequest;
pub use create_suggestion_intent_request::CreateSuggestionIntentRequest;
pub use create_workspace::CreateWorkspace;
pub use decide_approval::DecideApproval;
pub use get_action_catalog::GetActionCatalog;
pub use get_ai_evaluation_history::GetAiEvaluationHistory;
pub use get_actor_capabilities::GetActorCapabilities;
pub use get_audit_history::GetAuditHistory;
pub use get_permission_approvals::GetPermissionApprovals;
pub use workspace_observation::{
    CaptureWorkspaceObservation, GateObservationRead, GetLatestObservationDelta,
    GetLatestWorkspaceObservation, GetObservationSchedulerStatus, GetWorkspaceObservationById,
    GetWorkspaceObservationStatus,
};
pub use workspace_state::GetWorkspaceState;
pub use get_execution_outcomes::GetExecutionOutcomes;
pub use get_execution_state::GetExecutionState;
pub use get_execution_states::GetExecutionStates;
pub use get_observations::GetObservations;
pub use get_suggestion_lifecycle::GetSuggestionLifecycle;
pub use get_suggestions::GetSuggestions;
pub use get_workspace::GetWorkspace;
pub use get_workspace_context::GetWorkspaceContext;
pub use get_workspace_metrics::GetWorkspaceMetrics;
pub use get_workspace_snapshot::GetWorkspaceSnapshot;
pub use handler::CommandHandler;
pub use initialize::InitializeWorkspace;
pub use launch_application::LaunchApplication;
pub use memory::{
    ClearMemoryEntries, CreateMemoryEntry, DeleteMemoryEntry, GetMemoryContext, ListMemoryEntries,
};
pub use model_provider::{
    GetModelProviderMetadata, ListModelProviders, TestModelProviderRequest,
};
pub use personalization::{
    CreateUserPreference, DeleteUserPreference, GetPreferenceProfile, SetPersonalizationEnabled,
    UpdateUserPreference,
};
pub use pipeline::CommandPipeline;
pub use reject_suggestion::RejectSuggestion;
pub use request_execution_cancellation::RequestExecutionCancellation;
pub use r#trait::{Command, MutationCommand, QueryCommand};
pub use update_settings::UpdateSettings;
pub use widget::{CreateWidget, DeleteWidget, GetWidget};
pub use workspace_intent::{
    CreateProject, CreateTask, CreateWorkGoal, GetProject, GetTask, GetWorkflowContext,
    ListProjects, ListTasks, SetActiveWork, UpdateProject, UpdateTask,
};
pub use zone::{CreateZone, DeleteZone, GetZone};
