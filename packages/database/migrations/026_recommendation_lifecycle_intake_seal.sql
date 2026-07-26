-- Recommendation decision intake package seal (Sprint 252).
-- Freezes intake digest after proceed denial; never authorizes adapter/handoff/DE ownership.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN intake_seal_json TEXT;
