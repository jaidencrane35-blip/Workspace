//! Workspace Intent service — durable work context (Phase 4 Batch 5).
//!
//! Permission-neutral persistence. Never executes or grants authority.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceIntentRepository};
use workspace_domain::{
    ActorContext, IntentContext, Project, ProjectId, ProjectStatus, Task, TaskId, TaskPriority,
    TaskStatus, WorkGoal, WorkflowContext, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct WorkspaceIntentService;

impl WorkspaceIntentService {
    pub(crate) fn create_project(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        name: impl Into<String>,
        description: Option<String>,
        metadata: Option<String>,
    ) -> Result<Project> {
        let project = Project::new(workspace_id, name, description, metadata)
            .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_project(&project)?;
        }
        Self::audit_intent(db, actor, "workspace.intent.created", &project.id.to_string(), "project")?;
        Ok(project)
    }

    pub(crate) fn update_project(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        project_id: impl Into<String>,
        name: Option<String>,
        description: Option<Option<String>>,
        status: Option<ProjectStatus>,
    ) -> Result<Project> {
        let project_id = ProjectId::new(project_id.into()).map_err(KernelError::Domain)?;
        let mut project = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard)
                .get_project(&project_id)?
                .ok_or_else(|| KernelError::WorkspaceIntentValidation {
                    message: "project not found".into(),
                })?
        };
        if let Some(name) = name {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err(KernelError::WorkspaceIntentValidation {
                    message: "project name must not be empty".into(),
                });
            }
            project.name = trimmed.to_string();
        }
        if let Some(description) = description {
            project.description = description.and_then(|v| {
                let t = v.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            });
        }
        if let Some(status) = status {
            project.status = status;
        }
        project.touch();
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_project(&project)?;
        }
        Self::audit_intent(db, actor, "workspace.intent.updated", &project.id.to_string(), "project")?;
        Ok(project)
    }

    pub(crate) fn get_project(
        db: &Arc<Mutex<Database>>,
        project_id: impl Into<String>,
    ) -> Result<Project> {
        let project_id = ProjectId::new(project_id.into()).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        WorkspaceIntentRepository::new(&guard)
            .get_project(&project_id)?
            .ok_or_else(|| KernelError::WorkspaceIntentValidation {
                message: "project not found".into(),
            })
    }

    pub(crate) fn list_projects(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<Project>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        WorkspaceIntentRepository::new(&guard)
            .list_projects(workspace_id, limit)
            .map_err(Into::into)
    }

    pub(crate) fn create_task(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        project_id: impl Into<String>,
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        priority: TaskPriority,
    ) -> Result<Task> {
        let project_id = project_id.into();
        let workspace_id = workspace_id.into();
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            let project = WorkspaceIntentRepository::new(&guard)
                .get_project(&ProjectId::new(project_id.clone()).map_err(KernelError::Domain)?)?
                .ok_or_else(|| KernelError::WorkspaceIntentValidation {
                    message: "project not found".into(),
                })?;
            if project.workspace_id.as_str() != workspace_id {
                return Err(KernelError::WorkspaceIntentValidation {
                    message: "project does not belong to workspace".into(),
                });
            }
        }
        let task = Task::new(project_id, workspace_id, title, priority).map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_task(&task)?;
        }
        Self::audit_intent(db, actor, "workspace.intent.created", &task.id.to_string(), "task")?;
        Ok(task)
    }

    pub(crate) fn update_task(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        task_id: impl Into<String>,
        title: Option<String>,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
    ) -> Result<Task> {
        let task_id = TaskId::new(task_id.into()).map_err(KernelError::Domain)?;
        let mut task = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard)
                .get_task(&task_id)?
                .ok_or_else(|| KernelError::WorkspaceIntentValidation {
                    message: "task not found".into(),
                })?
        };
        if let Some(title) = title {
            let trimmed = title.trim();
            if trimmed.is_empty() {
                return Err(KernelError::WorkspaceIntentValidation {
                    message: "task title must not be empty".into(),
                });
            }
            task.title = trimmed.to_string();
        }
        if let Some(status) = status {
            task.status = status;
        }
        if let Some(priority) = priority {
            task.priority = priority;
        }
        task.touch();
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_task(&task)?;
        }
        Self::audit_intent(db, actor, "workspace.intent.updated", &task.id.to_string(), "task")?;
        Ok(task)
    }

    pub(crate) fn get_task(
        db: &Arc<Mutex<Database>>,
        task_id: impl Into<String>,
    ) -> Result<Task> {
        let task_id = TaskId::new(task_id.into()).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        WorkspaceIntentRepository::new(&guard)
            .get_task(&task_id)?
            .ok_or_else(|| KernelError::WorkspaceIntentValidation {
                message: "task not found".into(),
            })
    }

    pub(crate) fn list_tasks(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        project_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Task>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        WorkspaceIntentRepository::new(&guard)
            .list_tasks(workspace_id, project_id, limit)
            .map_err(Into::into)
    }

    pub(crate) fn create_goal(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        description: impl Into<String>,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<WorkGoal> {
        let goal = WorkGoal::new(workspace_id, description, project_id, task_id)
            .map_err(KernelError::from)?;
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_goal(&goal)?;
        }
        Self::audit_intent(db, actor, "workspace.intent.created", &goal.id.to_string(), "goal")?;
        Ok(goal)
    }

    pub(crate) fn list_goals(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<WorkGoal>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        WorkspaceIntentRepository::new(&guard)
            .list_goals(workspace_id, limit)
            .map_err(Into::into)
    }

    /// Read-only workflow context. Missing rows yield an empty in-memory context (no insert).
    pub(crate) fn get_workflow_context_readonly(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<WorkflowContext> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        if let Some(existing) =
            WorkspaceIntentRepository::new(&guard).get_workflow_context(workspace_id.as_str())?
        {
            return Ok(existing);
        }
        WorkflowContext::empty(workspace_id.as_str()).map_err(KernelError::from)
    }

    pub(crate) fn get_or_create_workflow_context(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<WorkflowContext> {
        let workspace_id = WorkspaceId::new(workspace_id).map_err(KernelError::Domain)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        let repo = WorkspaceIntentRepository::new(&guard);
        if let Some(existing) = repo.get_workflow_context(workspace_id.as_str())? {
            return Ok(existing);
        }
        let context = WorkflowContext::empty(workspace_id.as_str()).map_err(KernelError::from)?;
        repo.upsert_workflow_context(&context)?;
        Ok(context)
    }

    pub(crate) fn set_active_work(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: &str,
        project_id: Option<String>,
        task_id: Option<String>,
    ) -> Result<WorkflowContext> {
        let mut context = Self::get_or_create_workflow_context(db, workspace_id)?;
        let active_project = project_id
            .map(ProjectId::new)
            .transpose()
            .map_err(KernelError::Domain)?;
        let active_task = task_id
            .map(TaskId::new)
            .transpose()
            .map_err(KernelError::Domain)?;

        if let Some(ref project_id) = active_project {
            let project = Self::get_project(db, project_id.as_str())?;
            if project.workspace_id.as_str() != workspace_id || project.deleted {
                return Err(KernelError::WorkspaceIntentValidation {
                    message: "active project does not belong to this workspace".into(),
                });
            }
        }
        if let Some(ref task_id) = active_task {
            let task = Self::get_task(db, task_id.as_str())?;
            if task.workspace_id.as_str() != workspace_id || task.deleted {
                return Err(KernelError::WorkspaceIntentValidation {
                    message: "active task does not belong to this workspace".into(),
                });
            }
            if let Some(ref project_id) = active_project {
                if task.project_id.as_str() != project_id.as_str() {
                    return Err(KernelError::WorkspaceIntentValidation {
                        message: "active task does not belong to the active project".into(),
                    });
                }
            }
        }

        context.active_project_id = active_project;
        context.active_task_id = active_task;
        context.updated_at = Utc::now().to_rfc3339();
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            WorkspaceIntentRepository::new(&guard).upsert_workflow_context(&context)?;
        }
        Self::audit_context_updated(db, actor, &context)?;
        Ok(context)
    }

    fn audit_intent(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event_type: &str,
        entity_id: &str,
        entity_kind: &str,
    ) -> Result<()> {
        let metadata = json!({
            "entity_id": entity_id,
            "entity_kind": entity_kind,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event_type,
            true,
            metadata,
        )
    }

    fn audit_context_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        context: &WorkflowContext,
    ) -> Result<()> {
        let metadata = json!({
            "workspace_id": context.workspace_id.as_str(),
            "active_project_id": context.active_project_id.as_ref().map(|id| id.to_string()),
            "active_task_id": context.active_task_id.as_ref().map(|id| id.to_string()),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.context.updated",
            true,
            metadata,
        )
    }
}
