-- Decision Candidate evaluation origin contract (DE-owned).
-- Persists only evaluated acknowledgments — not scores, ranks, or planner handoffs.
-- Never mutates recommendation_lifecycle.

CREATE TABLE IF NOT EXISTS decision_candidate_evaluation_origin (
    workspace_id TEXT NOT NULL,
    evaluation_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    evaluation_state TEXT NOT NULL,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    evaluated_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, evaluation_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_evaluation_origin_candidate
    ON decision_candidate_evaluation_origin (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_evaluation_origin_workspace
    ON decision_candidate_evaluation_origin (workspace_id, evaluated_at DESC);
