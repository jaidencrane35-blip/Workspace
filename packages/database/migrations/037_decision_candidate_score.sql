-- DecisionScore (DecisionCandidateScore) — DE-owned scoring result.
-- Persists historical score identity for accepted_for_scoring candidates.
-- Never stores ranking, selection, planner handoffs, or Recommendation Engine data.

CREATE TABLE IF NOT EXISTS decision_candidate_score (
    workspace_id TEXT NOT NULL,
    score_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    resolution_id TEXT NOT NULL,
    score_total INTEGER NOT NULL,
    attention_contribution INTEGER NOT NULL,
    memory_contribution INTEGER NOT NULL,
    personalization_contribution INTEGER NOT NULL,
    goal_contribution INTEGER NOT NULL,
    scoring_factors_json TEXT NOT NULL,
    scored_at TEXT NOT NULL,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, score_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_score_candidate
    ON decision_candidate_score (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_score_workspace
    ON decision_candidate_score (workspace_id, scored_at DESC);
