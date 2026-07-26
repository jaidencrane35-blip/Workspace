//! Workspace Intelligence — read-only aggregation of work understanding (P4-B5).
//!
//! Aggregates existing systems. Cannot execute, approve, or grant authority.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::automation_contract::AutomationContractSummary;
use crate::automation_trigger::{AutomationIntentProposalSummary, TriggerRejectionSummary};
use crate::decision_engine::DecisionEngineSummary;
use crate::decision_queue::DecisionQueueSummary;
use crate::workspace_composition::WorkspaceCompositionSummary;
use crate::workspace_purpose::WorkspacePurposeSummary;
use crate::workspace_evolution::WorkspaceEvolutionSummary;
use crate::workspace_recommendation::WorkspaceRecommendationEngineSummary;
use crate::workspace_operating_state::WorkspaceOperatingStateSummary;
use crate::workspace_pattern::WorkspacePatternSummary;
use crate::workspace_adaptation::WorkspaceAdaptationSummary;
use crate::workspace_readiness::WorkspaceReadinessSummary;
use crate::workspace_work_context::WorkspaceWorkContextSummary;
use crate::workspace_navigation::WorkspaceNavigationSummary;
use crate::workspace_milestone::WorkspaceMilestoneSummary;
use crate::workspace_working_style::WorkspaceWorkingStyleSummary;
use crate::workspace_transition::WorkspaceTransitionSummary;
use crate::workspace_interaction::WorkspaceInteractionSummary;
use crate::workspace_profile::WorkspaceProfileSummary;
use crate::workspace_environment::WorkspaceEnvironmentSummary;
use crate::workspace_task_graph::TaskGraphSummary;
use crate::workspace_activity::WorkspaceActivityGraphSummary;
use crate::workspace_attention::WorkspaceAttentionSummary;
use crate::workspace_continuity::WorkspaceContinuitySummary;
use crate::workspace_intent::{Project, Task, WorkGoal, WorkflowContext};

/// Intelligence-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceIntelligenceError {
    #[error("workspace intelligence requires a workspace id")]
    MissingWorkspace,

    #[error("workspace intelligence cannot execute or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// User-facing recommendation with an explanation (no chain-of-thought).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRecommendation {
    pub id: String,
    pub title: String,
    pub explanation: String,
    pub kind: String,
}

/// Highlight drawn from memory (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceHighlight {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub source: String,
}

/// Pending decision surfaced for the user (not an approval action).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingDecisionSummary {
    pub id: String,
    pub summary: String,
    pub explanation: String,
}

/// Blocked action summary (display only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedActionSummary {
    pub id: String,
    pub summary: String,
    pub explanation: String,
}

/// Recent activity line for the workspace summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentActivityItem {
    pub event_type: String,
    pub summary: String,
    pub timestamp: String,
}

/// Application currently relevant to the workspace (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelligenceApplicationSummary {
    pub id: String,
    pub name: String,
    pub appears_active: bool,
}

/// Aggregated, read-only understanding of the user's work context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIntelligenceState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub current_project: Option<Project>,
    pub current_task: Option<Task>,
    pub workflow_context: WorkflowContext,
    pub recent_goals: Vec<WorkGoal>,
    pub recent_activity: Vec<RecentActivityItem>,
    pub pending_plans: Vec<String>,
    pub pending_approvals: Vec<PendingDecisionSummary>,
    pub blocked_actions: Vec<BlockedActionSummary>,
    pub recommended_actions: Vec<WorkspaceRecommendation>,
    pub memory_highlights: Vec<IntelligenceHighlight>,
    pub preference_highlights: Vec<IntelligenceHighlight>,
    pub current_applications: Vec<IntelligenceApplicationSummary>,
    /// Read-only automation contract summaries (Batch 6). Never mutates contracts.
    pub automation_contracts: Vec<AutomationContractSummary>,
    /// Pending intent proposals from trigger evaluation (Batch 7). Read-only.
    pub pending_automation_proposals: Vec<AutomationIntentProposalSummary>,
    /// Recent explainable evaluation rejections (Batch 7). Read-only.
    pub recent_trigger_rejections: Vec<TriggerRejectionSummary>,
    /// Aggregated Decision Queue summary (Batch 8). Read-only.
    pub decision_queue: DecisionQueueSummary,
    /// Activity Graph summary (Batch 9). Read-only.
    pub activity_graph: WorkspaceActivityGraphSummary,
    /// Continuity Engine summary (Phase 5 Batch 1). Read-only.
    pub continuity: WorkspaceContinuitySummary,
    /// Attention Engine summary (Phase 5 Batch 2). Canonical prioritization.
    pub attention: WorkspaceAttentionSummary,
    /// Decision Engine summary (Phase 5). Ranked recommendations — never executes.
    pub decision_engine: DecisionEngineSummary,
    /// Task Graph summary (Phase 5). Canonical work model — never executes.
    pub task_graph: TaskGraphSummary,
    /// Environment Model summary (Phase 5). Live desktop read model — never executes.
    pub environment: WorkspaceEnvironmentSummary,
    /// Composition Engine summary (Phase 5). Logical working environment — never executes.
    pub composition: WorkspaceCompositionSummary,
    /// Purpose Model summary (Phase 5). Why work exists — never executes.
    pub purpose: WorkspacePurposeSummary,
    /// Evolution Model summary (Phase 5). How work changed — never executes.
    pub evolution: WorkspaceEvolutionSummary,
    /// Recommendation Engine summary (Phase 5). What might help next — never executes.
    pub recommendation_engine: WorkspaceRecommendationEngineSummary,
    /// Operating State summary (Phase 5). What is happening right now — never executes.
    pub operating_state: WorkspaceOperatingStateSummary,
    /// Pattern Model summary (Phase 5). Recurring structures — never executes.
    pub pattern: WorkspacePatternSummary,
    /// Adaptation Proposal summary (Phase 5). Possible improvements — never executes.
    pub adaptation: WorkspaceAdaptationSummary,
    /// Readiness Model summary (Phase 5). Preparedness for current work — never executes.
    /// Distinct from runtime `workspace_health` (kernel lifecycle).
    pub readiness: WorkspaceReadinessSummary,
    /// Work Context Engine summary (Phase 6). What kind of work — never executes.
    /// Embedded after Session/Experience projection; before future restoration systems.
    pub work_context: WorkspaceWorkContextSummary,
    /// Navigation Engine summary (Phase 6). Where to go next — never executes.
    /// Embedded after Work Context.
    pub navigation: WorkspaceNavigationSummary,
    /// Milestone Engine summary (Phase 6). Progress toward outcomes — never executes.
    /// Embedded after Navigation.
    pub milestones: WorkspaceMilestoneSummary,
    /// Working Style Model summary (Phase 6). How work usually happens — never profiles/executes.
    /// Embedded after Milestones. Separates observed behaviour from explicit preference.
    pub working_style: WorkspaceWorkingStyleSummary,
    /// Transition Engine summary (Phase 6). Movement between work states — never restores/executes.
    /// Embedded after Working Style.
    pub transition: WorkspaceTransitionSummary,
    /// Interaction Model summary (Phase 6). What the user can interact with — never executes.
    /// Embedded after Transition. Owns nothing; aggregates existing cognition.
    pub interaction: WorkspaceInteractionSummary,
    /// Workspace Environment Profiles summary (Phase 6). User-owned setups — never executes.
    /// Embedded after Interaction. Durable references only; comparison is informational.
    pub profiles: WorkspaceProfileSummary,
    pub workspace_health: String,
    pub summary: String,
    /// Explicit marker for audits and UI: this state grants nothing.
    pub authority_effect: String,
}

impl WorkspaceIntelligenceState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Side-by-side comparison of two intelligence snapshots (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIntelligenceComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
}

impl WorkspaceIntelligenceComparison {
    pub fn compare(left: &WorkspaceIntelligenceState, right: &WorkspaceIntelligenceState) -> Self {
        let mut differences = Vec::new();
        if left.current_project.as_ref().map(|p| p.id.as_str())
            != right.current_project.as_ref().map(|p| p.id.as_str())
        {
            differences.push("Active project differs.".into());
        }
        if left.current_task.as_ref().map(|t| t.id.as_str())
            != right.current_task.as_ref().map(|t| t.id.as_str())
        {
            differences.push("Active task differs.".into());
        }
        if left.pending_approvals.len() != right.pending_approvals.len() {
            differences.push(format!(
                "Pending approvals: {} → {}",
                left.pending_approvals.len(),
                right.pending_approvals.len()
            ));
        }
        if left.memory_highlights.len() != right.memory_highlights.len() {
            differences.push("Memory highlights differ.".into());
        }
        if left.preference_highlights.len() != right.preference_highlights.len() {
            differences.push("Preference highlights differ.".into());
        }
        if left.recommended_actions.len() != right.recommended_actions.len() {
            differences.push(format!(
                "Recommendation count: {} → {}",
                left.recommended_actions.len(),
                right.recommended_actions.len()
            ));
        }
        if left.decision_engine.open_count != right.decision_engine.open_count {
            differences.push(format!(
                "Decision Engine open candidates: {} → {}",
                left.decision_engine.open_count,
                right.decision_engine.open_count
            ));
        }
        if left.work_context.primary_context_type != right.work_context.primary_context_type
            || left.work_context.context_count != right.work_context.context_count
        {
            differences.push(format!(
                "Work Context: {:?} ({}) → {:?} ({})",
                left.work_context.primary_context_type,
                left.work_context.context_count,
                right.work_context.primary_context_type,
                right.work_context.context_count
            ));
        }
        if left.navigation.node_count != right.navigation.node_count
            || left.navigation.blocked_count != right.navigation.blocked_count
        {
            differences.push(format!(
                "Navigation nodes/blocked: {}/{} → {}/{}",
                left.navigation.node_count,
                left.navigation.blocked_count,
                right.navigation.node_count,
                right.navigation.blocked_count
            ));
        }
        if left.milestones.milestone_count != right.milestones.milestone_count
            || left.milestones.current_milestone_title != right.milestones.current_milestone_title
        {
            differences.push(format!(
                "Milestones: {:?} ({}) → {:?} ({})",
                left.milestones.current_milestone_title,
                left.milestones.milestone_count,
                right.milestones.current_milestone_title,
                right.milestones.milestone_count
            ));
        }
        if left.working_style.observation_count != right.working_style.observation_count
            || left.working_style.preference_count != right.working_style.preference_count
        {
            differences.push(format!(
                "Working Style: {} obs / {} prefs → {} obs / {} prefs",
                left.working_style.observation_count,
                left.working_style.preference_count,
                right.working_style.observation_count,
                right.working_style.preference_count
            ));
        }
        if left.transition.transition_count != right.transition.transition_count
            || left.transition.current_transition_title != right.transition.current_transition_title
        {
            differences.push(format!(
                "Transition: {:?} ({}) → {:?} ({})",
                left.transition.current_transition_title,
                left.transition.transition_count,
                right.transition.current_transition_title,
                right.transition.transition_count
            ));
        }
        if left.interaction.item_count != right.interaction.item_count
            || left.interaction.decision_count != right.interaction.decision_count
        {
            differences.push(format!(
                "Interaction: {} items / {} decisions → {} items / {} decisions",
                left.interaction.item_count,
                left.interaction.decision_count,
                right.interaction.item_count,
                right.interaction.decision_count
            ));
        }
        if left.profiles.profile_count != right.profiles.profile_count
            || left.profiles.best_profile_name != right.profiles.best_profile_name
        {
            differences.push(format!(
                "Profiles: {} ({:?}) → {} ({:?})",
                left.profiles.profile_count,
                left.profiles.best_profile_name,
                right.profiles.profile_count,
                right.profiles.best_profile_name
            ));
        }
        if differences.is_empty() {
            differences.push("No material differences between workspace intelligence states.".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
        }
    }
}
