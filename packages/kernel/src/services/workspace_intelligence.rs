//! Workspace Intelligence aggregator — read-only (Phase 4 Batch 5 / 5.5).
//!
//! Consumes existing awareness, memory, personalization, plans, approvals.
//! Cannot execute, approve, grant, or bypass Permission Gateway.
//! Batch 5.5: workspace-scoped aggregation, no create-on-read.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{ApplicationRepository, Database};
use workspace_domain::{
    ActorContext, AutomationContractSummary, BlockedActionSummary, DecisionSourceType,
    DecisionState, IntelligenceApplicationSummary, IntelligenceHighlight, IntentContext,
    PendingDecisionSummary, RecentActivityItem, WorkspaceId, WorkspaceIntelligenceState,
    WorkspaceRecommendation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    list_rejection_summaries, AiMemoryService, AiPersonalizationService, AssistantWorkflowStore,
    AuditService, AutomationContractService, DecisionEngineService, DecisionQueueService,
    OrchestratedPlanStore, TaskGraphService, TriggerEvaluatorService,
    WorkspaceActivityGraphService, WorkspaceAttentionService, WorkspaceContinuityService,
    WorkspaceEnvironmentService, WorkspaceIntentService, WorkspaceCompositionService,
    WorkspacePurposeService, WorkspaceEvolutionService, WorkspaceRecommendationEngineService,
    WorkspaceOperatingStateService, WorkspacePatternService, WorkspaceAdaptationService,
    WorkspaceReadinessService, WorkspaceSessionService, WorkspaceExperienceService,
    WorkspaceWorkContextService, WorkspaceNavigationService, WorkspaceMilestoneService,
    WorkspaceWorkingStyleService, WorkspaceTransitionService, WorkspaceInteractionService,
    WorkspaceProfileService, WorkspaceStateEngine,
};

pub(crate) struct WorkspaceIntelligenceService;

impl WorkspaceIntelligenceService {
    /// Build a read-only intelligence snapshot. Never mutates authority or intent tables.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceIntelligenceState> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();
        let ws = workspace_id.as_str();

        // Read-only: never insert WorkflowContext during generate.
        let workflow_context = WorkspaceIntentService::get_workflow_context_readonly(db, ws)?;
        // Current work SoT is WorkflowContext only — never infer first project/task.
        let current_project = match &workflow_context.active_project_id {
            Some(id) => WorkspaceIntentService::get_project(db, id.as_str())
                .ok()
                .filter(|project| project.workspace_id.as_str() == ws && !project.deleted),
            None => None,
        };
        let current_task = match &workflow_context.active_task_id {
            Some(id) => WorkspaceIntentService::get_task(db, id.as_str())
                .ok()
                .filter(|task| task.workspace_id.as_str() == ws && !task.deleted),
            None => None,
        };
        let recent_goals = WorkspaceIntentService::list_goals(db, ws, 10)?;
        // Read-only — never mutate contracts or proposals from intelligence.
        let automation_contracts = AutomationContractService::list(db, ws, Some(20))?
            .iter()
            .map(AutomationContractSummary::from)
            .collect::<Vec<_>>();
        let pending_automation_proposals =
            TriggerEvaluatorService::pending_summaries(db, ws, 20).unwrap_or_default();
        let recent_trigger_rejections =
            list_rejection_summaries(db, ws, 20).unwrap_or_default();
        // Decision Queue is the sole pending-decision aggregation (persist overlays once).
        // Phase 5.5: intentional sole-writer on the Intelligence product path — nested
        // consumers must use aggregate_readonly. Not a Gateway grant; overlay presentation only.
        let full_decision_queue = DecisionQueueService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            ws,
        )?;
        let decision_queue = full_decision_queue.summary(10);
        // Activity Graph consumes the same queue — no nested overlay writes.
        let full_activity_graph = WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            ws,
            Some(&full_decision_queue),
        )?;
        let activity_graph = full_activity_graph.summary(12);
        let full_continuity = WorkspaceContinuityService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_decision_queue,
            &full_activity_graph,
        )?;
        let continuity = full_continuity.summary_projection(6);
        let full_task_graph = TaskGraphService::generate(db, actor, ws)?;
        let task_graph = full_task_graph.summary_projection(8);

        // One WorkspaceState instance for the intelligence generation cycle.
        let workspace_state = WorkspaceStateEngine::get_current(
            db,
            actor,
            &IntentContext::user_request(),
        )?;
        let applications = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            ApplicationRepository::new(&guard).list_by_workspace(&workspace_id)?
        };
        let layout = {
            let guard = db.lock().ok();
            guard.and_then(|g| {
                crate::services::LayoutService::load_by_workspace(&g, &workspace_id).ok()
            })
        };
        let full_environment = WorkspaceEnvironmentService::generate_from_state(
            db,
            actor,
            ws,
            &workspace_state,
            &applications,
            &workflow_context,
            Some(&full_task_graph),
            layout.as_ref(),
        )?;
        let environment = WorkspaceEnvironmentService::summary_projection(&full_environment, 6);

        let full_composition = WorkspaceCompositionService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_environment,
            Some(&full_task_graph),
            &full_continuity,
            &full_activity_graph,
            &workflow_context,
            &full_decision_queue,
            current_project.as_ref(),
        )?;
        let composition = WorkspaceCompositionService::summary_projection(&full_composition, 8);

        let full_purpose = WorkspacePurposeService::generate_with_inputs(
            db,
            actor,
            ws,
            &recent_goals,
            &workflow_context,
            current_project.as_ref(),
            Some(&full_task_graph),
            &full_composition,
            &full_continuity,
            &full_activity_graph,
            &full_decision_queue,
        )?;
        let purpose = WorkspacePurposeService::summary_projection(&full_purpose, 8);

        let full_evolution = WorkspaceEvolutionService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_activity_graph,
            Some(&full_task_graph),
            &full_purpose,
            &full_composition,
            &full_continuity,
            &full_decision_queue,
        )?;
        let evolution = WorkspaceEvolutionService::summary_projection(&full_evolution, 8);

        let base_attention = WorkspaceAttentionService::generate_with_task_graph(
            db,
            actor,
            ws,
            &full_decision_queue,
            &full_activity_graph,
            &full_continuity,
            Some(&full_task_graph),
            Some(&full_environment),
            Some(&full_composition),
            Some(&full_purpose),
            Some(&full_evolution),
        )?;

        let base_recommendation_engine =
            WorkspaceRecommendationEngineService::generate_with_inputs(
                db,
                actor,
                ws,
                &base_attention,
                &full_continuity,
                &full_evolution,
                &full_purpose,
                Some(&full_task_graph),
                &full_composition,
                &full_decision_queue,
                &full_environment,
            )?;

        // Attention may surface recommendations after RE is built (no circular regen).
        let attention_with_recs = WorkspaceAttentionService::enrich_with_recommendations(
            &base_attention,
            &base_recommendation_engine,
        )?;

        let full_operating_state = WorkspaceOperatingStateService::generate_with_inputs(
            db,
            actor,
            ws,
            &workflow_context,
            current_project.as_ref(),
            current_task.as_ref(),
            &full_purpose,
            &full_environment,
            &full_composition,
            Some(&full_task_graph),
            &full_continuity,
            &full_activity_graph,
            &full_decision_queue,
            &attention_with_recs,
            &base_recommendation_engine,
            &full_evolution,
        )?;
        let operating_state =
            WorkspaceOperatingStateService::summary_projection(&full_operating_state, 8);

        let full_pattern = WorkspacePatternService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_activity_graph,
            &full_evolution,
            &full_operating_state,
            &full_composition,
            Some(&full_task_graph),
            &full_environment,
            &full_purpose,
            &full_continuity,
            &full_decision_queue,
        )?;
        let pattern = WorkspacePatternService::summary_projection(&full_pattern, 8);

        // Recommendation Engine may consume patterns as evidence (no circular regen).
        let recommendation_with_patterns =
            WorkspaceRecommendationEngineService::enrich_with_patterns(
                db,
                actor,
                &base_recommendation_engine,
                &full_pattern,
            )?;

        // Attention may surface pattern-informed context after Pattern is built.
        let attention_with_patterns = WorkspaceAttentionService::enrich_with_patterns(
            &attention_with_recs,
            &full_pattern,
        )?;

        // Readiness consumes OS + Pattern + upstream aggregators (before RE/Adaptation enrich).
        let full_readiness = WorkspaceReadinessService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_operating_state,
            &full_environment,
            &full_composition,
            &full_task_graph,
            &full_purpose,
            &full_continuity,
            &full_evolution,
            &full_pattern,
            &full_decision_queue,
        )?;
        let readiness = WorkspaceReadinessService::summary_projection(&full_readiness, 8);

        // Recommendation Engine may consume readiness gaps as evidence (no circular regen).
        let full_recommendation_engine =
            WorkspaceRecommendationEngineService::enrich_with_readiness(
                &recommendation_with_patterns,
                &full_readiness,
            )?;
        let recommendation_engine = WorkspaceRecommendationEngineService::summary_projection(
            &full_recommendation_engine,
            8,
        );

        let full_attention = WorkspaceAttentionService::enrich_with_recommendations(
            &attention_with_patterns,
            &full_recommendation_engine,
        )?;
        let attention = full_attention.summary_projection(8);

        let full_adaptation = WorkspaceAdaptationService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_pattern,
            &full_recommendation_engine,
            &full_operating_state,
            &full_composition,
            &full_environment,
            &full_continuity,
            &full_purpose,
            Some(&full_readiness),
        )?;
        let adaptation = WorkspaceAdaptationService::summary_projection(&full_adaptation, 8);

        let memory = AiMemoryService::assemble_awareness(db, Some(ws), 10)
            .unwrap_or_else(|_| workspace_domain::AiMemoryAwareness::from_entries(Vec::new()));
        let personalization = AiPersonalizationService::assemble_awareness(db, Some(ws), 10, None)
            .unwrap_or_else(|_| {
                workspace_domain::AiPersonalizationAwareness::from_preferences(Vec::new(), true)
            });

        let memory_highlights = memory
            .entries
            .iter()
            .take(5)
            .map(|entry| IntelligenceHighlight {
                id: entry.id.to_string(),
                label: entry.key.clone(),
                summary: entry.summary.clone(),
                source: "memory".into(),
            })
            .collect::<Vec<_>>();

        let preference_highlights = if personalization.enabled {
            personalization
                .preferences
                .iter()
                .take(5)
                .map(|pref| IntelligenceHighlight {
                    id: pref.id.to_string(),
                    label: pref.label.clone().unwrap_or_else(|| pref.key.clone()),
                    summary: pref.value.clone(),
                    source: "preference".into(),
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let full_decision_engine = DecisionEngineService::generate_with_inputs(
            db,
            actor,
            ws,
            &full_attention,
            &full_decision_queue,
            &workflow_context,
            &recent_goals,
            &memory_highlights,
            &preference_highlights,
            Some(&full_task_graph),
        )?;
        let decision_engine = DecisionEngineService::summary_projection(&full_decision_engine, 5);

        let applications = full_environment
            .applications
            .iter()
            .map(|app| IntelligenceApplicationSummary {
                id: app.application_id.clone(),
                name: app.name.clone(),
                appears_active: app.appears_running,
            })
            .collect::<Vec<_>>();

        // Project attention fields from Decision Queue — do not re-scan sources.
        let attention_open = |state: DecisionState| {
            matches!(
                state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            )
        };
        let pending_approvals = full_decision_queue
            .items
            .iter()
            .filter(|item| {
                item.source_type == DecisionSourceType::PendingApproval
                    && attention_open(item.decision_state)
            })
            .map(|item| PendingDecisionSummary {
                id: item.source_id.clone(),
                summary: item.summary.clone(),
                explanation: item.explanation.clone(),
            })
            .collect::<Vec<_>>();

        let blocked_actions = full_decision_queue
            .items
            .iter()
            .filter(|item| {
                item.source_type == DecisionSourceType::BlockedAction
                    && attention_open(item.decision_state)
            })
            .map(|item| BlockedActionSummary {
                id: item.source_id.clone(),
                summary: item.summary.clone(),
                explanation: item.explanation.clone(),
            })
            .collect::<Vec<_>>();

        let pending_plans = full_decision_queue
            .items
            .iter()
            .filter(|item| {
                item.source_type == DecisionSourceType::PlanningContinuation
                    && attention_open(item.decision_state)
            })
            .map(|item| format!("{} — {}", item.title, item.summary))
            .collect::<Vec<_>>();

        // Product recent activity comes from Activity Graph (single relationship read model).
        let recent_activity = full_activity_graph
            .timeline
            .iter()
            .rev()
            .take(15)
            .map(|activity| RecentActivityItem {
                event_type: activity.activity_type.as_str().to_string(),
                summary: activity.summary.clone(),
                timestamp: activity.timestamp.clone(),
            })
            .collect::<Vec<_>>();

        let recommended_actions = Self::recommendations_from_attention(&full_attention);

        let summary = Self::build_summary(
            &workspace_name,
            &current_project,
            &current_task,
            decision_queue.pending_count,
            blocked_actions.len(),
            recommended_actions.len(),
            &automation_contracts,
            pending_automation_proposals.len(),
        );

        // Assemble cognition envelope first (work_context filled after Experience).
        let mut state = WorkspaceIntelligenceState {
            workspace_id: ws.to_string(),
            workspace_name,
            generated_at: Utc::now().to_rfc3339(),
            current_project,
            current_task,
            workflow_context,
            recent_goals,
            recent_activity,
            pending_plans,
            pending_approvals,
            blocked_actions,
            recommended_actions,
            memory_highlights,
            preference_highlights,
            current_applications: applications,
            automation_contracts,
            pending_automation_proposals,
            recent_trigger_rejections,
            decision_queue,
            activity_graph,
            continuity,
            attention,
            decision_engine,
            task_graph,
            environment,
            composition,
            purpose,
            evolution,
            recommendation_engine,
            operating_state,
            pattern,
            adaptation,
            readiness,
            work_context: Default::default(),
            navigation: Default::default(),
            milestones: Default::default(),
            working_style: Default::default(),
            transition: Default::default(),
            interaction: Default::default(),
            profiles: Default::default(),
            workspace_health: health_label,
            summary,
            authority_effect: WorkspaceIntelligenceState::AUTHORITY_EFFECT_NONE.into(),
        };

        // Phase 6: Session → … → Transition → Interaction → Profiles.
        let session = WorkspaceSessionService::generate_with_inputs(db, actor, &state)?;
        let experience = WorkspaceExperienceService::generate_with_inputs(db, actor, &session)?;
        let work_context = WorkspaceWorkContextService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
        )?;
        let navigation = WorkspaceNavigationService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
            &work_context,
        )?;
        let milestones = WorkspaceMilestoneService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
            &work_context,
            &navigation,
        )?;
        let working_style = WorkspaceWorkingStyleService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
        )?;
        let transition = WorkspaceTransitionService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
            &working_style,
        )?;

        // Evidence-only consumption — does not grant authority or mutate Transitions.
        let enriched_attention = WorkspaceAttentionService::enrich_with_transition(
            &WorkspaceAttentionService::enrich_with_working_style(
                &WorkspaceAttentionService::enrich_with_milestones(
                    &WorkspaceAttentionService::enrich_with_navigation(
                        &WorkspaceAttentionService::enrich_with_work_context(
                            &full_attention,
                            &work_context,
                        )?,
                        &navigation,
                    )?,
                    &milestones,
                )?,
                &working_style,
            )?,
            &transition,
        )?;
        let enriched_recommendations =
            WorkspaceRecommendationEngineService::enrich_with_transition(
                &WorkspaceRecommendationEngineService::enrich_with_working_style(
                    &WorkspaceRecommendationEngineService::enrich_with_milestones(
                        &WorkspaceRecommendationEngineService::enrich_with_navigation(
                            &WorkspaceRecommendationEngineService::enrich_with_work_context(
                                &full_recommendation_engine,
                                &work_context,
                            )?,
                            &navigation,
                        )?,
                        &milestones,
                    )?,
                    &working_style,
                )?,
                &transition,
            )?;
        let enriched_adaptation = WorkspaceAdaptationService::enrich_with_transition(
            &WorkspaceAdaptationService::enrich_with_working_style(
                &WorkspaceAdaptationService::enrich_with_milestones(
                    &WorkspaceAdaptationService::enrich_with_navigation(
                        &WorkspaceAdaptationService::enrich_with_work_context(
                            &full_adaptation,
                            &work_context,
                        )?,
                        &navigation,
                    )?,
                    &milestones,
                )?,
                &working_style,
            )?,
            &transition,
        )?;
        let _enriched_session = WorkspaceSessionService::enrich_with_transition(
            &WorkspaceSessionService::enrich_with_working_style(
                &WorkspaceSessionService::enrich_with_milestones(
                    &WorkspaceSessionService::enrich_with_navigation(&session, &navigation)?,
                    &milestones,
                )?,
                &working_style,
            )?,
            &transition,
        )?;
        let _enriched_experience = WorkspaceExperienceService::enrich_with_transition(
            &WorkspaceExperienceService::enrich_with_working_style(&experience, &working_style)?,
            &transition,
        )?;
        let enriched_navigation = WorkspaceNavigationService::enrich_with_transition(
            &WorkspaceNavigationService::enrich_with_working_style(
                &WorkspaceNavigationService::enrich_with_milestones(&navigation, &milestones)?,
                &working_style,
            )?,
            &transition,
        )?;

        state.attention = enriched_attention.summary_projection(8);
        state.recommendation_engine =
            WorkspaceRecommendationEngineService::summary_projection(&enriched_recommendations, 6);
        state.adaptation =
            WorkspaceAdaptationService::summary_projection(&enriched_adaptation, 8);
        state.work_context = work_context.summary_projection(5);
        state.navigation = enriched_navigation.summary_projection(6);
        state.milestones = milestones.summary_projection(6);
        state.working_style = working_style.summary_projection(6);
        state.transition = transition.summary_projection(6);
        state.recommended_actions = Self::recommendations_from_attention(&enriched_attention);

        // Interaction consumes assembled cognition (after Transition) — never creates new cognition.
        let interaction = WorkspaceInteractionService::generate_with_inputs(
            db,
            actor,
            &state,
            &session,
            &experience,
            &transition,
        )?;
        state.interaction = interaction.summary_projection(8);

        // Profiles are durable user-owned data — comparison only; never executes.
        let profiles = WorkspaceProfileService::generate_state(db, actor, &state)?;
        state.profiles = profiles.summary_projection(6);

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    /// Recommendations are projected from Attention — Intelligence does not re-prioritize.
    fn recommendations_from_attention(
        attention: &workspace_domain::WorkspaceAttentionState,
    ) -> Vec<WorkspaceRecommendation> {
        let mut recommendations = attention
            .top_items
            .iter()
            .take(6)
            .map(|item| WorkspaceRecommendation {
                id: format!("rec-attention-{}", item.id),
                title: item.title.clone(),
                explanation: item.explanation.clone(),
                kind: format!("attention:{}", item.category.as_str()),
                reasons: item.reasons.clone(),
            })
            .collect::<Vec<_>>();

        if recommendations.is_empty() {
            recommendations.push(WorkspaceRecommendation {
                id: "rec-idle".into(),
                title: "Define current work".into(),
                explanation:
                    "Suggested because Attention has no scored items for this workspace.".into(),
                kind: "bootstrap".into(),
                reasons: Vec::new(),
            });
        }

        recommendations
    }

    fn build_summary(
        workspace_name: &str,
        project: &Option<workspace_domain::Project>,
        task: &Option<workspace_domain::Task>,
        pending_approvals: usize,
        blocked: usize,
        recommendations: usize,
        automation_contracts: &[AutomationContractSummary],
        pending_proposals: usize,
    ) -> String {
        let project_label = project
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or("no active project");
        let task_label = task
            .as_ref()
            .map(|t| t.title.as_str())
            .unwrap_or("no active task");
        let approved_contracts = automation_contracts
            .iter()
            .filter(|c| c.status == "approved" && c.approval_state == "approved")
            .count();
        format!(
            "Workspace \"{workspace_name}\" — working on {project_label} / {task_label}. \
             {pending_approvals} decision(s) needing attention, {blocked} blocked action(s), \
             {recommendations} recommendation(s), {approved_contracts} approved Automation \
             Contract(s), {pending_proposals} pending Intent Proposal(s). \
             Intelligence is informational only — Decision Queue organizes attention."
        )
    }

    /// Architecture guard — Intelligence must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceIntelligenceError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceIntelligenceState,
    ) -> Result<()> {
        // Single operational event — avoid audit fan-out noise (Batch 5.5).
        let metadata = json!({
            "workspace_id": state.workspace_id,
            "recommendation_count": state.recommended_actions.len(),
            "pending_approvals": state.pending_approvals.len(),
            "blocked_actions": state.blocked_actions.len(),
            "decision_queue_pending": state.decision_queue.pending_count,
            "activity_count": state.activity_graph.activity_count,
            "decision_engine_candidates": state.decision_engine.candidate_count,
            "task_graph_nodes": state.task_graph.node_count,
            "environment_windows": state.environment.window_count,
            "composition_members": state.composition.member_count,
            "purpose_label": state.purpose.label,
            "evolution_insights": state.evolution.insight_count,
            "memory_highlights": state.memory_highlights.len(),
            "preference_highlights": state.preference_highlights.len(),
            "summary_len": state.summary.len(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.intelligence.generated",
            true,
            metadata,
        )?;
        // Batch 9.5 informational coherence signal.
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.coherence.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "decision_queue_pending": state.decision_queue.pending_count,
                "activity_count": state.activity_graph.activity_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
