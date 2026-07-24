use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    TaskMetadata, TaskRelationship, TaskRelationshipKind, WorkspaceTask, WorkspaceTaskPriority,
    WorkspaceTaskStatus,
};

/// Persistence for Workspace Task Graph nodes and relationships.
pub struct TaskGraphRepository<'a> {
    db: &'a Database,
}

impl<'a> TaskGraphRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_task(&self, task: &WorkspaceTask) -> Result<()> {
        let metadata_json = serde_json::to_string(&task.metadata).unwrap_or_else(|_| "{}".into());
        self.db.connection().execute(
            "INSERT INTO workspace_task_nodes (
                id, workspace_id, project_id, title, status, priority, metadata_json,
                source_intent_task_id, work_goal_id, progress_percent, explanation,
                created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(id) DO UPDATE SET
                project_id = excluded.project_id,
                title = excluded.title,
                status = excluded.status,
                priority = excluded.priority,
                metadata_json = excluded.metadata_json,
                source_intent_task_id = excluded.source_intent_task_id,
                work_goal_id = excluded.work_goal_id,
                progress_percent = excluded.progress_percent,
                explanation = excluded.explanation,
                updated_at = excluded.updated_at",
            (
                task.id.as_str(),
                task.workspace_id.as_str(),
                task.project_id.as_deref(),
                &task.title,
                task.status.as_str(),
                task.priority.as_str(),
                metadata_json,
                task.source_intent_task_id.as_deref(),
                task.work_goal_id.as_deref(),
                i64::from(task.progress_percent),
                &task.explanation,
                &task.created_at,
                &task.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn get_task(&self, id: &str) -> Result<Option<WorkspaceTask>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, title, status, priority, metadata_json,
                    source_intent_task_id, work_goal_id, progress_percent, explanation,
                    created_at, updated_at
             FROM workspace_task_nodes WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_task(row)?));
        }
        Ok(None)
    }

    pub fn list_tasks(&self, workspace_id: &str) -> Result<Vec<WorkspaceTask>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, title, status, priority, metadata_json,
                    source_intent_task_id, work_goal_id, progress_percent, explanation,
                    created_at, updated_at
             FROM workspace_task_nodes
             WHERE workspace_id = ?1
             ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], map_task)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn find_by_intent_task(
        &self,
        workspace_id: &str,
        intent_task_id: &str,
    ) -> Result<Option<WorkspaceTask>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, project_id, title, status, priority, metadata_json,
                    source_intent_task_id, work_goal_id, progress_percent, explanation,
                    created_at, updated_at
             FROM workspace_task_nodes
             WHERE workspace_id = ?1 AND source_intent_task_id = ?2
             LIMIT 1",
        )?;
        let mut rows = stmt.query((workspace_id, intent_task_id))?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_task(row)?));
        }
        Ok(None)
    }

    pub fn insert_relationship(&self, rel: &TaskRelationship) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO workspace_task_relationships (
                id, workspace_id, from_task_id, to_task_id, kind, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(workspace_id, from_task_id, to_task_id, kind) DO NOTHING",
            (
                &rel.id,
                &rel.workspace_id,
                &rel.from_task_id,
                &rel.to_task_id,
                rel.kind.as_str(),
                &rel.created_at,
            ),
        )?;
        Ok(())
    }

    pub fn delete_relationship(&self, id: &str) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM workspace_task_relationships WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    pub fn list_relationships(&self, workspace_id: &str) -> Result<Vec<TaskRelationship>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, from_task_id, to_task_id, kind, created_at
             FROM workspace_task_relationships
             WHERE workspace_id = ?1",
        )?;
        let rows = stmt.query_map([workspace_id], map_relationship)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceTask> {
    let status_raw: String = row.get(4)?;
    let priority_raw: String = row.get(5)?;
    let metadata_raw: Option<String> = row.get(6)?;
    let status = WorkspaceTaskStatus::parse(&status_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(4, "status".into(), rusqlite::types::Type::Text)
    })?;
    let priority = WorkspaceTaskPriority::parse(&priority_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(5, "priority".into(), rusqlite::types::Type::Text)
    })?;
    let metadata = metadata_raw
        .and_then(|raw| serde_json::from_str::<TaskMetadata>(&raw).ok())
        .unwrap_or_default();
    let progress: i64 = row.get(9)?;
    Ok(WorkspaceTask {
        id: workspace_domain::WorkspaceTaskId::new(row.get::<_, String>(0)?)
            .map_err(|_| rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text))?,
        workspace_id: workspace_domain::WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(1, "workspace_id".into(), rusqlite::types::Type::Text)
            })?,
        project_id: row.get(2)?,
        title: row.get(3)?,
        status,
        priority,
        metadata,
        source_intent_task_id: row.get(7)?,
        work_goal_id: row.get(8)?,
        progress_percent: progress.clamp(0, 100) as u8,
        explanation: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
        authority_effect: WorkspaceTask::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_relationship(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskRelationship> {
    let kind_raw: String = row.get(4)?;
    let kind = TaskRelationshipKind::parse(&kind_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(4, "kind".into(), rusqlite::types::Type::Text)
    })?;
    Ok(TaskRelationship {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        from_task_id: row.get(2)?,
        to_task_id: row.get(3)?,
        kind,
        created_at: row.get(5)?,
        authority_effect: WorkspaceTask::AUTHORITY_EFFECT_NONE.into(),
    })
}
