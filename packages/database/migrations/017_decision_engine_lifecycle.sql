-- Decision Engine lifecycle overlay (Phase 5).
-- Presentation lifecycle only — never duplicates recommendation payloads.
-- Distinct from decision_item_lifecycle (Decision Queue inbox).

CREATE TABLE IF NOT EXISTS decision_engine_lifecycle (
    workspace_id TEXT NOT NULL,
    candidate_key TEXT NOT NULL,
    outcome TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, candidate_key)
);

CREATE INDEX IF NOT EXISTS idx_decision_engine_lifecycle_workspace
    ON decision_engine_lifecycle (workspace_id, updated_at DESC);
