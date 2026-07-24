//! Workspace Task Graph service (Phase 5 — Sprints 82–83).
//!
//! Persistent work model. Syncs intent tasks into graph nodes, manages
//! relationships, and feeds Intelligence / Decision / Attention.
//! Never executes or grants authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{Database, TaskGraphRepository};
use workspace_domain::{
    would_create_cycle, ActorContext, IntentContext, TaskGraph, TaskGraphError, TaskGraphSummary,
    TaskPriority as IntentTaskPriority, TaskRelationship, TaskRelationshipKind,
    TaskStatus as IntentTaskStatus, WorkspaceTask, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{AuditService, WorkspaceIntentService};

pub(crate) struct TaskGraphService;

impl TaskGraphService {
    /// Load graph; sync intent tasks into durable nodes (idempotent).
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<TaskGraph> {
        let workspace_id = workspace_id.into();
        Self::sync_intent_tasks(db, actor, &workspace_id)?;
        let (tasks, relationships) = Self::load(db, &workspace_id)?;
        let (integrity_ok, integrity_notes) = Self::validate_integrity(&tasks, &relationships);
        let graph = TaskGraph::from_parts(
            workspace_id,
            tasks,
            relationships,
            integrity_ok,
            integrity_notes,
        );
        Self::audit(
            db,
            actor,
            "task_graph.updated",
            json!({
                "workspace_id": graph.workspace_id,
                "node_count": graph.nodes.len(),
                "relationship_count": graph.relationships.len(),
                "integrity_ok": graph.integrity_ok,
                "authority_effect": "none",
            }),
        )?;
        Ok(graph)
    }

    pub(crate) fn summary(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        limit: usize,
    ) -> Result<TaskGraphSummary> {
        Ok(Self::generate(db, actor, workspace_id)?.summary_projection(limit))
    }

    pub(crate) fn create_task(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        project_id: Option<String>,
        priority: WorkspaceTaskPriority,
    ) -> Result<WorkspaceTask> {
        let mut task = WorkspaceTask::new(workspace_id, title, project_id, priority)
            .map_err(KernelError::from)?;
        task.status = WorkspaceTaskStatus::Planned;
        task.explanation = "Created in Task Graph (planned).".into();
        Self::persist_task(db, &task)?;
        Self::audit(
            db,
            actor,
            "task_node.created",
            json!({
                "workspace_id": task.workspace_id.as_str(),
                "task_id": task.id.as_str(),
                "title": task.title,
                "authority_effect": "none",
            }),
        )?;
        Self::audit(
            db,
            actor,
            "task_graph.updated",
            json!({
                "workspace_id": task.workspace_id.as_str(),
                "reason": "task_node.created",
                "authority_effect": "none",
            }),
        )?;
        Ok(task)
    }

    pub(crate) fn update_task_status(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        task_id: impl Into<String>,
        status: WorkspaceTaskStatus,
        explanation: Option<String>,
    ) -> Result<WorkspaceTask> {
        let task_id = task_id.into();
        let mut task = Self::get_task(db, &task_id)?
            .ok_or_else(|| KernelError::from(TaskGraphError::TaskNotFound))?;
        let explanation = explanation.unwrap_or_else(|| match status {
            WorkspaceTaskStatus::Completed => "Marked completed via Task Graph.".into(),
            WorkspaceTaskStatus::Blocked => "Marked blocked via Task Graph.".into(),
            WorkspaceTaskStatus::Waiting => "Marked waiting via Task Graph.".into(),
            WorkspaceTaskStatus::InProgress => "Marked in progress via Task Graph.".into(),
            other => format!("Status set to {}.", other.as_str()),
        });
        task = task.with_status(status, explanation);
        Self::persist_task(db, &task)?;
        let event = if status == WorkspaceTaskStatus::Completed {
            "task_node.completed"
        } else {
            "task_node.updated"
        };
        Self::audit(
            db,
            actor,
            event,
            json!({
                "workspace_id": task.workspace_id.as_str(),
                "task_id": task.id.as_str(),
                "status": task.status.as_str(),
                "authority_effect": "none",
            }),
        )?;
        Ok(task)
    }

    pub(crate) fn add_relationship(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        from_task_id: impl Into<String>,
        to_task_id: impl Into<String>,
        kind: TaskRelationshipKind,
    ) -> Result<TaskRelationship> {
        let workspace_id = workspace_id.into();
        let from_task_id = from_task_id.into();
        let to_task_id = to_task_id.into();
        let from = Self::get_task(db, &from_task_id)?
            .ok_or_else(|| KernelError::from(TaskGraphError::TaskNotFound))?;
        let to = Self::get_task(db, &to_task_id)?
            .ok_or_else(|| KernelError::from(TaskGraphError::TaskNotFound))?;
        if from.workspace_id.as_str() != workspace_id || to.workspace_id.as_str() != workspace_id {
            return Err(KernelError::from(TaskGraphError::TaskNotFound));
        }

        let (_, existing) = Self::load(db, &workspace_id)?;
        if would_create_cycle(&existing, &from_task_id, &to_task_id, kind) {
            return Err(KernelError::from(TaskGraphError::CircularDependency));
        }

        let rel = TaskRelationship::new(&workspace_id, &from_task_id, &to_task_id, kind)
            .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            TaskGraphRepository::new(&guard).insert_relationship(&rel)?;
        }

        // Derive waiting/blocked explanations when depends_on / blocks added.
        if kind == TaskRelationshipKind::DependsOn
            && from.status.is_open()
            && to.status.is_open()
            && from.status != WorkspaceTaskStatus::Completed
        {
            let _ = Self::update_task_status(
                db,
                actor,
                from.id.as_str(),
                WorkspaceTaskStatus::Waiting,
                Some(format!(
                    "This task is waiting because Task {} must complete first.",
                    to.title
                )),
            );
        }
        if kind == TaskRelationshipKind::Blocks
            && to.status.is_open()
            && from.status.is_open()
        {
            let _ = Self::update_task_status(
                db,
                actor,
                to.id.as_str(),
                WorkspaceTaskStatus::Blocked,
                Some(format!("This task is blocked by Task {}.", from.title)),
            );
        }

        Self::audit(
            db,
            actor,
            "task_relationship.created",
            json!({
                "workspace_id": workspace_id,
                "relationship_id": rel.id,
                "kind": kind.as_str(),
                "from": from_task_id,
                "to": to_task_id,
                "authority_effect": "none",
            }),
        )?;
        Ok(rel)
    }

    pub(crate) fn remove_relationship(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        relationship_id: impl Into<String>,
    ) -> Result<()> {
        let relationship_id = relationship_id.into();
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            TaskGraphRepository::new(&guard).delete_relationship(&relationship_id)?;
        }
        Self::audit(
            db,
            actor,
            "task_relationship.removed",
            json!({
                "relationship_id": relationship_id,
                "authority_effect": "none",
            }),
        )?;
        Ok(())
    }

    pub(crate) fn validate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<TaskGraph> {
        let graph = Self::generate(db, actor, workspace_id)?;
        Ok(graph)
    }

    /// Planning context: open incomplete nodes only (no duplication of planner state).
    pub(crate) fn planning_inputs(graph: &TaskGraph) -> Vec<WorkspaceTask> {
        graph
            .open_incomplete_nodes()
            .into_iter()
            .map(|n| n.task.clone())
            .collect()
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(TaskGraphError::CannotExecute))
    }

    fn sync_intent_tasks(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: &str,
    ) -> Result<()> {
        let intent_tasks = WorkspaceIntentService::list_tasks(db, workspace_id, None, 200)?;
        let mut created = 0usize;
        for intent in intent_tasks {
            if intent.deleted {
                continue;
            }
            let existing = {
                let guard = db
                    .lock()
                    .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
                TaskGraphRepository::new(&guard)
                    .find_by_intent_task(workspace_id, intent.id.as_str())?
            };
            if let Some(mut node) = existing {
                // Keep title/priority/status aligned with intent when not graph-advanced.
                node.title = intent.title.clone();
                node.priority = map_intent_priority(intent.priority);
                let mapped = map_intent_status(intent.status);
                if node.status.is_terminal() && mapped.is_terminal() {
                    node.status = mapped;
                } else if !node.status.is_terminal() {
                    // Preserve waiting/blocked graph states unless intent completed/cancelled.
                    if mapped.is_terminal() {
                        node.status = mapped;
                        if mapped == WorkspaceTaskStatus::Completed {
                            node.progress_percent = 100;
                        }
                    } else if !matches!(
                        node.status,
                        WorkspaceTaskStatus::Waiting | WorkspaceTaskStatus::Blocked
                    ) {
                        node.status = mapped;
                    }
                }
                node.project_id = Some(intent.project_id.to_string());
                node.touch();
                Self::persist_task(db, &node)?;
            } else {
                let mut task = WorkspaceTask::new(
                    workspace_id,
                    intent.title.clone(),
                    Some(intent.project_id.to_string()),
                    map_intent_priority(intent.priority),
                )
                .map_err(KernelError::from)?;
                task.source_intent_task_id = Some(intent.id.to_string());
                task.status = map_intent_status(intent.status);
                task.created_at = intent.created_at.clone();
                task.updated_at = intent.updated_at.clone();
                task.explanation = "Projected from Workspace Intent task.".into();
                if task.status == WorkspaceTaskStatus::Completed {
                    task.progress_percent = 100;
                }
                Self::persist_task(db, &task)?;
                created += 1;
                Self::audit(
                    db,
                    actor,
                    "task_node.created",
                    json!({
                        "workspace_id": workspace_id,
                        "task_id": task.id.as_str(),
                        "source_intent_task_id": intent.id.as_str(),
                        "authority_effect": "none",
                    }),
                )?;
            }
        }
        if created > 0 {
            Self::audit(
                db,
                actor,
                "task_graph.created",
                json!({
                    "workspace_id": workspace_id,
                    "nodes_projected": created,
                    "authority_effect": "none",
                }),
            )?;
        }
        Ok(())
    }

    fn load(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<(Vec<WorkspaceTask>, Vec<TaskRelationship>)> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        let repo = TaskGraphRepository::new(&guard);
        Ok((repo.list_tasks(workspace_id)?, repo.list_relationships(workspace_id)?))
    }

    fn get_task(db: &Arc<Mutex<Database>>, id: &str) -> Result<Option<WorkspaceTask>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        Ok(TaskGraphRepository::new(&guard).get_task(id)?)
    }

    fn persist_task(db: &Arc<Mutex<Database>>, task: &WorkspaceTask) -> Result<()> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        TaskGraphRepository::new(&guard).upsert_task(task)?;
        Ok(())
    }

    fn validate_integrity(
        tasks: &[WorkspaceTask],
        relationships: &[TaskRelationship],
    ) -> (bool, Vec<String>) {
        let mut notes = Vec::new();
        let ids: std::collections::HashSet<_> =
            tasks.iter().map(|t| t.id.as_str().to_string()).collect();
        for rel in relationships {
            if !ids.contains(&rel.from_task_id) || !ids.contains(&rel.to_task_id) {
                notes.push(format!(
                    "Dangling relationship {} references missing task.",
                    rel.id
                ));
            }
        }
        if has_any_cycle(relationships) {
            notes.push("Graph contains at least one ordering cycle.".into());
        }
        (notes.is_empty(), notes)
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )
    }
}

fn map_intent_status(status: IntentTaskStatus) -> WorkspaceTaskStatus {
    match status {
        IntentTaskStatus::Todo => WorkspaceTaskStatus::Planned,
        IntentTaskStatus::InProgress => WorkspaceTaskStatus::InProgress,
        IntentTaskStatus::Blocked => WorkspaceTaskStatus::Blocked,
        IntentTaskStatus::Done => WorkspaceTaskStatus::Completed,
        IntentTaskStatus::Cancelled => WorkspaceTaskStatus::Cancelled,
    }
}

fn map_intent_priority(priority: IntentTaskPriority) -> WorkspaceTaskPriority {
    match priority {
        IntentTaskPriority::Low => WorkspaceTaskPriority::Low,
        IntentTaskPriority::Medium => WorkspaceTaskPriority::Medium,
        IntentTaskPriority::High => WorkspaceTaskPriority::High,
    }
}

fn has_any_cycle(relationships: &[TaskRelationship]) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for rel in relationships {
        if !rel.kind.is_ordering() {
            continue;
        }
        adj.entry(rel.from_task_id.clone())
            .or_default()
            .push(rel.to_task_id.clone());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();

    fn dfs(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }
        visiting.insert(node.to_string());
        if let Some(next) = adj.get(node) {
            for child in next {
                if dfs(child, adj, visiting, visited) {
                    return true;
                }
            }
        }
        visiting.remove(node);
        visited.insert(node.to_string());
        false
    }

    let nodes: Vec<_> = adj.keys().cloned().collect();
    for node in nodes {
        if dfs(&node, &adj, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}
