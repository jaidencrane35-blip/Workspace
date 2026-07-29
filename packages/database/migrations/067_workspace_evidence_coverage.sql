-- Workspace Evidence Coverage Engine (Programme IV Batch 4).
-- Measure evidence coverage. Never measure truth.
-- coverage ≠ correctness; completeness ≠ confidence; gap ≠ recommendation.

CREATE TABLE IF NOT EXISTS workspace_evidence_coverage_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL CHECK (
        completeness IN ('complete', 'partial', 'unknown', 'contradictory', 'unavailable')
    ),
    observed_sources INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    narrative_summary TEXT NOT NULL DEFAULT '',
    narrative TEXT NOT NULL DEFAULT '',
    scope_json TEXT NOT NULL DEFAULT '{}',
    assessment_json TEXT NOT NULL DEFAULT '{}',
    metrics_json TEXT NOT NULL DEFAULT '{}',
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

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_coverage_snapshots_workspace
    ON workspace_evidence_coverage_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_coverage_snapshots_status
    ON workspace_evidence_coverage_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_evidence_coverage_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    coverage_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    completeness TEXT NOT NULL,
    observed_sources INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    source_revision_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_evidence_coverage_history_workspace
    ON workspace_evidence_coverage_history (workspace_id, recorded_at DESC);
