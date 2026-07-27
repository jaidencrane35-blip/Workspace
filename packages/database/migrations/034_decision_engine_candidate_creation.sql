-- Decision Engine candidate creation (DE-owned native DecisionCandidate provenance).
-- Persists created records only. Never mutates recommendation_lifecycle.
-- Does not store scores, planner handoffs, goals, or intents.

CREATE TABLE IF NOT EXISTS decision_engine_candidate_creation (
    workspace_id TEXT NOT NULL,
    creation_id TEXT NOT NULL,
    intake_candidate_id TEXT NOT NULL,
    creation_request_id TEXT NOT NULL,
    recommendation_reference TEXT NOT NULL,
    package_seal_digest TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    title TEXT NOT NULL,
    goal_statement TEXT NOT NULL,
    created_at TEXT NOT NULL,
    creation_state TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, creation_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_engine_candidate_creation_intake
    ON decision_engine_candidate_creation (workspace_id, intake_candidate_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_engine_candidate_creation_candidate
    ON decision_engine_candidate_creation (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_engine_candidate_creation_workspace
    ON decision_engine_candidate_creation (workspace_id, created_at DESC);
