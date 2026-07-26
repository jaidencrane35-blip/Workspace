-- Prior recommendation outcomes for historical continuity (Sprint 207).
-- Retains immutable outcome records across supersede / generation reopen.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN prior_outcomes_json TEXT;
