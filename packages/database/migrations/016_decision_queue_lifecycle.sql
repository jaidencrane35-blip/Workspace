-- Governed Decision Queue lifecycle overlay (Phase 4 Batch 8).
-- Stores presentation lifecycle only — never duplicates source payloads.
-- Sources of truth remain approvals, proposals, plans, and workflows.

CREATE TABLE IF NOT EXISTS decision_item_lifecycle (
    workspace_id TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    decision_state TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, source_type, source_id)
);

CREATE INDEX IF NOT EXISTS idx_decision_item_lifecycle_workspace
    ON decision_item_lifecycle (workspace_id, updated_at DESC);
