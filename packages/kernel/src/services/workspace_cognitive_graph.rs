//! Workspace Cognitive Graph — Programme II Batch 4.
//!
//! Cross-domain reference-only topology. Composes Cognitive Model, Planning,
//! Reasoning Memory, Intent, Task Graph, and related surfaces. Never executes,
//! never invents relationships, never owns foreign lifecycles.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{CognitiveGraphRepository, Database};
use workspace_domain::{
    dedupe_edges, dedupe_nodes, ActorContext, CognitiveGraphEdge, CognitiveGraphEdgeKind,
    CognitiveGraphMeta, CognitiveGraphNode, CognitiveGraphNodeKind, CognitiveGraphSnapshot,
    CognitiveGraphStatus, CognitiveGraphView, CognitiveNodeKind, CognitiveRelationKind,
    IntentContext, TaskRelationshipKind,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, TaskGraphService, WorkspaceCognitiveModelService, WorkspacePlanningService,
    WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceCognitiveGraphService;

impl WorkspaceCognitiveGraphService {
    /// Generate and persist a graph snapshot from explicit source relationships only.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveGraphSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, actor, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = CognitiveGraphRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.cognitive_graph.snapshot_generated",
            &view.meta.id,
            json!({
                "workspace_id": workspace_id,
                "node_count": view.meta.node_count,
                "edge_count": view.meta.edge_count,
                "broken_node_count": view.meta.broken_node_count,
                "broken_edge_count": view.meta.broken_edge_count,
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveGraphSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = CognitiveGraphRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == CognitiveGraphStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(CognitiveGraphSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "WorkspaceCognitiveGraphService cannot execute, dispatch, claim, or launch"
                .into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "Cognitive Graph cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "Cognitive Graph cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_create_decision() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "Cognitive Graph cannot create decisions".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "Cognitive Graph cannot mutate lifecycle".into(),
        })
    }

    pub(crate) fn attempt_invent_relationship() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveGraphValidation {
            message: "Cognitive Graph cannot invent missing relationships".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, actor, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = CognitiveGraphRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced cognitive graph transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceCognitiveGraphValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: &str,
        now: &str,
    ) -> Result<CognitiveGraphView> {
        let cognitive = WorkspaceCognitiveModelService::generate_model(db, workspace_id)?;
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id)?;
        let reasoning = WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id)?;
        let task_graph = TaskGraphService::generate(db, actor, workspace_id)?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut known_refs: HashSet<String> = HashSet::new();

        for node in &cognitive.nodes {
            let kind = map_cognitive_node_kind(node.kind);
            let n = CognitiveGraphNode::new(
                workspace_id,
                kind,
                node.id.clone(),
                node.title.clone(),
                false,
            )
            .map_err(KernelError::from)?;
            known_refs.insert(n.external_ref.clone());
            nodes.push(n);
            if let Some(ext) = &node.external_ref {
                let goal_node = CognitiveGraphNode::new(
                    workspace_id,
                    CognitiveGraphNodeKind::Goal,
                    ext.clone(),
                    format!("Intent ref {ext}"),
                    false,
                )
                .map_err(KernelError::from)?;
                // Intent WorkGoal may or may not be present as a separate node;
                // record as reference edge only when cognitive goal points at it.
                if node.kind == CognitiveNodeKind::Goal {
                    known_refs.insert(ext.clone());
                    nodes.push(goal_node);
                    edges.push(
                        CognitiveGraphEdge::new(
                            workspace_id,
                            node.id.clone(),
                            ext.clone(),
                            CognitiveGraphEdgeKind::References,
                            "Cognitive Goal references WorkGoal",
                            90,
                            vec![node.id.clone()],
                            false,
                        )
                        .map_err(KernelError::from)?,
                    );
                }
            }
        }

        for rel in &cognitive.relations {
            let kind = map_cognitive_relation(rel.kind);
            let from_ok = known_refs.contains(&rel.from_id);
            let to_ok = known_refs.contains(&rel.to_id);
            let broken = !(from_ok && to_ok);
            if !from_ok {
                nodes.push(
                    CognitiveGraphNode::new(
                        workspace_id,
                        CognitiveGraphNodeKind::BrokenReference,
                        rel.from_id.clone(),
                        format!("Unresolved {}", rel.from_id),
                        true,
                    )
                    .map_err(KernelError::from)?,
                );
                known_refs.insert(rel.from_id.clone());
            }
            if !to_ok {
                nodes.push(
                    CognitiveGraphNode::new(
                        workspace_id,
                        CognitiveGraphNodeKind::BrokenReference,
                        rel.to_id.clone(),
                        format!("Unresolved {}", rel.to_id),
                        true,
                    )
                    .map_err(KernelError::from)?,
                );
                known_refs.insert(rel.to_id.clone());
            }
            edges.push(
                CognitiveGraphEdge::new(
                    workspace_id,
                    rel.from_id.clone(),
                    rel.to_id.clone(),
                    kind,
                    rel.explanation
                        .clone()
                        .unwrap_or_else(|| format!("Cognitive relation {}", rel.kind.as_str())),
                    rel.confidence,
                    vec![rel.id.clone()],
                    broken,
                )
                .map_err(KernelError::from)?,
            );
        }

        if let Some(proposal) = &planning.current {
            let plan_ref = proposal.plan.id.clone();
            nodes.push(
                CognitiveGraphNode::new(
                    workspace_id,
                    CognitiveGraphNodeKind::Plan,
                    plan_ref.clone(),
                    proposal.plan.title.clone(),
                    false,
                )
                .map_err(KernelError::from)?,
            );
            known_refs.insert(plan_ref.clone());
            for step in &proposal.steps {
                for r in &step.evidence_refs {
                    ensure_ref_node(&mut nodes, &mut known_refs, workspace_id, r)?;
                    edges.push(
                        CognitiveGraphEdge::new(
                            workspace_id,
                            plan_ref.clone(),
                            r.clone(),
                            CognitiveGraphEdgeKind::DerivedFrom,
                            format!("Planning step '{}' cites evidence", step.title),
                            step.confidence,
                            vec![step.id.clone()],
                            !known_refs.contains(r),
                        )
                        .map_err(KernelError::from)?,
                    );
                }
            }
            for e in &proposal.evidence_refs {
                ensure_ref_node(&mut nodes, &mut known_refs, workspace_id, &e.external_ref)?;
                edges.push(
                    CognitiveGraphEdge::new(
                        workspace_id,
                        plan_ref.clone(),
                        e.external_ref.clone(),
                        CognitiveGraphEdgeKind::InformedBy,
                        format!("Plan informed by {}", e.kind),
                        proposal.plan.confidence,
                        vec![plan_ref.clone()],
                        false,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            for dep in &proposal.dependencies {
                // Planning step deps are between planning_step ids — project as relates_to
                // only when both step ids are already known via evidence; otherwise skip
                // inventing step nodes. Explicit step identities as evidence refs only.
                let _ = dep;
            }
        }

        if let Some(record) = &reasoning.current {
            nodes.push(
                CognitiveGraphNode::new(
                    workspace_id,
                    CognitiveGraphNodeKind::Reasoning,
                    record.id.clone(),
                    record.title.clone(),
                    false,
                )
                .map_err(KernelError::from)?,
            );
            known_refs.insert(record.id.clone());
            for link in &record.links {
                ensure_ref_node(&mut nodes, &mut known_refs, workspace_id, &link.external_ref)?;
                edges.push(
                    CognitiveGraphEdge::new(
                        workspace_id,
                        record.id.clone(),
                        link.external_ref.clone(),
                        CognitiveGraphEdgeKind::DerivedFrom,
                        link.note.clone(),
                        record.confidence,
                        vec![record.id.clone()],
                        false,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            for e in &record.evidence_refs {
                ensure_ref_node(&mut nodes, &mut known_refs, workspace_id, &e.external_ref)?;
                edges.push(
                    CognitiveGraphEdge::new(
                        workspace_id,
                        record.id.clone(),
                        e.external_ref.clone(),
                        CognitiveGraphEdgeKind::EvidenceFor,
                        format!("Reasoning evidence ({})", e.kind),
                        record.confidence,
                        vec![record.id.clone()],
                        false,
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }

        for node in &task_graph.nodes {
            let task_ref = format!("task:{}", node.task.id.as_str());
            nodes.push(
                CognitiveGraphNode::new(
                    workspace_id,
                    CognitiveGraphNodeKind::Task,
                    task_ref.clone(),
                    node.task.title.clone(),
                    false,
                )
                .map_err(KernelError::from)?,
            );
            known_refs.insert(task_ref);
        }
        for rel in &task_graph.relationships {
            let from = format!("task:{}", rel.from_task_id);
            let to = format!("task:{}", rel.to_task_id);
            let kind = map_task_relationship(rel.kind);
            let broken = !(known_refs.contains(&from) && known_refs.contains(&to));
            if broken {
                if !known_refs.contains(&from) {
                    nodes.push(
                        CognitiveGraphNode::new(
                            workspace_id,
                            CognitiveGraphNodeKind::BrokenReference,
                            from.clone(),
                            format!("Unresolved {from}"),
                            true,
                        )
                        .map_err(KernelError::from)?,
                    );
                    known_refs.insert(from.clone());
                }
                if !known_refs.contains(&to) {
                    nodes.push(
                        CognitiveGraphNode::new(
                            workspace_id,
                            CognitiveGraphNodeKind::BrokenReference,
                            to.clone(),
                            format!("Unresolved {to}"),
                            true,
                        )
                        .map_err(KernelError::from)?,
                    );
                    known_refs.insert(to.clone());
                }
            }
            edges.push(
                CognitiveGraphEdge::new(
                    workspace_id,
                    from,
                    to,
                    kind,
                    format!("Task Graph {}", rel.kind.as_str()),
                    80,
                    vec![rel.id.clone()],
                    broken,
                )
                .map_err(KernelError::from)?,
            );
        }

        let nodes = dedupe_nodes(nodes);
        let known: HashSet<_> = nodes.iter().map(|n| n.external_ref.clone()).collect();
        let broken_refs: HashSet<_> = nodes
            .iter()
            .filter(|n| n.broken)
            .map(|n| n.external_ref.clone())
            .collect();
        let edges: Vec<_> = dedupe_edges(edges)
            .into_iter()
            .map(|mut e| {
                e.broken = !known.contains(&e.from_ref)
                    || !known.contains(&e.to_ref)
                    || broken_refs.contains(&e.from_ref)
                    || broken_refs.contains(&e.to_ref);
                e
            })
            .collect();

        let broken_node_count = nodes.iter().filter(|n| n.broken).count();
        let broken_edge_count = edges.iter().filter(|e| e.broken).count();
        let meta = CognitiveGraphMeta::new(
            workspace_id,
            now,
            nodes.len(),
            edges.len(),
            broken_node_count,
            broken_edge_count,
        );

        Ok(CognitiveGraphView {
            meta,
            nodes,
            edges,
            authority_effect: CognitiveGraphView::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        detail: serde_json::Value,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            json!({
                "subject_id": subject_id,
                "detail": detail,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn map_cognitive_node_kind(kind: CognitiveNodeKind) -> CognitiveGraphNodeKind {
    match kind {
        CognitiveNodeKind::Goal => CognitiveGraphNodeKind::Goal,
        CognitiveNodeKind::Objective => CognitiveGraphNodeKind::Objective,
        CognitiveNodeKind::Initiative => CognitiveGraphNodeKind::Initiative,
        CognitiveNodeKind::Milestone => CognitiveGraphNodeKind::Milestone,
        CognitiveNodeKind::Context => CognitiveGraphNodeKind::Context,
        CognitiveNodeKind::WorkingSet => CognitiveGraphNodeKind::WorkingSet,
        CognitiveNodeKind::Constraint => CognitiveGraphNodeKind::Constraint,
        CognitiveNodeKind::Risk => CognitiveGraphNodeKind::Risk,
        CognitiveNodeKind::Opportunity => CognitiveGraphNodeKind::Opportunity,
    }
}

fn map_cognitive_relation(kind: CognitiveRelationKind) -> CognitiveGraphEdgeKind {
    match kind {
        CognitiveRelationKind::DependsOn => CognitiveGraphEdgeKind::DependsOn,
        CognitiveRelationKind::Blocks => CognitiveGraphEdgeKind::Blocks,
        CognitiveRelationKind::Supports => CognitiveGraphEdgeKind::Supports,
        CognitiveRelationKind::Constrains => CognitiveGraphEdgeKind::Constrains,
        CognitiveRelationKind::Mitigates => CognitiveGraphEdgeKind::Mitigates,
        CognitiveRelationKind::Enables => CognitiveGraphEdgeKind::Enables,
        CognitiveRelationKind::PartOf => CognitiveGraphEdgeKind::PartOf,
        CognitiveRelationKind::Focuses => CognitiveGraphEdgeKind::Focuses,
    }
}

fn map_task_relationship(kind: TaskRelationshipKind) -> CognitiveGraphEdgeKind {
    match kind.as_str() {
        "blocks" => CognitiveGraphEdgeKind::Blocks,
        "depends_on" => CognitiveGraphEdgeKind::DependsOn,
        "related" | "relates_to" => CognitiveGraphEdgeKind::RelatesTo,
        _ => CognitiveGraphEdgeKind::RelatesTo,
    }
}

fn ensure_ref_node(
    nodes: &mut Vec<CognitiveGraphNode>,
    known: &mut HashSet<String>,
    workspace_id: &str,
    external_ref: &str,
) -> Result<()> {
    if known.contains(external_ref) {
        return Ok(());
    }
    let kind = infer_kind_from_ref(external_ref);
    let broken = kind == CognitiveGraphNodeKind::BrokenReference;
    nodes.push(
        CognitiveGraphNode::new(
            workspace_id,
            kind,
            external_ref,
            format!("Referenced {external_ref}"),
            broken,
        )
        .map_err(KernelError::from)?,
    );
    known.insert(external_ref.into());
    Ok(())
}

fn infer_kind_from_ref(external_ref: &str) -> CognitiveGraphNodeKind {
    if external_ref.starts_with("planning_plan:") {
        CognitiveGraphNodeKind::Plan
    } else if external_ref.starts_with("work_goal:") {
        CognitiveGraphNodeKind::Goal
    } else if external_ref.starts_with("task:") {
        CognitiveGraphNodeKind::Task
    } else if external_ref.starts_with("reasoning:") {
        CognitiveGraphNodeKind::Reasoning
    } else if external_ref.starts_with("recommendation:") {
        CognitiveGraphNodeKind::Recommendation
    } else if external_ref.starts_with("decision:") {
        CognitiveGraphNodeKind::Decision
    } else if external_ref.starts_with("memory:") {
        CognitiveGraphNodeKind::Memory
    } else if external_ref.starts_with("attention:") {
        CognitiveGraphNodeKind::Attention
    } else if external_ref.starts_with("purpose:") {
        CognitiveGraphNodeKind::Purpose
    } else if external_ref.starts_with("execution:") {
        CognitiveGraphNodeKind::Execution
    } else if external_ref.starts_with("cognitive:") {
        CognitiveGraphNodeKind::Context
    } else {
        CognitiveGraphNodeKind::BrokenReference
    }
}
