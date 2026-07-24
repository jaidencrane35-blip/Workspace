-- Governed Trigger Evaluation (Phase 4 Batch 7).
-- Informational trigger events + intent proposals.
-- No workers, schedulers, or automatic execution.

CREATE TABLE IF NOT EXISTS trigger_events (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    context TEXT NOT NULL,
    project_id TEXT,
    task_id TEXT,
    actor_id TEXT NOT NULL,
    actor_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    authority_effect TEXT NOT NULL DEFAULT 'none'
);

CREATE INDEX IF NOT EXISTS idx_trigger_events_workspace
    ON trigger_events (workspace_id, created_at DESC);

CREATE TABLE IF NOT EXISTS automation_intent_proposals (
    id TEXT PRIMARY KEY NOT NULL,
    contract_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    task_id TEXT,
    trigger_event_id TEXT NOT NULL,
    intent_statement TEXT NOT NULL,
    required_capabilities TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL,
    explanation TEXT NOT NULL,
    definition_fingerprint TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_automation_intent_proposals_workspace
    ON automation_intent_proposals (workspace_id, status, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_automation_intent_proposals_event
    ON automation_intent_proposals (trigger_event_id);

CREATE TABLE IF NOT EXISTS trigger_evaluation_rejections (
    id TEXT PRIMARY KEY NOT NULL,
    trigger_event_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    contract_id TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    reason TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_trigger_evaluation_rejections_workspace
    ON trigger_evaluation_rejections (workspace_id, created_at DESC);
