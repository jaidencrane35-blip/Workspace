-- DecisionCandidateSelection — DE-owned progression decision after ranking.
-- Persists selected / rejected only (non-derivable decisions).
-- Never stores planner handoffs, Gateway actions, or Recommendation Engine data.

CREATE TABLE IF NOT EXISTS decision_candidate_selection (
    workspace_id TEXT NOT NULL,
    selection_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    selection_state TEXT NOT NULL,
    ranking_id TEXT,
    ranking_position INTEGER,
    score_id TEXT,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    selection_reason TEXT,
    selected_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, selection_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_selection_candidate
    ON decision_candidate_selection (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_selection_workspace
    ON decision_candidate_selection (workspace_id, selected_at DESC);
