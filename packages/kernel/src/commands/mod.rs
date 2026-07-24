mod accept_suggestion;
mod application;
mod automation_contract;
mod automation_trigger;
mod decision_queue;
mod workspace_activity;
mod workspace_attention;
mod workspace_continuity;
mod context;
mod execute_intent_request;
mod create_suggestion_intent_request;
mod create_workspace;
mod decide_approval;
mod get_action_catalog;
mod get_ai_evaluation_history;
mod get_actor_capabilities;
mod get_audit_history;
mod get_desktop_windows;
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
pub use get_desktop_windows::GetDesktopWindows;
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
