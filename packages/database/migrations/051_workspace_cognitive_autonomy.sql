-- Workspace Cognitive Autonomy (Programme II Batch 8).
-- Governed suggestion layer only. Never executes or self-approves.

CREATE TABLE IF NOT EXISTS workspace_cognitive_autonomy_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    opportunity_count INTEGER NOT NULL DEFAULT 0,
    proposal_count INTEGER NOT NULL DEFAULT 0,
    recommendation_count INTEGER NOT NULL DEFAULT 0,
    opportunities_json TEXT NOT NULL DEFAULT '[]',
    proposals_json TEXT NOT NULL DEFAULT '[]',
    recommendations_json TEXT NOT NULL DEFAULT '[]',
    risk_assessments_json TEXT NOT NULL DEFAULT '[]',
    approval_requirements_json TEXT NOT NULL DEFAULT '[]',
    evidence_links_json TEXT NOT NULL DEFAULT '[]',
    constraints_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_autonomy_snapshots_workspace
    ON workspace_cognitive_autonomy_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_autonomy_snapshots_status
    ON workspace_cognitive_autonomy_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_cognitive_autonomy_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    autonomy_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    opportunity_count INTEGER NOT NULL DEFAULT 0,
    proposal_count INTEGER NOT NULL DEFAULT 0,
    recommendation_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_autonomy_history_workspace
    ON workspace_cognitive_autonomy_history (workspace_id, recorded_at DESC);
