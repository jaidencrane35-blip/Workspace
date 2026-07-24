-- Workspace Task Graph (Phase 5 Batch 2).
-- Persistent work model: nodes + relationships. Never authority.

CREATE TABLE IF NOT EXISTS workspace_task_nodes (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    project_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    metadata_json TEXT,
    source_intent_task_id TEXT,
    work_goal_id TEXT,
    progress_percent INTEGER NOT NULL DEFAULT 0,
    explanation TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_task_nodes_workspace
    ON workspace_task_nodes (workspace_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_task_nodes_intent
    ON workspace_task_nodes (workspace_id, source_intent_task_id);

CREATE TABLE IF NOT EXISTS workspace_task_relationships (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    from_task_id TEXT NOT NULL,
    to_task_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE (workspace_id, from_task_id, to_task_id, kind)
);

CREATE INDEX IF NOT EXISTS idx_workspace_task_relationships_workspace
    ON workspace_task_relationships (workspace_id);

CREATE INDEX IF NOT EXISTS idx_workspace_task_relationships_from
    ON workspace_task_relationships (from_task_id);

CREATE INDEX IF NOT EXISTS idx_workspace_task_relationships_to
    ON workspace_task_relationships (to_task_id);
