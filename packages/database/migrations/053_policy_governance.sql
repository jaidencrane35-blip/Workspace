-- Policy & Governance Engine (Programme III Batch 2).
-- Policy explains authority. It does not become authority.
-- Persistence of evaluation evidence only — no grants, no execution.

CREATE TABLE IF NOT EXISTS workspace_policy_definitions (
    policy_id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    scope TEXT NOT NULL CHECK (scope IN ('workspace', 'project', 'capability', 'command', 'resource', 'actor_context')),
    version TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('critical', 'high', 'medium', 'low')),
    status TEXT NOT NULL CHECK (status IN ('draft', 'active', 'deprecated', 'retired')),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE TABLE IF NOT EXISTS workspace_policy_governance_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    context_revision TEXT,
    policy_catalog_revision TEXT NOT NULL,
    evaluation_count INTEGER NOT NULL DEFAULT 0,
    aggregate_result TEXT NOT NULL,
    policies_json TEXT NOT NULL DEFAULT '[]',
    evaluations_json TEXT NOT NULL DEFAULT '[]',
    recommendation_json TEXT,
    unknowns_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    terminal INTEGER NOT NULL DEFAULT 0 CHECK (terminal IN (0, 1)),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0)
);

CREATE INDEX IF NOT EXISTS idx_workspace_policy_governance_snapshots_workspace
    ON workspace_policy_governance_snapshots (workspace_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_workspace_policy_governance_snapshots_status
    ON workspace_policy_governance_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS workspace_policy_governance_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    evaluation_set_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    created_at TEXT NOT NULL,
    superseded_at TEXT,
    context_revision TEXT,
    policy_catalog_revision TEXT NOT NULL,
    evaluation_count INTEGER NOT NULL DEFAULT 0,
    aggregate_result TEXT NOT NULL,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_workspace_policy_governance_history_workspace
    ON workspace_policy_governance_history (workspace_id, recorded_at DESC);
