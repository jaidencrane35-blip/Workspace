-- Workspace Cognitive Agent Cast (Programme II Batch 7).
-- Role representations / perspectives / critiques / syntheses. Never executes.

CREATE TABLE IF NOT EXISTS workspace_cognitive_agent_cast_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    agent_count INTEGER NOT NULL DEFAULT 0,
    perspective_count INTEGER NOT NULL DEFAULT 0,
    critique_count INTEGER NOT NULL DEFAULT 0,
    synthesis_count INTEGER NOT NULL DEFAULT 0,
    agents_json TEXT NOT NULL DEFAULT '[]',
    roles_json TEXT NOT NULL DEFAULT '[]',
    perspectives_json TEXT NOT NULL DEFAULT '[]',
    critiques_json TEXT NOT NULL DEFAULT '[]',
    syntheses_json TEXT NOT NULL DEFAULT '[]',
    evidence_links_json TEXT NOT NULL DEFAULT '[]',
    source_references_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_agent_cast_snapshots_workspace
    ON workspace_cognitive_agent_cast_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_agent_cast_snapshots_status
    ON workspace_cognitive_agent_cast_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_cognitive_agent_cast_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    cast_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    agent_count INTEGER NOT NULL DEFAULT 0,
    perspective_count INTEGER NOT NULL DEFAULT 0,
    critique_count INTEGER NOT NULL DEFAULT 0,
    synthesis_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_cognitive_agent_cast_history_workspace
    ON workspace_cognitive_agent_cast_history (workspace_id, recorded_at DESC);
