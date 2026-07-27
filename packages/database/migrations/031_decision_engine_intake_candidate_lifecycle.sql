-- Decision Engine intake candidate lifecycle (DE-owned).
-- States: active | withdrawn | invalidated. Never mutates recommendation_lifecycle.

ALTER TABLE decision_engine_intake_candidate
    ADD COLUMN lifecycle_state TEXT NOT NULL DEFAULT 'active';

ALTER TABLE decision_engine_intake_candidate
    ADD COLUMN lifecycle_reason TEXT;

ALTER TABLE decision_engine_intake_candidate
    ADD COLUMN lifecycle_updated_at TEXT;
