-- Recommendation decision intake adapter preparation (Sprint 257).
-- Records prepared RE→future-DE adapter path; never invokes adapter or creates DE objects.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN adapter_preparation_json TEXT;
