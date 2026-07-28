-- Unified Workspace State Envelope (Programme III Batch 1).
-- Read-model persistence only. Composition envelope — never a source of truth.
-- No transitions, repair, or authority.

CREATE TABLE IF NOT EXISTS workspace_state_envelope_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    revision TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    freshness TEXT NOT NULL CHECK (freshness IN ('fresh', 'stale', 'unavailable', 'unknown')),
    completeness TEXT NOT NULL CHECK (completeness IN ('complete', 'partial', 'unknown', 'contradictory')),
    consistency TEXT NOT NULL CHECK (consistency IN ('consistent', 'contradictory', 'unknown')),
    composition_status TEXT NOT NULL,
    source_count INTEGER NOT NULL DEFAULT 0,
    conflict_count INTEGER NOT NULL DEFAULT 0,
    unknown_count INTEGER NOT NULL DEFAULT 0,
    sources_json TEXT NOT NULL DEFAULT '[]',
    contradictions_json TEXT NOT NULL DEFAULT '[]',
    unknowns_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_state_envelope_snapshots_workspace
    ON workspace_state_envelope_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_state_envelope_snapshots_status
    ON workspace_state_envelope_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_state_envelope_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    state_id TEXT NOT NULL,
    revision TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    freshness TEXT NOT NULL,
    completeness TEXT NOT NULL,
    consistency TEXT NOT NULL,
    composition_status TEXT NOT NULL,
    source_count INTEGER NOT NULL DEFAULT 0,
    conflict_count INTEGER NOT NULL DEFAULT 0,
    unknown_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_state_envelope_history_workspace
    ON workspace_state_envelope_history (workspace_id, recorded_at DESC);
