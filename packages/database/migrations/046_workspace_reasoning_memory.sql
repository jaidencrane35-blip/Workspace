-- Workspace Reasoning Memory (Programme II Batch 3).
-- Durable reasoning evidence only. Reference IDs. Append-only history. No execution.

CREATE TABLE IF NOT EXISTS reasoning_records (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'current', 'superseded', 'archived')),
    hypothesis TEXT NOT NULL DEFAULT '',
    assumptions_json TEXT NOT NULL DEFAULT '[]',
    alternatives_json TEXT NOT NULL DEFAULT '[]',
    rejected_alternatives_json TEXT NOT NULL DEFAULT '[]',
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    links_json TEXT NOT NULL DEFAULT '[]',
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    confidence_evolution_json TEXT NOT NULL DEFAULT '[]',
    uncertainty_evolution_json TEXT NOT NULL DEFAULT '[]',
    reflection TEXT NOT NULL DEFAULT '',
    lessons_json TEXT NOT NULL DEFAULT '[]',
    rationale TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    superseded_at TEXT,
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none')
);

CREATE INDEX IF NOT EXISTS idx_reasoning_records_workspace
    ON reasoning_records (workspace_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_reasoning_records_status
    ON reasoning_records (workspace_id, status);

-- Append-only terminal evidence channel. No UPDATE/DELETE of retained rows by contract.
CREATE TABLE IF NOT EXISTS reasoning_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    record_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    reflection_excerpt TEXT NOT NULL DEFAULT '',
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL,
    FOREIGN KEY (record_id) REFERENCES reasoning_records(id)
);

CREATE INDEX IF NOT EXISTS idx_reasoning_history_workspace
    ON reasoning_history (workspace_id, recorded_at DESC);

CREATE INDEX IF NOT EXISTS idx_reasoning_history_record
    ON reasoning_history (record_id);
