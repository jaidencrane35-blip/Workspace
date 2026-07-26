-- Recommendation lifecycle continuity fingerprint (Sprint 197).
-- Detects material candidate content change for supersession — never a score.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN content_fingerprint TEXT;
