-- Workspace Learning & Adaptation (Programme II Batch 6).
-- Meta-evidence / adaptation suggestions only. Never executes or owns foreign truth.

CREATE TABLE IF NOT EXISTS workspace_learning_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    observation_count INTEGER NOT NULL DEFAULT 0,
    pattern_count INTEGER NOT NULL DEFAULT 0,
    adaptation_count INTEGER NOT NULL DEFAULT 0,
    observations_json TEXT NOT NULL DEFAULT '[]',
    patterns_json TEXT NOT NULL DEFAULT '[]',
    success_signals_json TEXT NOT NULL DEFAULT '[]',
    failure_signals_json TEXT NOT NULL DEFAULT '[]',
    confidence_updates_json TEXT NOT NULL DEFAULT '[]',
    adaptation_candidates_json TEXT NOT NULL DEFAULT '[]',
    evidence_links_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_learning_snapshots_workspace
    ON workspace_learning_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_learning_snapshots_status
    ON workspace_learning_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_learning_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    learning_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    observation_count INTEGER NOT NULL DEFAULT 0,
    pattern_count INTEGER NOT NULL DEFAULT 0,
    adaptation_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_learning_history_workspace
    ON workspace_learning_history (workspace_id, recorded_at DESC);
