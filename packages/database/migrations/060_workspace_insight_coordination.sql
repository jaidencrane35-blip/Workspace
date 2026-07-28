-- Workspace Insight Coordination (Programme III Batch 9).
-- Coordinate understanding. Never create authority.
-- prioritisation ≠ recommendation; intersection ≠ causation; confidence ≠ permission.

CREATE TABLE IF NOT EXISTS workspace_insight_coordination_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL CHECK (
        completeness IN ('complete', 'partial', 'unknown', 'contradictory', 'unavailable')
    ),
    cluster_count INTEGER NOT NULL DEFAULT 0,
    intersection_count INTEGER NOT NULL DEFAULT 0,
    attention_signal_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    summary TEXT NOT NULL DEFAULT '',
    narrative TEXT NOT NULL DEFAULT '',
    frame_json TEXT NOT NULL DEFAULT '{}',
    source_revisions_json TEXT NOT NULL DEFAULT '[]',
    clusters_json TEXT NOT NULL DEFAULT '[]',
    intersections_json TEXT NOT NULL DEFAULT '[]',
    attention_signals_json TEXT NOT NULL DEFAULT '[]',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    assessment_json TEXT NOT NULL DEFAULT '{}',
    provenance_links_json TEXT NOT NULL DEFAULT '[]',
    limitations_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_insight_coordination_snapshots_workspace
    ON workspace_insight_coordination_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_insight_coordination_snapshots_status
    ON workspace_insight_coordination_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_insight_coordination_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    coordination_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL,
    cluster_count INTEGER NOT NULL DEFAULT 0,
    intersection_count INTEGER NOT NULL DEFAULT 0,
    attention_signal_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_insight_coordination_history_workspace
    ON workspace_insight_coordination_history (workspace_id, recorded_at DESC);
