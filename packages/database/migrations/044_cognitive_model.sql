-- Workspace Cognitive Model (Programme II Batch 1).
-- Durable semantic nodes + relations. Never executes. Goal nodes reference WorkGoals.

CREATE TABLE IF NOT EXISTS cognitive_nodes (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN (
        'goal','objective','initiative','milestone','context',
        'working_set','constraint','risk','opportunity'
    )),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('active','paused','completed','abandoned')),
    importance INTEGER NOT NULL CHECK (importance >= 0 AND importance <= 100),
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    is_current_focus INTEGER NOT NULL DEFAULT 0 CHECK (is_current_focus IN (0, 1)),
    external_ref TEXT,
    parent_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    CHECK (
        kind != 'goal'
        OR (external_ref IS NOT NULL AND external_ref LIKE 'work_goal:%')
    )
);

CREATE INDEX IF NOT EXISTS idx_cognitive_nodes_workspace
    ON cognitive_nodes (workspace_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_cognitive_nodes_kind
    ON cognitive_nodes (workspace_id, kind);

CREATE TABLE IF NOT EXISTS cognitive_relations (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN (
        'depends_on','blocks','supports','constrains',
        'mitigates','enables','part_of','focuses'
    )),
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    explanation TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    CHECK (from_id != to_id),
    FOREIGN KEY (from_id) REFERENCES cognitive_nodes(id) ON DELETE CASCADE,
    FOREIGN KEY (to_id) REFERENCES cognitive_nodes(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_cognitive_relations_workspace
    ON cognitive_relations (workspace_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_cognitive_relations_from
    ON cognitive_relations (from_id);

CREATE INDEX IF NOT EXISTS idx_cognitive_relations_to
    ON cognitive_relations (to_id);
