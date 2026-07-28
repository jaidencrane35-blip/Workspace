-- Temporal Intelligence (Programme III Batch 4).
-- Temporal intelligence organises evidence over time. It does not predict, simulate, correct, or become truth.
-- Persistence of analysis artefacts only — no authoritative timeline, event stream, or lifecycle mirror.

CREATE TABLE IF NOT EXISTS workspace_temporal_intelligence_snapshots (
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
    conflict_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    chain_ref_count INTEGER NOT NULL DEFAULT 0,
    narrative TEXT NOT NULL DEFAULT '',
    window_json TEXT NOT NULL DEFAULT '{}',
    chain_summary_json TEXT NOT NULL DEFAULT '{}',
    conflict_explanations_json TEXT NOT NULL DEFAULT '[]',
    evidence_quality_json TEXT NOT NULL DEFAULT '{}',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    provenance_links_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_temporal_intelligence_snapshots_workspace
    ON workspace_temporal_intelligence_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_temporal_intelligence_snapshots_status
    ON workspace_temporal_intelligence_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_temporal_intelligence_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    analysis_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    from_revision TEXT,
    to_revision TEXT,
    completeness TEXT NOT NULL,
    conflict_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    chain_ref_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_temporal_intelligence_history_workspace
    ON workspace_temporal_intelligence_history (workspace_id, recorded_at DESC);
