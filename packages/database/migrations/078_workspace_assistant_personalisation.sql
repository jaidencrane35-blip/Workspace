-- Workspace Assistant Personalisation Boundary (Programme IV Batch 16).
-- Dual-channel non-actionable personalisation packaging evidence.
-- Adapt presentation from explicit preferences. Never invent who the user is.
-- Personalisation-specific columns (request, preferences/items, adaptation, lineage, diagnostics, scope)
-- — NOT a clone of user_preferences / hidden profile DB.

CREATE TABLE IF NOT EXISTS workspace_assistant_personalisation_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    item_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    narrative_summary TEXT NOT NULL DEFAULT '',
    narrative TEXT NOT NULL DEFAULT '',
    request_json TEXT NOT NULL DEFAULT '{}',
    preferences_json TEXT NOT NULL DEFAULT '[]',
    items_json TEXT NOT NULL DEFAULT '[]',
    adaptation_json TEXT NOT NULL DEFAULT '{}',
    lineage_json TEXT NOT NULL DEFAULT '{}',
    gaps_json TEXT NOT NULL DEFAULT '[]',
    diagnostics_json TEXT NOT NULL DEFAULT '{}',
    scope_json TEXT NOT NULL DEFAULT '{}',
    limitations_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_assistant_personalisation_snapshots_workspace
    ON workspace_assistant_personalisation_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_assistant_personalisation_snapshots_status
    ON workspace_assistant_personalisation_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_assistant_personalisation_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    personalisation_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    item_count INTEGER NOT NULL DEFAULT 0,
    gap_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_assistant_personalisation_history_workspace
    ON workspace_assistant_personalisation_history (workspace_id, recorded_at DESC);
