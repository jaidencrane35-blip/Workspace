-- Historical Workspace Reconstruction (Programme III Batch 3).
-- Reconstruction explains change. It does not become the source of truth.
-- Persistence of reconstruction artefacts only — no authoritative state, replay, or lifecycle.

CREATE TABLE IF NOT EXISTS workspace_historical_reconstruction_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    from_revision TEXT,
    to_revision TEXT,
    completeness TEXT NOT NULL CHECK (
        completeness IN ('complete', 'partial', 'unknown', 'contradictory', 'unavailable')
    ),
    change_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    comparison_count INTEGER NOT NULL DEFAULT 0,
    explanation TEXT NOT NULL DEFAULT '',
    temporal_snapshots_json TEXT NOT NULL DEFAULT '[]',
    timeline_json TEXT NOT NULL DEFAULT '[]',
    comparisons_json TEXT NOT NULL DEFAULT '[]',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    provenance_links_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_historical_reconstruction_snapshots_workspace
    ON workspace_historical_reconstruction_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_historical_reconstruction_snapshots_status
    ON workspace_historical_reconstruction_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_historical_reconstruction_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    reconstruction_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    from_revision TEXT,
    to_revision TEXT,
    completeness TEXT NOT NULL,
    change_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    comparison_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_historical_reconstruction_history_workspace
    ON workspace_historical_reconstruction_history (workspace_id, recorded_at DESC);
