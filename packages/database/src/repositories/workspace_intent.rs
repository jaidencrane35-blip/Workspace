use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    Project, ProjectId, ProjectStatus, Task, TaskId, TaskPriority, TaskStatus, WorkGoal,
    WorkGoalId, WorkGoalStatus, WorkflowContext, WorkspaceId,
};

/// Persistence for durable workspace intent (projects, tasks, goals, context).
pub struct WorkspaceIntentRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceIntentRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_project(&self, project: &Project) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO work_projects (
                id, workspace_id, name, description, status, metadata,
                created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                metadata = excluded.metadata,
                updated_at = excluded.updated_at,
                deleted = excluded.deleted",
            (
                project.id.as_str(),
                project.workspace_id.as_str(),
                &project.name,
                &project.description,
                project.status.as_str(),
                &project.metadata,
                &project.created_at,
                &project.updated_at,
                if project.deleted { 1_i64 } else { 0 },
            ),
        )?;
        Ok(())
    }

    pub fn get_project(&self, id: &ProjectId) -> Result<Option<Project>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, metadata,
                    created_at, updated_at, deleted
             FROM work_projects WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_project(row)?));
        }
        Ok(None)
    }

    pub fn list_projects(&self, workspace_id: &str, limit: usize) -> Result<Vec<Project>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, metadata,
                    created_at, updated_at, deleted
             FROM work_projects
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_project)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn upsert_task(&self, task: &Task) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO work_tasks (
                id, project_id, workspace_id, title, status, priority,
                created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                status = excluded.status,
                priority = excluded.priority,
                updated_at = excluded.updated_at,
                deleted = excluded.deleted",
            (
                task.id.as_str(),
                task.project_id.as_str(),
                task.workspace_id.as_str(),
                &task.title,
                task.status.as_str(),
                task.priority.as_str(),
                &task.created_at,
                &task.updated_at,
                if task.deleted { 1_i64 } else { 0 },
            ),
        )?;
        Ok(())
    }

    pub fn get_task(&self, id: &TaskId) -> Result<Option<Task>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, project_id, workspace_id, title, status, priority,
                    created_at, updated_at, deleted
             FROM work_tasks WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_task(row)?));
        }
        Ok(None)
    }

    pub fn list_tasks(
        &self,
        workspace_id: &str,
        project_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Task>> {
        let limit = limit.clamp(1, 200) as i64;
        if let Some(project_id) = project_id {
            let mut stmt = self.db.connection().prepare(
                "SELECT id, project_id, workspace_id, title, status, priority,
                        created_at, updated_at, deleted
                 FROM work_tasks
                 WHERE workspace_id = ?1 AND project_id = ?2 AND deleted = 0
                 ORDER BY updated_at DESC
                 LIMIT ?3",
            )?;
            let rows = stmt.query_map((workspace_id, project_id, limit), map_task)?;
            return rows
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into);
        }
        let mut stmt = self.db.connection().prepare(
            "SELECT id, project_id, workspace_id, title, status, priority,
                    created_at, updated_at, deleted
             FROM work_tasks
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_task)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn upsert_goal(&self, goal: &WorkGoal) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO work_goals (
                id, workspace_id, project_id, task_id, description, status,
                created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                project_id = excluded.project_id,
                task_id = excluded.task_id,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at,
                deleted = excluded.deleted",
            (
                goal.id.as_str(),
                goal.workspace_id.as_str(),
                goal.project_id.as_ref().map(|id| id.as_str().to_string()),
                goal.task_id.as_ref().map(|id| id.as_str().to_string()),
                &goal.description,
                goal.status.as_str(),
                &goal.created_at,
                &goal.updated_at,
                if goal.deleted { 1_i64 } else { 0 },
            ),
        )?;
        Ok(())
    }

    pub fn list_goals(&self, workspace_id: &str, limit: usize) -> Result<Vec<WorkGoal>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, task_id, description, status,
                    created_at, updated_at, deleted
             FROM work_goals
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id, limit), map_goal)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn upsert_workflow_context(&self, context: &WorkflowContext) -> Result<()> {
        let related = serde_json::to_string(&context.related_plan_ids).unwrap_or_else(|_| "[]".into());
        let pending =
            serde_json::to_string(&context.pending_decision_notes).unwrap_or_else(|_| "[]".into());
        let blockers =
            serde_json::to_string(&context.blocker_notes).unwrap_or_else(|_| "[]".into());
        self.db.connection().execute(
            "INSERT INTO workflow_contexts (
                workspace_id, active_project_id, active_task_id,
                related_plan_ids, pending_decision_notes, blocker_notes, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(workspace_id) DO UPDATE SET
                active_project_id = excluded.active_project_id,
                active_task_id = excluded.active_task_id,
                related_plan_ids = excluded.related_plan_ids,
                pending_decision_notes = excluded.pending_decision_notes,
                blocker_notes = excluded.blocker_notes,
                updated_at = excluded.updated_at",
            (
                context.workspace_id.as_str(),
                context
                    .active_project_id
                    .as_ref()
                    .map(|id| id.as_str().to_string()),
                context
                    .active_task_id
                    .as_ref()
                    .map(|id| id.as_str().to_string()),
                related,
                pending,
                blockers,
                &context.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_workflow_context(&self, workspace_id: &str) -> Result<Option<WorkflowContext>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT workspace_id, active_project_id, active_task_id,
                    related_plan_ids, pending_decision_notes, blocker_notes, updated_at
             FROM workflow_contexts WHERE workspace_id = ?1",
        )?;
        let mut rows = stmt.query([workspace_id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_workflow_context(row)?));
        }
        Ok(None)
    }
}

fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
    let status = ProjectStatus::parse(row.get::<_, String>(4)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(Project {
        id: ProjectId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        name: row.get(2)?,
        description: row.get(3)?,
        status,
        metadata: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        deleted: row.get::<_, i64>(8)? != 0,
    })
}

fn map_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    let status = TaskStatus::parse(row.get::<_, String>(4)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let priority = TaskPriority::parse(row.get::<_, String>(5)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(Task {
        id: TaskId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        project_id: ProjectId::new(row.get::<_, String>(1)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(2)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        title: row.get(3)?,
        status,
        priority,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        deleted: row.get::<_, i64>(8)? != 0,
    })
}

fn map_goal(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkGoal> {
    let status = WorkGoalStatus::parse(row.get::<_, String>(5)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let project_id = row
        .get::<_, Option<String>>(2)?
        .map(ProjectId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let task_id = row
        .get::<_, Option<String>>(3)?
        .map(TaskId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(WorkGoal {
        id: WorkGoalId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        project_id,
        task_id,
        description: row.get(4)?,
        status,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        deleted: row.get::<_, i64>(8)? != 0,
    })
}

fn map_workflow_context(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkflowContext> {
    let related: Vec<String> =
        serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default();
    let pending: Vec<String> =
        serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default();
    let blockers: Vec<String> =
        serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default();
    let active_project_id = row
        .get::<_, Option<String>>(1)?
        .map(ProjectId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let active_task_id = row
        .get::<_, Option<String>>(2)?
        .map(TaskId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(WorkflowContext {
        workspace_id: WorkspaceId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        active_project_id,
        active_task_id,
        related_plan_ids: related,
        pending_decision_notes: pending,
        blocker_notes: blockers,
        updated_at: row.get(6)?,
    })
}
