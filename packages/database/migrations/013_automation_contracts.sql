-- Governed Automation Contracts (Phase 4 Batch 6).
-- Durable Intent templates only. Non-executable.
-- approval_state is definition lifecycle — NOT a capability grant.
-- No workers, schedulers, or automatic execution in this batch.

CREATE TABLE IF NOT EXISTS automation_contracts (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    task_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    trigger_kind TEXT NOT NULL,
    trigger_definition TEXT NOT NULL,
    intent_statement TEXT NOT NULL,
    scope TEXT NOT NULL,
    required_capabilities TEXT NOT NULL DEFAULT '[]',
    approval_state TEXT NOT NULL,
    created_by_actor TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_automation_contracts_workspace
    ON automation_contracts (workspace_id, deleted);

CREATE INDEX IF NOT EXISTS idx_automation_contracts_project
    ON automation_contracts (project_id, deleted);

CREATE INDEX IF NOT EXISTS idx_automation_contracts_status
    ON automation_contracts (workspace_id, status, approval_state, deleted);
