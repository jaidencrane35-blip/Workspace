-- Workspace Cognitive Orchestration (Programme II Batch 5).
-- Coordination layer for refresh ordering / staleness. Never executes.

CREATE TABLE IF NOT EXISTS workspace_orchestration_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    current_generation INTEGER NOT NULL DEFAULT 1,
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    rationale TEXT NOT NULL DEFAULT '',
    refresh_plan_json TEXT NOT NULL DEFAULT '[]',
    dependency_order_json TEXT NOT NULL DEFAULT '[]',
    dependencies_json TEXT NOT NULL DEFAULT '[]',
    blocked_items_json TEXT NOT NULL DEFAULT '[]',
    stale_items_json TEXT NOT NULL DEFAULT '[]',
    skipped_items_json TEXT NOT NULL DEFAULT '[]',
    cycles_json TEXT NOT NULL DEFAULT '[]',
    evidence_links_json TEXT NOT NULL DEFAULT '[]',
    graph_links_json TEXT NOT NULL DEFAULT '[]',
    planning_links_json TEXT NOT NULL DEFAULT '[]',
    reasoning_links_json TEXT NOT NULL DEFAULT '[]',
    execution_links_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_orchestration_snapshots_workspace
    ON workspace_orchestration_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_orchestration_snapshots_status
    ON workspace_orchestration_snapshots (workspace_id, status);

-- Append-only terminal evidence. Snapshot cascade does not erase history rows.
CREATE TABLE IF NOT EXISTS workspace_orchestration_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    orchestration_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    current_generation INTEGER NOT NULL DEFAULT 1,
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    rationale_excerpt TEXT NOT NULL DEFAULT '',
    stage_count INTEGER NOT NULL DEFAULT 0,
    cycle_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_orchestration_history_workspace
    ON workspace_orchestration_history (workspace_id, recorded_at DESC);
