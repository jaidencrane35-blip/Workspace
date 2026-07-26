-- Recommendation decision handoff request (Sprint 262).
-- Records non-executing RE→future-DE handoff request; never performs handoff or creates DE objects.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN handoff_request_json TEXT;
