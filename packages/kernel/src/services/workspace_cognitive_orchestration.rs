//! Workspace Cognitive Orchestration — Programme II Batch 5.
//!
//! Coordination layer above Cognitive Model / Planning / Reasoning / Graph.
//! Produces refresh ordering and staleness observations only.
//! Never executes refreshes, never owns foreign lifecycles.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{CognitiveOrchestrationRepository, Database};
use workspace_domain::{
    order_dependencies, refresh_stages_from_order, ActorContext, OrchestrationArtefactKind,
    OrchestrationDependency, OrchestrationEvidenceLink, OrchestrationObservation,
    OrchestrationStatus, WorkspaceOrchestrationMeta, WorkspaceOrchestrationSnapshot,
    WorkspaceOrchestrationView,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceCognitiveGraphService, WorkspaceCognitiveModelService,
    WorkspacePlanningService, WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceCognitiveOrchestrationService;

impl WorkspaceCognitiveOrchestrationService {
    /// Generate and persist an orchestration snapshot from read-only artefact views.
    /// Describes refresh ordering only — never executes refreshes.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceOrchestrationSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = CognitiveOrchestrationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.cognitive_orchestration.snapshot_generated",
            &view.meta.orchestration_id,
            json!({
                "workspace_id": workspace_id,
                "current_generation": view.meta.current_generation,
                "stage_count": view.refresh_plan.len(),
                "stale_count": view.stale_items.len(),
                "cycle_count": view.cycles.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceOrchestrationSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = CognitiveOrchestrationRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == OrchestrationStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(WorkspaceOrchestrationSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message:
                "WorkspaceCognitiveOrchestrationService cannot execute, dispatch, claim, or launch"
                    .into(),
        })
    }

    pub(crate) fn attempt_mutate_intent() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message: "Orchestration cannot mutate Intent lifecycle".into(),
        })
    }

    pub(crate) fn attempt_mutate_task_graph() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message: "Orchestration cannot mutate Task Graph".into(),
        })
    }

    pub(crate) fn attempt_accept_recommendation() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message: "Orchestration cannot accept recommendations".into(),
        })
    }

    pub(crate) fn attempt_select_decision() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message: "Orchestration cannot select decisions".into(),
        })
    }

    pub(crate) fn attempt_execute_refresh_plan() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
            message: "Orchestration refresh plans are descriptive only — never executed here"
                .into(),
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
            let repo = CognitiveOrchestrationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced cognitive orchestration transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceCognitiveOrchestrationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<WorkspaceOrchestrationView> {
        // Read-only composition — never call generate on planning/reasoning/graph.
        let cognitive = WorkspaceCognitiveModelService::generate_model(db, workspace_id)?;
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id)?;
        let reasoning = WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id)?;
        let graph = WorkspaceCognitiveGraphService::load_snapshot(db, workspace_id)?;

        let previous_generation = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let repo = CognitiveOrchestrationRepository::new(&guard);
            repo.list_meta(workspace_id)?
                .into_iter()
                .map(|m| m.current_generation)
                .max()
                .unwrap_or(0)
        };
        let current_generation = previous_generation.saturating_add(1);

        let nodes = [
            OrchestrationArtefactKind::CognitiveModel,
            OrchestrationArtefactKind::Planning,
            OrchestrationArtefactKind::Reasoning,
            OrchestrationArtefactKind::CognitiveGraph,
        ];
        let dependencies = vec![
            OrchestrationDependency {
                from: OrchestrationArtefactKind::CognitiveModel,
                to: OrchestrationArtefactKind::Planning,
                reason: "planning consumes cognitive model semantics".into(),
            },
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Planning,
                to: OrchestrationArtefactKind::Reasoning,
                reason: "reasoning cites planning proposals".into(),
            },
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Reasoning,
                to: OrchestrationArtefactKind::CognitiveGraph,
                reason: "graph projects reasoning evidence topology".into(),
            },
        ];

        let order_result = order_dependencies(&nodes, &dependencies);
        let refresh_plan = if order_result.cycles.is_empty() {
            refresh_stages_from_order(&order_result.order)
        } else {
            // Cycle: do not invent a resolution — no fabricated refresh plan stages
            // for remaining cyclic nodes; only ordered prefix (if any) is described.
            refresh_stages_from_order(&order_result.order)
        };

        let mut stale_items = Vec::new();
        let mut blocked_items = Vec::new();
        let mut skipped_items = Vec::new();
        let mut evidence_links = Vec::new();
        let mut planning_links = Vec::new();
        let mut reasoning_links = Vec::new();
        let mut graph_links = Vec::new();
        let mut execution_links = Vec::new();

        let has_cognitive = !cognitive.nodes.is_empty();
        let has_planning = planning.current.is_some();
        let has_reasoning = reasoning.current.is_some();
        let has_graph = graph.current.is_some();

        if !has_cognitive {
            stale_items.push(OrchestrationObservation {
                kind: "missing_references".into(),
                artefact: Some(OrchestrationArtefactKind::CognitiveModel),
                statement: "Cognitive model has no durable nodes — refresh consideration noted"
                    .into(),
                evidence_refs: vec![],
            });
        } else {
            for node in &cognitive.nodes {
                evidence_links.push(OrchestrationEvidenceLink {
                    external_ref: node.id.clone(),
                    kind: "cognitive_node".into(),
                });
            }
        }

        if !has_planning {
            stale_items.push(OrchestrationObservation {
                kind: "outdated_planning".into(),
                artefact: Some(OrchestrationArtefactKind::Planning),
                statement: "No current planning snapshot — planning refresh considered".into(),
                evidence_refs: vec![],
            });
            if has_cognitive {
                blocked_items.push(OrchestrationObservation {
                    kind: "dependency_conflicts".into(),
                    artefact: Some(OrchestrationArtefactKind::Planning),
                    statement: "Planning missing while cognitive model present".into(),
                    evidence_refs: evidence_links
                        .iter()
                        .map(|l| l.external_ref.clone())
                        .take(5)
                        .collect(),
                });
            }
        } else if let Some(proposal) = &planning.current {
            let plan_id = proposal.plan.id.clone();
            planning_links.push(OrchestrationEvidenceLink {
                external_ref: plan_id.clone(),
                kind: "planning_snapshot".into(),
            });
            if !has_cognitive {
                stale_items.push(OrchestrationObservation {
                    kind: "outdated_planning".into(),
                    artefact: Some(OrchestrationArtefactKind::Planning),
                    statement: "Planning present without cognitive model nodes — may be outdated"
                        .into(),
                    evidence_refs: vec![plan_id],
                });
            }
            for eref in &proposal.evidence_refs {
                let rid = eref.external_ref.clone();
                if rid.starts_with("execution:") || rid.contains("execution_lifecycle") {
                    execution_links.push(OrchestrationEvidenceLink {
                        external_ref: rid,
                        kind: "execution_ref".into(),
                    });
                }
            }
        }

        if !has_reasoning {
            stale_items.push(OrchestrationObservation {
                kind: "superseded_reasoning".into(),
                artefact: Some(OrchestrationArtefactKind::Reasoning),
                statement: "No current reasoning record — reasoning refresh considered".into(),
                evidence_refs: vec![],
            });
        } else if let Some(record) = &reasoning.current {
            reasoning_links.push(OrchestrationEvidenceLink {
                external_ref: record.id.clone(),
                kind: "reasoning_record".into(),
            });
            if record.uncertainty > 70 {
                stale_items.push(OrchestrationObservation {
                    kind: "uncertainty_increases".into(),
                    artefact: Some(OrchestrationArtefactKind::Reasoning),
                    statement: format!(
                        "Reasoning uncertainty is elevated ({})",
                        record.uncertainty
                    ),
                    evidence_refs: vec![record.id.clone()],
                });
            }
            if record.confidence < 40 {
                stale_items.push(OrchestrationObservation {
                    kind: "confidence_degradation".into(),
                    artefact: Some(OrchestrationArtefactKind::Reasoning),
                    statement: format!(
                        "Reasoning confidence is degraded ({})",
                        record.confidence
                    ),
                    evidence_refs: vec![record.id.clone()],
                });
            }
            if !has_planning {
                blocked_items.push(OrchestrationObservation {
                    kind: "broken_references".into(),
                    artefact: Some(OrchestrationArtefactKind::Reasoning),
                    statement: "Reasoning present without current planning upstream".into(),
                    evidence_refs: vec![record.id.clone()],
                });
            }
        }

        if !has_graph {
            stale_items.push(OrchestrationObservation {
                kind: "graph_divergence".into(),
                artefact: Some(OrchestrationArtefactKind::CognitiveGraph),
                statement: "No current cognitive graph — graph refresh considered".into(),
                evidence_refs: vec![],
            });
        } else if let Some(view) = &graph.current {
            graph_links.push(OrchestrationEvidenceLink {
                external_ref: view.meta.id.clone(),
                kind: "cognitive_graph".into(),
            });
            if view.meta.broken_node_count > 0 || view.meta.broken_edge_count > 0 {
                stale_items.push(OrchestrationObservation {
                    kind: "broken_references".into(),
                    artefact: Some(OrchestrationArtefactKind::CognitiveGraph),
                    statement: format!(
                        "Graph has {} broken nodes and {} broken edges",
                        view.meta.broken_node_count, view.meta.broken_edge_count
                    ),
                    evidence_refs: vec![view.meta.id.clone()],
                });
            }
        }

        // Execution links collected above are references only — never launch or claim.

        for cycle in &order_result.cycles {
            skipped_items.push(OrchestrationObservation {
                kind: "cycle_detected".into(),
                artefact: None,
                statement: "Refresh ordering incomplete due to dependency cycle — no resolution invented"
                    .into(),
                evidence_refs: cycle.evidence_refs.clone(),
            });
            for artefact in &cycle.cycle {
                skipped_items.push(OrchestrationObservation {
                    kind: "intentionally_skipped".into(),
                    artefact: Some(*artefact),
                    statement: format!(
                        "{} refresh intentionally skipped until cycle evidence is resolved externally",
                        artefact.as_str()
                    ),
                    evidence_refs: cycle.evidence_refs.clone(),
                });
            }
        }

        let uncertainty = Self::compute_uncertainty(
            has_cognitive,
            has_planning,
            has_reasoning,
            has_graph,
            &stale_items,
            &order_result.cycles,
        );

        let rationale = format!(
            "Orchestration generation {current_generation}: dependency order has {} stage(s), {} stale observation(s), {} blocked, {} skipped, {} cycle(s). Refresh plan is descriptive only.",
            refresh_plan.len(),
            stale_items.len(),
            blocked_items.len(),
            skipped_items.len(),
            order_result.cycles.len()
        );

        let meta = WorkspaceOrchestrationMeta::new(
            workspace_id,
            now,
            current_generation,
            uncertainty,
            rationale,
        )
        .map_err(KernelError::from)?;

        Ok(WorkspaceOrchestrationView {
            meta,
            refresh_plan,
            dependency_order: order_result.order,
            dependencies,
            blocked_items,
            stale_items,
            skipped_items,
            cycles: order_result.cycles,
            evidence_links,
            graph_links,
            planning_links,
            reasoning_links,
            execution_links,
            authority_effect: WorkspaceOrchestrationView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn compute_uncertainty(
        has_cognitive: bool,
        has_planning: bool,
        has_reasoning: bool,
        has_graph: bool,
        stale: &[OrchestrationObservation],
        cycles: &[workspace_domain::OrchestrationCycleDetected],
    ) -> u8 {
        let mut score: u16 = 20;
        if !has_cognitive {
            score += 15;
        }
        if !has_planning {
            score += 15;
        }
        if !has_reasoning {
            score += 15;
        }
        if !has_graph {
            score += 10;
        }
        score += (stale.len() as u16).saturating_mul(3);
        if !cycles.is_empty() {
            score += 25;
        }
        score.min(100) as u8
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &workspace_domain::IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )?;
        let _ = subject_id;
        Ok(())
    }
}
