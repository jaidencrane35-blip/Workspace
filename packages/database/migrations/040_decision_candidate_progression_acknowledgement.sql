-- DecisionCandidateProgressionAcknowledgement — DE-owned progression receipt.
-- Persists acknowledged / rejected only (non-derivable decisions).
-- Never stores planner handoffs, Gateway actions, or Recommendation Engine data.

CREATE TABLE IF NOT EXISTS decision_candidate_progression_acknowledgement (
    workspace_id TEXT NOT NULL,
    acknowledgement_id TEXT NOT NULL,
    decision_candidate_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    acknowledgement_state TEXT NOT NULL,
    request_id TEXT NOT NULL,
    request_state TEXT NOT NULL,
    selection_id TEXT NOT NULL,
    ranking_id TEXT,
    score_id TEXT,
    recommendation_reference TEXT,
    package_seal_digest TEXT,
    intake_candidate_id TEXT,
    creation_request_id TEXT,
    acknowledgement_reason TEXT,
    acknowledged_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, acknowledgement_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_candidate_progression_ack_candidate
    ON decision_candidate_progression_acknowledgement (workspace_id, decision_candidate_id);

CREATE INDEX IF NOT EXISTS idx_decision_candidate_progression_ack_workspace
    ON decision_candidate_progression_acknowledgement (workspace_id, acknowledged_at DESC);
