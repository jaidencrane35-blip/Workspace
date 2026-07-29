-- Workspace Evidence Freshness Engine (Programme IV Batch 7).
-- Observe freshness. Never refresh evidence.
-- freshness ≠ validity; observation ≠ regeneration; stale ≠ refresh request.

CREATE TABLE IF NOT EXISTS workspace_evidence_freshness_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL CHECK (
        completeness IN ('complete', 'partial', 'unknown', 'contradictory', 'unavailable')
    ),
    observation_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    narrative_summary TEXT NOT NULL DEFAULT '',
    narrative TEXT NOT NULL DEFAULT '',
    scope_json TEXT NOT NULL DEFAULT '{}',
    assessment_json TEXT NOT NULL DEFAULT '{}',
    observations_json TEXT NOT NULL DEFAULT '[]',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    lineage_json TEXT NOT NULL DEFAULT '{}',
    diagnostics_json TEXT NOT NULL DEFAULT '{}',
    provenance_links_json TEXT NOT NULL DEFAULT '[]',
    source_revisions_json TEXT NOT NULL DEFAULT '[]',
    limitations_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_freshness_snapshots_workspace
    ON workspace_evidence_freshness_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_freshness_snapshots_status
    ON workspace_evidence_freshness_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_evidence_freshness_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    freshness_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL,
    observation_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_freshness_history_workspace
    ON workspace_evidence_freshness_history (workspace_id, recorded_at DESC);
