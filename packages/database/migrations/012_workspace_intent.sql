-- Workspace Intent Model (Phase 4 Batch 5).
-- Durable work context: projects, tasks, goals, workflow context.
-- Permission-neutral and non-executable — never authority.

CREATE TABLE IF NOT EXISTS work_projects (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_work_projects_workspace
    ON work_projects (workspace_id, deleted);

CREATE TABLE IF NOT EXISTS work_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_work_tasks_project
    ON work_tasks (project_id, deleted);

CREATE INDEX IF NOT EXISTS idx_work_tasks_workspace
    ON work_tasks (workspace_id, deleted);

CREATE TABLE IF NOT EXISTS work_goals (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    project_id TEXT,
    task_id TEXT,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_work_goals_workspace
    ON work_goals (workspace_id, deleted);

CREATE TABLE IF NOT EXISTS workflow_contexts (
    workspace_id TEXT PRIMARY KEY NOT NULL,
    active_project_id TEXT,
    active_task_id TEXT,
    related_plan_ids TEXT NOT NULL DEFAULT '[]',
    pending_decision_notes TEXT NOT NULL DEFAULT '[]',
    blocker_notes TEXT NOT NULL DEFAULT '[]',
    updated_at TEXT NOT NULL
);
