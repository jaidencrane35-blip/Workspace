-- User-authored handoff / intended next action (Product Proof PP-P01A).
--
-- Explicit user text only. Never inferred, generated, or observed.
-- Empty string means the context was saved before this field existed, or the
-- row predated the requirement; new Saves refuse an empty handoff in domain
-- validation.

ALTER TABLE saved_contexts ADD COLUMN handoff_note TEXT NOT NULL DEFAULT '';
