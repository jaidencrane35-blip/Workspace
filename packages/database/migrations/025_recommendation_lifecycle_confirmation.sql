-- Recommendation decision confirmation overlay (Sprint 227).
-- Records user confirmation beyond accept-as-agreement; never creates DE/intent/execution.

ALTER TABLE recommendation_lifecycle
    ADD COLUMN confirmation_json TEXT;
