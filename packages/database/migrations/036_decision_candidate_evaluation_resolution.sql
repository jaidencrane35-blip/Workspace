-- Decision Candidate evaluation resolution (DE-owned scoring-path admission).
-- Persists accepted_for_scoring / rejected_for_scoring only.
-- Never stores scores, ranks, planner handoffs, or Recommendation Engine data.

CREATE TABLE IF NOT EXISTS decision_candidate_evaluation_resolution (
    workspace_id TEXT NOT NULL,
    resolution_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    resolution_state TEXT NOT NULL,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    resolution_reason TEXT,
    resolved_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, resolution_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_evaluation_resolution_candidate
    ON decision_candidate_evaluation_resolution (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_evaluation_resolution_workspace
    ON decision_candidate_evaluation_resolution (workspace_id, resolved_at DESC);
