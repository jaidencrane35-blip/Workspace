-- Recommendation decision engine acceptance boundary (Sprint 267).
-- Records non-executing DE acceptance of handoff request; never transfers ownership or creates DE objects.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN decision_engine_acceptance_json TEXT;
