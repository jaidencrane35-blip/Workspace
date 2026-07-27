-- Decision Engine intake candidate (DE-owned intake lifecycle acknowledgement).
-- Distinct from decision_engine_lifecycle (DecisionCandidate outcome overlay).
-- Never stores Recommendation Engine payloads; never transfers RE ownership.

CREATE TABLE IF NOT EXISTS decision_engine_intake_candidate (
    workspace_id TEXT NOT NULL,
    intake_candidate_id TEXT NOT NULL,
    intake_receipt_reference TEXT NOT NULL,
    recommendation_reference TEXT NOT NULL,
    package_seal_digest TEXT NOT NULL,
    acceptance_reference TEXT NOT NULL,
    compatibility_version TEXT NOT NULL,
    state TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (workspace_id, intake_candidate_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_decision_engine_intake_candidate_digest
    ON decision_engine_intake_candidate (workspace_id, package_seal_digest);

CREATE INDEX IF NOT EXISTS idx_decision_engine_intake_candidate_workspace
    ON decision_engine_intake_candidate (workspace_id, updated_at DESC);
