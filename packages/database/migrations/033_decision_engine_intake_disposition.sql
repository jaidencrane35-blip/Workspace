-- Decision Engine intake disposition (DE-owned lifecycle decision).
-- Distinct from DecisionCandidate outcomes and recommendation_lifecycle.
-- Never grants planning/execution authority or mutates Recommendation Engine records.

CREATE TABLE IF NOT EXISTS decision_engine_intake_disposition (
    workspace_id TEXT NOT NULL,
    disposition_id TEXT NOT NULL,
    intake_candidate_id TEXT NOT NULL,
    evaluation_id TEXT NOT NULL,
    disposed_at TEXT NOT NULL,
    disposition_state TEXT NOT NULL,
    disposition_reason TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, disposition_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_engine_intake_disposition_candidate
    ON decision_engine_intake_disposition (workspace_id, intake_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_engine_intake_disposition_workspace
    ON decision_engine_intake_disposition (workspace_id, disposed_at DESC);
