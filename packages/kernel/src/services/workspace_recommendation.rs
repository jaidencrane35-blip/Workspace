//! Workspace Recommendation Engine (Phase 5).
//!
//! Aggregates Attention + Continuity + Evolution + Purpose + Task Graph +
//! Composition + Decision Queue + Environment into typed next-step suggestions.
//! Never executes, never accepts into planner.
//! Candidate payloads remain regenerable; only lifecycle overlays persist.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{Database, RecommendationLifecycleRepository};
use workspace_domain::{
    build_recommendation_engine_summary, recommendation_engine_now_rfc3339,
    validate_recommendation_engine_workspace_id, ActionProposalError, ActorContext,
    AttentionCategory, DecisionQueue, IntentContext, RecommendationConfidence,
    RecommendationEvidence, RecommendationExplanationView, RecommendationGovernanceRecord,
    RecommendationHistoryEntry, RecommendationItem, RecommendationKind,
    RecommendationLifecycleOverlay, RecommendationLifecycleState, RecommendationOutcome,
    RecommendationDecisionBoundary, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionEngineAcceptance,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
    RecommendationDecisionIntakePackageSeal, RecommendationDecisionIntakeProceedDenial,
    RecommendationDecisionIntakeRequest, RecommendationDecisionReadiness,
    RecommendationOutcomeView,
    RecommendationRelationship, RecommendationReviewActionResult,
    TaskGraph, WorkspaceAttentionState, WorkspaceCompositionState, WorkspaceContinuityState,
    WorkspaceEnvironmentState, WorkspaceEvolutionState, WorkspacePurposeState,
    WorkspaceRecommendationEngineError, WorkspaceRecommendationEngineState,
    WorkspaceRecommendationEngineSummary,
};

use crate::error::{KernelError, Result};
use crate::services::explanation_resolver::resolve_attention_reason_traced;
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    TaskGraphService, WorkspaceActivityGraphService, WorkspaceAttentionService,
    WorkspaceCompositionService, WorkspaceContinuityService, WorkspaceEnvironmentService,
    WorkspaceEvolutionService, WorkspaceIntentService, WorkspacePurposeService,
};

pub(crate) struct WorkspaceRecommendationEngineService;

impl WorkspaceRecommendationEngineService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceRecommendationEngineState> {
        Self::generate_maybe_sealed(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            true,
        )
    }

    /// Lifecycle mutation lookup view — includes terminals still addressable by overlay id.
    /// Never returned to IPC consumers; consumers always use sealed `generate`.
    fn generate_for_lifecycle_mutation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceRecommendationEngineState> {
        Self::generate_maybe_sealed(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            false,
        )
    }

    fn generate_maybe_sealed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        seal_actionable: bool,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let workspace_id = workspace_id.into();
        let attention = WorkspaceAttentionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let purpose = WorkspacePurposeService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let evolution = WorkspaceEvolutionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let workflow =
            WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| WorkspaceIntentService::get_project(db, id.as_str()).ok());
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let decision_queue = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let activity = WorkspaceActivityGraphService::generate_with_decision_queue(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            Some(&decision_queue),
        )?;
        let continuity = WorkspaceContinuityService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &decision_queue,
            &activity,
        )?;
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
        let composition = WorkspaceCompositionService::generate_with_inputs(
            db,
            actor,
            workspace_id.clone(),
            &environment,
            Some(&task_graph),
            &continuity,
            &activity,
            &workflow,
            &decision_queue,
            project.as_ref(),
        )?;
        Self::generate_with_inputs_unsealed(
            db,
            actor,
            &workspace_id,
            &attention,
            &continuity,
            &evolution,
            &purpose,
            Some(&task_graph),
            &composition,
            &decision_queue,
            &environment,
        )
        .map(|state| {
            if seal_actionable {
                Self::seal_actionable_projection(state)
            } else {
                state
            }
        })
    }

    /// Preferred **consumer** path — Intelligence injects shared aggregator inputs.
    ///
    /// Always returns a **sealed** projection (`candidates` actionable-only).
    /// Lifecycle mutation paths must use [`Self::generate_for_lifecycle_mutation`] /
    /// [`Self::generate_with_inputs_unsealed`] instead.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        attention: &WorkspaceAttentionState,
        continuity: &WorkspaceContinuityState,
        evolution: &WorkspaceEvolutionState,
        purpose: &WorkspacePurposeState,
        task_graph: Option<&TaskGraph>,
        composition: &WorkspaceCompositionState,
        decision_queue: &DecisionQueue,
        environment: &WorkspaceEnvironmentState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let state = Self::generate_with_inputs_unsealed(
            db,
            actor,
            workspace_id,
            attention,
            continuity,
            evolution,
            purpose,
            task_graph,
            composition,
            decision_queue,
            environment,
        )?;
        Ok(Self::seal_actionable_projection(state))
    }

    /// Internal unsealed assembly — terminals remain addressable for lifecycle mutation.
    ///
    /// **Not** for IPC or React consumers. Prefer [`Self::generate_with_inputs`].
    #[allow(clippy::too_many_arguments)]
    fn generate_with_inputs_unsealed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        attention: &WorkspaceAttentionState,
        continuity: &WorkspaceContinuityState,
        evolution: &WorkspaceEvolutionState,
        purpose: &WorkspacePurposeState,
        task_graph: Option<&TaskGraph>,
        composition: &WorkspaceCompositionState,
        decision_queue: &DecisionQueue,
        environment: &WorkspaceEnvironmentState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let workspace_id =
            validate_recommendation_engine_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let label = if !purpose.label.is_empty() {
            purpose.label.clone()
        } else {
            composition.label.clone()
        };

        let mut candidates = Vec::new();
        let mut relationships = Vec::new();
        let mut evidence = Vec::new();
        let mut seen = HashSet::new();

        // ResolveBlocker — Attention blockers + Continuity interrupted.
        for item in attention
            .items
            .iter()
            .filter(|i| i.category == AttentionCategory::Blocker)
            .take(3)
        {
            let id = format!("recommendation:resolve_blocker:{}", item.id);
            if !seen.insert(id.clone()) {
                continue;
            }
            candidates.push(RecommendationItem {
                id: id.clone(),
                kind: RecommendationKind::ResolveBlocker,
                title: format!("Resolve blocker: {}", item.title),
                reason: format!(
                    "Attention surfaces \"{}\" as a blocker for current work.",
                    item.title
                ),
                evidence: vec![RecommendationEvidence {
                    id: format!("ev:attention:{}", item.id),
                    source_model: "attention".into(),
                    source_ref: item.id.to_string(),
                    summary: item.explanation.clone(),
                }],
                impact: "Clearing blockers unblocks Purpose progress and reduces Attention load."
                    .into(),
                confidence: RecommendationConfidence::High,
                related_attention_id: Some(item.id.to_string()),
                attention_reasons: item.reasons.clone(),
                related_task_id: None,
                related_purpose_label: Some(purpose.label.clone()),
                related_decision_id: None,
                lifecycle_state: None,
                lifecycle_presented_at: None,
                lifecycle_resolved_at: None,
                lifecycle_resolution_type: None,
                explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
            });
            relationships.push(RecommendationRelationship {
                id: format!("rel:rec-attention:{}", item.id),
                from_id: id,
                to_id: format!("attention:{}", item.id),
                kind: "derived_from".into(),
                explanation: "Recommendation is grounded in an Attention blocker item.".into(),
                evidence: vec!["source Attention Engine".into()],
            });
        }

        // ReviewDecision — Attention requires_decision + DQ pending.
        for item in attention
            .items
            .iter()
            .filter(|i| i.category == AttentionCategory::RequiresDecision)
            .take(3)
        {
            let id = format!("recommendation:review_decision:{}", item.id);
            if !seen.insert(id.clone()) {
                continue;
            }
            candidates.push(RecommendationItem {
                id: id.clone(),
                kind: RecommendationKind::ReviewDecision,
                title: format!("Review decision: {}", item.title),
                reason: format!(
                    "A pending decision needs human attention: \"{}\".",
                    item.title
                ),
                evidence: vec![
                    RecommendationEvidence {
                        id: format!("ev:attention:{}", item.id),
                        source_model: "attention".into(),
                        source_ref: item.id.to_string(),
                        summary: item.explanation.clone(),
                    },
                    RecommendationEvidence {
                        id: format!("ev:dq:{ws}"),
                        source_model: "decision_queue".into(),
                        source_ref: ws.to_string(),
                        summary: format!("{} pending Decision Queue item(s)", decision_queue.pending_count),
                    },
                ],
                impact: "Resolving decisions unblocks Purpose and Continuity next steps.".into(),
                confidence: RecommendationConfidence::High,
                related_attention_id: Some(item.id.to_string()),
                attention_reasons: item.reasons.clone(),
                related_task_id: None,
                related_purpose_label: Some(purpose.label.clone()),
                related_decision_id: Some(item.source_id.clone()),
                lifecycle_state: None,
                lifecycle_presented_at: None,
                lifecycle_resolved_at: None,
                lifecycle_resolution_type: None,
                explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if decision_queue.pending_count > 0
            && !candidates
                .iter()
                .any(|c| c.kind == RecommendationKind::ReviewDecision)
        {
            let id = format!("recommendation:review_decision:queue:{ws}");
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ReviewDecision,
                    title: format!(
                        "Review {} outstanding decision(s)",
                        decision_queue.pending_count
                    ),
                    reason: "Decision Queue reports pending human decisions.".into(),
                    evidence: vec![RecommendationEvidence {
                        id: format!("ev:dq:{ws}"),
                        source_model: "decision_queue".into(),
                        source_ref: ws.to_string(),
                        summary: format!("pending_count={}", decision_queue.pending_count),
                    }],
                    impact: "Clearing the Decision Queue reduces Attention pressure.".into(),
                    confidence: RecommendationConfidence::Medium,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                    outcome: None,
                    decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                    decision_handoff_request: None,
                    decision_engine_acceptance: None,
                    authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // ContinueWork — Continuity suggested next / focus + Purpose.
        if let Some(next) = &continuity.suggested_next_step {
            let id = format!("recommendation:continue_work:{}", next.id);
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ContinueWork,
                    title: format!("Continue: {}", next.title),
                    reason: format!(
                        "Continuity suggests \"{}\" as the next step for \"{}\".",
                        next.title, purpose.label
                    ),
                    evidence: vec![
                        RecommendationEvidence {
                            id: format!("ev:continuity:{}", next.id),
                            source_model: "continuity".into(),
                            source_ref: next.id.to_string(),
                            summary: next.why.clone(),
                        },
                        RecommendationEvidence {
                            id: format!("ev:purpose:{ws}"),
                            source_model: "purpose".into(),
                            source_ref: purpose.workspace_id.clone(),
                            summary: purpose.label.clone(),
                        },
                    ],
                    impact: "Continuing focused work advances Purpose without inventing new tasks."
                        .into(),
                    confidence: RecommendationConfidence::High,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        } else if let Some(focus) = &continuity.current_focus {
            let id = format!("recommendation:continue_work:focus:{}", focus.id);
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ContinueWork,
                    title: format!("Continue focus: {}", focus.title),
                    reason: format!(
                        "Current Continuity focus \"{}\" aligns with Purpose \"{}\".",
                        focus.title, purpose.label
                    ),
                    evidence: vec![RecommendationEvidence {
                        id: format!("ev:continuity:{}", focus.id),
                        source_model: "continuity".into(),
                        source_ref: focus.id.to_string(),
                        summary: focus.summary.clone(),
                    }],
                    impact: "Resuming current focus maintains Continuity.".into(),
                    confidence: RecommendationConfidence::Medium,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                    outcome: None,
                    decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                    decision_handoff_request: None,
                    decision_engine_acceptance: None,
                    authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // RestoreContext — Continuity interrupted + Evolution interrupted.
        if !continuity.interrupted_work.is_empty() {
            let facet = &continuity.interrupted_work[0];
            let id = format!("recommendation:restore_context:{}", facet.id);
            if seen.insert(id.clone()) {
                let mut ev = vec![RecommendationEvidence {
                    id: format!("ev:continuity:{}", facet.id),
                    source_model: "continuity".into(),
                    source_ref: facet.id.to_string(),
                    summary: facet.title.clone(),
                }];
                if evolution
                    .insights
                    .iter()
                    .any(|i| {
                        matches!(
                            i.kind,
                            workspace_domain::EvolutionInsightKind::InterruptedWork
                        )
                    })
                {
                    ev.push(RecommendationEvidence {
                        id: format!("ev:evolution:{ws}"),
                        source_model: "evolution".into(),
                        source_ref: ws.to_string(),
                        summary: "Evolution reports interrupted-work insight.".into(),
                    });
                }
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::RestoreContext,
                    title: format!("Restore context: {}", facet.title),
                    reason: format!(
                        "Work was interrupted (\"{}\"); restoring context helps resume Purpose.",
                        facet.title
                    ),
                    evidence: ev,
                    impact: "Restored context reduces Continuity interruption drag.".into(),
                    confidence: RecommendationConfidence::High,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // CompleteTask — open Task Graph nodes.
        if let Some(graph) = task_graph {
            for node in graph.open_incomplete_nodes().into_iter().take(3) {
                let id = format!("recommendation:complete_task:{}", node.task.id);
                if !seen.insert(id.clone()) {
                    continue;
                }
                let confidence = if node.task.progress_percent >= 50 {
                    RecommendationConfidence::High
                } else {
                    RecommendationConfidence::Medium
                };
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::CompleteTask,
                    title: format!("Complete task: {}", node.task.title),
                    reason: format!(
                        "Task Graph lists \"{}\" as open ({}, {}%).",
                        node.task.title,
                        node.task.status.as_str(),
                        node.task.progress_percent
                    ),
                    evidence: vec![RecommendationEvidence {
                        id: format!("ev:task:{}", node.task.id),
                        source_model: "task_graph".into(),
                        source_ref: node.task.id.to_string(),
                        summary: node.task.explanation.clone(),
                    }],
                    impact: "Completing open Task Graph work advances Purpose progress.".into(),
                    confidence,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: Some(node.task.id.to_string()),
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // ReorganizeWorkspace — Composition gaps / Environment disconnected.
        if composition.missing_application_count > 0 || !composition.gaps.is_empty() {
            let id = format!("recommendation:reorganize:{ws}");
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ReorganizeWorkspace,
                    title: format!("Improve working environment: {}", composition.label),
                    reason: format!(
                        "Composition reports {} missing application(s) for Purpose \"{}\".",
                        composition.missing_application_count, purpose.label
                    ),
                    evidence: vec![
                        RecommendationEvidence {
                            id: format!("ev:composition:{ws}"),
                            source_model: "composition".into(),
                            source_ref: composition.workspace_id.clone(),
                            summary: composition.summary.clone(),
                        },
                        RecommendationEvidence {
                            id: format!("ev:environment:{ws}"),
                            source_model: "environment".into(),
                            source_ref: environment.workspace_id.clone(),
                            summary: environment.summary.clone(),
                        },
                    ],
                    impact: "A coherent environment makes Purpose work easier to continue.".into(),
                    confidence: RecommendationConfidence::Medium,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        } else if environment.disconnected_work {
            let id = format!("recommendation:reorganize:disconnected:{ws}");
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ReorganizeWorkspace,
                    title: "Reconnect desktop to active work".into(),
                    reason: "Environment Model reports active work appears disconnected from open windows."
                        .into(),
                    evidence: vec![RecommendationEvidence {
                        id: format!("ev:environment:{ws}"),
                        source_model: "environment".into(),
                        source_ref: environment.workspace_id.clone(),
                        summary: environment.summary.clone(),
                    }],
                    impact: "Aligning desktop apps with Purpose reduces Continuity friction.".into(),
                    confidence: RecommendationConfidence::Medium,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                    outcome: None,
                    decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                    decision_handoff_request: None,
                    decision_engine_acceptance: None,
                    authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // ExploreOpportunity — Evolution purpose progression without blockers.
        if candidates.len() < 3
            && evolution
                .insights
                .iter()
                .any(|i| {
                    matches!(
                        i.kind,
                        workspace_domain::EvolutionInsightKind::PurposeProgression
                            | workspace_domain::EvolutionInsightKind::TaskProgression
                    )
                })
        {
            let id = format!("recommendation:explore:{ws}");
            if seen.insert(id.clone()) {
                candidates.push(RecommendationItem {
                    id,
                    kind: RecommendationKind::ExploreOpportunity,
                    title: format!("Explore next progress on \"{}\"", purpose.label),
                    reason: "Evolution shows purpose/task progression with room for further useful work."
                        .into(),
                    evidence: vec![RecommendationEvidence {
                        id: format!("ev:evolution:{ws}"),
                        source_model: "evolution".into(),
                        source_ref: ws.to_string(),
                        summary: evolution.summary.clone(),
                    }],
                    impact: "Exploring next progress keeps Momentum without inventing tasks.".into(),
                    confidence: RecommendationConfidence::Low,
                    related_attention_id: None,
                    attention_reasons: Vec::new(),
                    related_task_id: None,
                    related_purpose_label: Some(purpose.label.clone()),
                    related_decision_id: None,
                    lifecycle_state: None,
                    lifecycle_presented_at: None,
                    lifecycle_resolved_at: None,
                    lifecycle_resolution_type: None,
                    explanation: None,
                    outcome: None,
                    decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                    decision_handoff_request: None,
                    decision_engine_acceptance: None,
                    authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // Deterministic ordering: kind priority then id.
        candidates.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        // Cap for product clarity.
        if candidates.len() > 12 {
            candidates.truncate(12);
        }
        relationships.sort_by(|a, b| a.id.cmp(&b.id));

        evidence.push(format!("Attention items: {}", attention.items.len()));
        evidence.push(format!("Purpose: {}", purpose.label));
        evidence.push(format!("Evolution insights: {}", evolution.insight_count));
        evidence.push(format!(
            "Decision Queue pending: {}",
            decision_queue.pending_count
        ));

        let explanation = format!(
            "Recommendations for \"{label}\" suggest possible useful next steps from Attention, \
             Continuity, Evolution, Purpose, Task Graph, Composition, Decision Queue, and Environment. \
             They are suggestions only — distinct from Decision Engine accept/handoff and from \
             Intelligence Attention projections."
        );
        let summary = build_recommendation_engine_summary(&label, candidates.len());

        let state = WorkspaceRecommendationEngineState {
            workspace_id: ws.to_string(),
            generated_at: recommendation_engine_now_rfc3339(),
            label,
            candidate_count: candidates.len(),
            relationship_count: relationships.len(),
            candidates,
            relationships,
            history: Vec::new(),
            history_count: 0,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceRecommendationEngineState::AUTHORITY_EFFECT_NONE.into(),
        };

        let state = Self::project_lifecycle_overlays(db, actor, state)?;
        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceRecommendationEngineState,
        limit: usize,
    ) -> WorkspaceRecommendationEngineSummary {
        state.summary_projection(limit)
    }

    /// Merge Pattern Model observations into recommendations as evidence-only context.
    /// Does not regenerate Pattern or Attention — avoids circular regen.
    pub(crate) fn enrich_with_patterns(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        recommendations: &WorkspaceRecommendationEngineState,
        patterns: &workspace_domain::WorkspacePatternState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let mut seen: HashSet<String> = state.candidates.iter().map(|c| c.id.clone()).collect();
        for pattern in patterns.patterns.iter().take(3) {
            let id = format!("recommendation:from_pattern:{}", pattern.id);
            if !seen.insert(id.clone()) {
                continue;
            }
            let kind = match pattern.kind {
                workspace_domain::PatternKind::DecisionPattern => {
                    RecommendationKind::ReviewDecision
                }
                workspace_domain::PatternKind::ApplicationPattern
                | workspace_domain::PatternKind::EnvironmentPattern => {
                    RecommendationKind::ReorganizeWorkspace
                }
                workspace_domain::PatternKind::TaskPattern => RecommendationKind::CompleteTask,
                workspace_domain::PatternKind::WorkflowPattern => RecommendationKind::ContinueWork,
            };
            let candidate = RecommendationItem {
                id: id.clone(),
                kind,
                title: format!("Consider pattern: {}", pattern.title),
                reason: format!(
                    "Pattern observation: {}. Suggestion only — patterns never execute.",
                    pattern.observation
                ),
                evidence: pattern
                    .evidence
                    .iter()
                    .map(|e| RecommendationEvidence {
                        id: e.id.clone(),
                        source_model: format!("pattern:{}", e.source_model),
                        source_ref: e.source_ref.clone(),
                        summary: e.summary.clone(),
                    })
                    .collect(),
                impact: pattern.impact.clone(),
                confidence: match pattern.confidence {
                    workspace_domain::PatternConfidence::High => RecommendationConfidence::High,
                    workspace_domain::PatternConfidence::Medium => RecommendationConfidence::Medium,
                    workspace_domain::PatternConfidence::Low => RecommendationConfidence::Low,
                },
                related_attention_id: None,
                attention_reasons: Vec::new(),
                related_task_id: None,
                related_purpose_label: Some(patterns.label.clone()),
                related_decision_id: None,
                lifecycle_state: None,
                lifecycle_presented_at: None,
                lifecycle_resolved_at: None,
                lifecycle_resolution_type: None,
                explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
            };
            crate::services::WorkspacePatternService::audit_used_for_recommendation(
                db,
                actor,
                pattern,
                &id,
            )?;
            state.candidates.push(candidate);
        }
        state.candidates.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        if state.candidates.len() > 12 {
            state.candidates.truncate(12);
        }
        state.candidate_count = state.candidates.len();
        state.evidence.push(format!(
            "Pattern Model patterns used as evidence: {}",
            patterns.pattern_count
        ));
        state.summary = build_recommendation_engine_summary(&state.label, state.candidate_count);
        Ok(Self::seal_actionable_projection(Self::project_lifecycle_overlays(
            db, actor, state,
        )?))
    }

    /// Merge Readiness gaps into recommendations as evidence-only context.
    /// Does not regenerate Readiness — avoids circular regen.
    pub(crate) fn enrich_with_readiness(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        recommendations: &WorkspaceRecommendationEngineState,
        readiness: &workspace_domain::WorkspaceReadinessState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let mut seen: HashSet<String> = state.candidates.iter().map(|c| c.id.clone()).collect();
        for gap in readiness
            .assessments
            .iter()
            .flat_map(|a| a.gaps.iter())
            .take(4)
        {
            let id = format!("recommendation:from_readiness:{}", gap.id);
            if !seen.insert(id.clone()) {
                continue;
            }
            let kind = match gap.kind.as_str() {
                "pending_approval" | "pending_decisions" => RecommendationKind::ReviewDecision,
                "blocked_tasks" | "unresolved_dependencies" | "continuity_blocker" => {
                    RecommendationKind::ResolveBlocker
                }
                "interrupted_work" | "thin_context" => RecommendationKind::RestoreContext,
                "missing_applications" | "disconnected_work" | "composition_gap" => {
                    RecommendationKind::ReorganizeWorkspace
                }
                _ => RecommendationKind::ContinueWork,
            };
            state.candidates.push(RecommendationItem {
                id,
                kind,
                title: format!("Address readiness gap: {}", gap.title),
                reason: format!(
                    "Readiness gap ({}): {}. Suggestion only — readiness never prepares or executes.",
                    gap.kind, gap.explanation
                ),
                evidence: vec![RecommendationEvidence {
                    id: format!("ev:readiness:{}", gap.id),
                    source_model: format!("readiness:{}", gap.source_model),
                    source_ref: gap.source_ref.clone(),
                    summary: gap.impact.clone(),
                }],
                impact: gap.impact.clone(),
                confidence: match readiness.overall_status {
                    workspace_domain::ReadinessStatus::Blocked => RecommendationConfidence::High,
                    workspace_domain::ReadinessStatus::PartiallyReady => {
                        RecommendationConfidence::Medium
                    }
                    workspace_domain::ReadinessStatus::Ready => RecommendationConfidence::Low,
                },
                related_attention_id: None,
                attention_reasons: Vec::new(),
                related_task_id: None,
                related_purpose_label: Some(readiness.label.clone()),
                related_decision_id: None,
                lifecycle_state: None,
                lifecycle_presented_at: None,
                lifecycle_resolved_at: None,
                lifecycle_resolution_type: None,
                explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                decision_engine_acceptance: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        state.candidates.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        if state.candidates.len() > 12 {
            state.candidates.truncate(12);
        }
        state.candidate_count = state.candidates.len();
        state.evidence.push(format!(
            "Readiness gaps used as evidence: {} (status: {})",
            readiness.gap_count,
            readiness.overall_status.as_str()
        ));
        state.summary = build_recommendation_engine_summary(&state.label, state.candidate_count);
        Ok(Self::seal_actionable_projection(Self::project_lifecycle_overlays(
            db, actor, state,
        )?))
    }

    /// Reference Milestones without mutating milestone ownership or ranking authority.
    pub(crate) fn enrich_with_milestones(
        recommendations: &WorkspaceRecommendationEngineState,
        milestones: &workspace_domain::WorkspaceMilestoneState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let marker = format!(
            "milestones:current={:?},blocked={},count={}",
            milestones.current_milestone().map(|m| &m.title),
            milestones.blocked_count,
            milestones.milestone_count
        );
        if !state.evidence.iter().any(|e| e.starts_with("milestones:")) {
            state.evidence.push(marker.clone());
        }
        state.explanation = format!(
            "{} References Milestones as evidence only ({}) — does not change Milestones.",
            state.explanation, marker
        );
        Ok(state)
    }

    /// Reference Working Style as evidence only — never creates actions from style.
    pub(crate) fn enrich_with_working_style(
        recommendations: &WorkspaceRecommendationEngineState,
        working_style: &workspace_domain::WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let marker = format!(
            "working_style:obs={},prefs={},rhythm={}",
            working_style.observation_count,
            working_style.preference_count,
            working_style.rhythm_count
        );
        if !state.evidence.iter().any(|e| e.starts_with("working_style:")) {
            state.evidence.push(marker.clone());
        }
        state.explanation = format!(
            "{} References Working Style as evidence only ({}) — does not change Working Style \
             or create actions from observations.",
            state.explanation, marker
        );
        Ok(state)
    }

    /// Reference Transitions as evidence only — never performs transitions.
    pub(crate) fn enrich_with_transition(
        recommendations: &WorkspaceRecommendationEngineState,
        transition: &workspace_domain::WorkspaceTransitionState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let marker = format!(
            "transition:count={},returning={},current={:?}",
            transition.transition_count,
            transition.returning_count,
            transition.current_transition().map(|t| &t.title)
        );
        if !state.evidence.iter().any(|e| e.starts_with("transition:")) {
            state.evidence.push(marker.clone());
        }
        state.explanation = format!(
            "{} References Transitions as evidence only ({}) — does not restore or execute.",
            state.explanation, marker
        );
        Ok(state)
    }

    /// Reference Navigation without mutating navigation ownership or ranking authority.
    pub(crate) fn enrich_with_navigation(
        recommendations: &WorkspaceRecommendationEngineState,
        navigation: &workspace_domain::WorkspaceNavigationState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        let marker = format!(
            "navigation:nodes={},blocked={},next={}",
            navigation.node_count,
            navigation.blocked_count,
            navigation.navigation_summary.next_inspection_line
        );
        if !state.evidence.iter().any(|e| e.starts_with("navigation:")) {
            state.evidence.push(marker.clone());
        }
        state.explanation = format!(
            "{} References Navigation as evidence only ({}) — does not change Navigation.",
            state.explanation, marker
        );
        Ok(state)
    }

    /// Reference Work Context without mutating context ownership or ranking authority.
    /// Appends evidence / explanation only — does not regenerate Work Context.
    pub(crate) fn enrich_with_work_context(
        recommendations: &WorkspaceRecommendationEngineState,
        work_context: &workspace_domain::WorkspaceWorkContextState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let mut state = recommendations.clone();
        if let Some(primary) = work_context.primary_context() {
            let marker = format!(
                "work_context:{}:{} ({})",
                primary.name,
                primary.context_type.as_str(),
                primary.confidence.as_str()
            );
            if !state.evidence.iter().any(|e| e.contains("work_context:")) {
                state.evidence.push(marker.clone());
            }
            state.explanation = format!(
                "{} References Work Context as evidence only ({}) — does not change Work Context.",
                state.explanation, marker
            );
            for candidate in state.candidates.iter_mut().take(3) {
                candidate.evidence.push(workspace_domain::RecommendationEvidence {
                    id: format!("ev:work_context:{}", primary.id),
                    source_model: "work_context".into(),
                    source_ref: primary.id.clone(),
                    summary: format!(
                        "Current work kind appears to be {} — suggestion framing only.",
                        primary.name
                    ),
                });
            }
            Self::attach_explanation_views(&mut state)?;
            Self::attach_decision_readiness(
                &mut state,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
            )?;
        }
        Ok(state)
    }

    /// Mark a recommendation as presented on a human surface — lifecycle only.
    pub(crate) fn present_recommendation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::review_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            RecommendationLifecycleState::Presented,
            "workspace.recommendation_engine.presented",
            "Presented for human review — still a proposal only.",
        )
    }

    /// Record human acceptance — lifecycle + outcome only. Never executes.
    pub(crate) fn accept_recommendation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::review_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            RecommendationLifecycleState::Accepted,
            "workspace.recommendation_engine.accepted",
            "Accepted as a human decision record only — does not execute or grant authority. Any follow-through must use Intent → Command Pipeline → Permission Gateway.",
        )
    }

    /// Record human rejection — lifecycle + outcome only. Never executes.
    pub(crate) fn reject_recommendation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::review_transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            RecommendationLifecycleState::Rejected,
            "workspace.recommendation_engine.rejected",
            "Rejected as a human decision record only — valid outcome, not a system failure.",
        )
    }

    /// Confirm desire for *future* Decision Engine consideration — never creates DE/intent.
    pub(crate) fn confirm_recommendation_decision(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
        confirmation_intent: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::update_confirmation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            confirmation_intent.into(),
            true,
            "workspace.recommendation_engine.decision_confirmed",
            "Confirmation recorded for future Decision Engine consideration only — does not create a Decision object, intent, or execution authority.",
        )
    }

    /// Decline future Decision Engine consideration — never executes.
    pub(crate) fn decline_recommendation_decision(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::update_confirmation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            RecommendationDecisionConfirmation::INTENT_AGREEMENT_ONLY.into(),
            false,
            "workspace.recommendation_engine.decision_declined",
            "Future Decision Engine consideration declined — recommendation agreement (if any) remains; no Decision object, intent, or execution.",
        )
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            WorkspaceRecommendationEngineError::CannotExecute,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn review_transition(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
        to: RecommendationLifecycleState,
        audit_event: &str,
        explanation: &str,
    ) -> Result<RecommendationReviewActionResult> {
        let workspace_id = workspace_id.into();
        let recommendation_id = recommendation_id.into();
        let state = Self::generate_for_lifecycle_mutation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let item = state
            .candidates
            .iter()
            .find(|c| c.id == recommendation_id)
            .cloned()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;

        let now = recommendation_engine_now_rfc3339();
        let actor_id = actor.actor.id.to_string();
        let overlay = Self::load_overlay(db, &workspace_id, &recommendation_id)?
            .unwrap_or_else(|| {
                let record = RecommendationGovernanceRecord::from_recommendation_item(&item, &now);
                RecommendationLifecycleOverlay::from_governance_record(
                    workspace_id.clone(),
                    &record,
                    None,
                    now.clone(),
                )
            });

        let mut record = RecommendationGovernanceRecord::from_recommendation_item(&item, &overlay.created_at);
        record.lifecycle.state = overlay.lifecycle_state;
        record.lifecycle.presented_at = overlay.presented_at.clone();
        record.lifecycle.resolved_at = overlay.resolved_at.clone();
        record.lifecycle.resolution_type = overlay.resolution_type;
        record.lifecycle.transition_actor_id = overlay.actor_id.clone();

        // Ensure Available before Present/Accept/Reject when still Created.
        if record.lifecycle.state == RecommendationLifecycleState::Created {
            record
                .transition(
                    RecommendationLifecycleState::Available,
                    now.clone(),
                    Some(actor_id.clone()),
                )
                .map_err(map_lifecycle_err)?;
        }
        // Accept requires Presented — auto-present when still Available.
        if to == RecommendationLifecycleState::Accepted
            && record.lifecycle.state == RecommendationLifecycleState::Available
        {
            record
                .transition(
                    RecommendationLifecycleState::Presented,
                    now.clone(),
                    Some(actor_id.clone()),
                )
                .map_err(map_lifecycle_err)?;
        }

        record
            .transition(to, now.clone(), Some(actor_id.clone()))
            .map_err(map_lifecycle_err)?;

        let outcome = if matches!(
            to,
            RecommendationLifecycleState::Accepted | RecommendationLifecycleState::Rejected
        ) {
            Some(Self::record_outcome_with_experience(&record, &item, &now)?)
        } else {
            overlay.outcome.clone()
        };

        let mut next = RecommendationLifecycleOverlay::from_governance_record(
            workspace_id.clone(),
            &record,
            outcome.clone(),
            now.clone(),
        )
        .with_prior_outcomes(overlay.prior_outcomes.clone());
        next.content_fingerprint = overlay
            .content_fingerprint
            .or_else(|| Some(item.continuity_fingerprint()));

        let mut assessed = item;
        next.apply_to_item(&mut assessed);
        if let Some(ref recorded) = outcome {
            assessed.outcome = Some(project_outcome_view(recorded));
        }
        if assessed.explanation.is_none() {
            assessed.explanation = Some(RecommendationExplanationView::from_item(&assessed));
        }
        let history_refs: Vec<String> = next
            .carried_outcomes()
            .into_iter()
            .map(|o| o.id)
            .collect();
        let decision_context =
            RecommendationDecisionContext::assemble(&workspace_id, &assessed, &history_refs);

        // Enforce resilience invariants: Recommendation Engine must not have handoff/intent capability.
        crate::services::validate_decision_context_boundary(&decision_context)?;

        let decision_readiness =
            RecommendationDecisionReadiness::assess_from_context(&decision_context);

        // Enforce resilience invariants: Authority must be "none"; cannot create commands.
        crate::services::validate_decision_readiness_boundary(&decision_readiness)?;

        let decision_boundary = RecommendationDecisionBoundary::from_context_and_readiness(
            &decision_context,
            &decision_readiness,
        );

        // Enforce resilience invariants: Decision boundary cannot grant authority or create intent.
        crate::services::validate_decision_boundary_constraints(&decision_boundary)?;

        // Accept ≠ confirmation: derive/required at most; never auto-confirm.
        let decision_confirmation = match (
            to,
            overlay.decision_confirmation.clone(),
        ) {
            (RecommendationLifecycleState::Accepted, _) => {
                RecommendationDecisionConfirmation::derive_from_boundary(&decision_boundary)
            }
            (_, Some(existing)) => existing,
            (_, None) => {
                RecommendationDecisionConfirmation::derive_from_boundary(&decision_boundary)
            }
        };

        // Enforce resilience invariants: Confirmation must be non-authoritative.
        crate::services::validate_decision_confirmation_non_authoritative(&decision_confirmation)?;
        next.decision_confirmation = Some(decision_confirmation.clone());
        // Accept never emits intake — confirmation required first.
        let decision_intake = RecommendationDecisionIntakeRequest::try_assemble(
            &decision_context,
            &decision_readiness,
            &decision_confirmation,
        );
        crate::services::validate_accept_emits_no_intake(decision_intake.is_none())?;
        let decision_intake_inspection = None;
        let decision_intake_compatibility = None;
        let decision_intake_proceed_denial = None;
        let decision_intake_package_seal = None;
        let decision_intake_adapter_preparation = None;
        let decision_handoff_request = None;
        let decision_engine_acceptance = None;

        Self::upsert_overlay(db, &next)?;
        Self::audit_lifecycle(db, actor, audit_event, &assessed, &next)?;

        Ok(RecommendationReviewActionResult {
            workspace_id,
            recommendation_id,
            lifecycle_state: next.lifecycle_state.as_str().into(),
            outcome,
            decision_context: Some(decision_context),
            decision_readiness: Some(decision_readiness),
            decision_boundary: Some(decision_boundary),
            decision_confirmation: Some(decision_confirmation),
            decision_intake,
            decision_intake_inspection,
            decision_intake_compatibility,
            decision_intake_proceed_denial,
            decision_intake_package_seal,
            decision_intake_adapter_preparation,
            decision_handoff_request,
            decision_engine_acceptance,
            explanation: explanation.into(),
            authority_effect: RecommendationReviewActionResult::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn update_confirmation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
        confirmation_intent: String,
        confirm: bool,
        audit_event: &str,
        explanation: &str,
    ) -> Result<RecommendationReviewActionResult> {
        let workspace_id = workspace_id.into();
        let recommendation_id = recommendation_id.into();
        let state = Self::generate_for_lifecycle_mutation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let item = state
            .candidates
            .iter()
            .find(|c| c.id == recommendation_id)
            .cloned()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let mut overlay = Self::load_overlay(db, &workspace_id, &recommendation_id)?
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;

        let boundary = item.decision_boundary.clone().unwrap_or_else(|| {
            let ctx = RecommendationDecisionContext::assemble(&workspace_id, &item, &[]);
            let readiness = RecommendationDecisionReadiness::assess_from_context(&ctx);
            RecommendationDecisionBoundary::from_context_and_readiness(&ctx, &readiness)
        });
        let mut confirmation = overlay.decision_confirmation.clone().unwrap_or_else(|| {
            RecommendationDecisionConfirmation::derive_from_boundary(&boundary)
        });
        let now = recommendation_engine_now_rfc3339();
        if confirm {
            confirmation
                .confirm(&confirmation_intent, &now)
                .map_err(|e| KernelError::from(e))?;
        } else {
            confirmation.decline(&now).map_err(|e| KernelError::from(e))?;
        }
        crate::services::validate_confirmation_post_action(&confirmation)?;

        overlay.decision_confirmation = Some(confirmation.clone());
        overlay.updated_at = now.clone();
        overlay.actor_id = Some(actor.actor.id.to_string());

        let context = item.decision_context.clone().unwrap_or_else(|| {
            RecommendationDecisionContext::assemble(&workspace_id, &item, &[])
        });
        let readiness = item.decision_readiness.clone().unwrap_or_else(|| {
            RecommendationDecisionReadiness::assess_from_context(&context)
        });
        let decision_intake = RecommendationDecisionIntakeRequest::try_assemble(
            &context,
            &readiness,
            &confirmation,
        );
        let decision_intake_inspection = match decision_intake.as_ref() {
            Some(intake) => {
                let inspection = RecommendationDecisionIntakeInspection::verify(
                    intake,
                    &context,
                    &confirmation,
                    &readiness,
                );
                crate::services::validate_intake_inspection_boundary(&inspection)?;
                Some(inspection)
            }
            None => None,
        };
        let decision_intake_compatibility =
            match (decision_intake.as_ref(), decision_intake_inspection.as_ref()) {
                (Some(intake), Some(inspection)) => {
                    let compatibility =
                        RecommendationDecisionIntakeCompatibility::derive_from_inspection(
                            intake, inspection,
                        );
                    crate::services::validate_intake_compatibility_boundary(&compatibility)?;
                    Some(compatibility)
                }
                _ => None,
            };
        let decision_intake_proceed_denial = match decision_intake_compatibility.as_ref() {
            Some(compat) => {
                let denial =
                    RecommendationDecisionIntakeProceedDenial::derive_from_compatibility(compat);
                crate::services::validate_intake_proceed_denial_boundary(&denial)?;
                Some(denial)
            }
            None => None,
        };
        let decision_intake_package_seal = match (
            decision_intake.as_ref(),
            decision_intake_compatibility.as_ref(),
            decision_intake_proceed_denial.as_ref(),
        ) {
            (Some(intake), Some(compat), Some(denial)) if confirm => {
                let seal = RecommendationDecisionIntakePackageSeal::derive_from_proceed_denial(
                    intake, compat, denial, &now,
                );
                crate::services::validate_intake_package_seal_boundary(&seal, Some(intake))?;
                Some(seal)
            }
            _ => None,
        };
        let decision_intake_adapter_preparation = match (
            decision_intake.as_ref(),
            decision_intake_package_seal.as_ref(),
        ) {
            (Some(intake), Some(seal)) if confirm => {
                let prep = RecommendationDecisionIntakeAdapterPreparation::try_prepare(
                    intake,
                    &confirmation,
                    seal,
                    &now,
                );
                if let Some(ref preparation) = prep {
                    crate::services::validate_adapter_preparation_boundary(preparation, Some(true))?;
                }
                prep
            }
            _ => None,
        };
        let decision_handoff_request = match (
            decision_intake_adapter_preparation.as_ref(),
            decision_intake_package_seal.as_ref(),
            decision_intake_compatibility.as_ref(),
        ) {
            (Some(prep), Some(seal), Some(compat)) if confirm => {
                let request = RecommendationDecisionHandoffRequest::try_request(
                    prep,
                    &confirmation,
                    seal,
                    compat,
                    &now,
                );
                if let Some(ref handoff_request) = request {
                    crate::services::validate_handoff_request_boundary(handoff_request, Some(true))?;
                }
                request
            }
            _ => None,
        };
        let decision_engine_acceptance = match decision_handoff_request.as_ref() {
            Some(request) => {
                let acceptance =
                    RecommendationDecisionEngineAcceptance::derive_from_handoff_request(request);
                if let Some(ref boundary) = acceptance {
                    crate::services::validate_engine_acceptance_boundary(
                        boundary,
                        Some(true),
                        None,
                    )?;
                }
                acceptance
            }
            None => None,
        };
        overlay.decision_intake_package_seal = decision_intake_package_seal.clone();
        overlay.decision_intake_adapter_preparation = decision_intake_adapter_preparation.clone();
        overlay.decision_handoff_request = decision_handoff_request.clone();
        overlay.decision_engine_acceptance = decision_engine_acceptance.clone();
        Self::upsert_overlay(db, &overlay)?;
        Self::audit_lifecycle(db, actor, audit_event, &item, &overlay)?;

        if let Some(ref intake) = decision_intake {
            crate::services::validate_intake_request_non_authoritative(intake)?;
        }
        if !confirm {
            crate::services::validate_decline_clears_intake_pipeline(
                decision_intake.is_none(),
                decision_intake_inspection.is_none(),
                decision_intake_compatibility.is_none(),
                decision_intake_proceed_denial.is_none(),
                decision_intake_package_seal.is_none(),
                decision_intake_adapter_preparation.is_none(),
                decision_handoff_request.is_none(),
                decision_engine_acceptance.is_none(),
            )?;
        }

        Ok(RecommendationReviewActionResult {
            workspace_id,
            recommendation_id,
            lifecycle_state: overlay.lifecycle_state.as_str().into(),
            outcome: overlay.outcome.clone(),
            decision_context: Some(context),
            decision_readiness: Some(readiness),
            decision_boundary: Some(boundary),
            decision_confirmation: Some(confirmation),
            decision_intake,
            decision_intake_inspection,
            decision_intake_compatibility,
            decision_intake_proceed_denial,
            decision_intake_package_seal,
            decision_intake_adapter_preparation,
            decision_handoff_request,
            decision_engine_acceptance,
            explanation: explanation.into(),
            authority_effect: RecommendationReviewActionResult::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Revoke prepared adapter path — reversible; never creates DE objects or invokes adapter.
    pub(crate) fn revoke_recommendation_adapter_preparation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        let workspace_id = workspace_id.into();
        let recommendation_id = recommendation_id.into();
        let state = Self::generate_for_lifecycle_mutation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let item = state
            .candidates
            .iter()
            .find(|c| c.id == recommendation_id)
            .cloned()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let mut overlay = Self::load_overlay(db, &workspace_id, &recommendation_id)?
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let mut preparation = overlay
            .decision_intake_adapter_preparation
            .clone()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let now = recommendation_engine_now_rfc3339();
        preparation
            .revoke(&now)
            .map_err(|e| KernelError::from(e))?;
        crate::services::validate_adapter_preparation_boundary(&preparation, Some(false))?;
        let mut handoff_request = overlay.decision_handoff_request.clone();
        if let Some(ref mut request) = handoff_request {
            request.revoke(&now).map_err(|e| KernelError::from(e))?;
            crate::services::validate_handoff_request_boundary(request, Some(false))?;
        }
        let mut engine_acceptance = overlay.decision_engine_acceptance.clone();
        if let Some(ref mut acceptance) = engine_acceptance {
            acceptance.revoke(&now).map_err(|e| KernelError::from(e))?;
            crate::services::validate_engine_acceptance_boundary(
                acceptance,
                Some(false),
                None,
            )?;
        }
        overlay.decision_intake_adapter_preparation = Some(preparation.clone());
        overlay.decision_handoff_request = handoff_request.clone();
        overlay.decision_engine_acceptance = engine_acceptance.clone();
        overlay.updated_at = now;
        overlay.actor_id = Some(actor.actor.id.to_string());
        Self::upsert_overlay(db, &overlay)?;
        Self::audit_lifecycle(
            db,
            actor,
            "workspace.recommendation_engine.adapter_preparation_revoked",
            &item,
            &overlay,
        )?;

        Ok(RecommendationReviewActionResult {
            workspace_id,
            recommendation_id,
            lifecycle_state: overlay.lifecycle_state.as_str().into(),
            outcome: overlay.outcome.clone(),
            decision_context: item.decision_context,
            decision_readiness: item.decision_readiness,
            decision_boundary: item.decision_boundary,
            decision_confirmation: overlay.decision_confirmation.clone(),
            decision_intake: item.decision_intake,
            decision_intake_inspection: item.decision_intake_inspection,
            decision_intake_compatibility: item.decision_intake_compatibility,
            decision_intake_proceed_denial: item.decision_intake_proceed_denial,
            decision_intake_package_seal: overlay.decision_intake_package_seal.clone(),
            decision_intake_adapter_preparation: Some(preparation),
            decision_handoff_request: handoff_request,
            decision_engine_acceptance: engine_acceptance,
            explanation: "Adapter preparation revoked — no DE objects or Gateway grants created."
                .into(),
            authority_effect: RecommendationReviewActionResult::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Record Decision Engine acceptance of handoff request — never transfers ownership or creates DE objects.
    pub(crate) fn accept_recommendation_decision_engine_acceptance(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::update_decision_engine_acceptance(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            true,
            "workspace.recommendation_engine.decision_engine_accepted",
            "Decision Engine acceptance recorded for future ownership only — no transfer, DE object, intent, or Gateway grant.",
        )
    }

    /// Decline Decision Engine acceptance of handoff request — never executes.
    pub(crate) fn decline_recommendation_decision_engine_acceptance(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
    ) -> Result<RecommendationReviewActionResult> {
        Self::update_decision_engine_acceptance(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            recommendation_id,
            false,
            "workspace.recommendation_engine.decision_engine_declined",
            "Decision Engine declined handoff request — Recommendation Engine retains ownership; no DE object or Gateway grant.",
        )
    }

    fn update_decision_engine_acceptance(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        recommendation_id: impl Into<String>,
        accept: bool,
        audit_event: &str,
        explanation: &str,
    ) -> Result<RecommendationReviewActionResult> {
        let workspace_id = workspace_id.into();
        let recommendation_id = recommendation_id.into();
        let state = Self::generate_for_lifecycle_mutation(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let item = state
            .candidates
            .iter()
            .find(|c| c.id == recommendation_id)
            .cloned()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let mut overlay = Self::load_overlay(db, &workspace_id, &recommendation_id)?
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        let handoff = overlay
            .decision_handoff_request
            .clone()
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        if !handoff.is_active_request() {
            return Err(KernelError::from(
                WorkspaceRecommendationEngineError::AcceptanceCannotCreateAuthority,
            ));
        }
        let mut acceptance = overlay
            .decision_engine_acceptance
            .clone()
            .or_else(|| {
                RecommendationDecisionEngineAcceptance::derive_from_handoff_request(&handoff)
            })
            .ok_or_else(|| KernelError::from(WorkspaceRecommendationEngineError::NotFound))?;
        acceptance = acceptance.rebind_to_handoff_request(&handoff);
        let now = recommendation_engine_now_rfc3339();
        if accept {
            acceptance.accept(&now).map_err(KernelError::from)?;
            crate::services::validate_engine_acceptance_boundary(
                &acceptance,
                None,
                Some(true),
            )?;
        } else {
            acceptance.decline(&now).map_err(KernelError::from)?;
            crate::services::validate_engine_acceptance_boundary(
                &acceptance,
                None,
                Some(false),
            )?;
        }
        overlay.decision_engine_acceptance = Some(acceptance.clone());
        overlay.updated_at = now;
        overlay.actor_id = Some(actor.actor.id.to_string());
        Self::upsert_overlay(db, &overlay)?;
        Self::audit_lifecycle(db, actor, audit_event, &item, &overlay)?;

        Ok(RecommendationReviewActionResult {
            workspace_id,
            recommendation_id,
            lifecycle_state: overlay.lifecycle_state.as_str().into(),
            outcome: overlay.outcome.clone(),
            decision_context: item.decision_context,
            decision_readiness: item.decision_readiness,
            decision_boundary: item.decision_boundary,
            decision_confirmation: overlay.decision_confirmation.clone(),
            decision_intake: item.decision_intake,
            decision_intake_inspection: item.decision_intake_inspection,
            decision_intake_compatibility: item.decision_intake_compatibility,
            decision_intake_proceed_denial: item.decision_intake_proceed_denial,
            decision_intake_package_seal: overlay.decision_intake_package_seal.clone(),
            decision_intake_adapter_preparation: overlay.decision_intake_adapter_preparation.clone(),
            decision_handoff_request: overlay.decision_handoff_request.clone(),
            decision_engine_acceptance: Some(acceptance),
            explanation: explanation.into(),
            authority_effect: RecommendationReviewActionResult::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    /// Continuity on regenerate (Sprint 197–201):
    /// - open overlays whose source vanished → Expired + outcome
    /// - open overlays whose content fingerprint changed → Superseded + new Available
    /// - terminal overlays with fingerprint change → new Available generation (audit only;
    ///   Accepted/Rejected cannot transition; history remains in prior outcome/audit)
    /// - active surfaces prefer non-terminal candidates via summary_projection
    fn project_lifecycle_overlays(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        mut state: WorkspaceRecommendationEngineState,
    ) -> Result<WorkspaceRecommendationEngineState> {
        let now = recommendation_engine_now_rfc3339();
        let actor_id = actor.actor.id.to_string();
        let overlays = Self::load_overlays(db, &state.workspace_id)?;
        let mut by_id: HashMap<String, RecommendationLifecycleOverlay> = overlays
            .into_iter()
            .map(|o| (o.native_id.clone(), o))
            .collect();
        let live_ids: HashSet<String> = state.candidates.iter().map(|c| c.id.clone()).collect();

        // Expire open overlays whose regenerable source no longer exists.
        let orphan_ids: Vec<String> = by_id
            .keys()
            .filter(|id| !live_ids.contains(*id))
            .cloned()
            .collect();
        for native_id in orphan_ids {
            let Some(overlay) = by_id.get(&native_id).cloned() else {
                continue;
            };
            if !overlay.lifecycle_state.is_open() {
                continue;
            }
            let expired = Self::resolve_open_overlay(
                db,
                actor,
                &overlay,
                None,
                RecommendationLifecycleState::Expired,
                "workspace.recommendation_engine.expired",
                &now,
                &actor_id,
            )?;
            by_id.insert(native_id, expired);
        }

        for item in &state.candidates {
            let fingerprint = item.continuity_fingerprint();
            match by_id.get(&item.id).cloned() {
                None => {
                    let overlay = Self::new_available_overlay(
                        &state.workspace_id,
                        item,
                        &fingerprint,
                        &now,
                        &actor_id,
                    )?;
                    Self::upsert_overlay(db, &overlay)?;
                    by_id.insert(item.id.clone(), overlay);
                }
                Some(existing) => {
                    let fingerprint_matches = existing
                        .content_fingerprint
                        .as_deref()
                        .map(|fp| fp == fingerprint.as_str())
                        .unwrap_or(false);
                    if fingerprint_matches {
                        continue;
                    }
                    if existing.content_fingerprint.is_none() {
                        // Backfill fingerprint for pre-197 overlays without
                        // superseding or reopening terminal history.
                        let mut updated = existing.clone();
                        updated.content_fingerprint = Some(fingerprint);
                        updated.updated_at = now.clone();
                        Self::upsert_overlay(db, &updated)?;
                        by_id.insert(item.id.clone(), updated);
                        continue;
                    }
                    if existing.lifecycle_state.is_open() {
                        let superseded = Self::build_resolved_open_overlay(
                            &existing,
                            Some(item),
                            RecommendationLifecycleState::Superseded,
                            &now,
                            &actor_id,
                        )?;
                        let fresh = Self::new_available_overlay(
                            &state.workspace_id,
                            item,
                            &fingerprint,
                            &now,
                            &actor_id,
                        )?
                        .with_prior_outcomes(superseded.carried_outcomes());
                        Self::upsert_overlay_pair(db, &superseded, &fresh)?;
                        Self::audit_lifecycle(
                            db,
                            actor,
                            "workspace.recommendation_engine.superseded",
                            item,
                            &superseded,
                        )?;
                        by_id.insert(item.id.clone(), fresh);
                        continue;
                    }
                    // Terminal + material content change (or source returned after
                    // Expired/Superseded): open a new Available generation. Prior
                    // outcomes are retained on the overlay for history reconstruction.
                    Self::audit_lifecycle(
                        db,
                        actor,
                        "workspace.recommendation_engine.generation_reopened",
                        item,
                        &existing,
                    )?;
                    let fresh = Self::new_available_overlay(
                        &state.workspace_id,
                        item,
                        &fingerprint,
                        &now,
                        &actor_id,
                    )?
                    .with_prior_outcomes(existing.carried_outcomes());
                    // Keep terminal `existing` row unchanged; only insert fresh.
                    // Atomic with a no-op-safe re-upsert of terminal identity.
                    Self::upsert_overlay_pair(db, &existing, &fresh)?;
                    by_id.insert(item.id.clone(), fresh);
                }
            }
        }

        for item in &mut state.candidates {
            if let Some(overlay) = by_id.get(&item.id) {
                overlay.apply_to_item(item);
                if let Some(outcome) = &overlay.outcome {
                    item.outcome = Some(project_outcome_view(outcome));
                }
            }
        }
        Self::attach_explanation_views(&mut state)?;
        state.history = Self::build_history_from_overlays(by_id.values());
        state.history_count = state.history.len();
        // Context/readiness after history so outcome_history_refs are meaningful.
        let confirmation_by_id: HashMap<String, RecommendationDecisionConfirmation> = by_id
            .iter()
            .filter_map(|(id, overlay)| {
                overlay
                    .decision_confirmation
                    .clone()
                    .map(|c| (id.clone(), c))
            })
            .collect();
        let seal_by_id: HashMap<String, RecommendationDecisionIntakePackageSeal> = by_id
            .iter()
            .filter_map(|(id, overlay)| {
                overlay
                    .decision_intake_package_seal
                    .clone()
                    .map(|s| (id.clone(), s))
            })
            .collect();
        let prep_by_id: HashMap<String, RecommendationDecisionIntakeAdapterPreparation> = by_id
            .iter()
            .filter_map(|(id, overlay)| {
                overlay
                    .decision_intake_adapter_preparation
                    .clone()
                    .map(|p| (id.clone(), p))
            })
            .collect();
        let handoff_by_id: HashMap<String, RecommendationDecisionHandoffRequest> = by_id
            .iter()
            .filter_map(|(id, overlay)| {
                overlay
                    .decision_handoff_request
                    .clone()
                    .map(|r| (id.clone(), r))
            })
            .collect();
        let acceptance_by_id: HashMap<String, RecommendationDecisionEngineAcceptance> = by_id
            .iter()
            .filter_map(|(id, overlay)| {
                overlay
                    .decision_engine_acceptance
                    .clone()
                    .map(|a| (id.clone(), a))
            })
            .collect();
        Self::attach_decision_readiness(
            &mut state,
            &confirmation_by_id,
            &seal_by_id,
            &prep_by_id,
            &handoff_by_id,
            &acceptance_by_id,
        )?;
        // Consumer projections seal via `seal_actionable_projection` — mutation
        // paths keep terminals addressable until commands complete.
        Ok(state)
    }

    fn seal_actionable_projection(
        mut state: WorkspaceRecommendationEngineState,
    ) -> WorkspaceRecommendationEngineState {
        state.retain_actionable_candidates();
        state
    }

    /// Seal for callers that hold an unsealed intermediate (rare).
    /// Prefer [`Self::generate_with_inputs`], which seals by contract.
    pub(crate) fn seal_consumer_projection(
        state: WorkspaceRecommendationEngineState,
    ) -> WorkspaceRecommendationEngineState {
        Self::seal_actionable_projection(state)
    }

    /// Consumer-facing invariant: actionable channel contains no terminal items.
    pub(crate) fn assert_consumer_sealed(
        state: &WorkspaceRecommendationEngineState,
    ) -> bool {
        state.is_consumer_sealed()
    }

    fn build_history_from_overlays<'a>(
        overlays: impl Iterator<Item = &'a RecommendationLifecycleOverlay>,
    ) -> Vec<RecommendationHistoryEntry> {
        let mut entries = Vec::new();
        for overlay in overlays {
            for outcome in overlay.carried_outcomes() {
                let resolution = outcome
                    .lifecycle_resolution
                    .map(|r| r.as_str().to_string())
                    .unwrap_or_else(|| overlay.lifecycle_state.as_str().into());
                entries.push(RecommendationHistoryEntry {
                    native_id: overlay.native_id.clone(),
                    lifecycle_state: resolution,
                    outcome: project_outcome_view(&outcome),
                    resolved_at: Some(outcome.recorded_at.clone()),
                    terminal: true,
                    actionable: false,
                    authority_effect: RecommendationHistoryEntry::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }
        entries.sort_by(|a, b| {
            b.resolved_at
                .cmp(&a.resolved_at)
                .then_with(|| a.native_id.cmp(&b.native_id))
        });
        entries
    }

    fn record_outcome_with_experience(
        record: &RecommendationGovernanceRecord,
        item: &RecommendationItem,
        recorded_at: &str,
    ) -> Result<RecommendationOutcome> {
        let mut outcome = record
            .record_outcome(recorded_at)
            .map_err(map_lifecycle_err)?;
        let keys = experience_keys_for_item(item);
        if !keys.is_empty() {
            outcome = outcome.with_experience_trace_match_keys(keys);
        }
        assert_eq!(outcome.authority_effect, RecommendationOutcome::AUTHORITY_EFFECT_NONE);
        assert!(!outcome.is_system_failure());
        Ok(outcome)
    }

    /// Project structured explanation views — Experience traces as provenance only.
    fn attach_explanation_views(state: &mut WorkspaceRecommendationEngineState) -> Result<()> {
        for item in &mut state.candidates {
            let mut view = RecommendationExplanationView::from_item(item);
            if !item.attention_reasons.is_empty() {
                let keys: Vec<String> = item
                    .attention_reasons
                    .iter()
                    .map(|reason| {
                        resolve_attention_reason_traced(reason, Some("recommendation_explanation"))
                            .resolver_path
                            .match_key
                    })
                    .collect();
                view = view.with_experience_trace_match_keys(keys);
            }
            crate::services::validate_explanation_view_non_authoritative(&view)?;
            item.explanation = Some(view);
        }
        Ok(())
    }

    /// Project decision context + readiness + confirmation — observational only.
    fn attach_decision_readiness(
        state: &mut WorkspaceRecommendationEngineState,
        confirmation_by_id: &HashMap<String, RecommendationDecisionConfirmation>,
        seal_by_id: &HashMap<String, RecommendationDecisionIntakePackageSeal>,
        prep_by_id: &HashMap<String, RecommendationDecisionIntakeAdapterPreparation>,
        handoff_by_id: &HashMap<String, RecommendationDecisionHandoffRequest>,
        acceptance_by_id: &HashMap<String, RecommendationDecisionEngineAcceptance>,
    ) -> Result<()> {
        let mut history_refs_by_id: HashMap<String, Vec<String>> = HashMap::new();
        for entry in &state.history {
            history_refs_by_id
                .entry(entry.native_id.clone())
                .or_default()
                .push(entry.outcome.outcome_id.clone());
        }
        let workspace_id = state.workspace_id.clone();
        for item in &mut state.candidates {
            let refs = history_refs_by_id
                .get(&item.id)
                .cloned()
                .unwrap_or_default();
            let context =
                RecommendationDecisionContext::assemble(&workspace_id, item, &refs);
            crate::services::validate_decision_context_boundary(&context)?;

            let readiness = RecommendationDecisionReadiness::assess_from_context(&context);
            crate::services::validate_decision_readiness_boundary(&readiness)?;

            let boundary =
                RecommendationDecisionBoundary::from_context_and_readiness(&context, &readiness);
            crate::services::validate_decision_boundary_constraints(&boundary)?;

            let confirmation = confirmation_by_id
                .get(&item.id)
                .cloned()
                .unwrap_or_else(|| {
                    RecommendationDecisionConfirmation::derive_from_boundary(&boundary)
                });
            crate::services::validate_confirmation_post_action(&confirmation)?;

            let intake = RecommendationDecisionIntakeRequest::try_assemble(
                &context,
                &readiness,
                &confirmation,
            );
            let intake_inspection = match intake.as_ref() {
                Some(request) => {
                    let inspection = RecommendationDecisionIntakeInspection::verify(
                        request,
                        &context,
                        &confirmation,
                        &readiness,
                    );
                    crate::services::validate_intake_inspection_boundary(&inspection)?;
                    Some(inspection)
                }
                None => None,
            };
            let intake_compatibility =
                match (intake.as_ref(), intake_inspection.as_ref()) {
                    (Some(request), Some(inspection)) => {
                        let compatibility =
                            RecommendationDecisionIntakeCompatibility::derive_from_inspection(
                                request, inspection,
                            );
                        crate::services::validate_intake_compatibility_boundary(&compatibility)?;
                        Some(compatibility)
                    }
                    _ => None,
                };
            let intake_proceed_denial = match intake_compatibility.as_ref() {
                Some(compat) => {
                    let denial =
                        RecommendationDecisionIntakeProceedDenial::derive_from_compatibility(compat);
                    crate::services::validate_intake_proceed_denial_boundary(&denial)?;
                    Some(denial)
                }
                None => None,
            };
            let intake_package_seal = match (
                intake.as_ref(),
                intake_compatibility.as_ref(),
                intake_proceed_denial.as_ref(),
                seal_by_id.get(&item.id),
            ) {
                (Some(request), _, _, Some(stored)) => {
                    let seal = stored.clone().reverify_against(request);
                    crate::services::validate_intake_package_seal_boundary(&seal, Some(request))?;
                    Some(seal)
                }
                (Some(request), Some(compat), Some(denial), None) => {
                    // Confirmed intake without persisted seal (legacy) — derive surface seal only.
                    let seal = RecommendationDecisionIntakePackageSeal::derive_from_proceed_denial(
                        request,
                        compat,
                        denial,
                        recommendation_engine_now_rfc3339(),
                    );
                    crate::services::validate_intake_package_seal_boundary(&seal, Some(request))?;
                    Some(seal)
                }
                _ => None,
            };
            let intake_adapter_preparation = match (
                intake_package_seal.as_ref(),
                prep_by_id.get(&item.id),
                intake.as_ref(),
            ) {
                (Some(seal), Some(stored), _) => {
                    let prep = stored.clone().rebind_to_seal(seal);
                    crate::services::validate_adapter_preparation_boundary(&prep, None)?;
                    Some(prep)
                }
                (Some(seal), None, Some(request)) => {
                    let prep = RecommendationDecisionIntakeAdapterPreparation::try_prepare(
                        request,
                        &confirmation,
                        seal,
                        recommendation_engine_now_rfc3339(),
                    );
                    if let Some(ref preparation) = prep {
                        crate::services::validate_adapter_preparation_boundary(
                            preparation,
                            None,
                        )?;
                    }
                    prep
                }
                _ => None,
            };
            let decision_handoff_request = match (
                intake_adapter_preparation.as_ref(),
                handoff_by_id.get(&item.id),
                intake_package_seal.as_ref(),
                intake_compatibility.as_ref(),
            ) {
                (Some(prep), Some(stored), _, _) => {
                    let request = stored.clone().rebind_to_preparation(prep);
                    crate::services::validate_handoff_request_boundary(&request, None)?;
                    Some(request)
                }
                (Some(prep), None, Some(seal), Some(compat)) => {
                    let request = RecommendationDecisionHandoffRequest::try_request(
                        prep,
                        &confirmation,
                        seal,
                        compat,
                        recommendation_engine_now_rfc3339(),
                    );
                    if let Some(ref handoff_request) = request {
                        crate::services::validate_handoff_request_boundary(handoff_request, None)?;
                    }
                    request
                }
                _ => None,
            };
            let decision_engine_acceptance = match (
                decision_handoff_request.as_ref(),
                acceptance_by_id.get(&item.id),
            ) {
                (Some(request), Some(stored)) => {
                    let acceptance = stored.clone().rebind_to_handoff_request(request);
                    crate::services::validate_engine_acceptance_boundary(
                        &acceptance,
                        None,
                        None,
                    )?;
                    Some(acceptance)
                }
                (Some(request), None) => {
                    let acceptance =
                        RecommendationDecisionEngineAcceptance::derive_from_handoff_request(
                            request,
                        );
                    if let Some(ref boundary) = acceptance {
                        crate::services::validate_engine_acceptance_boundary(
                            boundary,
                            None,
                            None,
                        )?;
                    }
                    acceptance
                }
                _ => None,
            };
            if let Some(ref request) = intake {
                crate::services::validate_intake_request_non_authoritative(request)?;
            }

            item.decision_context = Some(context);
            item.decision_readiness = Some(readiness);
            item.decision_boundary = Some(boundary);
            item.decision_confirmation = Some(confirmation);
            item.decision_intake = intake;
            item.decision_intake_inspection = intake_inspection;
            item.decision_intake_compatibility = intake_compatibility;
            item.decision_intake_proceed_denial = intake_proceed_denial;
            item.decision_intake_package_seal = intake_package_seal;
            item.decision_intake_adapter_preparation = intake_adapter_preparation;
            item.decision_handoff_request = decision_handoff_request;
            item.decision_engine_acceptance = decision_engine_acceptance;
        }
        Ok(())
    }

    fn new_available_overlay(
        workspace_id: &str,
        item: &RecommendationItem,
        fingerprint: &str,
        now: &str,
        actor_id: &str,
    ) -> Result<RecommendationLifecycleOverlay> {
        let mut record = RecommendationGovernanceRecord::from_recommendation_item(item, now);
        record
            .transition(
                RecommendationLifecycleState::Available,
                now.to_string(),
                Some(actor_id.into()),
            )
            .map_err(map_lifecycle_err)?;
        Ok(
            RecommendationLifecycleOverlay::from_governance_record(
                workspace_id,
                &record,
                None,
                now,
            )
            .with_content_fingerprint(fingerprint),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_open_overlay(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        overlay: &RecommendationLifecycleOverlay,
        live_item: Option<&RecommendationItem>,
        to: RecommendationLifecycleState,
        audit_event: &str,
        now: &str,
        actor_id: &str,
    ) -> Result<RecommendationLifecycleOverlay> {
        let next = Self::build_resolved_open_overlay(overlay, live_item, to, now, actor_id)?;
        Self::upsert_overlay(db, &next)?;
        if let Some(item) = live_item {
            Self::audit_lifecycle(db, actor, audit_event, item, &next)?;
        } else {
            AuditService::record_ai_planning_event(
                db,
                actor,
                &IntentContext::user_request(),
                audit_event,
                true,
                json!({
                    "workspace_id": next.workspace_id,
                    "recommendation_id": next.native_id,
                    "lifecycle_state": next.lifecycle_state.as_str(),
                    "resolution_type": next.resolution_type.map(|r| r.as_str()),
                    "authority_effect": "none",
                    "continuity": "source_absent",
                })
                .to_string(),
            )?;
        }
        Ok(next)
    }

    fn build_resolved_open_overlay(
        overlay: &RecommendationLifecycleOverlay,
        live_item: Option<&RecommendationItem>,
        to: RecommendationLifecycleState,
        now: &str,
        actor_id: &str,
    ) -> Result<RecommendationLifecycleOverlay> {
        let mut record = match live_item {
            Some(item) => {
                let mut record =
                    RecommendationGovernanceRecord::from_recommendation_item(item, &overlay.created_at);
                record.lifecycle.state = overlay.lifecycle_state;
                record.lifecycle.presented_at = overlay.presented_at.clone();
                record.lifecycle.resolved_at = overlay.resolved_at.clone();
                record.lifecycle.resolution_type = overlay.resolution_type;
                record.lifecycle.transition_actor_id = overlay.actor_id.clone();
                record
            }
            None => overlay.to_governance_record_for_continuity(),
        };
        if record.lifecycle.state == RecommendationLifecycleState::Created {
            record
                .transition(
                    RecommendationLifecycleState::Available,
                    now.to_string(),
                    Some(actor_id.into()),
                )
                .map_err(map_lifecycle_err)?;
        }
        record
            .transition(to, now.to_string(), Some(actor_id.into()))
            .map_err(map_lifecycle_err)?;
        let outcome = match live_item {
            Some(item) => Self::record_outcome_with_experience(&record, item, now)?,
            None => record.record_outcome(now).map_err(map_lifecycle_err)?,
        };
        Ok(
            RecommendationLifecycleOverlay::from_governance_record(
                overlay.workspace_id.clone(),
                &record,
                Some(outcome),
                now,
            )
            .with_prior_outcomes(overlay.prior_outcomes.clone())
            .with_content_fingerprint(
                overlay
                    .content_fingerprint
                    .clone()
                    .unwrap_or_else(|| "continuity".into()),
            ),
        )
    }

    fn load_overlays(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<RecommendationLifecycleOverlay>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        Ok(RecommendationLifecycleRepository::new(&guard).list_overlays(workspace_id)?)
    }

    fn load_overlay(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        native_id: &str,
    ) -> Result<Option<RecommendationLifecycleOverlay>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        Ok(RecommendationLifecycleRepository::new(&guard).get_overlay(workspace_id, native_id)?)
    }

    fn upsert_overlay(
        db: &Arc<Mutex<Database>>,
        overlay: &RecommendationLifecycleOverlay,
    ) -> Result<()> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        RecommendationLifecycleRepository::new(&guard)
            .upsert_overlay(overlay)
            .map_err(KernelError::from_recommendation_persistence)?;
        Ok(())
    }

    /// Persist two continuity overlays atomically (e.g. Superseded + fresh Available).
    /// Recovery must not leave a terminal supersede without its replacement row.
    fn upsert_overlay_pair(
        db: &Arc<Mutex<Database>>,
        first: &RecommendationLifecycleOverlay,
        second: &RecommendationLifecycleOverlay,
    ) -> Result<()> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        guard
            .run_in_transaction(|database| {
                let repo = RecommendationLifecycleRepository::new(database);
                repo.upsert_overlay(first)?;
                repo.upsert_overlay(second)?;
                Ok(())
            })
            .map_err(KernelError::from_recommendation_persistence)?;
        Ok(())
    }

    fn audit_lifecycle(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        item: &RecommendationItem,
        overlay: &RecommendationLifecycleOverlay,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            json!({
                "workspace_id": overlay.workspace_id,
                "recommendation_id": item.id,
                "lifecycle_state": overlay.lifecycle_state.as_str(),
                "resolution_type": overlay.resolution_type.map(|r| r.as_str()),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceRecommendationEngineState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.recommendation_engine.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "candidate_count": state.candidate_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn map_lifecycle_err(error: ActionProposalError) -> KernelError {
    match error {
        ActionProposalError::InvalidLifecycleTransition { from, to } => {
            KernelError::from(WorkspaceRecommendationEngineError::InvalidLifecycleTransition {
                from,
                to,
            })
        }
        ActionProposalError::OutcomeRequiresResolution => {
            KernelError::from(WorkspaceRecommendationEngineError::OutcomeRequiresResolution)
        }
        other => KernelError::WorkspaceRecommendationEngineValidation {
            message: other.to_string(),
        },
    }
}

fn project_outcome_view(outcome: &RecommendationOutcome) -> RecommendationOutcomeView {
    RecommendationOutcomeView {
        outcome_id: outcome.id.clone(),
        recommendation_id: outcome.identity.native_id.clone(),
        user_decision: outcome.user_decision.as_str().into(),
        result_kind: outcome.result_kind.as_str().into(),
        lifecycle_resolution: outcome
            .lifecycle_resolution
            .map(|r| r.as_str().into()),
        recorded_at: outcome.recorded_at.clone(),
        explanation_keys: outcome.provenance.explanation_keys.clone(),
        evidence_refs: outcome
            .provenance
            .source_evidence
            .iter()
            .map(|e| e.source_ref.clone())
            .collect(),
        experience_trace_match_keys: outcome.experience_trace_match_keys.clone(),
        is_system_failure: outcome.is_system_failure(),
        authority_effect: RecommendationOutcomeView::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn experience_keys_for_item(item: &RecommendationItem) -> Vec<String> {
    if let Some(view) = &item.explanation {
        if !view.experience_trace_match_keys.is_empty() {
            return view.experience_trace_match_keys.clone();
        }
    }
    item.attention_reasons
        .iter()
        .map(|reason| {
            resolve_attention_reason_traced(reason, Some("recommendation_outcome"))
                .resolver_path
                .match_key
        })
        .collect()
}

fn kind_rank(kind: RecommendationKind) -> u8 {
    match kind {
        RecommendationKind::ResolveBlocker => 0,
        RecommendationKind::ReviewDecision => 1,
        RecommendationKind::RestoreContext => 2,
        RecommendationKind::ContinueWork => 3,
        RecommendationKind::CompleteTask => 4,
        RecommendationKind::ReorganizeWorkspace => 5,
        RecommendationKind::ExploreOpportunity => 6,
    }
}
