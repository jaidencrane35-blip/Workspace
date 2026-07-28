-- Cross-Workspace Intelligence (Programme III Batch 10).
-- Aggregate understanding. Never centralise authority.

CREATE TABLE IF NOT EXISTS workspace_cross_intelligence_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    scope_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL CHECK (
        completeness IN ('complete', 'partial', 'unknown', 'contradictory', 'unavailable')
    ),
    pattern_count INTEGER NOT NULL DEFAULT 0,
    theme_count INTEGER NOT NULL DEFAULT 0,
    risk_signal_count INTEGER NOT NULL DEFAULT 0,
    constraint_pattern_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    workspace_count INTEGER NOT NULL DEFAULT 0,
    summary TEXT NOT NULL DEFAULT '',
    narrative TEXT NOT NULL DEFAULT '',
    participating_workspaces_json TEXT NOT NULL DEFAULT '[]',
    source_revisions_json TEXT NOT NULL DEFAULT '[]',
    patterns_json TEXT NOT NULL DEFAULT '[]',
    themes_json TEXT NOT NULL DEFAULT '[]',
    risk_signals_json TEXT NOT NULL DEFAULT '[]',
    constraint_patterns_json TEXT NOT NULL DEFAULT '[]',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    assessment_json TEXT NOT NULL DEFAULT '{}',
    provenance_links_json TEXT NOT NULL DEFAULT '[]',
    limitations_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_cross_intelligence_snapshots_scope
    ON workspace_cross_intelligence_snapshots (scope_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_cross_intelligence_snapshots_status
    ON workspace_cross_intelligence_snapshots (scope_id, status);

CREATE TABLE IF NOT EXISTS workspace_cross_intelligence_history (
    id TEXT PRIMARY KEY NOT NULL,
    scope_id TEXT NOT NULL,
    intelligence_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL,
    pattern_count INTEGER NOT NULL DEFAULT 0,
    theme_count INTEGER NOT NULL DEFAULT 0,
    risk_signal_count INTEGER NOT NULL DEFAULT 0,
    constraint_pattern_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    workspace_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_cross_intelligence_history_scope
    ON workspace_cross_intelligence_history (scope_id, recorded_at DESC);
