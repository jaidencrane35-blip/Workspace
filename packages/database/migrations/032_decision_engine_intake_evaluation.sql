-- Decision Engine intake evaluation (DE-owned examination record).
-- Distinct from DecisionCandidate scoring and recommendation_lifecycle.
-- Never grants planning authority or mutates Recommendation Engine records.

CREATE TABLE IF NOT EXISTS decision_engine_intake_evaluation (
    workspace_id TEXT NOT NULL,
    evaluation_id TEXT NOT NULL,
    intake_candidate_id TEXT NOT NULL,
    evaluated_at TEXT NOT NULL,
    evaluation_state TEXT NOT NULL,
    evaluation_reason TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, evaluation_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_engine_intake_evaluation_candidate
    ON decision_engine_intake_evaluation (workspace_id, intake_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_engine_intake_evaluation_workspace
    ON decision_engine_intake_evaluation (workspace_id, evaluated_at DESC);
