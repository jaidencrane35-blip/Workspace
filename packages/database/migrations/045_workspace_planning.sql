-- Workspace Planning Engine (Programme II Batch 2).
-- Durable non-executing planning artefacts. Reference-based only.

CREATE TABLE IF NOT EXISTS planning_plans (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    summary TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL CHECK (status IN ('active', 'superseded', 'abandoned')),
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    generated_at TEXT NOT NULL,
    superseded_at TEXT,
    explanation_summary TEXT NOT NULL DEFAULT '',
    explanation_why TEXT NOT NULL DEFAULT '',
    explanation_next TEXT NOT NULL DEFAULT '',
    sections_json TEXT NOT NULL DEFAULT '[]',
    alternatives_json TEXT NOT NULL DEFAULT '[]',
    constraint_refs_json TEXT NOT NULL DEFAULT '[]',
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none')
);

CREATE INDEX IF NOT EXISTS idx_planning_plans_workspace
    ON planning_plans (workspace_id, generated_at DESC);

CREATE INDEX IF NOT EXISTS idx_planning_plans_status
    ON planning_plans (workspace_id, status);

CREATE TABLE IF NOT EXISTS planning_steps (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    title TEXT NOT NULL,
    rationale TEXT NOT NULL DEFAULT '',
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    FOREIGN KEY (plan_id) REFERENCES planning_plans(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_planning_steps_plan
    ON planning_steps (plan_id, ordinal ASC);

CREATE TABLE IF NOT EXISTS planning_assumptions (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    statement TEXT NOT NULL,
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    FOREIGN KEY (plan_id) REFERENCES planning_plans(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_planning_assumptions_plan
    ON planning_assumptions (plan_id);

CREATE TABLE IF NOT EXISTS planning_risks (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    statement TEXT NOT NULL,
    uncertainty INTEGER NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 100),
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    FOREIGN KEY (plan_id) REFERENCES planning_plans(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_planning_risks_plan
    ON planning_risks (plan_id);

CREATE TABLE IF NOT EXISTS planning_gaps (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    statement TEXT NOT NULL,
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    FOREIGN KEY (plan_id) REFERENCES planning_plans(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_planning_gaps_plan
    ON planning_gaps (plan_id);

CREATE TABLE IF NOT EXISTS planning_dependencies (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    from_step_id TEXT NOT NULL,
    to_step_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    CHECK (from_step_id != to_step_id),
    FOREIGN KEY (plan_id) REFERENCES planning_plans(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_planning_dependencies_plan
    ON planning_dependencies (plan_id);
