//! Workspace Learning & Adaptation — Programme II Batch 6.
//!
//! Downstream meta-evidence layer. Observes planning/reasoning/graph/
//! orchestration/recommendation/decision/task/execution outcomes and produces
//! patterns, confidence evolution, and adaptation suggestions.
//! Never mutates foreign domains. Never auto-applies adaptations.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{
    DecisionEngineRepository, ExecutionLifecycleRepository, LearningAdaptationRepository,
    RecommendationLifecycleRepository, TaskGraphRepository, Database,
};
use workspace_domain::{
    ActorContext, AdaptationCandidate, ConfidenceUpdate, DecisionOutcome, ExecutionState,
    LearningEvidenceLink, LearningMeta, LearningObservation, LearningPattern, LearningSignal,
    LearningSnapshot, LearningStatus, LearningView, RecommendationLifecycleState,
    WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceCognitiveGraphService, WorkspaceCognitiveModelService,
    WorkspaceCognitiveOrchestrationService, WorkspacePlanningService,
    WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceLearningAdaptationService;

impl WorkspaceLearningAdaptationService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<LearningSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = LearningAdaptationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.learning_adaptation.snapshot_generated",
            &view.meta.learning_id,
            json!({
                "workspace_id": workspace_id,
                "observation_count": view.observations.len(),
                "pattern_count": view.patterns.len(),
                "adaptation_count": view.adaptation_candidates.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<LearningSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = LearningAdaptationRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == LearningStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(LearningSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "WorkspaceLearningAdaptationService cannot execute, dispatch, or launch"
                .into(),
        })
    }

    pub(crate) fn attempt_auto_change_plan() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "Learning cannot auto-change plans".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "Learning cannot modify lifecycle states".into(),
        })
    }

    pub(crate) fn attempt_approve_decision() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "Learning cannot approve decisions".into(),
        })
    }

    pub(crate) fn attempt_auto_apply_adaptation() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "Adaptation candidates are suggestion-only — never auto-applied".into(),
        })
    }

    pub(crate) fn attempt_rewrite_history() -> Result<()> {
        Err(KernelError::WorkspaceLearningAdaptationValidation {
            message: "Learning cannot rewrite history".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = LearningAdaptationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced learning adaptation transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceLearningAdaptationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<LearningView> {
        // Read-only cognitive composition — never call foreign generate paths.
        let cognitive = WorkspaceCognitiveModelService::generate_model(db, workspace_id)?;
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id)?;
        let reasoning = WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id)?;
        let graph = WorkspaceCognitiveGraphService::load_snapshot(db, workspace_id)?;
        let orchestration =
            WorkspaceCognitiveOrchestrationService::load_snapshot(db, workspace_id)?;

        let (tasks, rec_overlays, decision_overlays, executions) = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let tasks = TaskGraphRepository::new(&guard).list_tasks(workspace_id)?;
            let rec_overlays =
                RecommendationLifecycleRepository::new(&guard).list_overlays(workspace_id)?;
            let decision_overlays =
                DecisionEngineRepository::new(&guard).list_overlays(workspace_id)?;
            // Repository read only — do not call execution lifecycle service APIs.
            let executions = ExecutionLifecycleRepository::new(&guard).list_recent(50)?;
            (tasks, rec_overlays, decision_overlays, executions)
        };

        let mut observations = Vec::new();
        let mut patterns = Vec::new();
        let mut success_signals = Vec::new();
        let mut failure_signals = Vec::new();
        let mut confidence_updates = Vec::new();
        let mut adaptation_candidates = Vec::new();
        let mut evidence_links = Vec::new();

        if cognitive.nodes.is_empty() {
            observations.push(LearningObservation {
                kind: "missing_outcome".into(),
                statement: "Cognitive model has no nodes — outcome evidence unknown".into(),
                source_domain: "cognitive_model".into(),
                evidence_refs: vec![],
            });
        } else {
            for node in cognitive.nodes.iter().take(20) {
                evidence_links.push(LearningEvidenceLink {
                    external_ref: node.id.clone(),
                    kind: "cognitive_node".into(),
                });
            }
        }

        if let Some(proposal) = &planning.current {
            let plan_id = proposal.plan.id.clone();
            evidence_links.push(LearningEvidenceLink {
                external_ref: plan_id.clone(),
                kind: "planning".into(),
            });
            if !proposal.gaps.is_empty() {
                observations.push(LearningObservation {
                    kind: "planning_gaps_observed".into(),
                    statement: format!(
                        "Planning snapshot has {} gap(s) — observational only",
                        proposal.gaps.len()
                    ),
                    source_domain: "planning".into(),
                    evidence_refs: vec![plan_id.clone()],
                });
            }
            if !proposal.risks.is_empty() {
                observations.push(LearningObservation {
                    kind: "planning_risks_observed".into(),
                    statement: format!(
                        "Planning snapshot has {} risk(s)",
                        proposal.risks.len()
                    ),
                    source_domain: "planning".into(),
                    evidence_refs: vec![plan_id],
                });
            }
        } else {
            observations.push(LearningObservation {
                kind: "missing_outcome".into(),
                statement: "No current planning snapshot — planning outcome unknown".into(),
                source_domain: "planning".into(),
                evidence_refs: vec![],
            });
        }

        if let Some(record) = &reasoning.current {
            evidence_links.push(LearningEvidenceLink {
                external_ref: record.id.clone(),
                kind: "reasoning".into(),
            });
            observations.push(LearningObservation {
                kind: "reasoning_outcome".into(),
                statement: format!(
                    "Reasoning confidence={}, uncertainty={}",
                    record.confidence, record.uncertainty
                ),
                source_domain: "reasoning".into(),
                evidence_refs: vec![record.id.clone()],
            });
            if !record.confidence_evolution.is_empty() {
                let before = *record.confidence_evolution.first().unwrap_or(&record.confidence);
                let after = record.confidence;
                confidence_updates.push(
                    ConfidenceUpdate::new(
                        record.id.clone(),
                        "reasoning",
                        before,
                        "reasoning confidence evolution trail observed",
                        after,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            if record.uncertainty > 60 {
                failure_signals.push(LearningSignal::failure(
                    "reasoning_uncertainty_elevated",
                    format!("Reasoning uncertainty elevated ({})", record.uncertainty),
                    vec![record.id.clone()],
                ));
            } else if record.confidence >= 70 {
                success_signals.push(LearningSignal::success(
                    "reasoning_confidence_stable",
                    format!("Reasoning confidence stable ({})", record.confidence),
                    vec![record.id.clone()],
                ));
            }
        } else {
            observations.push(LearningObservation {
                kind: "missing_outcome".into(),
                statement: "No current reasoning record — reasoning outcome unknown".into(),
                source_domain: "reasoning".into(),
                evidence_refs: vec![],
            });
        }

        if let Some(gview) = &graph.current {
            evidence_links.push(LearningEvidenceLink {
                external_ref: gview.meta.id.clone(),
                kind: "cognitive_graph".into(),
            });
            if gview.meta.broken_node_count > 0 || gview.meta.broken_edge_count > 0 {
                observations.push(LearningObservation {
                    kind: "graph_evolution_broken_refs".into(),
                    statement: format!(
                        "Graph evolution shows {} broken nodes / {} broken edges",
                        gview.meta.broken_node_count, gview.meta.broken_edge_count
                    ),
                    source_domain: "cognitive_graph".into(),
                    evidence_refs: vec![gview.meta.id.clone()],
                });
                failure_signals.push(LearningSignal::failure(
                    "graph_broken_references",
                    "Broken references observed in cognitive graph",
                    vec![gview.meta.id.clone()],
                ));
            } else {
                success_signals.push(LearningSignal::success(
                    "graph_integrity",
                    "Cognitive graph has no broken references",
                    vec![gview.meta.id.clone()],
                ));
            }
        }

        if let Some(orch) = &orchestration.current {
            evidence_links.push(LearningEvidenceLink {
                external_ref: orch.meta.orchestration_id.clone(),
                kind: "orchestration".into(),
            });
            for stale in &orch.stale_items {
                observations.push(LearningObservation {
                    kind: "orchestration_stale".into(),
                    statement: stale.statement.clone(),
                    source_domain: "orchestration".into(),
                    evidence_refs: stale.evidence_refs.clone(),
                });
            }
        }

        let completed_tasks: Vec<_> = tasks
            .iter()
            .filter(|t| t.status == WorkspaceTaskStatus::Completed)
            .collect();
        let cancelled_tasks: Vec<_> = tasks
            .iter()
            .filter(|t| t.status == WorkspaceTaskStatus::Cancelled)
            .collect();
        for t in tasks.iter().take(30) {
            evidence_links.push(LearningEvidenceLink {
                external_ref: t.id.to_string(),
                kind: "task".into(),
            });
        }
        if !completed_tasks.is_empty() {
            success_signals.push(LearningSignal::success(
                "task_completions",
                format!("{} completed task(s) observed", completed_tasks.len()),
                completed_tasks
                    .iter()
                    .take(10)
                    .map(|t| t.id.to_string())
                    .collect(),
            ));
        }
        if cancelled_tasks.len() >= 2 {
            patterns.push(LearningPattern::observed(
                "repeated_task_cancellations",
                "Similar task cancellations observed",
                cancelled_tasks.len() as u32,
                cancelled_tasks
                    .iter()
                    .take(10)
                    .map(|t| t.id.to_string())
                    .collect(),
            ));
            failure_signals.push(LearningSignal::failure(
                "task_cancellations",
                format!("{} cancelled task(s) observed", cancelled_tasks.len()),
                cancelled_tasks
                    .iter()
                    .take(10)
                    .map(|t| t.id.to_string())
                    .collect(),
            ));
        }

        let accepted_recs: Vec<_> = rec_overlays
            .iter()
            .filter(|o| o.lifecycle_state == RecommendationLifecycleState::Accepted)
            .collect();
        let rejected_recs: Vec<_> = rec_overlays
            .iter()
            .filter(|o| o.lifecycle_state == RecommendationLifecycleState::Rejected)
            .collect();
        for o in &rec_overlays {
            evidence_links.push(LearningEvidenceLink {
                external_ref: o.native_id.clone(),
                kind: "recommendation".into(),
            });
        }
        if !accepted_recs.is_empty() {
            success_signals.push(LearningSignal::success(
                "recommendation_accepted",
                format!("{} accepted recommendation outcome(s)", accepted_recs.len()),
                accepted_recs.iter().map(|o| o.native_id.clone()).collect(),
            ));
        }
        if rejected_recs.len() >= 2 {
            patterns.push(LearningPattern::observed(
                "repeated_recommendation_rejections",
                "Similar recommendation rejections observed",
                rejected_recs.len() as u32,
                rejected_recs.iter().map(|o| o.native_id.clone()).collect(),
            ));
        }

        let selected_decisions: Vec<_> = decision_overlays
            .iter()
            .filter(|o| o.outcome == DecisionOutcome::Selected)
            .collect();
        let dismissed_decisions: Vec<_> = decision_overlays
            .iter()
            .filter(|o| {
                matches!(
                    o.outcome,
                    DecisionOutcome::Dismissed | DecisionOutcome::Expired
                )
            })
            .collect();
        for o in &decision_overlays {
            evidence_links.push(LearningEvidenceLink {
                external_ref: o.candidate_key.clone(),
                kind: "decision".into(),
            });
        }
        if !selected_decisions.is_empty() {
            success_signals.push(LearningSignal::success(
                "decision_selected",
                format!("{} selected decision outcome(s)", selected_decisions.len()),
                selected_decisions
                    .iter()
                    .map(|o| o.candidate_key.clone())
                    .collect(),
            ));
        }
        if dismissed_decisions.len() >= 2 {
            patterns.push(LearningPattern::observed(
                "repeated_decision_dismissals",
                "Similar decision dismissals/expiries observed",
                dismissed_decisions.len() as u32,
                dismissed_decisions
                    .iter()
                    .map(|o| o.candidate_key.clone())
                    .collect(),
            ));
        }

        let failed_execs: Vec<_> = executions
            .iter()
            .filter(|e| e.state == ExecutionState::Failed)
            .collect();
        let completed_execs: Vec<_> = executions
            .iter()
            .filter(|e| e.state == ExecutionState::Completed)
            .collect();
        for e in &executions {
            evidence_links.push(LearningEvidenceLink {
                external_ref: e.execution_request_id.clone(),
                kind: "execution".into(),
            });
            if e.state == ExecutionState::Failed {
                observations.push(LearningObservation {
                    kind: "execution_failure".into(),
                    statement: e
                        .failure_reason
                        .clone()
                        .unwrap_or_else(|| "Execution failed — outcome observed".into()),
                    source_domain: "execution".into(),
                    evidence_refs: vec![e.execution_request_id.clone()],
                });
            }
            if e.state == ExecutionState::Completed {
                if let (Some(claimed), Some(completed_raw)) =
                    (e.claimed_at.parse::<chrono::DateTime<chrono::FixedOffset>>().ok(), e.completed_at.as_ref())
                {
                    if let Ok(completed) = completed_raw.parse::<chrono::DateTime<chrono::FixedOffset>>() {
                        let actual_secs = (completed - claimed).num_seconds().max(0);
                        observations.push(LearningObservation {
                            kind: "duration_estimation_error".into(),
                            statement: format!(
                                "Observed execution duration {actual_secs}s (planned duration unknown — not invented)"
                            ),
                            source_domain: "execution".into(),
                            evidence_refs: vec![e.execution_request_id.clone()],
                        });
                    }
                }
            }
        }
        if !completed_execs.is_empty() {
            success_signals.push(LearningSignal::success(
                "execution_completed",
                format!("{} completed execution(s) observed", completed_execs.len()),
                completed_execs
                    .iter()
                    .map(|e| e.execution_request_id.clone())
                    .collect(),
            ));
        }
        if failed_execs.len() >= 2 {
            patterns.push(LearningPattern::observed(
                "repeated_execution_failures",
                "Similar execution failures observed",
                failed_execs.len() as u32,
                failed_execs
                    .iter()
                    .map(|e| e.execution_request_id.clone())
                    .collect(),
            ));
            failure_signals.push(LearningSignal::failure(
                "execution_failures",
                format!("{} failed execution(s) observed", failed_execs.len()),
                failed_execs
                    .iter()
                    .map(|e| e.execution_request_id.clone())
                    .collect(),
            ));
        }

        // Adaptation candidates — suggestions only, never auto-applied.
        if completed_tasks.len() >= 3
            && planning
                .current
                .as_ref()
                .map(|p| !p.gaps.is_empty() || !p.risks.is_empty())
                .unwrap_or(false)
        {
            adaptation_candidates.push(
                AdaptationCandidate::suggest(
                    "Optimistic planning estimates",
                    "Planning estimates frequently optimistic relative to completed task volume.",
                    format!("{} completed tasks with planning gaps/risks present", completed_tasks.len()),
                    completed_tasks
                        .iter()
                        .take(7)
                        .map(|t| t.id.to_string())
                        .collect(),
                    82,
                )
                .map_err(KernelError::from)?,
            );
        }
        if failed_execs.len() >= 2 {
            adaptation_candidates.push(
                AdaptationCandidate::suggest(
                    "Execution failure pattern review",
                    "Consider reviewing failure reasons before future launches — suggestion only.",
                    format!("{} failed executions observed", failed_execs.len()),
                    failed_execs
                        .iter()
                        .map(|e| e.execution_request_id.clone())
                        .collect(),
                    70,
                )
                .map_err(KernelError::from)?,
            );
        }
        if rejected_recs.len() >= 2 {
            adaptation_candidates.push(
                AdaptationCandidate::suggest(
                    "Recommendation relevance drift",
                    "Repeated recommendation rejections may indicate relevance drift — suggestion only.",
                    format!("{} rejected recommendations", rejected_recs.len()),
                    rejected_recs.iter().map(|o| o.native_id.clone()).collect(),
                    65,
                )
                .map_err(KernelError::from)?,
            );
        }

        let uncertainty = Self::compute_uncertainty(
            planning.current.is_some(),
            reasoning.current.is_some(),
            graph.current.is_some(),
            &observations,
            &failure_signals,
        );

        let meta = LearningMeta::new(
            workspace_id,
            now,
            uncertainty,
            observations.len(),
            patterns.len(),
            adaptation_candidates.len(),
        )
        .map_err(KernelError::from)?;

        let view = LearningView {
            meta,
            observations,
            patterns,
            success_signals,
            failure_signals,
            confidence_updates,
            adaptation_candidates,
            evidence_links,
            uncertainty,
            authority_effect: LearningView::AUTHORITY_EFFECT_NONE.into(),
        };
        view.validate().map_err(KernelError::from)?;
        Ok(view)
    }

    fn compute_uncertainty(
        has_planning: bool,
        has_reasoning: bool,
        has_graph: bool,
        observations: &[LearningObservation],
        failures: &[LearningSignal],
    ) -> u8 {
        let mut score: u16 = 25;
        if !has_planning {
            score += 15;
        }
        if !has_reasoning {
            score += 15;
        }
        if !has_graph {
            score += 10;
        }
        let missing = observations
            .iter()
            .filter(|o| o.kind == "missing_outcome")
            .count();
        score += (missing as u16).saturating_mul(8);
        score += (failures.len() as u16).saturating_mul(4);
        score.min(100) as u8
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let _ = subject_id;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &workspace_domain::IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )
    }
}
