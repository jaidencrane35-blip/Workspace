-- DecisionCandidateProgressionRequest — DE-owned downstream consideration request.
-- Persists requested / cancelled only (non-derivable decisions).
-- Never stores planner handoffs, Gateway actions, or Recommendation Engine data.

CREATE TABLE IF NOT EXISTS decision_candidate_progression_request (
    workspace_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    request_state TEXT NOT NULL,
    selection_id TEXT NOT NULL,
    selection_state TEXT NOT NULL,
    ranking_id TEXT,
    ranking_position INTEGER,
    score_id TEXT,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    request_reason TEXT,
    requested_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, request_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_progression_request_candidate
    ON decision_candidate_progression_request (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_progression_request_workspace
    ON decision_candidate_progression_request (workspace_id, requested_at DESC);
