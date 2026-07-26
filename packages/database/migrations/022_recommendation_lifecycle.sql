-- Recommendation Engine lifecycle overlay (Sprint 192).
-- Presentation / human-decision lifecycle only — never duplicates recommendation payloads.
-- Distinct from decision_engine_lifecycle and decision_item_lifecycle.

CREATE TABLE IF NOT EXISTS recommendation_lifecycle (
    workspace_id TEXT NOT NULL,
    native_id TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL,
    created_at TEXT NOT NULL,
    presented_at TEXT,
    resolved_at TEXT,
    resolution_type TEXT,
    actor_id TEXT,
    outcome_json TEXT,
    updated_at TEXT NOT NULL,
    authority_effect TEXT NOT NULL DEFAULT 'none',
    PRIMARY KEY (workspace_id, native_id)
);

CREATE INDEX IF NOT EXISTS idx_recommendation_lifecycle_workspace
    ON recommendation_lifecycle (workspace_id, updated_at DESC);
